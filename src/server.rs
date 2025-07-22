use axum::{
    extract::{Multipart, State},
    http::{StatusCode, HeaderMap},
    response::{Json, IntoResponse},
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;
use tracing::{info, error, warn};
use sha2::{Sha256, Digest};
use anyhow::Result;

use stark_squeeze::{
    ascii_converter::convert_to_printable_ascii,
    compression::compress_file,
    mapping::{create_minimal_mapping, save_minimal_mapping},
    starknet_client::upload_data,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionRequest {
    pub file_name: String,
    pub file_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionResponse {
    pub success: bool,
    pub file_url: Option<String>,
    pub compression_ratio: Option<f64>,
    pub original_size: Option<usize>,
    pub compressed_size: Option<usize>,
    pub error: Option<String>,
    pub mapping_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    pub status: String,
    pub dictionary_loaded: bool,
    pub dictionary_size: Option<usize>,
    pub uptime: String,
    pub total_files_processed: usize,
}

#[derive(Debug)]
pub struct AppState {
    pub dictionary_loaded: bool,
    pub dictionary_path: Option<String>,
    pub total_files_processed: usize,
    pub start_time: std::time::Instant,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            dictionary_loaded: false,
            dictionary_path: None,
            total_files_processed: 0,
            start_time: std::time::Instant::now(),
        }
    }
}

pub type SharedState = Arc<Mutex<AppState>>;

/// Initialize the server and generate dictionary
async fn initialize_server() -> Result<SharedState> {
    info!("🚀 Initializing Stark Squeeze Server...");
    
    let state = Arc::new(Mutex::new(AppState::new()));
    
    // Generate dictionary if it doesn't exist
    let dictionary_path = "ascii_combinations.json";
    if !std::path::Path::new(dictionary_path).exists() {
        info!("📚 Dictionary not found. Generating ASCII combinations dictionary...");
        
        // Run the dictionary generation
        match generate_dictionary().await {
            Ok(_) => {
                info!("✅ Dictionary generated successfully");
                let mut state_guard = state.lock().await;
                state_guard.dictionary_loaded = true;
                state_guard.dictionary_path = Some(dictionary_path.to_string());
            }
            Err(e) => {
                error!("❌ Failed to generate dictionary: {}", e);
                return Err(e);
            }
        }
    } else {
        info!("✅ Dictionary found at {}", dictionary_path);
        let mut state_guard = state.lock().await;
        state_guard.dictionary_loaded = true;
        state_guard.dictionary_path = Some(dictionary_path.to_string());
    }
    
    info!("🎉 Server initialization complete!");
    Ok(state)
}

/// Generate the ASCII combinations dictionary
async fn generate_dictionary() -> Result<()> {
    info!("🔤 Generating ASCII combinations dictionary...");
    
    // This would call your existing dictionary generation logic
    // For now, we'll create a simple placeholder
    let dictionary_data = serde_json::json!({
        "metadata": {
            "length": 5,
            "total_combinations": 1000,
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "compression_ratio": "80% (5 chars → 1 byte)"
        },
        "combinations": {}
    });
    
    fs::write("ascii_combinations.json", serde_json::to_string_pretty(&dictionary_data)?)?;
    
    Ok(())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "stark-squeeze",
        "version": "1.0.0"
    }))
}

/// Server status endpoint
async fn server_status(State(state): State<SharedState>) -> impl IntoResponse {
    let state_guard = state.lock().await;
    let uptime = state_guard.start_time.elapsed();
    
    let status = ServerStatus {
        status: "running".to_string(),
        dictionary_loaded: state_guard.dictionary_loaded,
        dictionary_size: state_guard.dictionary_path.as_ref().and_then(|path| {
            fs::metadata(path).ok().map(|metadata| metadata.len() as usize)
        }),
        uptime: format!("{:?}", uptime),
        total_files_processed: state_guard.total_files_processed,
    };
    
    Json(status)
}

/// Compress file endpoint
async fn compress_file_endpoint(
    State(state): State<SharedState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<CompressionResponse>)> {
    let mut file_data = Vec::new();
    let mut file_name = String::new();
    
    // Extract file from multipart form data
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            if let Some(filename) = field.file_name() {
                file_name = filename.to_string();
            }
            
            if let Ok(data) = field.bytes().await {
                file_data = data.to_vec();
            }
        }
    }
    
    if file_data.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(CompressionResponse {
                success: false,
                file_url: None,
                compression_ratio: None,
                original_size: None,
                compressed_size: None,
                error: Some("No file data provided".to_string()),
                mapping_file: None,
            })
        ));
    }
    
    info!("📁 Processing file: {} ({} bytes)", file_name, file_data.len());
    
    // Process the file through your compression pipeline
    match process_file_compression(&file_name, &file_data).await {
        Ok(result) => {
            let mut state_guard = state.lock().await;
            state_guard.total_files_processed += 1;
            
            Ok(Json(result))
        }
        Err(e) => {
            error!("❌ Compression failed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CompressionResponse {
                    success: false,
                    file_url: None,
                    compression_ratio: None,
                    original_size: None,
                    compressed_size: None,
                    error: Some(e.to_string()),
                    mapping_file: None,
                })
            ))
        }
    }
}

/// Process file compression using your existing pipeline
async fn process_file_compression(
    file_name: &str,
    file_data: &[u8],
) -> Result<CompressionResponse> {
    let original_size = file_data.len();
    
    // Step 1: Convert to printable ASCII
    let (ascii_buffer, ascii_stats) = convert_to_printable_ascii(file_data)
        .map_err(|e| anyhow::anyhow!("ASCII conversion failed: {}", e))?;
    
    // Step 2: Convert ASCII buffer to binary string
    let binary_string: String = ascii_buffer.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();
    
    // Step 3: Compress the data
    let bytes = binary_string.as_bytes();
    let result = compress_file(bytes)
        .map_err(|e| anyhow::anyhow!("Compression failed: {}", e))?;
    let encoded_data = result.compressed_data;
    let mapping = result.mapping;
    
    // Step 4: Calculate compression metrics
    let compressed_size = encoded_data.len();
    let compression_ratio = ((compressed_size as f64 / original_size as f64) * 100.0) as f64;
    
    // Step 5: Generate hash for file identification
    let mut hasher = Sha256::new();
    let encoded_data_bytes: Vec<u8> = encoded_data.iter().flat_map(|x| x.to_be_bytes()).collect();
    hasher.update(&encoded_data_bytes);
    let hash = hasher.finalize();
    let short_hash = hex::encode(&hash[..8]);
    
    // Step 6: Create minimal mapping for file reconstruction
    let minimal_mapping = create_minimal_mapping(
        mapping,
        &ascii_stats,
        &encoded_data_bytes,
    );
    
    // Step 7: Save mapping file
    let mapping_file_name = format!("{}.map", short_hash);
    save_minimal_mapping(&minimal_mapping, &mapping_file_name)
        .map_err(|e| anyhow::anyhow!("Failed to save mapping: {}", e))?;
    
    // Step 8: Upload to Starknet (optional - you can disable this for testing)
    let file_url = if std::env::var("ENABLE_STARKNET_UPLOAD").unwrap_or_default() == "true" {
        match upload_to_starknet(&short_hash, file_name, original_size, compressed_size).await {
            Ok(url) => Some(url),
            Err(e) => {
                warn!("⚠️ Starknet upload failed: {}", e);
                None
            }
        }
    } else {
        Some(format!("http://localhost:3000/files/{}", short_hash))
    };
    
    info!("✅ File compressed successfully: {} -> {} bytes ({:.1}% compression)", 
          original_size, compressed_size, 100.0 - compression_ratio);
    
    Ok(CompressionResponse {
        success: true,
        file_url,
        compression_ratio: Some(100.0 - compression_ratio),
        original_size: Some(original_size),
        compressed_size: Some(compressed_size),
        error: None,
        mapping_file: Some(mapping_file_name),
    })
}

/// Upload compressed file metadata to Starknet
async fn upload_to_starknet(
    uri: &str,
    file_format: &str,
    original_size: usize,
    compressed_size: usize,
) -> Result<String> {
    // Prepare data for upload
    let compressed_by = if compressed_size < original_size {
        ((original_size - compressed_size) * 100 / original_size) as u8
    } else {
        0
    };
    
    // Create minimal arrays for on-chain storage
    let chunk_mappings = vec![starknet::core::types::FieldElement::from(0u32)];
    let chunk_values = vec![0u8];
    let byte_mappings = vec![0u8];
    let byte_values = vec![starknet::core::types::FieldElement::from(0u32)];
    let reconstruction_steps = vec![starknet::core::types::FieldElement::from(0u32)];
    let metadata = vec![starknet::core::types::FieldElement::from(0u32)];
    
    upload_data(
        uri,
        file_format,
        compressed_by,
        original_size,
        compressed_size,
        8, // chunk_size
        chunk_mappings,
        chunk_values,
        byte_mappings,
        byte_values,
        reconstruction_steps,
        metadata,
    ).await.map_err(|e| anyhow::anyhow!("Starknet upload failed: {}", e))?;
    
    Ok(format!("starknet://{}", uri))
}

/// Download compressed file endpoint
async fn download_file(axum::extract::Path(file_id): axum::extract::Path<String>) -> impl IntoResponse {
    let mapping_file = format!("{}.map", file_id);
    
    if !std::path::Path::new(&mapping_file).exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }
    
    // Here you would implement file reconstruction logic
    // For now, return the mapping file
    match fs::read(&mapping_file) {
        Ok(data) => {
            let headers = HeaderMap::from_iter(vec![
                ("content-type".parse().unwrap(), "application/json".parse().unwrap()),
                ("content-disposition".parse().unwrap(), format!("attachment; filename=\"{}\"", mapping_file).parse().unwrap()),
            ]);
            (StatusCode::OK, headers, data).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file").into_response()
    }
}

/// Create the router with all endpoints
fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/status", get(server_status))
        .route("/compress", post(compress_file_endpoint))
        .route("/files/:file_id", get(download_file))
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Stark Squeeze Server...");
    
    // Initialize server state and generate dictionary
    let state = initialize_server().await?;
    
    // Create router
    let app = create_router(state);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("🌐 Server listening on http://0.0.0.0:3000");
    info!("📚 Health check: http://0.0.0.0:3000/health");
    info!("📊 Status: http://0.0.0.0:3000/status");
    info!("📁 Compress files: POST http://0.0.0.0:3000/compress");
    
    axum::serve(listener, app).await?;
    
    Ok(())
} 