use stark_squeeze::cli;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use std::io::Write;

#[tokio::test]
async fn test_upload_file_not_exists() {
    // Test with a non-existent file
    let non_existent_path = PathBuf::from("non_existent_file.txt");
    
    // This should return early due to validation, we can't test the return value
    // But we can verify it doesn't panic
    cli::upload_data_cli(Some(non_existent_path)).await;
}

#[tokio::test]
async fn test_upload_empty_file() {
    // Create a temporary empty file
    let tmp_file = NamedTempFile::new().unwrap();
    let file_path = tmp_file.path().to_path_buf();
    
    // This should return early due to validation (empty file)
    cli::upload_data_cli(Some(file_path)).await;
}

#[tokio::test]
async fn test_upload_valid_file() {
    // Create a temporary file with some content
    let mut tmp_file = NamedTempFile::new().unwrap();
    writeln!(tmp_file, "Test content for validation").unwrap();
    let file_path = tmp_file.path().to_path_buf();
    
    cli::upload_data_cli(Some(file_path)).await;
}

#[tokio::test]
async fn test_retrieve_invalid_id() {
    // Test with an invalid ID
    let invalid_id = Some("invalid".to_string());
    
    // This should return early due to validation
    cli::retrieve_data_cli(invalid_id).await;
}

#[tokio::test]
async fn test_retrieve_malformed_hex() {
    // Test with a malformed hex string (non-hex characters)
    let malformed_hex = Some("0xZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ".to_string());
    
    // This should return early due to validation
    cli::retrieve_data_cli(malformed_hex).await;
}

#[tokio::test]
async fn test_retrieve_valid_format_id() {
    // Test with a correctly formatted ID (may not exist on network, but format is valid)
    let valid_format_id = Some("0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string());
    
    cli::retrieve_data_cli(valid_format_id).await;
}
