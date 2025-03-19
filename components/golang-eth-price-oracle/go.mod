module github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle

go 1.23.1

replace github.com/dev-wasm/dev-wasm-go/lib => github.com/Reecepbcups/dev-wasm-go/lib v0.0.0-20250302004559-caf3bb14c8e6

replace github.com/Lay3rLabs/wavs-wasi/go => /home/reece/Desktop/Programming/Rust/wavs-wasi/go

require (
	github.com/Lay3rLabs/wavs-wasi/go v0.0.0-20250312161606-bae6ec979f97
	github.com/dev-wasm/dev-wasm-go/lib v0.0.0
	go.bytecodealliance.org/cm v0.1.1-0.20250218151459-e57ac0139b6f
)

require (
	github.com/btcsuite/btcd/btcec/v2 v2.3.2 // indirect
	github.com/decred/dcrd/dcrec/secp256k1/v4 v4.2.0 // indirect
	github.com/defiweb/go-anymapper v0.3.0 // indirect
	github.com/defiweb/go-eth v0.7.0 // indirect
	github.com/defiweb/go-rlp v0.3.0 // indirect
	github.com/defiweb/go-sigparser v0.6.0 // indirect
	github.com/stretchr/testify v1.9.0 // indirect
	golang.org/x/crypto v0.22.0 // indirect
	golang.org/x/sys v0.22.0 // indirect
)
