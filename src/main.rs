use stark_squeeze::cli;
use clap::{Parser, Subcommand, error::ErrorKind};
use std::path::PathBuf;
use std::collections::HashSet;
use thiserror::Error;

const APP_NAME: &str = "StarkSqueeze CLI";
const APP_ABOUT: &str = "Interact with StarkSqueeze";

/// CLI arguments for StarkSqueeze
#[derive(Parser, Debug)]
#[command(name = APP_NAME, about = APP_ABOUT)]
#[command(arg_required_else_help = true)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Commands for the StarkSqueeze CLI
#[derive(Subcommand, Debug)]
enum Commands {
    /// Upload data to StarkNet
    Upload {
        /// Path to the file to upload
        #[arg(short, long, value_parser = validate_file_path)]
        file: Option<PathBuf>,
    },
    /// Retrieve data from StarkNet
    Retrieve {
        /// The upload ID or hash to retrieve
        #[arg(short, long, value_parser = validate_upload_id)]
        id: Option<String>,
    },
    /// List all uploaded data
    List,
}

/// Validates that the provided file path exists and is readable
fn validate_file_path(path: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    if !path_buf.is_file() {
        return Err(format!("Path is not a file: {}", path));
    }
    // Check if file is readable
    match std::fs::File::open(&path_buf) {
        Ok(_) => Ok(path_buf),
        Err(e) => Err(format!("Cannot read file {}: {}", path, e)),
    }
}

/// Validates that the provided upload ID is a valid hex string
fn validate_upload_id(id: &str) -> Result<String, String> {
    // Check if the input is a valid hex string
    if !id.starts_with("0x") && id.len() != 66 {
        return Err(format!("Invalid upload ID format. Expected 0x-prefixed 64-character hex string, got: {}", id));
    }
    
    // Check if all characters after 0x are valid hex digits
    if !id[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("Upload ID contains non-hexadecimal characters: {}", id));
    }
    
    Ok(id.to_string())
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.command {
        Some(Commands::Upload { file }) => {
            cli::upload_data_cli(file).await;
        },
        Some(Commands::Retrieve { id }) => {
            cli::retrieve_data_cli(id).await;
        },
        Some(Commands::List) => {
            cli::list_all_uploads().await;
        },
        None => cli::main_menu().await,
    }
}
