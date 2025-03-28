use std::fs::File;
use std::io::{self, Read, Write, BufWriter};
use std::thread::sleep;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

pub fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn split_by_5(binary_string: &str) -> String {
    // Handle the edge case of an empty string
    if binary_string.is_empty() {
        return serde_json::json!([]).to_string();
    }

    // Validate input: ensure it only contains '0' and '1'
    if !binary_string.chars().all(|c| c == '0' || c == '1') {
        return serde_json::json!([]).to_string(); // Return an empty JSON array for invalid input
    }

    // Split the string into chunks of 5 characters using as_bytes for efficiency
    let chunks: Vec<String> = binary_string
        .as_bytes()
        .chunks(5)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect();

    // Convert the chunks into a JSON array
    serde_json::json!(chunks).to_string()
}

pub fn join_by_5(input: &[u8], output_path: &str) -> io::Result<()> {
    let total_size = input.len();
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::with_template("[{bar:40.cyan/blue}] {percent}% 🚀")
        .unwrap()
        .progress_chars("=> "));

    for chunk in input.chunks(5) {
        writer.write_all(chunk)?;
        pb.inc(chunk.len() as u64);

        // Simulate processing time (only needed for small files)
        sleep(Duration::from_millis(50));
    }

    writer.flush()?;
    pb.finish_with_message("Processing Complete ✅");
    Ok(())
}

fn main() {
    let file_path = "example.txt"; // Change this to your actual file path
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
