#!/usr/bin/make -f

# Check if user is in docker group to determine if sudo is needed
SUDO := $(shell if groups | grep -q docker; then echo ''; else echo 'sudo'; fi)

# Default target is build
default: build

# Customize these variables
COMPONENT_FILENAME=eth_price_oracle.wasm
TRIGGER_EVENT="NewTrigger(bytes)"
SERVICE_CONFIG='{"fuel_limit":100000000,"max_gas":5000000,"host_envs":[],"kv":[],"workflow_id":"default","component_id":"default"}'

# Define common variables
CARGO=cargo
WAVS_CMD ?= $(SUDO) docker run --rm --network host $$(test -f .env && echo "--env-file ./.env") -v $$(pwd):/data ghcr.io/lay3rlabs/wavs:0.3.0-rc1 wavs-cli
ANVIL_PRIVATE_KEY?=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
RPC_URL?=http://localhost:8545
SERVICE_MANAGER_ADDR?=`jq -r '.eigen_service_managers.local | .[-1]' .docker/deployments.json`
SERVICE_TRIGGER_ADDR?=`jq -r '.trigger' "./.docker/script_deploy.json"`
SERVICE_SUBMISSION_ADDR?=`jq -r '.service_handler' "./.docker/script_deploy.json"`
COIN_MARKET_CAP_ID?=1

## check-requirements: verify system requirements are installed
check-requirements: check-node check-jq check-cargo

## build: building the project
build: _build_forge wasi-build

## wasi-build: building the WAVS wasi component(s)
wasi-build:
	@for component in $(shell ls ./components); do \
		echo "Building component: $$component"; \
		(cd components/$$component; cargo component build --release; cargo fmt); \
	done
	@mkdir -p ./compiled
	@cp ./target/wasm32-wasip1/release/*.wasm ./compiled/

## wasi-exec: executing the WAVS wasi component(s) | COMPONENT_FILENAME, COIN_MARKET_CAP_ID
wasi-exec:
	@$(WAVS_CMD) exec --log-level=info --data /data/.docker --home /data \
	--component "/data/compiled/${COMPONENT_FILENAME}" \
	--input `cast format-bytes32-string $(COIN_MARKET_CAP_ID)`

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
	@$(SUDO) docker rm -v $(shell $(SUDO) docker ps --filter status=exited -q) || true

## fmt: formatting solidity and rust code
fmt:
	@forge fmt --check
	@$(CARGO) fmt

## test: running tests
test:
	@forge test

## setup: install initial dependencies
setup: check-requirements
	@forge install
	@npm install

## start-all: starting anvil and WAVS with docker compose
# running anvil out of compose is a temp work around for MacOS
start-all: clean-docker setup-env
	@rm --interactive=never .docker/*.json || true
	@bash -ec 'anvil & anvil_pid=$$!; trap "kill -9 $$anvil_pid 2>/dev/null" EXIT; $(SUDO) docker compose up; wait'

## deploy-contracts: deploying the contracts | SERVICE_MANAGER_ADDR, RPC_URL
deploy-contracts:
# `sudo chmod 0666 .docker/deployments.json`
	@forge script ./script/Deploy.s.sol ${SERVICE_MANAGER_ADDR} --sig "run(string)" --rpc-url $(RPC_URL) --broadcast

## get-service-handler: getting the service handler address from the script deploy
get-service-handler-from-deploy:
	@jq -r '.service_handler' "./.docker/script_deploy.json"

## get-trigger: getting the trigger address from the script deploy
get-trigger-from-deploy:
	@jq -r '.trigger' "./.docker/script_deploy.json"

## wavs-cli: running wavs-cli in docker
wavs-cli:
	@$(WAVS_CMD) $(filter-out $@,$(MAKECMDGOALS))

## deploy-service: deploying the WAVS component service | COMPONENT_FILENAME, TRIGGER_EVENT, SERVICE_TRIGGER_ADDR, SERVICE_SUBMISSION_ADDR, SERVICE_CONFIG
deploy-service:
	@$(WAVS_CMD) deploy-service --log-level=info --data /data/.docker --home /data \
	--component "/data/compiled/${COMPONENT_FILENAME}" \
	--trigger-event-name ${TRIGGER_EVENT} \
	--trigger-address "${SERVICE_TRIGGER_ADDR}" \
	--submit-address "${SERVICE_SUBMISSION_ADDR}" \
	--service-config ${SERVICE_CONFIG}

## trigger-service: triggering the service | SERVICE_TRIGGER_ADDR, COIN_MARKET_CAP_ID, RPC_URL
trigger-service:
	@forge script ./script/Trigger.s.sol ${SERVICE_TRIGGER_ADDR} ${COIN_MARKET_CAP_ID} --sig "run(string,string)" --rpc-url $(RPC_URL) --broadcast -v 4

## show-result: showing the result | SERVICE_TRIGGER_ADDR, SERVICE_SUBMISSION_ADDR, RPC_URL
show-result:
	@forge script ./script/ShowResult.s.sol ${SERVICE_TRIGGER_ADDR} ${SERVICE_SUBMISSION_ADDR} --sig "run(string,string)" --rpc-url $(RPC_URL) --broadcast -v 4

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

# helpers

.PHONY: setup-env
setup-env:
	@if [ ! -f .env ]; then \
		if [ -f .env.example ]; then \
			echo "Creating .env file from .env.example..."; \
			cp .env.example .env; \
			echo ".env file created successfully!"; \
		fi; \
	fi

# check versions

check-command:
	@command -v $(1) > /dev/null 2>&1 || (echo "Command $(1) not found. Please install $(1), reference the System Requirements section"; exit 1)

.PHONY: check-node
check-node:
	@$(call check-command,node)
	@NODE_VERSION=$$(node --version); \
	MAJOR_VERSION=$$(echo $$NODE_VERSION | sed 's/^v\([0-9]*\)\..*/\1/'); \
	if [ $$MAJOR_VERSION -lt 21 ]; then \
		echo "Error: Node.js version $$NODE_VERSION is less than the required v21."; \
		echo "Please upgrade Node.js to v21 or higher."; \
		exit 1; \
	fi

.PHONY: check-jq
check-jq:
	@$(call check-command,jq)

.PHONY: check-cargo
check-cargo:
	@$(call check-command,cargo)
