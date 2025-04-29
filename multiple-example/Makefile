# Multi-operator WAVS Makefile

.PHONY: help start-multi stop-multi get-trigger show-result create-operators

help:
	@echo "Multi-operator WAVS example commands:"
	@echo ""
	@echo "make start-multi        Start the multi-operator environment"
	@echo "make stop-multi         Stop the multi-operator environment"
	@echo "make create-operators   Create new operator keys"
	@echo "make get-trigger        Get the latest trigger ID"
	@echo "make show-result        Show the result for a specific trigger ID (TRIGGER_ID=1)"

start-multi:
	./start_multi.sh

stop-multi:
	cd .. && docker compose -f multiple-example/docker-compose.yml down

create-operators:
	mkdir -p ../.docker
	# Create operator 1 keys
	cast wallet new-mnemonic --json > ../.docker/operator1.json
	export OPERATOR1_MNEMONIC=`cat ../.docker/operator1.json | jq -r .mnemonic` && \
	export OPERATOR1_PK=`cat ../.docker/operator1.json | jq -r .accounts[0].private_key` && \
	sed -i'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$$/WAVS_SUBMISSION_MNEMONIC=\"$$OPERATOR1_MNEMONIC\"/" .env1 && \
	sed -i'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$$/WAVS_CLI_EVM_CREDENTIAL=\"$$OPERATOR1_PK\"/" .env1 && \
	sed -i'' -e "s/^WAVS_AGGREGATOR_CREDENTIAL=.*$$/WAVS_AGGREGATOR_CREDENTIAL=\"$$OPERATOR1_PK\"/" .env1

	# Create operator 2 keys
	cast wallet new-mnemonic --json > ../.docker/operator2.json
	export OPERATOR2_MNEMONIC=`cat ../.docker/operator2.json | jq -r .mnemonic` && \
	export OPERATOR2_PK=`cat ../.docker/operator2.json | jq -r .accounts[0].private_key` && \
	sed -i'' -e "s/^WAVS_SUBMISSION_MNEMONIC=.*$$/WAVS_SUBMISSION_MNEMONIC=\"$$OPERATOR2_MNEMONIC\"/" .env2 && \
	sed -i'' -e "s/^WAVS_CLI_EVM_CREDENTIAL=.*$$/WAVS_CLI_EVM_CREDENTIAL=\"$$OPERATOR2_PK\"/" .env2 && \

get-trigger:
	cd .. && cast call $$(jq -r .deployedTo .docker/submit.json) "lastId()(uint256)" --rpc-url http://localhost:8545

show-result:
	cd .. && forge script ./script/ShowResult.s.sol $$(jq -r .deployedTo .docker/submit.json) $(TRIGGER_ID) --sig 'run(string,string)' --rpc-url http://localhost:8545
