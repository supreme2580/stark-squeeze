use std::fs::File;
use std::io::{self, Read};

pub fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let file_path = "example.txt"; // Change this to the actual file path
    match file_to_binary(file_path) {
        Ok(binary_data) => println!("Binary content: {:?}", binary_data),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}