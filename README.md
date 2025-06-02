# StarkSqueeze 🗜️

A high-performance data compression and storage solution for StarkNet, featuring automatic ASCII conversion for enhanced compatibility.

## Features ✨

### 🔤 Automatic ASCII Conversion (New!)
- **Automatic Detection**: Files are automatically scanned for non-printable characters
- **Intelligent Mapping**: Non-printable characters are converted to their printable ASCII equivalents
- **Transparency**: Conversion happens seamlessly in the pre-compression pipeline
- **Statistics**: Detailed logging of conversion statistics and character mappings
- **Performance**: Minimal overhead with progress indication for large files

[Read more about ASCII Conversion →](docs/ASCII_CONVERSION.md)

### 📦 Core Features
- **File Compression**: Efficient two-stage encoding for optimal compression
- **StarkNet Integration**: Direct upload and retrieval from StarkNet
- **Progress Tracking**: Visual progress bars for all operations
- **CLI Interface**: User-friendly command-line interface

## Installation

```bash
# Clone the repository
git clone https://github.com/onlydust/stark-squeeze.git
cd stark-squeeze

# Build the project
cargo build --release
```

## Usage

### Basic Commands

```bash
# Upload a file (ASCII conversion happens automatically)
stark-squeeze upload myfile.bin

# Retrieve a file
stark-squeeze retrieve <upload-id>

# List all uploads
stark-squeeze list
```

### Example Output

```
🔄 Converting file to printable ASCII...
🔤 [████████████████████] 100% ⏳ Converting to ASCII...
✅ ASCII conversion complete!
📊 ASCII Conversion Summary:
  1024 Total bytes processed
  156 Bytes converted (15.23%)

  Character conversion details:
    0xFF (extended) → converted 45 times
    0x00 (control) → converted 23 times
    0x0A (control) → converted 12 times

📦 [████████████████████] 100% ⏳ 1024/1024 read
✅ File loaded into memory! 🎉
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test ascii_conversion

# Run example
cargo run --example ascii_conversion
```

## Architecture

```
StarkSqueeze
├── ASCII Converter     # Converts non-printable to printable ASCII
├── Binary Encoder      # Converts data to binary representation
├── Two-Stage Encoding  # Applies compression algorithms
└── StarkNet Client    # Handles blockchain storage
```

## Documentation

- [ASCII Conversion Feature](docs/ASCII_CONVERSION.md)
- [API Reference](docs/API.md) (Coming soon)
- [Architecture Guide](docs/ARCHITECTURE.md) (Coming soon)

## Contributing

We welcome contributions! Please check out our [Contributing Guide](CONTRIBUTING.md) for details.

### Contributors Group Chat

Telegram group chat link: <https://t.me/+IfwMzjTrmI5kODk0>

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- StarkNet community for blockchain infrastructure
- Contributors and testers who helped shape this project
