mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::Provider;
use alloy_rpc_types::{eth::TransactionRequest, TransactionInput};
use alloy_sol_types::{sol, SolCall};
use serde::{Deserialize, Serialize};
use wavs_wasi_chain::ethereum::new_eth_provider;
use wstd::runtime::block_on;

struct Component;
export!(Component with_types_in bindings);

// Define ERC20 interface for USDT
sol! {
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint256);
    }
}

// USDT token address on Ethereum mainnet
const USDT_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
const RPC_URL: &str = "https://ethereum-rpc.publicnode.com";

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to an Ethereum address
        // First try direct string conversion
        let input = match std::str::from_utf8(&req) {
            Ok(s) => s.to_string(),
            // If that fails, try to interpret it as a hex string
            Err(_) => {
                // Convert raw bytes to a hex string
                format!("0x{}", hex::encode(&req))
            }
        };
        println!("Input address: {}", input);

        // Parse the Ethereum address
        let owner_address =
            input.parse::<Address>().map_err(|e| format!("Invalid address: {}", e))?;

        // Get the USDT token address
        let usdt_address =
            USDT_ADDRESS.parse::<Address>().map_err(|e| format!("Invalid USDT address: {}", e))?;

        // Query the USDT balance
        let balance_data = block_on(async move {
            let balance = query_usdt_balance(owner_address, usdt_address).await?;
            println!("USDT Balance: {}", balance);

            // Create a response object
            let response = UsdtBalanceResponse {
                address: input.to_string(),
                balance: balance.to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| e.to_string())?
                    .as_secs(),
            };

            serde_json::to_vec(&response).map_err(|e| e.to_string())
        })?;

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &balance_data)),
            Destination::CliOutput => Some(balance_data),
        };
        Ok(output)
    }
}

async fn query_usdt_balance(owner_address: Address, usdt_address: Address) -> Result<U256, String> {
    // Create Ethereum provider - this function already returns a provider directly, not a Result
    let provider = new_eth_provider::<Ethereum>(RPC_URL.to_string());

    // Prepare the contract call
    let balance_call = IERC20::balanceOfCall { owner: owner_address };

    // Construct the transaction request for a read-only call
    let tx = TransactionRequest {
        to: Some(TxKind::Call(usdt_address)),
        input: TransactionInput { input: Some(balance_call.abi_encode().into()), data: None },
        ..Default::default()
    };

    // Execute the call
    let result_bytes =
        provider.call(&tx).await.map_err(|e| format!("Provider call failed: {}", e))?;

    // Decode the result (balanceOf returns uint256)
    if result_bytes.len() != 32 {
        return Err(format!("Unexpected result length: {}", result_bytes.len()));
    }
    let balance = U256::from_be_slice(&result_bytes);

    Ok(balance)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsdtBalanceResponse {
    address: String,
    balance: String,
    timestamp: u64,
}
