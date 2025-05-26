module github.com/Lay3rLabs/wavs-foundry-template/components/golang-evm-price-oracle

go 1.24.3

// namespace import fix
replace github.com/dev-wasm/dev-wasm-go/lib => github.com/Reecepbcups/dev-wasm-go/lib v0.0.0-20250302004559-caf3bb14c8e6

// tinygo >0.35 support
replace (
	github.com/defiweb/go-eth => github.com/Reecepbcups/go-eth v0.7.1-0.20250320155602-e7f53502e2df
	// fix: assignment mismatch: 3 variables but rlp.Decode returns 2 values
	github.com/defiweb/go-rlp => github.com/defiweb/go-rlp v0.3.0
)

require (
	github.com/Lay3rLabs/wavs-wasi/go v0.4.0-beta.4
	github.com/dev-wasm/dev-wasm-go/lib v0.0.0
	go.bytecodealliance.org/cm v0.2.2
)

require (
	github.com/btcsuite/btcd/btcec/v2 v2.3.2 // indirect
	github.com/decred/dcrd/dcrec/secp256k1/v4 v4.4.0 // indirect
	github.com/defiweb/go-anymapper v0.3.0 // indirect
	github.com/defiweb/go-eth v0.7.0 // indirect
	github.com/defiweb/go-rlp v0.3.0 // indirect
	github.com/defiweb/go-sigparser v0.6.0 // indirect
	github.com/stretchr/testify v1.10.0 // indirect
	golang.org/x/crypto v0.36.0 // indirect
	golang.org/x/sys v0.31.0 // indirect
)
