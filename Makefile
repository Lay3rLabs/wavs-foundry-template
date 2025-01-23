#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo

## bindings: generating bindings
bindings:
# Generate new bindings
	@forge bind --bindings-path ./crates/bindings --crate-name bindings --overwrite \
		--alloy --alloy-version v0.9.2

## build: building the project
build: bindings
	@$(CARGO) build

## wasi-build: building the WAVS wasi component(s)
wasi-build:
	@for component in $(shell ls ./components); do \
		echo "Building component: $$component"; \
		(cd components/$$component; cargo component build --release); \
	done
	@mkdir -p ./compiled
	@cp ./target/wasm32-wasip1/release/*.wasm ./compiled/

## build-release: building the project in release mode
build-release: bindings
	@$(CARGO) build --release

## update-submodules: update the git submodules
update-submodules:
	@git submodule update --init --recursive

## clean: cleaning the project files
clean:
	@forge clean
	@$(CARGO) clean

## fmt: formatting solidity and rust code
fmt:
	@forge fmt --check
	@$(CARGO) fmt

## test: running forge and rust tests
test:
	@forge test
	@$(CARGO) test

## setup: installing forge dependencies
setup:
	@forge install


# Declare phony targets
.PHONY: build build-release clean fmt bindings test

.PHONY: help
help: Makefile
	@echo
	@echo " Choose a command run"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo
