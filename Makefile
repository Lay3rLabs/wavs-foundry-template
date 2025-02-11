#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo
WAVS_CMD ?= docker run --network host --env-file ./.env -v $(shell pwd):/data ghcr.io/lay3rlabs/wavs:0.3.0-alpha5 wavs-cli

## build: building the project
build: _build_forge wasi-build

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
	@rm -rf cache
	@rm -rf out
	@rm -rf broadcast

## clean-docker: remove unused docker containers
clean-docker:
	@docker rm -v $(shell docker ps --filter status=exited -q) || true

## fmt: formatting solidity and rust code
fmt:
	@forge fmt --check
	@$(CARGO) fmt

## test: running tests
test:
	@forge test

## setup: install initial dependencies
setup:
	@forge install
	@npm install

## start-all: starting anvil and WAVS with docker compose
start-all: clean-docker
	@rm .docker/*.json || true
	@trap 'kill $(jobs -pr)' EXIT
# running anvil out of compose is a temp work around for MacOS
	@anvil &
	@docker compose up
	@wait

## deploy-contracts: deploying contracts with forge script | ANVIL_PRIVATE_KEY, RPC_URL, SERVICE_MANAGER
ANVIL_PRIVATE_KEY?=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
RPC_URL?=http://localhost:8545
SERVICE_MANAGER?=`jq -r '.eigen_service_managers.local | .[-1]' .docker/deployments.json`
deploy-contracts:
# `sudo chmod 0666 .docker/deployments.json`
	@forge script ./script/Deploy.s.sol ${SERVICE_MANAGER} --sig "run(string)" --rpc-url $(RPC_URL) --broadcast

## get-service-handler: getting the service handler address from the script deploy
get-service-handler-from-deploy:
	@jq -r '.service_handler' "./.docker/script_deploy.json"

## get-trigger: getting the trigger address from the script deploy
get-trigger-from-deploy:
	@jq -r '.trigger' "./.docker/script_deploy.json"

## wavs-cli: running wavs-cli in docker
wavs-cli:
	@$(WAVS_CMD) $(filter-out $@,$(MAKECMDGOALS))

## deploy-service: deploying the WAVS component service | WAVS_CLI_DATA, WAVS_CLI_HOME, WAVS_CLI_COMPONENT, TRIGGER_EVENT, TRIGGER_ADDR, SERVICE_HANDLER_ADDR, WAVS_SERVICE_CONFIG
deploy-service:
	@$(WAVS_CMD) deploy-service --log-level=info --data $(WAVS_CLI_DATA) --home $(WAVS_CLI_HOME) \
--component $(WAVS_CLI_COMPONENT) \
--trigger-event-name $(TRIGGER_EVENT) \
--trigger eth-contract-event \
--trigger-address $(TRIGGER_ADDR) \
--submit-address $(SERVICE_HANDLER_ADDR) \
--service-config='$(WAVS_SERVICE_CONFIG)'

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
