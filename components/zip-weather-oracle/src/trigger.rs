use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent};
use alloy_sol_types::{sol, SolValue};
use wavs_wasi_chain::ethereum::alloy_primitives::Bytes;

pub(crate) mod solidity {
    use alloy_sol_macro::sol;
    pub use ITypes::*;

    sol!("../../src/interfaces/ITypes.sol");
}

// Define output for weather data
sol! {
    struct WeatherResult {
        uint64 triggerId;
        string location;
        string description;
        int16 temperature;  // Temperature in Celsius * 10 (to handle decimals as integers)
        uint16 humidity;    // Humidity percentage
        uint16 pressure;    // Pressure in hPa
    }
}

#[derive(Debug)]
pub enum Destination {
    Ethereum,
    CliOutput,
}

// Decodes trigger event and returns (triggerId, trigger_data, destination)
pub fn decode_trigger_event(
    trigger_data: TriggerData,
) -> Result<(u64, Vec<u8>, Destination), String> {
    match trigger_data {
        // Handle on-chain event
        TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
            // For on-chain events, we use the full Solidity interface
            use wavs_wasi_chain::decode_event_log_data;
            let event: solidity::NewTrigger = decode_event_log_data!(log)
                .map_err(|e| format!("Failed to decode event data: {}", e))?;

            // In a real component, we would need to properly decode the _triggerInfo bytes
            // This is simplified for now:
            Ok((0, event._triggerInfo.to_vec(), Destination::Ethereum))
        }
        // Handle raw data (used in testing with wasi-exec)
        TriggerData::Raw(data) => {
            // For testing (wasi-exec), we just return the raw data
            Ok((0, data.to_vec(), Destination::CliOutput))
        }
        _ => Err("Unsupported trigger type".to_string()),
    }
}

// Encodes data for return based on destination
pub fn encode_data_for_destination(
    trigger_id: u64,
    weather_data: WeatherResult,
    destination: Destination,
) -> Result<Vec<u8>, String> {
    match destination {
        Destination::Ethereum => {
            // For on-chain, encode using Solidity DataWithId format
            use solidity::DataWithId;
            let data = DataWithId {
                triggerId: trigger_id,
                data: weather_data.abi_encode().into(), // Convert Vec<u8> to Bytes
            };
            Ok(data.abi_encode())
        }
        Destination::CliOutput => {
            // For CLI output, just return the JSON as bytes
            let json = serde_json::to_string(&serde_json::json!({
                "location": weather_data.location,
                "description": weather_data.description,
                "temperature": weather_data.temperature as f32 / 10.0, // Convert back to decimal
                "humidity": weather_data.humidity,
                "pressure": weather_data.pressure,
            }))
            .map_err(|e| format!("Failed to encode JSON: {}", e))?;

            Ok(json.into_bytes())
        }
    }
}
