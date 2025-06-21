use stark_squeeze::{
    mapping::{create_complete_mapping, save_mapping, load_mapping, reverse_compression},
    compression::{create_chunk_mapping, compress_data},
    ascii_converter::{convert_to_printable_ascii, ConversionStats},
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 StarkSqueeze Comprehensive Mapping Demo");
    println!("==========================================\n");

    // Test data with some non-printable characters
    let original_data = vec![
        0,    // NULL character
        72,   // 'H'
        101,  // 'e'
        108,  // 'l'
        108,  // 'l'
        111,  // 'o'
        10,   // Newline
        87,   // 'W'
        111,  // 'o'
        114,  // 'r'
        108,  // 'l'
        100,  // 'd'
        33,   // '!'
    ];

    println!("📄 Original data: {:?}", String::from_utf8_lossy(&original_data));
    println!("📊 Original size: {} bytes", original_data.len());

    // Step 1: Convert to printable ASCII
    println!("\n🔄 Step 1: Converting to printable ASCII...");
    let (ascii_data, ascii_stats) = convert_to_printable_ascii(&original_data)?;
    println!("   Converted data: {:?}", String::from_utf8_lossy(&ascii_data));
    println!("   ASCII conversion stats: {} bytes converted ({:.1}%)", 
        ascii_stats.converted_bytes, 
        (ascii_stats.converted_bytes as f64 / ascii_stats.total_bytes as f64) * 100.0);

    // Step 2: Convert to binary string
    println!("\n🔄 Step 2: Converting to binary string...");
    let binary_string: String = ascii_data.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();
    println!("   Binary string length: {} bits", binary_string.len());

    // Step 3: Create compression mapping
    println!("\n🔄 Step 3: Creating compression mapping...");
    let compression_mapping = create_chunk_mapping(binary_string.as_bytes(), 8)?;
    println!("   Chunk size: {}", compression_mapping.chunk_size);
    println!("   Unique chunks: {}", compression_mapping.byte_to_chunk.len());
    println!("   Compression ratio: {:.2}", compression_mapping.compression_ratio);

    // Step 4: Compress the data
    println!("\n🔄 Step 4: Compressing data...");
    let compressed_data = compress_data(binary_string.as_bytes(), &compression_mapping)?;
    println!("   Compressed size: {} bytes", compressed_data.len());
    println!("   Size reduction: {:.1}%", 
        (1.0 - compressed_data.len() as f64 / binary_string.len() as f64) * 100.0);

    // Step 5: Create comprehensive mapping
    println!("\n🔄 Step 5: Creating comprehensive mapping...");
    let complete_mapping = create_complete_mapping(
        compression_mapping,
        &ascii_stats,
        "demo.txt",
        "demo_upload_id",
        &original_data,
    )?;

    println!("   Mapping version: {}", complete_mapping.version);
    println!("   File info: {} bytes, extension: {}", 
        complete_mapping.file_info.original_size,
        complete_mapping.file_info.file_extension);
    println!("   Reversal steps: {}", complete_mapping.reversal_instructions.total_steps);

    // Show reversal steps
    for step in &complete_mapping.reversal_instructions.steps {
        println!("     {}. {}: {}", step.step_number, step.operation, step.description);
    }

    // Step 6: Save mapping to file
    println!("\n🔄 Step 6: Saving mapping to file...");
    let mapping_file = "demo.mapping.json";
    save_mapping(&complete_mapping, mapping_file)?;
    println!("   Mapping saved to: {}", mapping_file);

    // Step 7: Load mapping and reverse
    println!("\n🔄 Step 7: Loading mapping and reversing...");
    let loaded_mapping = load_mapping(mapping_file)?;
    let reversed_data = reverse_compression(&compressed_data, &loaded_mapping)?;

    // Step 8: Verify reversal
    println!("\n🔄 Step 8: Verifying reversal...");
    println!("   Original data: {:?}", String::from_utf8_lossy(&original_data));
    println!("   Reversed data: {:?}", String::from_utf8_lossy(&reversed_data));
    println!("   Original size: {} bytes", original_data.len());
    println!("   Reversed size: {} bytes", reversed_data.len());
    
    if original_data == reversed_data {
        println!("✅ Perfect reversal achieved!");
    } else {
        println!("❌ Reversal failed - data mismatch");
    }

    // Show mapping file structure
    println!("\n📋 Mapping file structure:");
    println!("   - Version: {}", complete_mapping.version);
    println!("   - File metadata: size, extension, hash");
    println!("   - Compression mapping: chunk size, byte-to-chunk mappings");
    println!("   - ASCII conversion: conversion maps and statistics");
    println!("   - Reversal instructions: step-by-step reversal guide");
    println!("   - Metadata: creation time, tool version, etc.");

    println!("\n🎯 Key benefits of this mapping structure:");
    println!("   - Complete lossless reversal information");
    println!("   - Structured JSON format for easy parsing");
    println!("   - Step-by-step reversal instructions");
    println!("   - ASCII conversion tracking");
    println!("   - File integrity verification (hash)");
    println!("   - Comprehensive metadata for debugging");

    Ok(())
} 