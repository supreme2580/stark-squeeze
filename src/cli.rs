use crate::starknet_client::{get_all_data, retrieve_data, upload_data};
use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::path::Path;
use std::time::Duration;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;
use crate::{encoding_one, encoding_two};

/// Prints a styled error message
fn print_error(context: &str, error: &dyn std::fmt::Display) {
    eprintln!("{} {}: {}", "Error".red().bold(), context, error);
}

/// Prints a styled info message
fn print_info(label: &str, value: impl std::fmt::Display) {
    println!("{} {}", label.blue().bold(), value);
}

///Prints a styled warning message
fn print_warning(message: &str) {
    println!("{} {}", "⚠️ Warning".red().bold(), message);
}

/// Analyzes file contents for non-ASCII bytes and returns detection results
fn analyze_non_ascii_bytes(buffer: &[u8]) -> Option<NonAsciiAnalysis> {
    let mut non_ascii_positions = Vec::new();
    let mut byte_values = Vec::new();

    for (pos, &byte) in buffer.iter().enumerate(){
        if byte > 127 {
            non_ascii_positions.push(pos);
            byte_values.push(byte);

            // Limit collection to first 10 occurences
            if non_ascii_positions.len() >= 10 {
                break;
            }
        }
    }

    if non_ascii_positions.is_empty() {
        None
    } else {
        Some(NonAsciiAnalysis { positions: non_ascii_positions, byte_values, total_non_ascii: buffer.iter().filter(|&&b| b > 127).count(), })
    }
}

/// Contains analysis results for non-ASCII byte detection
struct NonAsciiAnalysis {
    positions: Vec<usize>,
    byte_values: Vec<u8>,
    total_non_ascii: usize,
}


/// Displays a comprehensive warning about non-ASCII bytes in the file
fn display_non_ascii_warning(analysis: &NonAsciiAnalysis, file_path: &str) {
    println!("\n{}", "═".repeat(60).yellow());
    print_warning("Non-ASCII bytes detected in your file!");
    println!();
    
    println!("{}", "📋 File Analysis:".bold());
    print_info("  File:", file_path);
    print_info("  Total non-ASCII bytes found:", analysis.total_non_ascii);
    
    println!("\n{}", "🔍 Sample Locations:".bold());
    for (i, (&pos, &byte_val)) in analysis.positions.iter().zip(analysis.byte_values.iter()).enumerate() {
        println!("  • Position {}: byte value {} (0x{:02X})", pos, byte_val, byte_val);
        if i >= 4 { // Show max 5 examples
            if analysis.positions.len() > 5 {
                println!("  • ... and {} more", analysis.positions.len() - 5);
            }
            break;
        }
    }
    
    println!("\n{}", "⚡ Potential Impacts:".bold());
    println!("  • May cause encoding/decoding errors in CLI tools");
    println!("  • Could lead to unexpected behavior during processing");
    println!("  • Might affect data integrity in text-based operations");
    println!("  • Can cause compatibility issues across different systems");
    
    println!("\n{}", "💡 Recommendations:".bold());
    println!("  • Review your file content for unexpected characters");
    println!("  • Consider converting to ASCII-only format if possible");
    println!("  • Verify that non-ASCII content is intentional and necessary");
    println!("  • Test thoroughly if proceeding with mixed content");
    
    println!("{}", "═".repeat(60).yellow());
    println!();
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
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            print_error("Failed to open file", &e);
            return;
        }
    };

    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        print_error("Failed to read file", &e);
        return;
    }
    hasher.update(&buffer);
    let hash = hasher.finalize();

    // Check for non-ASCII bytes and warn user
    if let Some(analysis) = analyze_non_ascii_bytes(&buffer) {
        display_non_ascii_warning(&analysis, &file_path);
        
        // Ask user if they want to continue
        let options = vec!["Continue with upload", "Cancel and review file"];
        let selection = match Select::new()
            .with_prompt("How would you like to proceed?")
            .items(&options)
            .default(0)
            .interact()
        {
            Ok(sel) => sel,
            Err(e) => {
                print_error("Selection failed", &e);
                return;
            }
        };
        
        match selection {
            0 => {
                println!("{}", "⚠️  Proceeding with upload despite non-ASCII content...".yellow());
            }
            1 => {
                println!("{}", "📝 Upload cancelled. Please review your file and try again.".blue());
                return;
            }
            _ => unreachable!(),
        }
    } else {
        println!("{}", "✅ File contains only ASCII bytes - no compatibility issues detected.".green());
    }
    
    // Convert first 16 bytes of hash to FieldElement
    let upload_id = match FieldElement::from_byte_slice_be(&hash[..16]) {
        Ok(id) => id,
        Err(e) => {
            print_error("Failed to generate upload ID", &e);
            return;
        }
    };

    // Automatically determine file size and type
    let original_size = buffer.len() as u64;
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
    let binary_string: String = buffer.iter()
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
