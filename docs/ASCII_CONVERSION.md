# ASCII Conversion 🔤

StarkSqueeze automatically converts non-printable characters in your files to standard ASCII characters before compression.

## Why?

This ensures maximum compatibility and consistency for the compression process, especially for files with binary data or unusual characters.

## What Changes?

- **Non-Printable Control Characters** (e.g., NULL, TAB, ESC): Converted to printable ASCII equivalents (e.g., '0', space, 'E').
- **Extended ASCII Characters** (values 128-255): Mapped to standard printable ASCII characters.
- **Standard Printable ASCII Characters** (values 32-126): Remain unchanged.

## ⚠️ Important Note

This conversion is designed for data files. If you upload binary executables (e.g., `.exe`, `.dmg`, compiled programs), they will be converted and **will likely become non-functional**. This feature prioritizes data integrity and compressibility over preserving executable functionality.

This conversion is always active and cannot be disabled.
