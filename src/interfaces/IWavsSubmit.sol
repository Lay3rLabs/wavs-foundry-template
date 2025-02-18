// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {ITypes} from "interfaces/ITypes.sol";

interface ISimpleSubmit is ITypes {
    /*///////////////////////////////////////////////////////////////
                            VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/
    /**
     * @notice Check if a triggerId is valid
     * @param _triggerId The identifier of the trigger
     * @return _isValid True if the trigger is valid, false otherwise
     */
    function isValidTriggerId(TriggerId _triggerId) external view returns (bool _isValid);

    /**
     * @notice Get the signature for a triggerId
     * @param _triggerId The identifier of the trigger
     * @return _signature The signature associated with the trigger
     */
    function getSignature(TriggerId _triggerId) external view returns (bytes memory _signature);

    /**
     * @notice Get the data for a triggerId
     * @param _triggerId The identifier of the trigger
     * @return _data The data associated with the trigger
     */
    function getData(TriggerId _triggerId) external view returns (bytes memory _data);
}
