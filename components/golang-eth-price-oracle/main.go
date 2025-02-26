package main

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/big"
	"os"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-trigger-world"
	wavstypes "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-types"

	// _ "github.com/umbracle/ethgo" // TODO: just so it stays loaded
	"github.com/defiweb/go-eth/abi"
	types "github.com/defiweb/go-eth/types"
	"go.bytecodealliance.org/cm"
)

// cat out/ITypes.sol/ITypes.json | jq -r -c .abi
// const eventABI = `[{"type":"event","name":"NewTrigger","inputs":[{"name":"_triggerInfo","type":"bytes","indexed":false,"internalType":"bytes"}],"anonymous":false}]`

// cat out/IWavsTrigger.sol/ISimpleTrigger.json | jq -r -c .abi
// const inputContractABI = `[{"type":"function","name":"addTrigger","inputs":[{"name":"_data","type":"bytes","internalType":"bytes"}],"outputs":[],"stateMutability":"nonpayable"},{"type":"function","name":"getTrigger","inputs":[{"name":"_triggerId","type":"uint64","internalType":"ITypes.TriggerId"}],"outputs":[{"name":"_triggerInfo","type":"tuple","internalType":"struct ITypes.TriggerInfo","components":[{"name":"triggerId","type":"uint64","internalType":"ITypes.TriggerId"},{"name":"creator","type":"address","internalType":"address"},{"name":"data","type":"bytes","internalType":"bytes"}]}],"stateMutability":"view"},{"type":"function","name":"nextTriggerId","inputs":[],"outputs":[{"name":"_triggerId","type":"uint64","internalType":"ITypes.TriggerId"}],"stateMutability":"view"},{"type":"function","name":"triggerIdsByCreator","inputs":[{"name":"_creator","type":"address","internalType":"address"}],"outputs":[{"name":"_triggerIds","type":"uint64[]","internalType":"ITypes.TriggerId[]"}],"stateMutability":"view"},{"type":"function","name":"triggersById","inputs":[{"name":"_triggerId","type":"uint64","internalType":"ITypes.TriggerId"}],"outputs":[{"name":"_creator","type":"address","internalType":"address"},{"name":"_data","type":"bytes","internalType":"bytes"}],"stateMutability":"view"},{"type":"event","name":"NewTrigger","inputs":[{"name":"_triggerInfo","type":"bytes","indexed":false,"internalType":"bytes"}],"anonymous":false}]`

type TriggerInfo struct {
	TriggerID *big.Int      `abi:"triggerId"` // TODO: ?
	Creator   types.Address `abi:"creator"`
	Data      []byte        `abi:"data"`
}

// ITypes.sol
type DataWithID struct {
	TriggerID *big.Int `abi:"triggerId"`
	Data      []byte   `abi:"data"`
}

// [{"type":"event","name":"NewTrigger","inputs":[{"name":"_triggerInfo","type":"bytes","indexed":false,"internalType":"bytes"}],"anonymous":false}]
type NewTriggerEvent struct {
	TriggerInfo []byte `abi:"_triggerInfo"` // TriggerInfo is the underlying i think? or is it bytes
}

var (
	DataWithIdABI      = abi.MustParseStruct(`struct DataWithId { uint256 triggerId; bytes data; }`) // ITypes.sol
	NewTriggerEventABI = abi.MustParseEvent("NewTrigger(bytes _triggerInfo)")
	TriggerInfoTypeABI = abi.MustParseStruct("struct TriggerInfo { uint256 triggerId; address creator; bytes data; }")
)

func init() {
	// Add custom type.
	abi.Default.Types["TriggerID"] = abi.MustParseType("uint256")
	abi.Default.Types["DataWithID"] = abi.MustParseStruct(`struct DataWithId { TriggerID triggerId; bytes data; }`)

	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) (result cm.Result[cm.List[uint8], cm.List[uint8], string]) {
		fmt.Println("This is an example print statement")

		trigger_id, req, dest := decode_trigger_event(triggerAction.Data)

		// output := fmt.Sprintf("Golang output: trigger_id: %d", trigger_id)
		// outputBytes := []uint8(output)
		// outputBytes := []uint8("testing 123")

		bz := req.Slice()
		fmt.Println("req bz as string:", string(bz)) // TODO: figure this out

		switch dest {
		case Ethereum:
			fmt.Println("Ethereum")

			bz := encode_trigger_output(trigger_id, bz) // TODO: change to the actual request output
			fmt.Println("encode_trigger_output", bz)
			fmt.Println("encode_trigger_output", string(bz))
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

// pub fn decode_trigger_event(trigger_data: TriggerData) -> Result<(u64, Vec<u8>, Destination)> {
// 	match trigger_data {
// 		TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
// 			let event: solidity::NewTrigger = decode_event_log_data!(log)?;
// 			let trigger_info = solidity::TriggerInfo::abi_decode(&event._triggerInfo, false)?;
// 			Ok((trigger_info.triggerId, trigger_info.data.to_vec(), Destination::Ethereum))
// 		}
// 		TriggerData::Raw(data) => Ok((0, data.clone(), Destination::CliOutput)),
// 		_ => Err(anyhow::anyhow!("Unsupported trigger data type")),
// 	}
// }

type Destination string

const (
	Ethereum  Destination = "Ethereum"
	CliOutput Destination = "CliOutput"
)

// TODO: if json decoding does not work, ffjson may work. talk to Ethan about this if it is a problem. json encode/decode does not out of

// DecodeEventLogData decodes event log data into the provided event struct.
// The event struct should have appropriate `abi` tags for mapping fields.
//
// Parameters:
// - event: An abi.Event parsed from an event signature
// - logData: An Ethereum log containing Topics and Data fields
// - result: A pointer to a struct where the decoded event will be stored
//
// Returns an error if decoding fails.
func DecodeEventLogData(event abi.Event, logData types.Log, result interface{}) error {
	return event.DecodeValues(logData.Topics, logData.Data, result)
}

// DecodeTypedEventLogData is a generic version that handles the type conversion.
// T must be a struct type that can receive the event data.
func DecodeTypedEventLogData[T any](event abi.Event, logData types.Log) (T, error) {
	var result T
	err := event.DecodeValues(logData.Topics, logData.Data, &result)
	return result, err
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

	//! TODO: I need a triggerAction that is a NewTriggerEvent yea? how does this work for? or at least how to get more info than I can currently

	if triggerAction.EthContractEvent() != nil {
		fmt.Println("EthContractEvent")
		ethEvent := triggerAction.EthContractEvent()
		logTopics := ethEvent.Log.Topics.Data().Slice()

		// ethEvent.Log has Topics and Data. Decode data into the trigger struct

		fmt.Println("ethEvent CA", ethEvent.ContractAddress)
		fmt.Println("ethEvent height:", ethEvent.BlockHeight)
		fmt.Println("ethEvent log data", ethEvent.Log.Data.Slice())

		// ! decode input value from data (works do not touch)
		var logData []byte = ethEvent.Log.Data.Slice()
		asHex := hex.EncodeToString(logData)
		input := getStringValue(asHex)
		fmt.Println("input from data:", input)
		// ! decode input value from data (works do not touch)

		// TODO: decode event log data -> NewTrigger type here
		// TODO::::::::::::::::::::::::::::::::::::::::::::::::::::: This is only gettin the calldata and not actually decoding the entire thing, not enbough input?
		var triggerEvent NewTriggerEvent
		if err := NewTriggerEventABI.DecodeValues([]types.Hash{types.MustHashFromBytes(logTopics, types.PadNone)}, ethEvent.Log.Data.Slice(), &triggerEvent.TriggerInfo); err != nil {
			fmt.Printf("Error decoding trigger event: %v\n", err)
			return
		}

		fmt.Println("ti raw", triggerEvent)
		fmt.Println("ti string", string(triggerEvent.TriggerInfo)) // TODO: convert this into a nested type i think

		var triggerInfo TriggerInfo
		if err := abi.DecodeValue(TriggerInfoTypeABI, triggerEvent.TriggerInfo, &triggerInfo); err != nil {
			fmt.Printf("Error decoding trigger info: %v\n", err)
			return
		}

		// Now you can use the decoded triggerInfo
		fmt.Printf("Trigger ID: %x\n", triggerInfo.TriggerID)
		fmt.Printf("Creator: %s\n", triggerInfo.Creator.String())
		fmt.Printf("Data: %x\n", triggerInfo.Data)

		// panic(7777)

		// submitcontract.SubmitContractNewTrigger
		// var trigger submitcontract.SubmitContractNewTrigger
		// we want to get TriggerInfo out of the ethEvent Log data
		// decode the triggerAction.EthContractEvent().Log into this

		// ethEvent.Log.Data

		// tokenAbi, err := abi.JSON(strings.NewReader(string(submitcontract.SubmitContractABI)))
		// if err != nil {
		// 	log.Fatal(err)
		// }

		// fmt.Println("tokenAbi", tokenAbi)

		// // convert ethEvent.Log.Data to bytes
		// data := ethEvent.Log.Data.Slice()
		// if err := tokenAbi.UnpackIntoInterface(&trigger, "NewTrigger", data); err != nil {
		// 	log.Fatal(err)
		// }

		// // print out data
		// fmt.Println("trigger & data", trigger, data)

		// decode event log data
		// decode trigger info
		return triggerInfo.TriggerID.Uint64(), triggerAction.EthContractEvent().Log.Data, Ethereum
	} else {
		// TODO: cosmos support
		panic("Unsupported trigger data type")
	}

	v := []uint8{0}
	emptyList := cm.NewList(&v[0], 0)
	return 0, emptyList, ""
}

func loadABI(path string) ([]byte, error) {
	abiData, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	type ABIData struct {
		ABI []json.RawMessage `json:"abi"`
	}

	var abiDataStruct ABIData
	err = json.Unmarshal(abiData, &abiDataStruct)
	if err != nil {
		return nil, err
	}

	// back to bytes
	var bz []byte
	for _, raw := range abiDataStruct.ABI {
		bz = append(bz, raw...)
	}

	return bz, nil
}

// func parseABI(a []byte) (abi.ABI, error) {
// 	var parsedABI abi.ABI
// 	err := json.Unmarshal(a, &parsedABI)
// 	if err != nil {
// 		return abi.ABI{}, err
// 	}
// 	return parsedABI, nil
// }

// main is required for the `wasi` target, even if it isn't used.
func main() {
	// rawBzNum1 := `0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 32 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 192 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 32 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 5 0 0 0 0 0 0 0 0 0 0 0 0 243 159 214 229 26 173 136 246 244 206 106 184 130 114 121 207 255 185 34 102 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 96 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 49 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0`
	// encodedInput1 := convertStrToBytes(rawBzNum1)

	// rawBzNum3 := `0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 32 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 192 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 32 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 4 0 0 0 0 0 0 0 0 0 0 0 0 243 159 214 229 26 173 136 246 244 206 106 184 130 114 121 207 255 185 34 102 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 96 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 51 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0`
	// bz3 := convertStrToBytes(rawBzNum3)

	// // decode_trigger_event(wavstypes.TriggerData{})

	// s := getStringValue(hex.EncodeToString(bz3))
	// fmt.Println("s", s)
	// panic(8888)

	// // The ABI snippet containing just the NewTrigger event from the out/ dir

	// // this was gotten from `fmt.Println("bz str", string(ethEvent.Log.Data.Slice()))`

	// decodeABI(encodedInput1)

}

// func convertStrToBytes(bz string) []uint8 {
// 	// iterate over each number, split by space. Convert it to bytes

// 	// split by space
// 	split := strings.Split(bz, " ")

// 	// convert to bytes
// 	var bytes []uint8
// 	for _, s := range split {
// 		// convert s to a number
// 		num, err := strconv.Atoi(s)
// 		if err != nil {
// 			panic(err)
// 		}

// 		// convert number to byte
// 		bytes = append(bytes, uint8(num))

// 	}

// 	fmt.Println("Successfully decoded the trigger event", string(bytes))

// 	// convert to hex
// 	hexEncoded := hex.EncodeToString(bytes)
// 	fmt.Println("hex", hexEncoded)

// 	return bytes
// }

// func decodeABI(encodedInput1 []uint8) {

// 	// Parse the ABI
// 	parsedABI, err := abi.JSON(strings.NewReader(inputContractABI))
// 	if err != nil {
// 		log.Fatalf("Failed to parse ABI: %v", err)
// 	}

// 	fmt.Println("Successfully parsed the ABI")

// 	// print parsedABI
// 	fmt.Println("parsedABI", parsedABI)

// 	t := "addTrigger"

// 	// Get the event definition
// 	// eventDef, exists := parsedABI.Events["NewTrigger"]

// 	// addTrigger
// 	eventDef, exists := parsedABI.Methods[t]
// 	if !exists {
// 		log.Fatal("Event 'NewTrigger' not found in ABI")
// 	}

// 	// Print event details
// 	fmt.Printf("Name: %s\n", eventDef.Name)
// 	fmt.Printf("Inputs: %v\n", eventDef.Inputs)
// 	fmt.Printf("Number of Inputs: %d\n", len(eventDef.Inputs))

// 	exampleLog := types.Log{
// 		// Address:     common.HexToAddress("0x123456789abcdef0123456789abcdef012345678"),
// 		// Topics:      []common.Hash{{}},
// 		Data: encodedInput1,
// 		// BlockNumber: 123456,
// 	}

// 	// var number uint64

// 	// var decodedEvent struct {
// 	// 	TriggerInfo []uint8
// 	// }
// 	// err = parsedABI.UnpackIntoInterface(&decodedEvent, "NewTrigger", exampleLog.Data)

// 	// prijnt out exampleLog.Data
// 	// fmt.Println("exampleLog.Data", string(exampleLog.Data)) // exampleLog.Data  � ��������j��ry���"f`1
// 	// decode this into a decoded inputs type

// 	// parse ABI into the addTrigger(bytes)
// 	var decodedInputs struct {
// 		Data []uint8 `abi:"_data"`
// 	}

// 	err = parsedABI.UnpackIntoInterface(&decodedInputs, t, exampleLog.Data)
// 	if err != nil {
// 		log.Fatal(err)
// 	}

// 	fmt.Println("decodedInputs", decodedInputs)

// }

// addTrigger(bytes) input method to string decode
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
