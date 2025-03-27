# Setting up WAVS deploy on testnet (fork)

## foundry

```bash 
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

## Start anvil fork

```bash 
anvil --fork-url https://ethereum-holesky-rpc.publicnode.com
```

## Create private key

We need to create a new private key and fund it for later use. 
If this is really testnet/mainnet, you can use a more secure key, for testing, just make a one-off key

```bash
export PRIVATE_KEY=$(cast wallet new --json | jq -r .[0].private_key)
export MY_ADDR=$(cast wallet address --private-key $PRIVATE_KEY)
cast rpc anvil_setBalance $MY_ADDR 0x10000000000000000000 -r http://localhost:8545
```



TODO: how to fund it?

## Deploy eigen middleware

https://github.com/Lay3rLabs/wavs-middleware/blob/dev/docker/README.md

After deploying middleware:

```json
{"lastUpdate":.... ,
"layerServiceManager":"0xc73da08814528003ebadeb19d7ebebe51fb67b6b",
....}
```

Now you have the service manager address. Store it as `SERVICE_MANAGER_ADDRESS`.

TODO: mount deployment json and get info there

## Build deps and the ServiceHandler contract

https://github.com/Lay3rLabs/wavs-foundry-template

```bash
make setup
forge build
```

## Deploy ServiceHandler

```bash
cd src/contracts
forge create SimpleSubmit --broadcast --rpc-url http://127.0.0.1:8545 --private-key "$PRIVATE_KEY" --constructor-args "$SERVICE_MANAGER_ADDRESS"
```

Now you have the service handler address. Store it as `SERVICE_HANDLER_ADDRESS`.

## Deploy aggregator

https://github.com/Lay3rLabs/WAVS

TODO: use this from wavs-middleware soon

```bash
cd contracts/solidty
forge create WavsServiceAggregator --broadcast --rpc-url http://127.0.0.1:8545 --private-key "$PRIVATE_KEY" --constructor-args "$SERVICE_HANDLER_ADDRESS"
```

Now run the aggregator server and add the `ServiceHandler` through `add-service` endpoint:

```bash
docker compose up # wavs-staking-template/docker-compose.yml
```

And add the service to the aggregator

```bash
curl -X POST http://localhost:8001/add-service \
     -H "Content-Type: application/json" \
     -d '{"eth_trigger": {"address": "SERVICE_HANDLER_ADDRESS"}}
```

## Generate service.json file

Use `wavs-cli service` command:

```bash
wavs-cli service init --name <your_service_name>
wavs-cli service component add --digest <your_component_digest>
```

Now you have the component ID in the `service.json` file:

```bash
wavs-cli service workflow add --component-id <your_component_id>
```

Take workflow ID from the `service.json` file:

```bash
wavs-cli service trigger set-ethereum --workflow-id <your_workflow_id> --address <address_of_trigger_contract> --chain-name <chain_name> --event-hash <signature_or_hash_of_event_signature>

wavs-cli service submit set-ethereum --workflow-id <your_workflow_id> --address <address_of_service_handler> --chain-name <chain_name>
```
