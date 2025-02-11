// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {ITypes} from "./interfaces/ITypes.sol";

contract SimpleTrigger {
    // Data structures
    struct Trigger {
        address creator;
        bytes data;
    }

    // Storage

    mapping(ITypes.TriggerId => Trigger) public triggersById;

    mapping(address => ITypes.TriggerId[]) public triggerIdsByCreator;

    // Global vars
    ITypes.TriggerId public nextTriggerId;

    // Functions

    /**
     * @notice Add a new trigger.
     * @param data The request data (bytes).
     */
    function addTrigger(bytes memory data) public {
        // Get the next trigger id
        nextTriggerId = ITypes.TriggerId.wrap(ITypes.TriggerId.unwrap(nextTriggerId) + 1);
        ITypes.TriggerId triggerId = nextTriggerId;

        // Create the trigger
        Trigger memory trigger = Trigger({creator: msg.sender, data: data});

        // update storages
        triggersById[triggerId] = trigger;
        triggerIdsByCreator[msg.sender].push(triggerId);

        ITypes.TriggerInfo memory triggerInfo =
            ITypes.TriggerInfo({triggerId: triggerId, creator: trigger.creator, data: trigger.data});

        emit ITypes.NewTrigger(abi.encode(triggerInfo));
    }

    /**
     * @notice Get a single trigger by triggerId.
     * @param triggerId The identifier of the trigger.
     */
    function getTrigger(ITypes.TriggerId triggerId) public view returns (ITypes.TriggerInfo memory) {
        Trigger storage trigger = triggersById[triggerId];

        return ITypes.TriggerInfo({triggerId: triggerId, creator: trigger.creator, data: trigger.data});
    }
}
