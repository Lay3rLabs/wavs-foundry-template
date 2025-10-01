use crate::bindings::wavs::types::events::{TriggerData, TriggerDataEvmContractEvent};
use crate::bindings::WasmResponse;
use crate::solidity;
use alloy_sol_types::SolValue;
use anyhow::Result;
use wavs_wasi_utils::decode_event_log_data;

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
/// 1. EvmContractEvent - Decodes Ethereum event logs using the NewTrigger ABI
/// 2. Raw - Used for direct CLI testing with no encoding
pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>, Destination)> {
    match trigger_data {
        TriggerData::EvmContractEvent(TriggerDataEvmContractEvent { log, .. }) => {
            let event: solidity::NewTrigger = decode_event_log_data!(log.data)?;
            let trigger_info =
                <solidity::TriggerInfo as SolValue>::abi_decode(&event._triggerInfo)?;
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
pub fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> WasmResponse {
    WasmResponse {
        payload: solidity::DataWithId {
            triggerId: trigger_id,
            data: output.as_ref().to_vec().into(),
        }
        .abi_encode(),
        ordering: None,
    }
}
