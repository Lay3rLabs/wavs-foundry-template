// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

interface ITypes {
    struct DataWithId {
        TriggerId triggerId;
        bytes data;
    }

    struct TriggerInfo {
        TriggerId triggerId;
        address creator;
        bytes data;
    }

    event NewTrigger(bytes);

    type TriggerId is uint64;
}
