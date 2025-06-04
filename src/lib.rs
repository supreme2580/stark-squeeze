pub mod dictionary;
pub mod utils;

pub mod cli;
pub mod starknet_client;

use std::fs::File;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, BufWriter, Read, Write};
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use utils::matches_pattern;
use dictionary::{Dictionary, FIRST_DICT, SECOND_DICT, CustomDictionary, DictionaryError};


pub fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let total_size = metadata.len();

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("📦 [{bar:40.green/blue}] {percent}% ⏳ {bytes}/{total_bytes} read")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    let mut reader = io::BufReader::new(file);
    let mut buffer = Vec::with_capacity(total_size as usize);
    let mut chunk = [0u8; 4096]; // 4KB chunk

    loop {
        match reader.read(&mut chunk) {
            Ok(0) => break, // EOF
            Ok(n) => {
                buffer.extend_from_slice(&chunk[..n]);
                pb.inc(n as u64);
            }
            Err(e) => {
                pb.finish_and_clear();
                return Err(io::Error::new(io::ErrorKind::Other, format!("Read error: {}", e)));
            }
        }
    }

    pb.finish_with_message("✅ File loaded into memory! 🎉");
    Ok(buffer)
}

pub fn binary_to_file(
    input: &(impl AsRef<str> + ?Sized),
    output_path: Option<&str>,
) -> io::Result<()> {
    let binary_string: String = input.as_ref().split_whitespace().collect();
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid binary string",
        ));
    }

    let file_path = output_path.unwrap_or("output.bin");
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    let original_length = binary_string.len() as u16;
    writer.write_all(&original_length.to_be_bytes())?;

    let padded_binary_string = pad_binary_string(&binary_string);
    let total_chunks = padded_binary_string.len() / 8;

    println!(
        "🚀 Converting binary string of size {} bits to file...",
        binary_string.len()
    );

    let pb = ProgressBar::new(total_chunks as u64);
    pb.set_style(
        ProgressStyle::with_template("🔸 [{bar:40.green/blue}] {percent}% ⏳ {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    let mut byte_buffer = Vec::with_capacity(total_chunks);

    for (i, chunk) in padded_binary_string.as_bytes().chunks(8).enumerate() {
        let chunk_str = std::str::from_utf8(chunk).expect("Invalid UTF-8 sequence in binary data");
        let byte = u8::from_str_radix(chunk_str, 2)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid binary chunk"))?;
        byte_buffer.push(byte);

        pb.inc(1);
        pb.set_message(format!("Processing chunk {}/{}", i + 1, total_chunks));
    }

    pb.set_message("Writing bytes to file...");
    writer.write_all(&byte_buffer)?;
    writer.flush()?;

    pb.finish_with_message(format!("✅ File saved successfully to: {} 🎉", file_path));
    Ok(())
}

pub fn pad_binary_string(binary_string: &str) -> String {
    let padding_needed = (8 - (binary_string.len() % 8)) % 8;
    format!("{}{}", binary_string, "0".repeat(padding_needed))
}

pub fn unpad_binary_string(padded: &str, original_length: usize) -> String {
    padded.chars().take(original_length).collect()
}

pub fn read_binary_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut length_bytes = [0u8; 2];
    file.read_exact(&mut length_bytes)?;
    let original_length = u16::from_be_bytes(length_bytes) as usize;
    let mut binary_string = String::new();
    let mut byte_buffer = [0u8; 1];

    while binary_string.len() < original_length {
        file.read_exact(&mut byte_buffer)?;
        let byte_binary = format!("{:08b}", byte_buffer[0]);
        binary_string.push_str(&byte_binary);
    }

    // Use unpad_binary_string to truncate to the original length
    Ok(unpad_binary_string(&binary_string, original_length))
}
pub fn split_by_5(binary_string: &str) -> String {
    if binary_string.is_empty() {
        return serde_json::json!([]).to_string();
    }

    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return serde_json::json!([]).to_string();
    }

    let total_size = binary_string.len();
    println!("🚀 Splitting binary string of size {} bits...", total_size);

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(
        ProgressStyle::with_template("🔹 [{bar:40.green/blue}] {percent}% ⏳ {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    let chunks: Vec<String> = binary_string
        .as_bytes()
        .chunks(5)
        .enumerate()
        .map(|(i, chunk)| {
            pb.inc(chunk.len() as u64);
            pb.set_message(format!(
                "Processing chunk {}/{}",
                i + 1,
                (total_size + 4) / 5
            ));
            String::from_utf8_lossy(chunk).to_string()
        })
        .collect();

    pb.finish_with_message("✅ Splitting Complete! 🎉");
    serde_json::json!(chunks).to_string()
}

pub fn join_by_5(input: &[u8], output_path: &str) -> io::Result<()> {
    let total_size = input.len();
    println!("🚀 Processing {} bytes...", total_size);

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(
        ProgressStyle::with_template("🔵 [{bar:40.cyan/blue}] {percent}% 🚀 {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    for (i, chunk) in input.chunks(5).enumerate() {
        writer.write_all(chunk)?;
        pb.inc(chunk.len() as u64);
        pb.set_message(format!("Writing chunk {}/{}", i + 1, (total_size + 4) / 5));

        // Adaptive delay for smoother progress bar experience
        if total_size < 500 {
            sleep(Duration::from_millis(50));
        }
    }

    writer.flush()?;
    pb.finish_with_message("✅ Processing Complete! 🎉");
    println!("📁 File saved: {}", output_path);
    Ok(())
}

pub fn encoding_one_with_dict(binary_string: &str, dict: &impl Dictionary) -> Result<String, DictionaryError> {
    if binary_string.is_empty() {
        return Ok(String::new());
    }

    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err(DictionaryError::InvalidFormat("Input must be a binary string".to_string()));
    }

    let chunks: Vec<String> = binary_string
        .as_bytes()
        .chunks(5)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect();

    let mut result = String::new();
    for chunk in chunks {
        if let Some(value) = dict.get(&chunk) {
            result.push_str(value);
        } else {
            return Err(DictionaryError::InvalidFormat(format!("No mapping found for chunk: {}", chunk)));
        }
    }

    Ok(result)
}

pub fn decoding_one_with_dict(dot_string: &str, dict: &impl Dictionary) -> Result<String, DictionaryError> {
    if dot_string.is_empty() {
        return Ok(String::new());
    }

    let mut result = String::new();
    let mut current = String::new();

    for c in dot_string.chars() {
        current.push(c);
        if let Some(value) = dict.get(&current) {
            result.push_str(value);
            current.clear();
        }
    }

    if !current.is_empty() {
        return Err(DictionaryError::InvalidFormat(format!("Invalid sequence at end: {}", current)));
    }

    Ok(result)
}

pub fn encoding_two_with_dict(dot_string: &str, dict: &impl Dictionary) -> Result<String, DictionaryError> {
    if dot_string.is_empty() {
        return Ok(String::new());
    }

    let mut result = String::new();
    let mut current = String::new();

    for c in dot_string.chars() {
        current.push(c);
        if let Some(value) = dict.get(&current) {
            result.push_str(value);
            current.clear();
        }
    }

    if !current.is_empty() {
        return Err(DictionaryError::InvalidFormat(format!("Invalid sequence at end: {}", current)));
    }

    Ok(result)
}

pub fn decoding_two_with_dict(encoded_string: &str, dict: &impl Dictionary) -> Result<String, DictionaryError> {
    if encoded_string.is_empty() {
        return Ok(String::new());
    }

    let mut result = String::new();
    let mut current = String::new();

    for c in encoded_string.chars() {
        current.push(c);
        if let Some(value) = dict.get(&current) {
            result.push_str(value);
            current.clear();
        }
    }

    if !current.is_empty() {
        return Err(DictionaryError::InvalidFormat(format!("Invalid sequence at end: {}", current)));
    }

    Ok(result)
}

// Update existing functions to use the new dictionary-aware versions
pub fn encoding_one(binary_string: &str) -> io::Result<String> {
    encoding_one_with_dict(binary_string, &FIRST_DICT)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub fn decoding_one(dot_string: &str) -> Result<String, io::Error> {
    decoding_one_with_dict(dot_string, &FIRST_DICT)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub fn encoding_two(dot_string: &str) -> Result<String, io::Error> {
    encoding_two_with_dict(dot_string, &SECOND_DICT)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub fn decoding_two(encoded_string: &str) -> Result<String, io::Error> {
    decoding_two_with_dict(encoded_string, &SECOND_DICT)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

#[derive(Debug, Error)]
pub enum DictionaryValidationError {
    #[error("Field contains invalid ASCII characters: {0}")]
    InvalidASCIIError(String),

    #[error("Field has incorrect length (must be 5): {0}")]
    LengthMismatchError(String),

    #[error("Duplicate entry found: {0}")]
    DuplicateEntryError(String),

    #[error("Dictionary missing ASCII characters: {0:?}")]
    MissingCharsError(Vec<char>),
}

pub fn validate_ascii_dictionary(dict_array: &[String]) -> Result<(), DictionaryValidationError> {
    let mut seen = HashSet::new();
    let mut all_chars = HashSet::new();

    for field in dict_array {
        // Length check
        if field.len() != 5 {
            return Err(DictionaryValidationError::LengthMismatchError(field.clone()));
        }

        for ch in field.chars() {
            // ASCII check
            if !(0..=126).contains(&(ch as u8)) {
                return Err(DictionaryValidationError::InvalidASCIIError(field.clone()));
            }
            all_chars.insert(ch);
        }

        // Duplicate check
        if !seen.insert(field.clone()) {
            return Err(DictionaryValidationError::DuplicateEntryError(field.clone()));
        }
    }

    // Coverage check
    let expected_chars: HashSet<char> = (0..=126).map(|c| c as u8 as char).collect();
    let missing: Vec<char> = expected_chars.difference(&all_chars).cloned().collect();

    if !missing.is_empty() {
        return Err(DictionaryValidationError::MissingCharsError(missing));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_valid_dict() -> Vec<String> {
        let chars: Vec<char> = (0..=126).map(|c| c as u8 as char).collect();
        chars
            .chunks(5)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }

    #[test]
    fn test_valid_dictionary() {
        let dict = make_valid_dict();
        assert!(validate_ascii_dictionary(&dict).is_ok());
    }

    #[test]
    fn test_invalid_ascii() {
        let mut dict = make_valid_dict();
        dict[0] = "abce".to_string();
        let result = validate_ascii_dictionary(&dict);
        assert!(matches!(result, Err(DictionaryValidationError::InvalidASCIIError(_))));
    }

    #[test]
    fn test_length_mismatch() {
        let mut dict = make_valid_dict();
        dict[0] = "abcd".to_string(); // only 4 characters
        let result = validate_ascii_dictionary(&dict);
        assert!(matches!(result, Err(DictionaryValidationError::LengthMismatchError(_))));
    }

    #[test]
    fn test_duplicate_entry() {
        let mut dict = make_valid_dict();
        dict[1] = dict[0].clone();
        let result = validate_ascii_dictionary(&dict);
        assert!(matches!(result, Err(DictionaryValidationError::DuplicateEntryError(_))));
    }

    #[test]
    fn test_missing_characters() {
        let mut dict = make_valid_dict();
        dict.pop(); // remove one field => lose 5 characters
        let result = validate_ascii_dictionary(&dict);
        assert!(matches!(result, Err(DictionaryValidationError::MissingCharsError(_))));
    }
}
