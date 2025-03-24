WAVS_PACKAGE=wavs:worker@0.4.0-alpha.2
WAVS_WIT_WORLD=wavs:worker/layer-trigger-world
OUTPUT_DIR?=../../compiled

check-package:
# TODO: fix import to use the $(WAVS_PACKAGE) instead (same for golang)
	@if [ ! -f $(WAVS_PACKAGE).wasm ]; then \
		echo "Downloading WAVS package: $(WAVS_PACKAGE)"; \
		wkg get reecepbcups:wavs-testing@0.4.0-alpha.2 --overwrite --format wasm --output $(WAVS_PACKAGE).wasm; \
	fi
# TODO: figure out jco better than whatever this is
	@mkdir -p wit/deps/
	@if [ ! -f wit/$(WAVS_PACKAGE).wit ]; then \
		echo "Downloading WAVS package: $(WAVS_PACKAGE) wit"; \
		wkg get reecepbcups:wavs-testing@0.4.0-alpha.2 --overwrite --format wit --output wit/$(WAVS_PACKAGE).wit; \
	fi

build-bindings: check-package
	@npx jco types $(WAVS_PACKAGE).wasm --out-dir out/

download-wit-deps:
	@if [ ! -d wit/deps/*.wit ]; then \
		mkdir -p wit/deps; \
		wkg get wasi:io --overwrite --output=wit/deps/; \
		wkg get wasi:clocks --overwrite --output=wit/deps/; \
		wkg get wasi:http --overwrite --output=wit/deps/; \
		wkg get wasi:cli --overwrite --output=wit/deps/; \
		wkg get wasi:filesystem --overwrite --output=wit/deps/; \
		wkg get wasi:sockets --overwrite --output=wit/deps/; \
		wkg get wasi:random --overwrite --output=wit/deps/; \
	fi

## wasi-build: building the WAVS wasi component
wasi-build: build-bindings download-wit-deps
	@echo "Building component: js_eth_price_oracle"
	@npx tsc --outDir out/ --target es6 --strict --module preserve index.ts
	@npx esbuild ./index.js --bundle --outfile=out/out.js --platform=node --format=esm
	@npx jco componentize out/out.js --wit ./wit --world-name $(WAVS_WIT_WORLD) --out ../../compiled/js_eth_price_oracle.wasm


.PHONY: help
help: Makefile
	@echo
	@echo " Choose a command run"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo
