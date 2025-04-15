// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @dev Script to add a new trigger
contract Trigger is Common {
    function run(string calldata serviceTriggerAddr, string calldata coinMarketCapID) public {
        vm.startBroadcast(_privateKey);
        SimpleTrigger trigger = SimpleTrigger(vm.parseAddress(serviceTriggerAddr));

        trigger.addTrigger(abi.encodePacked(coinMarketCapID));
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("TriggerId", ITypes.TriggerId.unwrap(triggerId));
        vm.stopBroadcast();
    }
}
