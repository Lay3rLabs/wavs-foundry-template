// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @dev Script to add a new trigger
contract Trigger is Common {
    function run(string calldata serviceTriggerAddr, string calldata coinMarketCapID) public {
        vm.startBroadcast(_privateKey);
        SimpleTrigger trigger = SimpleTrigger(vm.parseAddress(serviceTriggerAddr));

        trigger.addTrigger(coinMarketCapID);
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("TriggerId", ITypes.TriggerId.unwrap(triggerId));
        vm.stopBroadcast();
    }
}
