// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../src/OtpVerifier.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";

/// @title AdminActions
/// @notice Foundry playbook for pausing and rotating keys on the deployed OtpVerifier contract.
contract AdminActions is Script {
    error AdminKeyNotConfigured();

    function pause(address verifier, bool paused) external {
        uint256 adminKey = _adminKey();
        vm.startBroadcast(adminKey);
        OtpVerifier(verifier).pause(paused);
        vm.stopBroadcast();
        console.log("Pause toggled to %s for verifier %s", paused, verifier);
    }

    function rotateIssuer(address verifier, address newIssuer) external {
        if (newIssuer == address(0)) {
            revert("new issuer cannot be zero address");
        }
        uint256 adminKey = _adminKey();
        vm.startBroadcast(adminKey);
        OtpVerifier(verifier).setIssuer(newIssuer);
        vm.stopBroadcast();
        console.log("Issuer rotated to %s for verifier %s", newIssuer, verifier);
    }

    function rotateAdmin(address verifier, address newAdmin) external {
        if (newAdmin == address(0)) {
            revert("new admin cannot be zero address");
        }
        uint256 adminKey = _adminKey();
        vm.startBroadcast(adminKey);
        OtpVerifier(verifier).setAdmin(newAdmin);
        vm.stopBroadcast();
        console.log("Admin rotated to %s for verifier %s", newAdmin, verifier);
    }

    function _adminKey() private view returns (uint256) {
        if (!vm.envExists("ADMIN_PRIVATE_KEY")) {
            revert AdminKeyNotConfigured();
        }
        return vm.envUint("ADMIN_PRIVATE_KEY");
    }
}
