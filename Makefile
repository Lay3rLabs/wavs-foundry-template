#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo

## bindings: generating bindings
bindings: _build_forge
# Generate new bindings
	@forge bind --bindings-path ./crates/bindings --crate-name bindings --overwrite \
		--alloy --alloy-version v0.9.2
	@$(CARGO) fmt --manifest-path ./crates/bindings/Cargo.toml

## build: building the project
build: _build_forge bindings wasi-build
	@$(CARGO) build --target-dir ./target --manifest-path ./app/Cargo.toml

## wasi-build: building the WAVS wasi component(s)
wasi-build:
	@for component in $(shell ls ./components); do \
		echo "Building component: $$component"; \
		(cd components/$$component; cargo component build --release); \
	done
	@mkdir -p ./compiled
	@cp ./target/wasm32-wasip1/release/*.wasm ./compiled/

## update-submodules: update the git submodules
update-submodules:
	@git submodule update --init --recursive

## clean: cleaning the project files
clean: clean-docker
	@forge clean
	@$(CARGO) clean

## clean-docker: remove unused docker containers
clean-docker:
	@docker rm -v $(shell docker ps --filter status=exited -q) || true

## fmt: formatting solidity and rust code
fmt:
	@forge fmt --check
	@$(CARGO) fmt

## test: running forge and rust tests
test:
	@forge test
	@$(CARGO) test --manifest-path ./app/Cargo.toml

## setup: installing forge dependencies
setup:
	@forge install

## start-all: starting anvil and WAVS with docker compose
start-all: clean-docker
	@trap 'kill $(jobs -pr)' EXIT
# running anvil out of compose is a temp work around for MacOS
	@anvil &
	@docker compose up
	@wait

_build_forge:
	@forge build

# Declare phony targets
.PHONY: build clean fmt bindings test

.PHONY: help
help: Makefile
	@echo
	@echo " Choose a command run"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' |  sed -e 's/^/ /'
	@echo
