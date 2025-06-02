# WAVS Component Validation Checks

This document outlines the validation checks performed by the `validate_component.sh` script on WAVS components. Each check is described along with what issues it's designed to detect and prevent.

## ABI ENCODING CHECKS

### 1. String::from_utf8 Misuse Check
- **What it checks**: Searches for uses of `String::from_utf8` on ABI-encoded data
- **What it catches**: Prevents runtime errors from trying to interpret binary ABI data as UTF-8 text
- **Common error pattern**: `String::from_utf8(abi_encoded_data)`
- **Fix**: Use proper ABI decoding methods like `functionCall::abi_decode()` or `String::abi_decode()`

### 2. ABI Decoding Methods Check
- **What it checks**: Verifies that components handling ABI-encoded inputs use proper decoding methods
- **What it catches**: Prevents runtime errors from improper decoding of function calls or parameters
- **Fix**: Implement appropriate ABI decoding using `FunctionCall::abi_decode`, `String::abi_decode`, etc.

### 3. Solidity Function Definition Check
- **What it checks**: Ensures components receiving function calls define Solidity function signatures
- **What it catches**: Prevents missing function definitions needed for proper ABI decoding
- **Fix**: Define Solidity functions using the `sol!` macro

## DATA HANDLING CHECKS

### 4. Clone Derivation Check
- **What it checks**: Verifies that API response structs derive the `Clone` trait
- **What it catches**: Prevents ownership issues when reusing data from API responses
- **Common error pattern**: `#[derive(Deserialize)]` without `Clone`
- **Fix**: Add `Clone` to derive macros for structs: `#[derive(Deserialize, Clone)]`

### 5. Temporary Clone Pattern Check
- **What it checks**: Detects `&data.clone()` patterns that create immediately dropped values
- **What it catches**: Prevents subtle ownership bugs where cloned values are dropped immediately
- **Common error pattern**: `let result = process(&data.clone());`
- **Fix**: Create a variable to hold the cloned data: `let data_clone = data.clone(); let result = process(&data_clone);`

### 6. "Move Out of Index" Error Check
- **What it checks**: Finds code that accesses fields from collection elements without cloning
- **What it catches**: Prevents ownership errors when accessing fields from collections
- **Common error pattern**: `let field = collection[0].field;`
- **Fix**: Clone the field to avoid moving it out: `let field = collection[0].field.clone();`

## ERROR HANDLING CHECKS

### 7. map_err on Option Types Check
- **What it checks**: Finds `map_err` used on `Option` types (especially `get_eth_chain_config`)
- **What it catches**: Prevents type errors from using `Result` methods on `Option` types
- **Common error pattern**: `get_eth_chain_config("mainnet").map_err(...)?`
- **Fix**: Use `ok_or_else` to convert `Option` to `Result`: `get_eth_chain_config("mainnet").ok_or_else(|| "Error message")?`

## IMPORT CHECKS

### 8. Essential Trait Import Check
- **What it checks**: Verifies that commonly used traits like `FromStr` are properly imported
- **What it catches**: Prevents compile errors from missing trait imports
- **Fix**: Add appropriate imports like `use std::str::FromStr;`

### 9. Min Function Import Check
- **What it checks**: Ensures `std::cmp::min` is imported when the `min` function is used
- **What it catches**: Prevents compile errors from missing the min function import
- **Fix**: Add `use std::cmp::min;` to imports

### 10. TxKind Import Path Check
- **What it checks**: Detects incorrect import paths for `TxKind` (a common blockchain type)
- **What it catches**: Prevents critical compilation errors from incorrect import paths
- **Common error pattern**: `alloy_rpc_types::eth::TxKind` (incorrect)
- **Fix**: Use `alloy_primitives::TxKind` instead

### 11. Block_on Import Check
- **What it checks**: Ensures `block_on` function is properly imported when used
- **What it catches**: Prevents compile errors from missing async runtime imports
- **Fix**: Add `use wstd::runtime::block_on;` to imports

### 12. HTTP Function Import Check
- **What it checks**: Verifies that HTTP-related functions are properly imported
- **What it catches**: Prevents compile errors from missing HTTP function imports
- **Fix**: Add `use wavs_wasi_chain::http::{fetch_json, http_request_get};`

### 13. SolCall Trait Import Check
- **What it checks**: Ensures the `SolCall` trait is imported when using `abi_encode` on function calls
- **What it catches**: Prevents compile errors when encoding function calls
- **Fix**: Add `use alloy_sol_types::{SolCall, SolValue};` to imports

## COMPONENT STRUCTURE CHECKS

### 14. Export Macro Usage Check
- **What it checks**: Verifies that the component uses the `export!` macro
- **What it catches**: Prevents missing component exports required for WASM compatibility
- **Fix**: Add `export!(YourComponent with_types_in bindings);` to the component

### 15. Export Macro Syntax Check
- **What it checks**: Ensures the correct syntax for the `export!` macro
- **What it catches**: Prevents errors from incorrect export macro syntax
- **Common error pattern**: `export!(YourComponent);` (incorrect)
- **Fix**: Use `export!(YourComponent with_types_in bindings);`

### 16. TriggerAction Structure Check
- **What it checks**: Detects incorrect field access on the `TriggerAction` struct
- **What it catches**: Prevents runtime errors from accessing non-existent fields
- **Common error pattern**: `trigger.trigger_data` (incorrect)
- **Fix**: Use `trigger.data` instead

### 17. Trigger.data Matching Check
- **What it checks**: Verifies that `trigger.data` is not treated as an `Option`
- **What it catches**: Prevents incorrect matching patterns on non-optional fields
- **Common error pattern**: `match trigger.data { Some(data) => {}, None => {} }`
- **Fix**: Treat as direct value: `match trigger.data { TriggerData::Raw => {}, ... }`

### 18. Guest Trait Implementation Check
- **What it checks**: Ensures the component implements the `Guest` trait
- **What it catches**: Prevents missing required trait implementations
- **Fix**: Implement `Guest` trait: `impl Guest for YourComponent { fn run(...) {...} }`

### 19. Run Function Signature Check
- **What it checks**: Verifies the `run` function has the correct return type signature
- **What it catches**: Prevents incompatible function signatures
- **Fix**: Use correct signature: `fn run(trigger: TriggerAction) -> Result<Option<Vec<u8>>, String>`

## SECURITY CHECKS

### 20. Hardcoded API Key Check
- **What it checks**: Searches for potential hardcoded API keys in the component
- **What it catches**: Prevents security issues from committing API keys to source control
- **Fix**: Use environment variables: `std::env::var("WAVS_ENV_YOUR_API_KEY")`

### 21. Hardcoded Secrets Check
- **What it checks**: Looks for other hardcoded secrets like tokens or passwords
- **What it catches**: Prevents security vulnerabilities from exposed credentials
- **Fix**: Use environment variables for all sensitive data

## DEPENDENCIES CHECKS

### 22. Workspace Dependency Usage Check
- **What it checks**: Ensures dependencies use `workspace = true` instead of explicit versions
- **What it catches**: Prevents version conflicts and ensures consistent dependency management
- **Common error pattern**: `some-crate = "0.1.0"` (incorrect)
- **Fix**: Use `some-crate = { workspace = true }`

## CODE QUALITY CHECKS

### 23. Cargo Check Compilation Check
- **What it checks**: Runs `cargo check` to find compile errors and warnings
- **What it catches**: Detects general Rust compilation issues early
- **Fix**: Resolve specific compiler errors and warnings

## SOLIDITY TYPES CHECKS

### 24. Sol Macro Import Check
- **What it checks**: Ensures the `sol!` macro is properly imported when used
- **What it catches**: Prevents compile errors from missing macro imports
- **Fix**: Add `use alloy_sol_types::sol;` or `use alloy_sol_macro::sol;`

### 25. Solidity Module Structure Check
- **What it checks**: Verifies that components using Solidity types define a proper `solidity` module
- **What it catches**: Prevents structural issues with Solidity type definitions
- **Fix**: Create a proper module structure:
  ```rust
  mod solidity {
      use alloy_sol_macro::sol;
      sol! { /* your solidity types */ }
  }
  ```

### 26. String Literal Conversion Check
- **What it checks**: Finds string literals assigned directly to `String` type fields
- **What it catches**: Prevents type mismatch errors between `&str` and `String`
- **Common error pattern**: `field: "literal string",` (when field is `String` type)
- **Fix**: Add explicit conversion: `field: "literal string".to_string(),`

## STRING SAFETY CHECKS

### 27. Unbounded String.repeat Check
- **What it checks**: Detects potentially unbounded `string.repeat()` operations
- **What it catches**: Prevents capacity overflow errors from excessive string repetition
- **Common error pattern**: `.repeat(variable)` with unbounded variable
- **Fix**: Add bounds: `.repeat(std::cmp::min(variable, 100))`

## NETWORK REQUEST CHECKS

### 28. Async Function Handling Check
- **What it checks**: Ensures async functions are properly wrapped with `block_on`
- **What it catches**: Prevents runtime issues with async function execution
- **Fix**: Wrap async calls: `block_on(async { make_request().await })`

## SUMMARY

These validation checks help ensure WAVS components:
1. Use proper ABI encoding/decoding techniques
2. Handle data ownership correctly
3. Import all required traits and functions
4. Follow the correct component structure
5. Handle errors properly
6. Maintain security best practices
7. Use workspace dependencies correctly
8. Avoid common string manipulation errors
9. Structure Solidity types correctly
10. Handle async operations properly

By validating components against these checks, developers can avoid common pitfalls and ensure their components will build and run correctly in the WAVS environment.