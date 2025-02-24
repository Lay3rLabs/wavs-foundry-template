package main

import (
	"fmt"

	wavs "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/wavs/worker/layer-trigger-world"
	"go.bytecodealliance.org/cm"
)

func init() {
	wavs.Exports.Run = func(triggerAction wavs.TriggerAction) (result cm.Result[cm.List[uint8], cm.List[uint8], string]) {
		output := "golang output!"
		outputBytes := []uint8(output)

		fmt.Println("This is an example print statement")

		// if the trigger action is from ethereum, the output must be decoded / encoded

		successData := cm.NewList(&outputBytes[0], len(outputBytes))
		return cm.OK[cm.Result[cm.List[uint8], cm.List[uint8], string]](successData)
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
