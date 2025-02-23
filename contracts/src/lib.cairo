use starknet::ContractAddress;

#[starknet::interface]
trait IDataStorage<TContractState> {
<<<<<<< HEAD
    fn add_data(ref self: TContractState, name: ByteArray, cid: ByteArray, file_format: ByteArray);
    fn get_data(
        self: @TContractState, address: ContractAddress
    ) -> (ByteArray, ByteArray, ContractAddress, u64, ByteArray);
=======
    fn add_data(ref self: TContractState, cid: ByteArray, file_format: ByteArray, encrypted: bool,file_type:ByteArray);
    fn get_data(
        self: @TContractState, address: ContractAddress
    ) -> (ByteArray, ContractAddress, u64, ByteArray, bool,ByteArray);
>>>>>>> 3a3ef36a23b5fa3aca7add01efd407502335dd1c
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
        name: ByteArray,
        cid: ByteArray,
        address: ContractAddress,
        timestamp: u64,
        file_format: ByteArray,
        encrypted: bool,
        file_type:ByteArray,
    }

    #[storage]
    struct Storage {
        name: ByteArray,
        cid: ByteArray,
        address: ContractAddress,
        timestamp: u64,
        file_format: ByteArray,
        encrypted: bool,
        file_type:ByteArray,
    }

    #[abi(embed_v0)]
    impl DataStorage of super::IDataStorage<ContractState> {
<<<<<<< HEAD
        fn add_data(ref self: ContractState, name: ByteArray, cid: ByteArray, file_format: ByteArray) {
=======
        fn add_data(ref self: ContractState, cid: ByteArray, file_format: ByteArray, encrypted: bool,file_type:ByteArray) {
>>>>>>> 3a3ef36a23b5fa3aca7add01efd407502335dd1c
            let caller = get_caller_address();
            let timestamp = get_block_timestamp();

            self.name.write(name.clone());
            self.cid.write(cid.clone());
            self.address.write(caller);
            self.timestamp.write(timestamp);
            self.file_format.write(file_format.clone());
<<<<<<< HEAD

            self.emit(DataAdded { name, cid, address: caller, timestamp, file_format });
=======
            self.encrypted.write(encrypted);
            self.file_type.write(file_type.clone());
            self.emit(DataAdded { cid, address: caller, timestamp, file_format, encrypted, file_type });
>>>>>>> 3a3ef36a23b5fa3aca7add01efd407502335dd1c
        }

        fn get_data(
            self: @ContractState, address: ContractAddress
<<<<<<< HEAD
        ) -> (ByteArray, ByteArray, ContractAddress, u64, ByteArray) {
            assert(self.address.read() == address, 'Address not found');
            (
                self.name.read(),
                self.cid.read(),
                self.address.read(),
                self.timestamp.read(),
                self.file_format.read()
            )
=======
        ) -> (ByteArray, ContractAddress, u64, ByteArray, bool, ByteArray ) {
            assert(self.address.read() == address, 'Address not found');
            (self.cid.read(), self.address.read(), self.timestamp.read(), self.file_format.read(), self.encrypted.read(), self.file_type.read())
>>>>>>> 3a3ef36a23b5fa3aca7add01efd407502335dd1c
        }
    }
}