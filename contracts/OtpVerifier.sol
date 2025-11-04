// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract OtpVerifier {
    struct OtpEntry { 
        bytes32 hash; 
        uint64 expiry; 
        bool used; 
    }
    
    mapping(bytes32 => OtpEntry) public entries; // requestId -> entry
    mapping(bytes32 => uint8) public attempts; // requestId -> failed attempts
    address public issuer; // your backend relayer
    bool public paused; // emergency pause mechanism

    event OtpSet(bytes32 indexed requestId, uint64 expiry);
    event OtpVerified(bytes32 indexed requestId, address indexed by);
    event Paused(bool paused);
    event IssuerChanged(address indexed oldIssuer, address indexed newIssuer);
    event AttemptFailed(bytes32 indexed requestId, uint8 attemptsMade);

    modifier onlyIssuer() { 
        require(msg.sender == issuer, "not issuer"); 
        _; 
    }
    
    modifier notPaused() { 
        require(!paused, "paused"); 
        _; 
    }

    constructor(address _issuer) { 
        issuer = _issuer; 
    }

    function setOtp(bytes32 requestId, bytes32 otpHash, uint64 expiry) 
        external 
        onlyIssuer 
        notPaused 
    {
        require(expiry > block.timestamp, "bad expiry");
        require(entries[requestId].expiry == 0, "exists");
        require(attempts[requestId] == 0, "attempts exist");
        entries[requestId] = OtpEntry({
            hash: otpHash, 
            expiry: expiry, 
            used: false
        });
        emit OtpSet(requestId, expiry);
    }

    function verify(bytes32 requestId, string calldata otp) 
        external 
        notPaused 
        returns (bool) 
    {
        OtpEntry storage e = entries[requestId];
        require(e.expiry != 0, "no entry");
        require(!e.used, "used");
        require(block.timestamp <= e.expiry, "expired");
        
        // Check attempt limit (max 3 attempts)
        require(attempts[requestId] < 3, "too many attempts");
        
        // Check OTP
        if (keccak256(abi.encodePacked(otp)) != e.hash) {
            attempts[requestId] += 1;
            emit AttemptFailed(requestId, attempts[requestId]);
            revert("invalid");
        }
        
        // Reset attempts on success
        delete attempts[requestId];
        
        // Mark as used
        e.used = true;
        emit OtpVerified(requestId, msg.sender);
        return true;
    }
    
    // Emergency pause functionality
    function pause(bool _paused) external onlyIssuer {
        paused = _paused;
        emit Paused(_paused);
    }
    
    // Issuer rotation for key management
    function setIssuer(address newIssuer) external onlyIssuer {
        emit IssuerChanged(issuer, newIssuer);
        issuer = newIssuer;
    }
    
    // Helper function for hashing OTP with domain separation as recommended
    function _hashOtp(string memory otp) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("OTP:v1:", otp));
    }
}