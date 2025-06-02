# ASCII Conversion Feature Documentation

## Overview

The ASCII conversion feature in StarkSqueeze automatically converts all non-printable characters in files to printable ASCII characters before compression. This ensures compatibility and consistency across different systems and storage mechanisms.

## Features

### 1. Automatic Detection and Conversion
- Files are automatically scanned for non-printable characters
- All characters outside the printable ASCII range (32-126) are converted
- Conversion happens transparently in the pre-compression pipeline

### 2. Intelligent Character Mapping
The conversion uses intelligent mapping to preserve meaning where possible:

#### Control Characters (0-31)
- `NULL (0x00)` → `'0'`
- `TAB (0x09)` → `' '` (space)
- `LF (0x0A)` → `' '` (space)
- `CR (0x0D)` → `' '` (space)
- `ESC (0x1B)` → `'E'`
- Other control characters are mapped to alphanumeric equivalents

#### Extended ASCII (128-255)
- Mapped to printable characters using a modulo algorithm
- Ensures consistent conversion across the range

### 3. Conversion Statistics
The system provides detailed statistics about conversions:
- Total bytes processed
- Number of bytes converted
- Character frequency analysis
- Top 10 most frequently converted characters

### 4. Performance Optimization
- Efficient byte-by-byte conversion
- Progress indication for large files
- Minimal memory overhead
- Pre-allocated buffers for speed

## Usage

### Basic Usage
When you upload a file through StarkSqueeze, ASCII conversion happens automatically:

```bash
stark-squeeze upload myfile.bin
```

You'll see:
```
🔄 Converting file to printable ASCII...
🔤 [████████████████████] 100% ⏳ Converting to ASCII...
✅ ASCII conversion complete!
📊 ASCII Conversion Summary:
  1024 Total bytes processed
  156 Bytes converted (15.23%)
```

### Programmatic Usage
```rust
use stark_squeeze::ascii_converter::{convert_to_printable_ascii, validate_printable_ascii};

// Convert data to ASCII
let data = vec![0xFF, 0x00, b'H', b'i', 0x0A];
let (converted, stats) = convert_to_printable_ascii(&data)?;

// Validate data is ASCII
if validate_printable_ascii(&converted).is_ok() {
    println!("Data is valid ASCII!");
}
```

## Character Mapping Table

| Original Character | Type | Converted To | Notes |
|-------------------|------|--------------|-------|
| 0x00 (NULL) | Control | '0' | Numeric zero |
| 0x09 (TAB) | Control | ' ' | Space |
| 0x0A (LF) | Control | ' ' | Space |
| 0x0D (CR) | Control | ' ' | Space |
| 0x1B (ESC) | Control | 'E' | Letter E |
| 0x20-0x7E | Printable | Unchanged | Already ASCII |
| 0x7F (DEL) | Control | 'D' | Letter D |
| 0x80-0xFF | Extended | Varies | Mapped to 0-9, A-Z, a-z range |

## Examples

### Example 1: Binary File with Mixed Content
```bash
# Original file contains:
# - Text: "Hello"
# - Binary: 0xFF, 0x00, 0xAB
# - Control: TAB, LF

stark-squeeze upload mixed_content.bin

# Output shows conversion details:
# 0xFF → 'W' (converted)
# 0x00 → '0' (converted)
# 0xAB → 'k' (converted)
# TAB → ' ' (converted)
# LF → ' ' (converted)
```

### Example 2: Already ASCII File
```bash
# File contains only printable ASCII

stark-squeeze upload readme.txt

# Output:
# ✅ No character conversions needed - file already contains only printable ASCII!
```

## Edge Cases and Limitations

### 1. Information Loss
- Non-printable characters are permanently converted
- Original byte values cannot be recovered after conversion
- This is by design for compatibility

### 2. File Size
- File size remains the same (1:1 byte mapping)
- No expansion or contraction occurs during conversion

### 3. Binary Files
- Binary files (executables, images) will be converted
- They will not be executable after conversion
- This feature is intended for data storage, not executable preservation

## Testing

Run the test suite to verify ASCII conversion:

```bash
# Run all tests
cargo test

# Run ASCII conversion tests specifically
cargo test ascii_conversion

# Run the example
cargo run --example ascii_conversion
```

## Performance Considerations

- **Small files (<1MB)**: Negligible overhead, typically <10ms
- **Medium files (1-100MB)**: Linear scaling, ~10-50ms per MB
- **Large files (>100MB)**: Progress indication shown, ~10ms per MB

## Configuration

Currently, ASCII conversion is always enabled and cannot be disabled. This ensures consistency across all compressed data in the StarkSqueeze system.

## Future Enhancements

1. **Configurable Mappings**: Allow custom character mapping tables
2. **Reversible Conversion**: Option to store mapping metadata for reversal
3. **Encoding Options**: Support for different ASCII-safe encodings (Base64, etc.)
4. **Selective Conversion**: Convert only specific file types

## Troubleshooting

### Issue: Unexpected Character Conversions
**Solution**: Check the conversion statistics log to see which characters were converted and their frequencies.

### Issue: Performance Impact
**Solution**: The conversion is optimized for speed. If experiencing slowdowns, check disk I/O as the likely bottleneck.

### Issue: File Corruption After Conversion
**Solution**: This is expected for binary executables. The feature is designed for data files, not executable preservation.
