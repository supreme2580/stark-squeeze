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
            0 => upload_data_cli(None).await,
            1 => retrieve_data_cli(None).await,
            2 => list_all_uploads().await,
            3 => {
                println!("{}", "👋 Goodbye!".bold().green());
                break;
            }
            _ => unreachable!(),
        }use crate::starknet_client::{get_all_data, retrieve_data, upload_data};
use colored::*;
use dialoguer::{Input, Select, Confirm};
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::path::Path;
use std::time::Duration;
use sha2::{Sha256, Digest};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::{encoding_one, encoding_two};
use crate::ascii_converter::convert_file_to_ascii;

/// Compression metrics for detailed reporting
#[derive(Debug, Clone)]
pub struct CompressionMetrics {
    pub raw_size: u64,
    pub ascii_size: u64,
    pub first_encoding_size: u64,
    pub final_size: u64,
    pub verbose: bool,
}

impl CompressionMetrics {
    pub fn new(verbose: bool) -> Self {
        Self {
            raw_size: 0,
            ascii_size: 0,
            first_encoding_size: 0,
            final_size: 0,
            verbose,
        }
    }

    /// Calculate compression ratio for a given stage
    fn calculate_ratio(&self, current_size: u64, base_size: u64) -> f64 {
        if base_size == 0 {
            return 0.0;
        }
        (current_size as f64 / base_size as f64) * 100.0
    }

    /// Format bytes into human-readable units
    fn format_bytes(&self, bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    /// Color-code ratio based on compression effectiveness
    fn colorize_ratio(&self, ratio: f64) -> ColoredString {
        let ratio_str = format!("{:.1}%", ratio);
        if ratio > 100.0 {
            ratio_str.red().bold()
        } else {
            ratio_str.green().bold()
        }
    }

    /// Display the detailed compression report
    pub fn display_report(&self) {
        println!("\n{}", "═══════════════════════════════════════".cyan().bold());
        println!("{}", "         COMPRESSION REPORT            ".cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan().bold());

        // Main compression stages
        let ascii_ratio = self.calculate_ratio(self.ascii_size, self.raw_size);
        let first_encoding_ratio = self.calculate_ratio(self.first_encoding_size, self.raw_size);
        let final_ratio = self.calculate_ratio(self.final_size, self.raw_size);

        println!("{:<15} {} ({})", 
            "Input:".blue().bold(), 
            self.format_bytes(self.raw_size),
            "raw".dimmed()
        );
        
        println!("{:<15} {} ({})", 
            "ASCII:".blue().bold(), 
            self.format_bytes(self.ascii_size),
            self.colorize_ratio(ascii_ratio)
        );
        
        println!("{:<15} {} ({})", 
            "5-bit Chunks:".blue().bold(), 
            self.format_bytes(self.first_encoding_size),
            self.colorize_ratio(first_encoding_ratio)
        );
        
        println!("{:<15} {} ({})", 
            "Final:".blue().bold(), 
            self.format_bytes(self.final_size),
            self.colorize_ratio(final_ratio)
        );

        // Overall compression summary
        println!("{} {}", 
            "→ Overall:".yellow().bold(), 
            format!("{} of original size", self.colorize_ratio(final_ratio))
        );

        // Space savings
        let space_saved = self.raw_size.saturating_sub(self.final_size);
        let space_saved_ratio = self.calculate_ratio(space_saved, self.raw_size);
        
        println!("{} {} ({})", 
            "Space Saved:".green().bold(), 
            self.format_bytes(space_saved),
            format!("{:.1}%", space_saved_ratio).green().bold()
        );

        // Verbose mode details
        if self.verbose {
            self.display_verbose_details();
        }

        println!("{}", "═══════════════════════════════════════".cyan().bold());
    }

    /// Display verbose bit-level efficiency details
    fn display_verbose_details(&self) {
        println!("\n{}", "┌─ VERBOSE DETAILS ─┐".yellow().bold());
        
        // Bit-level calculations
        let raw_bits = self.raw_size * 8;
        let ascii_bits = self.ascii_size * 8;
        let first_encoding_bits = self.first_encoding_size * 8;
        let final_bits = self.final_size * 8;

        println!("{}", "Bit-level Analysis:".yellow().bold());
        println!("  Raw bits:          {:>12}", format!("{:,}", raw_bits).dimmed());
        println!("  ASCII bits:        {:>12}", format!("{:,}", ascii_bits).dimmed());
        println!("  First encoding:    {:>12}", format!("{:,}", first_encoding_bits).dimmed());
        println!("  Final bits:        {:>12}", format!("{:,}", final_bits).dimmed());

        // Compression efficiency per stage
        println!("\n{}", "Stage-by-stage Efficiency:".yellow().bold());
        
        let ascii_efficiency = if self.raw_size > 0 {
            ((self.raw_size as f64 - self.ascii_size as f64) / self.raw_size as f64) * 100.0
        } else { 0.0 };
        
        let first_encoding_efficiency = if self.ascii_size > 0 {
            ((self.ascii_size as f64 - self.first_encoding_size as f64) / self.ascii_size as f64) * 100.0
        } else { 0.0 };
        
        let final_encoding_efficiency = if self.first_encoding_size > 0 {
            ((self.first_encoding_size as f64 - self.final_size as f64) / self.first_encoding_size as f64) * 100.0
        } else { 0.0 };

        println!("  ASCII Conversion:  {:>9}", 
            if ascii_efficiency >= 0.0 { 
                format!("{:.1}%", ascii_efficiency).green() 
            } else { 
                format!("{:.1}%", ascii_efficiency.abs()).red() 
            }
        );
        
        println!("  First Encoding:    {:>9}", 
            if first_encoding_efficiency >= 0.0 { 
                format!("{:.1}%", first_encoding_efficiency).green() 
            } else { 
                format!("{:.1}%", first_encoding_efficiency.abs()).red() 
            }
        );
        
        println!("  Final Encoding:    {:>9}", 
            if final_encoding_efficiency >= 0.0 { 
                format!("{:.1}%", final_encoding_efficiency).green() 
            } else { 
                format!("{:.1}%", final_encoding_efficiency.abs()).red() 
            }
        );

        // Theoretical limits and recommendations
        println!("\n{}", "Analysis:".yellow().bold());
        if final_bits > raw_bits {
            println!("  ⚠️  Final size larger than input - consider different encoding for this file type");
        } else if self.calculate_ratio(self.final_size, self.raw_size) > 80.0 {
            println!("  ℹ️  Low compression ratio - file may already be compressed or encrypted");
        } else if self.calculate_ratio(self.final_size, self.raw_size) < 20.0 {
            println!("  ✅ Excellent compression achieved!");
        } else {
            println!("  ✅ Good compression ratio achieved");
        }
        
        println!("{}", "└─────────────────────┘".yellow().bold());
    }
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

/// Prompts for verbose mode
async fn prompt_verbose_mode() -> bool {
    match Confirm::new()
        .with_prompt("Enable verbose mode for detailed bit-level analysis?")
        .default(false)
        .interact()
    {
        Ok(verbose) => verbose,
        Err(_) => false,
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

/// Uploads a file with compression metadata and detailed reporting
pub async fn upload_data_cli(file_path_arg: Option<std::path::PathBuf>) {
    // Check if user wants verbose mode
    let verbose = prompt_verbose_mode().await;
    let mut metrics = CompressionMetrics::new(verbose);

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

    // Track raw input size
    metrics.raw_size = buffer.len() as u64;

    // Convert to printable ASCII before hashing and compression
    println!("\n🔄 Converting file to printable ASCII...");
    let ascii_buffer = match convert_file_to_ascii(buffer) {
        Ok(converted) => converted,
        Err(e) => {
            print_error("Failed to convert file to ASCII", &e);
            return;
        }
    };

    // Track ASCII size
    metrics.ascii_size = ascii_buffer.len() as u64;

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

    // Validate file size
    if metrics.ascii_size == 0 {
        print_error("Invalid file", &"File is empty after ASCII conversion");
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
    spinner.set_message("First encoding step (5-bit chunks)...".yellow().to_string());
    let encoded_one = match encoding_one(&binary_string).await {
        Ok(encoded) => encoded,
        Err(e) => {
            print_error("Failed in first encoding step", &e);
            return;
        }
    };

    // Track first encoding size (estimate from string length)
    metrics.first_encoding_size = (encoded_one.len() / 8) as u64; // Convert bits to bytes estimate

    // Second encoding step
    spinner.set_message("Second encoding step (final compression)...".yellow().to_string());
    let encoded_two = match encoding_two(&encoded_one).await {
        Ok(encoded) => encoded,
        Err(e) => {
            print_error("Failed in second encoding step", &e);
            return;
        }
    };

    // Track final compressed size
    metrics.final_size = encoded_two.len() as u64;

    // Calculate compression ratio for upload
    let compression_ratio = metrics.calculate_ratio(metrics.final_size, metrics.raw_size);

    spinner.set_message("Uploading compressed data...".yellow().to_string());
    if let Err(e) = upload_data(metrics.final_size, &file_type, metrics.raw_size).await {
        print_error("Failed to upload data", &e);
        return;
    }

    spinner.finish_with_message("Upload complete!".green().to_string());

    // Display detailed compression report
    metrics.display_report();

    // Display upload information
    println!("\n{}", "Upload Information:".cyan().bold());
    print_info("Upload ID:", upload_id);
    print_info("File Type:", file_type);
}

/// Retrieves previously uploaded data with enhanced reporting
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
            println!("\n{}", "═══════════════════════════════════════".cyan().bold());
            println!("{}", "         RETRIEVAL REPORT              ".cyan().bold());
            println!("{}", "═══════════════════════════════════════".cyan().bold());
            
            println!("{}", "Decoded binary status: Success".green().bold());
            
            // Create metrics for display
            let mut metrics = CompressionMetrics::new(false);
            metrics.raw_size = original_size;
            metrics.final_size = compressed_size;
            
            print_info("Upload ID:", upload_id);
            print_info("File Type:", file_type);
            print_info("Original Size:", metrics.format_bytes(original_size));
            print_info("Compressed Size:", metrics.format_bytes(compressed_size));
            print_info("Compression Ratio:", format!("{}%", compression_ratio));
            
            let space_saved = original_size.saturating_sub(compressed_size);
            let space_saved_ratio = metrics.calculate_ratio(space_saved, original_size);
            print_info("Space Saved:", format!("{} ({:.1}%)", 
                metrics.format_bytes(space_saved), 
                space_saved_ratio
            ));
            
            println!("{}", "═══════════════════════════════════════".cyan().bold());
        }
        Err(e) => {
            print_error("Failed to retrieve data", &e);
            println!("Hint: Ensure the upload ID is correct and try again.");
        }
    }
}

/// Lists all uploaded files with enhanced formatting
pub async fn list_all_uploads() {
    match get_all_data().await {
        Ok(data) => {
            if data.is_empty() {
                println!("{}", "No uploads found.".yellow().bold());
            } else {
                println!("\n{}", "═══════════════════════════════════════".cyan().bold());
                println!("{}", "           ALL UPLOADS                 ".cyan().bold());
                println!("{}", "═══════════════════════════════════════".cyan().bold());
                
                for (i, (upload_id, file_type, compression_ratio)) in data.iter().enumerate() {
                    println!("{} #{}", "Upload".blue().bold(), i + 1);
                    print_info("ID:", upload_id);
                    print_info("File Type:", file_type);
                    
                    let ratio_colored = if *compression_ratio > 100.0 {
                        format!("{:.1}%", compression_ratio).red().bold()
                    } else {
                        format!("{:.1}%", compression_ratio).green().bold()
                    };
                    print_info("Compression Ratio:", ratio_colored);
                    
                    if i < data.len() - 1 {
                        println!("{}", "───────────────────────────────────────".dimmed());
                    }
                }
                println!("{}", "═══════════════════════════════════════".cyan().bold());
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
        println!("{}", "Enhanced with detailed compression reporting".dimmed());
        println!("{}", "Please choose an option:".bold());

        let options = vec![
            "Upload Data (with compression report)",
            "Retrieve Data", 
            "Get All Data", 
            "Exit"
        ];
        
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
    }
}
