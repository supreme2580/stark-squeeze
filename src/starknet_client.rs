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
    uri: &str,
    file_format: &str,
    compressed_by: u8,
    original_size: usize,
    final_size: usize,
    chunk_size: usize,
    chunk_mappings: Vec<FieldElement>,
    chunk_values: Vec<u8>,
    byte_mappings: Vec<u8>,
    byte_values: Vec<FieldElement>,
    reconstruction_steps: Vec<FieldElement>,
    metadata: Vec<FieldElement>,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let account = get_account().await?;
    let contract_address = env::var("CONTRACT_ADDRESS").map_err(|_| "CONTRACT_ADDRESS not set in .env")?;
    let contract_address = FieldElement::from_hex_be(&contract_address)?;

    let uri_felt = match short_string_to_felt(uri) {
        Ok(felt) => felt,
        Err(e) => {
            eprintln!("[short_string_to_felt ERROR] Failed string: '{}', error: {}", uri, e);
            return Err(format!("short_string_to_felt failed for uri '{}': {}", uri, e).into());
        }
    };
    let file_format_felt = match short_string_to_felt(file_format) {
        Ok(felt) => felt,
        Err(e) => {
            eprintln!("[short_string_to_felt ERROR] Failed string: '{}', error: {}", file_format, e);
            return Err(format!("short_string_to_felt failed for file_format '{}': {}", file_format, e).into());
        }
    };

    // Store lengths before moving vectors
    let chunk_mappings_len = chunk_mappings.len();
    let chunk_values_len = chunk_values.len();
    let byte_mappings_len = byte_mappings.len();
    let byte_values_len = byte_values.len();
    let reconstruction_steps_len = reconstruction_steps.len();
    let metadata_len = metadata.len();

    // Convert vectors to calldata format
    let mut calldata = vec![
        uri_felt,                                    // uri
        file_format_felt,                            // file_format
        FieldElement::from(compressed_by),           // compressed_by
        FieldElement::from(original_size),           // original_size
        FieldElement::from(final_size),              // final_size
        FieldElement::from(chunk_size),              // chunk_size
        FieldElement::from(chunk_mappings_len),      // chunk_mappings array length
    ];
    
    // Add chunk_mappings
    calldata.extend(chunk_mappings);
    
    // Add chunk_values array length and values
    calldata.push(FieldElement::from(chunk_values_len));
    calldata.extend(chunk_values.into_iter().map(FieldElement::from));
    
    // Add byte_mappings array length and values
    calldata.push(FieldElement::from(byte_mappings_len));
    calldata.extend(byte_mappings.into_iter().map(FieldElement::from));
    
    // Add byte_values array length and values
    calldata.push(FieldElement::from(byte_values_len));
    calldata.extend(byte_values);
    
    // Add reconstruction_steps array length and values
    calldata.push(FieldElement::from(reconstruction_steps_len));
    calldata.extend(reconstruction_steps);
    
    // Add metadata array length and values
    calldata.push(FieldElement::from(metadata_len));
    calldata.extend(metadata);

    // Debug: Print calldata structure
    println!("[DEBUG] Calldata structure:");
    println!("  uri: {}", uri_felt);
    println!("  file_format: {}", file_format_felt);
    println!("  compressed_by: {}", compressed_by);
    println!("  original_size: {}", original_size);
    println!("  final_size: {}", final_size);
    println!("  chunk_size: {}", chunk_size);
    println!("  chunk_mappings: {} items", chunk_mappings_len);
    println!("  chunk_values: {} items", chunk_values_len);
    println!("  byte_mappings: {} items", byte_mappings_len);
    println!("  byte_values: {} items", byte_values_len);
    println!("  reconstruction_steps: {} items", reconstruction_steps_len);
    println!("  metadata: {} items", metadata_len);
    println!("  Total calldata length: {}", calldata.len());

    let call = Call {
        to: contract_address,
        selector: get_selector_from_name("store_compression_mapping")?,
        calldata,
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
            eprintln!("[CONTRACT ERROR] Full error details: {:?}", e);
            eprintln!("[CONTRACT ERROR] Error string: {}", e);
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
