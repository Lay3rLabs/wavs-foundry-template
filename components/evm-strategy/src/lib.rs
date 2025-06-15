mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction, WasmResponse};
use serde::{Deserialize, Serialize};
use wstd::runtime::block_on;

struct Component;
export!(Component with_types_in bindings);



impl Guest for Component {
    /// Main entry point for the vault lending strategy component.
    /// WAVS is subscribed to watch for events emitted by the blockchain.
    /// When WAVS observes an event is emitted, it will internally route the event and its data to this function (component).
    /// The processing then occurs before the output is returned back to WAVS to be submitted to the blockchain by the operator(s).
    ///
    /// This function:
    /// 1. Receives a trigger action containing encoded data
    /// 2. Decodes the input to get vault strategy parameters
    /// 3. Calculates optimal lending strategy based on current vault state
    /// 4. Returns the encoded response for on-chain validation and execution
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Parse strategy parameters from input
        let strategy_params: StrategyParams = serde_json::from_slice(&req)
            .map_err(|e| format!("Failed to parse strategy params: {}", e))?;
        
        println!("Strategy params: {:?}", strategy_params);

        let res = block_on(async move {
            // Calculate optimal lending strategy
            let lending_decision = calculate_lending_strategy(&strategy_params).await?;
            
            println!("Lending decision: {:?}", lending_decision);
            
            // Return the strategy decision for on-chain validation
            serde_json::to_vec(&lending_decision).map_err(|e| e.to_string())
        })?;

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &res)),
            Destination::CliOutput => Some(WasmResponse { payload: res.into(), ordering: None }),
        };
        Ok(output)
    }
}

/// Calculates the optimal lending strategy based on vault parameters
///
/// # Arguments
/// * `params` - Strategy parameters including vault state and market conditions
///
/// # Returns
/// * `LendingDecision` containing:
///   - action: Whether to lend, recall, or hold
///   - amount: Amount to lend/recall
///   - target_ratio: Optimal lending ratio
///   - confidence: Confidence level of the decision
async fn calculate_lending_strategy(params: &StrategyParams) -> Result<LendingDecision, String> {
    // Simple strategy logic - in production this would be more sophisticated
    let current_utilization = if params.vault_total_assets > 0 {
        (params.current_lent_amount as f64) / (params.vault_total_assets as f64)
    } else {
        0.0
    };
    
    let target_ratio = 0.8; // 80% target lending ratio
    let tolerance = 0.05; // 5% tolerance
    
    let action = if current_utilization < target_ratio - tolerance {
        "lend".to_string()
    } else if current_utilization > target_ratio + tolerance {
        "recall".to_string()
    } else {
        "hold".to_string()
    };
    
    let amount = if action == "lend" {
        ((target_ratio * params.vault_total_assets as f64) - params.current_lent_amount as f64) as u64
    } else if action == "recall" {
        (params.current_lent_amount as f64 - (target_ratio * params.vault_total_assets as f64)) as u64
    } else {
        0
    };
    
    Ok(LendingDecision {
        action,
        amount,
        target_ratio,
        confidence: 0.85, // Mock confidence score
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}

/// Strategy parameters passed to the off-chain compute function
#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyParams {
    vault_total_assets: u64,
    current_lent_amount: u64,
    lending_protocol_rate: f64,
    vault_utilization: f64,
    market_volatility: f64,
}

/// Decision output from the lending strategy calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingDecision {
    action: String, // "lend", "recall", or "hold"
    amount: u64,
    target_ratio: f64,
    confidence: f64,
    timestamp: u64,
}

