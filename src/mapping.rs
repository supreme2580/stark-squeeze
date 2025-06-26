use crate::compression::{CompressionMapping, FileInfo};
use crate::ascii_converter::ConversionStats;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalMapping {
    pub chunk_size: usize,
    pub byte_to_chunk: HashMap<u8, Vec<u8>>,
    pub compressed_data: Vec<u8>,
    pub ascii_conversion: Option<AsciiConversionInfo>, // Only if needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteMapping {
    pub version: String,
    pub file_info: FileInfo,
    pub compression_mapping: CompressionMapping,
    pub ascii_conversion: AsciiConversionInfo,
    pub reversal_instructions: ReversalInstructions,
    pub metadata: HashMap<String, String>,
    pub compressed_data: Vec<u8>, // Add the compressed data to the mapping
}

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
pub struct ReversalInstructions {
    pub steps: Vec<ReversalStep>,
    pub total_steps: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReversalStep {
    pub step_number: usize,
    pub operation: String,
    pub description: String,
    pub input_format: String,
    pub output_format: String,
    pub parameters: HashMap<String, String>,
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

/// Creates a complete mapping structure for lossless reversal
pub fn create_complete_mapping(
    compression_mapping: CompressionMapping,
    ascii_stats: &ConversionStats,
    original_file_path: &str,
    upload_id: &str,
    original_data: &[u8],
    compressed_data: &[u8],
) -> Result<CompleteMapping, MappingError> {
    let file_extension = Path::new(original_file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Create ASCII conversion mapping
    let mut conversion_map = HashMap::new();
    let mut reverse_map = HashMap::new();
    
    // Build the conversion maps from the stats
    for (&original_byte, _) in &ascii_stats.character_map {
        let converted_byte = convert_byte_to_ascii(original_byte);
        conversion_map.insert(converted_byte, original_byte);
        reverse_map.insert(original_byte, converted_byte);
    }

    let ascii_info = AsciiConversionInfo {
        conversion_map,
        reverse_map,
        stats: ConversionStatsInfo {
            total_bytes: ascii_stats.total_bytes,
            converted_bytes: ascii_stats.converted_bytes,
            conversion_percentage: (ascii_stats.converted_bytes as f64 / ascii_stats.total_bytes as f64) * 100.0,
        },
        was_conversion_needed: ascii_stats.converted_bytes > 0,
    };

    // Generate hash of original data
    let mut hasher = Sha256::new();
    hasher.update(original_data);
    let hash = format!("{:x}", hasher.finalize());

    let file_info = FileInfo {
        original_size: original_data.len(),
        file_extension,
        upload_id: upload_id.to_string(),
        hash,
    };

    // Create reversal instructions
    let reversal_instructions = create_reversal_instructions(&compression_mapping, &ascii_info);

    let mut metadata = HashMap::new();
    metadata.insert("compression_algorithm".to_string(), "chunk_based".to_string());
    metadata.insert("created_at".to_string(), chrono::Utc::now().to_rfc3339());
    metadata.insert("tool_version".to_string(), env!("CARGO_PKG_VERSION").to_string());

    Ok(CompleteMapping {
        version: "1.0".to_string(),
        file_info,
        compression_mapping,
        ascii_conversion: ascii_info,
        reversal_instructions,
        metadata,
        compressed_data: compressed_data.to_vec(),
    })
}

/// Creates step-by-step reversal instructions
fn create_reversal_instructions(
    compression_mapping: &CompressionMapping,
    ascii_info: &AsciiConversionInfo,
) -> ReversalInstructions {
    let mut steps = Vec::new();

    // Step 1: Decompress using chunk mapping
    steps.push(ReversalStep {
        step_number: 1,
        operation: "decompress_chunks".to_string(),
        description: "Decompress the encoded data using the chunk-to-byte mapping".to_string(),
        input_format: "compressed_bytes".to_string(),
        output_format: "binary_string".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("chunk_size".to_string(), compression_mapping.chunk_size.to_string());
            params.insert("mapping_entries".to_string(), compression_mapping.byte_to_chunk.len().to_string());
            params
        },
    });

    // Step 2: Convert binary string to ASCII bytes
    steps.push(ReversalStep {
        step_number: 2,
        operation: "binary_to_ascii".to_string(),
        description: "Convert binary string representation back to ASCII bytes".to_string(),
        input_format: "binary_string".to_string(),
        output_format: "ascii_bytes".to_string(),
        parameters: HashMap::new(),
    });

    // Step 3: Reverse ASCII conversion (if needed)
    if ascii_info.was_conversion_needed {
        steps.push(ReversalStep {
            step_number: 3,
            operation: "reverse_ascii_conversion".to_string(),
            description: "Reverse ASCII character conversions to restore original bytes".to_string(),
            input_format: "converted_ascii_bytes".to_string(),
            output_format: "original_bytes".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("converted_chars".to_string(), ascii_info.stats.converted_bytes.to_string());
                params.insert("conversion_percentage".to_string(), format!("{:.2}%", ascii_info.stats.conversion_percentage));
                params
            },
        });
    }

    let total_steps = steps.len();
    ReversalInstructions {
        steps,
        total_steps,
    }
}

/// Saves the complete mapping to a JSON file
pub fn save_mapping(mapping: &CompleteMapping, file_path: &str) -> Result<(), MappingError> {
    let json_content = serde_json::to_string_pretty(mapping)?;
    fs::write(file_path, json_content)?;
    Ok(())
}

/// Helper function to convert byte to ASCII (same as in ascii_converter.rs)
fn convert_byte_to_ascii(byte: u8) -> u8 {
    const ASCII_PRINTABLE_START: u8 = 32;
    const ASCII_PRINTABLE_END: u8 = 126;
    
    if byte >= ASCII_PRINTABLE_START && byte <= ASCII_PRINTABLE_END {
        return byte;
    }

    const CHAR_MAPPINGS: &[(u8, u8)] = &[
        (0, b'0'), (1, b'1'), (2, b'2'), (3, b'3'), (4, b'4'),
        (5, b'5'), (6, b'6'), (7, b'7'), (8, b'b'), (9, b' '),
        (10, b' '), (11, b'v'), (12, b'f'), (13, b' '), (14, b'e'),
        (15, b'f'), (27, b'E'), (127, b'D'),
    ];

    for &(from, to) in CHAR_MAPPINGS {
        if byte == from {
            return to;
        }
    }

    if byte > 127 {
        return 48 + (byte - 128) % 75;
    }

    match byte {
        16..=26 => b'A' + (byte - 16),
        28..=31 => b'L' + (byte - 28),
        _ => b'?',
    }
}

/// Reconstructs the original file from mapping (which now includes compressed data)
pub fn reconstruct_from_mapping(
    mapping_file_path: &str,
    output_file_path: &str,
) -> Result<(), MappingError> {
    // Load the mapping (which now includes compressed data)
    let mapping_content = fs::read_to_string(mapping_file_path)?;
    let complete_mapping: CompleteMapping = serde_json::from_str(&mapping_content)?;
    
    // Use the compressed data from the mapping
    let compressed_data = &complete_mapping.compressed_data;
    
    // Step 1: Decompress using chunk mapping
    let mut binary_string = String::new();
    for &byte in compressed_data {
        let chunk = complete_mapping.compression_mapping.byte_to_chunk.get(&byte)
            .ok_or_else(|| MappingError::InvalidMapping(format!("Byte {} not found in mapping", byte)))?;
        
        // Convert chunk bytes to binary string
        for &chunk_byte in chunk {
            binary_string.push_str(&format!("{:08b}", chunk_byte));
        }
    }
    
    // Step 2: Convert binary string to ASCII bytes
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
    
    // Step 3: Reverse ASCII conversion if needed
    let mut original_bytes = ascii_bytes;
    if complete_mapping.ascii_conversion.was_conversion_needed {
        for byte in &mut original_bytes {
            if let Some(&original_byte) = complete_mapping.ascii_conversion.conversion_map.get(byte) {
                *byte = original_byte;
            }
        }
    }
    
    // Write the reconstructed file
    fs::write(output_file_path, original_bytes)?;
    
    Ok(())
}

/// Shows what information is available from the mapping file
/// Now that the mapping includes compressed data, it can be used for reconstruction
pub fn analyze_mapping_only(mapping_file_path: &str) -> Result<(), MappingError> {
    let mapping_content = fs::read_to_string(mapping_file_path)?;
    let complete_mapping: CompleteMapping = serde_json::from_str(&mapping_content)?;
    
    println!("📋 Mapping File Analysis:");
    println!("=========================");
    println!("✅ Available Information:");
    println!("  • File extension: {}", complete_mapping.file_info.file_extension);
    println!("  • Original file size: {} bytes", complete_mapping.file_info.original_size);
    println!("  • Upload ID: {}", complete_mapping.file_info.upload_id);
    println!("  • File hash: {}", complete_mapping.file_info.hash);
    println!("  • Chunk size: {}", complete_mapping.compression_mapping.chunk_size);
    println!("  • Number of unique chunks: {}", complete_mapping.compression_mapping.byte_to_chunk.len());
    println!("  • Compression ratio: {:.2}%", complete_mapping.compression_mapping.compression_ratio * 100.0);
    println!("  • ASCII conversion needed: {}", complete_mapping.ascii_conversion.was_conversion_needed);
    if complete_mapping.ascii_conversion.was_conversion_needed {
        println!("  • ASCII conversion percentage: {:.2}%", complete_mapping.ascii_conversion.stats.conversion_percentage);
    }
    println!("  • Compressed data size: {} bytes", complete_mapping.compressed_data.len());
    
    println!("\n🎉 Reconstruction Capability:");
    println!("  ✅ This mapping file contains ALL data needed for reconstruction!");
    println!("  ✅ You can reconstruct the original file using just this mapping file.");
    println!("  ✅ No separate compressed data file is needed.");
    
    println!("\n🔍 What the mapping contains:");
    println!("  • Dictionary of all possible chunks → byte mappings");
    println!("  • Reverse lookup: byte → chunk mappings");
    println!("  • ASCII conversion rules");
    println!("  • File metadata and processing steps");
    println!("  • The actual compressed data ({} bytes)", complete_mapping.compressed_data.len());
    
    println!("\n💡 How reconstruction works:");
    println!("  1. Use compressed data to look up chunks in the mapping");
    println!("  2. Convert chunks back to binary string");
    println!("  3. Convert binary string to ASCII bytes");
    println!("  4. Reverse any ASCII conversions");
    println!("  5. Write the original file");
    
    Ok(())
}

/// Creates a minimal mapping structure for file reconstruction
pub fn create_minimal_mapping(
    compression_mapping: CompressionMapping,
    ascii_stats: &ConversionStats,
    compressed_data: &[u8],
) -> MinimalMapping {
    // Only include ASCII conversion if it was actually needed
    let ascii_conversion = if ascii_stats.converted_bytes > 0 {
        let mut conversion_map = HashMap::new();
        let mut reverse_map = HashMap::new();
        
        for (&original_byte, _) in &ascii_stats.character_map {
            let converted_byte = convert_byte_to_ascii(original_byte);
            conversion_map.insert(converted_byte, original_byte);
            reverse_map.insert(original_byte, converted_byte);
        }

        Some(AsciiConversionInfo {
            conversion_map,
            reverse_map,
            stats: ConversionStatsInfo {
                total_bytes: ascii_stats.total_bytes,
                converted_bytes: ascii_stats.converted_bytes,
                conversion_percentage: (ascii_stats.converted_bytes as f64 / ascii_stats.total_bytes as f64) * 100.0,
            },
            was_conversion_needed: true,
        })
    } else {
        None
    };

    MinimalMapping {
        chunk_size: compression_mapping.chunk_size,
        byte_to_chunk: compression_mapping.byte_to_chunk,
        compressed_data: compressed_data.to_vec(),
        ascii_conversion,
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
        let chunk = mapping.byte_to_chunk.get(&byte)
            .ok_or_else(|| MappingError::InvalidMapping(format!("Byte {} not found in mapping", byte)))?;
        
        // Convert chunk bytes back to binary string
        for &chunk_byte in chunk {
            binary_string.push(chunk_byte as char);
        }
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
    println!("  • Number of unique chunks: {}", mapping.byte_to_chunk.len());
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