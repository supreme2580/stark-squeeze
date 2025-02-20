use starknet::ContractAddress;

#[starknet::interface]
trait IDataStorage<TContractState> {
    fn add_data(ref self: TContractState, cid: ByteArray);
    fn get_data(self: @TContractState, address: ContractAddress) -> (ByteArray, ContractAddress, u64);
}

#[starknet::contract]
mod DataStorage {
    use super::ContractAddress;
    use core::starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};
    use starknet::get_caller_address;
    use starknet::get_block_timestamp;

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        DataAdded: DataAdded,
    }

    #[derive(Drop, starknet::Event)]
    struct DataAdded {
        cid: ByteArray,
        address: ContractAddress,
        timestamp: u64,
    }

    #[storage]
    struct Storage {
        cid: ByteArray,
        address: ContractAddress,
        timestamp: u64,
    }

    #[abi(embed_v0)]
    impl DataStorage of super::IDataStorage<ContractState> {
        fn add_data(ref self: ContractState, cid: ByteArray) {
            let caller = get_caller_address();
            let timestamp = get_block_timestamp();
            
            self.cid.write(cid.clone());
            self.address.write(caller);
            self.timestamp.write(timestamp);

            self.emit(DataAdded { cid, address: caller, timestamp });
        }

        fn get_data(self: @ContractState, address: ContractAddress) -> (ByteArray, ContractAddress, u64) {
            assert(self.address.read() == address, 'Address not found');
            (self.cid.read(), self.address.read(), self.timestamp.read())
        }
    }
}
