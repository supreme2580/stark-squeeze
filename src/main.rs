use stark_squeeze::cli::{main_menu, generate_ultra_compressed_ascii_combinations_cli};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Check if --generate flag is provided (JSON format with 90% compression)
    if args.len() > 1 && args[1] == "--generate" {
        generate_ultra_compressed_ascii_combinations_cli().await;
    } else if args.len() > 1 && args[1] == "--compress" {
        // compress_file_cli().await; // This line is removed as per the edit hint.
    } else if args.len() > 1 && args[1] == "--decompress" {
        // decompress_file_cli().await; // This line is removed as per the edit hint.
    } else {
        main_menu().await;
    }
} 