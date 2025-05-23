WAVS_PACKAGE=wavs:worker@0.4.0-beta.4
WAVS_WIT_WORLD=wavs:worker/layer-trigger-world
# get the first line of the go.mod and get the last element after the last /, then replace - with _
GO_MOD_NAME?=$(shell head -n 1 go.mod | cut -d' ' -f2 | rev | cut -d'/' -f1 | rev | tr '-' '_')

OUTPUT_DIR?=../../compiled

check-package:
	@if [ ! -f $(WAVS_PACKAGE).wasm ]; then \
		echo "Downloading WAVS package: $(WAVS_PACKAGE)"; \
		wkg get ${WAVS_PACKAGE} --overwrite --format wasm --output ${WAVS_PACKAGE}.wasm; \
	fi

## wasi-build: building the WAVS wasi component
wasi-build: check-package
	@echo "Building component: $(GO_MOD_NAME) [takes a few seconds...]"
	@go mod tidy
	@mkdir -p $(OUTPUT_DIR)
	@tinygo build -target=wasip2 -o $(OUTPUT_DIR)/$(GO_MOD_NAME).wasm --wit-package $(WAVS_PACKAGE).wasm --wit-world $(WAVS_WIT_WORLD) ./src
	@echo "WASI component built: $(GO_MOD_NAME).wasm"
.PHONY: wasi-build

.PHONY: help
help: Makefile
	@echo
	@echo " Choose a command run"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo
