// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/OtpVerifier.sol";

contract SecurityLayerTest is Test {
    OtpVerifier private verifier;
    address private issuer = address(0xBEEF);
    address private admin = address(this);
    address private attacker = address(0xa710CA71a710ca71A710cA71a710Ca71A710ca71);

    function setUp() public {
        verifier = new OtpVerifier(issuer, admin);
    }

    // Test brute force attack prevention
    function testBruteForceAttackPrevention() public {
        bytes32 requestId = keccak256("BRUTE_FORCE_TEST");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 correctHash = _hashOtp("123456");

        // Set up the OTP
        vm.prank(issuer);
        verifier.setOtp(requestId, correctHash, expiry);

        // Try to brute force with wrong OTPs
        for (uint8 i = 0; i < 3; i++) {
            bool invalid = verifier.verify(requestId, "000000");
            assertTrue(!invalid, "Brute force attempt should fail");
        }

        // After 3 attempts, should be locked
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.AttemptsExceeded.selector, requestId));
        verifier.verify(requestId, "123456");
    }

    // Test replay attack prevention
    function testReplayAttackPrevention() public {
        bytes32 requestId = keccak256("REPLAY_TEST");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 correctHash = _hashOtp("123456");

        // Set up the OTP
        vm.prank(issuer);
        verifier.setOtp(requestId, correctHash, expiry);

        // First verification should succeed
        bool result = verifier.verify(requestId, "123456");
        assertTrue(result, "First verification should succeed");

        // Second verification with same OTP should fail
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.EntryUsed.selector, requestId));
        verifier.verify(requestId, "123456");
    }

    // Test expiration attack prevention
    function testExpirationAttackPrevention() public {
        bytes32 requestId = keccak256("EXPIRATION_TEST");
        uint64 expiry = uint64(block.timestamp + 10); // Short expiry
        bytes32 correctHash = _hashOtp("123456");

        // Set up the OTP
        vm.prank(issuer);
        verifier.setOtp(requestId, correctHash, expiry);

        // Advance time past expiration
        vm.warp(block.timestamp + 15);

        // Verification should fail due to expiration
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.EntryExpired.selector, requestId, expiry));
        verifier.verify(requestId, "123456");
    }

    // Test unauthorized access prevention
    function testUnauthorizedAccessPrevention() public {
        bytes32 requestId = keccak256("UNAUTHORIZED_TEST");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 correctHash = _hashOtp("123456");

        // Attacker tries to set OTP
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, attacker));
        verifier.setOtp(requestId, correctHash, expiry);

        // Admin sets the OTP correctly
        vm.prank(issuer);
        verifier.setOtp(requestId, correctHash, expiry);

        // Attacker tries to verify (should work since verification doesn't require special permissions)
        // But with wrong OTP, it should fail
        vm.prank(attacker);
        bool invalidAttempt = verifier.verify(requestId, "000000");
        assertTrue(!invalidAttempt, "Attacker verification should fail");
    }

    // Test emergency pause functionality
    function testEmergencyPause() public {
        bytes32 requestId = keccak256("PAUSE_TEST");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 correctHash = _hashOtp("123456");

        // Admin pauses the contract
        verifier.pause(true);

        // Try to set OTP while paused
        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ContractPaused.selector));
        verifier.setOtp(requestId, correctHash, expiry);

        // Try to verify while paused
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ContractPaused.selector));
        verifier.verify(requestId, "123456");

        // Admin unpauses the contract
        verifier.pause(false);

        // Now operations should work
        vm.prank(issuer);
        verifier.setOtp(requestId, correctHash, expiry);
        
        bool result = verifier.verify(requestId, "123456");
        assertTrue(result, "Verification should work after unpausing");
    }

    // Test unauthorized admin functions
    function testUnauthorizedAdminFunctions() public {
        address newIssuer = address(0x1234);
        address newAdmin = address(0x5678);

        // Attacker tries to change issuer
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, attacker));
        verifier.setIssuer(newIssuer);

        // Attacker tries to change admin
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, attacker));
        verifier.setAdmin(newAdmin);

        // Attacker tries to pause
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, attacker));
        verifier.pause(true);
    }

    // Test cleanup security
    function testCleanupSecurity() public {
        bytes32 requestId = keccak256("CLEANUP_TEST");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 correctHash = _hashOtp("123456");

        // Set up the OTP
        vm.prank(issuer);
        verifier.setOtp(requestId, correctHash, expiry);

        // Non-controller tries to cleanup
        vm.prank(attacker);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, attacker));
        verifier.cleanup(requestId);

        // Admin (controller) tries to cleanup active entry
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ActiveEntry.selector, requestId));
        verifier.cleanup(requestId);

        // Verify the OTP
        verifier.verify(requestId, "123456");

        // Now admin can cleanup
        verifier.cleanup(requestId);

        // Verify entry is cleared
        (bytes32 hash, uint64 exp, uint8 attempts, bool used) = verifier.entries(requestId);
        assertEq(hash, bytes32(0), "Hash should be zero after cleanup");
        assertEq(exp, 0, "Expiry should be zero after cleanup");
        assertEq(attempts, 0, "Attempts should be zero after cleanup");
        assertTrue(!used, "Used should be false after cleanup");
    }

    // Test hash collision resistance
    function testHashCollisionResistance() public {
        bytes32 requestId1 = keccak256("REQUEST_1");
        bytes32 requestId2 = keccak256("REQUEST_2");
        uint64 expiry = uint64(block.timestamp + 60);
        
        // Same OTP for both requests but different request IDs
        bytes32 hash1 = _hashOtp("123456");
        bytes32 hash2 = _hashOtp("123456");

        // Set up both OTPs
        vm.prank(issuer);
        verifier.setOtp(requestId1, hash1, expiry);
        
        vm.prank(issuer);
        verifier.setOtp(requestId2, hash2, expiry);

        // Both should be verifiable independently
        bool result1 = verifier.verify(requestId1, "123456");
        bool result2 = verifier.verify(requestId2, "123456");
        
        assertTrue(result1, "First request should verify");
        assertTrue(result2, "Second request should verify");
    }

    // Test zero hash prevention
    function testZeroHashPrevention() public {
        bytes32 requestId = keccak256("ZERO_HASH_TEST");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 zeroHash = bytes32(0);

        // Try to set OTP with zero hash
        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.InvalidHash.selector));
        verifier.setOtp(requestId, zeroHash, expiry);
    }

    // Helper function to hash OTP
    function _hashOtp(string memory otp) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("OTP:v1", otp));
    }
}
