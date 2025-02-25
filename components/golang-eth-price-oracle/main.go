package main

import (
	"fmt"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-trigger-world"
	wavstypes "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-types"

	submitcontract "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/submit" // TODO: wasm support
	// "github.com/ethereum/go-ethereum/accounts/abi"
	"go.bytecodealliance.org/cm"
)

func init() {
	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) (result cm.Result[cm.List[uint8], cm.List[uint8], string]) {
		fmt.Println("This is an example print statement")

		// trigger_id, _, dest := decode_trigger_event(triggerAction.Data)

		// output := fmt.Sprintf("Golang output: trigger_id: %d", trigger_id)
		// outputBytes := []uint8(output)
		outputBytes := []uint8("testing 123")

		dest := CliOutput

		switch dest {
		case Ethereum:
			v := submitcontract.SubmitContract{}
			fmt.Println("v", v)
			panic("Ethereum destination not supported yet")
		case CliOutput:
			successData := cm.NewList(&outputBytes[0], len(outputBytes))
			return cm.OK[cm.Result[cm.List[uint8], cm.List[uint8], string]](successData)
		}

		// if the trigger action is from ethereum, the output must be decoded / encoded

		return cm.OK[cm.Result[cm.List[uint8], cm.List[uint8], string]](cm.NewList(&outputBytes[0], len(outputBytes)))
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

func decode_trigger_event(triggerAction wavstypes.TriggerData) (trigger_id uint64, req cm.List[uint8], dest Destination) {
	// TODO: we just need to decode the event log data -> the type, nothing with networking here. a lightweight go-eth w/ wasm compatability would work here
	if triggerAction.EthContractEvent() != nil {
		ethEvent := triggerAction.EthContractEvent()
		// ethEvent.Log has Topics and Data. Decode data into the trigger struct

		fmt.Println("ethEvent", ethEvent)

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
		// return (trigger_info.triggerId, trigger_info.data.to_vec(), Destination::Ethereum)
	} else if triggerAction.Raw() != nil {
		return 0, *triggerAction.Raw(), CliOutput
	} else {
		// TODO: cosmos support
		panic("Unsupported trigger data type")
	}

	v := []uint8{0}
	emptyList := cm.NewList(&v[0], 0)
	return 0, emptyList, ""
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
