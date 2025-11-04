// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../src/OtpVerifier.sol";

/// @notice Lightweight deployment wrapper compatible with `forge script`.
contract DeployOtpVerifier {
    function deploy(address issuer) external returns (OtpVerifier) {
        return new OtpVerifier(issuer);
    }
}
