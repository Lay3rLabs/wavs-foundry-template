package main

import (
	"encoding/binary"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/src/internal/wavs/worker/layer-trigger-world"

	"github.com/defiweb/go-eth/abi"
	types "github.com/defiweb/go-eth/types"
	"go.bytecodealliance.org/cm"
)

// TriggerResult is the return type for wavs trigger world Run function
type TriggerResult = cm.Result[wavs.OptionListU8Shape, cm.Option[cm.List[uint8]], string]

// Destination represents output destinations
type Destination string

const (
	Ethereum  Destination = "Ethereum"
	CliOutput Destination = "CliOutput"
)

// TriggerInfo contains all the data related to a trigger event from the contract
type TriggerInfo struct {
	TriggerID uint64        `abi:"triggerId"`
	Creator   types.Address `abi:"creator"`
	Data      []byte        `abi:"data"`
}

// DataWithIdABI defines the ABI structure used for encoding return values
var DataWithIdABI = abi.MustParseStruct(`struct DataWithId { uint64 triggerId; bytes data; }`)

// DataWithID contains data to be sent back to the contract
type DataWithID struct {
	TriggerID uint64 `abi:"triggerId"`
	Data      []byte `abi:"data"`
}

// decodeTriggerInfo decodes raw log data into a structured TriggerInfo
func decodeTriggerInfo(rawLog []byte) TriggerInfo {
	sections := chunkBytes(rawLog, 32)

	// TriggerID from section 4 (last 8 bytes of the 32-byte chunk)
	triggerIdSection := sections[3]
	triggerID := binary.BigEndian.Uint64(triggerIdSection[len(triggerIdSection)-8:])

	// Creator address from section 5 (last 20 bytes of the 32-byte chunk)
	creatorSection := sections[4]
	creatorAddress := types.MustAddressFromBytes(creatorSection[12:32]) // Ethereum addresses are 20 bytes

	// Data length from section 7 (last byte of the 32-byte chunk)
	dataLengthSection := sections[6]
	dataLength := int(dataLengthSection[31])

	// Actual data from section 8 (first dataLength bytes)
	dataSection := sections[7]
	data := make([]byte, dataLength)
	copy(data, dataSection[:dataLength])

	return TriggerInfo{
		TriggerID: triggerID,
		Creator:   creatorAddress,
		Data:      data,
	}
}

// chunkBytes divides a byte slice into chunks of specified size
func chunkBytes(data []byte, chunkSize int) [][]byte {
	chunksCount := (len(data) + chunkSize - 1) / chunkSize
	chunks := make([][]byte, chunksCount)

	for i := range chunks {
		start := i * chunkSize
		end := start + chunkSize
		if end > len(data) {
			end = len(data)
		}
		chunks[i] = data[start:end]
	}

	return chunks
}
