use std::fs::File;
use std::io::{self, Read, Write, BufWriter};

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

    let mut buffer = [0u8; 1];
    for chunk in padded_binary_string.as_bytes().chunks(8) {
        let chunk_str = std::str::from_utf8(chunk).unwrap();
        let byte = u8::from_str_radix(chunk_str, 2).unwrap_or(0);
        buffer[0] = byte;
        writer.write_all(&buffer)?;
    }

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

fn main() {
    let file_path = "example.txt"; // Change this to the actual file path
    match file_to_binary(file_path) {
        Ok(binary_data) => println!("Binary content: {:?}", binary_data),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    //binary to file with default path
    match binary_to_file("1101011010111", None) {
        Ok(_) => println!("File created successfully from single binary string"),
        Err(e) => eprintln!("Error creating file: {}", e),
    }

    //binary to file with custom path
    match binary_to_file(&["1101", "0110", "1011"].join(" "), Some("./custom_output.bin")) { // put your own custom path
        Ok(_) => println!("File created successfully from binary string array"),
        Err(e) => eprintln!("Error creating file: {}", e),
    }

    //reading back the binary file
    match read_binary_file("./custom_output.bin") { // put your own custom path
        Ok(restored_binary) => println!("Restored binary string: {}", restored_binary),
        Err(e) => eprintln!("Error reading binary file: {}", e),
    }
}
