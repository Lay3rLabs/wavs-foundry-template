// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {ITypes} from "interfaces/ITypes.sol";

contract SimpleSubmit is ITypes, IWavsServiceHandler {
    /// @notice Mapping of valid triggers
    mapping(TriggerId _triggerId => bool _isValid) internal _validTriggers;
    /// @notice Mapping of trigger data
    mapping(TriggerId _triggerId => bytes _data) internal _datas;
    /// @notice Mapping of trigger signatures
    mapping(TriggerId _triggerId => SignatureData _signature) internal _signatures;

    /// @notice Service manager instance
    IWavsServiceManager private _serviceManager;

    /**
     * @notice Initialize the contract
     * @param serviceManager The service manager instance
     */
    constructor(IWavsServiceManager serviceManager) {
        _serviceManager = serviceManager;
    }

    /// @inheritdoc IWavsServiceHandler
    function handleSignedEnvelope(Envelope calldata envelope, SignatureData calldata signatureData) external {
        _serviceManager.validate(envelope, signatureData);

        DataWithId memory dataWithId = abi.decode(envelope.payload, (DataWithId));

        _signatures[dataWithId.triggerId] = signatureData;
        _datas[dataWithId.triggerId] = dataWithId.data;
        _validTriggers[dataWithId.triggerId] = true;
    }

    function isValidTriggerId(TriggerId _triggerId) external view returns (bool _isValid) {
        _isValid = _validTriggers[_triggerId];
    }

    function getSignature(TriggerId _triggerId) external view returns (SignatureData memory _signature) {
        _signature = _signatures[_triggerId];
    }

    function getData(TriggerId _triggerId) external view returns (bytes memory _data) {
        _data = _datas[_triggerId];
    }
}
