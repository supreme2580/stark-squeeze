# StarkSqueeze v2

StarkSqueeze v2 is a Rust implementation of super data compression techniques optimized for StarkNet.

## Overview

This library provides advanced data compression techniques designed to minimize the cost of storing data on StarkNet. It builds on the concepts from the JavaScript implementation (v1) but with improved performance, better compression ratios, and a more robust API.

## Features

- Multiple compression algorithms (Brotli, LZ4, Zstandard)
- Custom encoding schemes for further data reduction
- Efficient binary-to-dot notation conversion
- Second-level dictionary compression
- CLI for easy compression/decompression
- Benchmarking tools for performance measurement

## Installation

Add StarkSqueeze to your `Cargo.toml`:

```toml
[dependencies]
stark-squeeze-v2 = { git = "https://github.com/supreme2580/stark-squeeze.git" }
```

## Usage

### Basic Compression

```rust
use stark_squeeze_v2::prelude::*;
use stark_squeeze_v2::compression::CompressionOptions;

fn main() -> Result<()> {
    // Read data
    let data = std::fs::read("input.txt")?;
    
    // Configure compression
    let options = CompressionOptions::default();
    
    // Compress data
    let compressed = compress(&data, options)?;
    
    // Save compressed data
    std::fs::write("output.bin", compressed)?;
    
    Ok(())
}
```

### CLI Usage

```bash
cargo run -- --input myfile.txt --output compressed.bin --level 6
```

## Compression Pipeline

1. Binary data → Binary string
2. Binary string → Dot notation (first-level encoding)
3. Dot notation → Symbol notation (second-level encoding)
4. Symbol notation → Brotli/LZ4/Zstd compression
5. Final compressed data

## Benchmarks

Run benchmarks to compare compression algorithms:

```bash
cargo bench
```

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## License

MIT

## Credits

StarkSqueeze is built upon the original concepts from the JavaScript implementation, adapted and optimized for Rust. 