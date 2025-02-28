package main

import (
	"encoding/hex"
	"fmt"
	"math/big"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-trigger-world"
	wavstypes "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-types"

	// _ "github.com/umbracle/ethgo" // TODO: just so it stays loaded
	"github.com/defiweb/go-eth/abi"
	types "github.com/defiweb/go-eth/types"
	"go.bytecodealliance.org/cm"
)

type Destination string

const (
	Ethereum  Destination = "Ethereum"
	CliOutput Destination = "CliOutput"
)

type NewTriggerEvent struct {
	TriggerInfo []byte `abi:"_triggerInfo"`
	// Add any other fields you need
}

type TriggerInfo struct {
	TriggerID *big.Int      `abi:"triggerId"`
	Creator   types.Address `abi:"creator"`
	Data      []byte        `abi:"data"`
}

// ITypes.sol
type DataWithID struct {
	TriggerID *big.Int `abi:"triggerId"`
	Data      []byte   `abi:"data"`
}

// [{"type":"event","name":"NewTrigger","inputs":[{"name":"_triggerInfo","type":"bytes","indexed":false,"internalType":"bytes"}],"anonymous":false}]
// type NewTriggerEvent struct {
// 	TriggerInfo []byte `abi:"_triggerInfo"` // TriggerInfo is the underlying i think? or is it bytes
// }

var (
	DataWithIdABI      = abi.MustParseStruct(`struct DataWithId { uint64 triggerId; bytes data; }`) // ITypes.sol
	TriggerInfoTypeABI = abi.MustParseStruct("struct TriggerInfo { uint64 triggerId; address creator; bytes data; }")
	NewTriggerEventABI = abi.MustParseEvent("event NewTrigger(bytes _triggerInfo)")
)

func init() {
	// Add custom type.
	// abi.Default.Types["TriggerID"] = abi.MustParseType("uint256")
	abi.Default.Types["DataWithID"] = abi.MustParseStruct(`struct DataWithId { uint64 triggerId; bytes data; }`)

	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) (result cm.Result[cm.List[uint8], cm.List[uint8], string]) {
		fmt.Println("This is an example print statement")

		trigger_id, req, dest := decode_trigger_event(triggerAction.Data)
		_ = trigger_id
		_ = req

		if trigger_id == 0 && dest != CliOutput {
			// not fully ready yet, return nothing
			fmt.Println("trigger_id == 0 && dest != CliOutput")
			nothing := []uint8{0x00}
			return cm.OK[cm.Result[cm.List[uint8], cm.List[uint8], string]](cm.NewList(&nothing[0], 0))
		}

		// output := fmt.Sprintf("Golang output: trigger_id: %d", trigger_id)
		// outputBytes := []uint8(output)
		// outputBytes := []uint8("testing 123")

		// bz := req.Slice()
		// fmt.Println("req bz as string:", string(bz)) // TODO: figure this out
		// bz := []byte("testoutput")
		bz := req.Slice()

		switch dest {
		case Ethereum:
			fmt.Println("Ethereum")

			bz := encode_trigger_output(trigger_id, bz) // TODO: change to the actual request output
			fmt.Println("encode_trigger_output", string(bz))

			// fmt.Println("encode_trigger_output", bz)
			// fmt.Println("encode_trigger_output", string(bz))
			successData := cm.NewList(&bz[0], len(bz))
			return cm.OK[cm.Result[cm.List[uint8], cm.List[uint8], string]](successData)
		case CliOutput:
			fmt.Println("CliOutput")
			successData := cm.NewList(&bz[0], len(bz))
			return cm.OK[cm.Result[cm.List[uint8], cm.List[uint8], string]](successData)
		}

		// if the trigger action is from ethereum, the output must be decoded / encoded

		return cm.OK[cm.Result[cm.List[uint8], cm.List[uint8], string]](cm.NewList(&bz[0], len(bz)))
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
		fmt.Println("Raw")
		return 0, *triggerAction.Raw(), CliOutput
	}

	// get the TriggerSourceEthContractEvent_ of the event

	// print out all of the information about triggerAction and return something early
	// fmt.Println("triggerAction", triggerAction)
	// return 0, cm.List[uint8]{}, ""

	if triggerAction.EthContractEvent() == nil {
		panic("decode_trigger_event triggerAction.EthContractEvent() is nil")
	}

	ethEvent := triggerAction.EthContractEvent()
	logData := ethEvent.Log.Data.Slice()

	allTopics := ethEvent.Log.Topics.Slice()
	topics := []types.Hash{}
	for _, t := range allTopics {
		topics = append(topics, types.MustHashFromBytes(t.Slice(), types.PadRight)) // TODO: try padding?
	}
	t := topics[0] // this is all that is used in the main example

	fmt.Println("ethEvent CA", ethEvent.ContractAddress)
	fmt.Println("ethEvent height:", ethEvent.BlockHeight)
	fmt.Println("ethEvent log data", logData)

	fmt.Println("topic", t, "bytes", t.Bytes())

	// decode event log data raw
	// logData := layertypes.EthEventLogData{
	// 	// Topics: ethEvent.Log.Topics,
	// 	Data: ethEvent.Log.Data,
	// }

	// decode the event from the given log object
	var triggerEvent NewTriggerEvent
	if err := NewTriggerEventABI.DecodeValues(topics, logData, &triggerEvent.TriggerInfo); err != nil {
		panic(fmt.Sprintf("NewTriggerEvent decode error: %v", err))
	}

	fmt.Println("triggerEvent", triggerEvent.TriggerInfo)
	fmt.Println("triggerEvent", string(triggerEvent.TriggerInfo))

	// Now decode the TriggerInfo struct from the encoded bytes
	var triggerInfo TriggerInfo
	if err := abi.DecodeValue(TriggerInfoTypeABI, triggerEvent.TriggerInfo, &triggerInfo); err != nil {
		println(fmt.Printf("err triggerInfo: %+v\n", triggerInfo))
		panic(fmt.Sprintf("TriggerInfo decode error: %v", err))
	}

	fmt.Printf("Trigger ID: %x\n", triggerInfo.TriggerID)
	fmt.Printf("Creator: %s\n", triggerInfo.Creator.String())
	fmt.Printf("Data: %x\n", triggerInfo.Data)

	// TODO: return back out if valid
	d := triggerInfo.Data
	return triggerInfo.TriggerID.Uint64(), cm.NewList(&d[0], len(d)), Ethereum

}

func main() {}

// addTrigger(bytes) input method to string decode
/*
hacky temp work around:
	asHex := hex.EncodeToString(logData)
	input := getStringValue(asHex)
	fmt.Println("input from data:", input)
*/
func getStringValue(hexData string) string {
	// Convert hex to bytes
	data, err := hex.DecodeString(hexData)
	if err != nil || len(data) < 225 {
		return ""
	}

	// Get the first byte of word 7, which contains the ASCII value
	// Word 7 starts at index 224 (0xE0) for addTrigger
	return string(data[224:225])
}
