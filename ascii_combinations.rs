use std::env;

/// Generates ASCII character combinations of specified length
/// 
/// # Arguments
/// * `length` - The length of each combination (default 5 for ASCII chunks)
/// * `start_index` - The index to start generating from (0-based)
/// * `count` - Number of combinations to generate
/// 
/// # Returns
/// A vector of strings containing the combinations
pub fn generate_ascii_combinations(length: usize, start_index: u64, count: usize) -> Vec<String> {
    const ASCII_CHARS: usize = 128;
    let mut result = Vec::with_capacity(count);
    
    // Calculate the starting combination from the index
    let mut current_combination = index_to_combination(start_index, length, ASCII_CHARS);
    
    for _ in 0..count {
        result.push(current_combination.clone());
        
        // Generate next combination
        if !increment_combination(&mut current_combination, ASCII_CHARS) {
            // We've reached the end of all possible combinations
            break;
        }
    }
    
    result
}

/// Converts an index to its corresponding combination
fn index_to_combination(mut index: u64, length: usize, base: usize) -> String {
    let mut combination = String::with_capacity(length);
    
    for _ in 0..length {
        let remainder = (index % base as u64) as u8;
        combination.push(remainder as char);
        index /= base as u64;
    }
    
    // Reverse to get correct order (least significant digit first)
    combination.chars().rev().collect()
}

/// Increments a combination to the next one
fn increment_combination(combination: &mut String, base: usize) -> bool {
    let mut chars: Vec<char> = combination.chars().collect();
    
    // Start from the rightmost character
    for i in (0..chars.len()).rev() {
        let current_value = chars[i] as u8;
        
        if current_value < (base - 1) as u8 {
            chars[i] = (current_value + 1) as char;
            *combination = chars.into_iter().collect();
            return true;
        } else {
            // Carry over to next position
            chars[i] = 0 as char;
        }
    }
    
    // If we get here, we've overflowed (all characters are at max value)
    false
}

/// Alternative implementation using iterator pattern for memory efficiency
pub struct AsciiCombinationIterator {
    current_index: u64,
    length: usize,
    base: usize,
    max_combinations: u64,
}

impl AsciiCombinationIterator {
    pub fn new(length: usize, start_index: u64) -> Self {
        let base: usize = 128; // ASCII characters
        let max_combinations = base.pow(length as u32) as u64;
        
        Self {
            current_index: start_index,
            length,
            base,
            max_combinations,
        }
    }
}

impl Iterator for AsciiCombinationIterator {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.max_combinations {
            return None;
        }
        
        let combination = index_to_combination(self.current_index, self.length, self.base);
        self.current_index += 1;
        Some(combination)
    }
}

/// Generate combinations using iterator (more memory efficient for large ranges)
pub fn generate_ascii_combinations_iter(length: usize, start_index: u64, count: usize) -> Vec<String> {
    AsciiCombinationIterator::new(length, start_index)
        .take(count)
        .collect()
}

fn print_usage() {
    println!("ASCII Combination Generator");
    println!("Usage: rustc ascii_combinations.rs && ./ascii_combinations [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -l, --length <LENGTH>     Length of each combination (default: 5)");
    println!("  -s, --start <INDEX>       Starting index (default: 0)");
    println!("  -c, --count <COUNT>       Number of combinations to generate (default: 10)");
    println!("  -h, --help               Show this help message");
    println!();
    println!("Examples:");
    println!("  ./ascii_combinations                           # Generate first 10 combinations of length 5");
    println!("  ./ascii_combinations -l 3 -c 5                # Generate 5 combinations of length 3");
    println!("  ./ascii_combinations -s 1000000 -c 3          # Start from index 1M, generate 3 combinations");
    println!("  ./ascii_combinations -l 5 -s 1000000000 -c 5  # Start from 1B, generate 5 combinations of length 5");
}

fn parse_args() -> Result<(usize, u64, usize), String> {
    let args: Vec<String> = env::args().collect();
    let mut length = 5;
    let mut start_index = 0;
    let mut count = 10;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "-l" | "--length" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value for --length".to_string());
                }
                length = args[i].parse().map_err(|_| "Invalid length value".to_string())?;
            }
            "-s" | "--start" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value for --start".to_string());
                }
                start_index = args[i].parse().map_err(|_| "Invalid start index".to_string())?;
            }
            "-c" | "--count" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value for --count".to_string());
                }
                count = args[i].parse().map_err(|_| "Invalid count value".to_string())?;
            }
            _ => {
                return Err(format!("Unknown argument: {}", args[i]));
            }
        }
        i += 1;
    }
    
    Ok((length, start_index, count))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_combinations() {
        let combinations = generate_ascii_combinations(3, 0, 10);
        assert_eq!(combinations.len(), 10);
        assert_eq!(combinations[0], "\0\0\0"); // First combination
        assert_eq!(combinations[1], "\0\0\x01"); // Second combination
    }
    
    #[test]
    fn test_start_from_index() {
        let combinations = generate_ascii_combinations(2, 1000, 5);
        assert_eq!(combinations.len(), 5);
        // The 1000th combination should be different from the first few
        assert_ne!(combinations[0], "\0\0");
    }
    
    #[test]
    fn test_iterator() {
        let combinations: Vec<String> = AsciiCombinationIterator::new(2, 0).take(5).collect();
        assert_eq!(combinations.len(), 5);
    }
    
    #[test]
    fn test_large_start_index() {
        // Test starting from a large index (1 billion)
        let start_index = 1_000_000_000;
        let combinations = generate_ascii_combinations(5, start_index, 3);
        assert_eq!(combinations.len(), 3);
        
        // All combinations should be different
        assert_ne!(combinations[0], combinations[1]);
        assert_ne!(combinations[1], combinations[2]);
    }
}

fn main() {
    let (length, start_index, count) = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage();
            std::process::exit(1);
        }
    };
    
    // Calculate total possible combinations
    let total_combinations = 128u64.pow(length as u32);
    
    println!("ASCII Combination Generator");
    println!("Length: {}", length);
    println!("Starting index: {}", start_index);
    println!("Count: {}", count);
    println!("Total possible combinations for length {}: {}", length, total_combinations);
    println!();
    
    if start_index >= total_combinations {
        eprintln!("Error: Start index {} is beyond the maximum possible combinations ({})", start_index, total_combinations);
        std::process::exit(1);
    }
    
    // Calculate actual size requirements
    let bytes_per_combination = length;
    let total_bytes = count as u64 * bytes_per_combination as u64;
    let total_mb = total_bytes as f64 / (1024.0 * 1024.0);
    let total_gb = total_mb / 1024.0;
    
    println!("Size requirements:");
    println!("  Bytes per combination: {}", bytes_per_combination);
    println!("  Total bytes: {}", total_bytes);
    if total_mb < 1024.0 {
        println!("  Total size: {:.2} MB", total_mb);
    } else {
        println!("  Total size: {:.2} GB", total_gb);
    }
    println!();
    
    // Generate combinations
    let combinations = generate_ascii_combinations(length, start_index, count);
    
    println!("Generated {} combinations:", combinations.len());
    for (i, combo) in combinations.iter().enumerate() {
        let actual_index = start_index + i as u64;
        println!("[{}] {:?}", actual_index, combo);
    }
    
    // Show some statistics
    if combinations.len() > 1 {
        println!();
        println!("Statistics:");
        println!("  First combination: {:?}", combinations[0]);
        println!("  Last combination: {:?}", combinations[combinations.len() - 1]);
        println!("  Index range: {} to {}", start_index, start_index + combinations.len() as u64 - 1);
    }
} 