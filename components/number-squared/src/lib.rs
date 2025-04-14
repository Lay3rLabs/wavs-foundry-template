mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};

struct Component;
export!(Component with_types_in bindings);

#[derive(Debug, Serialize, Deserialize)]
pub struct SquaredResult {
    original: i64,
    squared: i64,
}

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        // Decode the incoming trigger data
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to string and parse as a number
        let input_str = std::str::from_utf8(&req).map_err(|e| e.to_string())?;
        println!("Input received: {}", input_str);

        // Parse the input as a number
        let number =
            input_str.trim().parse::<i64>().map_err(|e| format!("Invalid number: {}", e))?;

        // Square the number
        let squared = number * number;

        println!("Original: {}, Squared: {}", number, squared);

        // Create a result object
        let result = SquaredResult { original: number, squared };

        // Serialize the result to JSON
        let result_json = serde_json::to_vec(&result).map_err(|e| e.to_string())?;

        // Return the result based on the destination
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &result_json)),
            Destination::CliOutput => Some(result_json),
        };

        Ok(output)
    }
}
