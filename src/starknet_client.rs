use crate::utils::short_string_to_felt;
use starknet::accounts::Call;
use starknet::accounts::{Account, SingleOwnerAccount, ConnectedAccount};
use starknet::core::types::{BlockId, BlockTag, FieldElement, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use std::env;
use url::Url;
use dotenvy::dotenv;

/// Loads the StarkNet account from the environment.
pub async fn get_account() -> Result<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>, Box<dyn std::error::Error>>
{
    dotenv().ok(); // Load .env
    let rpc_url = env::var("RPC_URL").map_err(|_| "RPC_URL not set in .env")?;
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url)?));

    let private_key = env::var("PRIVATE_KEY").map_err(|_| "PRIVATE_KEY not set in .env")?;
    let private_key = FieldElement::from_hex_be(&private_key)
        .map_err(|e| format!("Invalid private key: {}", e))?;
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));

    let account_address = env::var("ACCOUNT_ADDRESS").map_err(|_| "ACCOUNT_ADDRESS not set in .env")?;
    let account_address = FieldElement::from_hex_be(&account_address)?;

    let chain_id = env::var("CHAIN_ID").map_err(|_| "CHAIN_ID not set in .env")?;
    let chain_id = FieldElement::from_hex_be(&chain_id)?;

    Ok(SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id,
        starknet::accounts::ExecutionEncoding::New,
    ))
}

/// Uploads compressed data metadata to the contract.
pub async fn upload_data(
    data_size: u64,
    file_type: &str,
    original_size: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let account = get_account().await?;
    let contract_address = env::var("CONTRACT_ADDRESS").map_err(|_| "CONTRACT_ADDRESS not set in .env")?;
    let contract_address = FieldElement::from_hex_be(&contract_address)?;

    let file_type_felt = short_string_to_felt(file_type)?;

    let call = Call {
        to: contract_address,
        selector: get_selector_from_name("store_compression_mapping")?,
        calldata: vec![
            FieldElement::from(data_size),    // data_size
            file_type_felt,                   // file_type
            FieldElement::from(original_size), // original_size
        ],
    };

    // Try to simulate the transaction first
    match account.provider().call(
        FunctionCall {
            contract_address,
            entry_point_selector: get_selector_from_name("store_compression_mapping")?,
            calldata: call.calldata.clone(),
        },
        BlockId::Tag(BlockTag::Latest),
    ).await {
        Ok(_) => {},
        Err(e) => {
            if e.to_string().contains("Invalid message selector") {
                return Err("Contract function 'store_compression_mapping' not found. Please verify the contract address and function name.".into());
            }
            return Err(format!("Transaction simulation failed: {}", e).into());
        }
    }

    let tx = account.execute(vec![call]).send().await?;
    println!("✅ Upload successful! Transaction hash: 0x{:x}", tx.transaction_hash);
    Ok(())
}
