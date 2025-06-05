// Example demonstrating ASCII conversion functionality
use stark_squeeze::ascii_converter::{convert_to_printable_ascii, validate_printable_ascii};
use std::fs::{File, write};
use std::io::Write;

fn main() {
    println!("🚀 StarkSqueeze ASCII Conversion Example\n");

    println!("📝 Creating test file with mixed characters...");
    let test_data = vec![
        b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
        0,   // NULL
        9,   // TAB
        10,  // LF (newline)
        13,  // CR (carriage return)
        27,  // ESC
        // Extended ASCII
        128, 200, 255,
        // More printable
        b' ', b'T', b'e', b's', b't', b'.',
    ];

    let mut file = File::create("test_mixed_chars.bin").expect("Failed to create test file");
    file.write_all(&test_data).expect("Failed to write test data");
    println!("✅ Test file created: test_mixed_chars.bin\n");

    println!("🔄 Converting to printable ASCII...");
    let (converted, stats) = convert_to_printable_ascii(&test_data).expect("Conversion failed");

    println!("\n📊 Conversion Results:");
    println!("Original bytes: {:?}", test_data.len());
    println!("Converted bytes: {:?}", stats.converted_bytes);

    println!("\n🔍 Character by character comparison:");
    for (i, (orig, conv)) in test_data.iter().zip(converted.iter()).enumerate() {
        let orig_repr = match *orig {
            32..=126 => format!("'{}'", *orig as char),
            _ => format!("0x{:02X}", orig),
        };
        let conv_repr = format!("'{}'", *conv as char);

        if orig != conv {
            println!("  [{}] {} → {} (converted)", i, orig_repr, conv_repr);
        } else {
            println!("  [{}] {} (unchanged)", i, orig_repr);
        }
    }

    println!("\n✔️  Validating converted data...");
    match validate_printable_ascii(&converted) {
        Ok(_) => println!("✅ All characters are now printable ASCII!"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    write("test_mixed_chars_ascii.txt", &converted).expect("Failed to write converted file");
    println!("\n📁 Converted file saved as: test_mixed_chars_ascii.txt");

    println!("\n📝 Testing with already-ASCII file...");
    let ascii_only = b"This is already ASCII text!";
    let (_converted_ascii, stats_ascii) = convert_to_printable_ascii(ascii_only).expect("Conversion failed");

    if stats_ascii.converted_bytes == 0 {
        println!("✅ No conversion needed - file was already pure ASCII!");
    }

    println!("\n🎉 Example complete!");
}
