// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {SimpleSubmit} from "contracts/WavsSubmit.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @dev Script to show the result of a trigger
contract ShowResult is Common {
    function getNextTriggerId(string calldata serviceTriggerAddr) public view returns (uint64) {
        SimpleTrigger trigger = SimpleTrigger(vm.parseAddress(serviceTriggerAddr));

        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        uint64 triggerIdUint = uint64(ITypes.TriggerId.unwrap(triggerId));
        console.log("Next TriggerId", triggerIdUint);

        return triggerIdUint;
    }

    function getData(string calldata serviceHandlerAddr, uint64 trigger) public view {
        SimpleSubmit submit = SimpleSubmit(vm.parseAddress(serviceHandlerAddr));

        ITypes.TriggerId triggerId = ITypes.TriggerId.wrap(trigger);
        bytes memory data = submit.getData(triggerId);
        console.log("Data:", string(data));
    }
}
