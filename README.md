# Stark Squeeze

## Overview

**Stark Squeeze** is a next-generation, high-compression file storage and retrieval system designed for the Starknet blockchain ecosystem. It enables users to compress, store, and reconstruct files with extremely high efficiency, leveraging both on-chain and off-chain components. The system is built in Rust and Cairo, and is designed for extensibility, transparency, and developer-friendliness.

---

## 🌟 Vision

- **Decentralized, verifiable file storage** for Starknet and beyond
- **>90% compression** for most file types using chunk-based, dictionary-driven algorithms
- **On-chain metadata and mapping** for trustless file reconstruction
- **User-friendly CLI and Web API** for seamless integration and adoption
- **Open, auditable, and extensible** for the Starknet community

---

## 🏗️ High-Level Architecture & Flow

> **Note:** If you do not see a rendered diagram below, please view this README on a platform that supports Mermaid diagrams, or refer to the ASCII diagram below.

### ASCII Diagram (Always Visible)

```
File Upload (CLI/Web)
        |
        v
ASCII-safe conversion
        |
        v
Chunking & Dictionary Mapping
        |
        v
Compression
        |
        +-------------------+
        |                   |
        v                   v
Store mapping &      Store compressed
metadata on-chain    file off-chain
        |                   |
        v                   v
   On-chain:           Off-chain:
   file hash,          compressed file,
   mapping,            mapping file
   metadata                 |
        |                   |
        +---------+---------+
                  |
                  v
        User retrieves file via hash/ID
                  |
                  v
        Reconstruction: decompress, reverse mapping, restore original file
```

### Mermaid Diagram (Rich Display, if supported)

```mermaid
graph TD
    A[User uploads file (CLI/Web)] --> B[ASCII-safe conversion]
    B --> C[Chunking & Dictionary Mapping]
    C --> D[Compression]
    D --> E[Store mapping & metadata on Starknet]
    D --> F[Store compressed file off-chain]
    E --> G[On-chain: file hash, mapping, metadata]
    F --> H[Off-chain: compressed file, mapping file]
    G --> I[User retrieves file via hash/ID]
    H --> I
    I --> J[Reconstruction: decompress, reverse mapping, restore original file]
```

---

## 🚀 Full System Flow

1. **File Upload**: User uploads a file via CLI or the HTTP server (`/compress` endpoint).
2. **ASCII Conversion**: File is converted to a printable ASCII-safe format for universal compatibility.
3. **Chunking & Mapping**: The ASCII data is split into optimal-sized chunks (2–8 bytes), and a dictionary is built mapping each unique chunk to a single byte.
4. **Compression**: The file is compressed by replacing each chunk with its dictionary byte, achieving >90% compression for many files.
5. **Hashing & Metadata**: The compressed data is hashed (SHA256) to generate a unique file ID. Metadata (original size, compressed size, file type, etc.) is prepared.
6. **On-chain Storage**: The mapping, metadata, and file hash are uploaded to a Starknet smart contract for verifiable, decentralized reference.
7. **Off-chain Storage**: The compressed file and mapping file are stored locally or on a decentralized storage network (IPFS/Arweave integration planned).
8. **Retrieval**: Users can download the mapping file and reconstruct the original file using the mapping and the compressed data, fully verifiable via on-chain metadata.

---

## 🖥️ Server API Documentation

### Running the Server

```bash
cargo run --bin server
```

- Server runs at `http://localhost:3000`

### Endpoints

#### Health Check
```bash
curl http://localhost:3000/health
```

#### Status
```bash
curl http://localhost:3000/status
```

#### Compress a File
```bash
curl -X POST http://localhost:3000/compress \
  -F "file=@/path/to/your/file.png"
```
- Returns JSON with compression stats and a download URL for the mapping file.

#### Download Mapping File
```bash
curl -O http://localhost:3000/files/{file_id}
```
- Downloads the mapping file for the compressed file.

---

## 🧩 Compression Pipeline

1. **ASCII Conversion**: Converts all bytes to printable ASCII (0–126) for universal compatibility.
2. **Chunking**: Splits the ASCII data into optimal-sized chunks (auto-optimized for best compression).
3. **Dictionary Mapping**: Maps each unique chunk to a single byte (max 255 unique chunks).
4. **Compression**: Replaces each chunk with its mapped byte, drastically reducing file size.
5. **Mapping File**: Stores the mapping and metadata needed for full, lossless reconstruction.
6. **On-chain Metadata**: Stores file hash, mapping, and compression stats on Starknet for verifiability.

---

## 📝 Smart Contract Integration

- **On-chain**: Stores file hash, mapping, compression ratio, and metadata using a Cairo contract.
- **Off-chain**: Stores the actual compressed file and mapping file (local or decentralized storage).
- **Reconstruction**: Anyone can verify and reconstruct the file using the on-chain mapping and the off-chain compressed data.

---

## 🛠️ How to Use

### CLI (for advanced users)
- Run CLI commands for compression, mapping, and upload (see `src/cli.rs` for details).

### HTTP Server (recommended)
- Start the server: `cargo run --bin server`
- Use `/compress` endpoint to upload and compress files
- Use `/files/{file_id}` to download mapping files
- Use `/status` and `/health` for monitoring

### Web Frontend
- A simple HTML frontend is provided in `public/index.html` for drag-and-drop uploads and status monitoring.

---

## ✅ What Has Been Built So Far

- **Rust backend**: Full compression pipeline, mapping, and file handling
- **Cairo smart contract**: On-chain storage of mapping and metadata
- **HTTP server**: File upload, compression, and mapping download endpoints
- **CLI**: Advanced command-line interface for power users
- **Web frontend**: Simple drag-and-drop UI for file uploads
- **Test coverage**: Unit tests for core compression and conversion logic
- **Documentation**: Mathematical formulas, API docs, and usage guides

---

## 🔮 What’s Next / Planned

- **IPFS/Arweave integration** for decentralized file storage
- **User authentication and file ownership**
- **Batch uploads and large file support**
- **Gas optimization and calldata minimization**
- **Advanced analytics and monitoring**
- **Security audits and formal verification**
- **Community engagement and open-source contributions**

---

## ⚠️ Technical Notes & Limitations

- **Max dictionary size**: 255 unique chunks (u8 mapping)
- **Chunk size**: Auto-optimized between 2–8 bytes
- **ASCII safety**: All files are converted to printable ASCII before compression
- **On-chain storage**: Only mapping and metadata are stored on-chain; actual file data is off-chain
- **Compression effectiveness**: Highest for files with repeated patterns; less effective for highly random data

---

## 💬 Community & Support

- Join the [Telegram group](https://t.me/+IfwMzjTrmI5kODk0) for questions, feedback, and contributions!

---

## 📜 License

MIT or Apache 2.0 (choose your preferred open-source license)