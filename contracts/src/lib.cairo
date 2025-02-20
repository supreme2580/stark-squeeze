use starknet::ContractAddress;

#[starknet::interface]
trait IDataStorage<TContractState> {
    fn add_data(ref self: TContractState, cid: felt252, file_format: ByteArray);
    fn get_data(self: @TContractState, address: ContractAddress) -> (felt252, ContractAddress, u64, ByteArray);
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
        cid: felt252,
        address: ContractAddress,
        timestamp: u64,
        file_format: ByteArray,
    }

    #[storage]
    struct Storage {
        cid: felt252,
        address: ContractAddress,
        timestamp: u64,
        file_format: ByteArray,

    }

    #[abi(embed_v0)]
    impl DataStorage of super::IDataStorage<ContractState> {
        fn add_data(ref self: ContractState, cid: felt252, file_format: ByteArray) {
            let caller = get_caller_address();
            let timestamp = get_block_timestamp();
            
            self.cid.write(cid);
            self.address.write(caller);
            self.timestamp.write(timestamp);
            self.file_format.write(file_format.clone());


            self.emit(DataAdded { cid, address: caller, timestamp, file_format });
        }

        fn get_data(self: @ContractState, address: ContractAddress) -> (felt252, ContractAddress, u64, ByteArray) {
            assert(self.address.read() == address, 'Address not found');
            (self.cid.read(), self.address.read(), self.timestamp.read(), self.file_format.read())
        }
    }
}
