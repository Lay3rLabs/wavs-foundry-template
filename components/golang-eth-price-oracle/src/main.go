package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"
	"time"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/src/internal/wavs/worker/layer-trigger-world"
	wavstypes "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/src/internal/wavs/worker/layer-types"

	incominghandler "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/src/internal/wasi/http/incoming-handler"

	"github.com/defiweb/go-eth/abi"
	wasiclient "github.com/dev-wasm/dev-wasm-go/lib/http/client"
	"go.bytecodealliance.org/cm"
)

func init() {
	incominghandler.Exports.Handle = theHandler

	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) TriggerResult {
		triggerID, requestInput, dest := decodeTriggerEvent(triggerAction.Data)

		result, err := compute(requestInput.Slice(), dest)
		if err != nil {
			return cm.Err[TriggerResult](err.Error())
		}
		fmt.Printf("Computation Result: %v\n", string(result))

		return routeResult(triggerID, result, dest)
	}
}

// routeResult sends the computation result to the appropriate destination
func routeResult(triggerID uint64, result []byte, dest Destination) TriggerResult {
	switch dest {
	case CliOutput:
		return cm.OK[TriggerResult](cm.Some(cm.NewList(&result[0], len(result))))
	case Ethereum:
		// WAVS & the contract expects abi encoded data
		encoded := encodeTriggerOutput(triggerID, result)
		fmt.Printf("Encoded output (raw): %x\n", encoded)
		return cm.OK[TriggerResult](cm.Some(cm.NewList(&encoded[0], len(encoded))))
	default:
		return cm.Err[TriggerResult](fmt.Sprintf("unsupported destination: %s", dest))
	}
}

func compute(input []uint8, dest Destination) ([]byte, error) {
	if dest == CliOutput {
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

// encodeTriggerOutput abi encodes the output of the computation to be sent back to the Ethereum contract
func encodeTriggerOutput(trigger_id uint64, output []byte) []byte {
	/*
		for trigger_id of 1 and output of `test-data`, the proper solidity encoding is:

		Offset to the start of the struct (32 bytes)
		0x0000000000000000000000000000000000000000000000000000000000000020
		triggerId: 1 (uint64)
		0000000000000000000000000000000000000000000000000000000000000001
		Offset to the dynamic bytes field (64 bytes from struct start)
		0000000000000000000000000000000000000000000000000000000000000040
		Length of bytes array: 9
		0000000000000000000000000000000000000000000000000000000000000009
		Content: "test-data" with padding
		746573742d646174610000000000000000000000000000000000000000000000

		this is not how the abi encoding library works, so manually prepend the offset bytes to the data.
		verified by console.log in the solidity contract of some encoded test data, then cross comparing
		to this functions output.
	*/

	bz := abi.MustEncodeValue(DataWithIdABI, DataWithID{
		TriggerID: trigger_id,
		Data:      output,
	})

	offsetBytes, _ := hex.DecodeString("0000000000000000000000000000000000000000000000000000000000000020")
	return append(offsetBytes, bz...)
}

func decodeTriggerEvent(triggerAction wavstypes.TriggerData) (trigger_id uint64, req cm.List[uint8], dest Destination) {
	// Handle CLI input case
	if triggerAction.Raw() != nil {
		return 0, *triggerAction.Raw(), CliOutput
	}

	// Handle Ethereum event case
	ethEvent := triggerAction.EthContractEvent()
	if ethEvent == nil {
		panic("triggerAction.EthContractEvent() is nil")
	}

	triggerInfo := decodeTriggerInfo(ethEvent.Log.Data.Slice())

	fmt.Printf("Trigger ID: %v\n", triggerInfo.TriggerID)
	fmt.Printf("Creator: %s\n", triggerInfo.Creator.String())
	fmt.Printf("Input Data: %v\n", string(triggerInfo.Data))

	return triggerInfo.TriggerID, cm.NewList(&triggerInfo.Data[0], len(triggerInfo.Data)), Ethereum
}

func main() {}

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

	// Create the price feed data
	return &PriceFeedData{
		Symbol:    root.Data.Symbol,
		Price:     root.Data.Statistics.Price,
		Timestamp: root.Status.Timestamp,
	}, nil
}
