

// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import "forge-std/Script.sol";

contract CmdRunner is Script{
    // `ffi = true` must be set in foundry.toml
    //
    // Run a command and return the output by creating a temporary script with
    // the entire command and running it via bash. This gets around the limits
    // of FFI, such as not being able to pipe between two commands.
    // string memory entry = runCmd(string.concat("curl -s ", url, " | jq -c .tree[0]"));
    function runCmd(string memory cmd) external returns (string memory) {
        string memory script = string.concat(vm.projectRoot(), "/.ffirun.sh");
        // Save the cmd to a file
        vm.writeFile(script, cmd);
        // Run the cmd
        string[] memory exec = new string[](2);
        exec[0] = "bash";
        exec[1] = script;
        string memory result = string(vm.ffi(exec));
        // Delete the file
        vm.removeFile(script);
        // Return the result
        return result;
    }
}
