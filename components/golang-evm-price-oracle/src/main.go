package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"math/big"
	"net/http"
	"strconv"
	"strings"
	"time"

	"github.com/Lay3rLabs/wavs-wasi/go/types"
	wavs "github.com/Lay3rLabs/wavs-wasi/go/wavs/worker/layer-trigger-world"
	trigger "github.com/Lay3rLabs/wavs-wasi/go/wavs/worker/layer-types"

	wasiclient "github.com/dev-wasm/dev-wasm-go/lib/http/client"
	"go.bytecodealliance.org/cm"

	"github.com/defiweb/go-eth/abi"
)

func init() {
	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) types.TriggerResult {
		triggerID, requestInput, dest := decodeTriggerEvent(triggerAction.Data)

		result, err := compute(requestInput.Slice(), dest)
		if err != nil {
			return cm.Err[types.TriggerResult](err.Error())
		}
		fmt.Printf("Computation Result: %v\n", string(result))

		return routeResult(triggerID, result, dest)
	}
}

// compute is the main function that computes the price of the crypto currency
func compute(input []uint8, dest types.Destination) ([]byte, error) {
	// Input is now properly decoded by decodeTriggerEvent, so we can parse it directly
	inputStr := string(input)
	// Clean any remaining control characters and whitespace
	cleanStr := strings.Map(func(r rune) rune {
		if r >= 32 && r < 127 { // Keep only printable ASCII characters
			return r
		}
		return -1 // Remove control characters
	}, inputStr)
	cleanStr = strings.TrimSpace(cleanStr)
	
	fmt.Printf("Compute input (cleaned): '%s'\n", cleanStr)

	id, err := strconv.Atoi(cleanStr)
	if err != nil {
		return nil, fmt.Errorf("failed to parse input: %w", err)
	}

	priceFeed, err := fetchCryptoPrice(id)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch price: %w", err)
	}

	priceJson, err := json.Marshal(priceFeed)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal JSON: %w", err)
	}

	return priceJson, nil
}

// routeResult sends the computation result to the appropriate destination
func routeResult(triggerID uint64, result []byte, dest types.Destination) types.TriggerResult {
	switch dest {
	case types.CliOutput:
		return types.Ok(result, cm.None[uint64]())
	case types.Ethereum:
		// WAVS & the contract expects abi encoded data
		encoded := types.EncodeTriggerOutput(triggerID, result)
		fmt.Printf("Encoded output (raw): %x\n", encoded)
		return types.Ok(encoded, cm.None[uint64]())
	default:
		return cm.Err[types.TriggerResult](fmt.Sprintf("unsupported destination: %s", dest))
	}
}

// decodeABIInput decodes ABI-encoded input data using go-eth library
func decodeABIInput(input []byte) (string, error) {
	// First, convert the input bytes to a string to check if it's a hex string
	inputStr := string(input)

	var hexData []byte
	var err error

	// Check if it's a hex string (starts with "0x")
	if strings.HasPrefix(inputStr, "0x") {
		// Decode the hex string to bytes
		hexData, err = hex.DecodeString(inputStr[2:])
		if err != nil {
			return "", fmt.Errorf("failed to decode hex string: %w", err)
		}
	} else {
		// If it's not a hex string, assume the input is already binary data
		hexData = input
	}

	// Use go-eth library for generalized ABI decoding
	// Try to decode as string first (most common case)
	stringType := abi.NewStringType()
	var result string
	err = abi.DecodeValue(stringType, hexData, &result)
	if err == nil {
		return result, nil
	}

	// If string decoding fails, try other common types
	// Try uint256 (common for numeric inputs)
	uintType := abi.NewUintType(256)
	var uintResult *big.Int
	err = abi.DecodeValue(uintType, hexData, &uintResult)
	if err == nil {
		return uintResult.String(), nil
	}

	// Try bytes (fallback for arbitrary data)
	bytesType := abi.NewBytesType()
	var bytesResult []byte
	err = abi.DecodeValue(bytesType, hexData, &bytesResult)
	if err == nil {
		return string(bytesResult), nil
	}

	// If all ABI decoding attempts fail, return the raw input as string
	return string(input), nil
}

// decodeTriggerEvent is the function that decodes the trigger event from the chain event to Go.
func decodeTriggerEvent(triggerAction trigger.TriggerData) (trigger_id uint64, req cm.List[uint8], dest types.Destination) {
	// Handle CLI input case
	if triggerAction.Raw() != nil {
		raw := *triggerAction.Raw()
		fmt.Printf("Raw input: %s\n", string(raw.Slice()))

		// For CLI input, just use the raw string directly (no ABI decoding needed)
		// CLI inputs are simple strings like "1", "2", etc.
		rawString := string(raw.Slice())
		// Remove null bytes and trim whitespace
		cleanString := strings.ReplaceAll(rawString, "\x00", "")
		trimmedString := strings.TrimSpace(cleanString)
		fmt.Printf("CLI input (cleaned): %s\n", trimmedString)

		// Convert the trimmed string back to bytes for processing
		inputBytes := []byte(trimmedString)
		inputList := cm.NewList(&inputBytes[0], len(inputBytes))

		return 0, inputList, types.CliOutput
	}

	// Handle Ethereum event case
	ethEvent := triggerAction.EvmContractEvent()
	if ethEvent == nil {
		panic("triggerAction.EthContractEvent() is nil")
	}

	// Use generalized ABI decoding for Ethereum event data
	decodedString, err := decodeABIInput(ethEvent.Log.Data.Slice())
	if err != nil {
		fmt.Printf("Failed to decode Ethereum event ABI input: %v, using raw data\n", err)
		// Fallback to original method if generalized decoding fails
		triggerInfo := types.DecodeTriggerInfo(ethEvent.Log.Data.Slice())
		fmt.Printf("Trigger ID: %v\n", triggerInfo.TriggerID)
		fmt.Printf("Creator: %s\n", triggerInfo.Creator.String())
		fmt.Printf("Input Data: %v\n", string(triggerInfo.Data))
		return triggerInfo.TriggerID, cm.NewList(&triggerInfo.Data[0], len(triggerInfo.Data)), types.Ethereum
	}

	fmt.Printf("Decoded Ethereum event input: %s\n", decodedString)

	// For now, use trigger ID 0 and convert decoded string to bytes
	// In a real implementation, you might need to extract trigger ID from the event differently
	decodedBytes := []byte(decodedString)
	decodedList := cm.NewList(&decodedBytes[0], len(decodedBytes))

	return 0, decodedList, types.Ethereum
}

// fetchCryptoPrice fetches the price of the crypto currency from the CoinMarketCap API by their ID.
func fetchCryptoPrice(id int) (*PriceFeedData, error) {
	// Create a new HTTP client with WASI transport
	client := &http.Client{
		Transport: wasiclient.WasiRoundTripper{},
	}

	// Prepare the URL
	url := fmt.Sprintf("https://api.coinmarketcap.com/data-api/v3/cryptocurrency/detail?id=%d&range=1h", id)

	// Create a new HTTP request
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return nil, err
	}

	// Set the headers
	currentTime := time.Now().Unix()
	req.Header.Set("Accept", "application/json")
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36")
	req.Header.Set("Cookie", fmt.Sprintf("myrandom_cookie=%d", currentTime))

	// Make the request
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	// Read and parse the response
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	// Parse the JSON
	var root Root
	if err := json.Unmarshal(body, &root); err != nil {
		return nil, err
	}

	// round to 2 decimal places
	price := math.Round(root.Data.Statistics.Price*100) / 100

	// timestamp is 2025-04-30T19:59:44.161Z, becomes 2025-04-30T19:59:44
	timestamp := strings.Split(root.Status.Timestamp, ".")[0]

	return &PriceFeedData{
		Symbol:    root.Data.Symbol,
		Price:     price,
		Timestamp: timestamp,
	}, nil
}

// empty main function to satisfy wasm-ld (wit)
func main() {}
