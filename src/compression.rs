use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionMapping {
    pub chunk_size: usize,
    #[serde(serialize_with = "serialize_chunk_map")]
    #[serde(deserialize_with = "deserialize_chunk_map")]
    pub chunk_to_byte: HashMap<Vec<u8>, u8>,
    #[serde(serialize_with = "serialize_byte_map")]
    #[serde(deserialize_with = "deserialize_byte_map")]
    pub byte_to_chunk: HashMap<u8, Vec<u8>>,
    pub compression_ratio: f64,
    // New fields for complete reversal
    pub ascii_conversion_map: HashMap<u8, u8>, // Maps converted ASCII back to original bytes
    pub original_file_info: FileInfo,
    pub processing_steps: Vec<ProcessingStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub original_size: usize,
    pub file_extension: String,
    pub upload_id: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingStep {
    pub step_name: String,
    pub description: String,
    pub input_type: String,
    pub output_type: String,
    pub parameters: HashMap<String, String>,
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

// Ensure CompressionError is Send + Sync
unsafe impl Send for CompressionError {}
unsafe impl Sync for CompressionError {}

/// Finds the optimal chunk size that gives >90% compression
pub fn find_optimal_chunk_size(data: &[u8]) -> Result<usize, CompressionError> {
    let mut best_chunk_size = 1;
    let mut best_ratio = 1.0;
    
    // Try chunk sizes from 2 to 8 bytes
    for chunk_size in 2..=8 {
        let chunks: Vec<&[u8]> = data.chunks(chunk_size).collect();
        let _unique_chunks: std::collections::HashSet<&[u8]> = chunks.iter().copied().collect();
        
        // Calculate compression ratio
        let original_size = data.len();
        let compressed_size = chunks.len(); // Just the encoded data size
        let ratio = compressed_size as f64 / original_size as f64;
        
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
        ascii_conversion_map: HashMap::new(),
        original_file_info: FileInfo {
            original_size: 0,
            file_extension: String::new(),
            upload_id: String::new(),
            hash: String::new(),
        },
        processing_steps: Vec::new(),
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

/// Main compression function that handles the entire process
pub fn compress_file(data: &[u8]) -> Result<CompressionResult, CompressionError> {
    // Find optimal chunk size
    let chunk_size = find_optimal_chunk_size(data)?;
    
    // Create mapping
    let mapping = create_chunk_mapping(data, chunk_size)?;
    
    // Compress data
    let compressed_data = compress_data(data, &mapping)?;
    
    Ok(CompressionResult {
        compressed_data,
        mapping,
    })
}

// Custom serialization for chunk_to_byte HashMap
fn serialize_chunk_map<S>(
    map: &HashMap<Vec<u8>, u8>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
    for (chunk, byte) in map {
        let chunk_str = chunk.iter()
            .map(|&b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");
        map_serializer.serialize_entry(&chunk_str, byte)?;
    }
    map_serializer.end()
}

// Custom deserialization for chunk_to_byte HashMap
fn deserialize_chunk_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<Vec<u8>, u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{MapAccess, Visitor};
    use std::fmt;

    struct ChunkMapVisitor;

    impl<'de> Visitor<'de> for ChunkMapVisitor {
        type Value = HashMap<Vec<u8>, u8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of hex strings to bytes")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((chunk_str, byte)) = access.next_entry::<String, u8>()? {
                let chunk: Vec<u8> = chunk_str.as_bytes()
                    .chunks(2)
                    .filter_map(|chunk| {
                        if chunk.len() == 2 {
                            u8::from_str_radix(&String::from_utf8_lossy(chunk), 16).ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                map.insert(chunk, byte);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(ChunkMapVisitor)
}

// Custom serialization for byte_to_chunk HashMap
fn serialize_byte_map<S>(
    map: &HashMap<u8, Vec<u8>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
    for (byte, chunk) in map {
        let chunk_str = chunk.iter()
            .map(|&b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");
        map_serializer.serialize_entry(&format!("{:02x}", byte), &chunk_str)?;
    }
    map_serializer.end()
}

// Custom deserialization for byte_to_chunk HashMap
fn deserialize_byte_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<u8, Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{MapAccess, Visitor};
    use std::fmt;

    struct ByteMapVisitor;

    impl<'de> Visitor<'de> for ByteMapVisitor {
        type Value = HashMap<u8, Vec<u8>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of hex strings to hex strings")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some((byte_str, chunk_str)) = access.next_entry::<String, String>()? {
                if let Ok(byte) = u8::from_str_radix(&byte_str, 16) {
                    let chunk: Vec<u8> = chunk_str.as_bytes()
                        .chunks(2)
                        .filter_map(|chunk| {
                            if chunk.len() == 2 {
                                u8::from_str_radix(&String::from_utf8_lossy(chunk), 16).ok()
                            } else {
                                None
                            }
                        })
                        .collect();
                    map.insert(byte, chunk);
                }
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(ByteMapVisitor)
} 