# WAVS Development Guide

This guide will help you build your first WAVS (WebAssembly AVS) service using our template.

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

## Next Steps

1. Study the example eth-price-oracle component for a complete implementation
2. Modify it to create your own service
3. Create a new component from scratch for your specific use case

Good luck with your WAVS development!

## Advanced

This section covers best practices and common pitfalls when developing more complex WAVS components.

### Working with Environment Variables

Environment variables may come with quotes or other unexpected formatting:

```rust
// Always trim quotes from environment variables, especially API keys
let api_key = std::env::var("WAVS_ENV_MY_API_KEY")
    .map_err(|_| "API key not found".to_string())?
    .trim_matches('"'); // Removes any surrounding quotes
```

### API Integration Best Practices

#### URL Construction

When constructing URLs for API calls:

```rust
// 1. Always use proper URL encoding for special characters
let url = format!(
    "https://api.example.com/endpoint?param1={}&param2={}",
    url_encode(param1), url_encode(param2)
);

// 2. For fixed special characters, use their encoded values directly
// Comma (,) → %2C, Space → %20, etc.
let url = format!(
    "https://api.example.com/locations?zip={}%2CUS", // %2C is encoded comma
    zip_code
);

// 3. Always log complete URLs to help with debugging
println!("API URL: {}", url);
```

#### Error Handling

Implement robust error handling for API calls:

```rust
// Detailed error context
let response = match fetch_json::<serde_json::Value>(req).await {
    Ok(json) => {
        println!("Received API response: {}", json);
        
        // Check for API-level errors in successful HTTP responses
        if let Some(error) = json.get("error") {
            return Err(format!("API returned error: {}", error));
        }
        
        json
    },
    Err(e) => return Err(format!("API request failed: {}", e)),
};

// Then parse into your type
let typed_response: MyResponseType = serde_json::from_value(response)
    .map_err(|e| format!("Failed to parse API response: {}", e))?;
```

#### HTTP Request Limitations

Important limitations when working with HTTP requests:

```rust
// INCORRECT: HTTP request objects cannot be cloned
let req = http_request_get(&url)?;
let response1 = fetch_json::<Value>(req.clone()).await?; // ❌ Will not compile

// CORRECT: Create new request objects for each API call
let req1 = http_request_get(&url)?;
let response1 = fetch_json::<Value>(req1).await?;

// If needed again:
let req2 = http_request_get(&url)?;
let response2 = fetch_json::<Value>(req2).await?;
```

### Multi-Step Processing Pattern

For components requiring multiple sequential operations:

```rust
async fn process_data(input: &str) -> Result<FinalOutput, String> {
    // Step 1: First operation (e.g., API call or data processing)
    println!("Starting step 1...");
    let intermediate_result = step_one(input).await?;
    println!("Step 1 completed: {:?}", intermediate_result);
    
    // Step 2: Second operation using results from step 1
    println!("Starting step 2...");
    let final_result = step_two(&intermediate_result).await?;
    println!("Step 2 completed: {:?}", final_result);
    
    Ok(final_result)
}
```
