/**
 * Contract ABI for Starksqueeze contract events
 * Generated from the Cairo contract event definitions
 */

export const starksqueezeAbi = [
  {
    "type": "event",
    "name": "contracts::Starksqueeze::CompressionMappingStoredEvent",
    "kind": "struct",
    "members": [
      {
        "name": "uri",
        "type": "core::felt252",
        "kind": "key"
      },
      {
        "name": "file_format",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "compressed_by",
        "type": "core::integer::u8",
        "kind": "data"
      },
      {
        "name": "original_size",
        "type": "core::integer::usize",
        "kind": "data"
      },
      {
        "name": "final_size", 
        "type": "core::integer::usize",
        "kind": "data"
      },
      {
        "name": "chunk_size",
        "type": "core::integer::usize",
        "kind": "data"
      },
      {
        "name": "chunk_mappings",
        "type": "core::array::Array::<core::felt252>",
        "kind": "data"
      },
      {
        "name": "chunk_values",
        "type": "core::array::Array::<core::integer::u8>",
        "kind": "data"
      },
      {
        "name": "byte_mappings",
        "type": "core::array::Array::<core::integer::u8>",
        "kind": "data"
      },
      {
        "name": "byte_values",
        "type": "core::array::Array::<core::felt252>",
        "kind": "data"
      },
      {
        "name": "reconstruction_steps",
        "type": "core::array::Array::<core::felt252>",
        "kind": "data"
      },
      {
        "name": "metadata",
        "type": "core::array::Array::<core::felt252>",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "contracts::Starksqueeze::FileUploadedEvent",
    "kind": "struct",
    "members": [
      {
        "name": "uri",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "owner",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "visibility",
        "type": "contracts::Starksqueeze::Visibility",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "contracts::Starksqueeze::FileUpdatedEvent",
    "kind": "struct", 
    "members": [
      {
        "name": "uri",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "owner",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "contracts::Starksqueeze::FileDeletedEvent",
    "kind": "struct",
    "members": [
      {
        "name": "uri", 
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "owner",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "contracts::Starksqueeze::FileSharedEvent",
    "kind": "struct",
    "members": [
      {
        "name": "uri",
        "type": "core::felt252",
        "kind": "data"
      },
      {
        "name": "owner",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "shared_with",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      }
    ]
  }
] as const;
