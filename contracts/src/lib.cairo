#[starknet::interface]
trait IDataStorage<TContractState> {
    fn add_data(ref self: TContractState, cid: felt252);
}

#[starknet::contract]
mod DataStorage {
    use core::starknet::storage::{StoragePointerWriteAccess};

    #[storage]
    struct Storage {
        cid: felt252,
    }

    #[abi(embed_v0)]
    impl DataStorage of super::IDataStorage<ContractState> {
        fn add_data(ref self: ContractState, cid: felt252) {
            self.cid.write(cid);
        }
    }
}
