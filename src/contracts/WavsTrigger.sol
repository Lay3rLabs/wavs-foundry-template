// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {ISimpleTrigger} from "interfaces/IWavsTrigger.sol";

contract SimpleTrigger is ISimpleTrigger {
    /// @inheritdoc ISimpleTrigger
    TriggerId public nextTriggerId;

    /// @inheritdoc ISimpleTrigger
    mapping(TriggerId _triggerId => Trigger _trigger) public triggersById;
    /// @notice See ISimpleTrigger.triggerIdsByCreator
    mapping(address _creator => TriggerId[] _triggerIds) internal _triggerIdsByCreator;

    /// @inheritdoc ISimpleTrigger
    function addTrigger(bytes memory _data) external {
        // Get the next trigger id
        nextTriggerId = TriggerId.wrap(TriggerId.unwrap(nextTriggerId) + 1);
        TriggerId _triggerId = nextTriggerId;

        // Create the trigger
        Trigger memory _trigger = Trigger({creator: msg.sender, data: _data});

        // Update storages
        triggersById[_triggerId] = _trigger;
        _triggerIdsByCreator[msg.sender].push(_triggerId);

        TriggerInfo memory _triggerInfo =
            TriggerInfo({triggerId: _triggerId, creator: _trigger.creator, data: _trigger.data});

        emit NewTrigger(abi.encode(_triggerInfo));
    }

    /// @inheritdoc ISimpleTrigger
    function getTrigger(TriggerId triggerId) external view override returns (TriggerInfo memory _triggerInfo) {
        Trigger storage _trigger = triggersById[triggerId];
        _triggerInfo = TriggerInfo({triggerId: triggerId, creator: _trigger.creator, data: _trigger.data});
    }

    /// @inheritdoc ISimpleTrigger
    function triggerIdsByCreator(address _creator) external view returns (TriggerId[] memory _triggerIds) {
        _triggerIds = _triggerIdsByCreator[_creator];
    }
}
