/// Interface representing the `Starksqueeze` contract.
/// This interface focuses on compression mapping storage for file reconstruction.
#[starknet::interface]
pub trait IStarksqueeze<TContractState> {
    /// Stores complete compression mapping data needed to reconstruct the original file.
    fn store_compression_mapping(
        ref self: TContractState,
        // File identification
        uri: felt252,
        file_format: felt252,
        // Compression metadata
        compressed_by: u8, // compression percentage (0-100)
        original_size: usize,
        final_size: usize,
        // Mapping data for reconstruction
        chunk_size: usize,
        chunk_mappings: Array<felt252>, // chunk keys
        chunk_values: Array<u8>, // chunk values
        byte_mappings: Array<u8>, // byte keys
        byte_values: Array<felt252>, // byte values
        // Additional reconstruction data
        reconstruction_steps: Array<felt252>,
        metadata: Array<felt252>,
    );
}

/// Contract for storing compression mapping data for file reconstruction.
#[starknet::contract]
mod Starksqueeze {
    #[storage]
    struct Storage {} // No state needed currently

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        CompressionMappingStored: CompressionMappingStoredEvent,
    }

    /// Event emitted when compression mapping data is stored.
    #[derive(Drop, starknet::Event)]
    struct CompressionMappingStoredEvent {
        #[key]
        uri: felt252,
        file_format: felt252,
        compressed_by: u8,
        original_size: usize,
        final_size: usize,
        chunk_size: usize,
        chunk_mappings: Array<felt252>,
        chunk_values: Array<u8>,
        byte_mappings: Array<u8>,
        byte_values: Array<felt252>,
        reconstruction_steps: Array<felt252>,
        metadata: Array<felt252>,
    }

    #[abi(embed_v0)]
    impl StarksqueezeImpl of super::IStarksqueeze<ContractState> {
        fn store_compression_mapping(
            ref self: ContractState,
            // File identification
            uri: felt252,
            file_format: felt252,
            // Compression metadata
            compressed_by: u8,
            original_size: usize,
            final_size: usize,
            // Mapping data for reconstruction
            chunk_size: usize,
            chunk_mappings: Array<felt252>,
            chunk_values: Array<u8>,
            byte_mappings: Array<u8>,
            byte_values: Array<felt252>,
            // Additional reconstruction data
            reconstruction_steps: Array<felt252>,
            metadata: Array<felt252>,
        ) {
            // Validate compression percentage (0-100)
            assert(compressed_by <= 100, 'Invalid compression percentage');
            
            self.emit(
                CompressionMappingStoredEvent {
                    uri,
                    file_format,
                    compressed_by,
                    original_size,
                    final_size,
                    chunk_size,
                    chunk_mappings,
                    chunk_values,
                    byte_mappings,
                    byte_values,
                    reconstruction_steps,
                    metadata,
                },
            );
        }
    }
}
