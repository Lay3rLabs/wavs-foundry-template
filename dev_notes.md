# foundry
```cd 
curl -L https://foundry.paradigm.xyz | bash && $HOME/.foundry/bin/foundryup
```

# Start anvil fork
```
anvil --fork-url https://ethereum-holesky-rpc.publicnode.com
```

# Build deps and the ServiceHandler contract
```
make setup
forge build
```

# Deploy eigen middleware
https://github.com/Lay3rLabs/wavs-middleware/tree/ethan-docker/docker


# Deploy ServiceHandler
```
forge create SimpleSubmit --broadcast --rpc-url http://127.0.0.1:8545 --private-key <go_figure> --constructor-args 0xServiceManagerAddress
```

# Deploy aggregator
```
forge create WavsServiceAggregator --broadcast --rpc-url http://127.0.0.1:8545 --private-key <go_figure> --constructor-args 0xServiceHandlerAddress
```
Now run the aggregator server in wavs/packages/aggregator and add the `ServiceHandler` through `add-service` endpoint:
```bash
curl -X POST http://localhost:8001/add-service \
     -H "Content-Type: application/json" \
     -d '{"eth_trigger": {"address": "SERVICE_HANDLER_ADDRESS"}}
```

# Generate service.json file
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