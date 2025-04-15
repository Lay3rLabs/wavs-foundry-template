# WAVS Development Guide

It's important that you read this guide in its entirety before creating a component.

## Overview

WAVS off-chain services:
1. Get triggered by on-chain events
2. Perform off-chain computation/data fetching
3. Submit results back to the blockchain

this template includes a sample price oracle:
- Fetches prices from CoinMarketCap based on coin IDs
- Stores the data on-chain for other contracts to use

## Components of a WAVS Service

1. **Rust Component** (off-chain logic)
   - Receives trigger data
   - Performs business logic (API calls, computation)
   - Returns processed results

2. **Smart Contracts**
   - Trigger Contract: Emits events that activate your service
   - Submission Contract: Receives and verifies your component's output

## Component structure:

```
components/your-component-name/
├── Cargo.toml           # Dependencies
├── src/
    ├── lib.rs           # Main logic
    ├── trigger.rs       # Trigger handling
    ├── bindings.rs      # Auto-generated during make wasi-build
```

## Build a service

### 1. Read the following files

1. /makefile
2. files in /components/eth-price-oracle/

### 2. Cargo.toml

This file defines component dependencies:

```toml
[package]
name = "your-component-name"
edition.workspace = true
version.workspace = true
authors.workspace = true
rust-version.workspace = true
repository.workspace = true

[dependencies]
wit-bindgen-rt = {workspace = true}
wavs-wasi-chain = { workspace = true }  # HTTP requests and chain interactions
serde = { workspace = true }            # Serialization
serde_json = { workspace = true }       # JSON handling
alloy-sol-macro = { workspace = true }  # Solidity interface generation
wstd = { workspace = true }             # Async runtime
alloy-sol-types = { workspace = true }  # ABI encoding/decoding
anyhow = { workspace = true }           # Error handling

[lib]
crate-type = ["cdylib"]

[package.metadata.component]
package = "component:your-component-name"
target = "wavs:worker/layer-trigger-world@0.3.0"
```

### 3. trigger.rs

Handles decoding incoming trigger data and encoding outgoing results:

```rust
use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent};
use alloy_sol_types::SolValue;
use anyhow::Result;
use wavs_wasi_chain::decode_event_log_data;

pub enum Destination {
    Ethereum,
    CliOutput,
}

pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>, Destination)> {
    match trigger_data {
        TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
            let event: solidity::NewTrigger = decode_event_log_data!(log)?;
            let trigger_info = solidity::TriggerInfo::abi_decode(&event._triggerInfo, false)?;
            Ok((trigger_info.triggerId, trigger_info.data.to_vec(), Destination::Ethereum))
        }
        TriggerData::Raw(data) => Ok((0, data.clone(), Destination::CliOutput)),
        _ => Err(anyhow::anyhow!("Unsupported trigger data type")),
    }
}

pub fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> Vec<u8> {
    solidity::DataWithId { triggerId: trigger_id, data: output.as_ref().to_vec().into() }
        .abi_encode()
}

mod solidity {
    use alloy_sol_macro::sol;
    pub use ITypes::*;

    sol!("../../src/interfaces/ITypes.sol");
}
```

### 4. lib.rs

Main component file with business logic:

```rust
mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use wavs_wasi_chain::http::{fetch_json, http_request_get};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};
use wstd::{http::HeaderValue, runtime::block_on};

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        // 1. Decode incoming trigger data
        let (trigger_id, input_data, dest) = 
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // 2. Process the data (your business logic here)
        // For example, parsing input and calling an API
        let input_str = std::str::from_utf8(&input_data).map_err(|e| e.to_string())?;
        println!("Processing input: {}", input_str);
        
        // 3. Your business logic (for example, making an API call)
        let result = block_on(async move {
            // Replace with your own API call
            let response = make_api_call(input_str).await?;
            serde_json::to_vec(&response).map_err(|e| e.to_string())
        })?;

        // 4. Return the result in appropriate format based on destination
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &result)),
            Destination::CliOutput => Some(result),
        };
        
        Ok(output)
    }
}

// Example async function for making API calls
async fn make_api_call(param: &str) -> Result<YourResponseType, String> {
    let url = format!("https://api.example.com/endpoint?param={}", param);
    
    let mut req = http_request_get(&url).map_err(|e| e.to_string())?;
    req.headers_mut().insert("Accept", HeaderValue::from_static("application/json"));
    
    let response: YourResponseType = fetch_json(req).await.map_err(|e| e.to_string())?;
    Ok(response)
}

// Define your response type with serde for JSON serialization
#[derive(Debug, Serialize, Deserialize)]
struct YourResponseType {
    // Your fields here
    value: String,
    timestamp: String,
}
```

Replace the `make_api_call` function with your own business logic!

## Build and Test

### 1. Build

```bash
make wasi-build
```
- Compiles all components in the `components/` directory
- Creates bindings.rs automatically
- builds `.wasm` files in the `compiled/` directory: "components/number-squarer/" becomes "compiled/number_squarer.wasm".

### Step 2: Test

```bash
# Test with a specific input parameter (in this example, "1" is the input)
COIN_MARKET_CAP_ID=1 make wasi-exec COMPONENT_FILENAME=your_component_name.wasm
```

1. `COMPONENT_FILENAME` must match the compiled filename (number_squarer.wasm)
2. The component file must be in the `compiled/` directory or specified in the Makefile

This command:
1. Uses Docker to run your component in a simulated environment
2. Passes the input value to your component via the Makefile
3. Shows the output of your component's processing

You don't need to deploy anything on-chain for this test!

## Debugging Tips

1. Use `println!()` statements in your code for local debugging:
   ```rust
   println!("Debug: input value is {}", input_value);
   ```

2. You can modify the Makefile to pass different inputs:
   ```bash
   # Test with different values
   COIN_MARKET_CAP_ID=2 make wasi-exec  # Ethereum
   COIN_MARKET_CAP_ID=1027 make wasi-exec  # Bitcoin
   ```

3. Inspect the component structure in the terminal:
   ```bash
   ls -la ./components/your-component-name/src/
   ```

## Deployment Workflow

Once your component is working locally:

1. Deploy your smart contracts:
   ```bash
   export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy`
   forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast
   ```

2. Deploy your service to WAVS:
   ```bash
   COMPONENT_FILENAME=your_component.wasm TRIGGER_EVENT="YourEvent(bytes)" make deploy-service
   ```

3. Trigger your service with test data:
   ```bash
   export YOUR_INPUT_VALUE=1
   export SERVICE_TRIGGER_ADDR=`make get-trigger-from-deploy`
   forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${YOUR_INPUT_VALUE} --sig "run(string,string)" --rpc-url http://localhost:8545 --broadcast
   ```

4. View the result:
   ```bash
   make show-result
   ```

## Input Handling and Testing

### Understanding Input Processing

When testing components with `make wasi-exec`, the input is processed as follows:

```makefile
## found in the makefile. Modify input format as needed for your component.
wasi-exec:
	@$(WAVS_CMD) exec --log-level=info --data /data/.docker --home /data \
	--component "/data/compiled/${COMPONENT_FILENAME}" \
	--service-config $(SERVICE_CONFIG) \
	--input `cast format-bytes32-string $(COIN_MARKET_CAP_ID)`
```
1. The input value is provided via the `COIN_MARKET_CAP_ID` environment variable
2. The Makefile passes this through `cast format-bytes32-string` before sending to your component
3. Your component receives the formatted bytes as input data

This process can introduce formatting quirks that your component must handle, including:
- Newline characters
- Non-printable characters
- Padding or special formatting

### Robust Input Parsing

For components that expect numeric inputs, implement robust parsing like this:

```rust
// Parse input, handling potential non-digit characters
let input_str = std::str::from_utf8(&input_data).map_err(|e| e.to_string())?;
println!("Raw input: {}", input_str);

// Extract digits from the string to handle formatted inputs
let digits: String = input_str.chars().filter(|c| c.is_digit(10)).collect();
println!("Filtered to digits: {}", digits);

// Parse the filtered string or fall back to original
let input_number: i64 = if !digits.is_empty() {
    digits.parse().map_err(|e| format!("Failed to parse digits as number: {}", e))?
} else {
    input_str.trim().parse().map_err(|e| format!("Failed to parse input as number: {}", e))?
};
```

### Testing Commands

To test your component correctly:

```bash
# The correct format for testing (replace with your component's filename)
COIN_MARKET_CAP_ID=5 make wasi-exec COMPONENT_FILENAME=your_component.wasm

# Make sure to use the .wasm extension and underscore format in the filename
# Example: components/my-component → my_component.wasm
```

## Common Issues and Solutions

- **Component doesn't build**: Make sure you have the correct Rust and cargo-component versions
- **Component crashes during test**: Check your error handling and debug with println!() statements
- **Input parsing errors**: Add filtering for expected input types (numbers, strings, etc.)
- **HTTP requests fail**: Verify network connectivity and URL format
- **Filename mismatch**: Ensure your COMPONENT_FILENAME uses underscores (not hyphens) and the .wasm extension
- **Unused imports warnings**: For components that don't use HTTP or async, remove unnecessary imports

## Environment Variables


### Public Variables (`kv`)
- Used for non-sensitive configuration
- Set in the Makefile's `SERVICE_CONFIG` variable
- Example:
```bash
SERVICE_CONFIG ?= '{"fuel_limit":100000000,"max_gas":5000000,"host_envs":[],"kv":[["max_retries","3"],["timeout_seconds","30"],["api_endpoint","https://api.example.com"]],"workflow_id":"default","component_id":"default"}'
```
- Access in component: `std::env::var("max_retries")?`

### Private Variables (`host_envs`)

- Used for sensitive data like API keys. NEVER PUT AN API KEY DIRECTLY IN A COMPONENT.
- Must be prefixed with `WAVS_ENV_`
- Set in `.env` file:
```bash
WAVS_ENV_MY_API_KEY=your_secret_key_here
```
- Add to `SERVICE_CONFIG`:
```bash
SERVICE_CONFIG ?= '{"fuel_limit":100000000,"max_gas":5000000,"host_envs":["WAVS_ENV_MY_API_KEY"],"kv":[],"workflow_id":"default","component_id":"default"}'
```
- Access in component: `std::env::var("WAVS_ENV_MY_API_KEY")?`

## Network Requests

Components can make HTTP requests using the `wavs-wasi-chain` crate. Since WASI components run synchronously but network requests are async, use `block_on` from `wstd` to bridge this gap.

Required dependencies in `Cargo.toml`:
```toml
# make sure these are in your toml file when working with network requests.
[dependencies]
wavs-wasi-chain = { workspace = true }  # HTTP utilities
wstd = { workspace = true }             # Runtime utilities
serde = { workspace = true }            # Serialization
serde_json = { workspace = true }       # JSON handling
```

Example GET request:
```rust
use wstd::runtime::block_on;

async fn make_request() -> Result<YourResponseType, String> {
    let url = "https://api.example.com/endpoint";
    let mut req = http_request_get(&url).map_err(|e| e.to_string())?;
    req.headers_mut().insert("Accept", HeaderValue::from_static("application/json"));
    let response: YourResponseType = fetch_json(req).await.map_err(|e| e.to_string())?;
    Ok(response)
}

// In your component:
fn process_data() -> Result<YourResponseType, String> {
    block_on(async move {
        make_request().await
    })?
}
```

Example POST request:
```rust
async fn make_post_request() -> Result<PostResponse, String> {
    let url = "https://api.example.com/endpoint";
    let post_data = ("key1", "value1");
    let response: PostResponse = fetch_json(
        http_request_post_json(&url, &post_data)?
    ).await.map_err(|e| e.to_string())?;
    Ok(response)
}
```

-  **Error Handling**:
   - Never assume an API request will succeed
   - Provide informative error messages or fallback functionality
   - Verify environment variables are properly loaded in the service config

-  **URL Handling**:
   - Be cautious with special characters in URLs (commas, spaces, etc.)
   - Properly encode URL parameters to avoid "invalid URI" errors
   - Follow best practices when writing in rust.

For more functions, see the [wavs-wasi-chain documentation](https://docs.rs/wavs-wasi-chain/latest/wavs_wasi_chain/all.html#functions).

## Input and Output Handling

WAVS components can receive input in two ways:

1. **On-chain Events**: Triggered by contract events after deployment
   - Component receives a `TriggerAction` containing event data
   - Use `decode_event_log_data!` macro to decode the event data

2. **Manual Testing**: Using `make wasi-exec` command
   - Simulates on-chain events
   - Passes data directly as `trigger::raw`
   - You may need to modify the makefile `--input` to format input for your component correctly.

### Data Processing Pattern

```rust
// 1. Define Solidity types using sol! macro
sol! {
    event MyEvent(uint64 indexed triggerId, bytes data);
    struct MyResult {
        uint64 triggerId;
        bytes processedData;
    }
}

// 2. Handle both trigger types
impl Guest for Component {
    fn run(action: TriggerAction) -> Result<Option<Vec<u8>>, String> {
        match action.data {
            // On-chain event handling
            TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                let event: MyEvent = decode_event_log_data!(log)?;
                let result = MyResult {
                    triggerId: event.triggerId,
                    processedData: process_data(&event.data)?,
                };
                Ok(Some(result.abi_encode()))
            }
            // Manual trigger handling
            TriggerData::Raw(data) => {
                let result = process_data(&data)?;
                Ok(Some(result))
            }
            _ => Err("Unsupported trigger type".to_string())
        }
    }
}
```

The template uses a `Destination` enum in `trigger.rs` to determine how to process and return data:
- `Destination::Ethereum` for on-chain events
- `Destination::CliOutput` for testing

## Helpers and Utilities

### `wavs-wasi-chain` Crate

The `wavs-wasi-chain` crate provides essential functions for:
- Making HTTP requests
- Interacting with blockchains
- Decoding trigger data
- Handling Ethereum ABI encoding/decoding

Key functions include:
- `http_request_get` and `http_request_post_json` for HTTP requests
- `fetch_json` for JSON responses
- `decode_event_log_data!` macro for trigger decoding
- `new_eth_provider` for Ethereum interactions

## Blockchain Interactions

Interacting with blockchains requires specific dependencies and setup:

### Dependencies
```toml
[dependencies]
# Core WAVS blockchain functionality
wit-bindgen-rt = {workspace = true}    # Required for WASI bindings
wavs-wasi-chain = { workspace = true }  # HTTP utilities
# other dependencies..

# Alloy crates for Ethereum interaction
alloy-sol-types = { workspace = true }  # ABI handling & type generation
alloy-sol-macro = { workspace = true }  # sol! macro for interfaces
alloy-primitives = { workspace = true } # Core primitive types
alloy-network = "0.11.1"               # Network trait and types
alloy-provider = { version = "0.11.1", default-features = false, features = ["rpc-api"] }
alloy-rpc-types = "0.11.1"            # RPC type definitions
```

### Chain Configuration
Chain configs are defined in `wavs.toml`:
```toml
[chains.eth.local]
chain_id = "31337"
ws_endpoint = "ws://localhost:8545"
http_endpoint = "http://localhost:8545"
```

### Accessing Configuration
```rust
// Get chain config
let chain_config = host::get_eth_chain_config(&chain_name)?;

// Create provider
let provider: RootProvider<Ethereum> = new_eth_provider::<Ethereum>(
    chain_config.http_endpoint
        .context("http_endpoint is missing")?
)?;
```

### Example: Querying NFT Balance
```rust
sol! {
    interface IERC721 {
        function balanceOf(address owner) external view returns (uint256);
    }
}

async fn query_nft_ownership(owner: Address, contract: Address) -> Result<bool, String> {
    // 1. Get chain config
    let chain_config = get_eth_chain_config("eth.local")?;
    
    // 2. Create provider
    let provider = new_eth_provider::<Ethereum>(chain_config.http_endpoint?)?;
    
    // 3. Prepare contract call
    let balance_call = IERC721::balanceOfCall { owner };
    
    // 4. Construct transaction request
    let tx = TransactionRequest {
        to: Some(TxKind::Call(contract)),
        input: TransactionInput {
            input: Some(balance_call.abi_encode().into()),
            data: None
        },
        ..Default::default()
    };
    
    // 5. Execute call
    let result_bytes = provider.call(&tx).await?;
    
    // 6. Decode result
    let balance = U256::from_be_slice(&result_bytes);
    Ok(balance > U256::ZERO)
}
```
