# Stark Squeeze - Compression System

## Contributors Group Chat

Telegram group chat link: <https://t.me/+IfwMzjTrmI5kODk0>

## File Conversion Pipeline

New: All file-to-string conversions now use ASCII-safe encoding.  
Use file_to_ascii(input_file) to convert a file to a human-readable ASCII string (1 char = 1 byte, ASCII 0-126 only).

Deprecated: The previous binary conversion (`file_to_binary`) is now deprecated and should not be used for new workflows.

### Error Handling

If a file contains non-ASCII bytes (outside 0-126), file_to_ascii will return a clear error message and abort the operation.

### Migration

- Update your code to use file_to_ascii instead of file_to_binary.
- Update your tests to use ASCII sample files.

## 📚 Compression System Documentation

This section explains the mathematical foundation behind the compression system, helping developers and users understand the underlying logic behind compression performance.

### 📘 Dictionary File Size Formula

The dictionary size is determined by the number of unique chunks found in your data:

```text
Dictionary Size = Number of Unique Chunks × (Chunk Size + 1 byte)
```

**Where:**

- **Number of Unique Chunks**: Count of distinct byte sequences in the data
- **Chunk Size**: Optimized size per chunk (typically 2–8 bytes)
- **+1 byte**: Each unique chunk is mapped to a single byte (u8)

**Example:**
For 100 unique 4-byte chunks:

```text
Dictionary Size = 100 × (4 + 1) = 500 bytes
```

### 📦 Compression Percentage Formula

```text
Compression % = (1 - (Compressed Size / Original Size)) × 100%
```

**Where:**

- **Original Size**: Size of the raw input file in bytes
- **Compressed Size**: Number of chunks (each chunk = 1 byte)

**Example:**
For a 1000-byte file using 4-byte chunks:

- Number of chunks = 1000 ÷ 4 = 250
- Compressed Size = 250 bytes
- Compression % = (1 - 250 ÷ 1000) × 100% = 75%

### 🧮 Total Storage Formula

```text
Total Storage = Dictionary Size + Compressed Data Size
```

**Where:**

- **Dictionary Size**: As defined above
- **Compressed Data Size**: 1 byte per chunk

### 🧩 Key Constraints

- **Max Dictionary Size**: 255 unique chunks (limited by u8)
- **Chunk Size Range**: 2–8 bytes (auto-optimized for >90% compression)
- **ASCII Safety**: Files are converted to printable ASCII before compression

### 💡 Why This System Achieves High Compression Rates

This mathematical breakdown explains why the system achieves high compression rates:

1. **Multi-byte to Single-byte Reduction**: By reducing multi-byte sequences to single-byte references
2. **Minimal Dictionary Overhead**: Keeping dictionary size proportional to unique patterns
3. **Optimized Chunk Sizing**: Auto-selecting chunk sizes that maximize compression efficiency
4. **ASCII Encoding**: Ensuring data integrity while maintaining human-readable format

The compression effectiveness depends on the repetition patterns in your data — files with more repeated sequences will achieve higher compression rates.