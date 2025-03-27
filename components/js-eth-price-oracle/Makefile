WAVS_PACKAGE=wavs:worker@0.4.0-alpha.2
WAVS_WIT_WORLD=wavs:worker/layer-trigger-world
OUTPUT_DIR?=../../compiled

check-package:
# TODO: fix import to use the $(WAVS_PACKAGE) instead (same for golang)
	@if [ ! -f $(WAVS_PACKAGE).wasm ]; then \
		echo "Downloading WAVS package: $(WAVS_PACKAGE)"; \
		wkg get reecepbcups:wavs-testing@0.4.0-alpha.2 --overwrite --format wasm --output $(WAVS_PACKAGE).wasm; \
	fi

# converts the entire .wasm package into a single .wit file, easily consumable by the jco command
convert-wasm-to-wit:
	@wasm-tools component wit $(WAVS_PACKAGE).wasm -o $(WAVS_PACKAGE).wit

## build-bindings: building the WAVS bindings
build-bindings: check-package
	@npx jco types $(WAVS_PACKAGE).wasm --out-dir out/

## wasi-build: building the WAVS wasi component
wasi-build: build-bindings convert-wasm-to-wit
	@echo "Building component: js_eth_price_oracle"
	@npx tsc --outDir out/ --target es6 --strict --module preserve index.ts
	@npx esbuild ./index.js --bundle --outfile=out/out.js --platform=node --format=esm
	@npx jco componentize out/out.js --wit $(WAVS_PACKAGE).wit --world-name $(WAVS_WIT_WORLD) --out ../../compiled/js_eth_price_oracle.wasm

.PHONY: help
help: Makefile
	@echo
	@echo " Choose a command run"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo
