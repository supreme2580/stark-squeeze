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

/// Uploads a file with compression metadata
pub async fn upload_data_cli(file_path_arg: Option<std::path::PathBuf>) {
    // Use the provided file path or prompt for one
    let file_path = match file_path_arg {
        Some(path) => path.to_string_lossy().to_string(),
        None => prompt_string("Enter the file path").await,
    };

    // Validate the file path with async file operations
    let path = std::path::Path::new(&file_path);
    if !tokio::fs::metadata(&path).await.map(|m| m.is_file()).unwrap_or(false) {
        print_error("Invalid file path", &format!("File does not exist or is not a file: {}", file_path));
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
        },
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
    let binary_string: String = ascii_buffer.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();

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

    spinner.set_message("Uploading data...".yellow().to_string());
    if let Err(e) = upload_data(compressed_size, &file_type, original_size).await {
        print_error("Failed to upload data", &e);
        return;
    }

    spinner.finish_with_message("Upload complete!".green().to_string());

    print_info("Upload ID:", upload_id);
    print_info("Original Size:", format!("{} bytes", original_size));
    print_info("New Size:", format!("{} bytes", compressed_size));
    let ratio_colored = if compression_ratio > 100 {
        format!("{:.1}%", compression_ratio).red().bold()
    } else {
        format!("{:.1}%", compression_ratio).green().bold()
    };
    print_info("Compression Ratio:", ratio_colored);
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
        },
        None => {
            // Interactive mode - prompt for ID and validate
            loop {
                let input = prompt_string("Enter the upload ID or hash").await;

                // Validate ID format before trying to convert
                if !input.starts_with("0x") && input.len() != 66 {
                    print_error("Invalid upload ID format",
                        &format!("Expected 0x-prefixed 64-character hex string, got: {}", input));
                    continue;
                }

                // Check for valid hex characters
                if !input[2..].chars().all(|c| c.is_ascii_hexdigit()) {
                    print_error("Invalid upload ID",
                        &format!("Upload ID contains non-hexadecimal characters: {}", input));
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
            let ratio_colored = if compression_ratio > 100 {
                format!("{:.1}%", compression_ratio).red().bold()
            } else {
                format!("{:.1}%", compression_ratio).green().bold()
            };
            print_info("Compression Ratio:", ratio_colored);
        }
        Err(e) => {
            print_error("Failed to retrieve data", &e);
            println!("Hint: Ensure the upload ID is correct and try again.");
        }
    }
}

/// Lists all uploaded files
pub async fn list_all_uploads() {
    match get_all_data().await {
        Ok(data) => {
            if data.is_empty() {
                println!("{}", "No uploads found.".yellow().bold());
            } else {
                for (upload_id, file_type, compression_ratio) in data {
                    print_info("ID:", upload_id);
                    print_info("File Type:", file_type);
                    let ratio_colored = if compression_ratio > 100 {
                        format!("{}%", compression_ratio).red().bold()
                    } else {
                        format!("{}%", compression_ratio).green().bold()
                    };
                    print_info("Compression Ratio:", ratio_colored);
                    println!("{}", "---".dimmed());
                }
            }
        }
        Err(e) => {
            print_error("Failed to retrieve uploads", &e);
        }
    }
}

/// Displays the CLI menu and handles command routing
pub async fn main_menu() {
    loop {
        println!("\n{}", "🚀 Welcome to StarkSqueeze CLI!".bold().cyan());
        println!("{}", "Please choose an option:".bold());

        let options = vec!["Upload Data", "Retrieve Data", "Get All Data", "Exit"];
        let selection = match Select::new()
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact()
        {
            Ok(sel) => sel,
            Err(e) => {
                print_error("Selection failed", &e);
                continue;
            }
        };

        match selection {
            0 => upload_data_cli(None).await,
            1 => retrieve_data_cli(None).await,
            2 => list_all_uploads().await,
            3 => {
                println!("{}", "👋 Goodbye!".bold().green());
                break;
            }
            _ => unreachable!(),
        }
    }
}
