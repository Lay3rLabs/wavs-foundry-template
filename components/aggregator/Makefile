OUTPUT_DIR?=../../compiled

# WASI_BUILD_DIR=components/aggregator make wasi-build

## wasi-build: build the aggregator WASI component
wasi-build:
	@echo "Building component: aggregator"
	@cargo component build --release; cargo fmt
	@mkdir -p $(OUTPUT_DIR)
	@cp ../../target/wasm32-wasip1/release/aggregator.wasm $(OUTPUT_DIR)
.PHONY: wasi-build
