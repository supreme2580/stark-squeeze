use std::fs::File;
use std::io::{self, Read, Write, BufWriter};
use std::thread::sleep;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use serde_json;

mod dictionary;
use dictionary::FIRST_DICT;

pub fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn binary_to_file(input: &(impl AsRef<str> + ?Sized), output_path: Option<&str>) -> io::Result<()> {
    let binary_string: String = input
        .as_ref()
        .split_whitespace()
        .collect();
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid binary string"));
    }
    let file_path = output_path.unwrap_or("output.bin");
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    let original_length = binary_string.len() as u16;
    writer.write_all(&original_length.to_be_bytes())?;
    let padded_binary_string = pad_binary_string(&binary_string);
    let mut byte_buffer = Vec::with_capacity(padded_binary_string.len() / 8);
    for chunk in padded_binary_string.as_bytes().chunks(8) {
        let chunk_str = std::str::from_utf8(chunk)
            .expect("Invalid UTF-8 sequence in binary data");
        let byte = u8::from_str_radix(chunk_str, 2)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid binary chunk"))?;
        byte_buffer.push(byte);
    }
    writer.write_all(&byte_buffer)?;
    writer.flush()?;
    Ok(())
}

fn pad_binary_string(binary_string: &str) -> String {
    let padding_needed = (8 - (binary_string.len() % 8)) % 8;
    format!("{}{}", binary_string, "0".repeat(padding_needed))
}

fn read_binary_file(file_path: &str) -> io::Result<String> {
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
    binary_string.truncate(original_length);
    Ok(binary_string)
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
            pb.set_message(format!("Processing chunk {}/{}", i + 1, (total_size + 4) / 5));
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

pub fn decoding_one(dot_string: &str) -> Result<String, io::Error> {
    // Handle empty input
    if dot_string.is_empty() {
        return Ok(String::new());
    }
    
    // Reverse the FIRST_DICT for lookup: dot_string -> 5-bit binary
    let mut reverse_dict: HashMap<&str, &str> = HashMap::new();
    for (bin, dot) in FIRST_DICT.entries() {
        if !dot.is_empty() {
        reverse_dict.insert(*dot, *bin);
        }
    }
    
    // Parse the dot_string into tokens
    let tokens: Vec<&str> = dot_string.split('.').filter(|t| !t.is_empty()).collect();
    
    let mut reconstructed_binary = String::new();
    
    for token in tokens {
        match reverse_dict.get(token) {
            Some(binary_chunk) => reconstructed_binary.push_str(binary_chunk),
            None => {
                return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown token '{}' found during decoding", token),
                ))
            }
        }
    }
    
    Ok(reconstructed_binary)
}

pub fn decoding_one(encoded_str: &str) -> Result<String, io::Error> {
    if encoded_str.is_empty() {
        return Ok(String::new());
    }

    // Reverse the dictionary
    let mut reverse_dict: HashMap<&str, &str> = HashMap::new();
    for (bin, sym) in FIRST_DICT.entries() {
        if !sym.is_empty() {
            reverse_dict.insert(*sym, *bin);
        }
    }

    let mut binary_string = String::new();
    let mut temp = String::new();

    let mut chars = encoded_str.chars().peekable();

    while let Some(c) = chars.next() {
        temp.push(c);

        // Try to match a known dot symbol
        if reverse_dict.contains_key(temp.as_str()) {
            binary_string.push_str(reverse_dict.get(temp.as_str()).unwrap());
            temp.clear();
        } else if chars.peek().is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid or incomplete symbol sequence: '{}'", temp),
            ));
        }
    }

    if !temp.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unmatched symbol at the end: '{}'", temp),
        ));
    }

    Ok(binary_string)
}

pub fn encoding_one(binary_string: &str) -> io::Result<String> {
    // Handle empty string case
    if binary_string.is_empty() {
        return Ok(String::new());
    }

    // Validate input - ensure only 0s and 1s
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, 
            "Invalid binary string: must contain only 0 and 1"
        ));
    }

    // Pad the binary string so it's divisible by 5
    let padded_binary_string = pad_binary_string(binary_string);
    
    // Process the padded string in 5-bit chunks
    let mut chunks = Vec::new();
    for i in (0..padded_binary_string.len()).step_by(5) {
        if i + 5 <= padded_binary_string.len() {
            chunks.push(&padded_binary_string[i..i+5]);
        }
    }
    
    // Map each chunk to its dot string representation
    let mut result = Vec::with_capacity(chunks.len());
    
    for chunk in &chunks {
        match FIRST_DICT.get(*chunk) {
            Some(dot_string) => {
                // Only add non-empty dot strings to the result
                if !dot_string.is_empty() {
                    result.push(*dot_string);
                }
            },
            None => return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Chunk {} not found in dictionary", chunk)
            )),
        }
    }
    
    // Concatenate the dot strings (no separator needed)
    Ok(result.concat())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_one() {
        // Test with "00010" (.) + "00010" (.)
        let binary = "0001000010";
        let result = encoding_one(binary).unwrap();
        assert_eq!(result, "..");
        
        // Test with "00111" (...) + "01110" (...)
        let binary2 = "0011101110";
        let result2 = encoding_one(binary2).unwrap();
        assert_eq!(result2, "......");  // "..." + "..."
        
        // Test with a longer binary string
        // "10101" (". . .") + "11111" (".....")
        let binary3 = "1010111111";
        let result3 = encoding_one(binary3).unwrap();
        assert_eq!(result3, ". . ......");
        
        // Test with empty string
        let result4 = encoding_one("").unwrap();
        assert_eq!(result4, "");
        
        // Test with invalid characters
        let invalid_binary = "001201";
        let result5 = encoding_one(invalid_binary);
        assert!(result5.is_err());
        
        // Test with binary that maps to empty string in FIRST_DICT
        let binary6 = "00000";
        let result6 = encoding_one(binary6).unwrap();
        assert_eq!(result6, "");
        
        // Test with a mix of emptry string mappings and non-empty
        // "00000" ("") + "00001" (".")
        let binary7 = "0000000001";
        let result7 = encoding_one(binary7).unwrap();
        assert_eq!(result7, ".");
    }
}

fn main() {
    // Example usage of encoding_one
    let binary_string = "0011101110";
    match encoding_one(binary_string) {
        Ok(encoded) => println!("Binary: {} -> Encoded: {}", binary_string, encoded),
        Err(e) => eprintln!("Error encoding binary string: {}", e),
    }
    
    // Original file processing code
    let file_path = "cat.mp4";
    let output_path = "output.bin";

    match file_to_binary(file_path) {
        Ok(binary_data) => {
            println!("Binary content loaded. Processing...");
            if let Err(e) = join_by_5(&binary_data, output_path) {
                eprintln!("Error processing file: {}", e);
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}