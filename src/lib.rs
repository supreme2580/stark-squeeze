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
pub use compression::{compress_file, decompress_data, CompressionResult, CompressionMapping};
pub use dictionary::{Dictionary, FIRST_DICT, SECOND_DICT, DictionaryError};
pub use encoding::{encoding_one, encoding_two};
pub use starknet_client::{get_all_data, retrieve_data, upload_data};
pub use utils::{binary_to_file, file_to_binary};

// Remove all duplicate imports and function definitions
// The encoding functions are now in the encoding module
// The dictionary and utility functions are now in their respective modules