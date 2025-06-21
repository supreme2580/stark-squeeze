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
use crate::encoding_one;
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

    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer).await {
        print_error("Failed to read file", &e);
        return;
    }

    // Convert to printable ASCII with detailed tracking
    println!("\n🔄 Converting file to printable ASCII...");
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

    // First encoding step
    spinner.set_message("Encoding data...".yellow().to_string());
    let (encoded_data, mapping) = match encoding_one(&binary_string).await {
        Ok(result) => result,
        Err(e) => {
            print_error("Failed in encoding step", &e);
            return;
        }
    };

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
        println!("\n📝 Comprehensive mapping saved to: {}", mapping_file);
        println!("   This file contains all information needed for lossless reversal");
    }

    // Also save a human-readable summary
    let summary_file = format!("{}.summary.txt", file_path);
    let summary_content = format!(
        "StarkSqueeze Compression Summary
=====================================

File Information:
- Original File: {}
- Upload ID: {}
- File Type: {}
- Original Size: {} bytes ({:.2} MB)

Compression Details:
- Chunk Size: {}
- Compressed Size: {} bytes ({:.2} MB)
- Compression Ratio: {:.1}%
- Size Reduction: {:.1}%

ASCII Conversion:
- Total Bytes: {}
- Converted Bytes: {} ({:.1}%)
- Conversion Needed: {}

Reversal Information:
- Total Reversal Steps: {}
- Steps: {}

Mapping File: {}
- Format: JSON
- Contains: Complete reversal instructions and mappings

To reverse this compression:
1. Use the mapping file: {}
2. Run: stark_squeeze reverse --mapping {} --input <compressed_file> --output <original_file>

",
        file_path,
        upload_id,
        file_type,
        buffer.len(),
        buffer.len() as f64 / 1_000_000.0,
        complete_mapping.compression_mapping.chunk_size,
        compressed_size,
        compressed_size as f64 / 1_000_000.0,
        compression_ratio,
        100.0 - compression_ratio as f64,
        ascii_stats.total_bytes,
        ascii_stats.converted_bytes,
        (ascii_stats.converted_bytes as f64 / ascii_stats.total_bytes as f64) * 100.0,
        if ascii_stats.converted_bytes > 0 { "Yes" } else { "No" },
        complete_mapping.reversal_instructions.total_steps,
        complete_mapping.reversal_instructions.steps.iter()
            .map(|step| format!("{}. {}", step.step_number, step.operation))
            .collect::<Vec<_>>()
            .join(", "),
        mapping_file,
        mapping_file,
        mapping_file
    );

    if let Err(e) = tokio::fs::write(&summary_file, summary_content).await {
        print_error("Failed to save summary file", &e);
    } else {
        println!("📋 Human-readable summary saved to: {}", summary_file);
    }

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

/// Reverses a compressed file using the comprehensive mapping
pub async fn reverse_data_cli() {
    // Prompt for mapping file
    let mapping_path = prompt_string("Enter the path to the mapping file (.mapping.json)").await;
    let mapping_path = std::path::Path::new(&mapping_path);
    if !tokio::fs::metadata(&mapping_path).await.map(|m| m.is_file()).unwrap_or(false) {
        print_error("Invalid mapping file", &format!("File does not exist or is not a file: {}", mapping_path.display()));
        return;
    }

    // Prompt for compressed file
    let compressed_path = prompt_string("Enter the path to the compressed file").await;
    let compressed_path = std::path::Path::new(&compressed_path);
    if !tokio::fs::metadata(&compressed_path).await.map(|m| m.is_file()).unwrap_or(false) {
        print_error("Invalid compressed file", &format!("File does not exist or is not a file: {}", compressed_path.display()));
        return;
    }

    // Load the comprehensive mapping
    let complete_mapping = match crate::mapping::load_mapping(mapping_path.to_str().unwrap()) {
        Ok(mapping) => mapping,
        Err(e) => {
            print_error("Failed to load mapping file", &e);
            return;
        }
    };

    // Read compressed file
    let compressed_data = match tokio::fs::read(&compressed_path).await {
        Ok(data) => data,
        Err(e) => {
            print_error("Failed to read compressed file", &e);
            return;
        }
    };

    println!("\n🔄 Reversing compression...");
    println!("   File: {}", compressed_path.display());
    println!("   Mapping: {}", mapping_path.display());
    println!("   Reversal steps: {}", complete_mapping.reversal_instructions.total_steps);

    // Show reversal steps
    for step in &complete_mapping.reversal_instructions.steps {
        println!("   {}. {}: {}", step.step_number, step.operation, step.description);
    }

    // Perform the reversal
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    spinner.set_message("Reversing compression...".yellow().to_string());
    let original_data = match crate::mapping::reverse_compression(&compressed_data, &complete_mapping) {
        Ok(data) => data,
        Err(e) => {
            print_error("Failed to reverse compression", &e);
            return;
        }
    };

    spinner.finish_with_message("Reversal complete!".green().to_string());

    // Determine output filename
    let output_path = if let Some(stem) = compressed_path.file_stem() {
        if let Some(stem_str) = stem.to_str() {
            // Try to restore original extension
            let original_ext = &complete_mapping.file_info.file_extension;
            if !original_ext.is_empty() && original_ext != "unknown" {
                compressed_path.with_file_name(format!("{}.{}", stem_str, original_ext))
            } else {
                compressed_path.with_file_name(format!("{}_reversed", stem_str))
            }
        } else {
            compressed_path.with_file_name("reversed_file")
        }
    } else {
        compressed_path.with_file_name("reversed_file")
    };

    // Save the reversed file
    if let Err(e) = tokio::fs::write(&output_path, &original_data).await {
        print_error("Failed to save reversed file", &e);
        return;
    }

    // Verify the file size matches the original
    let expected_size = complete_mapping.file_info.original_size;
    let actual_size = original_data.len();
    
    if expected_size == actual_size {
        println!("\n✅ Reversal successful!");
        print_info("Original file size:", format!("{} bytes", expected_size));
        print_info("Reversed file size:", format!("{} bytes", actual_size));
        print_info("Reversed file saved as:", output_path.display());
        
        // Verify hash if available
        let mut hasher = Sha256::new();
        hasher.update(&original_data);
        let actual_hash = format!("{:x}", hasher.finalize());
        
        if actual_hash == complete_mapping.file_info.hash {
            println!("🔐 Hash verification: {}", "PASSED".green().bold());
        } else {
            println!("⚠️  Hash verification: {}", "FAILED".yellow().bold());
            println!("   Expected: {}", complete_mapping.file_info.hash);
            println!("   Actual:   {}", actual_hash);
        }
    } else {
        println!("\n⚠️  Reversal completed but size mismatch detected");
        print_info("Expected size:", format!("{} bytes", expected_size));
        print_info("Actual size:", format!("{} bytes", actual_size));
        print_info("Reversed file saved as:", output_path.display());
    }
}

/// Displays the CLI menu and handles command routing
pub async fn main_menu() {
    loop {
        println!("\n{}", "🚀 Welcome to StarkSqueeze CLI!".bold().cyan());
        println!("{}", "Please choose an option:".bold());

        let options = vec!["Upload Data", "Retrieve Data", "Get All Data", "Reverse File", "Exit"];
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
            3 => reverse_data_cli().await,
            4 => {
                println!("{}", "👋 Goodbye!".bold().green());
                break;
            }
            _ => unreachable!(),
        }
    }
}
