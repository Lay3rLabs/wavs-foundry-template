// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {SimpleSubmit} from "contracts/WavsSubmit.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @dev Script to show the result of a trigger
contract ShowResult is Common {
    function trigger(string calldata serviceTriggerAddr) public view {
        SimpleTrigger triggerInstance = SimpleTrigger(vm.parseAddress(serviceTriggerAddr));
        ITypes.TriggerId triggerId = triggerInstance.nextTriggerId();

        console.log("TriggerID:", ITypes.TriggerId.unwrap(triggerId));
    }

    function data(string calldata serviceHandlerAddr, uint64 triggerId) public view {
        SimpleSubmit submit = SimpleSubmit(vm.parseAddress(serviceHandlerAddr));

        ITypes.TriggerId triggerIdTyped = ITypes.TriggerId.wrap(triggerId);

        bool isValid = submit.isValidTriggerId(triggerIdTyped);
        if(!isValid) {
            console.log("Trigger ID:", triggerId, " is not valid");
        }

        bytes memory triggerData = submit.getData(triggerIdTyped);
        console.log("TriggerID:", triggerId);
        console.log("Data:", string(triggerData));
    }


}
