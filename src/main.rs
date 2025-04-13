use std::fs::File;
use std::io::{self, Read, Write, BufWriter};
use std::thread::sleep;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json;


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

fn main() {
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