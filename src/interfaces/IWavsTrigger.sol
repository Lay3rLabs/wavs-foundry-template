// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {ITypes} from "interfaces/ITypes.sol";

interface ISimpleTrigger is ITypes {
    /**
     * @notice Struct to store trigger information
     * @param creator Address of the creator of the trigger
     * @param data Data associated with the trigger
     */
    struct Trigger {
        address creator;
        bytes data;
    }

    /*///////////////////////////////////////////////////////////////
                            LOGIC
    //////////////////////////////////////////////////////////////*/
    /**
     * @notice Add a new trigger
     * @param _data The request data (bytes)
     */
    function addTrigger(bytes memory _data) external;

    /**
     * @notice Get a single trigger by triggerId
     * @param _triggerId The identifier of the trigger
     * @return _triggerInfo The trigger info
     */
    function getTrigger(TriggerId _triggerId) external view returns (TriggerInfo memory _triggerInfo);

    /*///////////////////////////////////////////////////////////////
                            VARIABLES
    //////////////////////////////////////////////////////////////*/
    /**
     * @notice Get the next triggerId
     * @return _triggerId The next triggerId
     */
    function nextTriggerId() external view returns (TriggerId _triggerId);

    /**
     * @notice Get a single trigger by triggerId
     * @param _triggerId The identifier of the trigger
     * @return _creator The creator of the trigger
     * @return _data The data of the trigger
     */
    function triggersById(TriggerId _triggerId) external view returns (address _creator, bytes memory _data);

    /**
     * @notice Get all triggerIds by creator
     * @param _creator The address of the creator
     * @return _triggerIds The triggerIds
     */
    function triggerIdsByCreator(address _creator) external view returns (TriggerId[] memory _triggerIds);
}
