pub mod ascii_converter;
pub mod cli;
pub mod compression;
pub mod mapping;
pub mod starknet_client;
pub mod utils;

// Re-export commonly used items
pub use ascii_converter::convert_to_printable_ascii;
pub use cli::{main_menu, upload_data_cli};
pub use compression::{compress_file, CompressionMapping};
pub use mapping::{create_complete_mapping, save_mapping, CompleteMapping, MappingError};
pub use starknet_client::upload_data;
pub use utils::short_string_to_felt;

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    main_menu().await;
}
