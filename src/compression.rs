use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use serde::{Serialize, Deserialize};
use std::fs;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionMapping {
    pub chunk_size: usize, // always 10
    #[serde(serialize_with = "serialize_chunk_map")]
    #[serde(deserialize_with = "deserialize_chunk_map")]
    pub chunk_to_code: HashMap<Vec<u8>, u16>, // 10-bit pattern to code (0..1023)
    pub padding: u8, // number of padding bits added to the last chunk
    pub original_size: usize,
    pub code_to_chunk: HashMap<u16, Vec<u8>>,
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

/// Packs a slice of 10-bit values (u16, each <= 1023) into a compact Vec<u8>
pub fn pack_10bit_values(values: &[u16]) -> Vec<u8> {
    let mut packed = Vec::new();
    let mut bit_buffer: u32 = 0;
    let mut bits_in_buffer = 0;
    for &val in values {
        bit_buffer = (bit_buffer << 10) | (val as u32);
        bits_in_buffer += 10;
        while bits_in_buffer >= 8 {
            let byte = (bit_buffer >> (bits_in_buffer - 8)) as u8;
            packed.push(byte);
            bits_in_buffer -= 8;
            bit_buffer &= (1 << bits_in_buffer) - 1;
        }
    }
    if bits_in_buffer > 0 {
        packed.push((bit_buffer << (8 - bits_in_buffer)) as u8);
    }
    packed
}

/// Unpacks a slice of bytes into 10-bit values (u16)
pub fn unpack_10bit_values(packed: &[u8]) -> Vec<u16> {
    let mut values = Vec::new();
    let mut bit_buffer: u32 = 0;
    let mut bits_in_buffer = 0;
    for &byte in packed {
        bit_buffer = (bit_buffer << 8) | (byte as u32);
        bits_in_buffer += 8;
        while bits_in_buffer >= 10 {
            let val = (bit_buffer >> (bits_in_buffer - 10)) & 0x3FF;
            values.push(val as u16);
            bits_in_buffer -= 10;
            bit_buffer &= (1 << bits_in_buffer) - 1;
        }
    }
    values
}

fn load_10bit_dictionary() -> HashMap<u16, String> {
    let dict_str = fs::read_to_string("10bit_dictionary.json").expect("Failed to read 10bit_dictionary.json");
    serde_json::from_str(&dict_str).expect("Failed to parse 10bit_dictionary.json")
}

pub fn compress_file(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let dict = load_10bit_dictionary();
    let mut binary_str = String::with_capacity(data.len() * 8);
    for byte in data {
        binary_str.push_str(&format!("{:08b}", byte));
    }
    let mut codes = Vec::new();
    let mut i = 0;
    while i < binary_str.len() {
        let end = usize::min(i + 10, binary_str.len());
        let chunk = &binary_str[i..end];
        let code = u16::from_str_radix(chunk, 2).unwrap_or(0);
        codes.push(code);
        i += 10;
    }
    Ok(pack_10bit_values(&codes))
}

pub fn decompress_file(packed: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let dict = load_10bit_dictionary();
    let codes = unpack_10bit_values(packed);
    let mut binary_str = String::new();
    for code in codes {
        if let Some(bits) = dict.get(&code) {
            binary_str.push_str(bits);
        } else {
            return Err(CompressionError::Custom(format!("Code {} not found in dictionary", code)));
        }
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

// --- Serialization helpers for u16 maps ---
fn serialize_chunk_map<S>(map: &HashMap<Vec<u8>, u16>, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer {
    use serde::ser::SerializeMap;
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        map_serializer.serialize_entry(&k.iter().map(|b| format!("{:08b}", b)).collect::<String>(), &v.to_string())?;
    }
    map_serializer.end()
}
fn deserialize_chunk_map<'de, D>(deserializer: D) -> Result<HashMap<Vec<u8>, u16>, D::Error>
where D: serde::Deserializer<'de> {
    use serde::de::{MapAccess, Visitor};
    use std::fmt;
    struct MapVisitor;
    impl<'de> Visitor<'de> for MapVisitor {
        type Value = HashMap<Vec<u8>, u16>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of Vec<u8> to u16 as strings")
        }
        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where M: MapAccess<'de> {
            let mut map = HashMap::new();
            while let Some((k, v)) = access.next_entry::<String, String>()? {
                let k = k.chars().map(|c| u8::from_str_radix(&c.to_string(), 2).unwrap_or(0)).collect::<Vec<u8>>();
                let v = v.parse::<u16>().map_err(serde::de::Error::custom)?;
                map.insert(k, v);
            }
            Ok(map)
        }
    }
    deserializer.deserialize_map(MapVisitor)
} 