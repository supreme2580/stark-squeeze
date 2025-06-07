use std::time::Instant;
use tokio;
use stark_squeeze::utils::{file_to_binary, binary_to_file};

// Use imports from stark_squeeze crate
extern crate stark_squeeze;

const TEST_SIZE_MB: usize = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== StarkSqueeze Async I/O Performance Benchmark ===");
    
    // Create a test file
    let test_file = "test_large_file.txt";
    println!("Creating a {}MB test file...", TEST_SIZE_MB);
    
    let data = vec![b'A'; TEST_SIZE_MB * 1024 * 1024]; // 5MB of 'A's
    tokio::fs::write(test_file, &data).await?;
    println!("Test file created successfully.");
    
    // Test reading with async implementation
    println!("\nTesting async file_to_binary...");
    let start = Instant::now();
    let buffer = file_to_binary(test_file).await?;
    let duration = start.elapsed();
    
    println!("Read {}MB in {:?}", buffer.len() / (1024 * 1024), duration);
    println!("Throughput: {:.2} MB/s", 
        (buffer.len() as f64 / 1024.0 / 1024.0) / duration.as_secs_f64());
    
    // Test writing with async implementation
    println!("\nTesting async binary_to_file...");
    let binary_string = String::from_utf8(vec![b'0', b'1'].repeat(TEST_SIZE_MB * 1024 * 512))?; // 5MB of alternating 0s and 1s
    
    let start = Instant::now();
    binary_to_file(&binary_string, Some("test_output.bin")).await?;
    let duration = start.elapsed();
    
    println!("Wrote {}MB in {:?}", TEST_SIZE_MB, duration);
    println!("Throughput: {:.2} MB/s", 
        (TEST_SIZE_MB as f64) / duration.as_secs_f64());
    
    // Clean up
    println!("\nCleaning up test files...");
    tokio::fs::remove_file(test_file).await?;
    tokio::fs::remove_file("test_output.bin").await?;
    
    println!("\n=== Benchmark Complete ===");
    Ok(())
}
