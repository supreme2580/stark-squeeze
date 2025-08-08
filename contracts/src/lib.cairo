use starknet::{ContractAddress, get_caller_address};

#[derive(Copy, Drop, Serde, PartialEq, starknet::Store)]
enum Visibility {
    #[default]
    Public,
    Private,
    Shared,
}

#[derive(Copy, Drop, Serde, PartialEq, starknet::Store)]
enum Role {
    #[default]
    User,
    Admin,
}

#[derive(Drop, Serde, starknet::Store)]
struct FileMetadata {
    uri: felt252,
    file_format: felt252,
    owner: ContractAddress,
    visibility: Visibility,
    description: felt252,
    category: felt252,
    compressed_by: u8,
    original_size: u32,
    final_size: u32,
    chunk_size: u32,
}

#[starknet::interface]
trait IStarksqueeze<TContractState> {
    fn upload_file(
        ref self: TContractState,
        file_hash: felt252,
        filename: felt252,
        original_size: u32,
        compressed_size: u32,
        file_format: felt252,
        ipfs_cid: felt252,
    );
    
    fn delete_file(ref self: TContractState, file_hash: felt252);
    
    fn update_file_visibility(
        ref self: TContractState,
        file_hash: felt252,
        new_visibility: Visibility,
    );
    
    fn share_file(
        ref self: TContractState,
        file_hash: felt252,
        shared_with: ContractAddress,
    );
    
    fn get_user_files(self: @TContractState, user: ContractAddress) -> u32;
    fn get_user_file_by_index(self: @TContractState, user: ContractAddress, index: u32) -> felt252;
    fn get_file_data(self: @TContractState, owner: ContractAddress, file_hash: felt252) -> FileMetadata;
    fn file_exists(self: @TContractState, owner: ContractAddress, file_hash: felt252) -> bool;
    
    // File sharing query functions
    fn is_file_shared_with(self: @TContractState, owner: ContractAddress, file_hash: felt252, user: ContractAddress) -> bool;
    fn get_shared_files_count(self: @TContractState, user: ContractAddress) -> u32;
    fn get_shared_file_by_index(self: @TContractState, user: ContractAddress, index: u32) -> (ContractAddress, felt252);
    
    fn is_admin(self: @TContractState, user: ContractAddress) -> bool;
    fn add_admin(ref self: TContractState, new_admin: ContractAddress);
    
    fn store_compression_mapping(
        ref self: TContractState,
        uri: felt252,
        file_format: felt252,
        compressed_by: u8,
        original_size: u32,
        final_size: u32,
        chunk_size: u32,
        chunk_mappings: Array<felt252>,
        chunk_values: Array<u8>,
        byte_mappings: Array<u8>,
        byte_values: Array<felt252>,
        reconstruction_steps: Array<felt252>,
        metadata: Array<felt252>,
    );
}

#[starknet::contract]
mod Starksqueeze {
    use super::{Visibility, Role, FileMetadata, ContractAddress, get_caller_address};
    use starknet::storage::{Map, StoragePathEntry, StoragePointerReadAccess, StoragePointerWriteAccess};

    #[storage]
    struct Storage {
        files: Map<(ContractAddress, felt252), FileMetadata>,
        user_file_count: Map<ContractAddress, u32>,
        user_file_by_index: Map<(ContractAddress, u32), felt252>,
        roles: Map<ContractAddress, Role>,
        admin: ContractAddress,
        // File sharing: (file_owner, file_hash, shared_with_user) -> bool
        file_shares: Map<(ContractAddress, felt252, ContractAddress), bool>,
        // Track shared files for a user: (user, index) -> (owner, file_hash)
        shared_with_user_count: Map<ContractAddress, u32>,
        shared_with_user_by_index: Map<(ContractAddress, u32), (ContractAddress, felt252)>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        FileUploaded: FileUploadedEvent,
        FileUpdated: FileUpdatedEvent,
        FileDeleted: FileDeletedEvent,
        FileShared: FileSharedEvent,
        CompressionMappingStored: CompressionMappingStoredEvent,
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

    #[derive(Drop, starknet::Event)]
    struct CompressionMappingStoredEvent {
        #[key]
        uri: felt252,
        file_format: felt252,
        compressed_by: u8,
        original_size: u32,
        final_size: u32,
    }

    #[constructor]
    fn constructor(ref self: ContractState, admin: ContractAddress) {
        self.roles.entry(admin).write(Role::Admin);
        self.admin.write(admin);
    }

    #[abi(embed_v0)]
    impl StarksqueezeImpl of super::IStarksqueeze<ContractState> {
        fn upload_file(
            ref self: ContractState,
            file_hash: felt252,
            filename: felt252,
            original_size: u32,
            compressed_size: u32,
            file_format: felt252,
            ipfs_cid: felt252,
        ) {
            let sender = get_caller_address();
            assert(!self.file_exists(sender, file_hash), 'File already exists');
            
            let compression_percent = if original_size > 0 {
                ((original_size - compressed_size) * 100 / original_size).try_into().unwrap()
            } else {
                0_u8
            };
            
            let file_data = FileMetadata {
                uri: file_hash,
                file_format,
                owner: sender,
                visibility: Visibility::Public,
                description: 'File uploaded via StarkSqueeze',
                category: 'general',
                compressed_by: compression_percent,
                original_size,
                final_size: compressed_size,
                chunk_size: 8,
            };
            
            self.files.entry((sender, file_hash)).write(file_data);
            
            // Add to user files list
            let current_count = self.user_file_count.entry(sender).read();
            self.user_file_by_index.entry((sender, current_count)).write(file_hash);
            self.user_file_count.entry(sender).write(current_count + 1);
            
            self.emit(FileUploadedEvent {
                uri: file_hash,
                owner: sender,
                visibility: Visibility::Public,
            });
        }
        
        fn delete_file(ref self: ContractState, file_hash: felt252) {
            let sender = get_caller_address();
            assert(self.file_exists(sender, file_hash), 'File not found');
            
            let empty_file = FileMetadata {
                uri: 0,
                file_format: 0,
                owner: sender,
                visibility: Visibility::Private,
                description: 0,
                category: 0,
                compressed_by: 0,
                original_size: 0,
                final_size: 0,
                chunk_size: 0,
            };
            self.files.entry((sender, file_hash)).write(empty_file);
            
            self.emit(FileDeletedEvent {
                uri: file_hash,
                owner: sender
            });
        }
        
        fn update_file_visibility(
            ref self: ContractState,
            file_hash: felt252,
            new_visibility: Visibility,
        ) {
            let sender = get_caller_address();
            assert(self.file_exists(sender, file_hash), 'File not found');
            
            let file_data = self.files.entry((sender, file_hash)).read();
            let updated_file = FileMetadata {
                uri: file_data.uri,
                file_format: file_data.file_format,
                owner: file_data.owner,
                visibility: new_visibility,
                description: file_data.description,
                category: file_data.category,
                compressed_by: file_data.compressed_by,
                original_size: file_data.original_size,
                final_size: file_data.final_size,
                chunk_size: file_data.chunk_size,
            };
            self.files.entry((sender, file_hash)).write(updated_file);
            
            self.emit(FileUpdatedEvent {
                uri: file_hash,
                owner: sender
            });
        }
        
        fn share_file(
            ref self: ContractState,
            file_hash: felt252,
            shared_with: ContractAddress,
        ) {
            let sender = get_caller_address();
            assert(self.file_exists(sender, file_hash), 'File not found');
            assert(sender != shared_with, 'Cannot share with yourself');
            
            // Check if already shared
            let already_shared = self.file_shares.entry((sender, file_hash, shared_with)).read();
            assert(!already_shared, 'File already shared with user');
            
            // Mark file as shared
            self.file_shares.entry((sender, file_hash, shared_with)).write(true);
            
            // Add to shared_with_user tracking
            let shared_count = self.shared_with_user_count.entry(shared_with).read();
            self.shared_with_user_by_index.entry((shared_with, shared_count)).write((sender, file_hash));
            self.shared_with_user_count.entry(shared_with).write(shared_count + 1);
            
            self.emit(FileSharedEvent {
                uri: file_hash,
                owner: sender,
                shared_with,
            });
        }
        
        fn get_user_files(self: @ContractState, user: ContractAddress) -> u32 {
            self.user_file_count.entry(user).read()
        }
        
        fn get_user_file_by_index(self: @ContractState, user: ContractAddress, index: u32) -> felt252 {
            let caller = get_caller_address();
            let file_hash = self.user_file_by_index.entry((user, index)).read();
            
            // If caller is the owner, they can access all their files
            if caller == user {
                return file_hash;
            }
            
            // For non-owners, check if the file is accessible based on visibility
            let file_data = self.files.entry((user, file_hash)).read();
            
            match file_data.visibility {
                Visibility::Public => (),
                Visibility::Private => {
                    assert(false, 'Access denied: private file');
                },
                Visibility::Shared => {
                    // Check if file is shared with caller
                    let is_shared_with = self.file_shares.entry((user, file_hash, caller)).read();
                    if !is_shared_with {
                        assert(false, 'Access denied: unauthorized');
                    }
                }
            }

            file_hash
        }
        
        fn get_file_data(self: @ContractState, owner: ContractAddress, file_hash: felt252) -> FileMetadata {
            let caller = get_caller_address();
            let file_data = self.files.entry((owner, file_hash)).read();
            
            // Check if file exists
            assert(file_data.uri != 0, 'File not found');
            
            // Check access permissions based on visibility
            match file_data.visibility {
                Visibility::Public => {},
                Visibility::Private => {
                    assert(caller == owner, 'Access denied: private file');
                },
                Visibility::Shared => {
                    // Owner or users the file is shared with can access
                    let is_owner = caller == owner;
                    let is_shared_with = self.file_shares.entry((owner, file_hash, caller)).read();
                    assert(is_owner || is_shared_with, 'Access denied: unauthorized');
                }
            }
            
            file_data
        }
        
        fn file_exists(self: @ContractState, owner: ContractAddress, file_hash: felt252) -> bool {
            let file_data = self.files.entry((owner, file_hash)).read();
            file_data.uri != 0
        }
        
        fn is_admin(self: @ContractState, user: ContractAddress) -> bool {
            self.roles.entry(user).read() == Role::Admin
        }
        
        fn add_admin(ref self: ContractState, new_admin: ContractAddress) {
            let sender = get_caller_address();
            assert(self.is_admin(sender), 'Only admin can add admins');
            self.roles.entry(new_admin).write(Role::Admin);
        }
        
        // File sharing query functions
        fn is_file_shared_with(self: @ContractState, owner: ContractAddress, file_hash: felt252, user: ContractAddress) -> bool {
            self.file_shares.entry((owner, file_hash, user)).read()
        }
        
        fn get_shared_files_count(self: @ContractState, user: ContractAddress) -> u32 {
            self.shared_with_user_count.entry(user).read()
        }
        
        fn get_shared_file_by_index(self: @ContractState, user: ContractAddress, index: u32) -> (ContractAddress, felt252) {
            self.shared_with_user_by_index.entry((user, index)).read()
        }
        
        fn store_compression_mapping(
            ref self: ContractState,
            uri: felt252,
            file_format: felt252,
            compressed_by: u8,
            original_size: u32,
            final_size: u32,
            chunk_size: u32,
            chunk_mappings: Array<felt252>,
            chunk_values: Array<u8>,
            byte_mappings: Array<u8>,
            byte_values: Array<felt252>,
            reconstruction_steps: Array<felt252>,
            metadata: Array<felt252>,
        ) {
            assert(compressed_by <= 100, 'Invalid compression percentage');
            
            self.emit(CompressionMappingStoredEvent {
                uri,
                file_format,
                compressed_by,
                original_size,
                final_size,
            });
        }
    }
}

#[cfg(test)]
mod tests;

