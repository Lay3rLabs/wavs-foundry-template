package main

// go run go.bytecodealliance.org/cmd/wit-bindgen-go generate -o internal/ ./docs:adder@0.1.0.wasm
import (
	adder "github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle/internal/example/component/example"
)

// Exports represents the caller-defined exports from "docs:adder/adder@0.1.0".
// var Exports struct {
// 	// Add represents the caller-defined, exported function "add".
// 	//
// 	//	add: func(x: s32, y: s32)
// 	Add func(x int32, y int32)
// }

func init() {
	adder.Exports.Add = func(x int32, y int32) int32 {
		return x + y
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
