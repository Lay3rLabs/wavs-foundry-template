# Setting up WAVS deploy on testnet (fork)

## Setup

### Install foundry

```bash 
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

### Build WAVS tools

Currently, we must manually build images. These will come from tags and ideally be pushed to ghcr later

#### WAVS

This requires PR https://github.com/Lay3rLabs/WAVS/pull/494 if that is merged, you can use `main`, otherwise use `phase-2-chain` branch.

```bash
git clone --depth 1 git@github.com:Lay3rLabs/WAVS.git
cd WAVS
git checkout phase-2-chain
docker build . -t ghcr.io/lay3rlabs/wavs:local
```

#### WAVS-middleware

```bash
git clone --depth 1 git@github.com:Lay3rLabs/wavs-middleware.git
cd wavs-middleware
docker build . -t wavs-middleware:local 
```

## Chain Setup

### Local Fork

For testnet fork, we need to run anvil and create a private key. All operators must be on the same machine and point to this URL

```bash 
anvil --fork-url https://ethereum-holesky-rpc.publicnode.com
```

(You can also use another rpc endpoint of your choice )

We need to create a new private key and fund it for later use. 
If this is really testnet/mainnet, you can use a more secure key, for testing, just make a one-off key

```bash
export PRIVATE_KEY=$(cast wallet new --json | jq -r .[0].private_key)
export MY_ADDR=$(cast wallet address --private-key $PRIVATE_KEY)
cast rpc anvil_setBalance $MY_ADDR 0x10000000000000000000 -r http://localhost:8545
```

### Real Testnet

You must create an account and fund it with testnet ETH. You can use MetaMask or just the commands above.
Getting testnet ETH is out of scope of this guide (TODO: Ron - where to get it)

You will also want to set a testnet rpc url to use it (TODO: link to some good ones, which env var)

## Deploying as AVS Dev Team

More detailed instructions on commands and setup can be found in [wavs-middleware](https://github.com/Lay3rLabs/wavs-middleware/blob/dev/docker/README.md).

You need to prepare an .env file in this directory for running the rest of the commands.

```bash
cp env.example .env
# edit the file and make any changes you need there
```

### Deploy eigen middleware

```bash
docker run --rm --network host --env-file .env  -v ./.nodes:/root/.nodes wavs-middleware:local
export SERVICE_MANAGER_ADDRESS=$(jq -r .addresses.WavsServiceManager .nodes/avs_deploy.json)
```

### Build Custom Application

Assuming you are using the template app, you can do the following. More advanced apps will have custom build steps, run from the top-level directory:

```bash
cd ..
make setup
forge build
make wasi-build
```

TODO: `make wasi-build` now fails for me with `(jco componentize) ComponentError: package 'wasi:io@0.2.0' not found. no known packages.`

### Deploy ServiceHandler

```bash
cd src/contracts
forge create SimpleSubmit --broadcast --rpc-url http://127.0.0.1:8545 --private-key "$PRIVATE_KEY" --constructor-args "$SERVICE_MANAGER_ADDRESS"

forge create SimpleTrigger --broadcast --rpc-url http://127.0.0.1:8545 --private-key "$PRIVATE_KEY"
```

Store "deployed_to" from the first command as `SERVICE_HANDLER_ADDRESS`.
And store "deployed_to" from the second command as `TRIGGER_ADDRESS`.

### Start aggregator

In another window, Go to `aggregator` dir in `wavs-foundry-template` (you may want to edit `aggregator/.wavs_env` first)

```bash
cd wavs-staking-template/aggregator
docker compose up
```

TODO: the following is no longer valid - we need to add the servie.json later

```bash
curl -X POST http://localhost:8001/add-service \
     -H "Content-Type: application/json" \
     -d "{\"eth_trigger\": {\"address\": \"$SERVICE_AGGREGATOR_ADDRESS\"}}"
```

### Create Service.JSON

This file contains all the information about the service, including the service manager, the handler and trigger, as well as the aggregator address. To build your own, look at [create_service.md](create_service.md) for some info, or ask the WAVS team how to use these commands to build service.json.

For deploying the template service, you can just use the template file and fill in some values, like:

```bash
cat service.json.template \
    | sed "s/SERVICE_MANAGER_ADDRESS/$SERVICE_MANAGER_ADDRESS/g" \
    | sed "s/SERVICE_HANDLER_ADDRESS/$SERVICE_HANDLER_ADDRESS/g" \
    | sed "s/TRIGGER_ADDRESS/$TRIGGER_ADDRESS/g"  \
    > service.json
```

TODO: we need to update this service.json.template to include the aggregator path as well as the service manager address. We should also push the WASI component to a registry (Layer dev team can do that once) and reference that in the template. 

### Add Service.json to Service Manager

TODO: upload service.json

TODO: call set-service-uri script from wavs-middleware

### Inform the aggregator of the service

TODO: how???

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