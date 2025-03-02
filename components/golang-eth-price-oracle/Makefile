COMPONENT_FILENAME=eth_price_oracle.wasm

## wasi-build: building the WAVS wasi component(s)
wasi-build:
	@for component in $(shell ls ./components); do \
		[ -f components/$$component/Cargo.toml ] || continue; \
		echo "Building component: $$component"; \
		(cd components/$$component; cargo component build --release; cargo fmt); \
	done
	@mkdir -p ./compiled
	@cp ./target/wasm32-wasip1/release/*.wasm ./compiled/


.PHONY: help
help: Makefile
	@echo
	@echo " Choose a command run"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo
