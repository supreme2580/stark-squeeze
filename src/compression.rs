use std::collections::HashMap;
use sha2::{Sha256, Digest};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("Failed to compress data: {0}")]
    CompressionError(String),
    #[error("Failed to decompress data: {0}")]
    DecompressionError(String),
}

/// Represents the compression result containing the compressed data and the mapping
#[derive(Debug)]
pub struct CompressionResult {
    pub compressed_data: Vec<u8>,
    pub chunk_mapping: HashMap<Vec<u8>, u8>,
    pub hash: String,
}

/// Compresses data by replacing unique chunks with single bytes
pub fn compress_data(data: &[u8], chunk_size: usize) -> Result<CompressionResult, CompressionError> {
    if data.is_empty() {
        return Err(CompressionError::CompressionError("Empty data provided".into()));
    }

    if chunk_size == 0 {
        return Err(CompressionError::CompressionError("Invalid chunk size".into()));
    }

    // Create a mapping of unique chunks to single bytes
    let mut chunk_mapping: HashMap<Vec<u8>, u8> = HashMap::new();
    let mut current_byte: u8 = 0;

    // First pass: identify all unique chunks
    for chunk in data.chunks(chunk_size) {
        if !chunk_mapping.contains_key(chunk) {
            chunk_mapping.insert(chunk.to_vec(), current_byte);
            current_byte = current_byte.checked_add(1).ok_or_else(|| {
                CompressionError::CompressionError("Too many unique chunks for u8".into())
            })?;
        }
    }

    // Second pass: replace chunks with their byte representations
    let mut compressed_data = Vec::new();
    for chunk in data.chunks(chunk_size) {
        if let Some(&byte) = chunk_mapping.get(chunk) {
            compressed_data.push(byte);
        } else {
            return Err(CompressionError::CompressionError(
                "Failed to find chunk in mapping".into(),
            ));
        }
    }

    // Generate a hash of the mapping for verification
    let mut hasher = Sha256::new();
    for (chunk, byte) in chunk_mapping.iter() {
        hasher.update(chunk);
        hasher.update(&[*byte]);
    }
    let hash = format!("{:x}", hasher.finalize());

    Ok(CompressionResult {
        compressed_data,
        chunk_mapping,
        hash,
    })
}

/// Decompresses data using the provided mapping
pub fn decompress_data(
    compressed_data: &[u8],
    chunk_mapping: &HashMap<Vec<u8>, u8>,
    hash: &str,
) -> Result<Vec<u8>, CompressionError> {
    // Verify the hash of the mapping
    let mut hasher = Sha256::new();
    for (chunk, byte) in chunk_mapping.iter() {
        hasher.update(chunk);
        hasher.update(&[*byte]);
    }
    let computed_hash = format!("{:x}", hasher.finalize());
    
    if computed_hash != hash {
        return Err(CompressionError::DecompressionError(
            "Hash verification failed".into(),
        ));
    }

    // Create reverse mapping
    let mut reverse_mapping: HashMap<u8, &Vec<u8>> = HashMap::new();
    for (chunk, byte) in chunk_mapping.iter() {
        reverse_mapping.insert(*byte, chunk);
    }

    // Decompress the data
    let mut decompressed_data = Vec::new();
    for &byte in compressed_data {
        if let Some(chunk) = reverse_mapping.get(&byte) {
            decompressed_data.extend_from_slice(chunk);
        } else {
            return Err(CompressionError::DecompressionError(
                format!("Invalid byte in compressed data: {}", byte),
            ));
        }
    }

    Ok(decompressed_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_decompression() {
        let test_data = b"Hello, World! Hello, World!";
        let chunk_size = 4;

        let compression_result = compress_data(test_data, chunk_size).unwrap();
        let decompressed_data = decompress_data(
            &compression_result.compressed_data,
            &compression_result.chunk_mapping,
            &compression_result.hash,
        ).unwrap();

        assert_eq!(test_data, decompressed_data.as_slice());
    }

    #[test]
    fn test_empty_data() {
        let result = compress_data(b"", 4);
        assert!(matches!(result, Err(CompressionError::CompressionError(_))));
    }

    #[test]
    fn test_invalid_chunk_size() {
        let result = compress_data(b"test", 0);
        assert!(matches!(result, Err(CompressionError::CompressionError(_))));
    }
} 