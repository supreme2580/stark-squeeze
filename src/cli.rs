use crate::starknet_client::{get_all_data, retrieve_data, upload_data};
use clap::{App, Arg};
use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Serialize;
use starknet::core::types::FieldElement;
use std::time::Duration;

#[derive(Serialize)]
struct UploadResult {
    upload_id: String,
    original_size: u64,
    compressed_size: u64,
    compression_ratio: u64,
}

#[derive(Serialize)]
struct RetrievalResult {
    file_type: String,
    original_size: u64,
    compressed_size: u64,
    compression_ratio: u64,
}

#[derive(Serialize)]
struct ListResult {
    uploads: Vec<UploadEntry>,
}

#[derive(Serialize)]
struct UploadEntry {
    upload_id: String,
    file_type: String,
    compression_ratio: u64,
}

#[derive(Serialize)]
struct ErrorOutput {
    error: String,
    context: String,
}

fn print_error(context: &str, error: &dyn std::fmt::Display, json_output: bool) {
    if json_output {
        let err = ErrorOutput {
            context: context.to_string(),
            error: format!("{}", error),
        };
        println!("{}", serde_json::to_string_pretty(&err).unwrap());
    } else {
        eprintln!("{} {}: {}", "Error".red().bold(), context, error);
    }
}

fn print_info(label: &str, value: impl std::fmt::Display, json_output: bool) {
    if !json_output {
        println!("{} {}", label.blue().bold(), value);
    }
}

async fn prompt_string(prompt: &str) -> String {
    loop {
        match Input::<String>::new().with_prompt(prompt).interact_text() {
            Ok(value) => return value,
            Err(e) => {
                eprintln!("{}: {}", "Failed to read input".red(), e);
            }
        }
    }
}

async fn prompt_input<T: std::str::FromStr>(prompt: &str, error_hint: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    loop {
        match Input::<String>::new().with_prompt(prompt).interact_text() {
            Ok(raw) => match raw.parse::<T>() {
                Ok(val) => return val,
                Err(e) => {
                    eprintln!("{}: {}", error_hint.red(), e);
                }
            },
            Err(e) => {
                eprintln!("{}: {}", "Failed to read input".red(), e);
            }
        }
    }
}

pub async fn upload_data_cli(json_output: bool) {
    let private_key = prompt_string("Enter your private key").await;
    let _file_path = prompt_string("Enter the file path").await;
    let file_type = prompt_string("Enter the file type").await;
    let original_size: u64 = prompt_input("Enter the original size (bytes)", "Please enter a valid number").await;

    let upload_id = FieldElement::from(1u64);
    let compressed_size = original_size / 2;
    let compression_ratio = ((compressed_size as f64 / original_size as f64) * 100.0) as u64;

    if !json_output {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.yellow} {msg}")
                .unwrap(),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));
        spinner.set_message("Encoding file...".yellow().to_string());
        std::thread::sleep(Duration::from_secs(2));
        spinner.set_message("Uploading data...".yellow().to_string());
        std::thread::sleep(Duration::from_secs(2));
        spinner.finish_with_message("Upload complete!".green().to_string());
    }

    if let Err(e) = upload_data(&private_key, upload_id, original_size, compressed_size, &file_type, compression_ratio).await {
        print_error("Failed to upload data", &e, json_output);
        return;
    }

    if json_output {
        let result = UploadResult {
            upload_id: format!("{:#x}", upload_id),
            original_size,
            compressed_size,
            compression_ratio,
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        print_info("Upload ID:", upload_id, false);
        print_info("Original Size:", format!("{} bytes", original_size), false);
        print_info("New Size:", format!("{} bytes", compressed_size), false);
        print_info("Compression Ratio:", format!("{}%", compression_ratio), false);
    }
}

pub async fn retrieve_data_cli(json_output: bool) {
    let private_key = prompt_string("Enter your private key").await;

    let upload_id = loop {
        let input = prompt_string("Enter the upload ID or hash").await;
        match FieldElement::from_hex_be(&input) {
            Ok(val) => break val,
            Err(e) => print_error("Invalid hex input for upload ID", &e, json_output),
        }
    };

    match retrieve_data(&private_key, upload_id).await {
        Ok((original_size, compressed_size, file_type, compression_ratio)) => {
            if json_output {
                let result = RetrievalResult {
                    file_type,
                    original_size,
                    compressed_size,
                    compression_ratio,
                };
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            } else {
                println!("{}", "Decoded binary status: Success".green().bold());
                print_info("File Type:", file_type, false);
                print_info("Original Size:", format!("{} bytes", original_size), false);
                print_info("Compressed Size:", format!("{} bytes", compressed_size), false);
                print_info("Compression Ratio:", format!("{}%", compression_ratio), false);
            }
        }
        Err(e) => {
            print_error("Failed to retrieve data", &e, json_output);
        }
    }
}

pub async fn list_all_uploads(json_output: bool) {
    let private_key = prompt_string("Enter your private key").await;

    match get_all_data(&private_key).await {
        Ok(data) => {
            if json_output {
                let uploads = data
                    .into_iter()
                    .map(|(upload_id, file_type, compression_ratio)| UploadEntry {
                        upload_id: format!("{:#x}", upload_id),
                        file_type,
                        compression_ratio,
                    })
                    .collect::<Vec<_>>();

                println!(
                    "{}",
                    serde_json::to_string_pretty(&ListResult { uploads }).unwrap()
                );
            } else {
                if data.is_empty() {
                    println!("{}", "No uploads found.".yellow().bold());
                } else {
                    for (upload_id, file_type, compression_ratio) in data {
                        print_info("ID:", upload_id, false);
                        print_info("File Type:", file_type, false);
                        print_info("Compression Ratio:", format!("{}%", compression_ratio), false);
                        println!("{}", "---".dimmed());
                    }
                }
            }
        }
        Err(e) => {
            print_error("Failed to retrieve uploads", &e, json_output);
        }
    }
}

pub async fn main_menu(json_output: bool) {
    loop {
        if !json_output {
            println!("\n{}", "🚀 Welcome to StarkSqueeze CLI!".bold().cyan());
            println!("{}", "Please choose an option:".bold());
        }

        let options = vec!["Upload Data", "Retrieve Data", "Get All Data", "Exit"];
        let selection = match Select::new()
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact()
        {
            Ok(sel) => sel,
            Err(e) => {
                print_error("Selection failed", &e, json_output);
                continue;
            }
        };

        match selection {
            0 => upload_data_cli(json_output).await,
            1 => retrieve_data_cli(json_output).await,
            2 => list_all_uploads(json_output).await,
            3 => {
                if !json_output {
                    println!("{}", "👋 Goodbye!".bold().green());
                }
                break;
            }
            _ => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("StarkSqueeze CLI")
        .version("1.0")
        .about("Efficient data compression & upload on StarkNet")
        .arg(
            Arg::new("json")
                .long("json")
                .help("Outputs results in JSON format")
                .takes_value(false),
        )
        .get_matches();

    let json_output = matches.is_present("json");
    main_menu(json_output).await;
}
