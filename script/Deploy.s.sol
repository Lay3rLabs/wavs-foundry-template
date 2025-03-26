// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {stdJson} from "forge-std/StdJson.sol";
import {Strings} from "@openzeppelin-contracts/utils/Strings.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {SimpleSubmit} from "contracts/WavsSubmit.sol";
import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {Common, EigenContracts} from "script/Common.s.sol";

/// @dev Deployment script for SimpleSubmit and SimpleTrigger contracts
contract Deploy is Common {
    using stdJson for string;

    string public root = vm.projectRoot();
    string public deployments_path = string.concat(root, "/.docker/deployments.json");
    string public script_output_path = string.concat(root, "/.docker/script_deploy.json");

    /**
     * @dev Deploys the SimpleSubmit and SimpleTrigger contracts and writes the results to a JSON file
     * @param _serviceManagerAddr The address of the service manager
     */
    function run(string calldata _serviceManagerAddr) public {
        vm.startBroadcast(_privateKey);
        SimpleSubmit _submit = new SimpleSubmit(IWavsServiceManager(vm.parseAddress(_serviceManagerAddr)));
        SimpleTrigger _trigger = new SimpleTrigger();
        vm.stopBroadcast();

        string memory _json = "json";
        _json.serialize("service_handler", toChecksumAddress(address(_submit)));
        _json.serialize("trigger", toChecksumAddress(address(_trigger)));
        string memory _finalJson = _json.serialize("service_manager", _serviceManagerAddr);
        vm.writeFile(script_output_path, _finalJson);
    }

    /**
     * @dev Loads the Eigen contracts from the deployments.json file
     * @return _fixture The Eigen contracts
     */
    function loadEigenContractsFromFS() public view returns (EigenContracts memory _fixture) {
        address _dm = _jsonBytesToAddress(".eigen_core.local.delegation_manager");
        address _rc = _jsonBytesToAddress(".eigen_core.local.rewards_coordinator");
        address _avs = _jsonBytesToAddress(".eigen_core.local.avs_directory");

        _fixture = EigenContracts({delegation_manager: _dm, rewards_coordinator: _rc, avs_directory: _avs});
    }

    /**
     * @dev Loads the service managers from the deployments.json file
     * @return _service_managers The list of service managers
     */
    function loadServiceManagersFromFS() public view returns (address[] memory _service_managers) {
        _service_managers = vm.readFile(deployments_path).readAddressArray(".eigen_service_managers.local");
    }

    // --- Internal Utils ---

    /**
     * @dev Converts a string to an address
     * @param _byteString The string to convert
     * @return _address The address
     */
    function _jsonBytesToAddress(string memory _byteString) internal view returns (address _address) {
        _address = address(uint160(bytes20(vm.readFile(deployments_path).readBytes(_byteString))));
    }

    function toChecksumAddress(address addr) internal pure returns (string memory) {
        // Convert address to hex string without 0x prefix
        bytes memory addressBytes = abi.encodePacked(addr);
        bytes memory addressHex = new bytes(40);

        for (uint256 i = 0; i < 20; i++) {
            uint8 b = uint8(addressBytes[i]);
            addressHex[i*2] = toHexChar(b / 16);
            addressHex[i*2+1] = toHexChar(b % 16);
        }

        // Calculate hash of the hex address (without 0x)
        bytes32 hash = keccak256(addressHex);

        // Create checksummed hex string (with proper capitalization)
        bytes memory result = new bytes(42);
        result[0] = '0';
        result[1] = 'x';

        for (uint256 i = 0; i < 40; i++) {
            uint8 hashByte = uint8(hash[i / 2]);
            uint8 hashValue = i % 2 == 0 ? hashByte >> 4 : hashByte & 0xf;

            // If hash value is >= 8, uppercase the character if it's a letter
            if (hashValue >= 8 && addressHex[i] >= 0x61 && addressHex[i] <= 0x66) {
                // Convert a-f to A-F
                result[i+2] = bytes1(uint8(addressHex[i]) - 32);
            } else {
                result[i+2] = addressHex[i];
            }
        }

        return string(result);
    }

    function toHexChar(uint8 value) internal pure returns (bytes1) {
        if (value < 10) {
            return bytes1(uint8(bytes1('0')) + value);
        } else {
            return bytes1(uint8(bytes1('a')) + value - 10);
        }
    }
}
