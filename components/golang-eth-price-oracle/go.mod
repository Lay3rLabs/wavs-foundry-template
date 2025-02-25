module github.com/Lay3rLabs/wavs-foundry-template/components/golang-eth-price-oracle

go 1.23.1

// builds off of v1.14.13 but removes any references to github.com/gorilla/websocket (lazily)
// https://github.com/Reecepbcups/go-ethereum/pull/1
// replace github.com/ethereum/go-ethereum => github.com/reecepbcups/go-ethereum v1.14.14-0.20250225180615-656acb9d2a26
replace github.com/ethereum/go-ethereum => /home/reece/Desktop/Programming/Go/go-ethereum

require (
	github.com/ethereum/go-ethereum v1.14.13
	go.bytecodealliance.org/cm v0.1.1-0.20250218151459-e57ac0139b6f
)

require (
	github.com/Microsoft/go-winio v0.6.2 // indirect
	github.com/StackExchange/wmi v1.2.1 // indirect
	github.com/bits-and-blooms/bitset v1.13.0 // indirect
	github.com/consensys/bavard v0.1.13 // indirect
	github.com/consensys/gnark-crypto v0.12.1 // indirect
	github.com/crate-crypto/go-ipa v0.0.0-20240223125850-b1e8a79f509c // indirect
	github.com/crate-crypto/go-kzg-4844 v1.0.0 // indirect
	github.com/deckarep/golang-set/v2 v2.6.0 // indirect
	github.com/decred/dcrd/dcrec/secp256k1/v4 v4.0.1 // indirect
	github.com/ethereum/c-kzg-4844 v1.0.0 // indirect
	github.com/ethereum/go-verkle v0.1.1-0.20240829091221-dffa7562dbe9 // indirect
	github.com/fsnotify/fsnotify v1.6.0 // indirect
	github.com/go-ole/go-ole v1.3.0 // indirect
	github.com/google/uuid v1.3.0 // indirect
	github.com/holiman/uint256 v1.3.1 // indirect
	github.com/mmcloughlin/addchain v0.4.0 // indirect
	github.com/shirou/gopsutil v3.21.4-0.20210419000835-c7a38de76ee5+incompatible // indirect
	github.com/supranational/blst v0.3.13 // indirect
	github.com/tklauser/go-sysconf v0.3.12 // indirect
	github.com/tklauser/numcpus v0.6.1 // indirect
	golang.org/x/crypto v0.22.0 // indirect
	golang.org/x/exp v0.0.0-20231110203233-9a3e6036ecaa // indirect
	golang.org/x/sync v0.7.0 // indirect
	golang.org/x/sys v0.22.0 // indirect
	rsc.io/tmplfunc v0.0.3 // indirect
)
