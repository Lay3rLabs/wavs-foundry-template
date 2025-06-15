// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

import { ERC20 } from "@openzeppelin-contracts/token/ERC20/ERC20.sol";

/// @title MockERC20
/// @notice A simple mock ERC20 token for testing purposes
contract MockERC20 is ERC20 {
    constructor(
        string memory name,
        string memory symbol,
        uint256 initialSupply
    ) ERC20(name, symbol) {
        _mint(msg.sender, initialSupply * 10**decimals());
    }

    /// @notice Mint tokens to any address (for testing)
    /// @param to Address to mint tokens to
    /// @param amount Amount to mint
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    /// @notice Burn tokens from any address (for testing)
    /// @param from Address to burn tokens from
    /// @param amount Amount to burn
    function burn(address from, uint256 amount) external {
        _burn(from, amount);
    }
}