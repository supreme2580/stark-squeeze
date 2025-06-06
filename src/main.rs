use stark_squeeze::cli;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write, Read};
use std::time::{Duration, Instant};
use serde::Serialize;
use sha2::{Sha256, Digest};
use hex;
use std::collections::HashSet;
use thiserror::Error;

const APP_NAME: &str = "StarkSqueeze CLI";
const APP_ABOUT: &str = "Interact with StarkSqueeze";

const TOTAL_COMBINATIONS: u64 = 127u64.pow(5);
const BATCH_SIZE: u64 = 1_000_000;
const CHECKPOINT_INTERVAL: u64 = 100_000;
const MAX_MEMORY_BYTES: u64 = 1_000_000_000;
const TIMEOUT_MINUTES: u64 = 10;
const OUTPUT_FILE: &str = "dictionary.json";
const CHECKPOINT_FILE: &str = "checkpoint.txt";

#[derive(Parser, Debug)]
#[command(name = APP_NAME, about = APP_ABOUT)]
#[command(arg_required_else_help = true)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Upload data to StarkNet
    Upload {
        #[arg(short, long, value_parser = validate_file_path)]
        file: Option<PathBuf>,
    },
    /// Retrieve data from StarkNet
    Retrieve {
        #[arg(short, long, value_parser = validate_upload_id)]
        id: Option<String>,
    },
    /// List all uploaded data
    List,
    /// Generate dictionary of 5-character ASCII combinations
    GenerateDictionary,
}

fn validate_file_path(path: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    if !path_buf.is_file() {
        return Err(format!("Path is not a file: {}", path));
    }
    match std::fs::File::open(&path_buf) {
        Ok(_) => Ok(path_buf),
        Err(e) => Err(format!("Cannot read file {}: {}", path, e)),
    }
}

fn validate_upload_id(id: &str) -> Result<String, String> {
    if !id.starts_with("0x") && id.len() != 66 {
        return Err(format!("Invalid upload ID format. Expected 0x-prefixed 64-character hex string, got: {}", id));
    }

    // Check if all characters after 0x are valid hex digits
    if !id[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("Upload ID contains non-hexadecimal characters: {}", id));
    }
    Ok(id.to_string())
}

#[derive(Serialize)]
struct DictionaryHeader {
    version: String,
    total_combinations: u64,
}

#[derive(Serialize)]
struct Entry {
    pattern: String,
    value: u64,
}

fn get_memory_usage_bytes() -> u64 {
    if let Ok(contents) = fs::read_to_string("/proc/self/statm") {
        contents.split_whitespace()
            .next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0) * 4096
    } else {
        0
    }
}

fn save_checkpoint(index: u64) {
    let _ = fs::write(CHECKPOINT_FILE, index.to_string());
}

fn load_checkpoint() -> u64 {
    if let Ok(contents) = fs::read_to_string(CHECKPOINT_FILE) {
        contents.trim().parse().unwrap_or(0)
    } else {
        0
    }
}

fn generate_pattern(mut value: u64) -> String {
    let mut chars = vec![0u8; 5];
    for i in (0..5).rev() {
        chars[i] = (value % 127) as u8;
        value /= 127;
    }
    chars.iter().map(|&b| b as char).collect()
}

fn generate_dictionary() -> std::io::Result<()> {
    let start = Instant::now();
    let mut index = load_checkpoint();

    let mut file = OpenOptions::new()
        .create(true)
        .append(index != 0)
        .write(true)
        .open(OUTPUT_FILE)?;

    let mut writer = BufWriter::new(&file);

    if index == 0 {
        writeln!(writer, "{{")?;
        writeln!(writer, "\"version\": \"1.0\",")?;
        writeln!(writer, "\"total_combinations\": {},", TOTAL_COMBINATIONS)?;
        writeln!(writer, "\"entries\": [")?;
    }

    while index < TOTAL_COMBINATIONS {
        if start.elapsed() > Duration::from_secs(TIMEOUT_MINUTES * 60) {
            eprintln!("⏱ Timeout exceeded. Aborting...");
            break;
        }

        if get_memory_usage_bytes() > MAX_MEMORY_BYTES {
            eprintln!("🧠 Memory usage exceeded 1GB. Aborting...");
            break;
        }

        let end = (index + BATCH_SIZE).min(TOTAL_COMBINATIONS);
        for i in index..end {
            if i > 0 {
                write!(writer, ",")?;
            }
            let entry = Entry {
                pattern: generate_pattern(i),
                value: i,
            };
            serde_json::to_writer(&mut writer, &entry)?;
            writeln!(writer)?;

            if i % CHECKPOINT_INTERVAL == 0 {
                save_checkpoint(i);
            }
        }

        index = end;
        writer.flush()?;
        println!("✅ Progress: {}/{}", index, TOTAL_COMBINATIONS);
    }

    if index >= TOTAL_COMBINATIONS {
        writeln!(writer, "]\n}}")?;
        writer.flush()?;
        fs::remove_file(CHECKPOINT_FILE).ok();

        // SHA-256 hash of the file
        let mut file = std::fs::File::open(OUTPUT_FILE)?;
        let mut buffer = [0u8; 4096];
        let mut hasher = Sha256::new();
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        let result = hasher.finalize();
        println!("🔐 SHA-256: {}", hex::encode(result));
    }

    Ok(())
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
        Some(Commands::GenerateDictionary) => {
            if let Err(e) = generate_dictionary() {
                eprintln!("❌ Error generating dictionary: {}", e);
            }
        },
        None => cli::main_menu().await,
    }
}use std::time::{Instant, Duration};

#[cfg(target_os = "linux")]
fn current_rss_bytes() -> u64 {
    // Read resident-set size from /proc/self/statm (page-count * 4 kB)
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1)?.parse::<u64>().ok())
        .unwrap_or(0) * 4_096
}

#[cfg(not(target_os = "linux"))]
fn current_rss_bytes() -> u64 {
    0 // best-effort fallback – extend with `sysinfo` for mac/win if needed
}

/// A very light, zero-alloc memory profiler.
pub struct MemProfiler {
    label:   &'static str,
    t0:      Instant,
    rss0:    u64,
    printed: bool,
}

impl MemProfiler {
    pub fn new(label: &'static str) -> Self {
        Self { label, t0: Instant::now(), rss0: current_rss_bytes(), printed: false }
    }

    /// Print delta RSS and elapsed wall-time with an optional message.
    pub fn checkpoint(&mut self, msg: impl AsRef<str>) {
        let rss_now = current_rss_bytes();
        let delta   = rss_now.saturating_sub(self.rss0);
        let elapsed = self.t0.elapsed();
        eprintln!(
            "🧩 [{}] {} — ΔRSS: {:.2} MB, elapsed: {} ms",
            self.label,
            msg.as_ref(),
            delta as f64 / 1_048_576.0,
            elapsed.as_millis()
        );
        self.printed = true;
    }
}

impl Drop for MemProfiler {
    fn drop(&mut self) {
        if !self.printed {
            self.checkpoint("completed");
        }
    }
}

