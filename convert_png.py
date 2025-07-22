#!/usr/bin/env python3

def reverse_ascii_conversion(ascii_byte):
    """Reverse the ASCII conversion mapping to restore original bytes"""
    
    # Reverse the mapping from the original ASCII converter
    ascii_to_original = {
        ord('0'): 0,    # NULL
        ord('1'): 1,    # SOH
        ord('2'): 2,    # STX
        ord('3'): 3,    # ETX
        ord('4'): 4,    # EOT
        ord('5'): 5,    # ENQ
        ord('6'): 6,    # ACK
        ord('7'): 7,    # BEL
        ord('b'): 8,    # BS (backspace)
        ord(' '): 10,   # Most common space mapping (LF)
        ord('v'): 11,   # VT (vertical tab)
        ord('f'): 12,   # FF (form feed)
        ord('e'): 14,   # SO
        ord('E'): 27,   # ESC
        ord('D'): 127,  # DEL
    }
    
    # Characters A-K map to 16-26
    for i in range(11):  # A to K
        ascii_to_original[ord('A') + i] = 16 + i
    
    # Characters L-O map to 28-31
    for i in range(4):   # L to O
        ascii_to_original[ord('L') + i] = 28 + i
    
    # Extended ASCII mappings (128-255) were mapped to range 48-123
    # This is approximate reverse mapping
    for i in range(76):  # 0-75
        ascii_to_original[48 + i] = 128 + i
    
    # Return the mapped value or the original byte if printable ASCII
    if ascii_byte in ascii_to_original:
        return ascii_to_original[ascii_byte]
    elif 32 <= ascii_byte <= 126:  # Printable ASCII remains unchanged
        return ascii_byte
    else:
        return ascii_byte  # Fallback

def convert_ascii_to_png(input_file, output_file):
    """Convert ASCII-encoded PNG data back to binary PNG"""
    
    print(f"Reading ASCII data from {input_file}...")
    
    # Read the raw file
    with open(input_file, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # Extract just the data part (before filename and size info)
    lines = content.strip().split('\n')
    if len(lines) < 3:
        print("Error: Invalid file format")
        return False
    
    # Remove last 2 lines (filename and size metadata)
    data_lines = lines[:-2]
    ascii_data = '\n'.join(data_lines)
    
    print(f"ASCII data length: {len(ascii_data)} characters")
    
    # Convert ASCII characters to bytes
    print("Converting ASCII characters to bytes...")
    ascii_bytes = []
    for char in ascii_data:
        ascii_bytes.append(ord(char))
    
    print(f"Converted to {len(ascii_bytes)} ASCII bytes")
    
    # Apply reverse ASCII conversion
    print("Applying reverse ASCII conversion...")
    original_bytes = []
    for byte in ascii_bytes:
        original_byte = reverse_ascii_conversion(byte)
        original_bytes.append(original_byte)
    
    print(f"Converted to {len(original_bytes)} original bytes")
    
    # Write as binary file
    print(f"Writing binary PNG to {output_file}...")
    with open(output_file, 'wb') as f:
        f.write(bytes(original_bytes))
    
    # Check if it looks like a PNG
    if len(original_bytes) >= 8:
        png_header = original_bytes[:8]
        expected_png = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
        
        print(f"First 8 bytes: {png_header}")
        print(f"Expected PNG header: {expected_png}")
        
        if png_header == expected_png:
            print("✅ Valid PNG header detected!")
        else:
            print("⚠️  PNG header doesn't match - file may need additional processing")
    
    print(f"Conversion complete! Output file: {output_file}")
    return True

if __name__ == "__main__":
    # Convert the file.png.raw to file.png
    success = convert_ascii_to_png("file.png.raw", "file_recovered.png")
    
    if success:
        print("\n🎉 Conversion completed!")
        print("Try opening 'file_recovered.png' to see if it's a valid image.")
        print("\nIf the image still doesn't work, the ASCII conversion may have been lossy")
        print("and the original mapping file would be needed for perfect reconstruction.")
    else:
        print("\n❌ Conversion failed!")
