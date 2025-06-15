// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

import { IERC20 } from "@openzeppelin-contracts/token/ERC20/IERC20.sol";
import { IERC4626 } from "@openzeppelin-contracts/interfaces/IERC4626.sol";
import "./MockLendingProtocol.sol";
/// @title VaultLendingStrategy
/// @notice Strategy contract that lends vault assets to a mock lending protocol
/// @dev Integrates with WAVS for off-chain computation and on-chain validation
contract VaultLendingStrategy {
    IERC4626 public immutable vault;
    IERC20 public immutable asset;
    MockLendingProtocol public immutable lendingProtocol;

    /// @notice Address authorized to execute strategy operations
    address public operator;

    /// @notice Percentage of vault assets to lend (in basis points, 10000 = 100%)
    uint256 public lendingRatio = 8000; // 80% by default

    /// @notice Minimum time between rebalancing operations (in seconds)
    uint256 public rebalanceInterval = 3600; // 1 hour

    /// @notice Last time the strategy was rebalanced
    uint256 public lastRebalance;

    /// @notice Amount currently lent to the protocol
    uint256 public lentAmount;

    /// @notice Action types for lending decisions
    enum ActionType { HOLD, LEND, RECALL }
    
    /// @notice Struct to represent lending decision from off-chain compute
    struct LendingDecision {
        ActionType action;      // 0=HOLD, 1=LEND, 2=RECALL
        uint256 amount;         // Amount to lend/recall
        uint256 targetRatio;    // Target lending ratio (scaled by 1e18)
        uint256 confidence;     // Confidence level (scaled by 1e18)
        uint256 timestamp;      // Decision timestamp
    }

    /// @notice Events
    event StrategyExecuted(uint256 amount, uint256 timestamp);
    event AssetsLent(uint256 amount);
    event AssetsRecalled(uint256 amount);
    event RebalanceTriggered(uint256 triggerId, uint256 vaultBalance);

    modifier onlyOperator() {
        require(msg.sender == operator, "Only operator can call this function");
        _;
    }

    constructor(
        address _vault,
        address _lendingProtocol,
        address _operator
    ) {
        vault = IERC4626(_vault);
        asset = IERC20(vault.asset());
        lendingProtocol = MockLendingProtocol(_lendingProtocol);
        operator = _operator;

        // Approve lending protocol and vault to spend our tokens
        asset.approve(_lendingProtocol, type(uint256).max);
        asset.approve(_vault, type(uint256).max);
    }

    /// @notice WAVS trigger function - called by off-chain compute
    /// @param triggerInfo Encoded trigger information from WAVS
    function trigger(bytes calldata triggerInfo) external onlyOperator {
        // Decode trigger data containing lending decision from off-chain compute
        (uint256 triggerId, bytes memory data) = abi.decode(triggerInfo, (uint256, bytes));

        // Parse the lending decision from off-chain compute
        LendingDecision memory decision = abi.decode(data, (LendingDecision));

        // Validate the decision meets our criteria
        require(_validateLendingDecision(decision), "Invalid lending decision");

        // Get current vault balance
        uint256 vaultBalance = asset.balanceOf(address(vault));

        emit RebalanceTriggered(triggerId, vaultBalance);

        // Execute the validated strategy
        _executeLendingDecision(decision);
    }

    /// @notice Validates the lending decision from off-chain compute
    /// @param decision The lending decision to validate
    /// @return isValid Whether the decision is valid
    function _validateLendingDecision(LendingDecision memory decision) internal view returns (bool) {
        // Basic validation checks
        // if (decision.confidence < 0.5) return false; // Require minimum confidence
        // if (decision.targetRatio > 0.95) return false; // Max 95% lending ratio
        if (block.timestamp - decision.timestamp > 300) return false; // Decision must be recent (5 mins)

        // Validate action and amount consistency
        uint256 vaultTotalAssets = vault.totalAssets();
        uint256 expectedTargetAmount = (vaultTotalAssets * decision.targetRatio) / 1e18;

        if (decision.action == ActionType.LEND) {
            return decision.amount <= vaultTotalAssets && expectedTargetAmount > lentAmount;
        } else if (decision.action == ActionType.RECALL) {
            // For recall: amount should be <= lentAmount AND target should make sense
            return decision.amount <= lentAmount;
        } else if (decision.action == ActionType.HOLD) {
            return decision.amount == 0;
        }

        return false;
    }

    /// @notice Executes the validated lending decision
    /// @param decision The validated lending decision
    function _executeLendingDecision(LendingDecision memory decision) internal {
        if (decision.action == ActionType.LEND) {
            _lendAssets(decision.amount);
        } else if (decision.action == ActionType.RECALL) {
            _recallAssets(decision.amount);
        }
        // No action needed for HOLD

        lastRebalance = block.timestamp;
        emit StrategyExecuted(lentAmount, block.timestamp);
    }

    /// @notice Internal function to lend assets to the protocol
    /// @param amount Amount to lend
    function _lendAssets(uint256 amount) internal {
        uint256 availableAssets = asset.balanceOf(address(this));

        if (availableAssets < amount) {
            // Withdraw from vault if we don't have enough assets
            uint256 toWithdraw = amount - availableAssets;
            vault.withdraw(toWithdraw, address(this), address(this));
        }

        // Lend to protocol
        lendingProtocol.deposit(amount);
        lentAmount += amount;

        emit AssetsLent(amount);
    }

    /// @notice Internal function to recall assets from the protocol
    /// @param amount Amount to recall
    function _recallAssets(uint256 amount) internal {
        if (lentAmount == 0) return; // Nothing to recall
        
        // Simplified: withdraw all assets from protocol (in production, implement partial withdrawals)
        uint256 balanceBefore = asset.balanceOf(address(this));
        lendingProtocol.withdraw();
        uint256 balanceAfter = asset.balanceOf(address(this));
        
        // Safely calculate totalRecalled to avoid underflow
        uint256 totalRecalled = balanceAfter > balanceBefore ? balanceAfter - balanceBefore : 0;
        
        // We withdrew everything, so lentAmount is now 0
        lentAmount = 0;
        
        // Calculate how much to keep vs deposit back to vault
        // We want to keep 'amount' tokens, deposit the rest to vault
        if (totalRecalled > amount) {
            uint256 excessToDeposit = totalRecalled - amount;
            vault.deposit(excessToDeposit, address(this));
        }
        
        emit AssetsRecalled(amount);
    }

    /// @notice Execute the lending strategy
    /// @dev Withdraws assets from vault and lends them to the protocol
    function executeLendingStrategy() external onlyOperator {
        require(block.timestamp >= lastRebalance + rebalanceInterval, "Too soon to rebalance");

        _rebalance();
    }

    /// @notice Internal rebalancing logic
    function _rebalance() internal {
        // Calculate how much we can actually lend based on our vault shares and available assets
        uint256 strategyVaultShares = vault.balanceOf(address(this));
        uint256 maxWithdrawable = strategyVaultShares > 0 ? vault.convertToAssets(strategyVaultShares) : 0;
        uint256 availableAssets = asset.balanceOf(address(this));
        uint256 totalAvailable = availableAssets + maxWithdrawable;
        
        // Target lend amount based on what we can actually access
        uint256 targetLendAmount = (totalAvailable * lendingRatio) / 10000;

        if (targetLendAmount > lentAmount) {
            // Need to lend more
            uint256 toLend = targetLendAmount - lentAmount;
            
            // Make sure we don't try to lend more than we have access to
            if (toLend > totalAvailable) {
                toLend = totalAvailable;
            }

            if (availableAssets < toLend) {
                // Withdraw from vault if we don't have enough assets
                uint256 toWithdraw = toLend - availableAssets;
                // Make sure we don't withdraw more than we can
                if (toWithdraw > maxWithdrawable) {
                    toWithdraw = maxWithdrawable;
                }
                if (toWithdraw > 0) {
                    vault.withdraw(toWithdraw, address(this), address(this));
                }
            }

            // Lend to protocol
            uint256 actualAvailable = asset.balanceOf(address(this));
            uint256 actualToLend = toLend > actualAvailable ? actualAvailable : toLend;
            if (actualToLend > 0) {
                lendingProtocol.deposit(actualToLend);
                lentAmount += actualToLend;
                emit AssetsLent(actualToLend);
            }
        } else if (targetLendAmount < lentAmount) {
            // Need to recall some assets
            uint256 toRecall = lentAmount - targetLendAmount;

            // This is simplified - in practice, you'd need to handle partial withdrawals
            uint256 balanceBefore = asset.balanceOf(address(this));
            lendingProtocol.withdraw();
            uint256 balanceAfter = asset.balanceOf(address(this));
            uint256 recalled = balanceAfter > balanceBefore ? balanceAfter - balanceBefore : 0;

            // Deposit back to vault
            if (recalled > 0) {
                vault.deposit(recalled, address(this));
            }
            lentAmount = targetLendAmount;

            emit AssetsRecalled(recalled);
        }

        lastRebalance = block.timestamp;
        emit StrategyExecuted(lentAmount, block.timestamp);
    }

    /// @notice Get current strategy performance
    /// @return totalValue Total value of strategy (vault assets + lent assets + interest)
    /// @return lendingBalance Current balance in lending protocol
    function getStrategyValue() external view returns (uint256 totalValue, uint256 lendingBalance) {
        lendingBalance = lendingProtocol.getBalance(address(this));
        uint256 vaultAssets = asset.balanceOf(address(this));
        totalValue = vaultAssets + lendingBalance;
    }

    /// @notice Emergency function to recall all lent assets
    function emergencyRecall() external onlyOperator {
        if (lentAmount > 0) {
            lendingProtocol.withdraw();
            uint256 recalled = asset.balanceOf(address(this));
            vault.deposit(recalled, address(this));
            lentAmount = 0;
            emit AssetsRecalled(recalled);
        }
    }

    /// @notice Update lending ratio (only operator)
    /// @param newRatio New lending ratio in basis points
    function setLendingRatio(uint256 newRatio) external onlyOperator {
        require(newRatio <= 10000, "Ratio cannot exceed 100%");
        lendingRatio = newRatio;
    }

    /// @notice Update rebalance interval (only operator)
    /// @param newInterval New interval in seconds
    function setRebalanceInterval(uint256 newInterval) external onlyOperator {
        require(newInterval >= 300, "Interval must be at least 5 minutes");
        rebalanceInterval = newInterval;
    }
}
