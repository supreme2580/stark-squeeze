// ASCII Converter Module
// This module handles conversion of non-printable characters to printable ASCII
// before compression, ensuring compatibility and consistency

use std::io;
use std::collections::HashMap;
use std::error::Error;

// ASCII printable character range: 32 (space) to 126 (~)
const ASCII_PRINTABLE_START: u8 = 32;
const ASCII_PRINTABLE_END: u8 = 126;

const CHAR_MAPPINGS: &[(u8, u8)] = &[
    (0, b'0'),    // NULL → '0'
    (1, b'1'),    // SOH → '1'
    (2, b'2'),    // STX → '2'
    (3, b'3'),    // ETX → '3'
    (4, b'4'),    // EOT → '4'
    (5, b'5'),    // ENQ → '5'
    (6, b'6'),    // ACK → '6'
    (7, b'7'),    // BEL → '7'
    (8, b'b'),    // BS → 'b' (backspace)
    (9, b' '),    // TAB → space
    (10, b' '),   // LF → space (newline)
    (11, b'v'),   // VT → 'v' (vertical tab)
    (12, b'f'),   // FF → 'f' (form feed)
    (13, b' '),   // CR → space (carriage return)
    (14, b'e'),   // SO → 'e'
    (15, b'f'),   // SI → 'f'
    // 16-31: DLE to US → map to various printable chars
    (27, b'E'),   // ESC → 'E'
    (127, b'D'),  // DEL → 'D'
];

#[derive(Debug, Default)]
pub struct ConversionStats {
    pub total_bytes: usize,
    pub converted_bytes: usize,
    pub character_map: HashMap<u8, usize>,
}

fn convert_byte_to_ascii(byte: u8, stats: &mut ConversionStats) -> u8 {
    if byte >= ASCII_PRINTABLE_START && byte <= ASCII_PRINTABLE_END {
        return byte;
    }

    stats.converted_bytes += 1;
    *stats.character_map.entry(byte).or_insert(0) += 1;

    for &(from, to) in CHAR_MAPPINGS {
        if byte == from {
            return to;
        }
    }

    if byte > 127 {
        let mapped = 48 + (byte - 128) % 75;
        return mapped;
    }

    match byte {
        16..=26 => b'A' + (byte - 16),
        28..=31 => b'L' + (byte - 28),
        _ => b'?',
    }
}

pub fn convert_to_printable_ascii(data: &[u8]) -> Result<(Vec<u8>, ConversionStats), Box<dyn Error>> {
    let mut stats = ConversionStats {
        total_bytes: data.len(),
        ..Default::default()
    };
    let mut result = Vec::with_capacity(data.len());

    // Convert each byte
    for &byte in data {
        result.push(convert_byte_to_ascii(byte, &mut stats));
    }

    Ok((result, stats))
}

// Wrapper function for file conversion with progress indication
pub fn convert_file_to_ascii(file_data: Vec<u8>) -> io::Result<Vec<u8>> {
    use indicatif::{ProgressBar, ProgressStyle};

    let total_size = file_data.len();
    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(
        ProgressStyle::with_template("🔤 [{bar:40.cyan/blue}] {percent}% ⏳ Converting to ASCII...")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    let chunk_size = 8192;
    let mut result = Vec::with_capacity(total_size);
    let mut stats = ConversionStats {
        total_bytes: total_size,
        ..Default::default()
    };

    for chunk in file_data.chunks(chunk_size) {
        for &byte in chunk {
            result.push(convert_byte_to_ascii(byte, &mut stats));
        }
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message("✅ ASCII conversion complete!");
    Ok(result)
}

pub fn validate_printable_ascii(data: &[u8]) -> Result<(), String> {
    for (i, &byte) in data.iter().enumerate() {
        if byte < ASCII_PRINTABLE_START || byte > ASCII_PRINTABLE_END {
            return Err(format!(
                "Non-printable character found at position {}: 0x{:02X}",
                i, byte
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printable_ascii_unchanged() {
        let input = b"Hello, World!";
        let (result, stats) = convert_to_printable_ascii(input).unwrap();
        assert_eq!(result, input.to_vec());
        assert_eq!(stats.converted_bytes, 0);
    }

    #[test]
    fn test_control_characters_conversion() {
        let input = vec![0, 9, 10, 13, 27]; // NULL, TAB, LF, CR, ESC
        let expected = vec![b'0', b' ', b' ', b' ', b'E'];
        let (result, stats) = convert_to_printable_ascii(&input).unwrap();
        assert_eq!(result, expected);
        assert_eq!(stats.converted_bytes, 5);
    }

    #[test]
    fn test_extended_ascii_conversion() {
        let input = vec![128, 200, 255];
        let (result, stats) = convert_to_printable_ascii(&input).unwrap();
        assert!(result.iter().all(|&b| b >= 32 && b <= 126));
        assert_eq!(stats.converted_bytes, 3);
    }

    #[test]
    fn test_validation_function() {
        let valid = b"Valid ASCII!";
        assert!(validate_printable_ascii(valid).is_ok());

        let invalid = vec![0, 65, 127];
        assert!(validate_printable_ascii(&invalid).is_err());
    }
}
