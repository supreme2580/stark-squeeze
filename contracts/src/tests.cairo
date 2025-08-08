#[cfg(test)]
mod tests {
    use contracts::{IStarksqueezeDispatcher, IStarksqueezeDispatcherTrait, Visibility};
    use starknet::{ContractAddress, contract_address_const};
    use snforge_std::{declare, ContractClassTrait, DeclareResultTrait, start_cheat_caller_address, stop_cheat_caller_address};

    fn deploy_contract() -> (IStarksqueezeDispatcher, ContractAddress) {
        let admin = contract_address_const::<0x123>();
        let contract = declare("Starksqueeze").unwrap().contract_class();
        let (contract_address, _) = contract.deploy(@array![admin.into()]).unwrap();
        (IStarksqueezeDispatcher { contract_address }, admin)
    }

    #[test]
    fn test_constructor() {
        let (contract, admin) = deploy_contract();
        assert(contract.is_admin(admin), 'Admin should be set');
    }

    #[test]
    fn test_upload_file() {
        let (contract, admin) = deploy_contract();
       
        start_cheat_caller_address(contract.contract_address, admin);
        
        contract.upload_file(
            123, // file_hash
            'test.txt', // filename
            1000, // original_size
            800, // compressed_size
            'txt', // file_format
            'ipfs_hash' // ipfs_cid
        );
        
        stop_cheat_caller_address(contract.contract_address);
        
        assert(contract.file_exists(admin, 123), 'File should exist');
        assert(contract.get_user_files(admin) == 1, 'User should have 1 file');
        assert(contract.get_user_file_by_index(admin, 0) == 123, 'File hash should match');
    }

    #[test]
    fn test_file_visibility() {
        let (contract, admin) = deploy_contract();
        
        // Set caller address to admin for all operations
        start_cheat_caller_address(contract.contract_address, admin);
        
        // Upload a file
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        
        // Update visibility to private
        contract.update_file_visibility(123, Visibility::Private);
        
        // As the owner, should still be able to access the private file
        let file_data = contract.get_file_data(admin, 123);
        assert(file_data.visibility == Visibility::Private, 'Visibility should be private');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    fn test_admin_functions() {
        let (contract, admin) = deploy_contract();
        let new_admin = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, admin);
        
        contract.add_admin(new_admin);
        
        stop_cheat_caller_address(contract.contract_address);
        
        assert(contract.is_admin(new_admin), 'New admin should be set');
    }
    
    #[test]
    fn test_file_visibility_access_control() {
        let (contract, admin) = deploy_contract();
        let other_user = contract_address_const::<0x111>();
        
        // Admin uploads a public file
        start_cheat_caller_address(contract.contract_address, admin);
        contract.upload_file(123, 'public.txt', 1000, 800, 'txt', 'ipfs_hash');
        stop_cheat_caller_address(contract.contract_address);
        
        // Other user should be able to access public file
        start_cheat_caller_address(contract.contract_address, other_user);
        let public_file = contract.get_file_data(admin, 123);
        assert(public_file.visibility == Visibility::Public, 'Should access public file');
        stop_cheat_caller_address(contract.contract_address);
        
        // Admin makes file private
        start_cheat_caller_address(contract.contract_address, admin);
        contract.update_file_visibility(123, Visibility::Private);
        
        // Admin should still be able to access their private file
        let private_file = contract.get_file_data(admin, 123);
        assert(private_file.visibility == Visibility::Private, 'Owner can access private file');
        stop_cheat_caller_address(contract.contract_address);
    }
}
