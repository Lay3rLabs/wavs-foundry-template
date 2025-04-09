use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent};
use crate::bindings::{export, Guest, TriggerAction};
use alloy_sol_types::SolValue;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use wavs_wasi_chain::decode_event_log_data;
use wavs_wasi_chain::http::{fetch_json, http_request_get};
use wstd::runtime::block_on;

pub mod bindings;

/// Represents the destination where the trigger output should be sent
pub enum Destination {
    Ethereum,
    CliOutput,
}

/// Decodes incoming trigger event data into its components
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
fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> Vec<u8> {
    solidity::DataWithId { triggerId: trigger_id, data: output.as_ref().to_vec().into() }
        .abi_encode()
}

mod solidity {
    use alloy_sol_macro::sol;
    pub use ITypes::*;
    sol!("../../src/interfaces/ITypes.sol");
}

struct Component;
export!(Component with_types_in bindings);

#[derive(Debug, Serialize, Deserialize)]
struct Brewery {
    id: String,
    name: String,
    brewery_type: String,
    street: Option<String>,
    city: String,
    state: String,
    postal_code: String,
    country: String,
    phone: Option<String>,
    website_url: Option<String>,
}

/// Fetches breweries for a given city from the OpenBreweryDB API.
///
/// Note on Data Size Limits:
/// - Ethereum has a max transaction data size of ~24KB
/// - For gas efficiency, responses should ideally be under 5KB
/// - We limit to 5 breweries to stay within these constraints
/// - Each additional byte costs extra gas (16 gas for non-zero bytes)
///
/// This implementation uses pagination (per_page=5) to ensure
/// responses stay within reasonable size limits for blockchain storage.
async fn get_breweries_by_city(city: &str) -> Result<Vec<Brewery>, String> {
    // Manually percent-encode the city name
    let encoded_city =
        city.to_lowercase()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u8)
                }
            })
            .collect::<String>();

    // Add per_page parameter to limit results to first 5 breweries
    let url =
        format!("https://api.openbrewerydb.org/v1/breweries?by_city={}&per_page=5", encoded_city);

    println!("Fetching from URL: {}", url);
    let req = http_request_get(&url).map_err(|e| e.to_string())?;
    let breweries: Vec<Brewery> = fetch_json(req).await.map_err(|e| e.to_string())?;

    println!("Limiting response to first 5 breweries for size constraints");
    Ok(breweries)
}

impl Guest for Component {
    /// Main entry point for the brewery finder component.
    /// Takes a city name as input and returns a list of breweries in that city.
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to string
        let input = std::str::from_utf8(&req).map_err(|e| e.to_string())?;

        // When using COIN_MARKET_CAP_ID, the input will be a single hex digit
        // We'll just pass through the raw input since it's already the city name
        let city = input.trim().to_string(); // Add trim() to remove any whitespace
        println!("Searching breweries in city: {}", city);

        // Fetch brewery data
        let result = block_on(async move {
            let breweries = get_breweries_by_city(&city).await?;
            println!("Found {} breweries", breweries.len());

            // Convert to JSON string and then to bytes
            serde_json::to_vec(&breweries).map_err(|e| e.to_string())
        })?;

        // Return the result based on destination
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &result)),
            Destination::CliOutput => Some(result),
        };
        Ok(output)
    }
}
