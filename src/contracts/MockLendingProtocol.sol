// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

import { IERC20 } from "@openzeppelin-contracts/token/ERC20/IERC20.sol";

/// @title MockLendingProtocol
/// @notice A simple mock lending protocol for testing vault strategies
/// @dev This contract simulates a lending protocol that accepts deposits and pays interest
contract MockLendingProtocol {
    IERC20 public immutable asset;

    /// @notice Interest rate per second (5% APY ≈ 1.585489599e-9 per second)
    uint256 public constant INTEREST_RATE_PER_SECOND = 1585489599e0;

    /// @notice Mapping of depositor to their deposit info
    mapping(address => DepositInfo) public deposits;

    /// @notice Total amount deposited in the protocol
    uint256 public totalDeposits;

    struct DepositInfo {
        uint256 principal;      // Original deposit amount
        uint256 lastUpdate;     // Last time interest was calculated
        uint256 accruedInterest; // Interest accrued so far
    }

    event Deposited(address indexed depositor, uint256 amount);
    event Withdrawn(address indexed depositor, uint256 principal, uint256 interest);
    event InterestAccrued(address indexed depositor, uint256 interest);

    constructor(address _asset) {
        asset = IERC20(_asset);
    }

    /// @notice Deposit tokens to start earning interest
    /// @param amount Amount of tokens to deposit
    function deposit(uint256 amount) external {
        require(amount > 0, "Amount must be greater than 0");

        // Update interest before modifying deposit
        _updateInterest(msg.sender);

        // Transfer tokens from depositor
        asset.transferFrom(msg.sender, address(this), amount);

        // Update deposit info
        deposits[msg.sender].principal += amount;
        deposits[msg.sender].lastUpdate = block.timestamp;
        totalDeposits += amount;

        emit Deposited(msg.sender, amount);
    }

    /// @notice Withdraw all principal and accrued interest
    function withdraw() external {
        DepositInfo storage depositInfo = deposits[msg.sender];
        require(depositInfo.principal > 0, "No deposit found");

        // Update interest before withdrawal
        _updateInterest(msg.sender);

        uint256 principal = depositInfo.principal;
        uint256 interest = depositInfo.accruedInterest;
        uint256 totalAmount = principal + interest;

        // Clear deposit info
        depositInfo.principal = 0;
        depositInfo.accruedInterest = 0;
        depositInfo.lastUpdate = 0;
        
        // Safely update totalDeposits to avoid underflow
        if (totalDeposits >= principal) {
            totalDeposits -= principal;
        } else {
            totalDeposits = 0;
        }

        // Transfer tokens back to depositor
        asset.transfer(msg.sender, totalAmount);

        emit Withdrawn(msg.sender, principal, interest);
    }

    /// @notice Get current balance (principal + accrued interest) for a depositor
    /// @param depositor Address of the depositor
    /// @return total Current total balance
    function getBalance(address depositor) external view returns (uint256 total) {
        DepositInfo memory depositInfo = deposits[depositor];
        if (depositInfo.principal == 0) return 0;

        uint256 timeElapsed = block.timestamp - depositInfo.lastUpdate;
        uint256 newInterest = (depositInfo.principal * INTEREST_RATE_PER_SECOND * timeElapsed) / 1e18;

        return depositInfo.principal + depositInfo.accruedInterest + newInterest;
    }

    /// @notice Update accrued interest for a depositor
    /// @param depositor Address of the depositor
    function _updateInterest(address depositor) internal {
        DepositInfo storage depositInfo = deposits[depositor];
        if (depositInfo.principal == 0) return;

        uint256 timeElapsed = block.timestamp - depositInfo.lastUpdate;
        if (timeElapsed > 0) {
            uint256 newInterest = (depositInfo.principal * INTEREST_RATE_PER_SECOND * timeElapsed) / 1e18;
            depositInfo.accruedInterest += newInterest;
            depositInfo.lastUpdate = block.timestamp;

            emit InterestAccrued(depositor, newInterest);
        }
    }

    /// @notice Emergency function to add funds to the protocol (for testing)
    /// @dev In a real protocol, this would be funded through borrowers paying interest
    function addProtocolFunds(uint256 amount) external {
        asset.transferFrom(msg.sender, address(this), amount);
    }
}
