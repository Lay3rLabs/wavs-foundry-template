# Generate service.json file

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

## Example

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
