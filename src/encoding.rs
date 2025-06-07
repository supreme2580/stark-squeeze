use crate::compression::{compress_data, CompressionError};

/// First encoding step using compression
pub async fn encoding_one(data: &str) -> Result<Vec<u8>, CompressionError> {
    let bytes = data.as_bytes();
    let compression_result = compress_data(bytes, 4)?;
    Ok(compression_result.compressed_data)
}

/// Second encoding step using compression
pub async fn encoding_two(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let compression_result = compress_data(data, 2)?;
    Ok(compression_result.compressed_data)
} 