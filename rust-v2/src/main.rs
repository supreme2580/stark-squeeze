use std::fs::File;
use std::io::{self, Read};
mod progress;
use progress::join_by_5;

pub fn file_to_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    println!("🚀 Starting Stark-Squeeze Processing");
    println!("====================================");

    // more realistic test data (20 bytes)
    let test_data = vec![
        // first block (simulates text)
        1, 0, 1, 0, 1,  // 21
        0, 1, 0, 1, 0,  // 10
        1, 1, 1, 1, 1,  // 31
        0, 0, 0, 0, 0,  // 0
        // second block (simulates image)
        1, 1, 1, 0, 0,  // 28
        0, 0, 1, 1, 1,  // 7
        1, 0, 1, 0, 1,  // 21
        1, 1, 0, 0, 1   // 25
    ];
    
    println!("📁 Input size: {} bytes", test_data.len());
    println!("🔍 Processing data...\n");

    match join_by_5(&test_data) {
        Ok(result) => {
            println!("\n📊 Results:");
            println!("====================================");
            println!("📥 Input chunks: {:?}", test_data);
            println!("📤 Output values: {:?}", result);
            println!("📈 Compression ratio: {:.2}%", 
                (result.len() as f64 / test_data.len() as f64) * 100.0);
        },
        Err(e) => println!("❌ Error: {}", e),
    }
}