### Contributors Group Chat

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