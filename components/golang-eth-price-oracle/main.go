package main

import (
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
var DataWithIdABI = abi.MustParseStruct(`struct DataWithId { uint256 triggerId; bytes data; }`)

type DataWithID struct {
	TriggerID *big.Int `abi:"triggerId"`
	Data      []byte   `abi:"data"`
}

func doComputation(inputReq []uint8, dest Destination) ([]byte, error) {
	// if dest is CliOuput, remove right padded bytes (0s)
	if dest == CliOutput {
		for i := len(inputReq) - 1; i >= 0; i-- {
			if inputReq[i] != 0 {
				inputReq = inputReq[:i+1]
				break
			}
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
	type TriggerResult = cm.Result[cm.List[uint8], cm.List[uint8], string]

	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) TriggerResult {
		fmt.Println("This is an example print statement")

		trigger_id, req, dest := decode_trigger_event(triggerAction.Data)
		reqInput := req.Slice()

		result, err := doComputation(reqInput, dest)
		if err != nil {
			return cm.Err[TriggerResult](err.Error())
		}

		switch dest {
		case Ethereum:
			bz := encode_trigger_output(trigger_id, result)
			return cm.OK[TriggerResult](cm.NewList(&bz[0], len(bz)))
		case CliOutput:
			return cm.OK[TriggerResult](cm.NewList(&reqInput[0], len(reqInput)))
		default:
			panic("unknown destination, not supported")
		}
	}
}

func encode_trigger_output(trigger_id uint64, output []byte) []byte {
	return abi.MustEncodeValue(DataWithIdABI, DataWithID{
		TriggerID: big.NewInt(int64(trigger_id)),
		Data:      output,
	})
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

	fmt.Printf("Trigger ID: %x\n", triggerInfo.TriggerID)
	fmt.Printf("Creator: %s\n", triggerInfo.Creator.String())
	fmt.Printf("Data: %x\n", triggerInfo.Data)

	triggerId := uint64(0) // TODO: figure this out w/ triggerInfo.TriggerID

	return triggerId, cm.NewList(&triggerInfo.Data[0], len(triggerInfo.Data)), Ethereum

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
	triggerInfo.TriggerID = big.NewInt(int64(triggerIdSection[31])) // The last byte contains the ID // TODO: fix me

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
