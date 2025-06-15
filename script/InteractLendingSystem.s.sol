// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

import {Script, console} from "forge-std/Script.sol";
import {MockERC20} from "../src/contracts/MockERC20.sol";
import {SolidityVault} from "../src/contracts/SimpleVault.sol";
import {MockLendingProtocol} from "../src/contracts/MockLendingProtocol.sol";
import {VaultLendingStrategy} from "../src/contracts/VaultLendingStrategy.sol";

/// @title InteractLendingSystem
/// @notice Demonstrates the complete vault lending system workflow
contract InteractLendingSystem is Script {
    // Contract instances (need to be set with deployed addresses)
    MockERC20 public token;
    SolidityVault public vault;
    MockLendingProtocol public lendingProtocol;
    VaultLendingStrategy public strategy;

    address public user;

    function run() public {
        uint256 userPrivateKey = vm.envUint("PRIVATE_KEY");
        user = vm.addr(userPrivateKey);

        // Set deployed contract addresses (update these with actual deployed addresses)
        _setContractAddresses();

        vm.startBroadcast(userPrivateKey);

        console.log("=== Vault Lending System Interaction Demo ===");
        console.log("User:", user);

        // Step 1: Show initial state
        _showSystemState("Initial State");

        // Step 2: User deposits into vault
        _userDepositsToVault();

        // Step 3: Strategy lends to protocol (manual execution)
        _strategyLendsToProtocol();

        // Step 4: Wait and show interest accrual
        _waitAndShowInterest();

        // Step 5: User withdraws from vault
        _userWithdrawsFromVault();

        // Step 6: Demonstrate WAVS trigger functionality
        _demonstrateWavsTrigger();

        vm.stopBroadcast();

        console.log("\n=== Demo Complete ===");
    }

    function _setContractAddresses() internal {
        // IMPORTANT: Update these addresses with your deployed contract addresses
        // You can get these from the deployment script output

        // For local testing, you might need to deploy first or use pre-deployed addresses
        // These are placeholder addresses - replace with actual deployed addresses
        address tokenAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // Replace
        address vaultAddress = 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512; // Replace
        address lendingProtocolAddress = 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0; // Replace
        address strategyAddress = 0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9; // Replace

        // Comment out the above and uncomment below if you want to deploy fresh contracts
        _deployFreshContracts();
    }

    function _deployFreshContracts() internal {
        console.log("Deploying fresh contracts for interaction...");

        token = new MockERC20("Test Token", "TEST", 1_000_000);
        vault = new SolidityVault(address(token));
        lendingProtocol = new MockLendingProtocol(address(token));
        strategy = new VaultLendingStrategy(
            address(vault),
            address(lendingProtocol),
            user // Use user as operator
        );

        // Setup
        token.mint(user, 100_000 * 10**token.decimals());
        token.mint(address(strategy), 50_000 * 10**token.decimals()); // Give strategy initial tokens
        token.mint(address(lendingProtocol), 50_000 * 10**token.decimals());
    }

    function _showSystemState(string memory label) internal view {
        console.log(string(abi.encodePacked("\n=== ", label, " ===")));

        uint256 userTokenBalance = token.balanceOf(user);
        uint256 userVaultShares = vault.balanceOf(user);
        uint256 vaultTotalAssets = vault.totalAssets();
        uint256 strategyBalance = token.balanceOf(address(strategy));
        uint256 strategyLentAmount = strategy.lentAmount();

        (uint256 totalValue, uint256 lendingBalance) = strategy.getStrategyValue();

        console.log("User token balance:", userTokenBalance);
        console.log("User vault shares:", userVaultShares);
        console.log("Vault total assets:", vaultTotalAssets);
        console.log("Strategy token balance:", strategyBalance);
        console.log("Strategy lent amount:", strategyLentAmount);
        console.log("Strategy total value:", totalValue);
        console.log("Lending protocol balance:", lendingBalance);
    }

    function _userDepositsToVault() internal {
        console.log("\n=== User Deposits to Vault ===");

        uint256 depositAmount = 10_000 * 10**token.decimals();

        // Approve and deposit
        token.approve(address(vault), depositAmount);
        uint256 shares = vault.deposit(depositAmount, user);

        console.log("Deposited:", depositAmount);
        console.log("Received shares:", shares);

        _showSystemState("After User Deposit");
    }

    function _strategyLendsToProtocol() internal {
        console.log("\n=== Strategy Lends to Protocol ===");

        // First, deposit some assets to the strategy for it to lend
        uint256 strategyDeposit = 5_000 * 10**token.decimals();
        token.approve(address(vault), strategyDeposit);
        vault.deposit(strategyDeposit, address(strategy));

        console.log("Deposited to strategy via vault:", strategyDeposit);

        // Execute lending strategy (this would normally be triggered by WAVS)
        strategy.executeLendingStrategy();

        console.log("Strategy executed lending");

        _showSystemState("After Strategy Lending");
    }

    function _waitAndShowInterest() internal {
        console.log("\n=== Waiting for Interest Accrual ===");

        // Simulate time passing (1 hour)
        vm.warp(block.timestamp + 3600);

        console.log("Simulated 1 hour passage...");

        _showSystemState("After 1 Hour (Interest Accrued)");
    }

    function _userWithdrawsFromVault() internal {
        console.log("\n=== User Withdraws from Vault ===");

        uint256 userShares = vault.balanceOf(user);
        uint256 withdrawShares = userShares / 2; // Withdraw half

        uint256 assetsReceived = vault.redeem(withdrawShares, user, user);

        console.log("Redeemed shares:", withdrawShares);
        console.log("Assets received:", assetsReceived);

        _showSystemState("After User Withdrawal");
    }

    function _demonstrateWavsTrigger() internal {
        console.log("\n=== Demonstrating WAVS Trigger ===");

        // Get current state to make a valid decision
        uint256 vaultTotalAssets = vault.totalAssets();
        uint256 currentLentAmount = strategy.lentAmount();

        console.log("Current vault assets:", vaultTotalAssets);
        console.log("Current lent amount:", currentLentAmount);

        // Calculate a valid recall amount (recall some assets to rebalance)
        uint256 targetRatio = 0.5 * 1e18; // Target 50% lending ratio
        uint256 targetLentAmount = (vaultTotalAssets * 50) / 100; // 50% of vault assets
        uint256 recallAmount = currentLentAmount > targetLentAmount ?
            currentLentAmount - targetLentAmount : 0;

        console.log("Target lent amount:", targetLentAmount);
        console.log("Calculated recall amount:", recallAmount);

        // Create a mock lending decision (this would come from off-chain compute)
        VaultLendingStrategy.LendingDecision memory decision = VaultLendingStrategy.LendingDecision({
            action: VaultLendingStrategy.ActionType.RECALL, // 2 = RECALL
            amount: recallAmount,
            targetRatio: targetRatio, // 50% target ratio (scaled by 1e18)
            confidence: 0.8 * 1e18,  // 80% confidence (scaled by 1e18)
            timestamp: block.timestamp
        });

        // Encode the decision
        bytes memory encodedDecision = abi.encode(decision);
        bytes memory triggerInfo = abi.encode(uint256(1), encodedDecision); // triggerId = 1

        console.log("Triggering strategy with off-chain decision...");
        console.log("Action: RECALL (enum value 2)");
        console.log("Amount:", decision.amount);
        console.log("Target ratio:", decision.targetRatio);

        // Call the trigger function (simulate WAVS calling it)
        strategy.trigger(triggerInfo);

        console.log("WAVS trigger executed successfully");

        _showSystemState("After WAVS Trigger");
    }
}
