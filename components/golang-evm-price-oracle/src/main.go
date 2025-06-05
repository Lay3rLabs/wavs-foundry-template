package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"net/http"
	"strconv"
	"strings"
	"time"

	"github.com/Lay3rLabs/wavs-wasi/go/types"
	wavs "github.com/Lay3rLabs/wavs-wasi/go/wavs/worker/layer-trigger-world"
	trigger "github.com/Lay3rLabs/wavs-wasi/go/wavs/worker/layer-types"

	wasiclient "github.com/dev-wasm/dev-wasm-go/lib/http/client"
	"go.bytecodealliance.org/cm"
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
	if dest == types.CliOutput {
		input = bytes.TrimRight(input, "\x00")
	}

	id, err := strconv.Atoi(string(input))
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

// decodeABIStringInput decodes ABI-encoded string input, handling both hex string and binary data
func decodeABIStringInput(input []byte) (string, error) {
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

	// Manual ABI decoding for string parameter
	// ABI encoding for a string has the following format:
	// - First 32 bytes: offset to string data (should be 0x20 = 32 for single parameter)
	// - Next 32 bytes: length of string data
	// - Remaining bytes: string data (padded to 32-byte boundary)

	if len(hexData) < 64 {
		return "", fmt.Errorf("ABI data too short: expected at least 64 bytes, got %d", len(hexData))
	}

	// Read the offset (first 32 bytes) - should be 32 (0x20) for single string parameter
	offset := uint64(0)
	for i := 24; i < 32; i++ { // Read last 8 bytes as uint64
		offset = (offset << 8) | uint64(hexData[i])
	}

	if offset != 32 {
		return "", fmt.Errorf("unexpected offset: expected 32, got %d", offset)
	}

	// Read the length (next 32 bytes)
	length := uint64(0)
	for i := 56; i < 64; i++ { // Read last 8 bytes as uint64
		length = (length << 8) | uint64(hexData[i])
	}

	if uint64(len(hexData)) < 64+length {
		return "", fmt.Errorf("ABI data too short: expected at least %d bytes, got %d", 64+length, len(hexData))
	}

	// Extract the string data
	stringData := hexData[64 : 64+length]

	return string(stringData), nil
}

// decodeTriggerEvent is the function that decodes the trigger event from the chain event to Go.
func decodeTriggerEvent(triggerAction trigger.TriggerData) (trigger_id uint64, req cm.List[uint8], dest types.Destination) {
	// Handle CLI input case
	if triggerAction.Raw() != nil {
		raw := *triggerAction.Raw()
		fmt.Printf("Raw input: %s\n", string(raw.Slice()))

		// Try to decode as ABI-encoded string input
		decodedString, err := decodeABIStringInput(raw.Slice())
		if err != nil {
			fmt.Printf("Failed to decode ABI string input: %v, using raw input\n", err)
			return 0, raw, types.CliOutput
		}

		fmt.Printf("Decoded string input: %s\n", decodedString)

		// Convert the decoded string back to bytes for processing
		decodedBytes := []byte(decodedString)
		decodedList := cm.NewList(&decodedBytes[0], len(decodedBytes))

		return 0, decodedList, types.CliOutput
	}

	// Handle Ethereum event case
	ethEvent := triggerAction.EvmContractEvent()
	if ethEvent == nil {
		panic("triggerAction.EthContractEvent() is nil")
	}

	// if you modify the contract trigger from the default event, you will need to create a custom `DecodeTriggerInfo` function
	// to match the solidity contract data types.
	triggerInfo := types.DecodeTriggerInfo(ethEvent.Log.Data.Slice())

	fmt.Printf("Trigger ID: %v\n", triggerInfo.TriggerID)
	fmt.Printf("Creator: %s\n", triggerInfo.Creator.String())
	fmt.Printf("Input Data: %v\n", string(triggerInfo.Data))

	return triggerInfo.TriggerID, cm.NewList(&triggerInfo.Data[0], len(triggerInfo.Data)), types.Ethereum
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
