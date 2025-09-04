use anyhow::Result;
use axum::{
    extract::{Multipart, Path, Query, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool};
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn};
use uuid::Uuid;

use stark_squeeze::{
    ascii_converter::convert_to_printable_ascii, compression::compress_file,
    ipfs_client::pin_file_to_ipfs, starknet_client::upload_data,
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
    pub ipfs_cid: Option<String>,
    pub compression_ratio: Option<f64>,
    pub original_size: Option<usize>,
    pub compressed_size: Option<usize>,
    pub error: Option<String>,
    pub mapping_file: Option<String>,
    pub upload_timestamp: Option<i64>,
    pub file_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    pub status: String,
    pub dictionary_loaded: bool,
    pub dictionary_size: Option<usize>,
    pub uptime: String,
    pub total_files_processed: usize,
}

#[derive(Deserialize)]
pub struct FileQuery {
    pub owner: Option<String>,
    pub visibility: Option<i32>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct FileUploaded {
    pub uri: String,
    pub owner: String,
    pub visibility: i32,
    pub block_number: i64,
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct FileUpdated {
    pub uri: String,
    pub owner: String,
    pub block_number: i64,
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct FileDeleted {
    pub uri: String,
    pub owner: String,
    pub block_number: i64,
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct FileShared {
    pub uri: String,
    pub owner: String,
    pub shared_with: String,
    pub block_number: i64,
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CompressionRes {
    pub uri: String,
    pub file_format: String,
    pub compressed_by: i32,
    pub original_size: i64,
    pub final_size: i64,
    pub chunk_size: i64,
    pub chunk_mappings: serde_json::Value,
    pub chunk_values: serde_json::Value,
    pub byte_mappings: serde_json::Value,
    pub byte_values: serde_json::Value,
    pub reconstruction_steps: serde_json::Value,
    pub metadata: serde_json::Value,
    pub block_number: i64,
    pub transaction_hash: String,
}
#[derive(Debug)]
pub struct AppState {
    pub db: PgPool,
    pub dictionary_loaded: bool,
    pub dictionary_path: Option<String>,
    pub total_files_processed: usize,
    pub start_time: std::time::Instant,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
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

    let db_pool = initialize_database().await?;
    let state = Arc::new(Mutex::new(AppState::new(db_pool)));

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

/// Initialize the database
async fn initialize_database() -> Result<PgPool> {
    info!("💾 Initializing database...");

    let database_url = std::env::var("POSTGRES_CONNECTION_STRING")
        .expect("DATABASE_URL must be set, e.g. postgres://user:pass@localhost/db");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("✅ Connection to the database is successful!");
    Ok(pool)
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

    fs::write(
        "ascii_combinations.json",
        serde_json::to_string_pretty(&dictionary_data)?,
    )?;

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
            fs::metadata(path)
                .ok()
                .map(|metadata| metadata.len() as usize)
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
                ipfs_cid: None,
                compression_ratio: None,
                original_size: None,
                compressed_size: None,
                error: Some("No file data provided".to_string()),
                mapping_file: None,
                upload_timestamp: None,
                file_type: None,
            }),
        ));
    }

    info!(
        "📁 Processing file: {} ({} bytes)",
        file_name,
        file_data.len()
    );

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
                    ipfs_cid: None,
                    compression_ratio: None,
                    original_size: None,
                    compressed_size: None,
                    error: Some(e.to_string()),
                    mapping_file: None,
                    upload_timestamp: None,
                    file_type: None,
                }),
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
    let upload_timestamp = chrono::Utc::now().timestamp();

    // Get file extension for type detection
    let file_type = std::path::Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Step 1: Convert to printable ASCII (keeping this for now)
    let (ascii_buffer, _ascii_stats) = convert_to_printable_ascii(file_data)
        .map_err(|e| anyhow::anyhow!("ASCII conversion failed: {}", e))?;

    // Step 2: Convert ASCII buffer to binary string
    let binary_string: String = ascii_buffer
        .iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect();

    // Step 3: Mock compression (keeping original data)
    let bytes = binary_string.as_bytes();
    let encoded_data =
        compress_file(bytes).map_err(|e| anyhow::anyhow!("Compression failed: {}", e))?;

    // Step 4: Calculate compression metrics (mock - no actual compression)
    let compressed_size = encoded_data.len();
    let compression_ratio = ((compressed_size as f64 / original_size as f64) * 100.0) as f64;

    // Step 5: Generate hash for file identification
    let mut hasher = Sha256::new();
    let encoded_data_bytes: Vec<u8> = encoded_data.iter().flat_map(|x| x.to_be_bytes()).collect();
    hasher.update(&encoded_data_bytes);
    let hash = hasher.finalize();
    let short_hash = hex::encode(&hash[..8]);

    // Step 6: Upload original file to IPFS via Pinata
    let ipfs_cid = match pin_file_to_ipfs(file_data, file_name).await {
        Ok(cid) => {
            info!("✅ File pinned to IPFS: {}", cid);
            Some(cid)
        }
        Err(e) => {
            warn!("⚠️ IPFS upload failed: {}", e);
            None
        }
    };

    // Step 7: Generate file URLs
    let file_url = if let Some(ref cid) = ipfs_cid {
        Some(format!("https://gateway.pinata.cloud/ipfs/{}", cid))
    } else {
        // Fallback to local URL if IPFS upload failed
        Some(format!("http://localhost:8080/files/{}", short_hash))
    };

    // Step 8: Upload to Starknet (optional - you can disable this for testing)
    let _starknet_url = if std::env::var("ENABLE_STARKNET_UPLOAD").unwrap_or_default() == "true" {
        match upload_to_starknet(&short_hash, file_name, original_size, compressed_size).await {
            Ok(url) => Some(url),
            Err(e) => {
                warn!("⚠️ Starknet upload failed: {}", e);
                None
            }
        }
    } else {
        None
    };

    info!(
        "✅ File processed successfully: {} -> {} bytes ({:.1}% compression)",
        original_size,
        compressed_size,
        100.0 - compression_ratio
    );

    Ok(CompressionResponse {
        success: true,
        file_url,
        ipfs_cid,
        compression_ratio: Some(100.0 - compression_ratio),
        original_size: Some(original_size),
        compressed_size: Some(compressed_size),
        error: None,
        mapping_file: None,
        upload_timestamp: Some(upload_timestamp),
        file_type: Some(file_type),
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
    )
    .await
    .map_err(|e| anyhow::anyhow!("Starknet upload failed: {}", e))?;

    Ok(format!("starknet://{}", uri))
}

/// Download compressed file endpoint
async fn download_file(
    axum::extract::Path(file_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let mapping_file = format!("{}.map", file_id);

    if !std::path::Path::new(&mapping_file).exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    // Here you would implement file reconstruction logic
    // For now, return the mapping file
    match fs::read(&mapping_file) {
        Ok(data) => {
            let headers = HeaderMap::from_iter(vec![
                (
                    "content-type".parse().unwrap(),
                    "application/json".parse().unwrap(),
                ),
                (
                    "content-disposition".parse().unwrap(),
                    format!("attachment; filename=\"{}\"", mapping_file)
                        .parse()
                        .unwrap(),
                ),
            ]);
            (StatusCode::OK, headers, data).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file").into_response(),
    }
}

/// List/Search files (ignores deleted)
async fn list_files(
    State(state): State<SharedState>,
    Query(query): Query<FileQuery>,
) -> Json<Vec<FileUploaded>> {
    let state_guard = state.lock().await;
    let db = &state_guard.db;

    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let rows = sqlx::query_as::<_, FileUploaded>(
        r#"
        SELECT id, uri, owner, visibility, block_number, transaction_hash, created_at
        FROM file_uploaded
        WHERE ($1::text IS NULL OR owner = $1)
          AND ($2::int IS NULL OR visibility = $2)
          AND ($3::text IS NULL OR uri ILIKE '%' || $3 || '%')
          AND uri NOT IN (SELECT uri FROM file_deleted)
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(query.owner)
    .bind(query.visibility)
    .bind(query.search)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
    .unwrap();

    Json(rows)
}

// File history handler
async fn get_file_history(State(state): State<SharedState>, Path(id): Path<String>) -> Response {
    let id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid UUID format" })),
            )
                .into_response();
        }
    };
    let state_guard = state.lock().await;
    let db = &state_guard.db;

    let file = sqlx::query_as::<_, FileUploaded>("SELECT * FROM file_uploaded WHERE id = $1")
        .bind(id)
        .fetch_optional(db)
        .await
        .unwrap();

    if let Some(f) = file {
        let updates: Vec<FileUpdated> =
            sqlx::query_as("SELECT * FROM file_updated WHERE uri = $1 ORDER BY created_at ASC")
                .bind(f.uri.clone())
                .fetch_all(db)
                .await
                .unwrap();

        let deletions: Vec<FileDeleted> =
            sqlx::query_as("SELECT * FROM file_deleted WHERE uri = $1")
                .bind(f.uri.clone())
                .fetch_all(db)
                .await
                .unwrap();

        let shares: Vec<FileShared> = sqlx::query_as("SELECT * FROM file_shared WHERE uri = $1")
            .bind(f.uri.clone())
            .fetch_all(db)
            .await
            .unwrap();

        return (
            StatusCode::OK,
            Json(json!({
                "file": f,
                "updates": updates,
                "deletions": deletions,
                "shares": shares
            })),
        )
            .into_response();
    }

    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "File not found" })),
    )
        .into_response()
}

/// Get file + compression metadata handler
async fn get_file_with_metadata(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Response {
    let id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid UUID format" })),
            )
                .into_response();
        }
    };
    let state_guard = state.lock().await;
    let db = &state_guard.db;

    let file: Option<FileUploaded> = sqlx::query_as(
        r#"
        SELECT * FROM file_uploaded 
        WHERE id = $1
          AND uri NOT IN (SELECT uri FROM file_deleted)
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await
    .unwrap();

    if let Some(f) = file {
        let mapping: Option<CompressionRes> =
            sqlx::query_as("SELECT * FROM compression_mappings WHERE uri = $1")
                .bind(&f.uri)
                .fetch_optional(db)
                .await
                .unwrap();

        return (
            StatusCode::OK,
            Json(json!({
                "file": f,
                "metadata": mapping
            })),
        )
            .into_response();
    }

    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "File not found or deleted" })),
    )
        .into_response()
}

/// Get all files shared with a user
pub async fn get_shared_files(
    State(state): State<SharedState>,
    Path(user): Path<String>,
) -> Json<Vec<FileShared>> {
    let state_guard = state.lock().await;
    let db = &state_guard.db;

    let rows = sqlx::query_as::<_, FileShared>(
        "SELECT * FROM file_shared WHERE shared_with = $1 ORDER BY created_at DESC",
    )
    .bind(user)
    .fetch_all(db)
    .await
    .unwrap();

    Json(rows)
}

/// Create the router with all endpoints
fn create_router(state: SharedState) -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_credentials(false);

    Router::new()
        .route("/health", get(health_check))
        .route("/status", get(server_status))
        .route("/compress", post(compress_file_endpoint))
        .route("/files/:file_id", get(download_file))
        .route("/files", get(list_files))
        .route("/files/:id/metadata", get(get_file_with_metadata)) // Changed route path
        .route("/files/:id/history", get(get_file_history))
        .route("/files/shared/:user", get(get_shared_files))
        .layer(cors)
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Stark Squeeze Server...");

    // Initialize server state and generate dictionary
    let state = initialize_server().await?;

    // Create router
    let app = create_router(state);

    // Get port from environment variable (Render provides PORT, but we use SERVER_PORT)
    let port = std::env::var("PORT")
        .or_else(|_| std::env::var("SERVER_PORT"))
        .unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("🌐 Server listening on http://{}", addr);
    info!("📚 Health check: http://{}/health", addr);
    info!("📊 Status: http://{}/status", addr);
    info!("📁 Compress files: POST http://{}/compress", addr);
    info!("📁 All listed files: GET http://{}/files", addr);
    info!(
        " 📁Get file meta data : GET http://{}/files/:id/metadata",
        addr
    );
    info!(
        "📁 Get file history  : GET http://{}/files/:id/history",
        addr
    );
    info!(
        "📁 Get all files shared with a user : GET http://{}/files/shared/:user",
        addr
    );

    axum::serve(listener, app).await?;

    Ok(())
}
