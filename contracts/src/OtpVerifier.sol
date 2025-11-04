// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title OtpVerifier
/// @notice Minimal on-chain anchor for hashed OTP metadata. Designed to be chain-agnostic and auditable.
contract OtpVerifier {
    struct OtpEntry {
        bytes32 hash;
        uint64 expiry;
        bool used;
    }

    mapping(bytes32 => OtpEntry) public entries;

    address public issuer;
    bool public paused;

    event OtpSet(bytes32 indexed requestId, uint64 expiry);
    event OtpVerified(bytes32 indexed requestId, address indexed by);
    event IssuerChanged(address indexed oldIssuer, address indexed newIssuer);
    event Paused(bool paused);

    error NotIssuer();
    error AlreadyExists();
    error InvalidExpiry();
    error AlreadyUsed();
    error Expired();
    error InvalidOtp();
    error PausedError();

    modifier onlyIssuer() {
        if (msg.sender != issuer) revert NotIssuer();
        _;
    }

    modifier notPaused() {
        if (paused) revert PausedError();
        _;
    }

    constructor(address _issuer) {
        require(_issuer != address(0), "issuer required");
        issuer = _issuer;
    }

    function setOtp(bytes32 requestId, bytes32 otpHash, uint64 expiry) external onlyIssuer notPaused {
        if (expiry <= block.timestamp) revert InvalidExpiry();
        if (entries[requestId].expiry != 0) revert AlreadyExists();
        entries[requestId] = OtpEntry({hash: otpHash, expiry: expiry, used: false});
        emit OtpSet(requestId, expiry);
    }

    function verify(bytes32 requestId, string calldata otp) external notPaused returns (bool) {
        OtpEntry storage entry = entries[requestId];
        if (entry.expiry == 0) revert InvalidOtp();
        if (entry.used) revert AlreadyUsed();
        if (block.timestamp > entry.expiry) revert Expired();
        if (keccak256(abi.encodePacked(otp)) != entry.hash) revert InvalidOtp();
        entry.used = true;
        emit OtpVerified(requestId, msg.sender);
        return true;
    }

    function setIssuer(address newIssuer) external onlyIssuer {
        require(newIssuer != address(0), "issuer required");
        emit IssuerChanged(issuer, newIssuer);
        issuer = newIssuer;
    }

    function pause(bool _paused) external onlyIssuer {
        paused = _paused;
        emit Paused(_paused);
    }
}
