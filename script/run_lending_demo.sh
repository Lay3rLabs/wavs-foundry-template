#!/bin/bash

# Vault Lending System Demo Script
# This script deploys and demonstrates the complete vault lending system

set -e

echo "🚀 Starting Vault Lending System Demo"
echo "======================================"

# Check if required environment variables are set
if [ -z "$PRIVATE_KEY" ]; then
    echo "❌ Error: PRIVATE_KEY environment variable not set"
    echo "Please set it with: export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    exit 1
fi

# Default to localhost if RPC_URL not set
if [ -z "$RPC_URL" ]; then
    echo "ℹ️  RPC_URL not set, using localhost:8545"
    export RPC_URL="http://localhost:8545"
fi

echo "🔧 Configuration:"
echo "   RPC URL: $RPC_URL"
echo "   Using private key: ${PRIVATE_KEY:0:10}..."
echo ""

# Step 1: Start local blockchain (if needed)
echo "🔍 Checking if local blockchain is running..."
if curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' $RPC_URL > /dev/null 2>&1; then
    echo "✅ Blockchain is running"
else
    echo "⚠️  Local blockchain not detected. Starting Anvil..."
    echo "   You can also run: anvil --host 0.0.0.0 --port 8545"
    anvil --host 0.0.0.0 --port 8545 &
    ANVIL_PID=$!

    # Wait for anvil to start
    sleep 3

    # If we started anvil, we should use the default private key
    if [ "$RPC_URL" = "http://localhost:8545" ]; then
        export PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
        echo "   Using Anvil default private key"
    fi
fi

echo ""

# Step 2: Compile contracts
echo "🔨 Compiling contracts..."
forge build
if [ $? -ne 0 ]; then
    echo "❌ Compilation failed"
    exit 1
fi
echo "✅ Contracts compiled successfully"
echo ""

# Step 3: Deploy contracts
echo "📦 Deploying Vault Lending System..."
DEPLOY_OUTPUT=$(forge script script/DeployLendingSystem.s.sol:DeployLendingSystem --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -vvv)

if [ $? -ne 0 ]; then
    echo "❌ Deployment failed"
    exit 1
fi

echo "✅ Deployment successful!"
echo ""

# Extract contract addresses from deployment output (this is a simplified approach)
echo "📋 Deployment addresses:"
echo "$DEPLOY_OUTPUT" | grep -E "(MockERC20|SolidityVault|MockLendingProtocol|VaultLendingStrategy) deployed at:" || echo "   See output above for addresses"
echo ""

# Step 4: Wait a moment for deployment to settle
echo "⏳ Waiting for deployment to settle..."
sleep 2

# Step 5: Run interaction demo
echo "🎯 Running interaction demo..."
echo "   This will demonstrate:"
echo "   - User depositing to vault"
echo "   - Strategy lending to protocol"
echo "   - Interest accrual simulation"
echo "   - User withdrawal"
echo "   - WAVS trigger demonstration"
echo ""

# Note: The interaction script has placeholder addresses that need to be updated
echo "⚠️  Note: You may need to update contract addresses in InteractLendingSystem.s.sol"
echo "   Copy the deployed addresses from above into the _setContractAddresses() function"
echo ""

# Uncomment the line below once addresses are updated
# forge script script/InteractLendingSystem.s.sol:InteractLendingSystem --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -vvv

echo "🎉 Demo deployment complete!"
echo ""
echo "🔧 Next steps:"
echo "   1. Update contract addresses in script/InteractLendingSystem.s.sol"
echo "   2. Run the interaction script:"
echo "      forge script script/InteractLendingSystem.s.sol:InteractLendingSystem --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -vvv"
echo ""
echo "📚 Useful commands:"
echo "   - Check balance: cast balance [address] --rpc-url $RPC_URL"
echo "   - Call contract: cast call [contract] [signature] --rpc-url $RPC_URL"
echo "   - Send transaction: cast send [contract] [signature] [args] --private-key $PRIVATE_KEY --rpc-url $RPC_URL"

# Cleanup function
cleanup() {
    if [ ! -z "$ANVIL_PID" ]; then
        echo "🧹 Stopping Anvil (PID: $ANVIL_PID)..."
        kill $ANVIL_PID 2>/dev/null || true
    fi
}

# Set trap to cleanup on script exit
trap cleanup EXIT

# If we started anvil, keep it running
if [ ! -z "$ANVIL_PID" ]; then
    echo ""
    echo "⚡ Anvil is running in the background (PID: $ANVIL_PID)"
    echo "   Press Ctrl+C to stop the demo and cleanup"

    # Wait for user to stop
    wait $ANVIL_PID
fi
