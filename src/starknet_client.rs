use crate::utils::short_string_to_felt;
use starknet::accounts::Call;
use starknet::accounts::{Account, SingleOwnerAccount, ConnectedAccount};
use starknet::core::types::{BlockId, BlockTag, FieldElement, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use std::env;
use std::error::Error;
use url::Url;
use dotenvy::dotenv;
use indicatif::{ProgressBar, ProgressStyle};

/// Loads an environment variable or returns an error.
fn get_env_var(name: &str) -> Result<String, Box<dyn Error>> {
    env::var(name).map_err(|_| format!("Environment variable `{}` is not set", name).into())
}

/// Parses a FieldElement from an environment variable.
fn get_env_felt(name: &str) -> Result<FieldElement, Box<dyn Error>> {
    let val = get_env_var(name)?;
    FieldElement::from_hex_be(&val).map_err(|e| format!("Invalid FieldElement in `{}`: {}", name, e).into())
}

/// Loads the StarkNet account from the environment and private key.
pub async fn get_account(
    private_key_hex: &str,
) -> Result<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>, Box<dyn std::error::Error>>
{
    dotenv().ok(); // Load .env
    let rpc_url = get_env_var("RPC_URL")?;
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url)?));

    let private_key = FieldElement::from_hex_be(private_key_hex)
        .map_err(|e| format!("Invalid private key: {}", e))?;
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));

    let account_address = get_env_felt("ACCOUNT_ADDRESS")?;
    let chain_id = get_env_felt("CHAIN_ID")?;

    Ok(SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id,
        starknet::accounts::ExecutionEncoding::Legacy,
    ))
}

/// Uploads compressed data metadata to the contract.
pub async fn upload_data(
    private_key: &str,
    upload_id: FieldElement,
    original_size: u64,
    compressed_size: u64,
    file_type: &str,
    compression_ratio: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let account = get_account(private_key).await?;
    let contract_address = get_env_felt("CONTRACT_ADDRESS")?;
    let file_type_felt = short_string_to_felt(file_type)?;

    let call = Call {
        to: contract_address,
        selector: get_selector_from_name("upload_data")?,
        calldata: vec![
            upload_id,
            FieldElement::from(original_size),
            FieldElement::from(compressed_size),
            file_type_felt,
            FieldElement::from(compression_ratio),
        ],
    };

    let tx = account.execute(vec![call]).send().await?;
    println!(
        "✅ Upload transaction sent! Tx Hash: {:?}",
        tx.transaction_hash
    );
    Ok(())
}

/// Uploads data to the contract with progress display.
pub async fn upload_data_with_progress(
    private_key: &str,
    upload_id: FieldElement,
    original_size: u64,
    compressed_size: u64,
    file_type: &str,
    compression_ratio: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let account = get_account(private_key).await?;
    let contract_address = get_env_felt("CONTRACT_ADDRESS")?;
    let file_type_felt = short_string_to_felt(file_type)?;

    let call = Call {
        to: contract_address,
        selector: get_selector_from_name("upload_data")?,
        calldata: vec![
            upload_id,
            FieldElement::from(original_size),
            FieldElement::from(compressed_size),
            file_type_felt,
            FieldElement::from(compression_ratio),
        ],
    };

    // Initialize progress bar
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")?
            .progress_chars("#>-"),
    );

    progress_bar.set_message("Uploading data to StarkNet...");

    // Simulate progress (replace with actual progress tracking if possible)
    for i in 0..100 {
        progress_bar.inc(1);
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    let tx = account.execute(vec![call]).send().await?;
    progress_bar.finish_with_message("Upload complete!");

    println!(
        "✅ Upload transaction sent! Tx Hash: {:?}",
        tx.transaction_hash
    );
    Ok(())
}

/// Retrieves metadata for a given upload ID.
pub async fn retrieve_data(
    private_key: &str,
    upload_id: FieldElement,
) -> Result<(u64, u64, String, u64), Box<dyn std::error::Error>> {
    dotenv().ok();
    let account = get_account(private_key).await?;
    let contract_address = get_env_felt("CONTRACT_ADDRESS")?;

    let call = FunctionCall {
        contract_address,
        entry_point_selector: get_selector_from_name("retrieve_data")?,
        calldata: vec![upload_id],
    };

    let result = account
        .provider()
        .call(call, BlockId::Tag(BlockTag::Latest))
        .await?;

    Ok((
        result[0]
            .to_string()
            .parse::<u64>()
            .map_err(|e| format!("Invalid original size: {}", e))?,
        result[1]
            .to_string()
            .parse::<u64>()
            .map_err(|e| format!("Invalid compressed size: {}", e))?,
        result[2].to_string(), // file_type as a string
        result[3]
            .to_string()
            .parse::<u64>()
            .map_err(|e| format!("Invalid compression ratio: {}", e))?,
    ))
}

/// Retrieves all uploaded file metadata entries.
pub async fn get_all_data(
    private_key: &str,
) -> Result<Vec<(FieldElement, String, u64)>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let account = get_account(private_key).await?;
    let contract_address = get_env_felt("CONTRACT_ADDRESS")?;

    let call = FunctionCall {
        contract_address,
        entry_point_selector: get_selector_from_name("get_all_data")?,
        calldata: vec![],
    };

    let result = account
        .provider()
        .call(call, BlockId::Tag(BlockTag::Latest))
        .await?;

    let mut data = Vec::new();
    let chunks = result.chunks(3);
    for chunk in chunks {
        if let [upload_id, file_type_felt, compression_ratio_felt] = chunk {
            let file_type = file_type_felt.to_string(); // Consider converting back to string if you used `felt_to_short_string`
            let compression_ratio = compression_ratio_felt.to_string().parse::<u64>()?;
            data.push((*upload_id, file_type, compression_ratio));
        }
    }

    Ok(data)
}
