// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Minimal test helpers to avoid pulling full forge-std dependency
contract TestBase {
    error AssertionFailed(string message);

    function assertTrue(bool condition, string memory message) internal pure {
        if (!condition) {
            revert AssertionFailed(message);
        }
    }

    function assertEq(address a, address b, string memory message) internal pure {
        if (a != b) revert AssertionFailed(message);
    }

    function assertEq(bool a, bool b, string memory message) internal pure {
        if (a != b) revert AssertionFailed(message);
    }

    function assertEq(uint256 a, uint256 b, string memory message) internal pure {
        if (a != b) revert AssertionFailed(message);
    }
}
