// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Minimal test helpers to avoid pulling full forge-std dependency
interface Vm {
    function warp(uint256) external;
    function prank(address) external;
    function startPrank(address) external;
    function stopPrank() external;
    function expectRevert(bytes calldata) external;
    function assume(bool) external;
}

Vm constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

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

    function assertEq(bytes32 a, bytes32 b, string memory message) internal pure {
        if (a != b) revert AssertionFailed(message);
    }

    function assertEq(uint8 a, uint8 b, string memory message) internal pure {
        if (a != b) revert AssertionFailed(message);
    }

    function assertEq(string memory a, string memory b, string memory message) internal pure {
        bytes memory ba = bytes(a);
        bytes memory bb = bytes(b);
        if (ba.length != bb.length) revert AssertionFailed(message);
        for (uint256 i = 0; i < ba.length; i++) {
            if (ba[i] != bb[i]) revert AssertionFailed(message);
        }
    }

    function assertGt(uint256 a, uint256 b, string memory message) internal pure {
        if (a <= b) revert AssertionFailed(message);
    }
}
