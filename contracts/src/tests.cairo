#[cfg(test)]
mod tests {
    use contracts::{IStarksqueezeDispatcher, IStarksqueezeDispatcherTrait, Visibility};
    use starknet::{ContractAddress, contract_address_const};
    use snforge_std::{
        declare, ContractClassTrait, DeclareResultTrait, 
        start_cheat_caller_address, stop_cheat_caller_address
    };

    fn deploy_contract() -> (IStarksqueezeDispatcher, ContractAddress) {
        let admin = contract_address_const::<0x123>();
        let contract = declare("Starksqueeze").unwrap().contract_class();
        let (contract_address, _) = contract.deploy(@array![admin.into()]).unwrap();
        (IStarksqueezeDispatcher { contract_address }, admin)
    }

    // ========== CONSTRUCTOR TESTS ==========

    #[test]
    fn test_constructor() {
        let (contract, admin) = deploy_contract();
        assert(contract.is_admin(admin), 'Admin should be set');
    }

    #[test]
    fn test_constructor_sets_admin_role() {
        let (contract, admin) = deploy_contract();
        assert(contract.is_admin(admin), 'Admin role should be set');
    }

    // ========== FILE UPLOAD TESTS ==========

    #[test]
    fn test_upload_file() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
       
        start_cheat_caller_address(contract.contract_address, user);
        
        contract.upload_file(
            123, // file_hash
            'test.txt', // filename
            1000, // original_size
            800, // compressed_size
            'txt', // file_format
            'ipfs_hash' // ipfs_cid
        );
        
        stop_cheat_caller_address(contract.contract_address);
        
        assert(contract.file_exists(user, 123), 'File should exist');
        assert(contract.get_user_files(user) == 1, 'User should have 1 file');
        assert(contract.get_user_file_by_index(user, 0) == 123, 'File hash should match');
    }

    #[test]
    fn test_upload_file_metadata() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        
        let file_data = contract.get_file_data(user, 123);
        assert(file_data.uri == 123, 'URI should match');
        assert(file_data.file_format == 'txt', 'File format should match');
        assert(file_data.owner == user, 'Owner should match');
        assert(file_data.visibility == Visibility::Public, 'Default visibility public');
        assert(file_data.original_size == 1000, 'Original size should match');
        assert(file_data.final_size == 800, 'Compressed size should match');
        assert(file_data.compressed_by == 20, 'Compression should be 20%');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    #[should_panic(expected: ('File already exists',))]
    fn test_upload_duplicate_file() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload file first time
        contract.upload_file(123, 'test1.txt', 1000, 800, 'txt', 'cid1');
        
        // Try to upload same file again - should panic
        contract.upload_file(123, 'test2.txt', 2000, 1000, 'txt', 'cid2');
    }

    #[test]
    fn test_upload_multiple_files() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload multiple files
        contract.upload_file(123, 'test1.txt', 1000, 800, 'txt', 'cid1');
        contract.upload_file(456, 'test2.txt', 2000, 1500, 'pdf', 'cid2');
        contract.upload_file(789, 'test3.txt', 500, 400, 'jpg', 'cid3');
        
        stop_cheat_caller_address(contract.contract_address);
        
        assert(contract.get_user_files(user) == 3, 'User should have 3 files');
        assert(contract.get_user_file_by_index(user, 0) == 123, 'First file should match');
        assert(contract.get_user_file_by_index(user, 1) == 456, 'Second file should match');
        assert(contract.get_user_file_by_index(user, 2) == 789, 'Third file should match');
    }

    #[test]
    fn test_upload_file_compression_calculation() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Test 50% compression
        contract.upload_file(123, 'test.txt', 1000, 500, 'txt', 'cid1');
        let file_data = contract.get_file_data(user, 123);
        assert(file_data.compressed_by == 50, 'Compression should be 50%');
        
        // Test 0% compression
        contract.upload_file(456, 'test2.txt', 1000, 1000, 'txt', 'cid2');
        let file_data2 = contract.get_file_data(user, 456);
        assert(file_data2.compressed_by == 0, 'Compression should be 0%');
        
        // Test 100% compression (edge case)
        contract.upload_file(789, 'test3.txt', 1000, 0, 'txt', 'cid3');
        let file_data3 = contract.get_file_data(user, 789);
        assert(file_data3.compressed_by == 100, 'Compression should be 100%');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    // ========== FILE DELETION TESTS ==========

    #[test]
    fn test_delete_file() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload file
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        assert(contract.file_exists(user, 123), 'File exists before deletion');
        
        // Delete file
        contract.delete_file(123);
        
        // Verify file is deleted
        assert(!contract.file_exists(user, 123), 'File not exist after deletion');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    #[should_panic(expected: ('File not found',))]
    fn test_delete_nonexistent_file() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Try to delete file that doesn't exist
        contract.delete_file(999);
    }

    #[test]
    fn test_delete_file_updates_count() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload multiple files
        contract.upload_file(123, 'test1.txt', 1000, 800, 'txt', 'cid1');
        contract.upload_file(456, 'test2.txt', 2000, 1500, 'pdf', 'cid2');
        assert(contract.get_user_files(user) == 2, 'Should have 2 files');
        
        // Delete one file
        contract.delete_file(123);
        
        // Count should still be 2 (files are marked as deleted but count doesn't decrease)
        assert(contract.get_user_files(user) == 2, 'File count should remain 2');
        assert(!contract.file_exists(user, 123), 'First file should be deleted');
        assert(contract.file_exists(user, 456), 'Second file should still exist');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    // ========== FILE VISIBILITY TESTS ==========

    #[test]
    fn test_update_file_visibility() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload a file
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        
        // Update visibility to private
        contract.update_file_visibility(123, Visibility::Private);
        
        // Verify visibility change
        let file_data = contract.get_file_data(user, 123);
        assert(file_data.visibility == Visibility::Private, 'Visibility should be private');
        
        // Update to shared
        contract.update_file_visibility(123, Visibility::Shared);
        let file_data2 = contract.get_file_data(user, 123);
        assert(file_data2.visibility == Visibility::Shared, 'Visibility should be shared');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    #[should_panic(expected: ('File not found',))]
    fn test_update_visibility_nonexistent_file() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Try to update visibility of file that doesn't exist
        contract.update_file_visibility(999, Visibility::Private);
    }

    // ========== FILE SHARING TESTS ==========

    #[test]
    fn test_share_file() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let shared_with = contract_address_const::<0x789>();
        
        start_cheat_caller_address(contract.contract_address, owner);
        
        // Upload a file
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        
        // Share file
        contract.share_file(123, shared_with);
        
        stop_cheat_caller_address(contract.contract_address);
        
        // Verify sharing
        assert(contract.is_file_shared_with(owner, 123, shared_with), 'File should be shared');
        assert(contract.get_shared_files_count(shared_with) == 1, 'Shared files count should be 1');
        
        let (file_owner, file_hash) = contract.get_shared_file_by_index(shared_with, 0);
        assert(file_owner == owner, 'File owner should match');
        assert(file_hash == 123, 'File hash should match');
    }

    #[test]
    #[should_panic(expected: ('File not found',))]
    fn test_share_nonexistent_file() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let shared_with = contract_address_const::<0x789>();
        
        start_cheat_caller_address(contract.contract_address, owner);
        
        // Try to share file that doesn't exist
        contract.share_file(999, shared_with);
    }

    #[test]
    #[should_panic(expected: ('Cannot share with yourself',))]
    fn test_share_file_with_self() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, owner);
        
        // Upload a file
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        
        // Try to share with self
        contract.share_file(123, owner);
    }

    #[test]
    #[should_panic(expected: ('File already shared with user',))]
    fn test_share_file_already_shared() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let shared_with = contract_address_const::<0x789>();
        
        start_cheat_caller_address(contract.contract_address, owner);
        
        // Upload a file
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        
        // Share file first time
        contract.share_file(123, shared_with);
        
        // Try to share again - should panic
        contract.share_file(123, shared_with);
    }

    #[test]
    fn test_share_file_with_multiple_users() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let user1 = contract_address_const::<0x789>();
        let user2 = contract_address_const::<0xabc>();
        
        start_cheat_caller_address(contract.contract_address, owner);
        
        // Upload a file
        contract.upload_file(123, 'test.txt', 1000, 800, 'txt', 'ipfs_hash');
        
        // Share with multiple users
        contract.share_file(123, user1);
        contract.share_file(123, user2);
        
        stop_cheat_caller_address(contract.contract_address);
        
        // Verify both users can access
        assert(contract.is_file_shared_with(owner, 123, user1), 'File shared with user1');
        assert(contract.is_file_shared_with(owner, 123, user2), 'File shared with user2');
        
        // Both users should see the file in their shared files
        assert(contract.get_shared_files_count(user1) == 1, 'User1 should have 1 shared file');
        assert(contract.get_shared_files_count(user2) == 1, 'User2 should have 1 shared file');
    }

    // ========== ACCESS CONTROL TESTS ==========

    #[test]
    fn test_access_control_public_file() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let other_user = contract_address_const::<0x789>();
        
        // Owner uploads a public file
        start_cheat_caller_address(contract.contract_address, owner);
        contract.upload_file(123, 'public.txt', 1000, 800, 'txt', 'ipfs_hash');
        stop_cheat_caller_address(contract.contract_address);
        
        // Other user should be able to access public file
        start_cheat_caller_address(contract.contract_address, other_user);
        let file_data = contract.get_file_data(owner, 123);
        assert(file_data.visibility == Visibility::Public, 'Should access public file');
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    fn test_access_control_private_file() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let other_user = contract_address_const::<0x789>();
        
        // Owner uploads and makes file private
        start_cheat_caller_address(contract.contract_address, owner);
        contract.upload_file(123, 'private.txt', 1000, 800, 'txt', 'ipfs_hash');
        contract.update_file_visibility(123, Visibility::Private);
        stop_cheat_caller_address(contract.contract_address);
        
        // Owner should still be able to access their private file
        start_cheat_caller_address(contract.contract_address, owner);
        let file_data = contract.get_file_data(owner, 123);
        assert(file_data.visibility == Visibility::Private, 'Owner can access private file');
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    #[should_panic(expected: ('Access denied: private file',))]
    fn test_access_control_private_file_denied() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let other_user = contract_address_const::<0x789>();
        
        // Owner uploads and makes file private
        start_cheat_caller_address(contract.contract_address, owner);
        contract.upload_file(123, 'private.txt', 1000, 800, 'txt', 'ipfs_hash');
        contract.update_file_visibility(123, Visibility::Private);
        stop_cheat_caller_address(contract.contract_address);
        
        // Other user should not be able to access private file
        start_cheat_caller_address(contract.contract_address, other_user);
        contract.get_file_data(owner, 123);
    }

    #[test]
    fn test_access_control_shared_file() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let shared_user = contract_address_const::<0x789>();
        let other_user = contract_address_const::<0xabc>();
        
        // Owner uploads file and shares it
        start_cheat_caller_address(contract.contract_address, owner);
        contract.upload_file(123, 'shared.txt', 1000, 800, 'txt', 'ipfs_hash');
        contract.update_file_visibility(123, Visibility::Shared);
        contract.share_file(123, shared_user);
        stop_cheat_caller_address(contract.contract_address);
        
        // Shared user should be able to access
        start_cheat_caller_address(contract.contract_address, shared_user);
        let file_data = contract.get_file_data(owner, 123);
        assert(file_data.visibility == Visibility::Shared, 'Shared user can access file');
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    #[should_panic(expected: ('Access denied: unauthorized',))]
    fn test_access_control_shared_file_denied() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let shared_user = contract_address_const::<0x789>();
        let other_user = contract_address_const::<0xabc>();
        
        // Owner uploads file and shares it
        start_cheat_caller_address(contract.contract_address, owner);
        contract.upload_file(123, 'shared.txt', 1000, 800, 'txt', 'ipfs_hash');
        contract.update_file_visibility(123, Visibility::Shared);
        contract.share_file(123, shared_user);
        stop_cheat_caller_address(contract.contract_address);
        
        // Other user should not be able to access
        start_cheat_caller_address(contract.contract_address, other_user);
        contract.get_file_data(owner, 123);
    }

    #[test]
    fn test_get_user_file_by_index_access_control() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let other_user = contract_address_const::<0x789>();
        
        // Owner uploads files with different visibility
        start_cheat_caller_address(contract.contract_address, owner);
        contract.upload_file(123, 'public.txt', 1000, 800, 'txt', 'cid1');
        contract.upload_file(456, 'private.txt', 1000, 800, 'txt', 'cid2');
        contract.update_file_visibility(456, Visibility::Private);
        stop_cheat_caller_address(contract.contract_address);
        
        // Other user should only be able to access public file
        start_cheat_caller_address(contract.contract_address, other_user);
        let public_file = contract.get_user_file_by_index(owner, 0);
        assert(public_file == 123, 'Should get public file');
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    #[should_panic(expected: ('Access denied: private file',))]
    fn test_get_user_file_by_index_access_control_denied() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let other_user = contract_address_const::<0x789>();
        
        // Owner uploads files with different visibility
        start_cheat_caller_address(contract.contract_address, owner);
        contract.upload_file(123, 'public.txt', 1000, 800, 'txt', 'cid1');
        contract.upload_file(456, 'private.txt', 1000, 800, 'txt', 'cid2');
        contract.update_file_visibility(456, Visibility::Private);
        stop_cheat_caller_address(contract.contract_address);
        
        // Should not be able to access private file
        start_cheat_caller_address(contract.contract_address, other_user);
        contract.get_user_file_by_index(owner, 1);
    }

    // ========== ADMIN FUNCTION TESTS ==========

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
    #[should_panic(expected: ('Only admin can add admins',))]
    fn test_add_admin_non_admin() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        let new_admin = contract_address_const::<0x789>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Non-admin tries to add admin - should panic
        contract.add_admin(new_admin);
    }

    #[test]
    fn test_is_admin() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        assert(contract.is_admin(admin), 'Admin should be admin');
        assert(!contract.is_admin(user), 'User should not be admin');
    }

    // ========== COMPRESSION MAPPING TESTS ==========

    #[test]
    fn test_store_compression_mapping() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        let chunk_mappings = array![1, 2, 3];
        let chunk_values = array![4, 5, 6];
        let byte_mappings = array![7, 8, 9];
        let byte_values = array![10, 11, 12];
        let reconstruction_steps = array![13, 14, 15];
        let metadata = array![16, 17, 18];
        
        contract.store_compression_mapping(
            123, // uri
            'txt', // file_format
            50, // compressed_by
            1000, // original_size
            500, // final_size
            8, // chunk_size
            chunk_mappings,
            chunk_values,
            byte_mappings,
            byte_values,
            reconstruction_steps,
            metadata,
        );
        
        stop_cheat_caller_address(contract.contract_address);
        
        // Function should complete without error
        assert(true, 'Compression mapping stored');
    }

    #[test]
    #[should_panic(expected: ('Invalid compression percentage',))]
    fn test_store_compression_mapping_invalid_percentage() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        let chunk_mappings = array![];
        let chunk_values = array![];
        let byte_mappings = array![];
        let byte_values = array![];
        let reconstruction_steps = array![];
        let metadata = array![];
        
        // Try to store with invalid compression percentage (>100)
        contract.store_compression_mapping(
            123, // uri
            'txt', // file_format
            101, // compressed_by - invalid
            1000, // original_size
            500, // final_size
            8, // chunk_size
            chunk_mappings,
            chunk_values,
            byte_mappings,
            byte_values,
            reconstruction_steps,
            metadata,
        );
    }

    // ========== EDGE CASE TESTS ==========

    #[test]
    fn test_zero_sized_file() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload file with zero size
        contract.upload_file(123, 'empty.txt', 0, 0, 'txt', 'ipfs_hash');
        
        let file_data = contract.get_file_data(user, 123);
        assert(file_data.original_size == 0, 'Original size should be 0');
        assert(file_data.final_size == 0, 'Final size should be 0');
        assert(file_data.compressed_by == 0, 'Compression should be 0%');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    fn test_large_file_handling() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload file with large size
        let large_size = 1000000; // 1MB
        contract.upload_file(123, 'large.txt', large_size, large_size / 2, 'txt', 'ipfs_hash');
        
        let file_data = contract.get_file_data(user, 123);
        assert(file_data.original_size == large_size, 'Original size should match');
        assert(file_data.final_size == large_size / 2, 'Final size should match');
        assert(file_data.compressed_by == 50, 'Compression should be 50%');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    fn test_empty_filename() {
        let (contract, admin) = deploy_contract();
        let user = contract_address_const::<0x456>();
        
        start_cheat_caller_address(contract.contract_address, user);
        
        // Upload file with empty filename
        contract.upload_file(123, '', 1000, 800, 'txt', 'ipfs_hash');
        
        let file_data = contract.get_file_data(user, 123);
        assert(file_data.uri == 123, 'File should be stored');
        
        stop_cheat_caller_address(contract.contract_address);
    }

    // ========== INTEGRATION TESTS ==========

    #[test]
    fn test_complete_workflow() {
        let (contract, admin) = deploy_contract();
        let owner = contract_address_const::<0x456>();
        let shared_user = contract_address_const::<0x789>();
        
        start_cheat_caller_address(contract.contract_address, owner);
        
        // 1. Upload file
        contract.upload_file(123, 'workflow.txt', 1000, 800, 'txt', 'ipfs_hash');
        assert(contract.file_exists(owner, 123), 'File should exist');
        
        // 2. Update visibility to shared
        contract.update_file_visibility(123, Visibility::Shared);
        
        // 3. Share with another user
        contract.share_file(123, shared_user);
        
        stop_cheat_caller_address(contract.contract_address);
        
        // 4. Verify shared user can access
        start_cheat_caller_address(contract.contract_address, shared_user);
        let file_data = contract.get_file_data(owner, 123);
        assert(file_data.visibility == Visibility::Shared, 'File should be shared');
        stop_cheat_caller_address(contract.contract_address);
        
        // 5. Owner deletes file
        start_cheat_caller_address(contract.contract_address, owner);
        contract.delete_file(123);
        assert(!contract.file_exists(owner, 123), 'File should be deleted');
        stop_cheat_caller_address(contract.contract_address);
    }

    #[test]
    fn test_multiple_users_multiple_files() {
        let (contract, admin) = deploy_contract();
        let user1 = contract_address_const::<0x456>();
        let user2 = contract_address_const::<0x789>();
        let user3 = contract_address_const::<0xabc>();
        
        // User1 uploads files
        start_cheat_caller_address(contract.contract_address, user1);
        contract.upload_file(111, 'user1_file1.txt', 1000, 800, 'txt', 'cid1');
        contract.upload_file(112, 'user1_file2.txt', 2000, 1500, 'pdf', 'cid2');
        stop_cheat_caller_address(contract.contract_address);
        
        // User2 uploads files
        start_cheat_caller_address(contract.contract_address, user2);
        contract.upload_file(221, 'user2_file1.txt', 1500, 1200, 'txt', 'cid3');
        contract.upload_file(222, 'user2_file2.txt', 3000, 2000, 'doc', 'cid4');
        stop_cheat_caller_address(contract.contract_address);
        
        // User3 uploads files
        start_cheat_caller_address(contract.contract_address, user3);
        contract.upload_file(331, 'user3_file1.txt', 800, 600, 'jpg', 'cid5');
        stop_cheat_caller_address(contract.contract_address);
        
        // Verify file counts
        assert(contract.get_user_files(user1) == 2, 'User1 should have 2 files');
        assert(contract.get_user_files(user2) == 2, 'User2 should have 2 files');
        assert(contract.get_user_files(user3) == 1, 'User3 should have 1 file');
        
        // User1 shares file with User3
        start_cheat_caller_address(contract.contract_address, user1);
        contract.share_file(111, user3);
        stop_cheat_caller_address(contract.contract_address);
        
        // User2 shares file with User1
        start_cheat_caller_address(contract.contract_address, user2);
        contract.share_file(221, user1);
        stop_cheat_caller_address(contract.contract_address);
        
        // Verify sharing
        assert(contract.is_file_shared_with(user1, 111, user3), 'User1 file shared with User3');
        assert(contract.is_file_shared_with(user2, 221, user1), 'User2 file shared with User1');
        
        assert(contract.get_shared_files_count(user1) == 1, 'User1 should have 1 shared file');
        assert(contract.get_shared_files_count(user3) == 1, 'User3 should have 1 shared file');
    }
}
