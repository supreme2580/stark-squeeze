use std::env;
use reqwest::multipart;
use serde_json::Value;
use dotenvy::dotenv;

/// Error type for IPFS operations
#[derive(Debug)]
pub enum IpfsError {
    NetworkError(String),
    AuthError(String),
    ApiError(String),
    ConfigError(String),
}

impl std::fmt::Display for IpfsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpfsError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            IpfsError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            IpfsError::ApiError(msg) => write!(f, "API error: {}", msg),
            IpfsError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for IpfsError {}

/// Pins a file to IPFS using Pinata service
pub async fn pin_file_to_ipfs(
    file_data: &[u8],
    filename: &str,
) -> Result<String, IpfsError> {
    dotenv().ok();
    
    // Get Pinata credentials from environment
    let jwt_token = env::var("PINATA_JWT")
        .map_err(|_| IpfsError::ConfigError("PINATA_JWT not found in environment".to_string()))?;
    
    // Create HTTP client
    let client = reqwest::Client::new();
    
    // Prepare multipart form data
    let form = multipart::Form::new()
        .part(
            "file",
            multipart::Part::bytes(file_data.to_vec())
                .file_name(filename.to_string())
                .mime_str("application/octet-stream")
                .map_err(|e| IpfsError::ApiError(format!("Failed to create form part: {}", e)))?,
        );
    
    // Send request to Pinata
    let response = client
        .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
        .bearer_auth(&jwt_token)
        .multipart(form)
        .send()
        .await
        .map_err(|e| IpfsError::NetworkError(format!("Failed to send request: {}", e)))?;
    
    // Check response status
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(IpfsError::ApiError(format!("Pinata API error: {}", error_text)));
    }
    
    // Parse response JSON
    let response_json: Value = response
        .json()
        .await
        .map_err(|e| IpfsError::ApiError(format!("Failed to parse response: {}", e)))?;
    
    // Extract IPFS hash (CID)
    let ipfs_hash = response_json["IpfsHash"]
        .as_str()
        .ok_or_else(|| IpfsError::ApiError("No IpfsHash in response".to_string()))?;
    
    Ok(ipfs_hash.to_string())
}