use std::fs::File;
use std::io::{self, Read, Write};

pub fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn binary_to_file(input: &(impl AsRef<str> + ?Sized)) -> io::Result<()> {
    let binary_string: String = input
        .as_ref()
        .split_whitespace()
        .collect();

    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid binary string"));
    }

    let padded_binary_string = pad_binary_string(&binary_string);

    let bytes: Vec<u8> = padded_binary_string
        .as_bytes()
        .chunks(8)
        .map(|chunk| {
            let chunk_str = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(chunk_str, 2).unwrap_or(0)
        })
        .collect();

    let mut file = File::create("output.bin")?;
    file.write_all(&bytes)?;

    Ok(())
}

fn pad_binary_string(binary_string: &str) -> String {
    let padding_needed = (8 - (binary_string.len() % 8)) % 8;
    format!("{}{}", "0".repeat(padding_needed), binary_string)
}

fn main() {
    let file_path = "example.txt"; // Change this to the actual file path
    match file_to_binary(file_path) {
        Ok(binary_data) => println!("Binary content: {:?}", binary_data),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    match binary_to_file("1101011010111") {
        Ok(_) => println!("File created successfully from single binary string"),
        Err(e) => eprintln!("Error creating file: {}", e),
    }

    match binary_to_file(&["1101", "0110", "1011"].join(" ")) {
        Ok(_) => println!("File created successfully from binary string array"),
        Err(e) => eprintln!("Error creating file: {}", e),
    }
}
