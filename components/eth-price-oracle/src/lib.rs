mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};
use wstd::{
    http::{Client, Request},
    io::{empty, AsyncRead},
    runtime::block_on,
};

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Vec<u8>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to string and parse first char as u64
        let input = std::str::from_utf8(&req).map_err(|e| e.to_string())?;
        println!("input id: {}", input);

        let id = input.chars().next().ok_or("Empty input")?;
        let id = id.to_digit(16).ok_or("Invalid hex digit")? as u64;

        let res = block_on(async move {
            let resp_data = get_price_feed(id).await;
            println!("resp_data: {:?}", resp_data);

            match resp_data {
                Ok(resp) => {
                    let output: Vec<u8> = serde_json::to_vec(&resp).unwrap();
                    Ok(output)
                }
                Err(e) => Err(e),
            }
        });

        match res {
            Ok(data) => match dest {
                Destination::Ethereum => Ok(encode_trigger_output(trigger_id, &data)),
                Destination::CliOutput => Ok(data),
            },
            Err(e) => Err(e),
        }
    }
}

async fn get_price_feed(id: u64) -> Result<PriceFeedData, String> {
    let url = format!(
        "https://api.coinmarketcap.com/data-api/v3/cryptocurrency/detail?id={}&range=1h",
        id
    );

    let current_time = std::time::SystemTime::now().elapsed().unwrap().as_secs();

    let req = Request::get(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36")
        .header("Cookie", format!("myrandom_cookie={}", current_time))
        .body(empty())
        .unwrap();

    let response = Client::new().send(req).await;

    // print out response
    println!("response: {:?}", response);

    match response {
        Ok(mut response) => {
            let mut body_buf = Vec::new();
            response.body_mut().read_to_end(&mut body_buf).await.unwrap();

            let resp = String::from_utf8_lossy(&body_buf);
            let json: Root = serde_json::from_str(format!(r#"{}"#, resp).as_str()).unwrap();

            return Ok(PriceFeedData {
                symbol: json.data.symbol,
                price: json.data.statistics.price,
                timestamp: json.status.timestamp,
            });
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceFeedData {
    symbol: String,
    timestamp: String,
    price: f64,
}

/// -----
/// https://transform.tools/json-to-rust-serde
/// Generated from https://api.coinmarketcap.com/data-api/v3/cryptocurrency/detail?id=1&range=1h
/// -----
///
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
