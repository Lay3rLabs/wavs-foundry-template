#!/usr/bin/make -f

# Check if user is in docker group to determine if sudo is needed
SUDO := $(shell if groups | grep -q docker; then echo ''; else echo 'sudo'; fi)

# Default target is build
default: build

# Customize these variables
COMPONENT_FILENAME?=eth_price_oracle.wasm
SERVICE_CONFIG_FILE?=.docker/service.json

# Define common variables
CARGO=cargo
RECIPE := wasi-build
# the directory to build, or "" for all
WASI_BUILD_DIR ?= ""
WAVS_CMD ?= $(SUDO) docker run --rm --network host $$(test -f .env && echo "--env-file ./.env") -v $$(pwd):/data ghcr.io/lay3rlabs/wavs:latest wavs-cli
ANVIL_PRIVATE_KEY?=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
RPC_URL?=http://localhost:8545
SERVICE_MANAGER_ADDR?=`jq -r '.eigen_service_managers.local | .[-1]' .docker/deployments.json`
SERVICE_TRIGGER_ADDR?=`jq -r '.trigger' "./.docker/script_deploy.json"`
SERVICE_SUBMISSION_ADDR?=`jq -r '.service_handler' "./.docker/script_deploy.json"`
COIN_MARKET_CAP_ID?=1
MAKEFILE_DIRS := $(dir $(shell find components -name "Makefile" -o -name "makefile"))

## check-requirements: verify system requirements are installed
check-requirements: check-node check-jq check-cargo

## build: building the project
build: _build_forge wasi-build

## wasi-build: building WAVS wasi components
wasi-build:
	@for dir in $(MAKEFILE_DIRS); do \
		if grep -q "^$(RECIPE):" "$$dir/Makefile" 2>/dev/null || grep -q "^$(RECIPE):" "$$dir/makefile" 2>/dev/null; then \
			if [ "$(WASI_BUILD_DIR)" != "" ] && [[ "$$dir" != *"$(WASI_BUILD_DIR)"* ]]; then \
				continue; \
			fi; \
			echo "Running '$(RECIPE)' in $$dir"; \
			$(MAKE) -s -C "$$dir" $(RECIPE); \
		else \
			echo "Recipe '$(RECIPE)' not found in $$dir"; \
		fi; \
	done

## wasi-exec: executing the WAVS wasi component(s) | COMPONENT_FILENAME, COIN_MARKET_CAP_ID
wasi-exec:
	@$(WAVS_CMD) exec --log-level=info --data /data/.docker --home /data \
	--component "/data/compiled/$(COMPONENT_FILENAME)" \
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
	@$(SUDO) docker rm -v $(shell $(SUDO) docker ps -a --filter status=exited -q) 2> /dev/null || true

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
	@rm --interactive=never .docker/*.json 2> /dev/null || true

	@if [ "$(WAVS_IN_BACKGROUND)" = "true" ]; then \
		echo "Starting WAVS in the background"; \
		anvil & $(SUDO) docker compose up -d; \
	else \
		echo "Starting WAVS"; \
		bash -ec 'anvil & anvil_pid=$$!; trap "kill -9 $$anvil_pid 2>/dev/null" EXIT; $(SUDO) docker compose up; wait'; \
	fi

## get-service-handler: getting the service handler address from the script deploy
get-service-handler-from-deploy:
	@jq -r '.service_handler' "./.docker/script_deploy.json"

get-eigen-service-manager-from-deploy:
	@jq -r '.eigen_service_managers.local | .[-1]' .docker/deployments.json

## get-trigger: getting the trigger address from the script deploy
get-trigger-from-deploy:
	@jq -r '.trigger' "./.docker/script_deploy.json"

## wavs-cli: running wavs-cli in docker
wavs-cli:
	@$(WAVS_CMD) $(filter-out $@,$(MAKECMDGOALS))

## upload-component: uploading the WAVS component | COMPONENT_FILENAME
upload-component:
	@curl --silent -X POST http://127.0.0.1:8000/upload --data-binary @./compiled/$(COMPONENT_FILENAME) -H "Content-Type: application/wasm" | jq -r .digest

## deploy-service: deploying the WAVS component service json | SERVICE_CONFIG_FILE
deploy-service:
	@$(WAVS_CMD) deploy-service-raw --log-level=info --data /data/.docker --home /data --service `jq -c . < $(SERVICE_CONFIG_FILE)`

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
