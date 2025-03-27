# Setting up WAVS deploy on testnet (fork)

## Deploying as Developer

### foundry

```bash 
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

### Start anvil fork

```bash 
anvil --fork-url https://ethereum-holesky-rpc.publicnode.com
```

### Create private key

We need to create a new private key and fund it for later use. 
If this is really testnet/mainnet, you can use a more secure key, for testing, just make a one-off key

```bash
export PRIVATE_KEY=$(cast wallet new --json | jq -r .[0].private_key)
export MY_ADDR=$(cast wallet address --private-key $PRIVATE_KEY)
cast rpc anvil_setBalance $MY_ADDR 0x10000000000000000000 -r http://localhost:8545
```



TODO: how to fund it?

### Deploy eigen middleware

https://github.com/Lay3rLabs/wavs-middleware/blob/dev/docker/README.md

After deploying middleware:

```json
{"lastUpdate":.... ,
"layerServiceManager":"0xc73da08814528003ebadeb19d7ebebe51fb67b6b",
....}
```

Now you have the service manager address. Store it as `SERVICE_MANAGER_ADDRESS`.

TODO: mount deployment json and get info there

### Build deps and the ServiceHandler contract

https://github.com/Lay3rLabs/wavs-foundry-template

```bash
make setup
forge build
make wasi-build
```

### Deploy ServiceHandler

```bash
cd src/contracts
forge create SimpleSubmit --broadcast --rpc-url http://127.0.0.1:8545 --private-key "$PRIVATE_KEY" --constructor-args "$SERVICE_MANAGER_ADDRESS"
forge create SimpleTrigger --broadcast --rpc-url http://127.0.0.1:8545 --private-key "$PRIVATE_KEY"
```

Now you have the service handler address. Store it as `SERVICE_HANDLER_ADDRESS`.
And store the trigger address as `SERVICE_TRIGGER_ADDRESS`.

### Deploy aggregator

#### Contracts and Docker build

https://github.com/Lay3rLabs/WAVS

TODO: use this from wavs-middleware soon

```bash
cd contracts/solidty
forge create WavsServiceAggregator --broadcast --rpc-url http://127.0.0.1:8545 --private-key "$PRIVATE_KEY" --constructor-args "$SERVICE_HANDLER_ADDRESS"
```

Store this address as `SERVICE_AGGREGATOR_ADDRESS`.

`docker build . -t ghcr.io/lay3rlabs/wavs:local`

Also build the wavs-cli:

```bash
cd packages/cli
cargo install --path .
```

#### Run aggregator

Now run the aggregator server and add the `ServiceHandler` through `add-service` endpoint:

Go pack to `wavs-foundry-template`:

```bash
cd wavs-staking-template
docker compose up
```

And add the service to the aggregator

```bash
curl -X POST http://localhost:8001/add-service \
     -H "Content-Type: application/json" \
     -d "{\"eth_trigger\": {\"address\": \"$SERVICE_AGGREGATOR_ADDRESS\"}}"
```

Check this is now registered.
TODO: add a command to check what is running.

### Generate service.json file

Use `wavs-cli service` command (in `wavs-staking-template` dir):

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

#### Example

Use `wavs-cli service` command (in `wavs-staking-template` dir):

```bash
DIGEST=$(sha256sum ../compiled/eth_price_oracle.wasm | cut -f1 -d' ' )

wavs-cli service init --name madrid-demo
wavs-cli service component add --digest $DIGEST --id price_oracle
```

Now you have the component ID in the `service.json` file:

```bash
# COMPONENT_ID=$(cat service.json | jq -r '.components | keys | .[0]')

wavs-cli service workflow add --component-id price_oracle --id prices
```

Take workflow ID from the `service.json` file:

```bash
wavs-cli service trigger set-ethereum --workflow-id prices --address $SERVICE_TRIGGER_ADDRESS --chain-name holesky-fork --event-hash "NewTrigger(bytes)"

wavs-cli service submit set-ethereum --workflow-id prices --address $SERVICE_HANDLER_ADDRESS --chain-name holesky-fork
```

### Add Service.json to Service Manager

TODO: upload service.json

TODO: call set-service-uri script from wavs-middleware

## Running as Operator

All the below is for the operator.

### Start WAVS

```bash
export WAVS_SUBMISSION_MNEMONIC="test test test test test test test test test test test junk"
docker run --rm --network host -v $(pwd):/wavs -e WAVS_SUBMISSION_MNEMONIC -e WAVS_HOME="/wavs/packages/wavs" \
    -e WAVS_CLI_HOME="/wavs/packages/cli" -e WAVS_AGGREGATOR_HOME="/wavs/packages/aggregator" \
    ghcr.io/lay3rlabs/wavs:local  wavs
```

### Add Service to WAVS

```bash
wavs-cli upload-component '../compiled/eth_price_oracle.wasm'

# TODO: dies with missing chain in wavs.toml
wavs-cli deploy-service-raw --service '@service.json'
```

### Register Operator to AVS Service

TODO: call register-operator script from wavs-middleware