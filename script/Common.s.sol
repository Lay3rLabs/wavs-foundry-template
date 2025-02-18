// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {Script} from "forge-std/Script.sol";

/// @dev Struct to store Eigen contracts
struct EigenContracts {
    address delegation_manager;
    address rewards_coordinator;
    address avs_directory;
}

/// @dev Common script for all deployment scripts
contract Common is Script {
    uint256 internal _privateKey =
        vm.envOr("ANVIL_PRIVATE_KEY", uint256(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80));
}
