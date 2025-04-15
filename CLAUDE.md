# WAVS Development Guide

This guide will help you build your first WAVS (WebAssembly AVS) service using our template.

## Overview

WAVS allows you to build off-chain services that:
1. Get triggered by on-chain events
2. Perform off-chain computation/data fetching
3. Submit results back to the blockchain

Our template includes a sample cryptocurrency price oracle that:
- Fetches prices from CoinMarketCap based on coin IDs
- Stores the data on-chain for other contracts to use

## Components of a WAVS Service

A typical WAVS service consists of:

1. **Rust Component** (off-chain logic)
   - Receives trigger data
   - Performs business logic (API calls, computation)
   - Returns processed results

2. **Smart Contracts**
   - Trigger Contract: Emits events that activate your service
   - Submission Contract: Receives and verifies your component's output

## Building Your First Component

Let's start by understanding the component structure:

```
components/your-component-name/
├── Cargo.toml           # Dependencies
├── src/
    ├── lib.rs           # Main logic
    ├── trigger.rs       # Trigger handling
    ├── bindings.rs      # Auto-generated during build
```

### Step 1: Set up your Cargo.toml

The Cargo.toml file defines your component's dependencies:

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

### Step 2: Create your trigger.rs

This file handles decoding incoming trigger data and encoding outgoing results:

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

### Step 3: Implement your lib.rs

This is the main component file with your business logic:

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

## Building and Testing Your Component

### Step 1: Build the component

```bash
make wasi-build
# or use `make build` to build both Solidity and Rust components
```

The build process:
1. Compiles all components in the `components/` directory
2. Generates WASI bindings automatically
3. Places compiled `.wasm` files in the `compiled/` directory

### Step 2: Test your component locally

```bash
# Test with a specific input parameter (in this example, "1" is the input)
COIN_MARKET_CAP_ID=1 make wasi-exec
```

This command:
1. Uses Docker to run your component in a simulated environment
2. Passes the input value to your component
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

## Common Issues and Solutions

- **Component doesn't build**: Make sure you have the correct Rust and cargo-component versions
- **Component crashes during test**: Check your error handling and debug with println!() statements
- **HTTP requests fail**: Verify network connectivity and URL format

## Next Steps

1. Study the example eth-price-oracle component for a complete implementation
2. Modify it to create your own service
3. Create a new component from scratch for your specific use case

Good luck with your WAVS development!
