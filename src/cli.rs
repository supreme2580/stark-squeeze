use crate::starknet_client::{get_all_data, retrieve_data, upload_data};
use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::path::Path;
use std::time::Duration;
use sha2::{Sha256, Digest};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::{encoding_one, encoding_two};
use crate::ascii_converter::convert_file_to_ascii;
use std::env;

/// The default maximum file size allowed for upload (10 MiB).
const DEFAULT_MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

/// Returns the maximum file size allowed, checking the optional
/// `STARKSQUEEZE_MAX_FILE_SIZE` environment variable (bytes).
fn get_max_file_size() -> u64 {
    env::var("STARKSQUEEZE_MAX_FILE_SIZE")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(DEFAULT_MAX_FILE_SIZE)
}

/// Prints a styled error message
fn print_error(context: &str, error: &dyn std::fmt::Display) {
    eprintln!("{} {}: {}", "Error".red().bold(), context, error);
}

/// Prints a styled info message
fn print_info(label: &str, value: impl std::fmt::Display) {
    println!("{} {}", label.blue().bold(), value);
}

/// Prompts the user for string input with optional validation
async fn prompt_string(prompt: &str) -> String {
    loop {
        match Input::<String>::new().with_prompt(prompt).interact_text() {
            Ok(value) => {
                if value.trim().is_empty() {
                    print_error("Invalid input", &"Input cannot be empty");
                    continue;
                }
                return value;
            },
            Err(e) => print_error("Failed to read input", &e),
        }
    }
}

/// Validates that a string is not empty
#[allow(dead_code)]
fn validate_non_empty(input: &str, field_name: &str) -> Result<(), String> {
    if input.trim().is_empty() {
        return Err(format!("{} cannot be empty", field_name));
    }
    Ok(())
}

/// Uploads a file with compression metadata, enforcing a maximum file size limit
pub async fn upload_data_cli(file_path_arg: Option<std::path::PathBuf>) {
    // Use the provided file path or prompt for one
    let file_path = match file_path_arg {
        Some(path) => path.to_string_lossy().to_string(),
        None => prompt_string("Enter the file path").await,
    };

    // Validate the file path and size using async metadata calls
    let path = Path::new(&file_path);
    let metadata = match tokio::fs::metadata(&path).await {
        Ok(md) if md.is_file() => md,
        Ok(_) => {
            print_error("Invalid file path", &format!("{} is not a regular file", file_path));
            return;
        }
        Err(e) => {
            print_error("Invalid file path", &e);
            return;
        }
    };

    // Enforce maximum file size limit BEFORE opening/reading the file
    let max_size = get_max_file_size();
    if metadata.len() > max_size {
        print_error(
            "File too large",
            &format!(
                "File size ({} bytes) exceeds the allowed limit of {} bytes. \
nSet STARKSQUEEZE_MAX_FILE_SIZE to override.",
                metadata.len(), max_size
            ),
        );
        return;
    }

    // Read file contents and generate hash asynchronously
    let mut file = match File::open(&file_path).await {
        Ok(f) => f,
        Err(e) => {
            print_error("Failed to open file", &e);
            return;
        }
    };

    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer).await {
        print_error("Failed to read file", &e);
        return;
    }

    // Convert to printable ASCII before hashing and compression
    println!("\n🔄 Converting file to printable ASCII...");
    let ascii_buffer = match convert_file_to_ascii(buffer) {
        Ok(converted) => converted,
        Err(e) => {
            print_error("Failed to convert file to ASCII", &e);
            return;
        }
    };

    hasher.update(&ascii_buffer);
    let hash = hasher.finalize();

    // Convert first 16 bytes of hash to FieldElement
    let upload_id = match FieldElement::from_byte_slice_be(&hash[..16]) {
        Ok(id) => id,
        Err(e) => {
            print_error("Failed to generate upload ID", &e);
            return;
        }
    };

    // Automatically determine file size and type
    let original_size = ascii_buffer.len() as u64;
    if original_size == 0 {
        print_error("Invalid file", &"File is empty");
        return;
    }

    // Validate the file has a valid extension
    let file_type = match Path::new(&file_path).extension() {
        Some(ext) => {
            let ext_str = ext.to_string_lossy().to_string();
            if ext_str.is_empty() {
                print_error("Invalid file type", &"File extension is empty");
                return;
            }
            ext_str
        }
        None => {
            print_error("Failed to determine file type", &"No file extension found");
            return;
        }
    };

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Convert ASCII buffer to binary string
    let binary_string: String = ascii_buffer.iter().map(|&byte| format!("{:08b}", byte)).collect();

    // First encoding step
    spinner.set_message("First encoding step...".yellow().to_string());
    let encoded_one = match encoding_one(&binary_string).await {
        Ok(encoded) => encoded,
        Err(e) => {
            print_error("Failed in first encoding step", &e);
            return;
        }
    };

    // Second encoding step
    spinner.set_message("Second encoding step...".yellow().to_string());
    let encoded_two = match encoding_two(&encoded_one).await {
        Ok(encoded) => encoded,
        Err(e) => {
            print_error("Failed in second encoding step", &e);
            return;
        }
    };

    // Calculate sizes and ratios
    let compressed_size = encoded_two.len() as u64;
    let compression_ratio = ((compressed_size as f64 / original_size as f64) * 100.0) as u64;

    // Upload the data
    spinner.set_message("Uploading data...".yellow().to_string());
    if let Err(e) = upload_data(compressed_size, &file_type, original_size).await {
        print_error("Failed to upload data", &e);
        return;
    }

    spinner.finish_with_message("Upload complete!".green().to_string());

    print_info("Upload ID:", upload_id);
    print_info("Original Size:", format!("{} bytes", original_size));
    print_info("New Size:", format!("{} bytes", compressed_size));
    print_info("Compression Ratio:", format!("{}%", compression_ratio));
}

/// Retrieves previously uploaded data
pub async fn retrieve_data_cli(id_arg: Option<String>) {
    let upload_id = match id_arg {
        Some(id) => {
            // The ID has already been validated in main.rs, but we still need to convert it to FieldElement
            match FieldElement::from_hex_be(&id) {
                Ok(val) => val,
                Err(e) => {
                    print_error("Invalid hex input for upload ID", &e);
                    return;
                }
            }
        }
        None => {
            // Interactive mode - prompt for ID and validate
            loop {
                let input = prompt_string("Enter the upload ID or hash").await;

                // Validate ID format before trying to convert
                if !input.starts_with("0x") && input.len() != 66 {
                    print_error(
                        "Invalid upload ID format",
                        &format!(
                            "Expected 0x-prefixed 64-character hex string, got: {}",
                            input
                        ),
                    );
                    continue;
                }

                // Check for valid hex characters
                if !input[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                    print_error(
                        "Invalid upload ID",
                        &format!(
                            "Upload ID contains non-hexadecimal characters: {}",
                            input
                        ),
                    );
                    continue;
                }

                match FieldElement::from_hex_be(&input) {
                    Ok(val) => break val,
                    Err(e) => print_error("Invalid hex input for upload ID", &e),
                }
            }
        }
    };

    match retrieve_data(upload_id).await {
        Ok((original_size, compressed_size, file_type, compression_ratio)) => {
            println!("{}", "Decoded binary status: Success".green().bold());
            print_info("File Type:", file_type);
            print_info("Original Size:", format!("{} bytes", original_size));
            print_info("Compressed Size:", format!("{} bytes", compressed_size));
            print_info("Compression Ratio:", format!("{}%", compression_ratio));
        }
        Err(e) => {
            print_error("Failed to retrieve data", &e);
            println!("Hint: Ensure the upload ID is correct and try again.");
        }
    }
}
