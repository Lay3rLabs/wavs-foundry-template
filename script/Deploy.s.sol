// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Strings} from "@openzeppelin-contracts/utils/Strings.sol";

import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

import {SimpleSubmit} from "../src/WavsSubmit.sol";
import {SimpleTrigger} from "../src/WavsTrigger.sol";

// forge script ./script/Deploy.s.sol ${SERVICE_MANAGER} --sig "run(string)" --rpc-url http://localhost:8545 --broadcast
contract DeployScript is Script {
    using stdJson for string;

    string root = vm.projectRoot();
    string deployments_path = string.concat(root, "/.docker/deployments.json");
    string script_output_path = string.concat(root, "/.docker/script_deploy.json");

    uint256 privateKey = vm.envOr(
        "ANVIL_PRIVATE_KEY", uint256(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80)
    );

    function setUp() public {}

    function run(string calldata serviceManagerAddr) public {
        vm.startBroadcast(privateKey);

        SimpleSubmit submit = new SimpleSubmit(IWavsServiceManager(vm.parseAddress(serviceManagerAddr)));
        SimpleTrigger trigger = new SimpleTrigger();

        vm.stopBroadcast();

        string memory json = "json";
        json.serialize("service_handler", Strings.toHexString(address(submit)));
        json.serialize("trigger", Strings.toHexString(address(trigger)));
        string memory finalJson = json.serialize("service_manager", serviceManagerAddr);
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
