#!/bin/bash
# Component validation script - IMPROVED VERSION
# Runs comprehensive test utilities to validate a component before building
# Catches all common errors that would prevent successful builds or execution

# Don't exit on error, we want to collect all errors
set +e

# Create an array to hold all errors
errors=()
warnings=()

# Function to add an error
add_error() {
    errors+=("$1")
    echo "❌ Error: $1"
}

# Function to add a warning
add_warning() {
    warnings+=("$1")
    echo "⚠️ Warning: $1"
}

if [ -z "$1" ]; then
  echo "Usage: $0 <component-directory-name>"
  echo "Example: $0 eth-price-oracle"
  exit 1
fi

COMPONENT_NAME=$1
COMPONENT_DIR="../components/$COMPONENT_NAME"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if component directory exists
if [ ! -d "$COMPONENT_DIR" ]; then
  echo "❌ Error: Component directory $COMPONENT_DIR not found"
  exit 1
fi

echo "🔍 Validating component: $COMPONENT_NAME"

# Print a section header for better organization
print_section() {
  echo
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "🔍 $1"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

#=====================================================================================
# ABI ENCODING CHECKS
#=====================================================================================
print_section "ABI ENCODING CHECKS"

# 1. Check for String::from_utf8 usage on ABI data in non-generated files
echo "📝 Checking for common String::from_utf8 misuse..."
grep_result=$(grep -r "String::from_utf8" "$COMPONENT_DIR/src" --include="*.rs" | grep -v "bindings.rs" | grep -v "test" | grep -v "# CORRECT" || true)
if [ ! -z "$grep_result" ]; then
  if grep -r "String::from_utf8.*data" "$COMPONENT_DIR"/src/*.rs | grep -v "bindings.rs" > /dev/null; then
    error_detail=$(grep -r "String::from_utf8.*data" "$COMPONENT_DIR"/src/*.rs | grep -v "bindings.rs")
    add_error "Found String::from_utf8 used directly on ABI-encoded data.
      This will ALWAYS fail with 'invalid utf-8 sequence' because ABI-encoded data is binary.
      Use proper ABI decoding methods instead: 
      1. For function calls with string params: functionCall::abi_decode()
      2. For string params: String::abi_decode()
      $error_detail"
  else
    add_warning "Found String::from_utf8 usage. Ensure it's not being used on ABI-encoded data.
      This will likely cause runtime errors if used with encoded data.
      $grep_result"
  fi
fi

# 1b. Check for proper ABI decoding methods
echo "📝 Checking for proper ABI decoding methods..."
if grep -r "TriggerData::Raw" "$COMPONENT_DIR"/src/*.rs > /dev/null || 
   grep -r "cast abi-encode" "$COMPONENT_DIR" > /dev/null; then
  
  # Component deals with ABI-encoded input data
  if ! grep -r "abi_decode" "$COMPONENT_DIR"/src/*.rs > /dev/null && 
     ! grep -r "::abi_decode" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
    add_error "Component appears to handle ABI-encoded input but doesn't use abi_decode methods.
      This will cause runtime errors when processing ABI-encoded data.
      For ABI-encoded input, use proper decoding methods:
      1. <String as SolValue>::abi_decode(&hex_data)
      2. <Type as SolValue>::abi_decode(&data)
      3. functionCall::abi_decode(&data)"
  fi
  
  # Check for Solidity function definitions when receiving function calls
  if grep -r "cast abi-encode \"f(string)" "$COMPONENT_DIR" > /dev/null && 
     ! grep -r "function.*external" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
    add_error "Component receives ABI-encoded function calls but doesn't define Solidity functions.
      This will cause runtime errors when trying to decode function calls.
      Define appropriate Solidity functions to decode inputs, for example:
      sol! {
          function checkBalance(string address) external;
      }"
  fi
fi

#=====================================================================================
# DATA HANDLING CHECKS
#=====================================================================================
print_section "DATA HANDLING CHECKS"

# 2a. Check for proper Clone derivation on API structs used with network requests
echo "📝 Checking for Clone derivation on structs..."
# Look for structs used in HTTP responses
HTTP_USAGE=$(if grep -r "fetch_json\|http_request_get" "$COMPONENT_DIR"/src/*.rs > /dev/null 2>&1; then echo "1"; else echo "0"; fi)

# Find structs with Deserialize but missing Clone
STRUCTS_WITH_DERIVE=$(grep -r -B 2 "struct" "$COMPONENT_DIR/src" | grep "derive" || true)
STRUCTS_WITH_DESERIALIZE=$(echo "$STRUCTS_WITH_DERIVE" | grep "Deserialize" || true)
STRUCTS_WITHOUT_CLONE=$(echo "$STRUCTS_WITH_DESERIALIZE" | grep -v "Clone" || true)

if [ ! -z "$STRUCTS_WITHOUT_CLONE" ]; then
  # Check if any struct without Clone is used more than once
  STRUCT_USAGE_ERROR=false
  
  # Extract struct names from the output
  while read -r line; do
    # Extract struct name using sed - matches "struct Name {"
    STRUCT_LINE=$(echo "$line" | grep -A 1 "derive" || true)
    if [ ! -z "$STRUCT_LINE" ]; then
      STRUCT_NAME=$(echo "$STRUCT_LINE" | grep "struct" | sed -E 's/.*struct\s+([A-Za-z0-9_]+).*/\1/')
      
      if [ ! -z "$STRUCT_NAME" ]; then
        # Count usages of this struct (excluding declaration and imports)
        USAGE_COUNT=$(grep -r "$STRUCT_NAME" "$COMPONENT_DIR"/src/*.rs | grep -v "struct $STRUCT_NAME" | grep -v "use.*$STRUCT_NAME" | wc -l)
        
        # If used multiple times or in JSON handling, it should have Clone
        if [ "$USAGE_COUNT" -gt 2 ] || grep -q "serde_json.*$STRUCT_NAME" "$COMPONENT_DIR"/src/*.rs; then
          STRUCT_USAGE_ERROR=true
          break
        fi
      fi
    fi
  done <<< "$STRUCTS_WITHOUT_CLONE"
  
  # If HTTP request component or multiple usages detected, make it an error
  if [ "$HTTP_USAGE" != "0" ] && [ "$STRUCT_USAGE_ERROR" = true ]; then
    add_error "Found structs with Deserialize but missing Clone derivation that are used multiple times:
    $STRUCTS_WITHOUT_CLONE
  
  Structs used multiple times with API responses MUST derive Clone to prevent ownership errors.
  Fix: Add Clone to the derive list like this:
    #[derive(Serialize, Deserialize, Debug, Clone)]"
  else
    add_warning "Found structs with Deserialize but missing Clone derivation:
    $STRUCTS_WITHOUT_CLONE
  
  Consider adding Clone for consistency:
    #[derive(Serialize, Deserialize, Debug, Clone)]"
  fi
fi

# 2b. Check for temporary clone pattern (&data.clone())
echo "📝 Checking for incorrect &data.clone() pattern..."
TEMP_CLONE_PATTERN=$(grep -r "&.*\.clone()" "$COMPONENT_DIR"/src/*.rs || true)
if [ ! -z "$TEMP_CLONE_PATTERN" ]; then
  add_error "Found dangerous &data.clone() pattern which creates temporary values that are immediately dropped.
      This pattern causes ownership issues because the cloned data is immediately dropped.
      Fix: Create a named variable to hold the cloned data instead:
      WRONG:  let result = std::str::from_utf8(&data.clone());
      RIGHT:  let data_clone = data.clone();
              let result = std::str::from_utf8(&data_clone);
      $TEMP_CLONE_PATTERN"
fi

# 2c. Check for potential "move out of index" errors
echo "📝 Checking for potential 'move out of index' errors..."
MOVE_OUT_INDEX=$(grep -r "\[.*\]\..*" "$COMPONENT_DIR"/src/*.rs | grep -v "\.clone()" | grep -v "\.as_ref()" | grep -v "&" | grep -v "bindings.rs" || true)
if [ ! -z "$MOVE_OUT_INDEX" ]; then
  add_error "Found potential 'move out of index' errors - accessing collection elements without cloning.
      When accessing fields from elements in a collection, you should clone the field to avoid
      moving out of the collection, which would make the collection unusable afterward.
      WRONG:  let field = collection[0].field; // This moves the field out of the collection
      RIGHT:  let field = collection[0].field.clone(); // This clones the field
      $MOVE_OUT_INDEX"
fi

#=====================================================================================
# ERROR HANDLING CHECKS
#=====================================================================================
print_section "ERROR HANDLING CHECKS"

# 3a. Check for map_err on Option types - focus only on get_evm_chain_config specifically
echo "📝 Checking for map_err on Option types..."
MAP_ERR_CHAIN_CONFIG=$(grep -r "get_evm_chain_config" "$COMPONENT_DIR"/src/*.rs | grep "map_err" | grep -v "ok_or_else" 2>/dev/null || true)

if [ ! -z "$MAP_ERR_CHAIN_CONFIG" ]; then
  add_error "Found map_err used directly on get_evm_chain_config which returns Option, not Result.
      Option types don't have map_err method - it's only available on Result types.
      WRONG:  get_evm_chain_config(\"ethereum\").map_err(|e| e.to_string())?
      RIGHT:  get_evm_chain_config(\"ethereum\").ok_or_else(|| \"Failed to get config\".to_string())?
      $MAP_ERR_CHAIN_CONFIG"
fi

#=====================================================================================
# IMPORT CHECKS
#=====================================================================================
print_section "IMPORT CHECKS"

# 4a. Check for proper import of essential traits and types
echo "📝 Checking for essential imports..."
if grep -r "FromStr" "$COMPONENT_DIR"/src/*.rs > /dev/null && ! grep -r "use std::str::FromStr" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  FROMSTR_USAGE=$(grep -r "FromStr" "$COMPONENT_DIR"/src/*.rs | grep -v "use std::str::FromStr" || true)
  add_error "Found FromStr usage but std::str::FromStr is not imported.
      This will cause a compile error when using methods like from_str or parse<Type>().
      Fix: Add 'use std::str::FromStr;' to your imports.
      $FROMSTR_USAGE"
fi

# 4b. Check for min function usage without import
if grep -r "min(" "$COMPONENT_DIR"/src/*.rs > /dev/null && ! grep -r "use std::cmp::min" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  MIN_USAGE=$(grep -r "min(" "$COMPONENT_DIR"/src/*.rs | grep -v "use std::cmp::min" || true)
  add_error "Found min function usage but std::cmp::min is not imported.
      This will cause a compile error when using min().
      Fix: Add 'use std::cmp::min;' to your imports.
      $MIN_USAGE"
fi

# 4c. Check for TxKind import issues
if grep -r "alloy_rpc_types::eth::TxKind" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  TXKIND_USAGE=$(grep -r "alloy_rpc_types::eth::TxKind" "$COMPONENT_DIR"/src/*.rs || true)
  add_error "Found incorrect TxKind import path. Use alloy_primitives::TxKind instead of alloy_rpc_types::eth::TxKind.
      This is a critical error that will prevent component compilation.
      Fix: 1. Add 'use alloy_primitives::{Address, TxKind, U256};' (or add TxKind to existing import)
           2. Replace 'alloy_rpc_types::eth::TxKind::Call' with 'TxKind::Call'
      $TXKIND_USAGE"
fi

# 4d. Check for TxKind usage without import
if grep -r "::Call" "$COMPONENT_DIR"/src/*.rs > /dev/null && ! grep -r "use.*TxKind" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  CALL_USAGE=$(grep -r "::Call" "$COMPONENT_DIR"/src/*.rs | grep -v "use.*TxKind" || true)
  add_error "Found TxKind usage but TxKind is not properly imported.
      Fix: Add 'use alloy_primitives::TxKind;' to your imports.
      $CALL_USAGE"
fi

# 4e. Check for block_on usage without the correct import - improved to handle grouped imports
echo "📝 Checking for block_on import..."
if grep -r "block_on" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  # Check both formats: direct import and grouped import
  DIRECT_IMPORT=$(grep -r "use wstd::runtime::block_on" "$COMPONENT_DIR"/src/*.rs || true)
  GROUPED_IMPORT=$(grep -r "use wstd::{.*runtime::block_on" "$COMPONENT_DIR"/src/*.rs || true)
  RUNTIME_IMPORT=$(grep -r "use wstd::.*runtime" "$COMPONENT_DIR"/src/*.rs || true)
  
  if [ -z "$DIRECT_IMPORT" ] && [ -z "$GROUPED_IMPORT" ] && [ -z "$RUNTIME_IMPORT" ]; then
    BLOCK_ON_USAGE=$(grep -r "block_on" "$COMPONENT_DIR"/src/*.rs || true)
    add_error "Found block_on usage but wstd::runtime::block_on is not imported.
      This will cause a compile error when using async functions.
      Fix: Add 'use wstd::runtime::block_on;' to your imports.
      $BLOCK_ON_USAGE"
  fi
fi

# 4f. Check for HTTP function imports
if grep -r "http_request_" "$COMPONENT_DIR"/src/*.rs > /dev/null || grep -r "fetch_json" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  # Check for both direct import and grouped import patterns
  DIRECT_HTTP_IMPORT=$(grep -r "use wavs_wasi_utils::http::" "$COMPONENT_DIR"/src/*.rs || true)
  GROUPED_HTTP_IMPORT=$(grep -r "use wavs_wasi_utils::{.*http::{.*fetch_json\|.*http_request_" "$COMPONENT_DIR"/src/*.rs || true)
  
  if [ -z "$DIRECT_HTTP_IMPORT" ] && [ -z "$GROUPED_HTTP_IMPORT" ]; then
    HTTP_USAGE=$(grep -r "http_request_\|fetch_json" "$COMPONENT_DIR"/src/*.rs || true)
    add_error "Found HTTP function usage but wavs_wasi_utils::http is not imported.
      Fix: Add 'use wavs_wasi_utils::http::{fetch_json, http_request_get};' to your imports.
      $HTTP_USAGE"
  fi
fi

# 4g. Check for SolCall trait missing when using abi_encode
if grep -r "abi_encode" "$COMPONENT_DIR"/src/*.rs > /dev/null && ! grep -r "use.*SolCall" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  if grep -r "Call.*abi_encode" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
    CALL_ABI_USAGE=$(grep -r "Call.*abi_encode" "$COMPONENT_DIR"/src/*.rs || true)
    add_error "Found Call::abi_encode usage but SolCall trait is not imported.
      Function calls require the SolCall trait for encoding.
      Fix: Add 'use alloy_sol_types::{SolCall, SolValue};' to your imports.
      $CALL_ABI_USAGE"
  fi
fi

# After the existing import checks, add:
echo "📝 Checking for missing dependencies..."
# Get all local module names (mod foo;) from src/*.rs
LOCAL_MODS=$(grep -h -E '^mod ' "$COMPONENT_DIR"/src/*.rs | sed -E 's/^mod ([a-zA-Z0-9_]+);/\1/' | sort | uniq)
# Add known local modules
LOCAL_MODS="$LOCAL_MODS trigger bindings"
# Get all imports from the code, extract just the crate names
IMPORTS=$(grep -h -r "^use" "$COMPONENT_DIR"/src/*.rs | \
  sed -E 's/^use[[:space:]]+//' | \
  sed -E 's/ as [^;]+//' | \
  sed -E 's/[{].*//' | \
  sed -E 's/;.*//' | \
  cut -d: -f1 | \
  awk -F'::' '{print $1}' | \
  grep -vE '^(crate|self|super|std|core|wavs_wasi_utils|wstd)$' | \
  sort | uniq)

# Check each import against Cargo.toml dependencies
for import in $IMPORTS; do
  # Skip empty lines
  if [[ -z "$import" ]]; then
    continue
  fi
  # Skip local modules
  if echo "$LOCAL_MODS" | grep -wq "$import"; then
    continue
  fi
  # Convert import name to Cargo.toml format (replace underscores with hyphens)
  cargo_name=$(echo "$import" | tr '_' '-')
  # Check if the import is in Cargo.toml (either directly or as a workspace dependency)
  if ! grep -q "$cargo_name.*=.*{.*workspace.*=.*true" "$COMPONENT_DIR/Cargo.toml" && ! grep -q "$cargo_name.*=.*\"" "$COMPONENT_DIR/Cargo.toml"; then
    add_error "Import '$import' is used but not found in Cargo.toml dependencies.\n        Add it to your [dependencies] section in Cargo.toml and to [workspace.dependencies] in the root Cargo.toml."
  fi
done

#=====================================================================================
# COMPONENT STRUCTURE CHECKS 
#=====================================================================================
print_section "COMPONENT STRUCTURE CHECKS"

# 5a. Check for proper export! macro usage and syntax
echo "📝 Checking for proper component export..."
if ! grep -r "export!" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  add_error "export! macro not found. Components must use export! macro.
      Fix: Add 'export!(YourComponent with_types_in bindings);' to your component."
fi

# 5b. Check for correct export! macro syntax with with_types_in
if grep -r "export!" "$COMPONENT_DIR"/src/*.rs > /dev/null && ! grep -r "export!.*with_types_in bindings" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  EXPORT_USAGE=$(grep -r "export!" "$COMPONENT_DIR"/src/*.rs || true)
  add_error "Incorrect export! macro syntax. Use 'export!(YourComponent with_types_in bindings)' instead of just 'export!(YourComponent)'.
      Fix: Update to 'export!(YourComponent with_types_in bindings);'
      $EXPORT_USAGE"
fi

# 5c. Check for TriggerAction structure usage issues
echo "📝 Checking for TriggerAction structure usage..."
if grep -r "trigger.trigger_data" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  TRIGGER_DATA_USAGE=$(grep -r "trigger.trigger_data" "$COMPONENT_DIR"/src/*.rs || true)
  add_error "Component accesses non-existent 'trigger_data' field on TriggerAction. Use 'trigger.data' instead.
      $TRIGGER_DATA_USAGE"
fi

# 5d. Check for incorrect match pattern on trigger.data (treating it as Option)
if grep -r -A 5 -B 2 "match trigger.data" "$COMPONENT_DIR"/src/*.rs 2>/dev/null | grep -q "Some(" &&
   grep -r -A 8 -B 2 "match trigger.data" "$COMPONENT_DIR"/src/*.rs 2>/dev/null | grep -q "None =>"; then
  TRIGGER_MATCH=$(grep -r -A 5 -B 2 "match trigger.data" "$COMPONENT_DIR"/src/*.rs || true)
  add_error "Component incorrectly treats 'trigger.data' as an Option<TriggerData>, but it's a TriggerData.
      The field is not optional - don't match against Some/None patterns.
      $TRIGGER_MATCH"
fi

# 5e. Check for Guest trait implementation
if ! grep -r "impl Guest for" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  add_error "Guest trait implementation not found. Components must implement the Guest trait.
      Fix: Add 'impl Guest for YourComponent { fn run(trigger: TriggerAction) -> Result<Option<WasmResponse>, String> { ... } }'"
fi

# 5f. Check for run function with correct signature - improved to accept variations in naming/qualification
if ! grep -r "fn run(.*TriggerAction.*) -> .*Result<Option<WasmResponse>, String>" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  add_error "run function with correct result signature not found.
      The run function must return std::result::Result<Option<WasmResponse>, String>"
fi

#=====================================================================================
# SECURITY CHECKS 
#=====================================================================================
print_section "SECURITY CHECKS"

# 6a. Check for hardcoded API keys
echo "📝 Checking for hardcoded API keys..."
API_KEYS=$(grep -r "key=.*[0-9a-zA-Z]\{8,\}" "$COMPONENT_DIR" --include="*.rs" || true)
if [ ! -z "$API_KEYS" ]; then
  add_error "Found possible hardcoded API key. Use environment variables instead.
      Fix: Use std::env::var(\"WAVS_ENV_YOUR_API_KEY\") to get API keys from environment variables.
      $API_KEYS"
fi

# 6b. Check for other potential hardcoded secrets
OTHER_SECRETS=$(grep -r "token=\|secret=\|password=" "$COMPONENT_DIR" --include="*.rs" | grep "[0-9a-zA-Z]\{8,\}" || true)
if [ ! -z "$OTHER_SECRETS" ]; then
  add_error "Found possible hardcoded secret. Use environment variables instead.
      Fix: Use std::env::var(\"WAVS_ENV_YOUR_SECRET\") to get secrets from environment variables.
      $OTHER_SECRETS"
fi

#=====================================================================================
# DEPENDENCIES CHECKS
#=====================================================================================
print_section "DEPENDENCIES CHECKS"

# 7. Check for proper workspace dependency usage
echo "📝 Checking for proper workspace dependency usage..."
VERSION_NUMBERS=$(grep -r "version = \"[0-9]" "$COMPONENT_DIR/Cargo.toml" || true)
if [ ! -z "$VERSION_NUMBERS" ]; then
  add_error "Found direct version numbers in Cargo.toml. Use workspace = true instead.
      Fix: Replace version numbers with { workspace = true } for all dependencies.
      WRONG:  some-crate = \"0.1.0\"
      RIGHT:  some-crate = { workspace = true }
      $VERSION_NUMBERS"
fi

#=====================================================================================
# CODE QUALITY CHECKS
#=====================================================================================
print_section "CODE QUALITY CHECKS"

# 8. Check for unused imports with cargo check
echo "📝 Checking for unused imports and code issues..."
cd "$SCRIPT_DIR/.."
COMPONENT_NAME_SIMPLE=$(basename "$COMPONENT_DIR")

# Run cargo check and capture any errors (not just warnings)
CARGO_OUTPUT=$(cargo check -p "$COMPONENT_NAME_SIMPLE" 2>&1)
CARGO_ERRORS=$(echo "$CARGO_OUTPUT" | grep -i "error:" | grep -v "generated file bindings.rs" || true)

if [ ! -z "$CARGO_ERRORS" ]; then
  add_error "cargo check found compilation errors:
  $CARGO_ERRORS"
fi

# Show warnings but don't fail on them
CARGO_WARNINGS=$(echo "$CARGO_OUTPUT" | grep -i "warning:" | grep -v "profiles for the non root package" || true)
if [ ! -z "$CARGO_WARNINGS" ]; then
  add_warning "cargo check found warnings that might indicate issues:
  $CARGO_WARNINGS"
fi

cd "$SCRIPT_DIR"

#=====================================================================================
# SOLIDITY TYPES CHECKS
#=====================================================================================
print_section "SOLIDITY TYPES CHECKS"

# 9a. Check for sol! macro usage without proper import
echo "📝 Checking for sol! macro imports..."
if grep -r "sol!" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  if ! grep -r "use alloy_sol_types::sol" "$COMPONENT_DIR"/src/*.rs > /dev/null && ! grep -r "use alloy_sol_macro::sol" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
    SOL_USAGE=$(grep -r "sol!" "$COMPONENT_DIR"/src/*.rs || true)
    add_error "Found sol! macro usage but neither alloy_sol_types::sol nor alloy_sol_macro::sol is imported.
      Fix: Add 'use alloy_sol_types::sol;' to your imports.
      $SOL_USAGE"
  fi
fi

# 9b. Check for solidity module structure
echo "📝 Checking for proper solidity module structure..."
if grep -r "sol::" "$COMPONENT_DIR"/src/*.rs > /dev/null && ! grep -r "mod solidity" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  SOL_NAMESPACE=$(grep -r "sol::" "$COMPONENT_DIR"/src/*.rs || true)
  add_error "Found 'sol::' namespace usage without defining a 'solidity' module.
      Fix: Create a proper module structure like:
      mod solidity {
          use alloy_sol_types::sol;
          sol! { /* your solidity types */ }
      }
      $SOL_NAMESPACE"
fi

# 9c. Check for string literals assigned to String type fields in structs
echo "📝 Checking for string literal to String conversions..."
# Look for patterns like 'field: "string literal",' in struct initializations
# Only check lib.rs to avoid auto-generated bindings.rs
if [ -f "$COMPONENT_DIR/src/lib.rs" ]; then
  STRING_FIELDS=$(grep -A 20 "pub struct" "$COMPONENT_DIR/src/lib.rs" | grep -E "^\s*pub\s+[a-zA-Z0-9_]+:\s+String," | sed -E 's/^\s*pub\s+([a-zA-Z0-9_]+):\s+String,.*/\1/' || true)
  
  if [ ! -z "$STRING_FIELDS" ]; then
    # For each string field, check for literals without to_string()
    for FIELD in $STRING_FIELDS; do
      # Skip if field name is empty or contains special characters
      if [[ "$FIELD" =~ ^[a-zA-Z0-9_]+$ ]]; then
        # Look for patterns like 'field: "literal",' without to_string()
        STRING_LITERAL_USAGE=$(grep -r "$FIELD: \"" "$COMPONENT_DIR"/src/lib.rs | grep -v "\.to_string()" || true)
        
        if [ ! -z "$STRING_LITERAL_USAGE" ]; then
          add_error "Found string literals assigned directly to String type fields without .to_string() conversion:
          $STRING_LITERAL_USAGE
          
          This will cause a type mismatch error because &str cannot be assigned to String.
          Fix: Always convert string literals to String type using .to_string():
          WRONG:  field: \"literal string\",
          RIGHT:  field: \"literal string\".to_string(),"
          break
        fi
      fi
    done
  fi
fi

#=====================================================================================
# STRING SAFETY CHECKS
#=====================================================================================
print_section "STRING SAFETY CHECKS"

# 10a. Check for unbounded string.repeat operations
echo "📝 Checking for string capacity overflow risks..."

# First, collect all .repeat() calls - simpler approach to catch all possible cases
REPEAT_CALLS=$(grep -r "\.repeat(" "$COMPONENT_DIR"/src/*.rs || true)

if [ ! -z "$REPEAT_CALLS" ]; then
  # Look for any .repeat() calls with potentially unsafe variables
  RISKY_REPEAT_PATTERNS="decimals\|padding\|len\|size\|count\|width\|height\|indent\|offset\|spaces\|zeros\|chars\|digits"
  
  # Check for specific safety patterns
  SAFETY_PATTERNS="std::cmp::min\|::min(\|min(\|// SAFE:"
  
  # Check if any .repeat call doesn't use a safety bound
  UNSAFE_REPEATS=$(echo "$REPEAT_CALLS" | grep -i "$RISKY_REPEAT_PATTERNS" | grep -v "$SAFETY_PATTERNS" || true)
  
  if [ ! -z "$UNSAFE_REPEATS" ]; then
    add_error "Found potentially unbounded string.repeat operations:
$UNSAFE_REPEATS

This can cause capacity overflow errors. Options to fix:
  1. Add a direct safety check: \".repeat(std::cmp::min(variable, 100))\"
  2. Use a bounded variable: \"let safe_value = std::cmp::min(variable, MAX_SIZE); .repeat(safe_value)\"
  3. Add a safety comment if manually verified: \"// SAFE: bounded by check above\""
  fi
fi

#=====================================================================================
# NETWORK REQUEST CHECKS
#=====================================================================================
print_section "NETWORK REQUEST CHECKS"

# 11a. Check for proper block_on usage with async functions
echo "📝 Checking for proper async handling..."
if grep -r "async fn" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
  if ! grep -r "block_on" "$COMPONENT_DIR"/src/*.rs > /dev/null; then
    ASYNC_USAGE=$(grep -r "async fn" "$COMPONENT_DIR"/src/*.rs || true)
    add_error "Found async functions but no block_on usage.
      Async functions must be called with block_on in component run functions:
      block_on(async { make_request().await })
      $ASYNC_USAGE"
  fi
fi

#=====================================================================================
# FINAL SUCCESS MESSAGE
#=====================================================================================
print_section "VALIDATION SUMMARY"

# Check if there are any errors or warnings
ERROR_COUNT=${#errors[@]}
WARNING_COUNT=${#warnings[@]}

if [ $ERROR_COUNT -gt 0 ]; then
  echo "❌ Component validation failed with $ERROR_COUNT errors and $WARNING_COUNT warnings."
  echo 
  echo "⚠️  YOU MUST FIX ALL ERRORS BEFORE RUNNING 'make wasi-build'."
  echo "    Failure to fix these issues will result in build or runtime errors."
  exit 1
else
  if [ $WARNING_COUNT -gt 0 ]; then
    echo "⚠️  Component validation passed with $WARNING_COUNT warnings."
    echo "    Consider fixing these warnings to improve your component's reliability."
  else
    echo "✅ Component validation checks complete! No errors or warnings found."
  fi
  
  echo "🚀 Component is ready for building. Run the following command to build:"
  echo "    cd ../.. && make wasi-build"
fi

# After all static checks, add:
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 CARGO CHECK (compilation test)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo check --manifest-path "$(pwd)/../components/$COMPONENT_NAME/Cargo.toml" --target wasm32-wasip1
