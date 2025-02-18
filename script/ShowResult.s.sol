// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {SimpleSubmit} from "contracts/WavsSubmit.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @dev Script to show the result of a trigger
contract ShowResult is Common {
    function run(string calldata serviceTriggerAddr, string calldata serviceHandlerAddr) public {
        vm.startBroadcast(_privateKey);
        SimpleTrigger trigger = SimpleTrigger(vm.parseAddress(serviceTriggerAddr));
        SimpleSubmit submit = SimpleSubmit(vm.parseAddress(serviceHandlerAddr));

        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("Fetching data for TriggerId", ITypes.TriggerId.unwrap(triggerId));

        bytes memory data = submit.getData(triggerId);
        console.log("Data:", string(data));

        vm.stopBroadcast();
    }
}
