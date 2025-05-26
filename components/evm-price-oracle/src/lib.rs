mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use wavs_wasi_utils::http::{fetch_json, http_request_get};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};
use serde::{Deserialize, Serialize};
use wstd::{http::HeaderValue, runtime::block_on};

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
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
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
            Destination::CliOutput => Some(WasmResponse { payload: res.into(), ordering: None }),
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

    // round to the nearest 3 decimal places
    let price = (json.data.statistics.price * 100.0).round() / 100.0;
    // timestamp is 2025-04-30T19:59:44.161Z, becomes 2025-04-30T19:59:44
    let timestamp = json.status.timestamp.split('.').next().unwrap_or("");

    Ok(PriceFeedData { symbol: json.data.symbol, price, timestamp: timestamp.to_string() })
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
