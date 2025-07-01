# WAVS Component Creation Guide

You specialize in creating WAVS (WASI AVS) components. Your task is to guide the creation of a new WAVS component based on the provided information and user input. Follow these steps carefully to ensure a well-structured, error-free component that passes all validation checks with zero fixes.

## Component Structure

A WAVS component needs:
1. `Cargo.toml` - Dependencies configuration
2. `src/lib.rs` - Component implementation logic goes here
3. `src/trigger.rs` - trigger handling logic
4. `src/bindings.rs` - Auto-generated, never edit
5. `Makefile` - Do not edit
6. `config.json` - Only edit the name

## Creating a Component

### 1. Cargo.toml Template

```toml
[package]
name = "your-component-name"
edition.workspace = true
version.workspace = true
authors.workspace = true
rust-version.workspace = true
repository.workspace = true

[dependencies]
# Core dependencies (always needed)
wit-bindgen-rt ={ workspace = true}
wavs-wasi-utils = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
alloy-sol-macro = { workspace = true }
wstd = { workspace = true }
alloy-sol-types = { workspace = true }
anyhow = { workspace = true }

# Add for blockchain interactions
alloy-primitives = { workspace = true }
alloy-provider = { workspace = true }
alloy-rpc-types = { workspace = true }
alloy-network = { workspace = true }
alloy-contract = { workspace = true }

[lib]
crate-type = ["cdylib"]

[profile.release]
codegen-units = 1
opt-level = "s"
debug = false
strip = true
lto = true

[package.metadata.component]
package = "component:your-component-name"
target = "wavs:worker/layer-trigger-world@0.4.0-beta.4"
```

CRITICAL: Never use direct version numbers - always use `{ workspace = true }`.
IMPORTANT! Always add your component to workspace members in the root Cargo.toml

### 2. Component Implementation (lib.rs)

#### Basic Structure

```rust
mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use wavs_wasi_utils::{
    evm::alloy_primitives::hex,
    http::{fetch_json, http_request_get},
};
pub mod bindings;  // Never edit bindings.rs!
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};
use alloy_sol_types::SolValue;
use serde::{Deserialize, Serialize};
use wstd::{http::HeaderValue, runtime::block_on};
use anyhow::Result;

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let (trigger_id, req, dest) = 
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;
            
        // Decode trigger data inline - handles hex string input
        let request_input = {
            // First, convert the input bytes to a string to check if it's a hex string
            let input_str = String::from_utf8(req.clone())
                .map_err(|e| format!("Input is not valid UTF-8: {}", e))?;

            // Check if it's a hex string (starts with "0x")
            let hex_data = if input_str.starts_with("0x") {
                // Decode the hex string to bytes
                hex::decode(&input_str[2..])
                    .map_err(|e| format!("Failed to decode hex string: {}", e))?
            } else {
                // If it's not a hex string, assume the input is already binary data
                req.clone()
            };

            // Now ABI decode the binary data as a string parameter
            <String as SolValue>::abi_decode(&hex_data)
                .map_err(|e| format!("Failed to decode input as ABI string: {}", e))?
        };
        println!("Decoded string input: {}", request_input);
            
        // Process the decoded data here
        let result = process_data(request_input)?;
        
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &result)),
            Destination::CliOutput => Some(WasmResponse { payload: result.into(), ordering: None }),
        };
        Ok(output)
    }
}

// Example processing function - replace with your actual logic
fn process_data(input: String) -> Result<Vec<u8>, String> {
    // Your processing logic here
    Ok(input.as_bytes().to_vec())
}
```

#### Trigger Event Handling (trigger.rs)

```rust
use crate::bindings::wavs::worker::layer_types::{
    TriggerData, TriggerDataEvmContractEvent, WasmResponse,
};
use alloy_sol_types::SolValue;
use anyhow::Result;
use wavs_wasi_utils::decode_event_log_data;

pub enum Destination {
    Ethereum,
    CliOutput,
}

pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>, Destination)> {
    match trigger_data {
        TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { log, .. }) => {
            let event: solidity::NewTrigger = decode_event_log_data!(log)?;
            let trigger_info = <solidity::TriggerInfo as SolValue>::abi_decode(&event._triggerInfo)?;
            Ok((trigger_info.triggerId, trigger_info.data.to_vec(), Destination::Ethereum))
        }
        TriggerData::Raw(data) => Ok((0, data.clone(), Destination::CliOutput)),
        _ => Err(anyhow::anyhow!("Unsupported trigger data type")),
    }
}

pub fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> WasmResponse {
    WasmResponse {
        payload: solidity::DataWithId {
            triggerId: trigger_id,
            data: output.as_ref().to_vec().into(),
        }
        .abi_encode(),
        ordering: None,
    }
}

pub mod solidity {
    use alloy_sol_macro::sol;
    pub use ITypes::*;
    sol!("../../src/interfaces/ITypes.sol");

    // trigger contract function that encodes string input
    sol! {
        function addTrigger(string data) external;
    }
}
```

## Critical Components

### 1. ABI Handling

NEVER use `String::from_utf8` on ABI-encoded data. This will ALWAYS fail with "invalid utf-8 sequence".

```rust
// WRONG - Will fail on ABI-encoded data
let input_string = String::from_utf8(abi_encoded_data)?;

// CORRECT - Use proper ABI decoding with hex string support
let request_input = {
    // First, convert the input bytes to a string to check if it's a hex string
    let input_str = String::from_utf8(req.clone())
        .map_err(|e| format!("Input is not valid UTF-8: {}", e))?;

    // Check if it's a hex string (starts with "0x")
    let hex_data = if input_str.starts_with("0x") {
        // Decode the hex string to bytes
        hex::decode(&input_str[2..])
            .map_err(|e| format!("Failed to decode hex string: {}", e))?
    } else {
        // If it's not a hex string, assume the input is already binary data
        req.clone()
    };

    // Now ABI decode the binary data as a string parameter
    <String as SolValue>::abi_decode(&hex_data)
        .map_err(|e| format!("Failed to decode input as ABI string: {}", e))?
};

// For numeric parameters, parse from the string
// Example: When you need a number but input is a string:
let number = request_input
    .trim()
    .parse::<u64>()
    .map_err(|_| format!("Invalid number: {}", request_input))?;

// SAFE - Only use String::from_utf8 on data that has already been decoded as a string
// Example: When handling Raw trigger data that was already decoded as a string
let input = std::str::from_utf8(&req).map_err(|e| e.to_string())?;
```

### 2. Data Structure Ownership

ALWAYS derive `Clone` for API response data structures. If fields may be missing, also use `Option<T>`, `#[serde(default)]`, and `Default`:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ResponseData {
    field1: Option<String>,
    field2: Option<u64>,
    // other fields
}
```

ALWAYS clone data before use to avoid ownership issues:

```rust
// WRONG – creates a temporary that is dropped immediately
let result = process_data(&data.clone());

// CORRECT – clone into a named variable
let data_clone = data.clone();
let result = process_data(&data_clone);
```


### 3. Network Requests

```rust
use wstd::runtime::block_on;
use wstd::http::HeaderValue;
use wavs_wasi_utils::http::{fetch_json, http_request_get, http_request_post_json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApiResponse {
    #[serde(default)]
    field1: Option<String>,
    #[serde(default)]
    field2: Option<u64>,
}

async fn make_request() -> Result<ApiResponse, String> {
    let url = format!("https://api.example.com/endpoint?param={}", param);
    
    let mut req = http_request_get(&url).map_err(|e| e.to_string())?;
    req.headers_mut().insert("Accept", HeaderValue::from_static("application/json"));
    req.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
    req.headers_mut().insert("User-Agent", HeaderValue::from_static("Mozilla/5.0"));
    
    let response: ApiResponse = fetch_json(req).await.map_err(|e| e.to_string())?;
    Ok(response)
}

fn process_data() -> Result<ApiResponse, String> {
    block_on(async move { make_request().await })
}

// For POST requests with JSON data, use http_request_post_json(url, &data) instead of http_request_get
```

### 4. Option/Result Handling

```rust
// WRONG - Option types don't have map_err
let config = get_evm_chain_config("ethereum").map_err(|e| e.to_string())?;

// CORRECT - For Option types, use ok_or_else()
let config = get_evm_chain_config("ethereum")
    .ok_or_else(|| "Failed to get chain config".to_string())?;

// CORRECT - For Result types, use map_err()
let balance = fetch_balance(address).await
    .map_err(|e| format!("Balance fetch failed: {}", e))?;
```

### 5. Blockchain Interactions

```rust
use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::TransactionInput;
use std::str::FromStr; // Required for parsing addresses
use crate::bindings::host::get_evm_chain_config;
use wavs_wasi_utils::evm::new_evm_provider;

async fn query_blockchain(address_str: &str) -> Result<ResponseData, String> {
    // Parse address
    let address = Address::from_str(address_str)
        .map_err(|e| format!("Invalid address format: {}", e))?;
    
    // Get chain configuration from environment
    let chain_config = get_evm_chain_config("ethereum")
        .ok_or_else(|| "Failed to get chain config".to_string())?;
    
    // Create provider
    let provider: RootProvider<Ethereum> = 
        new_evm_provider::<Ethereum>(chain_config.http_endpoint.unwrap());
    
    // Create contract call
    let contract_call = IERC20::balanceOfCall { owner: address };
    let tx = alloy_rpc_types::eth::TransactionRequest {
        to: Some(TxKind::Call(contract_address)),
        input: TransactionInput { 
            input: Some(contract_call.abi_encode().into()), 
            data: None 
        },
        ..Default::default()
    };
    
    // Execute call
    let result = provider.call(tx).await.map_err(|e| e.to_string())?;
    let balance: U256 = U256::from_be_slice(&result);
    
    Ok(ResponseData { /* your data here */ })
}
```

### 6. Numeric Type Handling

```rust
// WRONG - Using .into() for numeric conversions between types
let temp_uint: U256 = temperature.into(); // DON'T DO THIS

// CORRECT - String parsing method works reliably for all numeric types
let temperature: u128 = 29300;
let temperature_uint256 = temperature.to_string().parse::<U256>().unwrap();

// CORRECT - Always use explicit casts between numeric types
let decimals: u8 = decimals_u32 as u8;

// CORRECT - Handling token decimals correctly
let mut divisor = U256::from(1);
for _ in 0..decimals {
    divisor = divisor * U256::from(10);
}
let formatted_amount = amount / divisor;
```

## Component Examples by Task

Here are templates for common WAVS component tasks:

### 1. Token Balance Checker

```rust
// IMPORTS
use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::TransactionInput;
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wavs_wasi_utils::{
    evm::{alloy_primitives::hex, new_evm_provider},
};
use wstd::runtime::block_on;

pub mod bindings;
mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use crate::bindings::host::get_evm_chain_config;
use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEvmContractEvent};
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};

// TOKEN INTERFACE
sol! {
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint256);
        function decimals() external view returns (uint8);
    }
}

// FIXED CONTRACT ADDRESS
const TOKEN_CONTRACT_ADDRESS: &str = "0x..."; // Your token contract address

// RESPONSE STRUCTURE - MUST DERIVE CLONE
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalanceData {
    wallet: String,
    balance_raw: String,
    balance_formatted: String,
    token_contract: String,
}

// COMPONENT IMPLEMENTATION
struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let (trigger_id, req, dest) = 
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;
        
        // Decode trigger data inline - handles hex string input
        let wallet_address_str = {
            // First, convert the input bytes to a string to check if it's a hex string
            let input_str = String::from_utf8(req.clone())
                .map_err(|e| format!("Input is not valid UTF-8: {}", e))?;

            // Check if it's a hex string (starts with "0x")
            let hex_data = if input_str.starts_with("0x") {
                // Decode the hex string to bytes
                hex::decode(&input_str[2..])
                    .map_err(|e| format!("Failed to decode hex string: {}", e))?
            } else {
                // If it's not a hex string, assume the input is already binary data
                req.clone()
            };

            // Now ABI decode the binary data as a string parameter
            <String as SolValue>::abi_decode(&hex_data)
                .map_err(|e| format!("Failed to decode input as ABI string: {}", e))?
        };
        
        // Check token balance
        let res = block_on(async move {
            let balance_data = get_token_balance(&wallet_address_str).await?;
            serde_json::to_vec(&balance_data).map_err(|e| e.to_string())
        })?;
        
        // Return result based on destination
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &res)),
            Destination::CliOutput => Some(WasmResponse { payload: res.into(), ordering: None }),
        };
        Ok(output)
    }
}

// BALANCE CHECKER IMPLEMENTATION
async fn get_token_balance(wallet_address_str: &str) -> Result<TokenBalanceData, String> {
    // Parse wallet address
    let wallet_address = Address::from_str(wallet_address_str)
        .map_err(|e| format!("Invalid wallet address: {}", e))?;
    
    // Parse token contract address
    let token_address = Address::from_str(TOKEN_CONTRACT_ADDRESS)
        .map_err(|e| format!("Invalid token address: {}", e))?;
    
    // Get Ethereum provider
    let chain_config = get_evm_chain_config("ethereum")
        .ok_or_else(|| "Failed to get Ethereum chain config".to_string())?;
    
    let provider: RootProvider<Ethereum> = 
        new_evm_provider::<Ethereum>(chain_config.http_endpoint.unwrap());
    
    // Get token balance
    let balance_call = IERC20::balanceOfCall { owner: wallet_address };
    let tx = alloy_rpc_types::eth::TransactionRequest {
        to: Some(TxKind::Call(token_address)),
        input: TransactionInput { input: Some(balance_call.abi_encode().into()), data: None },
        ..Default::default()
    };
    
    let result = provider.call(tx).await.map_err(|e| e.to_string())?;
    let balance_raw: U256 = U256::from_be_slice(&result);
    
    // Get token decimals
    let decimals_call = IERC20::decimalsCall {};
    let tx_decimals = alloy_rpc_types::eth::TransactionRequest {
        to: Some(TxKind::Call(token_address)),
        input: TransactionInput { input: Some(decimals_call.abi_encode().into()), data: None },
        ..Default::default()
    };
    
    let result_decimals = provider.call(tx_decimals).await.map_err(|e| e.to_string())?;
    let decimals: u8 = result_decimals[31]; // Last byte for uint8
    
    // Format balance
    let formatted_balance = format_token_amount(balance_raw, decimals);
    
    // Return data
    Ok(TokenBalanceData {
        wallet: wallet_address_str.to_string(),
        balance_raw: balance_raw.to_string(),
        balance_formatted: formatted_balance,
        token_contract: TOKEN_CONTRACT_ADDRESS.to_string(),
    })
}
```

### 2. API Data Fetcher

Important: Always verify API endpoints using curl to examine their response structure before creating any code that relies on them.

```rust
// IMPORTS
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use wavs_wasi_utils::{
    evm::alloy_primitives::hex,
    http::{fetch_json, http_request_get},
};
use wstd::{http::HeaderValue, runtime::block_on};

pub mod bindings;
mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEvmContractEvent};
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};

// RESPONSE STRUCTURE - MUST DERIVE CLONE
// IMPORTANT: Always Use #[serde(default)] and Option<T> for fields from external APIs. They might be missing or inconsistent
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApiResponse {
    // Use Option<T> for fields that might be missing in some responses
    #[serde(default)] 
    field1: Option<String>,
    #[serde(default)]
    field2: Option<u64>,
    // other fields
}

// RESULT DATA STRUCTURE - MUST DERIVE CLONE
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultData {
    input_param: String,
    result: String,
}

// COMPONENT IMPLEMENTATION
struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        // Decode trigger data
        let (trigger_id, req, dest) = 
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;
        
        // Decode trigger data inline - handles hex string input
        let param = {
            // First, convert the input bytes to a string to check if it's a hex string
            let input_str = String::from_utf8(req.clone())
                .map_err(|e| format!("Input is not valid UTF-8: {}", e))?;

            // Check if it's a hex string (starts with "0x")
            let hex_data = if input_str.starts_with("0x") {
                // Decode the hex string to bytes
                hex::decode(&input_str[2..])
                    .map_err(|e| format!("Failed to decode hex string: {}", e))?
            } else {
                // If it's not a hex string, assume the input is already binary data
                req.clone()
            };

            // Now ABI decode the binary data as a string parameter
            <String as SolValue>::abi_decode(&hex_data)
                .map_err(|e| format!("Failed to decode input as ABI string: {}", e))?
        };
        
        // Make API request
        let res = block_on(async move {
            let api_data = fetch_api_data(&param).await?;
            serde_json::to_vec(&api_data).map_err(|e| e.to_string())
        })?;
        
        // Return result based on destination
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &res)),
            Destination::CliOutput => Some(WasmResponse { payload: res.into(), ordering: None }),
        };
        Ok(output)
    }
}

// API FETCHER IMPLEMENTATION
async fn fetch_api_data(param: &str) -> Result<ResultData, String> {
    // Get API key from environment (IMPORTANT! you must add this variable to your .env file. All private variables must be prefixed with WAVS_ENV)
    let api_key = std::env::var("WAVS_ENV_API_KEY")
        .map_err(|_| "Failed to get API_KEY from environment variables".to_string())?;
    
    // Create API URL
    let url = format!(
        "https://api.example.com/endpoint?param={}&apikey={}",
        param, api_key
    );
    
    // Create request with headers
    let mut req = http_request_get(&url)
        .map_err(|e| format!("Failed to create request: {}", e))?;
    
    req.headers_mut().insert("Accept", HeaderValue::from_static("application/json"));
    req.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
    req.headers_mut().insert("User-Agent", HeaderValue::from_static("Mozilla/5.0"));
    
    // Make API request
    let api_response: ApiResponse = fetch_json(req).await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;
    
    // Process and return data - handle Option fields safely
    let field1 = api_response.field1.unwrap_or_else(|| "unknown".to_string());
    let field2 = api_response.field2.unwrap_or(0);
    
    Ok(ResultData {
        input_param: param.to_string(),
        result: format!("{}: {}", field1, field2),
    })
}
```

### 3. NFT Ownership Checker

```rust
// IMPORTS
use alloy_network::Ethereum;
use alloy_primitives::{Address, TxKind, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_types::TransactionInput;
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wavs_wasi_utils::{
    evm::{alloy_primitives::hex, new_evm_provider},
};
use wstd::runtime::block_on;

pub mod bindings;
mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use crate::bindings::host::get_evm_chain_config;
use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEvmContractEvent};
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};

// NFT INTERFACE
sol! {
    interface IERC721 {
        function balanceOf(address owner) external view returns (uint256);
        function ownerOf(uint256 tokenId) external view returns (address);
    }
}

// FIXED CONTRACT ADDRESS
const NFT_CONTRACT_ADDRESS: &str = "0xbd3531da5cf5857e7cfaa92426877b022e612cf8"; // Bored Ape contract

// RESPONSE STRUCTURE - MUST DERIVE CLONE
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NftOwnershipData {
    wallet: String,
    owns_nft: bool,
    balance: String,
    nft_contract: String,
    contract_name: String,
}

// COMPONENT IMPLEMENTATION
struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        // Decode trigger data
        let (trigger_id, req, dest) = 
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;
        
        // Decode trigger data inline - handles hex string input
        let wallet_address_str = {
            // First, convert the input bytes to a string to check if it's a hex string
            let input_str = String::from_utf8(req.clone())
                .map_err(|e| format!("Input is not valid UTF-8: {}", e))?;

            // Check if it's a hex string (starts with "0x")
            let hex_data = if input_str.starts_with("0x") {
                // Decode the hex string to bytes
                hex::decode(&input_str[2..])
                    .map_err(|e| format!("Failed to decode hex string: {}", e))?
            } else {
                // If it's not a hex string, assume the input is already binary data
                req.clone()
            };

            // Now ABI decode the binary data as a string parameter
            <String as SolValue>::abi_decode(&hex_data)
                .map_err(|e| format!("Failed to decode input as ABI string: {}", e))?
        };
        
        // Check NFT ownership
        let res = block_on(async move {
            let ownership_data = check_nft_ownership(&wallet_address_str).await?;
            serde_json::to_vec(&ownership_data).map_err(|e| e.to_string())
        })?;
        
        // Return result based on destination
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &res)),
            Destination::CliOutput => Some(WasmResponse { payload: res.into(), ordering: None }),
        };
        Ok(output)
    }
}

// NFT OWNERSHIP CHECKER IMPLEMENTATION
async fn check_nft_ownership(wallet_address_str: &str) -> Result<NftOwnershipData, String> {
    // Parse wallet address
    let wallet_address = Address::from_str(wallet_address_str)
        .map_err(|e| format!("Invalid wallet address: {}", e))?;
    
    // Parse NFT contract address
    let nft_address = Address::from_str(NFT_CONTRACT_ADDRESS)
        .map_err(|e| format!("Invalid NFT contract address: {}", e))?;
    
    // Get Ethereum provider
    let chain_config = get_evm_chain_config("ethereum")
        .ok_or_else(|| "Failed to get Ethereum chain config".to_string())?;
    
    let provider: RootProvider<Ethereum> = 
        new_evm_provider::<Ethereum>(chain_config.http_endpoint.unwrap());
    
    // Check NFT balance
    let balance_call = IERC721::balanceOfCall { owner: wallet_address };
    let tx = alloy_rpc_types::eth::TransactionRequest {
        to: Some(TxKind::Call(nft_address)),
        input: TransactionInput { input: Some(balance_call.abi_encode().into()), data: None },
        ..Default::default()
    };
    
    let result = provider.call(tx).await.map_err(|e| e.to_string())?;
    let balance: U256 = U256::from_be_slice(&result);
    
    // Determine if wallet owns at least one NFT
    let owns_nft = balance > U256::ZERO;
    
    // Return data
    Ok(NftOwnershipData {
        wallet: wallet_address_str.to_string(),
        owns_nft,
        balance: balance.to_string(),
        nft_contract: NFT_CONTRACT_ADDRESS.to_string(),
        contract_name: "BAYC".to_string(),
    })
}
```


## Component Creation Process

### Phase 1: Planning

When you ask me to create a WAVS component, I'll follow this systematic process to ensure it works perfectly on the first try:

1. **Research Phase**: I'll review the files in /components/evm-price-oracle to see common forms.
2. I will read any and all documentation links given to me and research any APIs or services needed.
3. I'll read `/test_utils/validate_component.sh` to see what validation checks I need to pass.
4. I'll verify API response structures by using curl before implementing code that depends on them: `curl -s "my-endpoint"`.
5.  I'll create a file called plan.md with an overview of the component I will make. I'll do this before actually creating the lib.rs file. I'll write each item in the [checklist](#validation-checklist) and check them off as I plan my code, making sure my code complies to the checklist and /test_utils/validate_component.sh. Each item must be checked and verified. I will list out all imports I will need. I will include a basic flow chart or visual of how the component will work. I will put plan.md in a new folder with the name of the component (`your-component-name`) in the `/components` directory.


### Phase 2: Implementation

After being 100% certain that my idea for a component will work without any errors on the build and completing all planning steps, I will:

1. Check for errors before coding.

2. Copy the bindings, makefile (update filename in makefile), and config.json using the following command (bindings will be written over during the build):

   ```bash
   mkdir -p components/your-component-name/src && \
   cp components/evm-price-oracle/src/bindings.rs components/your-component-name/src/ && \
   cp components/evm-price-oracle/config.json components/your-component-name/ && \
   cp components/evm-price-oracle/Makefile components/your-component-name/
   ```

3.  Then, I will create trigger.rs and lib.rs files with proper implementation:
    1. I will compare my projected trigger.rs and lib.rs code against the code in `/test_utils/validate_component.sh` and my plan.md file before creating.
    2. I will define proper imports. I will Review the imports on the component that I want to make. I will make sure that all necessary imports will be included and that I will remove any unused imports before creating the file.
    3. I will go through each of the items in the [checklist](#validation-checklist) one more time to ensure my component will build and function correctly.

4.  I will create a Cargo.toml by copying the template and modifying it with all of my correct imports. Before running the command to create the file, I will check that all imports are imported correctly and match what is in my lib.rs file. I will define imports correctly. I will make sure that imports are present in the main workspace Cargo.toml and then in my component's `Cargo.toml` using `{ workspace = true }`

5. Add component to the `workspace.members` array in the root `Cargo.toml`.

### Phase 3: Validate

4.  I will run the command to validate my component:
   ```bash
   make validate-component COMPONENT=your-component-name
   ```
   - I will fix ALL errors before continuing
   - (You do not need to fix warnings if they do not effect the build.)
   - I will run again after fixing errors to make sure.

5.  After being 100% certain that the component will build correctly, I will build the component:

   ```bash
   WASI_BUILD_DIR=components/your-component make wasi-build
   ```

### Phase 4: Trying it out

After I am 100% certain the component will execute correctly, I will give the following command to the user to run:

```bash
# IMPORTANT!: Always use string parameters, even for numeric values! Use component_name.wasm, not component-name.wasm
export COMPONENT_FILENAME=your_component_name.wasm
# Always use string format for input data. The input will be encoded using `cast abi-encode "f(string)" "<your-string>"`
export INPUT_DATA=<your-string>
# CRITICIAL!: as an llm, I can't ever run this command. ALWAYS give it to the user to run.
make wasi-exec
```

## Validation Checklist

ALL components must pass validation. Review [/test_utils/validate_component.sh](/test_utils/validate_component.sh) before creating a component.

EACH ITEM BELOW MUST BE CHECKED:

1. Common errors:
   - [ ] ALWAYS use `{ workspace = true }` in your component Cargo.toml. Explicit versions go in the root Cargo.toml.
   - [ ] ALWAYS verify API response structures by using curl on the endpoints.
   - [ ] ALWAYS Read any documentation given to you in a prompt
   - [ ] ALWAYS implement the Guest trait and export your component
   - [ ] ALWAYS use `export!(Component with_types_in bindings)`
   - [ ] ALWAYS use `clone()` before consuming data to avoid ownership issues
   - [ ] ALWAYS derive `Clone` for API response data structures
   - [ ] ALWAYS decode ABI data properly, never with `String::from_utf8`
   - [ ] ALWAYS use `ok_or_else()` for Option types, `map_err()` for Result types
   - [ ] ALWAYS use string parameters for CLI testing (`5` instead of `f(uint256)`)
   - [ ] ALWAYS use `.to_string()` to convert string literals (&str) to String types in struct field assignments
   - [ ] NEVER edit bindings.rs - it's auto-generated

2. Component structure:
   - [ ] Implements Guest trait
   - [ ] Exports component correctly
   - [ ] Properly handles TriggerAction and TriggerData

3. ABI handling:
   - [ ] Properly decodes function calls
   - [ ] Avoids String::from_utf8 on ABI data

4. Data ownership:
   - [ ] All API structures derive Clone
   - [ ] Clones data before use
   - [ ] Avoids moving out of collections
   - [ ] Avoids all ownership issues and "Move out of index" errors

5. Error handling:
   - [ ] Uses ok_or_else() for Option types
   - [ ] Uses map_err() for Result types
   - [ ] Provides descriptive error messages

6. Imports:
   - [ ] Includes all required traits and types
   - [ ] Uses correct import paths
   - [ ] Properly imports SolCall for encoding
   - [ ] Each and every method and type is used properly and has the proper import
   - [ ] Both structs and their traits are imported
   - [ ] Verify all required imports are imported properly
   - [ ] All dependencies are in Cargo.toml with `{workspace = true}`
   - [ ] Any unused imports are removed

7. Component structure:
   - [ ] Uses proper sol! macro with correct syntax
   - [ ] Correctly defines Solidity types in solidity module
   - [ ] Implements required functions

8. Security:
   - [ ] No hardcoded API keys or secrets
   - [ ] Uses environment variables for sensitive data

9.  Dependencies:
   - [ ] Uses workspace dependencies correctly
   - [ ] Includes all required dependencies

10. Solidity types:
   - [ ] Properly imports sol macro
   - [ ] Uses solidity module correctly
   - [ ] Handles numeric conversions safely
   - [ ] Uses .to_string() for all string literals in struct initialization

11. Network requests:
    - [ ] Uses block_on for async functions
    - [ ] Uses fetch_json with correct headers
    - [ ] ALL API endpoints have been tested with curl and responses are handled correctly in my component.
    - [ ] IMPORTANT! Always use #[serde(default)] and Option<T> for fields from external APIs.

With this guide, you should be able to create any WAVS component that passes validation, builds without errors, and executes correctly.
