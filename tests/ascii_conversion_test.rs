// Integration tests for ASCII conversion feature
use stark_squeeze::ascii_converter::{convert_to_printable_ascii, validate_printable_ascii};
use stark_squeeze::{file_to_binary, encoding_one, encoding_two};
use std::fs::{write, remove_file};
use std::path::Path;

#[test]
fn test_ascii_conversion_integration() {
    let test_file = "test_ascii_integration.bin";
    let test_data = vec![
        72, 101, 108, 108, 111,  // "Hello"
        0, 10, 13,               // Control chars
        200, 255,                // Extended ASCII
        32, 87, 111, 114, 108, 100, // " World"
    ];

    write(test_file, &test_data).expect("Failed to write test file");

    let result = file_to_binary(test_file).expect("Failed to read and convert file");

    assert!(validate_printable_ascii(&result).is_ok(), "Result contains non-printable characters");

    remove_file(test_file).ok();
}

#[test]
fn test_compression_with_ascii_conversion() {
    let test_file = "test_compression_ascii.bin";
    let test_data = vec![
        0xFF, 0x00, 0xAB, 0xCD, 0xEF,
        b'T', b'e', b's', b't',
        0x01, 0x02, 0x03,
    ];

    write(test_file, &test_data).expect("Failed to write test file");

    let ascii_data = file_to_binary(test_file).expect("Failed to read file");

    assert!(validate_printable_ascii(&ascii_data).is_ok());

    let binary_string: String = ascii_data.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();

    let encoded_one = encoding_one(&binary_string).expect("First encoding failed");
    let encoded_two = encoding_two(&encoded_one).expect("Second encoding failed");

    assert!(!encoded_two.is_empty(), "Encoding produced empty output");

    remove_file(test_file).ok();
}

#[test]
fn test_already_ascii_file() {
    let test_file = "test_already_ascii.txt";
    let ascii_content = b"This is already ASCII text!";

    write(test_file, ascii_content).expect("Failed to write test file");

    let result = file_to_binary(test_file).expect("Failed to read file");

    assert_eq!(result.len(), ascii_content.len());

    remove_file(test_file).ok();
}

#[test]
fn test_edge_cases() {
    let (empty_result, empty_stats) = convert_to_printable_ascii(&[]).expect("Failed empty conversion");
    assert_eq!(empty_result.len(), 0);
    assert_eq!(empty_stats.converted_bytes, 0);

    let control_chars = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (control_result, control_stats) = convert_to_printable_ascii(&control_chars).expect("Failed control conversion");
    assert_eq!(control_stats.converted_bytes, control_chars.len());
    assert!(validate_printable_ascii(&control_result).is_ok());

    let extended_ascii: Vec<u8> = (128..=255).collect();
    let (extended_result, extended_stats) = convert_to_printable_ascii(&extended_ascii).expect("Failed extended conversion");
    assert_eq!(extended_stats.converted_bytes, extended_ascii.len());
    assert!(validate_printable_ascii(&extended_result).is_ok());
}

#[test]
fn test_performance_large_file() {
    let test_file = "test_large_ascii.bin";
    let size = 1024 * 1024; // 1MB

    let mut test_data = Vec::with_capacity(size);
    for i in 0..size {
        test_data.push((i % 256) as u8);
    }

    write(test_file, &test_data).expect("Failed to write large test file");

    let start = std::time::Instant::now();
    let result = file_to_binary(test_file).expect("Failed to read large file");
    let duration = start.elapsed();

    println!("ASCII conversion of 1MB took: {:?}", duration);

    assert!(validate_printable_ascii(&result).is_ok());
    assert_eq!(result.len(), test_data.len());

    remove_file(test_file).ok();
}
