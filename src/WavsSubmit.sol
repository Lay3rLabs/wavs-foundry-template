// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {ILayerServiceManager} from "@wavs/interfaces/ILayerServiceManager.sol";
import {ILayerServiceHandler} from "@wavs/interfaces/ILayerServiceHandler.sol";

import {ITypes} from "./interfaces/ITypes.sol";

contract SimpleSubmit is ILayerServiceHandler {
    ILayerServiceManager private _serviceManager;

    mapping(ITypes.TriggerId => bool) validTriggers;
    mapping(ITypes.TriggerId => bytes) datas;
    mapping(ITypes.TriggerId => bytes) signatures;

    constructor(ILayerServiceManager serviceManager) {
        _serviceManager = serviceManager;
    }

    function handleSignedData(bytes calldata data, bytes calldata signature) external {
        _serviceManager.validate(data, signature);

        ITypes.DataWithId memory dataWithId = abi.decode(data, (ITypes.DataWithId));

        signatures[dataWithId.triggerId] = signature;
        datas[dataWithId.triggerId] = dataWithId.data;
        validTriggers[dataWithId.triggerId] = true;
    }

    function isValidTriggerId(ITypes.TriggerId triggerId) external view returns (bool) {
        return validTriggers[triggerId];
    }

    function getSignature(ITypes.TriggerId triggerId) external view returns (bytes memory signature) {
        signature = signatures[triggerId];
    }

    function getData(ITypes.TriggerId triggerId) external view returns (bytes memory data) {
        data = datas[triggerId];
    }
}
