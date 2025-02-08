#[starknet::interface]
trait IDataStorage<TContractState> {
    fn add_data(ref self: TContractState, cid: felt252);
}

#[starknet::contract]
mod DataStorage {
    use core::starknet::storage::{StoragePointerWriteAccess};
    use starknet::get_caller_address;
    use starknet::get_block_timestamp;

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        DataAdded: DataAdded,
    }

    #[derive(Drop, starknet::Event)]
    struct DataAdded {
        cid: felt252,
        address: starknet::ContractAddress,
        timestamp: u64,
    }

    #[storage]
    struct Storage {
        cid: felt252,
        address: starknet::ContractAddress,
        timestamp: u64,
    }

    #[abi(embed_v0)]
    impl DataStorage of super::IDataStorage<ContractState> {
        fn add_data(ref self: ContractState, cid: felt252) {
            let caller = get_caller_address();
            let timestamp = get_block_timestamp();
            
            self.cid.write(cid);
            self.address.write(caller);
            self.timestamp.write(timestamp);

            self.emit(DataAdded { cid, address: caller, timestamp });
        }
    }
}
