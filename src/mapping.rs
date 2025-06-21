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
pub struct CompleteMapping {
    pub version: String,
    pub file_info: FileInfo,
    pub compression_mapping: CompressionMapping,
    pub ascii_conversion: AsciiConversionInfo,
    pub reversal_instructions: ReversalInstructions,
    pub metadata: HashMap<String, String>,
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

/// Loads a complete mapping from a JSON file
pub fn load_mapping(file_path: &str) -> Result<CompleteMapping, MappingError> {
    let content = fs::read_to_string(file_path)?;
    let mapping: CompleteMapping = serde_json::from_str(&content)?;
    Ok(mapping)
}

/// Performs complete reversal using the mapping
pub fn reverse_compression(
    compressed_data: &[u8],
    mapping: &CompleteMapping,
) -> Result<Vec<u8>, MappingError> {
    // Step 1: Decompress chunks
    let binary_string = decompress_chunks(compressed_data, &mapping.compression_mapping)?;
    
    // Step 2: Convert binary string to ASCII bytes
    let ascii_bytes = binary_string_to_ascii(&binary_string)?;
    
    // Step 3: Reverse ASCII conversion if needed
    let original_bytes = if mapping.ascii_conversion.was_conversion_needed {
        reverse_ascii_conversion(&ascii_bytes, &mapping.ascii_conversion.conversion_map)?
    } else {
        ascii_bytes
    };

    Ok(original_bytes)
}

/// Decompresses data using chunk mapping
fn decompress_chunks(
    compressed_data: &[u8],
    mapping: &CompressionMapping,
) -> Result<String, MappingError> {
    let mut binary_string = String::new();
    
    for &byte in compressed_data {
        let chunk = mapping.byte_to_chunk.get(&byte)
            .ok_or_else(|| MappingError::InvalidMapping(
                format!("No mapping found for byte: 0x{:02x}", byte)
            ))?;
        
        // Convert chunk bytes back to binary string
        for &chunk_byte in chunk {
            binary_string.push(if chunk_byte == 48 { '0' } else { '1' });
        }
    }
    
    Ok(binary_string)
}

/// Converts binary string back to ASCII bytes
fn binary_string_to_ascii(binary_string: &str) -> Result<Vec<u8>, MappingError> {
    let mut ascii_bytes = Vec::new();
    
    for chunk in binary_string.as_bytes().chunks(8) {
        if chunk.len() == 8 {
            let byte_str: String = chunk.iter()
                .map(|&b| if b == 48 { '0' } else { '1' })
                .collect();
            
            if let Ok(byte) = u8::from_str_radix(&byte_str, 2) {
                ascii_bytes.push(byte);
            } else {
                return Err(MappingError::ConversionError(
                    format!("Invalid binary string: {}", byte_str)
                ));
            }
        }
    }
    
    Ok(ascii_bytes)
}

/// Reverses ASCII conversion using the conversion map
fn reverse_ascii_conversion(
    converted_bytes: &[u8],
    conversion_map: &HashMap<u8, u8>,
) -> Result<Vec<u8>, MappingError> {
    let mut original_bytes = Vec::with_capacity(converted_bytes.len());
    
    for &converted_byte in converted_bytes {
        let original_byte = conversion_map.get(&converted_byte)
            .copied()
            .unwrap_or(converted_byte); // If no mapping found, assume it wasn't converted
        original_bytes.push(original_byte);
    }
    
    Ok(original_bytes)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compression::{create_chunk_mapping, compress_data};

    #[test]
    fn test_complete_mapping_creation() {
        let test_data = b"Hello, World!";
        let mapping = create_chunk_mapping(test_data, 2).unwrap();
        let compressed = compress_data(test_data, &mapping).unwrap();
        
        let ascii_stats = crate::ascii_converter::ConversionStats {
            total_bytes: test_data.len(),
            converted_bytes: 0,
            character_map: HashMap::new(),
        };

        let complete_mapping = create_complete_mapping(
            mapping,
            &ascii_stats,
            "test.txt",
            "test_id",
            test_data,
        ).unwrap();

        assert_eq!(complete_mapping.file_info.original_size, test_data.len());
        assert_eq!(complete_mapping.reversal_instructions.total_steps, 2); // No ASCII conversion needed
    }

    #[test]
    fn test_reversal_with_ascii_conversion() {
        let test_data = vec![0, 72, 101, 108, 108, 111]; // NULL + "Hello"
        let mapping = create_chunk_mapping(&test_data, 2).unwrap();
        let compressed = compress_data(&test_data, &mapping).unwrap();
        
        let mut ascii_stats = crate::ascii_converter::ConversionStats {
            total_bytes: test_data.len(),
            converted_bytes: 1,
            character_map: HashMap::new(),
        };
        ascii_stats.character_map.insert(0, 1);

        let complete_mapping = create_complete_mapping(
            mapping,
            &ascii_stats,
            "test.bin",
            "test_id",
            &test_data,
        ).unwrap();

        assert_eq!(complete_mapping.reversal_instructions.total_steps, 3); // ASCII conversion needed
        
        // Test reversal
        let reversed = reverse_compression(&compressed, &complete_mapping).unwrap();
        assert_eq!(reversed, test_data);
    }
} 