use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsciiConversionInfo {
    pub conversion_map: HashMap<u8, u8>, // converted -> original
    pub reverse_map: HashMap<u8, u8>,    // original -> converted
    pub stats: ConversionStatsInfo,
    pub was_conversion_needed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionStatsInfo {
    pub total_bytes: usize,
    pub converted_bytes: usize,
    pub conversion_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalMapping {
    pub chunk_size: usize,
    pub code_to_chunk: std::collections::HashMap<u16, Vec<u8>>,
    pub compressed_data: Vec<u8>,
    pub ascii_conversion: Option<AsciiConversionInfo>, // Only if needed
}



#[derive(Debug)]
pub enum MappingError {
    SerializationError(serde_json::Error),
    IoError(std::io::Error),
    InvalidMapping(String),
    ConversionError(String),
}

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MappingError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            MappingError::IoError(e) => write!(f, "IO error: {}", e),
            MappingError::InvalidMapping(msg) => write!(f, "Invalid mapping: {}", msg),
            MappingError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
        }
    }
}

impl Error for MappingError {}

// Ensure MappingError is Send + Sync
unsafe impl Send for MappingError {}
unsafe impl Sync for MappingError {}

impl From<serde_json::Error> for MappingError {
    fn from(err: serde_json::Error) -> Self {
        MappingError::SerializationError(err)
    }
}

impl From<std::io::Error> for MappingError {
    fn from(err: std::io::Error) -> Self {
        MappingError::IoError(err)
    }
}









/// Saves a minimal mapping to a JSON file
pub fn save_minimal_mapping(mapping: &MinimalMapping, file_path: &str) -> Result<(), MappingError> {
    let json_content = serde_json::to_string_pretty(mapping)?;
    fs::write(file_path, json_content)?;
    Ok(())
}

/// Loads a minimal mapping from a JSON file
pub fn load_minimal_mapping(file_path: &str) -> Result<MinimalMapping, MappingError> {
    let mapping_content = fs::read_to_string(file_path)?;
    let mapping: MinimalMapping = serde_json::from_str(&mapping_content)?;
    Ok(mapping)
}

/// Reconstructs the original file from a minimal mapping
pub fn reconstruct_from_minimal_mapping(
    mapping_file_path: &str,
    output_file_path: &str,
) -> Result<(), MappingError> {
    // Load the minimal mapping
    let mapping = load_minimal_mapping(mapping_file_path)?;
    
    // Step 1: Decompress using chunk mapping to get binary string
    let mut binary_string = String::new();
    for &byte in &mapping.compressed_data {
        let chunk = mapping.code_to_chunk.get(&(byte as u16))
            .ok_or_else(|| MappingError::InvalidMapping(format!("Byte {} not found in mapping", byte)))?;
        
        // Convert chunk bytes back to binary string (8-bit representation)
        binary_string.push_str(&vec_u8_to_bin_string(chunk));
    }
    fs::write("debug_reconstructed_binary_string.txt", &binary_string).expect("Failed to write debug_reconstructed_binary_string.txt");
    
    // Step 2: Convert binary string back to ASCII bytes
    let mut ascii_bytes = Vec::new();
    for chunk in binary_string.as_bytes().chunks(8) {
        if chunk.len() == 8 {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit == b'1' {
                    byte |= 1 << (7 - i);
                }
            }
            ascii_bytes.push(byte);
        }
    }
    fs::write("debug_reconstructed_ascii.bin", &ascii_bytes).expect("Failed to write debug_reconstructed_ascii.bin");
    
    // Step 3: Reverse ASCII conversion if needed
    let mut original_bytes = ascii_bytes;
    if let Some(ascii_info) = &mapping.ascii_conversion {
        for byte in &mut original_bytes {
            if let Some(&original_byte) = ascii_info.conversion_map.get(byte) {
                *byte = original_byte;
            }
        }
    }
    
    // Write the reconstructed file
    fs::write(output_file_path, original_bytes)?;
    
    Ok(())
}

/// Shows information about a minimal mapping file
pub fn analyze_minimal_mapping(mapping_file_path: &str) -> Result<(), MappingError> {
    let mapping = load_minimal_mapping(mapping_file_path)?;
    
    println!("🗜️  Minimal Mapping File Analysis:");
    println!("==================================");
    println!("✅ File Information:");
    println!("  • Chunk size: {}", mapping.chunk_size);
    println!("  • Number of unique chunks: {}", mapping.code_to_chunk.len());
    println!("  • Compressed data size: {} bytes", mapping.compressed_data.len());
    println!("  • ASCII conversion needed: {}", mapping.ascii_conversion.is_some());
    
    if let Some(ascii_info) = &mapping.ascii_conversion {
        println!("  • ASCII conversion percentage: {:.2}%", ascii_info.stats.conversion_percentage);
    }
    
    // Calculate estimated original size
    let estimated_original_size = mapping.compressed_data.len() * mapping.chunk_size;
    println!("  • Estimated original size: {} bytes", estimated_original_size);
    
    // Calculate compression ratio
    let compression_ratio = mapping.compressed_data.len() as f64 / estimated_original_size as f64;
    println!("  • Compression ratio: {:.2}%", compression_ratio * 100.0);
    
    println!("\n🎉 Reconstruction Capability:");
    println!("  ✅ This file contains ALL data needed for reconstruction!");
    println!("  ✅ You can reconstruct the original file using just this file.");
    println!("  ✅ No additional files are needed.");
    
    println!("\n💡 How to use:");
    println!("  • Use the CLI option 'Reconstruct File (from mapping)'");
    println!("  • Or call the reconstruction function directly");
    println!("  • The file will be automatically decompressed and restored");
    
    Ok(())
}

fn vec_u8_to_bin_string(chunk: &Vec<u8>) -> String {
    chunk.iter().map(|b| format!("{:08b}", b)).collect::<Vec<_>>().join("")
}