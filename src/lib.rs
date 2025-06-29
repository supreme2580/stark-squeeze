pub mod ascii_converter;
pub mod cli;
pub mod compression;
pub mod mapping;
pub mod starknet_client;
pub mod utils;
pub mod ipfs_client;

// Re-export commonly used items
pub use ascii_converter::convert_to_printable_ascii;
pub use cli::{main_menu, upload_data_cli, generate_ultra_compressed_ascii_combinations_cli, read_ultra_compressed_file_cli, compress_file_with_dictionary_cli};
pub use compression::{compress_file, CompressionMapping};
pub use mapping::{create_complete_mapping, save_mapping, CompleteMapping, MappingError};
pub use starknet_client::upload_data;
pub use utils::short_string_to_felt;
pub use ipfs_client::pin_file_to_ipfs;

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Check if --generate flag is provided (JSON format with 90% compression)
    if args.len() > 1 && args[1] == "--generate" {
        generate_ultra_compressed_ascii_combinations_cli().await;
    } else if args.len() > 1 && args[1] == "--read" {
        read_ultra_compressed_file_cli().await;
    } else if args.len() > 1 && args[1] == "--compress" {
        compress_file_with_dictionary_cli().await;
    } else {
        main_menu().await;
    }
}
