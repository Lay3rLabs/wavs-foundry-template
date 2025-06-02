# WAVS Component Test Utilities

This library provides essential validation tools for WAVS components. All components **MUST** pass these tests before running the `make wasi-build` command.

## Overview

The test_utils component is a collection of utilities and validation scripts to ensure WAVS components meet the required standards and follow best practices. It's designed to catch common errors before they cause build failures or runtime issues.

## What It Does

- Validates component structure and implementation
- Checks for common anti-patterns and implementation mistakes
- Provides a standardized way to verify components
- Ensures consistent error handling, data management, and API usage

## Key Features

- Automated code analysis
- Comprehensive validation of ABI encoding/decoding
- Data ownership and cloning validation
- Error handling pattern verification
- Network request and API security validation

## Using the Validation Script

The main validation script can be used to verify any component:

```bash
# Validate a component using the Makefile command
make validate-component COMPONENT=your-component-name

# Or run the script directly
cd test_utils
./validate_component.sh your-component-name
```


## Test Modules

The test utilities are organized into focused modules:

| Module | Description |
|--------|-------------|
| `abi_encoding` | Proper handling of ABI-encoded data, avoiding common String::from_utf8 errors |
| `code_quality` | Code quality checks, including detecting unused imports and other best practices |
| `data_handling` | Correct data ownership, cloning, and avoiding moved value errors |
| `error_handling` | Proper Option/Result handling, avoiding map_err on Option errors |
| `network_requests` | HTTP request setup, error handling, and API key management |
| `solidity_types` | Working with Solidity types, numeric conversions, and struct handling |
| `input_validation` | Input data validation, safe decoding, and defensive programming |

## Common Errors Prevented

These tests help you avoid the following common errors:

1. Using `String::from_utf8` directly on ABI-encoded data
2. Missing Clone derivation on API response structs 
3. Using `map_err()` on Option types instead of `ok_or_else()`
4. Improper Rust-Solidity type conversions
5. Ownership issues with collection elements
6. Using `&data.clone()` pattern creating temporary values
7. Missing trait imports causing "no method" errors
8. Ambiguous method calls requiring fully qualified syntax
9. Unused imports cluttering the code
10. Direct version specifications instead of workspace dependencies

