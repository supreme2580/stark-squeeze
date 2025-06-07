use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CompressionMapping {
    pub chunk_size: usize,
    pub chunk_to_byte: HashMap<Vec<u8>, u8>,
    pub byte_to_chunk: HashMap<u8, Vec<u8>>,
    pub compression_ratio: f64,
}

#[derive(Debug)]
pub struct CompressionResult {
    pub compressed_data: Vec<u8>,
    pub mapping: CompressionMapping,
}

#[derive(Debug)]
pub enum CompressionError {
    InvalidChunkSize,
    CompressionFailed,
    Custom(String),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompressionError::InvalidChunkSize => write!(f, "Invalid chunk size"),
            CompressionError::CompressionFailed => write!(f, "Compression failed"),
            CompressionError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for CompressionError {}

/// Finds the optimal chunk size that gives >90% compression
pub fn find_optimal_chunk_size(data: &[u8]) -> Result<usize, CompressionError> {
    let mut best_chunk_size = 1;
    let mut best_ratio = 1.0;
    
    // Try chunk sizes from 2 to 8 bytes
    for chunk_size in 2..=8 {
        let chunks: Vec<&[u8]> = data.chunks(chunk_size).collect();
        let unique_chunks: std::collections::HashSet<&[u8]> = chunks.iter().copied().collect();
        
        // Calculate compression ratio
        let original_size = data.len();
        let compressed_size = chunks.len(); // Just the encoded data size
        let ratio = compressed_size as f64 / original_size as f64;
        
        println!("Chunk size {}: {} unique chunks, ratio: {:.2}", 
            chunk_size, unique_chunks.len(), ratio);
        
        if ratio < best_ratio {
            best_ratio = ratio;
            best_chunk_size = chunk_size;
        }
        
        // If we achieve >90% compression, we can stop
        if ratio <= 0.1 {
            return Ok(chunk_size);
        }
    }
    
    Ok(best_chunk_size)
}

/// Creates a mapping for unique chunks
pub fn create_chunk_mapping(data: &[u8], chunk_size: usize) -> Result<CompressionMapping, CompressionError> {
    let chunks: Vec<&[u8]> = data.chunks(chunk_size).collect();
    let unique_chunks: Vec<&[u8]> = chunks.iter()
        .copied()
        .collect::<std::collections::HashSet<&[u8]>>()
        .into_iter()
        .collect();
    
    if unique_chunks.len() > 255 {
        return Err(CompressionError::Custom(format!(
            "Too many unique chunks ({}), maximum is 255", 
            unique_chunks.len()
        )));
    }
    
    let mut chunk_to_byte = HashMap::new();
    let mut byte_to_chunk = HashMap::new();
    
    for (i, chunk) in unique_chunks.iter().enumerate() {
        let byte = i as u8;
        chunk_to_byte.insert(chunk.to_vec(), byte);
        byte_to_chunk.insert(byte, chunk.to_vec());
    }
    
    let original_size = data.len();
    let compressed_size = chunks.len(); // Just the encoded data size
    let compression_ratio = compressed_size as f64 / original_size as f64;
    
    Ok(CompressionMapping {
        chunk_size,
        chunk_to_byte,
        byte_to_chunk,
        compression_ratio,
    })
}

/// Compresses data using the chunk mapping
pub fn compress_data(data: &[u8], mapping: &CompressionMapping) -> Result<Vec<u8>, CompressionError> {
    let chunks: Vec<&[u8]> = data.chunks(mapping.chunk_size).collect();
    let mut compressed = Vec::with_capacity(chunks.len());
    
    for chunk in chunks {
        let byte = mapping.chunk_to_byte.get(chunk)
            .ok_or_else(|| CompressionError::Custom(format!(
                "Chunk not found in mapping: {:?}", chunk
            )))?;
        compressed.push(*byte);
    }
    
    Ok(compressed)
}

/// Decompresses data using the chunk mapping
pub fn decompress_data(compressed: &[u8], mapping: &CompressionMapping) -> Result<Vec<u8>, CompressionError> {
    let mut decompressed = Vec::with_capacity(compressed.len() * mapping.chunk_size);
    
    for &byte in compressed {
        let chunk = mapping.byte_to_chunk.get(&byte)
            .ok_or_else(|| CompressionError::Custom(format!(
                "Byte not found in mapping: {}", byte
            )))?;
        decompressed.extend_from_slice(chunk);
    }
    
    Ok(decompressed)
}

/// Main compression function that handles the entire process
pub fn compress_file(data: &[u8]) -> Result<CompressionResult, CompressionError> {
    // Find optimal chunk size
    let chunk_size = find_optimal_chunk_size(data)?;
    println!("\nSelected chunk size: {}", chunk_size);
    
    // Create mapping
    let mapping = create_chunk_mapping(data, chunk_size)?;
    println!("\nCreated mapping with {} unique chunks", mapping.chunk_to_byte.len());
    println!("Compression ratio: {:.2}", mapping.compression_ratio);
    
    // Compress data
    let compressed_data = compress_data(data, &mapping)?;
    
    Ok(CompressionResult {
        compressed_data,
        mapping,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression() {
        let data = b"Hello, World! Hello, World! Hello, World!";
        let result = compress_file(data).unwrap();
        
        // Verify compression ratio
        assert!(result.mapping.compression_ratio < 1.0);
        
        // Verify we can decompress
        let decompressed = decompress_data(&result.compressed_data, &result.mapping).unwrap();
        assert_eq!(decompressed, data);
    }
} 