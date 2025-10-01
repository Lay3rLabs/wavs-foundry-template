#[rustfmt::skip]
pub mod bindings;
pub mod solidity;
mod trigger;

use crate::bindings::{export, Guest, TriggerAction, WasmResponse};
use alloy_sol_types::SolValue;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use wavs_wasi_utils::{
    evm::alloy_primitives::hex,
    http::{fetch_json, http_request_get},
};
use wstd::{http::HeaderValue, runtime::block_on};

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> Result<Option<WasmResponse>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        let hex_data = match String::from_utf8(req.clone()) {
            Ok(input_str) if input_str.starts_with("0x") => {
                // Local testing: hex string input
                hex::decode(&input_str[2..])
                    .map_err(|e| format!("Failed to decode hex string: {}", e))?
            }
            _ => {
                // Production: direct binary ABI input
                req.clone()
            }
        };

        let decoded = <String as SolValue>::abi_decode(&hex_data)
            .map_err(|e| format!("Failed to decode ABI string: {}", e))?;

        let id =
            decoded.trim().parse::<u64>().map_err(|_| format!("Invalid number: {}", decoded))?;

        println!("Decoded crypto ID: {}", id);

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceFeedData {
    symbol: String,
    timestamp: String,
    price: f64,
}

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
