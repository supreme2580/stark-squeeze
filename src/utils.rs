use starknet::core::types::FieldElement;

/// Converts a short string to a FieldElement for StarkNet
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