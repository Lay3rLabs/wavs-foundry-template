// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {SimpleSubmit} from "../src/WavsSubmit.sol";
import {SimpleTrigger} from "../src/WavsTrigger.sol";
import {LayerServiceManager} from "../src/LayerServiceManager.sol";

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

        address[] memory service_managers = loadServiceManagersFromFS();
        LayerServiceManager service_manager = LayerServiceManager(service_managers[service_managers.length - 1]);

        SimpleSubmit submit = new SimpleSubmit(service_manager);
        SimpleTrigger trigger = new SimpleTrigger();

        vm.stopBroadcast();

        string memory json = "json";
        json.serialize("service_handler", Strings.toHexString(address(submit)));
        json.serialize("trigger", Strings.toHexString(address(trigger)));
        string memory finalJson = json.serialize("service_manager", Strings.toHexString(address(service_manager)));
        vm.writeFile(script_output_path, finalJson);
    }

    function loadEigenContractsFromFS() public view returns (EigenContracts memory) {
        string memory json = vm.readFile(deployments_path);
        address dm = address(uint160(bytes20(json.readBytes(".eigen_core.local.delegation_manager"))));
        address rc = address(uint160(bytes20(json.readBytes(".eigen_core.local.rewards_coordinator"))));
        address avs = address(uint160(bytes20(json.readBytes(".eigen_core.local.avs_directory"))));

        EigenContracts memory fixture = EigenContracts({delegation_manager: dm, rewards_coordinator: rc, avs_directory: avs});

        return fixture;
    }

    function loadServiceManagersFromFS() public view returns (address[] memory) {
        string memory json = vm.readFile(deployments_path);
        address[] memory service_managers = json.readAddressArray(".eigen_service_managers.local");
        return service_managers;
    }
}

struct EigenContracts {
    address delegation_manager;
    address rewards_coordinator;
    address avs_directory;
}
