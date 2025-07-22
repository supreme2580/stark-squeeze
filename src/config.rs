use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub description: String,
    pub compression: CompressionConfig,
    pub dictionary: DictionaryConfig,
    pub file_processing: FileProcessingConfig,
    pub upload: UploadConfig,
    pub server: ServerConfig,
    pub cli: CliConfig,
    pub mapping: MappingConfig,
    pub storage: StorageConfig,
    pub debug: DebugConfig,
    pub performance: PerformanceConfig,
    pub validation: ValidationConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub target_compression_ratio: f64,
    pub current_compression_ratio: f64,
    pub compression_method: String,
    pub chunk_size_range: ChunkSizeRange,
    pub optimal_compression_threshold: f64,
    pub max_unique_chunks: u8,
    pub compression_ratios: HashMap<String, CompressionRatio>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkSizeRange {
    pub min: usize,
    pub max: usize,
    pub default: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionRatio {
    pub input_chars: usize,
    pub output_bytes: usize,
    pub ratio: f64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryConfig {
    pub ascii_combinations: AsciiCombinationsConfig,
    pub ultra_compressed: UltraCompressedConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsciiCombinationsConfig {
    pub default_length: usize,
    pub default_start_index: u64,
    pub default_count: usize,
    pub total_possible_combinations: u64,
    pub ascii_chars: usize,
    pub output_file: String,
    pub generation: GenerationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub chunk_size: usize,
    pub json_chunk_size: usize,
    pub flush_interval: usize,
    pub combinations_per_second_estimate: u64,
    pub fast_system_rate: u64,
    pub medium_system_rate: u64,
    pub slow_system_rate: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UltraCompressedConfig {
    pub length: usize,
    pub start_index: u64,
    pub compression_ratio: f64,
    pub encoding: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileProcessingConfig {
    pub ascii_conversion: AsciiConversionConfig,
    pub binary_string_conversion: BinaryStringConversionConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsciiConversionConfig {
    pub chunk_size: usize,
    pub printable_range: PrintableRange,
    pub conversion_map: ConversionMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintableRange {
    pub min: u8,
    pub max: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionMap {
    pub control_chars: String,
    pub extended_ascii: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryStringConversionConfig {
    pub bits_per_byte: usize,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadConfig {
    pub hash: HashConfig,
    pub starknet: StarknetConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashConfig {
    pub algorithm: String,
    pub short_hash_length: usize,
    pub upload_id_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarknetConfig {
    pub chunk_size: usize,
    pub field_element_size: usize,
    pub calldata_optimization: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub endpoints: EndpointsConfig,
    pub dictionary: DictionaryServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointsConfig {
    pub health: String,
    pub status: String,
    pub compress: String,
    pub files: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryServerConfig {
    pub auto_generate: bool,
    pub path: String,
    pub fallback_metadata: FallbackMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FallbackMetadata {
    pub length: usize,
    pub total_combinations: usize,
    pub compression_ratio: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub progress: ProgressConfig,
    pub prompts: PromptsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressConfig {
    pub spinner_style: SpinnerStyle,
    pub bar_style: BarStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpinnerStyle {
    pub tick_strings: Vec<String>,
    pub template: String,
    pub steady_tick_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BarStyle {
    pub template: String,
    pub progress_chars: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptsConfig {
    pub default_length: usize,
    pub default_start_index: u64,
    pub default_count: usize,
    pub default_output_file: String,
    pub default_json_output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingConfig {
    pub minimal_mapping: MinimalMappingConfig,
    pub complete_mapping: CompleteMappingConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalMappingConfig {
    pub version: String,
    pub include_compressed_data: bool,
    pub include_ascii_conversion: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteMappingConfig {
    pub version: String,
    pub include_reversal_instructions: bool,
    pub include_metadata: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub ipfs: IpfsConfig,
    pub local: LocalStorageConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsConfig {
    pub enabled: bool,
    pub gateway: String,
    pub pinata_jwt_env: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalStorageConfig {
    pub mapping_files: bool,
    pub compressed_files: bool,
    pub debug_files: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugConfig {
    pub save_debug_files: bool,
    pub debug_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub memory: MemoryConfig,
    pub compression: CompressionPerformanceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub max_chunk_size: usize,
    pub json_processing_chunk_size: usize,
    pub file_read_chunk_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionPerformanceConfig {
    pub optimal_chunk_search_range: Vec<usize>,
    pub compression_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub file: FileValidationConfig,
    pub compression: CompressionValidationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileValidationConfig {
    pub max_size_mb: usize,
    pub allowed_extensions: Vec<String>,
    pub ascii_safety: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionValidationConfig {
    pub min_ratio: f64,
    pub max_ratio: f64,
    pub target_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfig {
    pub colors: ColorConfig,
    pub messages: MessageConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorConfig {
    pub success: String,
    pub error: String,
    pub warning: String,
    pub info: String,
    pub highlight: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageConfig {
    pub upload_complete: String,
    pub compression_achieved: String,
    pub generation_complete: String,
    pub file_reconstructed: String,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError(serde_json::Error),
    IoError(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "Configuration file not found: {}", path),
            ConfigError::ParseError(e) => write!(f, "Failed to parse configuration: {}", e),
            ConfigError::IoError(e) => write!(f, "IO error reading configuration: {}", e),
        }
    }
}

impl Error for ConfigError {}

/// Loads the configuration from the config.json file
pub fn load_config() -> Result<Config, ConfigError> {
    let config_path = "config.json";
    
    if !Path::new(config_path).exists() {
        return Err(ConfigError::FileNotFound(config_path.to_string()));
    }
    
    let config_content = fs::read_to_string(config_path)
        .map_err(ConfigError::IoError)?;
    
    let config: Config = serde_json::from_str(&config_content)
        .map_err(ConfigError::ParseError)?;
    
    Ok(config)
}

/// Loads configuration with fallback to default values if file doesn't exist
pub fn load_config_or_default() -> Config {
    match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Warning: Failed to load config.json: {}", e);
            eprintln!("Using default configuration values");
            create_default_config()
        }
    }
}

/// Creates a default configuration
fn create_default_config() -> Config {
    Config {
        version: "1.0.0".to_string(),
        description: "Default StarkSqueeze configuration".to_string(),
        compression: CompressionConfig {
            target_compression_ratio: 66.7,
            current_compression_ratio: 66.7,
            compression_method: "dictionary_based".to_string(),
            chunk_size_range: ChunkSizeRange {
                min: 2,
                max: 8,
                default: 3,
            },
            optimal_compression_threshold: 0.1,
            max_unique_chunks: 255,
            compression_ratios: {
                let mut map = HashMap::new();
                map.insert("3_to_1".to_string(), CompressionRatio {
                    input_chars: 3,
                    output_bytes: 1,
                    ratio: 66.7,
                    description: "3 characters → 1 byte (66.7% compression) - fast testing".to_string(),
                });
                map.insert("5_to_1".to_string(), CompressionRatio {
                    input_chars: 5,
                    output_bytes: 1,
                    ratio: 80.0,
                    description: "5 characters → 1 byte (80% compression)".to_string(),
                });
                map.insert("10_to_1".to_string(), CompressionRatio {
                    input_chars: 10,
                    output_bytes: 1,
                    ratio: 90.0,
                    description: "10 characters → 1 byte (90% compression) - theoretical".to_string(),
                });
                map
            },
        },
        dictionary: DictionaryConfig {
            ascii_combinations: AsciiCombinationsConfig {
                default_length: 3,
                default_start_index: 0,
                default_count: 1000,
                total_possible_combinations: 2097152,
                ascii_chars: 128,
                output_file: "ascii_combinations.json".to_string(),
                generation: GenerationConfig {
                    chunk_size: 100000,
                    json_chunk_size: 10000,
                    flush_interval: 5,
                    combinations_per_second_estimate: 1000000,
                    fast_system_rate: 5000000,
                    medium_system_rate: 1000000,
                    slow_system_rate: 100000,
                },
            },
            ultra_compressed: UltraCompressedConfig {
                length: 3,
                start_index: 0,
                compression_ratio: 66.7,
                encoding: "key_value_dictionary_3to1".to_string(),
                description: "Ultra-compressed JSON with 66.7% compression - fast testing".to_string(),
            },
        },
        file_processing: FileProcessingConfig {
            ascii_conversion: AsciiConversionConfig {
                chunk_size: 8192,
                printable_range: PrintableRange {
                    min: 32,
                    max: 126,
                },
                conversion_map: ConversionMap {
                    control_chars: "space".to_string(),
                    extended_ascii: "period".to_string(),
                },
            },
            binary_string_conversion: BinaryStringConversionConfig {
                bits_per_byte: 8,
                format: "{:08b}".to_string(),
            },
        },
        upload: UploadConfig {
            hash: HashConfig {
                algorithm: "sha256".to_string(),
                short_hash_length: 8,
                upload_id_length: 16,
            },
            starknet: StarknetConfig {
                chunk_size: 8,
                field_element_size: 16,
                calldata_optimization: true,
            },
        },
        server: ServerConfig {
            port: 3000,
            host: "localhost".to_string(),
            endpoints: EndpointsConfig {
                health: "/health".to_string(),
                status: "/status".to_string(),
                compress: "/compress".to_string(),
                files: "/files".to_string(),
            },
            dictionary: DictionaryServerConfig {
                auto_generate: true,
                path: "ascii_combinations.json".to_string(),
                fallback_metadata: FallbackMetadata {
                    length: 3,
                    total_combinations: 1000,
                    compression_ratio: "66.7% (3 chars → 1 byte) - fast testing".to_string(),
                },
            },
        },
        cli: CliConfig {
            progress: ProgressConfig {
                spinner_style: SpinnerStyle {
                    tick_strings: vec![
                        "⠋".to_string(), "⠙".to_string(), "⠹".to_string(), "⠸".to_string(),
                        "⠼".to_string(), "⠴".to_string(), "⠦".to_string(), "⠧".to_string(),
                        "⠇".to_string(), "⠏".to_string(),
                    ],
                    template: "{spinner:.yellow} {msg}".to_string(),
                    steady_tick_ms: 100,
                },
                bar_style: BarStyle {
                    template: "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}".to_string(),
                    progress_chars: "#>-".to_string(),
                },
            },
            prompts: PromptsConfig {
                default_length: 3,
                default_start_index: 0,
                default_count: 1000,
                default_output_file: "ascii_combinations.txt".to_string(),
                default_json_output: "ascii_combinations.json".to_string(),
            },
        },
        mapping: MappingConfig {
            minimal_mapping: MinimalMappingConfig {
                version: "1.0".to_string(),
                include_compressed_data: true,
                include_ascii_conversion: true,
            },
            complete_mapping: CompleteMappingConfig {
                version: "1.0".to_string(),
                include_reversal_instructions: true,
                include_metadata: true,
            },
        },
        storage: StorageConfig {
            ipfs: IpfsConfig {
                enabled: true,
                gateway: "https://gateway.pinata.cloud/ipfs/".to_string(),
                pinata_jwt_env: "PINATA_JWT".to_string(),
            },
            local: LocalStorageConfig {
                mapping_files: true,
                compressed_files: true,
                debug_files: false,
            },
        },
        debug: DebugConfig {
            save_debug_files: true,
            debug_files: vec![
                "debug_original.bin".to_string(),
                "debug_ascii.bin".to_string(),
                "debug_binary_string.txt".to_string(),
                "debug_reconstructed_binary_string.txt".to_string(),
                "debug_reconstructed_ascii.bin".to_string(),
            ],
        },
        performance: PerformanceConfig {
            memory: MemoryConfig {
                max_chunk_size: 100000,
                json_processing_chunk_size: 10000,
                file_read_chunk_size: 8192,
            },
            compression: CompressionPerformanceConfig {
                optimal_chunk_search_range: vec![2, 8],
                compression_threshold: 0.1,
            },
        },
        validation: ValidationConfig {
            file: FileValidationConfig {
                max_size_mb: 1000,
                allowed_extensions: vec!["*".to_string()],
                ascii_safety: true,
            },
            compression: CompressionValidationConfig {
                min_ratio: 0.0,
                max_ratio: 100.0,
                target_ratio: 66.7,
            },
        },
        ui: UiConfig {
            colors: ColorConfig {
                success: "green".to_string(),
                error: "red".to_string(),
                warning: "yellow".to_string(),
                info: "blue".to_string(),
                highlight: "cyan".to_string(),
            },
            messages: MessageConfig {
                upload_complete: "Upload complete!".to_string(),
                compression_achieved: "🎉 80%+ compression achieved!".to_string(),
                generation_complete: "Generation complete!".to_string(),
                file_reconstructed: "✅ File reconstructed successfully".to_string(),
            },
        },
    }
}

/// Saves the current configuration to config.json
pub fn save_config(config: &Config) -> Result<(), ConfigError> {
    let config_content = serde_json::to_string_pretty(config)
        .map_err(|e| ConfigError::ParseError(e))?;
    
    fs::write("config.json", config_content)
        .map_err(ConfigError::IoError)?;
    
    Ok(())
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = load_config_or_default();
}

/// Get a reference to the global configuration
pub fn get_config() -> &'static Config {
    &CONFIG
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_config() {
        let config = create_default_config();
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.compression.target_compression_ratio, 90.0);
        assert_eq!(config.compression.current_compression_ratio, 80.0);
        assert_eq!(config.dictionary.ascii_combinations.default_length, 5);
    }

    #[test]
    fn test_config_serialization() {
        let config = create_default_config();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config.version, parsed.version);
    }
} 