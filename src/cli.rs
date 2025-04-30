use crate::starknet_client::{get_all_data, retrieve_data, upload_data};
use colored::*;
use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use starknet::core::types::FieldElement;
use std::borrow::Cow;
use std::time::Duration;

pub async fn upload_data_cli() {
    let private_key = loop {
        match Input::<String>::new()
            .with_prompt("Enter your private key")
            .interact_text()
        {
            Ok(key) => break key,
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    };

    let file_path = loop {
        match Input::<String>::new()
            .with_prompt("Enter the file path")
            .interact_text()
        {
            Ok(path) => break path,
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    };

    let file_type = loop {
        match Input::<String>::new()
            .with_prompt("Enter the file type")
            .interact_text()
        {
            Ok(file_type) => break file_type,
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    };

    let original_size = loop {
        match Input::<String>::new()
            .with_prompt("Enter the original size (bytes)")
            .interact_text()
        {
            Ok(size_str) => match size_str.parse::<u64>() {
                Ok(size) => break size,
                Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
            },
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    };

    let upload_id = FieldElement::from(1u64); // Use u64 for FieldElement
    let compressed_size = original_size / 2; // Simulate compression
    let compression_ratio = (compressed_size as f64 / original_size as f64 * 100.0) as u64;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message(Cow::from("Encoding file...".yellow().to_string()));
    std::thread::sleep(Duration::from_secs(2));
    spinner.set_message(Cow::from("Uploading data...".yellow().to_string()));
    std::thread::sleep(Duration::from_secs(2));
    spinner.finish_with_message(Cow::from("Upload complete!".green().to_string()));

    if let Err(e) = upload_data(
        &private_key,
        upload_id,
        original_size,
        compressed_size,
        &file_type,
        compression_ratio,
    )
    .await
    {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return;
    }

    println!("{} {}", "Upload ID:".blue().bold(), upload_id);
    println!("{} {} bytes", "Original Size:".blue().bold(), original_size);
    println!("{} {} bytes", "New Size:".blue().bold(), compressed_size);
    println!(
        "{} {}%",
        "Compression Ratio:".blue().bold(),
        compression_ratio
    );
}

pub async fn retrieve_data_cli() {
    let private_key = loop {
        match Input::<String>::new()
            .with_prompt("Enter your private key")
            .interact_text()
        {
            Ok(key) => break key,
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    };

    let upload_id = loop {
        match Input::<String>::new()
            .with_prompt("Enter the upload ID or hash")
            .interact_text()
        {
            Ok(id_str) => match FieldElement::from_hex_be(&id_str) {
                Ok(id) => break id,
                Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
            },
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    };

    match retrieve_data(&private_key, upload_id).await {
        Ok((original_size, compressed_size, file_type, compression_ratio)) => {
            println!("{}", "Decoded binary status: Success".green().bold());
            println!("{} {}", "File Type:".blue().bold(), file_type);
            println!("{} {} bytes", "Original Size:".blue().bold(), original_size);
            println!(
                "{} {} bytes",
                "Compressed Size:".blue().bold(),
                compressed_size
            );
            println!(
                "{} {}%",
                "Compression Ratio:".blue().bold(),
                compression_ratio
            );
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
        }
    }
}

pub async fn list_all_uploads() {
    let private_key = loop {
        match Input::<String>::new()
            .with_prompt("Enter your private key")
            .interact_text()
        {
            Ok(key) => break key,
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    };

    match get_all_data(&private_key).await {
        Ok(data) => {
            if data.is_empty() {
                println!("{}", "No uploads found.".yellow().bold());
            } else {
                for (upload_id, file_type, compression_ratio) in data {
                    println!("{} {}", "ID:".blue().bold(), upload_id);
                    println!("{} {}", "File Type:".blue().bold(), file_type);
                    println!(
                        "{} {}%",
                        "Compression Ratio:".blue().bold(),
                        compression_ratio
                    );
                    println!("---");
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
        }
    }
}

pub async fn main_menu() {
    loop {
        println!("{}", "🚀 Welcome to StarkSqueeze CLI!".bold().cyan());
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
                eprintln!("{} {}", "Error:".red().bold(), e);
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
