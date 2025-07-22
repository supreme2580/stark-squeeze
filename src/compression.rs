use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionMapping {
    pub chunk_size: usize, // always 10
    #[serde(serialize_with = "serialize_chunk_map")]
    #[serde(deserialize_with = "deserialize_chunk_map")]
    pub chunk_to_code: HashMap<u16, u16>, // 10-bit pattern to code (0..1023)
    #[serde(serialize_with = "serialize_code_map")]
    #[serde(deserialize_with = "deserialize_code_map")]
    pub code_to_chunk: HashMap<u16, u16>, // code to 10-bit pattern
    pub padding: u8, // number of padding bits added to the last chunk
    pub original_size: usize,
}

#[derive(Debug)]
pub struct CompressionResult {
    pub compressed_data: Vec<u16>, // each code is 2 bytes
    pub mapping: CompressionMapping,
}

#[derive(Debug)]
pub enum CompressionError {
    CompressionFailed,
    Custom(String),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompressionError::CompressionFailed => write!(f, "Compression failed"),
            CompressionError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for CompressionError {}

// Add helper to load the 10:3 dictionary
fn load_10to3_dictionary(path: &str) -> HashMap<String, String> {
    let dict_str = fs::read_to_string(path).expect("Failed to read dictionary file");
    serde_json::from_str(&dict_str).expect("Failed to parse dictionary JSON")
}

// Compression: binary to 3-char codes
pub fn compress_file_with_10to3_dict(data: &[u8], dict_path: &str) -> Result<String, String> {
    let dict = load_10to3_dictionary(dict_path);
    let mut binary_str = String::with_capacity(data.len() * 8);
    for byte in data {
        binary_str.push_str(&format!("{:08b}", byte));
    }
    let mut compressed = String::new();
    let mut i = 0;
    while i < binary_str.len() {
        let end = usize::min(i + 10, binary_str.len());
        let mut chunk = binary_str[i..end].to_string();
        if chunk.len() < 10 {
            chunk.push_str(&"0".repeat(10 - chunk.len()));
        }
        let code = dict.get(&chunk).ok_or_else(|| format!("Chunk not found in dictionary: {}", chunk))?;
        compressed.push_str(code);
        i += 10;
    }
    Ok(compressed)
}

// Decompression: 3-char codes to binary
pub fn decompress_file_with_10to3_dict(compressed: &str, dict_path: &str) -> Result<Vec<u8>, String> {
    let dict = load_10to3_dictionary(dict_path);
    // Reverse the dictionary: code -> 10-bit binary string
    let rev_dict: HashMap<&str, &str> = dict.iter().map(|(k, v)| (v.as_str(), k.as_str())).collect();
    let mut binary_str = String::new();
    let chars: Vec<char> = compressed.chars().collect();
    let mut i = 0;
    while i + 3 <= chars.len() {
        let code: String = chars[i..i+3].iter().collect();
        let bin = rev_dict.get(code.as_str()).ok_or_else(|| format!("Code not found in dictionary: {}", code))?;
        binary_str.push_str(bin);
        i += 3;
    }
    // Remove any trailing padding bits to make length a multiple of 8
    let trimmed_len = binary_str.len() - (binary_str.len() % 8);
    let binary_str = &binary_str[..trimmed_len];
    let mut bytes = Vec::new();
    let mut j = 0;
    while j + 8 <= binary_str.len() {
        let byte = u8::from_str_radix(&binary_str[j..j+8], 2).unwrap();
        bytes.push(byte);
        j += 8;
    }
    Ok(bytes)
}

/// Compresses data using the 10:3 dictionary and returns CompressionResult (Vec<u16> codes and mapping)
pub fn compress_file(data: &[u8]) -> Result<CompressionResult, CompressionError> {
    let dict = load_10to3_dictionary("binary_dictionary_10to3.json");
    let mut binary_str = String::with_capacity(data.len() * 8);
    for byte in data {
        binary_str.push_str(&format!("{:08b}", byte));
    }
    let mut compressed = Vec::new();
    let mut i = 0;
    let mut chunk_to_code = HashMap::new();
    let mut code_to_chunk = HashMap::new();
    let mut code_counter = 0u16;
    let chunk_size = 10;
    while i < binary_str.len() {
        let end = usize::min(i + chunk_size, binary_str.len());
        let mut chunk = binary_str[i..end].to_string();
        if chunk.len() < chunk_size {
            chunk.push_str(&"0".repeat(chunk_size - chunk.len()));
        }
        // Map chunk to code
        let code = if let Some(existing_code) = chunk_to_code.get(&chunk) {
            *existing_code
        } else {
            let code = code_counter;
            chunk_to_code.insert(chunk.clone(), code);
            code_to_chunk.insert(code, u16::from_str_radix(&chunk, 2).unwrap_or(0));
            code_counter += 1;
            code
        };
        compressed.push(code);
        i += chunk_size;
    }
    let mapping = CompressionMapping {
        chunk_size,
        chunk_to_code: chunk_to_code.iter().map(|(k, v)| (u16::from_str_radix(k, 2).unwrap_or(0), *v)).collect(),
        code_to_chunk,
        padding: ((chunk_size - (binary_str.len() % chunk_size)) % chunk_size) as u8,
        original_size: data.len(),
    };
    Ok(CompressionResult {
        compressed_data: compressed,
        mapping,
    })
}

// --- Serialization helpers for u16 maps ---
fn serialize_chunk_map<S>(map: &HashMap<u16, u16>, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer {
    use serde::ser::SerializeMap;
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        map_serializer.serialize_entry(&k.to_string(), &v.to_string())?;
    }
    map_serializer.end()
}
fn deserialize_chunk_map<'de, D>(deserializer: D) -> Result<HashMap<u16, u16>, D::Error>
where D: serde::Deserializer<'de> {
    use serde::de::{MapAccess, Visitor};
    use std::fmt;
    struct MapVisitor;
    impl<'de> Visitor<'de> for MapVisitor {
        type Value = HashMap<u16, u16>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of u16 to u16 as strings")
        }
        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where M: MapAccess<'de> {
            let mut map = HashMap::new();
            while let Some((k, v)) = access.next_entry::<String, String>()? {
                let k = k.parse::<u16>().map_err(serde::de::Error::custom)?;
                let v = v.parse::<u16>().map_err(serde::de::Error::custom)?;
                map.insert(k, v);
            }
            Ok(map)
        }
    }
    deserializer.deserialize_map(MapVisitor)
}
fn serialize_code_map<S>(map: &HashMap<u16, u16>, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer {
    serialize_chunk_map(map, serializer)
}
fn deserialize_code_map<'de, D>(deserializer: D) -> Result<HashMap<u16, u16>, D::Error>
where D: serde::Deserializer<'de> {
    deserialize_chunk_map(deserializer)
} 