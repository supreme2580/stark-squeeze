#[cfg(test)]
mod tests {
    use stark_squeeze::*;
    use tokio;
    
    #[tokio::test]
    async fn test_encoding_one() {
        // Test with "00010" (.) + "00010" (.)
        let binary = "0001000010";
        let result = encoding_one(binary).await.unwrap();
        assert_eq!(result, "..");

        // Test with "00111" (...) + "01110" (...)
        let binary2 = "0011101110";
        let result2 = encoding_one(binary2).await.unwrap();
        assert_eq!(result2, "......"); // "..." + "..."

        // Test with a longer binary string
        // "10101" (". . .") + "11111" (".....")
        let binary3 = "1010111111";
        let result3 = encoding_one(binary3).await.unwrap();
        assert_eq!(result3, ". . ......");

        // Test with empty string
        let result4 = encoding_one("").await.unwrap();
        assert_eq!(result4, "");

        // Test with invalid characters
        let invalid_binary = "001201";
        let result5 = encoding_one(invalid_binary).await;
        assert!(result5.is_err());

        // Test with binary that maps to empty string in FIRST_DICT
        let binary6 = "00000";
        let result6 = encoding_one(binary6).await.unwrap();
        assert_eq!(result6, "");

        // Test with a mix of emptry string mappings and non-empty
        // "00000" ("") + "00001" (".")
        let binary7 = "0000000001";
        let result7 = encoding_one(binary7).await.unwrap();
        assert_eq!(result7, ".");
    }

    #[tokio::test]
    async fn test_encoding_two() {
        // Test with various patterns
        assert_eq!(encoding_two(".").await.unwrap(), "*");
        assert_eq!(encoding_two("..").await.unwrap(), "%");
        assert_eq!(encoding_two("...").await.unwrap(), "$");
        assert_eq!(encoding_two("....").await.unwrap(), "#");
        assert_eq!(encoding_two(".....").await.unwrap(), "!");
        assert_eq!(encoding_two(". .").await.unwrap(), "&");

        // Test with combinations
        assert_eq!(encoding_two(".. ...").await.unwrap(), "%$");
        assert_eq!(encoding_two(". . .").await.unwrap(), "&*");
        assert_eq!(encoding_two("...........").await.unwrap(), "!!*");

        // Tests with explicit spaces
        assert_eq!(encoding_two("... ...").await.unwrap(), "$$");
        assert_eq!(encoding_two(". . . . .").await.unwrap(), "&&*");
        assert_eq!(encoding_two(".. .. ..").await.unwrap(), "%%%");

        // Test with a mix of patterns and spaces
        let mixed = "...... . .....";
        assert_eq!(encoding_two(mixed).await.unwrap(), "!&!");

        // Test with leading and trailing spaces
        assert_eq!(encoding_two(" .").await.unwrap(), "*");
        assert_eq!(encoding_two(". ").await.unwrap(), "*");
        assert_eq!(encoding_two(" . ").await.unwrap(), "*");

        // Test with multiple consecutive spaces
        assert_eq!(encoding_two(".  .").await.unwrap(), "**");
        assert_eq!(encoding_two(".   .").await.unwrap(), "**");

        // Test with empty string
        assert_eq!(encoding_two("").await.unwrap(), "");

        // Test error case with invalid pattern
        assert!(encoding_two("...x").await.is_err());
        assert!(encoding_two("abc").await.is_err());
    }

    #[test]
    fn test_file_to_ascii_ascii_file() {
        use std::fs::File;
        use std::io::Write;
        let test_path = "test_ascii.txt";
        let content = "Hello, ASCII World! 12345";
        let mut file = File::create(test_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let ascii = crate::file_to_ascii(test_path).unwrap();
        assert_eq!(ascii, content);

        std::fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_file_to_ascii_non_ascii_file() {
        use std::fs::File;
        use std::io::Write;
        let test_path = "test_non_ascii.txt";
        let content = b"Hello\xFFWorld";
        let mut file = File::create(test_path).unwrap();
        file.write_all(content).unwrap();

        let result = crate::file_to_ascii(test_path);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Non-ASCII byte"));

        std::fs::remove_file(test_path).unwrap();
    }
}