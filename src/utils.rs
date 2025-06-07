use blake3;
use starknet::core::types::FieldElement;

pub fn short_string_to_felt(text: &str) -> Result<FieldElement, Box<dyn std::error::Error>> {
    if text.len() > 31 {
        return Err("String too long to fit in felt".into());
    }
    
    // Ensure the string only contains valid characters
    if !text.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("String contains invalid characters".into());
    }
    
    // Convert to lowercase to ensure consistency
    let text = text.to_lowercase();
    
    // Convert to bytes and create a felt from the first 31 bytes
    let bytes = text.as_bytes();
    let mut buf = [0u8; 31];
    let len = bytes.len().min(31);
    buf[..len].copy_from_slice(&bytes[..len]);
    
    // Convert to a number and then to FieldElement
    let mut num = 0u128;
    for &byte in buf.iter() {
        num = (num << 8) | (byte as u128);
    }
    
    Ok(FieldElement::from(num))
}

pub async fn file_to_binary(file_path: &str) -> std::io::Result<Vec<u8>> {
    tokio::fs::read(file_path).await
}

pub async fn binary_to_file(binary_string: &str, output_path: Option<&str>) -> std::io::Result<()> {
    let path = output_path.unwrap_or("output.bin");
    tokio::fs::write(path, binary_string.as_bytes()).await
}

pub fn encoding_one(binary_string: &str) -> std::io::Result<String> {
    if binary_string.is_empty() {
        return Ok(String::new());
    }
    Ok(binary_string.replace("0", ".").replace("1", " "))
}

pub fn generate_upload_id(encoded: &str) -> FieldElement {
    let hash = blake3::hash(encoded.as_bytes());
    FieldElement::from_bytes_be(hash.as_bytes()).unwrap()
}

pub fn matches_pattern<I>(chars: &mut I, pattern: &str) -> bool
where
    I: Iterator<Item = char> + Clone,
{
    let mut chars_clone = chars.clone();
    let mut pattern_chars = pattern.chars();

    loop {
        match (pattern_chars.next(), chars_clone.next()) {
            (Some(p), Some(c)) => {
                if p != c {
                    return false;
                }
            }
            (None, _) => {
                return true;
            }
            (Some(_), None) => {
                return false;
            }
        }
    }
}
