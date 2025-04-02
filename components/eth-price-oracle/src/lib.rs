use wavs_wasi_chain::http::{fetch_json, http_request_get};
pub mod bindings;
use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent};
use crate::bindings::{export, Guest, TriggerAction};
use alloy_sol_types::SolValue;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use wavs_wasi_chain::decode_event_log_data;
use wstd::{http::HeaderValue, runtime::block_on};

/// Represents the destination where the trigger output should be sent
///
/// # Variants
/// - `Ethereum`: Output will be ABI encoded and sent to an Ethereum contract
/// - `CliOutput`: Raw output for local testing/debugging
/// Note: Cosmos destination is also possible but not implemented in this example
pub enum Destination {
    Ethereum,
    CliOutput,
}

/// Decodes incoming trigger event data into its components
///
/// # Arguments
/// * `trigger_data` - The raw trigger data received from WAVS
///
/// # Returns
/// A tuple containing:
/// * `u64` - Trigger ID for tracking the request
/// * `Vec<u8>` - The actual data payload
/// * `Destination` - Where the processed result should be sent
///
/// # Implementation Details
/// Handles two types of triggers:
/// 1. EthContractEvent - Decodes Ethereum event logs using the NewTrigger ABI
/// 2. Raw - Used for direct CLI testing with no encoding
fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>, Destination)> {
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

/// Encodes the output data for submission back to Ethereum
///
/// # Arguments
/// * `trigger_id` - The ID of the original trigger request
/// * `output` - The data to be encoded, must implement AsRef<[u8]>
///
/// # Returns
/// ABI encoded bytes ready for submission to Ethereum
fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> Vec<u8> {
    solidity::DataWithId { triggerId: trigger_id, data: output.as_ref().to_vec().into() }
        .abi_encode()
}

/// Private module containing Solidity type definitions
///
/// The `sol!` macro from alloy_sol_macro reads a Solidity interface file
/// and generates corresponding Rust types and encoding/decoding functions.
///
/// In this case, it reads "../../src/interfaces/ITypes.sol" which defines:
/// - NewTrigger event
/// - TriggerInfo struct
/// - DataWithId struct
///
/// Documentation:
/// - <https://docs.rs/alloy-sol-macro/latest/alloy_sol_macro/macro.sol.html>
/// (You can also just sol! arbitrary solidity types like `event` or `struct` too)
mod solidity {
    use alloy_sol_macro::sol;
    pub use ITypes::*;

    // The objects here will be generated automatically into Rust types.
    // If you update the .sol file, you must re-run `cargo build` to see the changes.
    // or restart your editor / language server.
    sol!("../../src/interfaces/ITypes.sol");
}

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    /// Main entry point for the price oracle component.
    /// WAVS is subscribed to watch for events emitted by the blockchain.
    /// When WAVS observes an event is emitted, it will internally route the event and its data to this function (component).
    /// The processing then occurs before the output is returned back to WAVS to be submitted to the blockchain by the operator(s).
    ///
    /// This is why the `Destination::Ethereum` requires the encoded trigger output, it must be ABI encoded for the solidity contract.
    /// Failure to do so will result in a failed submission as the signature will not match the saved output.
    ///
    /// After the data is properly set by the operator through WAVS, any user can query the price data from the blockchain in the solidity contract.
    /// You can also return `None` as the output if nothing needs to be saved to the blockchain. (great for performing some off chain action)
    ///
    /// This function:
    /// 1. Receives a trigger action containing encoded data
    /// 2. Decodes the input to get a cryptocurrency ID (in hex)
    /// 3. Fetches current price data from CoinMarketCap
    /// 4. Returns the encoded response based on the destination
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to string and parse first char as u64
        let input = std::str::from_utf8(&req).map_err(|e| e.to_string())?;
        println!("input id: {}", input);

        let id = input.chars().next().ok_or("Empty input")?;
        let id = id.to_digit(16).ok_or("Invalid hex digit")? as u64;

        let res = block_on(async move {
            let resp_data = get_price_feed(id).await?;
            println!("resp_data: {:?}", resp_data);
            serde_json::to_vec(&resp_data).map_err(|e| e.to_string())
        })?;

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &res)),
            Destination::CliOutput => Some(res),
        };
        Ok(output)
    }
}

/// Fetches cryptocurrency price data from CoinMarketCap's API
///
/// # Arguments
/// * `id` - CoinMarketCap's unique identifier for the cryptocurrency
///
/// # Returns
/// * `PriceFeedData` containing:
///   - symbol: The cryptocurrency's ticker symbol (e.g., "BTC")
///   - price: Current price in USD
///   - timestamp: Server timestamp of the price data
///
/// # Implementation Details
/// - Uses CoinMarketCap's v3 API endpoint
/// - Includes necessary headers to avoid rate limiting:
///   * User-Agent to mimic a browser
///   * Random cookie with current timestamp
///   * JSON content type headers
///
/// As of writing (Mar 31, 2025), the CoinMarketCap API is free to use and has no rate limits.
/// This may change in the future so be aware of issues that you may encounter going forward.
/// There is a more proper API for pro users that you can use
/// - <https://coinmarketcap.com/api/documentation/v1/>
async fn get_price_feed(id: u64) -> Result<PriceFeedData, String> {
    let url = format!(
        "https://api.coinmarketcap.com/data-api/v3/cryptocurrency/detail?id={}&range=1h",
        id
    );

    let current_time = std::time::SystemTime::now().elapsed().unwrap().as_secs();

    let mut req = http_request_get(&url).map_err(|e| e.to_string())?;
    req.headers_mut().insert("Accept", HeaderValue::from_static("application/json"));
    req.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
    req.headers_mut()
        .insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"));
    req.headers_mut().insert(
        "Cookie",
        HeaderValue::from_str(&format!("myrandom_cookie={}", current_time)).unwrap(),
    );

    let json: Root = fetch_json(req).await.map_err(|e| e.to_string())?;

    Ok(PriceFeedData {
        symbol: json.data.symbol,
        price: json.data.statistics.price,
        timestamp: json.status.timestamp,
    })
}

/// Represents the price feed response data structure
/// This is the simplified version of the data that will be sent to the blockchain
/// via the Submission of the operator(s).
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceFeedData {
    symbol: String,
    timestamp: String,
    price: f64,
}

/// Root response structure from CoinMarketCap API
/// Generated from the API response using <https://transform.tools/json-to-rust-serde>
/// Contains detailed cryptocurrency information including price statistics
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub data: Data,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub id: f64,
    pub name: String,
    pub symbol: String,
    pub statistics: Statistics,
    pub description: String,
    pub category: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub price: f64,
    #[serde(rename = "totalSupply")]
    pub total_supply: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoinBitesVideo {
    pub id: String,
    pub category: String,
    #[serde(rename = "videoUrl")]
    pub video_url: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "previewImage")]
    pub preview_image: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: String,
    pub error_message: String,
    pub elapsed: String,
    pub credit_count: f64,
}
