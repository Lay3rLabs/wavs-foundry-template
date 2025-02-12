// Helpers to work with "trigger id" flows - which our example components do
use crate::bindings::wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent};
use alloy_sol_types::SolValue;
use anyhow::Result;

pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>)> {
    match trigger_data {
        TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
            let event: solidity::NewTrigger = wavs_wasi_chain::decode_event_log_data!(log)?;
            let trigger_info = solidity::TriggerInfo::abi_decode(&event._0, false)?;
            Ok((trigger_info.triggerId, trigger_info.data.to_vec()))
        }
        TriggerData::Raw(data) => {
            let trigger_info = solidity::TriggerInfo::abi_decode(&data, false)?;
            Ok((trigger_info.triggerId, data.to_vec()))
        }
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
