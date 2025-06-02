// ASCII Converter Module
// This module handles conversion of non-printable characters to printable ASCII
// before compression, ensuring compatibility and consistency

use std::io;
use std::collections::HashMap;
use colored::*;
use std::error::Error;

// ASCII printable character range: 32 (space) to 126 (~)
const ASCII_PRINTABLE_START: u8 = 32;
const ASCII_PRINTABLE_END: u8 = 126;

// Special handling for common control characters
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

// Structure to track conversion statistics
#[derive(Debug, Default)]
pub struct ConversionStats {
    pub total_bytes: usize,
    pub converted_bytes: usize,
    pub character_map: HashMap<u8, usize>, // Maps original char to count
}

impl ConversionStats {
    // Log conversion statistics
    pub fn log_summary(&self) {
        if self.converted_bytes == 0 {
            println!("{}", "✅ No character conversions needed - file already contains only printable ASCII!".green());
            return;
        }

        println!("{}", format!("📊 ASCII Conversion Summary:").blue().bold());
        println!("  {} Total bytes processed", self.total_bytes.to_string().cyan());
        println!("  {} Bytes converted ({:.2}%)",
            self.converted_bytes.to_string().yellow(),
            (self.converted_bytes as f64 / self.total_bytes as f64 * 100.0)
        );

        if !self.character_map.is_empty() {
            println!("\n{}", "  Character conversion details:".dimmed());
            let mut sorted_chars: Vec<_> = self.character_map.iter().collect();
            sorted_chars.sort_by_key(|(_, count)| *count);
            sorted_chars.reverse();

            for (byte, count) in sorted_chars.iter().take(10) {
                let char_repr = match **byte {
                    0..=31 => format!("0x{:02X} (control)", byte),
                    32..=126 => format!("0x{:02X} ('{}')", byte, **byte as char),
                    127 => "0x7F (DEL)".to_string(),
                    _ => format!("0x{:02X} (extended)", byte),
                };
                println!("    {} → converted {} times", char_repr.red(), count.to_string().yellow());
            }

            if self.character_map.len() > 10 {
                println!("    {} more unique characters...",
                    (self.character_map.len() - 10).to_string().dimmed());
            }
        }
    }
}

// Convert a single byte to printable ASCII
fn convert_byte_to_ascii(byte: u8, stats: &mut ConversionStats) -> u8 {
    if byte >= ASCII_PRINTABLE_START && byte <= ASCII_PRINTABLE_END {
        // Already printable
        return byte;
    }

    // Track conversion
    stats.converted_bytes += 1;
    *stats.character_map.entry(byte).or_insert(0) += 1;

    // Check special mappings first
    for &(from, to) in CHAR_MAPPINGS {
        if byte == from {
            return to;
        }
    }

    // For extended ASCII (128-255), map to printable range using modulo
    if byte > 127 {
        // Map to range 48-122 (0-9, A-Z, a-z)
        let mapped = 48 + (byte - 128) % 75;
        return mapped;
    }

    // For remaining control characters (16-26, 28-31), map to letters
    match byte {
        16..=26 => b'A' + (byte - 16),  // DLE-SUB → A-K
        28..=31 => b'L' + (byte - 28),  // FS-US → L-O
        _ => b'?', // Fallback (should not happen with above logic)
    }
}

// Main conversion function
pub fn convert_to_printable_ascii(data: &[u8]) -> Result<(Vec<u8>, ConversionStats), Box<dyn Error>> {
    let mut stats = ConversionStats {
        total_bytes: data.len(),
        ..Default::default()
    };

    // Pre-allocate result vector for efficiency
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

    // Create progress bar for conversion
    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(
        ProgressStyle::with_template("🔤 [{bar:40.cyan/blue}] {percent}% ⏳ Converting to ASCII...")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    // Process in chunks for progress updates
    let chunk_size = 8192; // 8KB chunks
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

    // Log conversion statistics
    stats.log_summary();

    Ok(result)
}

// Validation function to ensure data contains only printable ASCII
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
