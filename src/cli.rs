use crate::starknet_client::upload_data;
use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::path::Path;
use std::time::Duration;
use sha2::{Sha256, Digest};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::ascii_converter::convert_to_printable_ascii;
use crate::mapping::{create_complete_mapping, save_mapping};

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

    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer).await {
        print_error("Failed to read file", &e);
        return;
    }

    // Convert to printable ASCII with detailed tracking
    let (ascii_buffer, ascii_stats) = match convert_to_printable_ascii(&buffer) {
        Ok(result) => result,
        Err(e) => {
            print_error("Failed to convert file to ASCII", &e);
            return;
        }
    };

    // Convert ASCII buffer to binary string
    let binary_string: String = ascii_buffer.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Compress the data
    let bytes = binary_string.as_bytes();
    let result = match crate::compression::compress_file(bytes) {
        Ok(result) => result,
        Err(e) => {
            print_error("Failed in compression step", &e);
            return;
        }
    };
    let encoded_data = result.compressed_data;
    let mapping = result.mapping;

    // Calculate sizes and ratios
    let original_size = binary_string.len() as u64;
    let compressed_size = encoded_data.len() as u64;
    let compression_ratio = ((compressed_size as f64 / original_size as f64) * 100.0) as u64;

    // Generate hash from the compressed data
    let mut hasher = Sha256::new();
    hasher.update(&encoded_data);
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

    spinner.set_message("Uploading data...".yellow().to_string());
    if let Err(e) = upload_data(compressed_size, &file_type, original_size).await {
        print_error("Failed to upload data", &e);
        return;
    }

    spinner.finish_with_message("Upload complete!".green().to_string());

    // Create comprehensive mapping for lossless reversal
    let complete_mapping = match create_complete_mapping(
        mapping,
        &ascii_stats,
        &file_path,
        &upload_id.to_string(),
        &buffer,
    ) {
        Ok(mapping) => mapping,
        Err(e) => {
            print_error("Failed to create mapping", &e);
            return;
        }
    };

    // Save the comprehensive mapping to a JSON file
    let mapping_file = format!("{}.mapping.json", file_path);
    if let Err(e) = save_mapping(&complete_mapping, &mapping_file) {
        print_error("Failed to save mapping file", &e);
    } else {
        println!("📝 Mapping saved to: {}", mapping_file);
    }

    // Display results
    print_info("Upload ID:", upload_id);
    let original_mb = buffer.len() as f64 / 1_000_000.0;
    let compressed_mb = compressed_size as f64 / 1_000_000.0;
    let reduction = 100.0 - compression_ratio as f64;
    print_info("File Size:", format!("Reduced {:.1}% (from {:.2}MB to {:.2}MB)", 
        reduction, original_mb, compressed_mb));
    let ratio_colored = if compression_ratio > 100 {
        format!("{:.1}%", compression_ratio).red().bold()
    } else {
        format!("{:.1}%", compression_ratio).green().bold()
    };
    print_info("Compression Ratio:", ratio_colored);
    
    if ascii_stats.converted_bytes > 0 {
        print_info("ASCII Conversion:", format!("{} bytes converted ({:.1}%)", 
            ascii_stats.converted_bytes, 
            (ascii_stats.converted_bytes as f64 / ascii_stats.total_bytes as f64) * 100.0));
    }
}

/// Displays the CLI menu and handles command routing
pub async fn main_menu() {
    loop {
        println!("\n{}", "🚀 Welcome to StarkSqueeze CLI!".bold().cyan());
        println!("{}", "Please choose an option:".bold());

        let options = vec!["Upload Data", "Retrieve File", "Get All Files IDs", "Get All Files", "Exit"];
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
            1 => {
                println!("\n{}", "🔧 Retrieve File - Coming Soon!".yellow().bold());
                println!("This feature will allow you to retrieve files using their upload ID.");
                println!("Press Enter to continue...");
                let _ = std::io::stdin().read_line(&mut String::new());
            },
            2 => {
                println!("\n{}", "📋 Get All Files IDs - Coming Soon!".yellow().bold());
                println!("This feature will list all uploaded file IDs.");
                println!("Press Enter to continue...");
                let _ = std::io::stdin().read_line(&mut String::new());
            },
            3 => {
                println!("\n{}", "📁 Get All Files - Coming Soon!".yellow().bold());
                println!("This feature will list all uploaded files with details.");
                println!("Press Enter to continue...");
                let _ = std::io::stdin().read_line(&mut String::new());
            },
            4 => {
                println!("{}", "👋 Goodbye!".bold().green());
                break;
            }
            _ => unreachable!(),
        }
    }
}
