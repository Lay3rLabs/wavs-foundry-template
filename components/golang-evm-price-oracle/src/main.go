package main

import (
	"bytes"
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

// decodeTriggerEvent is the function that decodes the trigger event from the chain event to Go.
func decodeTriggerEvent(triggerAction trigger.TriggerData) (trigger_id uint64, req cm.List[uint8], dest types.Destination) {
	// Handle CLI input case
	if triggerAction.Raw() != nil {
		raw := *triggerAction.Raw()
		fmt.Printf("Raw input: %s\n", string(raw.Slice()))
		return 0, raw, types.CliOutput
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
