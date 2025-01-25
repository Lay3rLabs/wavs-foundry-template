// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {WavsServiceManager} from "../src/WavsServiceManager.sol";
import {ECDSAStakeRegistry} from "@eigenlayer/middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IDelegationManager} from
    "@eigenlayer/middleware/lib/eigenlayer-contracts/src/contracts/interfaces/IDelegationManager.sol";
import {Quorum, StrategyParams} from "@eigenlayer/middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {IStrategy} from "@eigenlayer/middleware/lib/eigenlayer-contracts/src/contracts/interfaces/IStrategy.sol";
import {stdJson} from "forge-std/StdJson.sol";

struct EigenContracts {
    address delegation_manager;
    address rewards_coordinator;
    address avs_directory;
}

// forge script ./script/WavsServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast --var-ir
contract WavsServiceManagerScript is Script {
    using stdJson for string;

    uint256 privateKey = vm.envOr(
        "FOUNDRY_ANVIL_PRIVATE_KEY", uint256(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80)
    );

    function setUp() public {}

    function run() public {
        vm.startBroadcast(privateKey);

        EigenContracts memory eigen = loadEigenContractsFromFS("deployments.json");

        ECDSAStakeRegistry ecdsa_registry = new ECDSAStakeRegistry(IDelegationManager(eigen.delegation_manager));

        console.log("delegation_manager:", eigen.delegation_manager);
        console.log("rewards_coordinator:", eigen.rewards_coordinator);
        console.log("avs_directory:", eigen.avs_directory);

        WavsServiceManager sm = new WavsServiceManager(
            eigen.avs_directory, address(ecdsa_registry), eigen.rewards_coordinator, eigen.delegation_manager
        );

        IStrategy mockStrategy = IStrategy(address(0x1234));
        Quorum memory quorum = Quorum({strategies: new StrategyParams[](1)});
        quorum.strategies[0] = StrategyParams({strategy: mockStrategy, multiplier: 10_000});
        ecdsa_registry.initialize(address(sm), 0, quorum);

        vm.stopBroadcast();

        console.log("ServiceManager:", address(sm));
        console.log("ecdssa_registry (deployed):", address(ecdsa_registry));
    }

    function loadEigenContractsFromFS(string memory fileName) public view returns (EigenContracts memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/.docker/cli/", fileName);
        string memory json = vm.readFile(path);

        address dm = address(uint160(bytes20(json.readBytes(".eigen_core.local.delegation_manager"))));
        address rc = address(uint160(bytes20(json.readBytes(".eigen_core.local.rewards_coordinator"))));
        address avs = address(uint160(bytes20(json.readBytes(".eigen_core.local.avs_directory"))));

        EigenContracts memory fixture =
            EigenContracts({delegation_manager: dm, rewards_coordinator: rc, avs_directory: avs});

        return fixture;
    }
}
