pub mod dictionary;
pub mod utils;
pub mod ascii_converter;

pub mod cli;
pub mod starknet_client;

use std::fs;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{self, BufWriter, Read, Write, BufReader};
use std::thread::sleep;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{self as tokio_io, AsyncReadExt, AsyncWriteExt};
use utils::matches_pattern;
use dictionary::{Dictionary, FIRST_DICT, SECOND_DICT, CustomDictionary, DictionaryError};
use ascii_converter::convert_file_to_ascii;

use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use thiserror::Error;

/// ANSI color codes for rainbow bit highlighting
const RAINBOW_COLORS: [&str; 5] = [
    "\x1b[31m", // Red
    "\x1b[33m", // Yellow
    "\x1b[32m", // Green
    "\x1b[36m", // Cyan
    "\x1b[35m", // Magenta
];
const RESET_COLOR: &str = "\x1b[0m";

#[derive(Debug)]
pub enum AsciiToFileError {
    InvalidASCIIError(String),
    FileIntegrityError(String),
    IOError(io::Error),
}

impl From<io::Error> for AsciiToFileError {
    fn from(err: io::Error) -> Self {
        AsciiToFileError::IOError(err)
    }
}

/// Enhanced ASCII-to-Dot Visualization Function with Bit Numbering and Special Character Handling
/// 
/// # Arguments
/// * `input_str` - The input string to visualize
/// * `group_size` - Number of characters per group (default: 5)
/// * `show_color` - Enable ANSI color codes for rainbow bit highlighting
/// 
/// # Returns
/// A formatted string with detailed ASCII visualization
pub fn ascii_to_dot(input_str: &str, group_size: Option<usize>, show_color: Option<bool>) -> String {
    let group_size = group_size.unwrap_or(5);
    let show_color = show_color.unwrap_or(false);
    
    if input_str.is_empty() {
        return "Empty input string".to_string();
    }
    
    let mut result = String::new();
    
    // Add bit numbering header
    result.push_str("Bit: ");
    for i in 0..5 {
        if show_color {
            result.push_str(&format!("{}{}{} ", RAINBOW_COLORS[i], i, RESET_COLOR));
        } else {
            result.push_str(&format!("{} ", i));
        }
    }
    result.push_str("\n");
    result.push_str("---------\n");
    
    // Process each character
    for (idx, ch) in input_str.chars().enumerate() {
        // Add group header
        if idx % group_size == 0 {
            if idx > 0 {
                result.push('\n');
            }
            result.push_str(&format!("[Group {}]\n", (idx / group_size) + 1));
        }
        
        let ascii_code = ch as u8;
        
        // Handle special characters
        let char_display = match ch {
            ' ' => "[SP]".to_string(),
            '\0' => "[NUL]".to_string(),
            c if c.is_control() => {
                if ascii_code < 32 {
                    format!("[CTRL+{}]", (ascii_code + 64) as char)
                } else if ascii_code == 127 {
                    "[DEL]".to_string()
                } else {
                    format!("[CTRL+{}]", c)
                }
            },
            c => c.to_string(),
        };
        
        // Generate 5-bit binary representation
        let binary = format!("{:05b}", ascii_code);
        let hex = format!("0x{:02X}", ascii_code);
        
        // Create dot visualization with bit positioning
        let mut dot_viz = String::new();
        for (bit_idx, bit_char) in binary.chars().enumerate() {
            if bit_char == '1' {
                if show_color {
                    dot_viz.push_str(&format!("{}{}{}", RAINBOW_COLORS[bit_idx], char_display.chars().next().unwrap_or('.'), RESET_COLOR));
                } else {
                    dot_viz.push(char_display.chars().next().unwrap_or('.'));
                }
            } else {
                dot_viz.push('.');
            }
        }
        
        // Format the complete line
        result.push_str(&format!("{} {} [{}] = {} ({})\n", 
            dot_viz, 
            char_display, 
            ascii_code, 
            binary, 
            hex
        ));
    }
    
    result
}

/// Compute a simple hash (using DefaultHasher) for file content
fn compute_hash<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let data = fs::read(path)?;
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    Ok(hasher.finish())
}

/// Converts an ASCII string back to a binary file
/// Ensures 1:1 byte correspondence (each ASCII char = 1 byte)
/// Validates the content and ensures integrity post-write
pub async fn ascii_to_file(ascii_input: &str, output_path: &str) -> Result<(), AsciiToFileError> {
    // Step 1: Validation - ensure all characters are within ASCII range (0-127)
    if !ascii_input.chars().all(|c| c as u32 <= 127) {
        return Err(AsciiToFileError::InvalidASCIIError(
            "Input contains non-ASCII characters (code > 127)".to_string(),
        ));
    }

    // Step 2: Convert ASCII chars to bytes
    let bytes = ascii_input.as_bytes();

    // Step 3: Write to file
    let file = File::create(output_path).await?;
    let mut writer = tokio_io::BufWriter::new(file);
    writer.write_all(bytes).await?;
    writer.flush().await?;

    // Step 4: Post-write validation
    let written_data = fs::read(output_path)?;
    if written_data.len() != ascii_input.len() {
        return Err(AsciiToFileError::FileIntegrityError(
            "File size mismatch after writing".to_string(),
        ));
    }

    let original_hash = {
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        hasher.finish()
    };

    let written_hash = compute_hash(output_path)?;

    if original_hash != written_hash {
        return Err(AsciiToFileError::FileIntegrityError(
            "Hash mismatch detected after file write".to_string(),
        ));
    }

    println!("✅ ASCII string successfully written to {} and verified.", output_path);
    Ok(())
}

pub async fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    let total_size = metadata.len();

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("📄 [{bar:40.green/blue}] {percent}% ⏳ {bytes}/{total_bytes} read")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    let mut reader = tokio_io::BufReader::new(file);
    let mut buffer = Vec::with_capacity(total_size as usize);
    let mut chunk = [0u8; 4096];

    loop {
        match reader.read(&mut chunk).await {
            Ok(0) => break, // EOF
            Ok(n) => {
                // Check for non-ASCII bytes in this chunk
                if let Some((idx, &b)) = chunk[..n].iter().enumerate().find(|&(_, &b)| b > 126) {
                    pb.finish_and_clear();
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Non-ASCII byte (value {}) found at offset {}", b, buffer.len() + idx),
                    ));
                }
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

    println!("\n🔄 Converting file to printable ASCII...");
    let ascii_buffer = convert_file_to_ascii(buffer)?;

    Ok(ascii_buffer)
}

pub async fn binary_to_file(
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
    let file = File::create(file_path).await?;
    let mut writer = tokio_io::BufWriter::new(file);

    let original_length = binary_string.len() as u16;
    writer.write_all(&original_length.to_be_bytes()).await?;

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
    writer.write_all(&byte_buffer).await?;
    writer.flush().await?;

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

pub async fn read_binary_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path).await?;
    let mut length_bytes = [0u8; 2];
    file.read_exact(&mut length_bytes).await?;
    let original_length = u16::from_be_bytes(length_bytes) as usize;
    let mut binary_string = String::new();
    let mut byte_buffer = [0u8; 1];

    while binary_string.len() < original_length {
        file.read_exact(&mut byte_buffer).await?;
        let byte_binary = format!("{:08b}", byte_buffer[0]);
        binary_string.push_str(&byte_binary);
    }

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

pub async fn join_by_5(input: &[u8], output_path: &str) -> io::Result<()> {
    let total_size = input.len();
    println!("🚀 Processing {} bytes...", total_size);

    let file = File::create(output_path).await?;
    let mut writer = tokio_io::BufWriter::new(file);

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(
        ProgressStyle::with_template("🔵 [{bar:40.cyan/blue}] {percent}% 🚀 {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    for (i, chunk) in input.chunks(5).enumerate() {
        writer.write_all(chunk).await?;
        pb.inc(chunk.len() as u64);
        pb.set_message(format!("Writing chunk {}/{}", i + 1, (total_size + 4) / 5));

        // Adaptive delay for smoother progress bar experience
        if total_size < 500 {
            sleep(Duration::from_millis(50));
        }
    }

    writer.flush().await?;
    pb.finish_with_message("✅ Processing Complete! 🎉");
    println!("📁 File saved: {}", output_path);
    Ok(())
}

pub async fn decoding_one(dot_string: &str) -> Result<String, io::Error> {
    // Delegate to the dictionary-based implementation
    decoding_one_with_dict(dot_string, &FIRST_DICT)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub fn encoding_one_with_dict(binary_string: &str, dict: &impl Dictionary) -> Result<String, DictionaryError> {
    if binary_string.is_empty() {
        return Ok(String::new());
    }

    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err(DictionaryError::InvalidFormat("Input must be a binary string".to_string()));
    }

    let padded_length = ((binary_string.len() + 4) / 5) * 5;
    let padded_binary = format!("{:0width$}", binary_string.parse::<u128>().unwrap_or(0), width = padded_length);

    let chunks: Vec<String> = padded_binary
        .as_bytes()
        .chunks(5)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect();

    let mut result = String::new();
    for chunk in chunks {
        if let Some(value) = dict.get(&chunk) {
            result.push_str(&value);
        } else {
            return Err(DictionaryError::InvalidFormat(format!("No mapping found for chunk: {}", chunk)));
        }
    }

    Ok(result)
}

pub async fn decoding_two(encoded_string: &str) -> Result<String, io::Error> {
    // Delegate to the dictionary-based implementation
    decoding_two_with_dict(encoded_string, &SECOND_DICT)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
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
            result.push_str(&value);
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
            result.push_str(&value);
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
            result.push_str(&value);
            current.clear();
        }
    }

    if !current.is_empty() {
        return Err(DictionaryError::InvalidFormat(format!("Invalid sequence at end: {}", current)));
    }

    Ok(result)
}

pub async fn encoding_one(binary_string: &str) -> io::Result<String> {
    encoding_one_with_dict(binary_string, &FIRST_DICT)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

pub async fn encoding_two(dot_string: &str) -> Result<String, io::Error> {
    encoding_two_with_dict(dot_string, &SECOND_DICT)
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
        dict[0] = "abce".to_string();
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

    // Tests for ascii_to_dot function
    #[test]
    fn test_ascii_to_dot_normal_characters() {
        let result = ascii_to_dot("ABC", Some(5), Some(false));
        assert!(result.contains("[Group 1]"));
        assert!(result.contains("A [65] = 01000001"));
        assert!(result.contains("B [66] = 01000010"));
        assert!(result.contains("C [67] = 01000011"));
    }

    #[test]
    fn test_ascii_to_dot_mixed_input_with_null() {
        let result = ascii_to_dot("A\0 C", Some(5), Some(false));
        assert!(result.contains("[NUL] [0]"));
        assert!(result.contains("[SP] [32]"));
    }

    #[test]
    fn test_ascii_to_dot_grouping() {
        let result = ascii_to_dot("ABCDEFGH", Some(3), Some(false));
        assert!(result.contains("[Group 1]"));
        assert!(result.contains("[Group 2]"));
        assert!(result.contains("[Group 3]"));
    }

    #[test]
    fn test_ascii_to_dot_control_characters() {
        let result = ascii_to_dot("\x01\x1F\x7F", Some(5), Some(false));
        assert!(result.contains("[CTRL+A]"));
        assert!(result.contains("[CTRL+_]"));
        assert!(result.contains("[DEL]"));
    }

    #[test]
    fn test_ascii_to_dot_empty_string() {
        let result = ascii_to_dot("", None, None);
        assert_eq!(result, "Empty input string");
    }

    #[test]
    fn test_ascii_to_dot_with_colors() {
        let result = ascii_to_dot("A", Some(5), Some(true));
        // Should contain ANSI escape codes
        assert!(result.contains("\x1b["));
    }

    #[test]
    fn test_ascii_to_dot_edge_case_127_chars() {
        let test_string: String = (0..127).map(|i| (i as u8) as char).collect();
        let result = ascii_to_dot(&test_string, Some(5), Some(false));
        
        // Should have multiple groups
        assert!(result.contains("[Group 1]"));
        assert!(result.contains("[Group 25]")); // 127/5 = 25.4, so 26 groups
        
        // Should handle all ASCII characters
        assert!(result.contains("[NUL]")); // char 0
        assert!(result.contains("[DEL]")); // char 127 if included
    }
}