use crate::starknet_client::{get_all_data, retrieve_data, upload_data};
use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::path::Path;
use std::time::Duration;
use sha2::{Sha256, Digest};
use crate::{encoding_one, encoding_two, file_to_binary};

/// Prints a styled error message
fn print_error(context: &str, error: &dyn std::fmt::Display) {
    eprintln!("{} {}: {}", "Error".red().bold(), context, error);
}

/// Prints a styled info message
fn print_info(label: &str, value: impl std::fmt::Display) {
    println!("{} {}", label.blue().bold(), value);
}

/// Prompts the user for string input
async fn prompt_string(prompt: &str) -> String {
    loop {
        match Input::<String>::new().with_prompt(prompt).interact_text() {
            Ok(value) => return value,
            Err(e) => print_error("Failed to read input", &e),
        }
    }
}

/// Uploads a file with compression metadata
pub async fn upload_data_cli() {
    let file_path = prompt_string("Enter the file path").await;

    // Read file contents and generate hash
    let binary_data = match file_to_binary(&file_path) {
        Ok(data) => data,
        Err(e) => {
            print_error("Failed to read file", &e);
            return;
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(&binary_data);
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
    let original_size = binary_data.len() as u64;
    let file_type = match Path::new(&file_path).extension() {
        Some(ext) => ext.to_string_lossy().to_string(),
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

    // Convert file contents to binary string
    let binary_string: String = binary_data.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();

    // First encoding step
    spinner.set_message("First encoding step...".yellow().to_string());
    let encoded_one = match encoding_one(&binary_string) {
        Ok(encoded) => encoded,
        Err(e) => {
            print_error("Failed in first encoding step", &e);
            return;
        }
    };

    // Second encoding step
    spinner.set_message("Second encoding step...".yellow().to_string());
    let encoded_two = match encoding_two(&encoded_one) {
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
    print_info("Compression Ratio:", format!("{}%", compression_ratio));
}

/// Retrieves previously uploaded data
pub async fn retrieve_data_cli() {
    let upload_id = loop {
        let input = prompt_string("Enter the upload ID or hash").await;
        match FieldElement::from_hex_be(&input) {
            Ok(val) => break val,
            Err(e) => print_error("Invalid hex input for upload ID", &e),
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
                    print_info("Compression Ratio:", format!("{}%", compression_ratio));
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
            0 => upload_data_cli().await,
            1 => retrieve_data_cli().await,
            2 => list_all_uploads().await,
            3 => {
                println!("{}", "👋 Goodbye!".bold().green());
                break;
            }
            _ => unreachable!(),
        }
    }
}
