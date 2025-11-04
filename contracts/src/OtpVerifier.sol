// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title OtpVerifier
/// @notice Stores OTP hashes with strict attempt limits and admin-governed controls.
contract OtpVerifier {
    struct OtpEntry {
        bytes32 hash;
        uint64 expiry;
        uint8 attempts;
        bool used;
    }

    uint8 public constant MAX_ATTEMPTS = 3;

    mapping(bytes32 => OtpEntry) public entries;

    address public issuer;
    address public admin;
    bool public paused;

    event OtpSet(bytes32 indexed requestId, uint64 expiry);
    event AttemptRecorded(bytes32 indexed requestId, uint8 attemptsMade);
    event OtpLocked(bytes32 indexed requestId);
    event OtpVerified(bytes32 indexed requestId, address indexed by);
    event OtpCleared(bytes32 indexed requestId, address indexed by);
    event IssuerChanged(address indexed oldIssuer, address indexed newIssuer);
    event AdminChanged(address indexed oldAdmin, address indexed newAdmin);
    event Paused(bool paused);

    error Unauthorized(address caller);
    error InvalidExpiry(uint64 provided, uint64 currentTime);
    error EntryExists(bytes32 requestId);
    error UnknownRequest(bytes32 requestId);
    error EntryUsed(bytes32 requestId);
    error EntryExpired(bytes32 requestId, uint64 expiry);
    error AttemptsExceeded(bytes32 requestId);
    error InvalidOtp(bytes32 requestId);
    error ActiveEntry(bytes32 requestId);
    error ZeroAddress();
    error Paused();
    error InvalidHash();

    modifier onlyIssuer() {
        if (msg.sender != issuer) revert Unauthorized(msg.sender);
        _;
    }

    modifier onlyAdmin() {
        if (msg.sender != admin) revert Unauthorized(msg.sender);
        _;
    }

    modifier onlyController() {
        if (msg.sender != admin && msg.sender != issuer) revert Unauthorized(msg.sender);
        _;
    }

    modifier notPaused() {
        if (paused) revert Paused();
        _;
    }

    constructor(address _issuer, address _admin) {
        if (_issuer == address(0) || _admin == address(0)) revert ZeroAddress();
        issuer = _issuer;
        admin = _admin;
    }

    function setOtp(bytes32 requestId, bytes32 otpHash, uint64 expiry) external onlyIssuer notPaused {
        if (expiry <= block.timestamp) revert InvalidExpiry(expiry, uint64(block.timestamp));
        if (entries[requestId].expiry != 0) revert EntryExists(requestId);
        if (otpHash == bytes32(0)) revert InvalidHash();

        entries[requestId] = OtpEntry({
            hash: otpHash,
            expiry: expiry,
            attempts: 0,
            used: false
        });

        emit OtpSet(requestId, expiry);
    }

    function verify(bytes32 requestId, string calldata otp) external notPaused returns (bool) {
        OtpEntry storage entry = entries[requestId];
        if (entry.expiry == 0) revert UnknownRequest(requestId);
        if (entry.used) revert EntryUsed(requestId);
        if (block.timestamp > entry.expiry) revert EntryExpired(requestId, entry.expiry);
        if (entry.attempts >= MAX_ATTEMPTS) revert AttemptsExceeded(requestId);

        bytes32 expected = _hashOtp(otp);
        if (expected != entry.hash) {
            unchecked {
                entry.attempts += 1;
            }
            emit AttemptRecorded(requestId, entry.attempts);
            if (entry.attempts >= MAX_ATTEMPTS) {
                emit OtpLocked(requestId);
            }
            revert InvalidOtp(requestId);
        }

        entry.used = true;
        emit OtpVerified(requestId, msg.sender);
        return true;
    }

    function cleanup(bytes32 requestId) external onlyController {
        OtpEntry storage entry = entries[requestId];
        if (entry.expiry == 0) revert UnknownRequest(requestId);
        if (!entry.used && block.timestamp <= entry.expiry && entry.attempts < MAX_ATTEMPTS) {
            revert ActiveEntry(requestId);
        }

        delete entries[requestId];
        emit OtpCleared(requestId, msg.sender);
    }

    function setIssuer(address newIssuer) external onlyAdmin {
        if (newIssuer == address(0)) revert ZeroAddress();
        address oldIssuer = issuer;
        issuer = newIssuer;
        emit IssuerChanged(oldIssuer, newIssuer);
    }

    function setAdmin(address newAdmin) external onlyAdmin {
        if (newAdmin == address(0)) revert ZeroAddress();
        address oldAdmin = admin;
        admin = newAdmin;
        emit AdminChanged(oldAdmin, newAdmin);
    }

    function pause(bool _paused) external onlyAdmin {
        paused = _paused;
        emit Paused(_paused);
    }

    function _hashOtp(string memory otp) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("OTP:v1", otp));
    }
}
