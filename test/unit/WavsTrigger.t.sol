// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {Test} from "forge-std/Test.sol";
import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {ITypes} from "interfaces/ITypes.sol";

contract TriggerTest is Test {
    SimpleTrigger public simpleTrigger;

    function setUp() public {
        simpleTrigger = new SimpleTrigger();
    }

    function testTrigger() public {
        simpleTrigger.addTrigger("data1");

        ITypes.TriggerId triggerId = ITypes.TriggerId.wrap(1);
        ITypes.TriggerInfo memory trigger = simpleTrigger.getTrigger(triggerId);

        assertEq(trigger.creator, address(this));
        assertEq(trigger.data, "data1");
        assertEq(ITypes.TriggerId.unwrap(trigger.triggerId), ITypes.TriggerId.unwrap(triggerId));
    }
}
