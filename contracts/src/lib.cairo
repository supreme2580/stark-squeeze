/// Interface representing the `Starksqueeze` contract.
/// This interface focuses on compression metadata extraction and event emission.
#[starknet::interface]
pub trait IStarksqueeze<TContractState> {
    /// Processes file compression data and emits metadata for analytics and tracking.
    fn process_file_metadata(
        ref self: TContractState,
        data_size: usize,
        file_type: felt252,
        original_size: usize,
    );
}

/// Contract for emitting compression metadata events used in file analytics.
#[starknet::contract]
mod Starksqueeze {
    #[storage]
    struct Storage {} // No state needed currently

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        CompressionMetadata: CompressionMetadataEvent,
    }

    /// Event emitted after file metadata is processed for compression analysis.
    #[derive(Drop, starknet::Event)]
    struct CompressionMetadataEvent {
        #[key]
        file_type: felt252,
        size: usize,
        original_size: usize,
        new_size: usize,
        compression_ratio: u64,
    }

    #[abi(embed_v0)]
    impl StarksqueezeImpl of super::IStarksqueeze<ContractState> {
        fn process_file_metadata(
            ref self: ContractState,
            data_size: usize,
            file_type: felt252,
            original_size: usize,
        ) {
            let new_size = if data_size == 0 { 0 } else { data_size / 2 };

            let compression_ratio = if new_size == 0 {
                0_u64
            } else {
                let ratio: u64 = (original_size * 100 / new_size).into();
                ratio
            };

            self.emit(CompressionMetadataEvent {
                file_type,
                size: data_size,
                original_size,
                new_size,
                compression_ratio,
            });
        }
    }
}
