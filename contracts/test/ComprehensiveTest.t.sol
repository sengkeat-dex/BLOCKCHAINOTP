// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/OtpVerifier.sol";
import "../script/DeployOtpVerifier.s.sol";

contract ComprehensiveTest is Test {
    DeployOtpVerifier private deployer;
    OtpVerifier private verifier;
    address private issuer = address(0xBEEF);
    address private admin = address(this);
    address private user = address(0x1234);

    function setUp() public {
        deployer = new DeployOtpVerifier();
        verifier = deployer.deploy(issuer, admin);
    }

    // Test complete deployment flow
    function testCompleteDeploymentFlow() public {
        // Test that deployment was successful
        assertTrue(address(verifier) != address(0), "Contract should be deployed");
        assertEq(verifier.issuer(), issuer, "Issuer should be correct");
        assertEq(verifier.admin(), admin, "Admin should be correct");
        assertTrue(!verifier.paused(), "Contract should not be paused");
        assertEq(verifier.MAX_ATTEMPTS(), 3, "MAX_ATTEMPTS should be 3");
    }

    // Test full OTP lifecycle
    function testFullOtpLifecycle() public {
        bytes32 requestId = keccak256("FULL_LIFECYCLE_TEST");
        string memory otp = "654321";
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 hash = _hashOtp(otp);

        // 1. Issuer sets OTP
        vm.prank(issuer);
        verifier.setOtp(requestId, hash, expiry);

        // Verify entry was created correctly
        (bytes32 storedHash, uint64 storedExpiry, uint8 attempts, bool used) = verifier.entries(requestId);
        assertEq(storedHash, hash, "Hash should match");
        assertEq(storedExpiry, expiry, "Expiry should match");
        assertEq(attempts, 0, "Attempts should be 0");
        assertTrue(!used, "Entry should not be used");

        // 2. User verifies OTP
        bool result = verifier.verify(requestId, otp);
        assertTrue(result, "Verification should succeed");

        // Verify entry state after verification
        (storedHash, storedExpiry, attempts, used) = verifier.entries(requestId);
        assertEq(storedHash, hash, "Hash should still match");
        assertEq(storedExpiry, expiry, "Expiry should still match");
        assertEq(attempts, 0, "Attempts should still be 0");
        assertTrue(used, "Entry should be marked as used");

        // 3. Cleanup after use
        verifier.cleanup(requestId);

        // Verify entry was cleaned up
        (storedHash, storedExpiry, attempts, used) = verifier.entries(requestId);
        assertEq(storedHash, bytes32(0), "Hash should be zero after cleanup");
        assertEq(storedExpiry, 0, "Expiry should be zero after cleanup");
        assertEq(attempts, 0, "Attempts should be zero after cleanup");
        assertTrue(!used, "Used should be false after cleanup");
    }

    // Test role rotation
    function testRoleRotation() public {
        address newIssuer = address(0x1111);
        address newAdmin = address(0x2222);

        // Admin changes issuer
        verifier.setIssuer(newIssuer);
        assertEq(verifier.issuer(), newIssuer, "Issuer should be updated");

        // Admin changes admin
        verifier.setAdmin(newAdmin);
        assertEq(verifier.admin(), newAdmin, "Admin should be updated");

        // Old admin should no longer have privileges
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, admin));
        verifier.setIssuer(issuer);

        // New admin should have privileges
        vm.prank(newAdmin);
        verifier.setIssuer(issuer);
        assertEq(verifier.issuer(), issuer, "Issuer should be updated by new admin");
    }

    // Test emergency procedures
    function testEmergencyProcedures() public {
        // Admin pauses the contract
        verifier.pause(true);
        assertTrue(verifier.paused(), "Contract should be paused");

        // All operations should fail when paused
        bytes32 requestId = keccak256("PAUSED_TEST");
        bytes32 hash = _hashOtp("123456");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ContractPaused.selector));
        verifier.setOtp(requestId, hash, expiry);

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ContractPaused.selector));
        verifier.verify(requestId, "123456");

        // Admin unpauses the contract
        verifier.pause(false);
        assertTrue(!verifier.paused(), "Contract should be unpaused");

        // Operations should work again
        vm.prank(issuer);
        verifier.setOtp(requestId, hash, expiry);
        
        bool result = verifier.verify(requestId, "123456");
        assertTrue(result, "Verification should work after unpausing");
    }

    // Test edge cases and boundary conditions
    function testEdgeCases() public {
        // Test with minimal expiry (1 second)
        bytes32 requestId1 = keccak256("MINIMAL_EXPIRY");
        bytes32 hash1 = _hashOtp("123456");
        uint64 minimalExpiry = uint64(block.timestamp + 1);

        vm.prank(issuer);
        verifier.setOtp(requestId1, hash1, minimalExpiry);

        // Should be verifiable immediately
        bool result = verifier.verify(requestId1, "123456");
        assertTrue(result, "Should verify with minimal expiry");

        // Test with maximum reasonable expiry
        bytes32 requestId2 = keccak256("MAX_EXPIRY");
        bytes32 hash2 = _hashOtp("654321");
        uint64 maxExpiry = uint64(block.timestamp + 365 days);

        vm.prank(issuer);
        verifier.setOtp(requestId2, hash2, maxExpiry);

        // Should be verifiable
        bool result2 = verifier.verify(requestId2, "654321");
        assertTrue(result2, "Should verify with long expiry");
    }

    // Test concurrent requests
    function testConcurrentRequests() public {
        // Create multiple OTP requests
        for (uint i = 0; i < 10; i++) {
            bytes32 requestId = keccak256(abi.encodePacked("REQUEST_", i));
            string memory otp = string(abi.encodePacked("00000", uint2str(i % 10)));
            bytes32 hash = _hashOtp(otp);
            uint64 expiry = uint64(block.timestamp + 60 + i);

            vm.prank(issuer);
            verifier.setOtp(requestId, hash, expiry);

            // Verify each OTP
            bool result = verifier.verify(requestId, otp);
            assertTrue(result, "All OTPs should verify successfully");
        }
    }

    // Test cleanup of expired entries
    function testExpiredEntryCleanup() public {
        bytes32 requestId = keccak256("EXPIRED_CLEANUP");
        bytes32 hash = _hashOtp("123456");
        uint64 expiry = uint64(block.timestamp + 10);

        vm.prank(issuer);
        verifier.setOtp(requestId, hash, expiry);

        // Advance time past expiry
        vm.warp(block.timestamp + 15);

        // Admin should be able to cleanup expired entry
        verifier.cleanup(requestId);

        // Verify entry was cleaned up
        (bytes32 storedHash, uint64 storedExpiry, uint8 attempts, bool used) = verifier.entries(requestId);
        assertEq(storedHash, bytes32(0), "Hash should be zero after cleanup");
        assertEq(storedExpiry, 0, "Expiry should be zero after cleanup");
        assertEq(attempts, 0, "Attempts should be zero after cleanup");
        assertTrue(!used, "Used should be false after cleanup");
    }

    // Test that the deployment script's basic test works
    function testDeploymentScriptBasicTest() public {
        deployer.runBasicTest(verifier);
        // If no revert occurs, the test passed
    }

    // Helper function to hash OTP
    function _hashOtp(string memory otp) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("OTP:v1", otp));
    }

    // Helper function to convert uint to string
    function uint2str(uint _i) internal pure returns (string memory _uintAsString) {
        if (_i == 0) {
            return "0";
        }
        uint j = _i;
        uint len;
        while (j != 0) {
            len++;
            j /= 10;
        }
        bytes memory bstr = new bytes(len);
        uint k = len;
        while (_i != 0) {
            k = k-1;
            uint8 temp = (48 + uint8(_i - _i / 10 * 10));
            bytes1 b1 = bytes1(temp);
            bstr[k] = b1;
            _i /= 10;
        }
        return string(bstr);
    }
}
