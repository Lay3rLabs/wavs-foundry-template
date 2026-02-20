#[allow(warnings)]
#[rustfmt::skip]
mod bindings;

mod gas_oracle;
mod utils;

use wavs_wasi_utils::impl_u128_conversions;

use crate::bindings::{
    export, host,
    wavs::{
        aggregator::aggregator::{EvmAddress, SubmitAction, TimerAction, U128},
        types::{core::Duration, service::Submit},
    },
    AggregatorAction, AnyTxHash, Guest, Packet,
};

impl_u128_conversions!(U128);

struct Component;

impl Guest for Component {
    fn process_packet(packet: Packet) -> Result<Vec<AggregatorAction>, String> {
        let timer_delay_secs = host::config_var("timer_delay_secs")
            .map(|delay_str| {
                delay_str.parse().map_err(|e| format!("Failed to parse timer_delay_secs: {e}"))
            })
            .transpose()?;

        match timer_delay_secs {
            Some(secs) => {
                // Use timer delay if specified
                let timer_action = TimerAction { delay: Duration { secs } };
                Ok(vec![AggregatorAction::Timer(timer_action)])
            }
            None => {
                // No timer delay - process immediately (skip tx validation)
                process_submission(packet, false)
            }
        }
    }

    fn handle_timer_callback(packet: Packet) -> Result<Vec<AggregatorAction>, String> {
        process_submission(packet, true)
    }

    fn handle_submit_callback(
        _packet: Packet,
        tx_result: Result<AnyTxHash, String>,
    ) -> Result<(), String> {
        match tx_result {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }
}

fn process_submission(packet: Packet, validate_tx: bool) -> Result<Vec<AggregatorAction>, String> {
    let workflow = host::get_workflow().workflow;

    let submit_config = match workflow.submit {
        Submit::None => unreachable!(),
        Submit::Aggregator(aggregator_submit) => aggregator_submit.component.config,
    };

    if submit_config.is_empty() {
        return Err("Workflow submit component config is empty".to_string());
    }

    let mut actions = Vec::new();

    if validate_tx && !utils::is_valid_tx(packet.trigger_data)? {
        return Ok(actions);
    }

    for (chain_key, service_handler_address) in submit_config {
        if host::get_evm_chain_config(&chain_key).is_some() {
            let address: alloy_primitives::Address = service_handler_address
                .parse()
                .map_err(|e| format!("Failed to parse address for '{chain_key}': {e}"))?;

            // Get gas price from Etherscan if configured.
            // Will fail the entire operation if API key is configured but fetching fails.
            let gas_price = gas_oracle::get_gas_price()?;

            let submit_action = SubmitAction {
                chain: chain_key.to_string(),
                contract_address: EvmAddress { raw_bytes: address.to_vec() },
                gas_price: gas_price.map(|x| x.into()),
            };

            actions.push(AggregatorAction::Submit(submit_action));
        } else if host::get_cosmos_chain_config(&chain_key).is_some() {
            todo!("Cosmos support coming soon...")
        } else {
            // Not a chain config key — skip (config may contain non-chain values)
            continue;
        }
    }

    Ok(actions)
}

export!(Component with_types_in bindings);
