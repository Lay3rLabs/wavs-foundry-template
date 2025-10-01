use alloy_sol_macro::sol;

// Import all types from ITypes.sol
pub use ITypes::*;

// The objects here will be generated automatically into Rust types.
// If you update the .sol file, you must re-run `cargo build` to see the changes.
sol!("../../src/interfaces/ITypes.sol");

// Define a simple struct representing the function that encodes string input
sol! {
    function addTrigger(string data) external;
}
