mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to string and parse as u64
        let input = std::str::from_utf8(&req).map_err(|e| e.to_string())?;
        println!("Raw input bytes: {:?}", req);
        println!("Input string: {:?}", input);
        println!("Input string length: {}", input.len());

        // Trim null bytes and parse the input as a number
        let trimmed_input = input.trim_matches('\0');
        println!("Trimmed input: {:?}", trimmed_input);

        let number =
            trimmed_input.parse::<u64>().map_err(|e| format!("Failed to parse number: {}", e))?;

        // Square the number
        let result = number * number;
        println!("squared result: {}", result);

        // Create result structure
        let computation_result = ComputationResult { input: number, result };

        // Serialize result to JSON
        let res = serde_json::to_vec(&computation_result).map_err(|e| e.to_string())?;

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &res)),
            Destination::CliOutput => Some(res),
        };
        Ok(output)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputationResult {
    input: u64,
    result: u64,
}
