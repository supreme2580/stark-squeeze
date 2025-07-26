use crate::starknet_client::upload_data;
use colored::*;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::path::Path;
use std::time::Duration;
use std::io::Write;
use sha2::{Sha256, Digest};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::ascii_converter::convert_to_printable_ascii;
use crate::mapping::{reconstruct_from_minimal_mapping, analyze_minimal_mapping};
use hex;
use crate::ipfs_client::pin_file_to_ipfs;
use std::fs;
use serde_json::{Value, json};
use crate::config::get_config;



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
    std::fs::write("debug_original.bin", &buffer).expect("Failed to write debug_original.bin");

    // Convert to printable ASCII with detailed tracking
    let (ascii_buffer, ascii_stats) = match convert_to_printable_ascii(&buffer) {
        Ok(result) => result,
        Err(e) => {
            print_error("Failed to convert file to ASCII", &e);
            return;
        }
    };
    std::fs::write("debug_ascii.bin", &ascii_buffer).expect("Failed to write debug_ascii.bin");

    // Convert ASCII buffer to binary string
    let binary_string: String = ascii_buffer.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();
    std::fs::write("debug_binary_string.txt", &binary_string).expect("Failed to write debug_binary_string.txt");

    let config = get_config();
    let spinner = ProgressBar::new_spinner();
    let tick_strings: Vec<&str> = config.cli.progress.spinner_style.tick_strings.iter().map(|s| s.as_str()).collect();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&tick_strings)
            .template(&config.cli.progress.spinner_style.template)
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(config.cli.progress.spinner_style.steady_tick_ms));

    // Compress the data
    let bytes = binary_string.as_bytes();
    let packed_bytes = match crate::compression::compress_file(&bytes) {
        Ok(packed) => packed,
        Err(e) => {
            print_error("Failed in compression step", &e);
            return;
        }
    };
    // Save packed_bytes to file, use for hashing, IPFS, etc.
    std::fs::write("debug_packed.bin", &packed_bytes).expect("Failed to write debug_packed.bin");

    // Calculate sizes and ratios
    let original_size = binary_string.len() as u64;
    let compressed_size = packed_bytes.len() as u64;
    let compression_ratio = ((compressed_size as f64 / original_size as f64) * 100.0) as u64;

    // Generate hash from the compressed data
    let mut hasher = Sha256::new();
    // Convert encoded_data (Vec<u16>) to Vec<u8> for hashing and other uses
    let encoded_data_bytes: Vec<u8> = packed_bytes.iter().flat_map(|x| x.to_be_bytes()).collect();
    hasher.update(&encoded_data_bytes);
    let hash = hasher.finalize();

    // Use a short hash (first 8 bytes, hex-encoded) as the URI
    let short_hash = hex::encode(&hash[..8]); // 16 hex chars, fits in felt
    let uri = &short_hash;

    // Convert first 16 bytes of hash to FieldElement (for upload_id, if needed)
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
    
    // Prepare data for upload - using minimal data to avoid calldata limits
    let compressed_by = if compression_ratio <= 100 { 
        (100 - compression_ratio) as u8 
    } else { 
        0 
    };
    
    // Remove the call to create_minimal_mapping and any code that tries to use or save a minimal mapping in upload_data_cli.
    let chunk_mappings = vec![FieldElement::from(0u32)]; // Placeholder
    let chunk_values = vec![0u8]; // Placeholder
    let byte_mappings = vec![0u8]; // Placeholder
    let byte_values = vec![FieldElement::from(0u32)]; // Placeholder
    let reconstruction_steps = vec![FieldElement::from(0u32)]; // Placeholder
    let metadata = vec![FieldElement::from(0u32)]; // Placeholder
    
    if let Err(e) = upload_data(
        &uri,
        &file_type,
        compressed_by,
        original_size as usize,
        compressed_size as usize,
        8, // chunk_size
        chunk_mappings,
        chunk_values,
        byte_mappings,
        byte_values,
        reconstruction_steps,
        metadata,
    ).await {
        print_error("Failed to upload data", &e);
        return;
    }

    spinner.finish_with_message(config.ui.messages.upload_complete.green().to_string());

    // IPFS Pinning after upload completion
    println!("\n{}", "🔗 Starting IPFS pinning...".blue().bold());
    
    match pin_file_to_ipfs(&packed_bytes, &format!("{}.compressed", file_path)).await {
        Ok(ipfs_cid) => {
            println!("✅ Pinned to IPFS: {}", ipfs_cid.green().bold());
            println!("🌐 IPFS Gateway: https://gateway.pinata.cloud/ipfs/{}", ipfs_cid);
        }
        Err(e) => {
            println!("❌ IPFS Pin Failed: {}", e.to_string().red().bold());
            println!("💡 Check your PINATA_JWT token in .env file");
        }
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

/// Reconstructs a file from the minimal mapping file
pub async fn reconstruct_from_mapping_cli() {
    let mapping_file_path = prompt_string("Enter the mapping file path (e.g., file.png.map)").await;
    let output_file_path = prompt_string("Enter the output file path (e.g., file.png)").await;

    match reconstruct_from_minimal_mapping(&mapping_file_path, &output_file_path) {
        Ok(_) => println!("✅ File reconstructed successfully: {}", output_file_path),
        Err(e) => print_error("Failed to reconstruct file", &e),
    }
}

/// Analyzes a minimal mapping file to show what information is available
pub async fn analyze_mapping_only_cli() {
    let mapping_file_path = prompt_string("Enter the mapping file path (e.g., file.png.map)").await;

    match analyze_minimal_mapping(&mapping_file_path) {
        Ok(_) => println!("\n✅ Analysis complete!"),
        Err(e) => print_error("Failed to analyze mapping file", &e),
    }
}

/// Generates ASCII character combinations and saves them to a file
pub async fn generate_ascii_combinations_cli() {
    println!("{}", "🔤 ASCII Combination Generator".blue().bold());
    println!();
    
    // Get parameters from user
    let length: usize = match Input::<String>::new()
        .with_prompt("Enter combination length (default: 10)")
        .default("10".to_string())
        .interact_text() {
            Ok(s) => s.parse().unwrap_or(10),
            Err(_) => 10,
    };
    
    let start_index: u64 = match Input::<String>::new()
        .with_prompt("Enter starting index (default: 0)")
        .default("0".to_string())
        .interact_text() {
            Ok(s) => s.parse().unwrap_or(0),
            Err(_) => 0,
    };
    
    // Calculate total possible combinations
    let total_combinations = 128u64.pow(length as u32);
    
    // Ask if user wants to generate all combinations
    let generate_all = match Input::<String>::new()
        .with_prompt("Generate ALL combinations? (y/N)")
        .default("N".to_string())
        .interact_text() {
            Ok(s) => s.to_lowercase() == "y" || s.to_lowercase() == "yes",
            Err(_) => false,
    };
    
    let count = if generate_all {
        total_combinations.saturating_sub(start_index) as usize
    } else {
        match Input::<String>::new()
            .with_prompt("Enter number of combinations to generate (default: 1000)")
            .default("1000".to_string())
            .interact_text() {
                Ok(s) => s.parse().unwrap_or(1000),
                Err(_) => 1000,
        }
    };
    
    let output_file = match Input::<String>::new()
        .with_prompt("Enter output file path (default: ascii_combinations.txt)")
        .default("ascii_combinations.txt".to_string())
        .interact_text() {
            Ok(s) => s,
            Err(_) => "ascii_combinations.txt".to_string(),
    };
    
    println!();
    println!("{}", "📊 Generation Parameters:".yellow().bold());
    print_info("Length", length);
    print_info("Starting index", start_index);
    print_info("Count", count);
    print_info("Output file", &output_file);
    print_info("Total possible combinations", total_combinations);
    
    if generate_all {
        let estimated_size_gb = (count as f64 * (length as f64 + 20.0)) / (1024.0 * 1024.0 * 1024.0);
        println!("{}", "⚠️  WARNING: This will generate a very large file!".red().bold());
        print_info("Estimated file size", format!("{:.2} GB", estimated_size_gb));
        print_info("Estimated time", "Several hours to days depending on your system");
        
        // Calculate more detailed estimates
        let combinations_per_second = 1_000_000; // Conservative estimate
        let estimated_seconds = count as f64 / combinations_per_second as f64;
        let estimated_hours = estimated_seconds / 3600.0;
        let estimated_days = estimated_hours / 24.0;
        
        println!();
        println!("{}", "📊 Detailed Estimates:".yellow().bold());
        print_info("Total combinations to generate", count);
        print_info("Combinations per second (estimate)", combinations_per_second);
        print_info("Estimated time (seconds)", format!("{:.0}", estimated_seconds));
        print_info("Estimated time (hours)", format!("{:.1}", estimated_hours));
        print_info("Estimated time (days)", format!("{:.1}", estimated_days));
        
        // File size breakdown
        let bytes_per_combination = length as f64 + 20.0; // combination + index + formatting
        let total_bytes = count as f64 * bytes_per_combination;
        let size_mb = total_bytes / (1024.0 * 1024.0);
        let size_gb = total_bytes / (1024.0 * 1024.0 * 1024.0);
        let size_tb = total_bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0);
        
        print_info("Bytes per combination", format!("{:.1}", bytes_per_combination));
        print_info("Total bytes", total_bytes as u64);
        print_info("File size (MB)", format!("{:.1}", size_mb));
        print_info("File size (GB)", format!("{:.2}", size_gb));
        if size_tb > 1.0 {
            print_info("File size (TB)", format!("{:.2}", size_tb));
        }
        
        // Storage requirements
        println!();
        println!("{}", "💾 Storage Requirements:".yellow().bold());
        if size_gb > 100.0 {
            println!("{}", "⚠️  You will need significant free disk space!".red().bold());
        }
        print_info("Minimum free space needed", format!("{:.1} GB", size_gb * 1.1)); // 10% buffer
        print_info("Recommended free space", format!("{:.1} GB", size_gb * 2.0)); // 2x buffer
        
        // Time estimates for different systems
        println!();
        println!("{}", "⏱️  Time Estimates by System:".yellow().bold());
        let fast_system = 5_000_000; // 5M combinations/sec
        let medium_system = 1_000_000; // 1M combinations/sec
        let slow_system = 100_000; // 100K combinations/sec
        
        let fast_time = count as f64 / fast_system as f64 / 3600.0;
        let medium_time = count as f64 / medium_system as f64 / 3600.0;
        let slow_time = count as f64 / slow_system as f64 / 3600.0;
        
        print_info("Fast system (5M/sec)", format!("{:.1} hours", fast_time));
        print_info("Medium system (1M/sec)", format!("{:.1} hours", medium_time));
        print_info("Slow system (100K/sec)", format!("{:.1} hours", slow_time));
        
        let confirm = match Input::<String>::new()
            .with_prompt("Are you sure you want to continue? (y/N)")
            .default("N".to_string())
            .interact_text() {
                Ok(s) => s.to_lowercase() == "y" || s.to_lowercase() == "yes",
                Err(_) => false,
        };
        
        if !confirm {
            println!("{}", "Generation cancelled.".yellow().bold());
            return;
        }
    }
    
    if start_index >= total_combinations {
        print_error("Invalid start index", &format!("Start index {} exceeds maximum possible combinations ({})", start_index, total_combinations));
        return;
    }
    
    // Create progress bar
    let progress_bar = ProgressBar::new(count as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    
    // Write to file
    let mut file = match fs::File::create(&output_file) {
        Ok(f) => f,
        Err(e) => {
            print_error("Failed to create output file", &e);
            return;
        }
    };
    
    // Write header
    writeln!(file, "# ASCII Combinations Generated by Stark Squeeze").unwrap();
    writeln!(file, "# Length: {}", length).unwrap();
    writeln!(file, "# Starting index: {}", start_index).unwrap();
    writeln!(file, "# Count: {}", count).unwrap();
    writeln!(file, "# Total possible combinations: {}", total_combinations).unwrap();
    writeln!(file, "# Format: [index] combination").unwrap();
    writeln!(file, "").unwrap();
    
    // Generate combinations in chunks for memory efficiency
    let chunk_size = 100_000; // Process 100k combinations at a time
    let mut current_index = start_index;
    let mut total_generated = 0;
    
    while total_generated < count {
        let remaining = count - total_generated;
        let current_chunk_size = std::cmp::min(chunk_size, remaining);
        
        // Generate current chunk
        let combinations = generate_ascii_combinations(length, current_index, current_chunk_size);
        
        // Write chunk to file
        for (i, combination) in combinations.iter().enumerate() {
            let actual_index = current_index + i as u64;
            writeln!(file, "[{}] {:?}", actual_index, combination).unwrap();
        }
        
        // Update progress
        total_generated += combinations.len();
        current_index += combinations.len() as u64;
        progress_bar.set_position(total_generated as u64);
        
        // Update progress message with current index
        progress_bar.set_message(format!("Current index: {}", current_index));
        
        // Flush file periodically
        if total_generated % (chunk_size * 10) == 0 {
            file.flush().unwrap();
        }
    }
    
    progress_bar.finish_with_message("Generation complete!".green().to_string());
    
    println!();
    println!("{}", "✅ Success!".green().bold());
    print_info("Combinations saved to", &output_file);
    print_info("Total generated", total_generated);
    
    if let Ok(metadata) = fs::metadata(&output_file) {
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
        print_info("File size", format!("{:.2} MB", size_mb));
    }
    
    if generate_all {
        println!();
        println!("{}", "🎉 All possible combinations have been generated!".green().bold());
        println!("This file contains every possible {} character ASCII combination.", length);
    }
}

/// Generates ASCII character combinations of specified length
fn generate_ascii_combinations(length: usize, start_index: u64, count: usize) -> Vec<String> {
    const ASCII_CHARS: usize = 128;
    let mut result = Vec::with_capacity(count);
    
    // Calculate the starting combination from the index
    let mut current_combination = index_to_combination(start_index, length, ASCII_CHARS);
    
    for _ in 0..count {
        result.push(current_combination.clone());
        
        // Generate next combination
        if !increment_combination(&mut current_combination, ASCII_CHARS) {
            // We've reached the end of all possible combinations
            break;
        }
    }
    
    result
}

/// Converts an index to its corresponding combination
fn index_to_combination(mut index: u64, length: usize, base: usize) -> String {
    let mut combination = String::with_capacity(length);
    
    for _ in 0..length {
        let remainder = (index % base as u64) as u8;
        combination.push(remainder as char);
        index /= base as u64;
    }
    
    // Reverse to get correct order (least significant digit first)
    combination.chars().rev().collect()
}

/// Increments a combination to the next one
fn increment_combination(combination: &mut String, base: usize) -> bool {
    let mut chars: Vec<char> = combination.chars().collect();
    
    // Start from the rightmost character
    for i in (0..chars.len()).rev() {
        let current_value = chars[i] as u8;
        
        if current_value < (base - 1) as u8 {
            chars[i] = (current_value + 1) as char;
            *combination = chars.into_iter().collect();
            return true;
        } else {
            // Carry over to next position
            chars[i] = 0 as char;
        }
    }
    
    // If we get here, we've overflowed (all characters are at max value)
    false
}

/// Generates ASCII character combinations in compressed JSON format
pub async fn generate_compressed_ascii_combinations_cli() {
    println!("{}", "🔤 Compressed ASCII Combination Generator".blue().bold());
    println!();
    
    // Get parameters from user
    let length: usize = match Input::<String>::new()
        .with_prompt("Enter combination length (default: 5)")
        .default("5".to_string())
        .interact_text() {
            Ok(s) => s.parse().unwrap_or(5),
            Err(_) => 5,
    };
    
    let start_index: u64 = match Input::<String>::new()
        .with_prompt("Enter starting index (default: 0)")
        .default("0".to_string())
        .interact_text() {
            Ok(s) => s.parse().unwrap_or(0),
            Err(_) => 0,
    };
    
    // Calculate total possible combinations
    let total_combinations = 128u64.pow(length as u32);
    
    // Ask if user wants to generate all combinations
    let generate_all = match Input::<String>::new()
        .with_prompt("Generate ALL combinations? (y/N)")
        .default("N".to_string())
        .interact_text() {
            Ok(s) => s.to_lowercase() == "y" || s.to_lowercase() == "yes",
            Err(_) => false,
    };
    
    let count = if generate_all {
        total_combinations.saturating_sub(start_index) as usize
    } else {
        match Input::<String>::new()
            .with_prompt("Enter number of combinations to generate (default: 1000)")
            .default("1000".to_string())
            .interact_text() {
                Ok(s) => s.parse().unwrap_or(1000),
                Err(_) => 1000,
        }
    };
    
    let output_file = match Input::<String>::new()
        .with_prompt("Enter output file path (default: ascii_combinations.json)")
        .default("ascii_combinations.json".to_string())
        .interact_text() {
            Ok(s) => s,
            Err(_) => "ascii_combinations.json".to_string(),
    };
    
    println!();
    println!("{}", "📊 Generation Parameters:".yellow().bold());
    print_info("Length", length);
    print_info("Starting index", start_index);
    print_info("Count", count);
    print_info("Output file", &output_file);
    print_info("Total possible combinations", total_combinations);
    print_info("Format", "Compressed JSON with 4-byte binary encoding");
    
    if generate_all {
        // Calculate compressed size estimates
        let bytes_per_combination = length as f64; // Just the binary values
        let json_overhead = 20.0; // JSON formatting overhead
        let total_bytes = count as f64 * (bytes_per_combination + json_overhead);
        let size_gb = total_bytes / (1024.0 * 1024.0 * 1024.0);
        
        let combinations_per_second = 500_000; // Conservative estimate for JSON
        let estimated_seconds = count as f64 / combinations_per_second as f64;
        let estimated_hours = estimated_seconds / 3600.0;
        
        println!();
        println!("{}", "📊 Compressed Format Estimates:".yellow().bold());
        print_info("Original size (5-char strings)", format!("{:.1} GB", count as f64 * 25.0 / (1024.0 * 1024.0 * 1024.0)));
        print_info("Compressed size (4-byte binary)", format!("{:.1} GB", size_gb));
        print_info("Compression ratio", format!("{:.1}%", (1.0 - size_gb / (count as f64 * 25.0 / (1024.0 * 1024.0 * 1024.0))) * 100.0));
        print_info("Estimated time", format!("{:.1} hours", estimated_hours));
        
        let confirm = match Input::<String>::new()
            .with_prompt("Are you sure you want to continue? (y/N)")
            .default("N".to_string())
            .interact_text() {
                Ok(s) => s.to_lowercase() == "y" || s.to_lowercase() == "yes",
                Err(_) => false,
        };
        
        if !confirm {
            println!("{}", "Generation cancelled.".yellow().bold());
            return;
        }
    }
    
    if start_index >= total_combinations {
        print_error("Invalid start index", &format!("Start index {} exceeds maximum possible combinations ({})", start_index, total_combinations));
        return;
    }
    
    // Create progress bar
    let progress_bar = ProgressBar::new(count as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    
    // Create JSON structure
    let mut json_data = json!({
        "metadata": {
            "length": length,
            "total_combinations": total_combinations,
            "start_index": start_index,
            "count": count,
            "encoding": "4-byte-binary",
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "compression_ratio": "80% smaller than string format"
        },
        "combinations": []
    });
    
    // Generate combinations in chunks for memory efficiency
    let chunk_size = 10_000; // Smaller chunks for JSON processing
    let mut current_index = start_index;
    let mut total_generated = 0;
    let mut combinations_array = Vec::new();
    
    while total_generated < count {
        let remaining = count - total_generated;
        let current_chunk_size = std::cmp::min(chunk_size, remaining);
        
        // Generate current chunk
        let combinations = generate_ascii_combinations(length, current_index, current_chunk_size);
        
        // Convert to compressed format
        for (i, combination) in combinations.iter().enumerate() {
            let actual_index = current_index + i as u64;
            let binary_values: Vec<u8> = combination.chars().map(|c| c as u8).collect();
            
            combinations_array.push(json!({
                "index": actual_index,
                "value": binary_values
            }));
        }
        
        // Update progress
        total_generated += combinations.len();
        current_index += combinations.len() as u64;
        progress_bar.set_position(total_generated as u64);
        progress_bar.set_message(format!("Current index: {}", current_index));
        
        // Write to file periodically to avoid memory issues
        if total_generated % (chunk_size * 5) == 0 {
            json_data["combinations"] = Value::Array(combinations_array.clone());
            if let Ok(json_string) = serde_json::to_string_pretty(&json_data) {
                fs::write(&output_file, json_string).unwrap();
            }
        }
    }
    
    // Final write
    json_data["combinations"] = Value::Array(combinations_array);
    if let Ok(json_string) = serde_json::to_string_pretty(&json_data) {
        fs::write(&output_file, json_string).unwrap();
    }
    
    progress_bar.finish_with_message("Generation complete!".green().to_string());
    
    println!();
    println!("{}", "✅ Success!".green().bold());
    print_info("Compressed combinations saved to", &output_file);
    print_info("Total generated", total_generated);
    
    if let Ok(metadata) = fs::metadata(&output_file) {
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
        let size_gb = size_mb / 1024.0;
        print_info("File size", format!("{:.2} MB ({:.3} GB)", size_mb, size_gb));
        
        // Calculate compression ratio
        let original_size_mb = total_generated as f64 * 25.0 / (1024.0 * 1024.0);
        let compression_ratio = (1.0 - size_mb / original_size_mb) * 100.0;
        print_info("Compression achieved", format!("{:.1}%", compression_ratio));
    }
    
    if generate_all {
        println!();
        println!("{}", "🎉 All possible combinations have been generated!".green().bold());
        println!("This file contains every possible {} character ASCII combination in compressed format.", length);
    }
    
    // Show sample of the JSON structure
    println!();
    println!("{}", "📋 JSON Structure Sample:".yellow().bold());
    println!("The file contains combinations in this format:");
    println!("{{\"index\": 0, \"value\": [0, 0, 0, 0, 0]}}");
    println!("{{\"index\": 1, \"value\": [0, 0, 0, 0, 1]}}");
    println!("...");
}

/// Generates ASCII character combinations in ultra-compressed JSON format (3:1 compression for fast testing)
pub async fn generate_ultra_compressed_ascii_combinations_cli() {
    let config = get_config();
    println!("{}", "🔤 Ultra-Compressed ASCII Combination Generator (3:1 compression for fast testing)".blue().bold());
    println!();
    
    // Use configuration for optimal settings
    let length = config.dictionary.ultra_compressed.length;
    let start_index = config.dictionary.ultra_compressed.start_index;
    
    // Calculate total possible combinations
    let total_combinations = 128u64.pow(length as u32);
    
    // Always generate all combinations
    let count = total_combinations as usize;
    
    let output_file = "ascii_combinations.json".to_string();
    
    println!();
    println!("{}", "📊 Automatic Generation Parameters:".yellow().bold());
    print_info("Length", length);
    print_info("Starting index", start_index);
    print_info("Count", count);
    print_info("Output file", &output_file);
    print_info("Total possible combinations", total_combinations);
    print_info("Format", "Key-value dictionary");
    print_info("Bytes per combination", format!("{} chars → 1 char ({}:1 ratio)", length, length));
    
    // Calculate size estimates
    let original_size_gb = count as f64 * 5.0 / (1024.0 * 1024.0 * 1024.0);
    let compressed_size_gb = count as f64 * 1.0 / (1024.0 * 1024.0 * 1024.0); // 1 byte per combination
    let json_overhead = 0.2; // 20% JSON overhead
    let final_size_gb = compressed_size_gb * (1.0 + json_overhead);
    
    // Calculate time estimates
    let combinations_per_second = 1_000_000; // Conservative estimate
    let estimated_seconds = count as f64 / combinations_per_second as f64;
    let estimated_hours = estimated_seconds / 3600.0;
    let estimated_days = estimated_hours / 24.0;
    
    println!();
    println!("{}", "📊 Generation Estimates:".yellow().bold());
    print_info("Original size", format!("{:.1} GB", original_size_gb));
    print_info("Compressed size", format!("{:.1} GB", final_size_gb));
    print_info("Compression ratio", format!("{:.1}%", (1.0 - final_size_gb / original_size_gb) * 100.0));
    print_info("Combinations per second", combinations_per_second);
    print_info("Estimated time (seconds)", format!("{:.0}", estimated_seconds));
    print_info("Estimated time (hours)", format!("{:.1}", estimated_hours));
    print_info("Estimated time (days)", format!("{:.1}", estimated_days));
    
    // Storage requirements
    println!();
    println!("{}", "💾 Storage Requirements:".yellow().bold());
    print_info("Minimum free space needed", format!("{:.1} GB", final_size_gb * 1.1));
    print_info("Recommended free space", format!("{:.1} GB", final_size_gb * 2.0));
    
    // Time estimates for different systems
    println!();
    println!("{}", "⏱️  Time Estimates by System:".yellow().bold());
    let fast_system = 5_000_000; // 5M combinations/sec
    let medium_system = 1_000_000; // 1M combinations/sec
    let slow_system = 100_000; // 100K combinations/sec
    
    let fast_time = count as f64 / fast_system as f64 / 3600.0;
    let medium_time = count as f64 / medium_system as f64 / 3600.0;
    let slow_time = count as f64 / slow_system as f64 / 360.0;
    
    print_info("Fast system (5M/sec)", format!("{:.1} hours", fast_time));
    print_info("Medium system (1M/sec)", format!("{:.1} hours", medium_time));
    print_info("Slow system (100K/sec)", format!("{:.1} hours", slow_time));
    
    let confirm = match Input::<String>::new()
        .with_prompt("Generate ALL combinations? (y/N)")
        .default("N".to_string())
        .interact_text() {
            Ok(s) => s.to_lowercase() == "y" || s.to_lowercase() == "yes",
            Err(_) => false,
    };
    
    if !confirm {
        println!("{}", "Generation cancelled.".yellow().bold());
        return;
    }
    
    // Create progress bar
    let progress_bar = ProgressBar::new(count as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    
    // Create JSON structure with key-value dictionary
    let mut json_data = json!({
        "metadata": {
            "length": length,
            "total_combinations": total_combinations,
            "start_index": start_index,
            "count": count,
                    "encoding": &config.dictionary.ultra_compressed.encoding,
        "generated_at": chrono::Utc::now().to_rfc3339(),
        "compression_ratio": &config.dictionary.ultra_compressed.description,
        "generation_time_estimate": format!("{:.1} hours", estimated_hours),
        "file_size_estimate": format!("{:.1} GB", final_size_gb)
        },
        "combinations": {}
    });
    
    // Generate combinations in chunks for memory efficiency
    let chunk_size = 100_000; // Larger chunks for faster generation
    let mut current_index = start_index;
    let mut total_generated = 0;
    let mut combinations_dict = serde_json::Map::new();
    
    while total_generated < count {
        let remaining = count - total_generated;
        let current_chunk_size = std::cmp::min(chunk_size, remaining);
        
        // Generate current chunk
        let combinations = generate_ascii_combinations(length, current_index, current_chunk_size);
        
        // Convert to key-value dictionary format
        for (i, combination) in combinations.iter().enumerate() {
            let actual_index = current_index + i as u64;
            
            // Create key-value pair: combination -> single character
            let key = combination.clone();
            let value = char::from_u32((actual_index % 128) as u32).unwrap_or('.'); // Use ASCII character as value
            
            combinations_dict.insert(key, Value::String(value.to_string()));
        }
        
        // Update progress
        total_generated += combinations.len();
        current_index += combinations.len() as u64;
        progress_bar.set_position(total_generated as u64);
        progress_bar.set_message(format!("Current index: {} ({:.1}%)", current_index, (total_generated as f64 / count as f64) * 100.0));
        
        // Write to file periodically to avoid memory issues
        if total_generated % (chunk_size * 5) == 0 {
            json_data["combinations"] = Value::Object(combinations_dict.clone());
            if let Ok(json_string) = serde_json::to_string(&json_data) {
                fs::write(&output_file, json_string).unwrap();
            }
        }
    }
    
    // Final write
    json_data["combinations"] = Value::Object(combinations_dict);
    if let Ok(json_string) = serde_json::to_string(&json_data) {
        fs::write(&output_file, json_string).unwrap();
    }
    
    progress_bar.finish_with_message("Generation complete!".green().to_string());
    
    println!();
    println!("{}", "✅ Success!".green().bold());
    print_info("Key-value dictionary saved to", &output_file);
    print_info("Total generated", total_generated);
    
    if let Ok(metadata) = fs::metadata(&output_file) {
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
        let size_gb = size_mb / 1024.0;
        print_info("File size", format!("{:.2} MB ({:.3} GB)", size_mb, size_gb));
        
        // Calculate compression ratio
        let original_size_mb = total_generated as f64 * 5.0 / (1024.0 * 1024.0);
        let compression_ratio = (1.0 - size_mb / original_size_mb) * 100.0;
        print_info("Compression achieved", format!("{:.1}%", compression_ratio));
        
        if compression_ratio >= config.dictionary.ultra_compressed.compression_ratio {
            println!("{}", format!("🎉 {:.1}%+ compression achieved!", config.dictionary.ultra_compressed.compression_ratio).green().bold());
        }
    }
    
    println!();
    println!("{}", "🎉 All possible combinations have been generated!".green().bold());
    println!("This file contains every possible {} character ASCII combination in key-value dictionary format.", length);
    
    // Show JSON format info
    println!();
    println!("{}", "📋 JSON Format Info:".yellow().bold());
    println!("File contains:");
    println!("- Metadata with generation info");
    println!("- Key-value dictionary: {{\"combination\": \"single_char\"}}");
    println!("- {:.1}% compression achieved through efficient encoding", config.dictionary.ultra_compressed.compression_ratio);
    println!("- Each {} character combination mapped to single character", length);
    println!("- Ready for file compression using option 8");
}

/// Generates ASCII character combinations in ultra-compressed JSON format (3:1 compression for fast testing)
pub async fn generate_10bit_dictionary_cli() {
    use std::collections::HashMap;
    use std::fs;

    println!("\u{1F522} Generating 10-bit Dictionary (0..1023)");
    let mut dict = HashMap::new();
    for i in 0..1024u16 {
        dict.insert(i, format!("{:010b}", i));
    }
    let json = serde_json::to_string_pretty(&dict).unwrap();
    let filename = "10bit_dictionary.json";
    if let Err(e) = fs::write(filename, json) {
        println!("Failed to write dictionary: {}", e);
        return;
    }
    println!("Dictionary saved to {} ({} entries)", filename, dict.len());
}

/// Decompresses a file using a minimal mapping
pub async fn decompress_file_cli() {
    use std::fs;
    use std::path::Path;
    println!("\u{1F513} Decompress file");
    let compressed_file = prompt_string("Enter compressed file path (.txt)").await;
    let path = Path::new(&compressed_file);
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    // Remove trailing .txt from file_stem if present
    let output_file = if file_stem.ends_with(".txt") {
        &file_stem[..file_stem.len()-4]
    } else {
        file_stem
    };
    println!("Output file will be: {}", output_file);
    // Read compressed data
    let compressed_data = match fs::read(&compressed_file) {
        Ok(data) => data,
        Err(e) => {
            print_error("Failed to read compressed file", &e);
            return;
        }
    };
    // Decompress
    match crate::compression::decompress_file(&compressed_data) {
        Ok(bytes) => {
            if let Err(e) = fs::write(&output_file, &bytes) {
                print_error("Failed to write output file", &e);
                return;
            }
            println!("\u{2705} Decompression complete! Output: {}", output_file);
        }
        Err(e) => {
            print_error("Decompression failed", &e);
        }
    }
}



/// Compresses a file using the bit-packed pipeline
pub async fn compress_file_cli() {
    use std::fs;
    use std::path::Path;
    println!("\u{1F4E6} Compress file");
    let input_file = prompt_string("Enter input file path").await;
    let path = Path::new(&input_file);
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let compressed_file = format!("{}.{}.txt", stem, ext);
    println!("Compressed file will be: {}", compressed_file);
    // Read input data
    let input_data = match fs::read(&input_file) {
        Ok(data) => data,
        Err(e) => {
            print_error("Failed to read input file", &e);
            return;
        }
    };
    // Compress
    let compressed_data = match crate::compression::compress_file(&input_data) {
        Ok(c) => c,
        Err(e) => {
            print_error("Compression failed", &e);
            return;
        }
    };
    // Save compressed data
    if let Err(e) = fs::write(&compressed_file, &compressed_data) {
        print_error("Failed to write compressed file", &e);
        return;
    }
    // Calculate and print compression ratio
    let original_size = input_data.len() as f64;
    let compressed_size = compressed_data.len() as f64;
    let reduction = if original_size > 0.0 {
        100.0 - (compressed_size / original_size * 100.0)
    } else {
        0.0
    };
    println!("\u{2705} Compression complete! Compressed: {}", compressed_file);
    println!("Original size: {:.2} KB, Compressed size: {:.2} KB", original_size / 1024.0, compressed_size / 1024.0);
    println!("Compression: {:.1}% smaller", reduction);
}

/// Displays the CLI menu and handles command routing
pub async fn main_menu() {
    println!("1. Upload data");
    println!("2. Reconstruct from mapping");
    println!("3. Analyze mapping");
    println!("4. Generate 10-bit Dictionary (0..1023)");
    println!("5. Decompress file");
    println!("6. Compress file");
    println!("7. Exit");
    let mut input = String::new();
    print!("Enter your choice (1-7): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => upload_data_cli(None).await,
        "2" => reconstruct_from_mapping_cli().await,
        "3" => analyze_mapping_only_cli().await,
        "4" => generate_10bit_dictionary_cli().await,
        "5" => decompress_file_cli().await,
        "6" => compress_file_cli().await,
        "7" => {
            println!("{}", "\u{1F44B} Goodbye!".bold().green());
            return;
        }
        _ => {
            println!("Invalid choice. Please enter a number between 1 and 7.");
        }
    }
}

