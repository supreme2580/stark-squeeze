use starknet::core::types::FieldElement;
use blake3;

pub fn short_string_to_felt(text: &str) -> Result<FieldElement, Box<dyn std::error::Error>> {
    if text.len() > 31 {
        return Err("String too long to fit in felt".into());
    }
    let bytes = text.as_bytes();
    let mut buf = [0u8; 32];
    buf[..bytes.len()].copy_from_slice(bytes);
    Ok(FieldElement::from_bytes_be(&buf)?)
}

pub fn file_to_binary(file_path: &str) -> std::io::Result<Vec<u8>> {
    std::fs::read(file_path)
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
