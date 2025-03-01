package main

import (
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"math/big"
	"strconv"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-trigger-world"
	wavstypes "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-types"

	"github.com/defiweb/go-eth/abi"
	types "github.com/defiweb/go-eth/types"
	"go.bytecodealliance.org/cm"
)

type Destination string

const (
	Ethereum  Destination = "Ethereum"
	CliOutput Destination = "CliOutput"
)

type TriggerInfo struct {
	TriggerID *big.Int      `abi:"triggerId"`
	Creator   types.Address `abi:"creator"`
	Data      []byte        `abi:"data"`
}

// ITypes.sol
var DataWithIdABI = abi.MustParseStruct(`struct DataWithId { uint64 triggerId; bytes data; }`)

type DataWithID struct {
	TriggerID uint64 `abi:"triggerId"`
	Data      []byte `abi:"data"`
}

func doComputation(inputReq []uint8, dest Destination) ([]byte, error) {
	// if dest is CliOuput, remove right padded bytes (0s)
	if dest == CliOutput {
		lastNonZero := -1
		for i := len(inputReq) - 1; i >= 0; i-- {
			if inputReq[i] != 0 {
				lastNonZero = i
				break
			}
		}

		// If we found a non-zero byte, trim after it
		// If all bytes are zero, we'll keep at least one zero byte
		if lastNonZero >= 0 {
			inputReq = inputReq[:lastNonZero+1]
		} else if len(inputReq) > 0 {
			inputReq = inputReq[:1] // Keep just one zero if all are zeros
		}
	}

	v, err := strconv.Atoi(string(inputReq))
	if err != nil {
		return nil, err
	}

	v = v * 2

	output := []byte(fmt.Sprintf("%d", v))
	return output, nil
}

func init() {
	type TriggerResult = cm.Result[wavs.OptionListU8Shape, cm.Option[cm.List[uint8]], string]

	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) TriggerResult {
		trigger_id, req, dest := decode_trigger_event(triggerAction.Data)
		reqInput := req.Slice()

		result, err := doComputation(reqInput, dest)
		if err != nil {
			return cm.Err[TriggerResult](err.Error())
		}
		fmt.Printf("doComputation Result: %v\n", string(result))

		switch dest {
		case Ethereum:
			bz := encode_trigger_output(trigger_id, result)
			fmt.Printf("Encoded output (raw): %x\n", bz)
			return cm.OK[TriggerResult](cm.Some(cm.NewList(&bz[0], len(bz))))
		case CliOutput:
			return cm.OK[TriggerResult](cm.Some(cm.NewList(&result[0], len(result))))
		default:
			panic("unknown destination, not supported")
		}
	}
}

// encode_trigger_output abi encodes the output of the computation to be sent back to the Ethereum contract
func encode_trigger_output(trigger_id uint64, output []byte) []byte {
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

func decode_trigger_event(triggerAction wavstypes.TriggerData) (trigger_id uint64, req cm.List[uint8], dest Destination) {
	if triggerAction.Raw() != nil {
		return 0, *triggerAction.Raw(), CliOutput
	}

	if triggerAction.EthContractEvent() == nil {
		panic("decode_trigger_event triggerAction.EthContractEvent() is nil")
	}

	ethEvent := triggerAction.EthContractEvent()

	triggerInfo := decodeTriggerInfo(ethEvent.Log.Data.Slice())

	fmt.Printf("Trigger ID: %v\n", triggerInfo.TriggerID)
	fmt.Printf("Creator: %s\n", triggerInfo.Creator.String())
	fmt.Printf("Input Data: %v\n", string(triggerInfo.Data))

	return triggerInfo.TriggerID.Uint64(), cm.NewList(&triggerInfo.Data[0], len(triggerInfo.Data)), Ethereum

}

func decodeTriggerInfo(rawLog []byte) TriggerInfo {
	// Parse the data into sections (each section is 32 bytes)
	sections := make([][]byte, 0)
	for i := 0; i < len(rawLog); i += 32 {
		end := i + 32
		if end > len(rawLog) {
			end = len(rawLog)
		}
		sec := rawLog[i:end]
		fmt.Println("sec", sec)
		sections = append(sections, sec)
	}

	triggerInfo := TriggerInfo{}

	// TriggerID from section 4 (last byte)
	triggerIdSection := sections[3]
	triggerInfo.TriggerID = big.NewInt(int64(binary.BigEndian.Uint32(triggerIdSection[len(triggerIdSection)-4:])))

	// Creator address from section 5
	creatorSection := sections[4]
	triggerInfo.Creator = types.MustAddressFromBytes(creatorSection[12:32]) // Ethereum addresses are 20 bytes

	// Data value
	// First get the data length from section 7
	dataLengthSection := sections[6]
	dataLength := int(dataLengthSection[31])

	// Then get the actual data from section 8
	dataSection := sections[7]
	triggerInfo.Data = make([]byte, dataLength)
	copy(triggerInfo.Data, dataSection[:dataLength])

	return triggerInfo
}

func main() {}
