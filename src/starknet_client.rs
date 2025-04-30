use crate::utils::short_string_to_felt;
use starknet::accounts::Call;
use starknet::accounts::{Account, ConnectedAccount, SingleOwnerAccount};
use starknet::core::types::{BlockId, BlockTag, FieldElement, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use std::num::ParseIntError;
use url::Url;

const RPC_URL: &str = "https://starknet-testnet.public.blastapi.io";
const CONTRACT_ADDRESS: &str = "0xYOUR_CONTRACT_ADDRESS";
const ACCOUNT_ADDRESS: &str = "0x478935085396927196704S";
const CHAIN_ID: &str = "YOUR_CHAIN_ID";

pub async fn get_account(
    private_key_hex: &str,
) -> Result<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>, Box<dyn std::error::Error>>
{
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(RPC_URL)?));
    let private_key = FieldElement::from_hex_be(private_key_hex)
        .map_err(|e| format!("Invalid private key: {}", e))?;
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));
    let account_address = FieldElement::from_hex_be(ACCOUNT_ADDRESS)
        .map_err(|e| format!("Invalid account address: {}", e))?;
    let chain_id =
        FieldElement::from_hex_be(CHAIN_ID).map_err(|e| format!("Invalid chain ID: {}", e))?;
    Ok(SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id,
        starknet::accounts::ExecutionEncoding::Legacy,
    ))
}

pub async fn upload_data(
    private_key: &str,
    upload_id: FieldElement,
    original_size: u64,
    compressed_size: u64,
    file_type: &str,
    compression_ratio: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let account = get_account(private_key).await?;
    let contract_address = FieldElement::from_hex_be(CONTRACT_ADDRESS)
        .map_err(|e| format!("Invalid contract address: {}", e))?;
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

pub async fn retrieve_data(
    private_key: &str,
    upload_id: FieldElement,
) -> Result<(u64, u64, String, u64), Box<dyn std::error::Error>> {
    let account = get_account(private_key).await?;
    let contract_address = FieldElement::from_hex_be(CONTRACT_ADDRESS)
        .map_err(|e| format!("Invalid contract address: {}", e))?;

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
        result[2].to_string(),
        result[3]
            .to_string()
            .parse::<u64>()
            .map_err(|e| format!("Invalid compression ratio: {}", e))?,
    ))
}

pub async fn get_all_data(
    private_key: &str,
) -> Result<Vec<(FieldElement, String, u64)>, Box<dyn std::error::Error>> {
    let account = get_account(private_key).await?;
    let contract_address = FieldElement::from_hex_be(CONTRACT_ADDRESS)
        .map_err(|e| format!("Invalid contract address: {}", e))?;

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
    for i in 0..result.len() {
        let upload_id = result[i];
        let file_type = "txt".to_string(); // Replace with actual file type
        let compression_ratio = 50; // Replace with actual compression ratio
        data.push((upload_id, file_type, compression_ratio));
    }

    Ok(data)
}
