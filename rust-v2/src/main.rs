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
