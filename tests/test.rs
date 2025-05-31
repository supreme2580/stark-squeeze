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
}