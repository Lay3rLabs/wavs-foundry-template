OUTPUT_DIR?=../../compiled

# WASI_BUILD_DIR=components/evm-price-oracle make wasi-build

## wasi-build: building the WAVS wasi component(s)
wasi-build:
	@echo "Building component: evm-price-oracle"
	@cargo component build --release; cargo fmt
	@mkdir -p $(OUTPUT_DIR)
	@cp ../../target/wasm32-wasip1/release/*.wasm $(OUTPUT_DIR)
.PHONY: wasi-build
