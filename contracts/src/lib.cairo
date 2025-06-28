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
    #[derive(Copy, Drop, Serde, PartialEq, Eq)]
    enum Visibility {
        Public = 0,
        Private = 1,
        Shared = 2,
    }

    #[derive(Copy, Drop, Serde, PartialEq, Eq)]
    enum Role {
        Admin = 0,
        User = 1,
    }

    #[derive(Drop, Serde)]
    struct FileMetadata {
        uri: felt252,
        file_format: felt252,
        owner: ContractAddress,
        visibility: Visibility,
        shared_with: Array<ContractAddress>,
        description: felt252,
        tags: Array<felt252>,
        category: felt252,
        compressed_by: u8,
        original_size: usize,
        final_size: usize,
        chunk_size: usize,
        chunk_mappings: Array<felt252>,
        chunk_values: Array<u8>,
        byte_mappings: Array<u8>,
        byte_values: Array<felt252>,
        reconstruction_steps: Array<felt252>,
        extra_metadata: Array<felt252>,
    }

    #[storage]
    struct Storage {
        files: LegacyMap<(ContractAddress, felt252), FileMetadata>,
        user_files: LegacyMap<ContractAddress, Array<felt252>>,
        roles: LegacyMap<ContractAddress, Role>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        CompressionMappingStored: CompressionMappingStoredEvent,
        FileUploaded: FileUploadedEvent,
        FileUpdated: FileUpdatedEvent,
        FileDeleted: FileDeletedEvent,
        FileShared: FileSharedEvent,
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

    #[derive(Drop, starknet::Event)]
    struct FileUploadedEvent {
        #[key]
        uri: felt252,
        #[key]
        owner: ContractAddress,
        visibility: Visibility,
    }
    #[derive(Drop, starknet::Event)]
    struct FileUpdatedEvent {
        #[key]
        uri: felt252,
        #[key]
        owner: ContractAddress,
    }
    #[derive(Drop, starknet::Event)]
    struct FileDeletedEvent {
        #[key]
        uri: felt252,
        #[key]
        owner: ContractAddress,
    }
    #[derive(Drop, starknet::Event)]
    struct FileSharedEvent {
        #[key]
        uri: felt252,
        #[key]
        owner: ContractAddress,
        #[key]
        shared_with: ContractAddress,
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
        // --- New: batch upload and advanced file management ---
        fn batch_upload_files(
            ref self: ContractState,
            uris: Array<felt252>,
            file_formats: Array<felt252>,
            visibilities: Array<Visibility>,
            descriptions: Array<felt252>,
            tags: Array<Array<felt252>>,
            categories: Array<felt252>,
            compressed_bys: Array<u8>,
            original_sizes: Array<usize>,
            final_sizes: Array<usize>,
            chunk_sizes: Array<usize>,
            chunk_mappings_arr: Array<Array<felt252>>,
            chunk_values_arr: Array<Array<u8>>,
            byte_mappings_arr: Array<Array<u8>>,
            byte_values_arr: Array<Array<felt252>>,
            reconstruction_steps_arr: Array<Array<felt252>>,
            extra_metadata_arr: Array<Array<felt252>>,
        ) {
            let sender = get_caller_address();
            let len = uris.len();
            assert(len == file_formats.len(), 'Array length mismatch');
            // ...repeat for all arrays...
            let mut i = 0;
            while i < len {
                let uri = uris[i];
                let file_format = file_formats[i];
                let visibility = visibilities[i];
                let description = descriptions[i];
                let tag = tags[i];
                let category = categories[i];
                let compressed_by = compressed_bys[i];
                let original_size = original_sizes[i];
                let final_size = final_sizes[i];
                let chunk_size = chunk_sizes[i];
                let chunk_mappings = chunk_mappings_arr[i];
                let chunk_values = chunk_values_arr[i];
                let byte_mappings = byte_mappings_arr[i];
                let byte_values = byte_values_arr[i];
                let reconstruction_steps = reconstruction_steps_arr[i];
                let extra_metadata = extra_metadata_arr[i];
                let file = FileMetadata {
                    uri,
                    file_format,
                    owner: sender,
                    visibility,
                    shared_with: ArrayTrait::new(),
                    description,
                    tags: tag,
                    category,
                    compressed_by,
                    original_size,
                    final_size,
                    chunk_size,
                    chunk_mappings,
                    chunk_values,
                    byte_mappings,
                    byte_values,
                    reconstruction_steps,
                    extra_metadata,
                };
                self.files.write((sender, uri), file);
                // Add to user_files
                let mut user_uris = self.user_files.read(sender).unwrap_or(ArrayTrait::new());
                user_uris.append(uri);
                self.user_files.write(sender, user_uris);
                self.emit(FileUploadedEvent { uri, owner: sender, visibility });
                i += 1;
            }
        }
        // ...other new functions for update, delete, share, etc...
    }
}
