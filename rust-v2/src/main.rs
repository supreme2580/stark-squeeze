use std::fs::File;
use std::io::{self, Read, Write};

pub fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn binary_to_file(input: &(impl AsRef<str> + ?Sized)) -> io::Result<()> {
    
    let binary_string = if let Some(input_str) = input.as_ref().split_whitespace().next() {
        input_str.to_string()
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty input"));
    };

    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid binary string"));
    }

    let bytes: Vec<u8> = (0..binary_string.len())
        .step_by(8)
        .map(|i| {
            let byte_str = &binary_string[i..std::cmp::min(i + 8, binary_string.len())];
            u8::from_str_radix(byte_str, 2).unwrap_or(0)
        })
        .collect();

    let mut file = File::create("output.bin")?;
    file.write_all(&bytes)?;

    Ok(())
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
