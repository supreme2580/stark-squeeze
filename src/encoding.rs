use crate::compression::{compress_file, CompressionResult, CompressionMapping};
use std::error::Error;

/// First encoding step using compression
pub async fn encoding_one(bytes: &str) -> Result<(Vec<u8>, CompressionMapping), Box<dyn Error>> {
    let bytes = bytes.as_bytes();
    let result = compress_file(bytes)?;
    
    println!("\nFirst Encoding Results:");
    println!("Original size: {} bytes", bytes.len());
    println!("Compressed size: {} bytes", result.compressed_data.len());
    println!("Compression ratio: {:.2}", result.mapping.compression_ratio);
    println!("\nChunk Mapping:");
    for (byte, chunk) in &result.mapping.byte_to_chunk {
        println!("0x{:02x} -> {:?}", byte, chunk);
    }
    
    Ok((result.compressed_data, result.mapping))
}

/// Second encoding step using compression
pub async fn encoding_two(data: &[u8]) -> Result<(Vec<u8>, CompressionMapping), Box<dyn Error>> {
    let result = compress_file(data)?;
    
    println!("\nSecond Encoding Results:");
    println!("Original size: {} bytes", data.len());
    println!("Compressed size: {} bytes", result.compressed_data.len());
    println!("Compression ratio: {:.2}", result.mapping.compression_ratio);
    println!("\nChunk Mapping:");
    for (byte, chunk) in &result.mapping.byte_to_chunk {
        println!("0x{:02x} -> {:?}", byte, chunk);
    }
    
    Ok((result.compressed_data, result.mapping))
} 