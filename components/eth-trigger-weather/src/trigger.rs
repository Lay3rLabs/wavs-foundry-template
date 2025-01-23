// Helpers to work with "trigger id" flows - which our example components do
use alloy_sol_types::SolValue;
use anyhow::Result;
use example_submit::ISimpleSubmit::DataWithId;
use example_trigger::{NewTrigger, TriggerInfo};
use layer_wasi::{
    bindings::compat::{TriggerData, TriggerDataEthContractEvent},
    ethereum::WasiProvider,
};

pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>)> {
    match trigger_data {
        TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
            let event: NewTrigger = layer_wasi::ethereum::decode_event_log_data(log)?;
            let trigger_info = TriggerInfo::abi_decode(&event._0, false)?;
            Ok((trigger_info.triggerId, trigger_info.data.to_vec()))
        }
        TriggerData::Raw(data) => {
            let trigger_info = TriggerInfo::abi_decode(&data, false)?;
            Ok((trigger_info.triggerId, data.to_vec()))
        }
        _ => Err(anyhow::anyhow!("Unsupported trigger data type")),
    }
}

pub fn encode_trigger_output(trigger_id: u64, output: impl AsRef<[u8]>) -> Vec<u8> {
    DataWithId { triggerId: trigger_id, data: output.as_ref().to_vec().into() }.abi_encode()
}

mod example_trigger {
    use alloy_sol_macro::sol;
    pub use ISimpleTrigger::TriggerInfo;
    pub use SimpleTrigger::NewTrigger;

    sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        SimpleTrigger,
        "../../out/WavsTrigger.sol/SimpleTrigger.json"
    );
}

mod example_submit {
    use alloy_sol_macro::sol;
    pub use ISimpleSubmit::DataWithId;

    sol!(
        #[allow(missing_docs)]
        ISimpleSubmit,
        "../../out/ISimpleSubmit.sol/ISimpleSubmit.json"
    );
}
