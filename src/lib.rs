pub mod ascii_converter;
pub mod cli;
pub mod compression;
pub mod dictionary;
pub mod encoding;
pub mod starknet_client;
pub mod utils;

// Re-export commonly used items
pub use ascii_converter::convert_file_to_ascii;
pub use cli::{main_menu, upload_data_cli, retrieve_data_cli, list_all_uploads};
pub use compression::{compress_data, decompress_data, CompressionResult, CompressionError};
pub use dictionary::{Dictionary, FIRST_DICT, SECOND_DICT, DictionaryError};
pub use encoding::{encoding_one, encoding_two};
pub use starknet_client::{upload_data, retrieve_data, get_all_data};
pub use utils::matches_pattern;

// Remove all duplicate imports and function definitions
// The encoding functions are now in the encoding module
// The dictionary and utility functions are now in their respective modules