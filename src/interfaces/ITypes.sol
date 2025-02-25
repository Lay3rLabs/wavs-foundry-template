// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

interface ITypes {
    /**
     * @notice Struct to store trigger information
     * @param triggerId Unique identifier for the trigger
     * @param data Data associated with the triggerId
     */
    struct DataWithId {
        TriggerId triggerId;
        bytes data;
    }

    /**
     * @notice Struct to store trigger information
     * @param triggerId Unique identifier for the trigger
     * @param creator Address of the creator of the trigger
     * @param data Data associated with the trigger
     */
    struct TriggerInfo {
        TriggerId triggerId;
        address creator;
        bytes data;
    }

    /**
     * @notice Event emitted when a new trigger is created
     * @param _triggerInfo Encoded TriggerInfo struct
     */
    event NewTrigger(bytes _triggerInfo);

    /// @notice TriggerId is a unique identifier for a trigger
    type TriggerId is uint64;
}
