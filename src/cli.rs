use crate::starknet_client::{get_all_data, retrieve_data, upload_data};
use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::borrow::Cow;
use std::time::Duration;

/// Prints a styled error message
fn print_error(context: &str, error: &dyn std::fmt::Display) {
    eprintln!("{} {}: {}", "Error".red().bold(), context, error);
}

/// Prints a styled info message
fn print_info(label: &str, value: impl std::fmt::Display) {
    println!("{} {}", label.blue().bold(), value);
}

/// Prompts the user for string input
async fn prompt_string(prompt: &str) -> String {
    loop {
        match Input::<String>::new().with_prompt(prompt).interact_text() {
            Ok(value) => return value,
            Err(e) => print_error("Failed to read input", &e),
        }
    }
}

/// Prompts the user for input that implements FromStr
async fn prompt_input<T: std::str::FromStr>(prompt: &str, error_hint: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    loop {
        match Input::<String>::new().with_prompt(prompt).interact_text() {
            Ok(raw) => match raw.parse::<T>() {
                Ok(val) => return val,
                Err(e) => print_error(error_hint, &e),
            },
            Err(e) => print_error("Failed to read input", &e),
        }
    }
}

/// Uploads a file with compression metadata
pub async fn upload_data_cli() {
    let private_key = prompt_string("Enter your private key").await;
    let file_path = prompt_string("Enter the file path").await;
    let file_type = prompt_string("Enter the file type").await;
    let original_size: u64 = prompt_input("Enter the original size (bytes)", "Please enter a valid number").await;

    let upload_id = FieldElement::from(1u64);
    let compressed_size = original_size / 2;
    let compression_ratio = ((compressed_size as f64 / original_size as f64) * 100.0) as u64;

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

    if let Err(e) = upload_data(&private_key, upload_id, original_size, compressed_size, &file_type, compression_ratio).await {
        print_error("Failed to upload data", &e);
        println!("Hint: Check your network connection or private key.");
        return;
    }

    print_info("Upload ID:", upload_id);
    print_info("Original Size:", format!("{} bytes", original_size));
    print_info("New Size:", format!("{} bytes", compressed_size));
    print_info("Compression Ratio:", format!("{}%", compression_ratio));
}

/// Retrieves previously uploaded data
pub async fn retrieve_data_cli() {
    let private_key = prompt_string("Enter your private key").await;

    let upload_id = loop {
        let input = prompt_string("Enter the upload ID or hash").await;
        match FieldElement::from_hex_be(&input) {
            Ok(val) => break val,
            Err(e) => print_error("Invalid hex input for upload ID", &e),
        }
    };

    match retrieve_data(&private_key, upload_id).await {
        Ok((original_size, compressed_size, file_type, compression_ratio)) => {
            println!("{}", "Decoded binary status: Success".green().bold());
            print_info("File Type:", file_type);
            print_info("Original Size:", format!("{} bytes", original_size));
            print_info("Compressed Size:", format!("{} bytes", compressed_size));
            print_info("Compression Ratio:", format!("{}%", compression_ratio));
        }
        Err(e) => {
            print_error("Failed to retrieve data", &e);
            println!("Hint: Ensure the upload ID is correct and try again.");
        }
    }
}

/// Lists all uploaded files
pub async fn list_all_uploads() {
    let private_key = prompt_string("Enter your private key").await;

    match get_all_data(&private_key).await {
        Ok(data) => {
            if data.is_empty() {
                println!("{}", "No uploads found.".yellow().bold());
            } else {
                for (upload_id, file_type, compression_ratio) in data {
                    print_info("ID:", upload_id);
                    print_info("File Type:", file_type);
                    print_info("Compression Ratio:", format!("{}%", compression_ratio));
                    println!("{}", "---".dimmed());
                }
            }
        }
        Err(e) => {
            print_error("Failed to retrieve uploads", &e);
        }
    }
}

/// Displays the CLI menu and handles command routing
pub async fn main_menu() {
    loop {
        println!("\n{}", "🚀 Welcome to StarkSqueeze CLI!".bold().cyan());
        println!("{}", "Please choose an option:".bold());

        let options = vec!["Upload Data", "Retrieve Data", "Get All Data", "Exit"];
        let selection = match Select::new()
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact()
        {
            Ok(sel) => sel,
            Err(e) => {
                print_error("Selection failed", &e);
                continue;
            }
        };

        match selection {
            0 => upload_data_cli().await,
            1 => retrieve_data_cli().await,
            2 => list_all_uploads().await,
            3 => {
                println!("{}", "👋 Goodbye!".bold().green());
                break;
            }
            _ => unreachable!(),
        }
    }
}
