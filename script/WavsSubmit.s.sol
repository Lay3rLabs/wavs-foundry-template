// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {WavsSubmit} from "../src/WavsSubmit.sol";
import {SimpleTrigger} from "../src/WavsTrigger.sol";

import "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {IDelegationManager} from "@eigenlayer-contracts/interfaces/IDelegationManager.sol";
import {IStrategy} from "@eigenlayer-contracts/interfaces/IStrategy.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {Quorum, StrategyParams} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {Strings} from "@openzeppelin-contracts/utils/Strings.sol";

// forge script ./script/WavsSubmit.s.sol --rpc-url http://localhost:8545 --broadcast
contract WavsSubmitScript is Script {
    using stdJson for string;

    string root = vm.projectRoot();
    string deployments_path = string.concat(root, "/.docker/cli/deployments.json");
    string script_output_path = string.concat(root, "/.docker/cli/script_deploy.json");

    uint256 privateKey = vm.envOr(
        "FOUNDRY_ANVIL_PRIVATE_KEY", uint256(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80)
    );

    function setUp() public {}

    function run() public {
        vm.startBroadcast(privateKey);

        EigenContracts memory eigen = loadEigenContractsFromFS();

        ECDSAStakeRegistry ecdsa_registry = new ECDSAStakeRegistry(IDelegationManager(eigen.delegation_manager));

        console.log("delegation_manager:", eigen.delegation_manager);
        console.log("rewards_coordinator:", eigen.rewards_coordinator);
        console.log("avs_directory:", eigen.avs_directory);

        WavsSubmit submit = new WavsSubmit(
            // eigen.avs_directory, address(ecdsa_registry), eigen.rewards_coordinator, eigen.delegation_manager
        );

        SimpleTrigger trigger = new SimpleTrigger();

        IStrategy mockStrategy = IStrategy(address(0x1234));
        Quorum memory quorum = Quorum({strategies: new StrategyParams[](1)});
        quorum.strategies[0] = StrategyParams({strategy: mockStrategy, multiplier: 10_000});
        ecdsa_registry.initialize(address(submit), 0, quorum);

        vm.stopBroadcast();

        console.log("ecdsa_registry:", address(ecdsa_registry));
        console.log("service_handler:", address(submit));
        console.log("trigger:", address(trigger));

        string memory json = "json";
        json.serialize("service_handler", Strings.toHexString(address(submit)));
        json.serialize("trigger", Strings.toHexString(address(trigger)));
        string memory finalJson = json.serialize("ecdsa_registry", Strings.toHexString(address(ecdsa_registry)));
        vm.writeFile(script_output_path, finalJson);
    }

    function loadEigenContractsFromFS() public view returns (EigenContracts memory) {
        string memory json = vm.readFile(deployments_path);
        address dm = address(uint160(bytes20(json.readBytes(".eigen_core.local.delegation_manager"))));
        address rc = address(uint160(bytes20(json.readBytes(".eigen_core.local.rewards_coordinator"))));
        address avs = address(uint160(bytes20(json.readBytes(".eigen_core.local.avs_directory"))));

        EigenContracts memory fixture =
            EigenContracts({delegation_manager: dm, rewards_coordinator: rc, avs_directory: avs});

        return fixture;
    }
}

struct EigenContracts {
    address delegation_manager;
    address rewards_coordinator;
    address avs_directory;
}
