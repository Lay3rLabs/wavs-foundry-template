// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {SimpleTrigger} from "../src/WavsTrigger.sol";
import {ITypes} from "../src/interfaces/ITypes.sol";

contract TriggerScript is Script {
    uint256 privateKey = vm.envOr(
        "ANVIL_PRIVATE_KEY", uint256(0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80)
    );

    function setUp() public {}

    function run(string calldata serviceTriggerAddr, string calldata coinMarketCapID) public {
        vm.startBroadcast(privateKey);
        SimpleTrigger trigger = SimpleTrigger(vm.parseAddress(serviceTriggerAddr));

        trigger.addTrigger(abi.encodePacked(coinMarketCapID));
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("triggerId:", ITypes.TriggerId.unwrap(triggerId));
        vm.stopBroadcast();
    }
}
