# notes

## ideal
- Merge the .env and docker-compose.yml into the root repo, then users just run 1 command
- if a localhost:8545 fails to start with wavs, try 'ethereum:8545' as the RPC/ws instead automatically, then this would work above for MacOS & Linux

## env
ENV requires the following lines to be changed. These should be first in the file and not set with defaults in the example:
((PRIVATE_KEY maybe with anvil default?))

PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 # with hex prefix
RPC_URL=https://holesky.infura.io/v3/MYAPIKEYHERE
ETHERSCAN_API_KEY=XXXXXXXXXXXXx



