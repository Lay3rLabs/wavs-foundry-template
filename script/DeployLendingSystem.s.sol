// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

import {Script, console} from "forge-std/Script.sol";
import {MockERC20} from "../src/contracts/MockERC20.sol";
import {SolidityVault} from "../src/contracts/SimpleVault.sol";
import {MockLendingProtocol} from "../src/contracts/MockLendingProtocol.sol";
import {VaultLendingStrategy} from "../src/contracts/VaultLendingStrategy.sol";

/// @title DeployLendingSystem
/// @notice Deploys the complete vault lending system for testing
contract DeployLendingSystem is Script {
    // Deployment addresses
    MockERC20 public token;
    SolidityVault public vault;
    MockLendingProtocol public lendingProtocol;
    VaultLendingStrategy public strategy;
    
    // Configuration
    uint256 public constant INITIAL_TOKEN_SUPPLY = 1_000_000; // 1M tokens
    address public operator;
    
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        operator = deployer; // Use deployer as operator for simplicity
        
        vm.startBroadcast(deployerPrivateKey);
        
        console.log("=== Deploying Vault Lending System ===");
        console.log("Deployer:", deployer);
        console.log("Operator:", operator);
        
        // 1. Deploy mock ERC20 token
        token = new MockERC20("Test Token", "TEST", INITIAL_TOKEN_SUPPLY);
        console.log("MockERC20 deployed at:", address(token));
        
        // 2. Deploy vault with the token as underlying asset
        vault = new SolidityVault(address(token));
        console.log("SolidityVault deployed at:", address(vault));
        
        // 3. Deploy mock lending protocol
        lendingProtocol = new MockLendingProtocol(address(token));
        console.log("MockLendingProtocol deployed at:", address(lendingProtocol));
        
        // 4. Deploy vault lending strategy
        strategy = new VaultLendingStrategy(
            address(vault),
            address(lendingProtocol),
            operator
        );
        console.log("VaultLendingStrategy deployed at:", address(strategy));
        
        // 5. Setup initial configuration
        _setupSystem();
        
        vm.stopBroadcast();
        
        console.log("\n=== Deployment Summary ===");
        console.log("Token:", address(token));
        console.log("Vault:", address(vault));
        console.log("Lending Protocol:", address(lendingProtocol));
        console.log("Strategy:", address(strategy));
        console.log("Operator:", operator);
        
        // 6. Save deployment addresses to file
        _saveDeploymentInfo();
    }
    
    function _setupSystem() internal {
        console.log("\n=== Setting up system ===");
        
        // Mint tokens to various accounts for testing
        token.mint(operator, 100_000 * 10**token.decimals()); // 100k to operator
        token.mint(address(strategy), 10_000 * 10**token.decimals()); // 10k to strategy
        
        // Add funds to lending protocol for interest payments
        uint256 protocolFunds = 50_000 * 10**token.decimals();
        token.mint(address(this), protocolFunds);
        token.approve(address(lendingProtocol), protocolFunds);
        lendingProtocol.addProtocolFunds(protocolFunds);
        
        console.log("- Minted tokens to operator and strategy");
        console.log("- Added funds to lending protocol for interest payments");
        
        // Approve vault to spend operator's tokens
        token.approve(address(vault), type(uint256).max);
        
        console.log("- Approved vault to spend operator's tokens");
    }
    
    function _saveDeploymentInfo() internal {
        string memory deploymentInfo = string(abi.encodePacked(
            "DEPLOYMENT_INFO={\n",
            "  \"token\": \"", vm.toString(address(token)), "\",\n",
            "  \"vault\": \"", vm.toString(address(vault)), "\",\n",
            "  \"lendingProtocol\": \"", vm.toString(address(lendingProtocol)), "\",\n",
            "  \"strategy\": \"", vm.toString(address(strategy)), "\",\n",
            "  \"operator\": \"", vm.toString(operator), "\"\n",
            "}\n"
        ));
        
        // This would be saved to a file in a real deployment
        console.log("\n=== Save this deployment info ===");
        console.log(deploymentInfo);
    }
}