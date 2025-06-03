#!/usr/bin/make -f

# Check if user is in docker group to determine if sudo is needed
SUDO := $(shell if groups | grep -q docker; then echo ''; else echo 'sudo'; fi)

# Define common variables
CARGO=cargo
COIN_MARKET_CAP_ID?=1
COMPONENT_FILENAME?=evm_price_oracle.wasm
CREDENTIAL?=""
DOCKER_IMAGE?=ghcr.io/lay3rlabs/wavs:35c96a4
MIDDLEWARE_DOCKER_IMAGE?=ghcr.io/lay3rlabs/wavs-middleware:79dffa2
IPFS_ENDPOINT?=http://127.0.0.1:5001
RPC_URL?=http://127.0.0.1:8545
SERVICE_FILE?=.docker/service.json
SERVICE_SUBMISSION_ADDR?=`jq -r .deployedTo .docker/submit.json`
SERVICE_TRIGGER_ADDR?=`jq -r .deployedTo .docker/trigger.json`
WASI_BUILD_DIR ?= ""
ENV_FILE?=.env
WAVS_CMD ?= $(SUDO) docker run --rm --network host $$(test -f ${ENV_FILE} && echo "--env-file ./${ENV_FILE}") -v $$(pwd):/data ${DOCKER_IMAGE} wavs-cli
WAVS_ENDPOINT?="http://127.0.0.1:8000"
-include ${ENV_FILE}

# Default target is build
default: build

## build: building the project
build: _build_forge wasi-build

## wasi-build: building WAVS wasi components | WASI_BUILD_DIR
wasi-build:
	@echo "🔨 Building WASI components..."
	@./script/build_components.sh $(WASI_BUILD_DIR)
	@echo "✅ WASI build complete"

## wasi-exec: executing the WAVS wasi component(s) | COMPONENT_FILENAME, COIN_MARKET_CAP_ID
wasi-exec: pull-image
	@$(WAVS_CMD) exec --log-level=info --data /data/.docker --home /data \
	--component "/data/compiled/$(COMPONENT_FILENAME)" \
	--input `cast format-bytes32-string $(COIN_MARKET_CAP_ID)`

## clean: cleaning the project files
clean: clean-docker
	@forge clean
	@$(CARGO) clean
	@rm -rf cache
	@rm -rf out
	@rm -rf broadcast

## clean-docker: remove unused docker containers
clean-docker:
	@$(SUDO) docker rm -v $(shell $(SUDO) docker ps -a --filter status=exited -q) > /dev/null 2>&1 || true

## fmt: formatting solidity and rust code
fmt:
	@forge fmt --check
	@$(CARGO) fmt

## test: running tests
test:
	@forge test

## setup: install initial dependencies
setup: check-requirements
	@echo "📦 Installing dependencies..."
	@echo "  • Installing Forge dependencies..."
	@forge install > /dev/null 2>&1
	@echo "  • Installing npm dependencies..."
	@npm install > /dev/null 2>&1
	@echo "✅ Dependencies installed"

## start-all-local: starting anvil and core services (like IPFS for example)
start-all-local: clean-docker setup-env
	@sh ./script/start_all.sh

## get-trigger-from-deploy: getting the trigger address from the script deploy
get-trigger-from-deploy:
	@jq -r '.deployedTo' "./.docker/trigger.json"

## get-submit-from-deploy: getting the submit address from the script deploy
get-submit-from-deploy:
	@jq -r '.deployedTo' "./.docker/submit.json"

## wavs-cli: running wavs-cli in docker
wavs-cli:
	@$(WAVS_CMD) $(filter-out $@,$(MAKECMDGOALS))

## upload-component: uploading the WAVS component | COMPONENT_FILENAME, WAVS_ENDPOINT
upload-component:
	@if [ -z "${COMPONENT_FILENAME}" ]; then \
		echo "❌ Error: COMPONENT_FILENAME is not set"; \
		echo "💡 Set it with: export COMPONENT_FILENAME=evm_price_oracle.wasm"; \
		echo "📖 See 'make help' for more info"; \
		exit 1; \
	fi
	@echo "📤 Uploading component: ${COMPONENT_FILENAME}..."
	@wget --post-file=./compiled/${COMPONENT_FILENAME} --header="Content-Type: application/wasm" -O - ${WAVS_ENDPOINT}/upload | jq -r .digest
	@echo "✅ Component uploaded successfully"

IPFS_GATEWAY?="https://ipfs.io/ipfs"
## deploy-service: deploying the WAVS component service json | SERVICE_URL, CREDENTIAL, WAVS_ENDPOINT
deploy-service:
# this wait is required to ensure the WAVS service has time to service check
	@if [ -z "${SERVICE_URL}" ]; then \
		echo "❌ Error: SERVICE_URL is not set"; \
		echo "💡 Set it with: export SERVICE_URL=<ipfs-or-http-url>"; \
		echo "📖 See 'make help' for more info"; \
		exit 1; \
	fi
	@if [ -n "${WAVS_ENDPOINT}" ]; then \
		echo "🔍 Checking WAVS service at ${WAVS_ENDPOINT}..."; \
		if [ "$$(curl -s -o /dev/null -w "%{http_code}" ${WAVS_ENDPOINT}/app)" != "200" ]; then \
			echo "❌ WAVS service not reachable at ${WAVS_ENDPOINT}"; \
			echo "💡 Re-try running in 1 second, if not then validate the wavs service is online / started."; \
			exit 1; \
		fi; \
		echo "✅ WAVS service is running"; \
	fi
	@echo "🚀 Deploying service from: ${SERVICE_URL}..."
	@$(WAVS_CMD) deploy-service --service-url ${SERVICE_URL} --log-level=debug --data /data/.docker --home /data $(if $(WAVS_ENDPOINT),--wavs-endpoint $(WAVS_ENDPOINT),) $(if $(IPFS_GATEWAY),--ipfs-gateway $(IPFS_GATEWAY),)
	@echo "✅ Service deployed successfully"

## get-trigger: get the trigger id | SERVICE_TRIGGER_ADDR, RPC_URL
get-trigger:
	@forge script ./script/ShowResult.s.sol ${SERVICE_TRIGGER_ADDR} --sig 'trigger(string)' --rpc-url $(RPC_URL) --broadcast

TRIGGER_ID?=1
## show-result: showing the result | SERVICE_SUBMISSION_ADDR, TRIGGER_ID, RPC_URL
show-result:
	@forge script ./script/ShowResult.s.sol ${SERVICE_SUBMISSION_ADDR} ${TRIGGER_ID} --sig 'data(string,uint64)' --rpc-url $(RPC_URL) --broadcast


PINATA_API_KEY?=""
## upload-to-ipfs: uploading the a service config to IPFS | SERVICE_FILE, [PINATA_API_KEY]
upload-to-ipfs:
	@if [ `sh script/get-deploy-status.sh` = "LOCAL" ]; then \
		curl -X POST "http://127.0.0.1:5001/api/v0/add?pin=true" -H "Content-Type: multipart/form-data" -F file=@${SERVICE_FILE} | jq -r .Hash; \
	else \
		if [ -z "${PINATA_API_KEY}" ]; then \
			echo "Error: PINATA_API_KEY is not set. Please set it to your Pinata API key -- https://app.pinata.cloud/developers/api-keys."; \
			exit 1; \
		fi; \
		curl -X POST --url https://uploads.pinata.cloud/v3/files --header "Authorization: Bearer ${PINATA_API_KEY}" --header 'Content-Type: multipart/form-data' --form file=@${SERVICE_FILE} --form network=public --form name=service-`date +"%b-%d-%Y"`.json | jq -r .data.cid; \
	fi

COMMAND?=""
PAST_BLOCKS?=500
wavs-middleware:
	@docker run --rm --network host --env-file ${ENV_FILE} \
		$(if ${WAVS_SERVICE_MANAGER_ADDRESS},-e WAVS_SERVICE_MANAGER_ADDRESS=${WAVS_SERVICE_MANAGER_ADDRESS}) \
		$(if ${PAST_BLOCKS},-e PAST_BLOCKS=${PAST_BLOCKS}) \
		-v ./.nodes:/root/.nodes ${MIDDLEWARE_DOCKER_IMAGE} ${COMMAND}

## update-submodules: update the git submodules
update-submodules:
	@git submodule update --init --recursive

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
_build_forge:
	@forge build

.PHONY: setup-env
setup-env:
	@if [ ! -f ${ENV_FILE} ]; then \
		if [ -f .env.example ]; then \
			echo "Creating ${ENV_FILE} file from .env.example..."; \
			cp .env.example ${ENV_FILE}; \
			echo "${ENV_FILE} file created successfully!"; \
		fi; \
	fi

pull-image:
	@if ! docker image inspect ${DOCKER_IMAGE} &>/dev/null; then \
		echo "Image ${DOCKER_IMAGE} not found. Pulling..."; \
		$(SUDO) docker pull ${DOCKER_IMAGE}; \
	fi

# check versions

## check-requirements: verify system requirements are installed
check-requirements:
	@echo "🔍 Validating system requirements..."
	@$(MAKE) check-node check-jq check-cargo check-docker
	@echo "✅ All requirements satisfied"

check-command:
	@command -v $(1) > /dev/null 2>&1 || (echo "❌ $(1) not found. Please install $(1), reference the System Requirements section"; exit 1)

check-command-with-help:
	@command -v $(1) > /dev/null 2>&1 || \
		(echo "❌ $(1) not found"; echo "💡 Install: $(2)"; exit 1)

.PHONY: check-node
check-node:
	@$(call check-command-with-help,node,"curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash && nvm install --lts")
	@NODE_VERSION=$$(node --version); \
	MAJOR_VERSION=$$(echo $$NODE_VERSION | sed 's/^v\([0-9]*\)\..*/\1/'); \
	if [ $$MAJOR_VERSION -lt 21 ]; then \
		echo "❌ Node.js version $$NODE_VERSION is less than required v21"; \
		echo "💡 Upgrade with: nvm install --lts"; \
		exit 1; \
	fi

.PHONY: check-jq
check-jq:
	@$(call check-command-with-help,jq,"brew install jq (macOS) or apt install jq (Linux)")

.PHONY: check-cargo
check-cargo:
	@$(call check-command-with-help,cargo,"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")

.PHONY: check-docker
check-docker:
	@$(call check-command-with-help,docker,"https://docs.docker.com/get-docker/")
