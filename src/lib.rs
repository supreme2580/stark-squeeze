pub mod ascii_converter;
pub mod cli;
pub mod compression;
pub mod mapping;
pub mod starknet_client;
pub mod utils;
pub mod ipfs_client;
pub mod config;

// Re-export commonly used items
pub use ascii_converter::convert_to_printable_ascii;
pub use cli::{main_menu, upload_data_cli, generate_ultra_compressed_ascii_combinations_cli};
pub use mapping::{MappingError};
pub use starknet_client::upload_data;
pub use utils::short_string_to_felt;
pub use ipfs_client::pin_file_to_ipfs;
pub use config::{get_config, Config, load_config, save_config};


