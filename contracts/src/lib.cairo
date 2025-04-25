/// Interface representing `HelloContract`.
/// This interface allows modification and retrieval of the contract balance.
#[starknet::interface]
pub trait IHelloStarknet<TContractState> {
    /// Increase contract balance.
    fn increase_balance(ref self: TContractState, amount: felt252);
    /// Retrieve contract balance.
    fn get_balance(self: @TContractState) -> felt252;
    /// Process file data and emit metadata event
    fn process_file_metadata(ref self: TContractState, data_size: usize, file_type: felt252, original_size: usize);
}

/// Simple contract for managing balance.
#[starknet::contract]
mod HelloStarknet {
    use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};

    #[storage]
    struct Storage {
        balance: felt252,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        FileMetadataEvent: FileMetadataEvent,
    }

    #[derive(Drop, starknet::Event)]
    struct FileMetadataEvent {
        #[key]
        file_type: felt252,
        size: usize,
        original_size: usize,
        new_size: usize,
        compression_ratio: u64,
    }

    #[abi(embed_v0)]
    impl HelloStarknetImpl of super::IHelloStarknet<ContractState> {
        fn increase_balance(ref self: ContractState, amount: felt252) {
            assert(amount != 0, 'Amount cannot be 0');
            self.balance.write(self.balance.read() + amount);
        }

        fn get_balance(self: @ContractState) -> felt252 {
            self.balance.read()
        }

        fn process_file_metadata(ref self: ContractState, data_size: usize, file_type: felt252, original_size: usize) {
            // Calculate compressed size (simplified implementation)
            let new_size = if data_size == 0 { 0 } else { data_size / 2 };
            
            // Calculate compression ratio
            let compression_ratio = if new_size == 0 {
                0_u64
            } else {
                let ratio: u64 = (original_size * 100 / new_size).into();
                ratio
            };
            
            // Emit event
            self.emit(FileMetadataEvent {
                file_type,
                size: data_size,
                original_size,
                new_size,
                compression_ratio,
            });
        }
    }
}
