WAVS_PACKAGE=wavs:worker@0.3.0-rc1
WAVS_WIT_WORLD=wavs:worker/layer-trigger-world
OUTPUT_DIR?=../../compiled

## wasi-build: building the WAVS wasi component
wasi-build:
	@echo "Building component: js_eth_price_oracle"
	@npx jco types $(WAVS_PACKAGE).wasm --out-dir out/
	@npx tsc --outDir out/ --target es6 --strict --module preserve index.ts
	@npx esbuild ./index.js --bundle --outfile=out/out.js --platform=node --format=esm
	@npx jco componentize out/out.js --wit /home/reece/Desktop/Programming/Rust/wavs/sdk/wit/ --world-name wavs:worker/layer-trigger-world --out ../../compiled/js_eth_price_oracle.wasm


.PHONY: help
help: Makefile
	@echo
	@echo " Choose a command run"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo
