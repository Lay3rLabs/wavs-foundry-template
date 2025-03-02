package main

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"strconv"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/src/internal/wavs/worker/layer-trigger-world"
	wavstypes "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/src/internal/wavs/worker/layer-types"

	"github.com/defiweb/go-eth/abi"
	"go.bytecodealliance.org/cm"
)

func init() {
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

	// Convert input to int, multiply by 2, and convert back to string
	v, err := strconv.Atoi(string(input))
	if err != nil {
		return nil, fmt.Errorf("failed to parse input: %w", err)
	}

	v *= 2
	return []byte(strconv.Itoa(v)), nil
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
