// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MyContract {
    event Request(uint64 x);

    function emitEvent(uint64 x) external {
        emit Request(x);
    }
}