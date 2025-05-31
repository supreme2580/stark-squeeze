use stark_squeeze::dictionary::{CustomDictionary, Dictionary};
use stark_squeeze::{encoding_one_with_dict, decoding_one_with_dict};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom dictionary
    let mut custom_dict = CustomDictionary::new();
    
    // Add some mappings
    custom_dict.insert("00000".to_string(), "A".to_string());
    custom_dict.insert("00001".to_string(), "B".to_string());
    custom_dict.insert("00010".to_string(), "C".to_string());
    custom_dict.insert("00011".to_string(), "D".to_string());
    custom_dict.insert("00100".to_string(), "E".to_string());

    // Example binary string
    let binary = "00000000010001000011";

    // Encode using custom dictionary
    println!("Encoding with custom dictionary...");
    let encoded = encoding_one_with_dict(binary, &custom_dict)?;
    println!("Encoded: {}", encoded);

    // Decode using custom dictionary
    println!("\nDecoding with custom dictionary...");
    let decoded = decoding_one_with_dict(&encoded, &custom_dict)?;
    println!("Decoded: {}", decoded);

    // Verify the round trip
    assert_eq!(binary, decoded);
    println!("\n✅ Round trip successful!");

    // Example of loading dictionary from file
    println!("\nLoading dictionary from file...");
    let file_dict = CustomDictionary::from_file("examples/custom_dict.txt")?;
    println!("Dictionary loaded with {} entries", file_dict.len());

    Ok(())
} 