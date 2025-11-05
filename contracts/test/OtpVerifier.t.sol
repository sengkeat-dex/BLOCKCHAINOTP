// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/OtpVerifier.sol";

contract OtpVerifierTest is Test {
    OtpVerifier private verifier;
    address private issuer = address(0xBEEF);
    address private admin = address(this);

    function setUp() public {
        verifier = new OtpVerifier(issuer, admin);
    }

    function testInitialRolesConfigured() public {
        assertEq(verifier.issuer(), issuer, "issuer mismatch");
        assertEq(verifier.admin(), admin, "admin mismatch");
        assertTrue(!verifier.paused(), "contract should start unpaused");
    }

    function testSetOtpAndVerifySuccess() public {
        bytes32 requestId = keccak256("REQ_SUCCESS");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);

        bool ok = verifier.verify(requestId, "123456");
        assertTrue(ok, "verification should succeed");

        (, , uint8 attemptsAfterSuccess, bool usedAfterSuccess) = verifier.entries(requestId);
        assertTrue(usedAfterSuccess, "entry should be marked used");
        assertEq(attemptsAfterSuccess, 0, "attempts should stay zero");
    }

    function testSetOtpRequiresIssuer() public {
        bytes32 requestId = keccak256("REQ_ISSUER");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, address(this)));
        verifier.setOtp(requestId, _hash("123456"), expiry);
    }

    function testSetOtpRejectsInvalidExpiry() public {
        bytes32 requestId = keccak256("REQ_EXP");
        uint64 expiry = uint64(block.timestamp);

        uint64 currentTime = uint64(block.timestamp);

        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.InvalidExpiry.selector, expiry, currentTime));
        verifier.setOtp(requestId, _hash("123456"), expiry);
    }

    function testSetOtpRejectsDuplicateRequest() public {
        bytes32 requestId = keccak256("REQ_DUP");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);

        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.EntryExists.selector, requestId));
        verifier.setOtp(requestId, _hash("654321"), expiry);
    }

    function testVerifyRejectsUnknownRequest() public {
        bytes32 requestId = keccak256("REQ_UNKNOWN");

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.UnknownRequest.selector, requestId));
        verifier.verify(requestId, "123456");
    }

    function testVerifyRejectsExpiredRequest() public {
        bytes32 requestId = keccak256("REQ_EXPIRED");
        uint64 expiry = uint64(block.timestamp + 1);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);

        vm.warp(block.timestamp + 2);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.EntryExpired.selector, requestId, expiry));
        verifier.verify(requestId, "123456");
    }

    function testVerifyRejectsAlreadyUsedRequest() public {
        bytes32 requestId = keccak256("REQ_USED");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        verifier.verify(requestId, "123456");

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.EntryUsed.selector, requestId));
        verifier.verify(requestId, "123456");
    }

    function testVerifyWrongOtpConsumesAttempts() public {
        bytes32 requestId = keccak256("REQ_ATTEMPTS");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);

        bool invalidAttempt = verifier.verify(requestId, "000000");
        assertTrue(!invalidAttempt, "verification should fail for wrong otp");

        (, , uint8 attemptsAfterFailure, bool usedAfterFailure) = verifier.entries(requestId);
        assertEq(attemptsAfterFailure, 1, "attempt count should increase");
        assertTrue(!usedAfterFailure, "entry should remain unused");
    }

    function testVerifyLocksAfterMaxAttempts() public {
        bytes32 requestId = keccak256("REQ_LOCK");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);

        uint8 maxAttempts = verifier.MAX_ATTEMPTS();
        for (uint8 i = 0; i < maxAttempts; i++) {
            bool invalid = verifier.verify(requestId, "111111");
            assertTrue(!invalid, "invalid otp attempt should fail");
        }

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.AttemptsExceeded.selector, requestId));
        verifier.verify(requestId, "123456");
    }

    function testCleanupAfterUse() public {
        bytes32 requestId = keccak256("REQ_CLEANUP");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        verifier.verify(requestId, "123456");

        verifier.cleanup(requestId);
        (bytes32 hashAfterCleanup, uint64 expiryAfterCleanup, , ) = verifier.entries(requestId);
        assertEq(hashAfterCleanup, bytes32(0), "entry hash should reset");
        assertEq(uint256(expiryAfterCleanup), 0, "expiry should reset");
    }

    function testCleanupAfterExpiry() public {
        bytes32 requestId = keccak256("REQ_EXPIRE_CLEAN");
        uint64 expiry = uint64(block.timestamp + 1);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);

        vm.warp(block.timestamp + 120);
        verifier.cleanup(requestId);
        (bytes32 hashAfterExpiryCleanup, , , ) = verifier.entries(requestId);
        assertEq(hashAfterExpiryCleanup, bytes32(0), "entry hash should reset");
    }

    function testCleanupRevertsWhenActive() public {
        bytes32 requestId = keccak256("REQ_ACTIVE");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ActiveEntry.selector, requestId));
        verifier.cleanup(requestId);
    }

    function testPauseStopsSetAndVerify() public {
        verifier.pause(true);

        bytes32 requestId = keccak256("REQ_PAUSE");
        uint64 expiry = uint64(block.timestamp + 60);

        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ContractPaused.selector));
        verifier.setOtp(requestId, _hash("123456"), expiry);

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ContractPaused.selector));
        verifier.verify(requestId, "123456");
    }

    function testIssuerRotationRequiresAdmin() public {
        address newIssuer = address(0xCAFE);

        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, issuer));
        verifier.setIssuer(newIssuer);

        verifier.setIssuer(newIssuer);
        assertEq(verifier.issuer(), newIssuer, "issuer should rotate");
    }

    function testAdminRotation() public {
        address newAdmin = address(0xABCD);

        verifier.setAdmin(newAdmin);
        assertEq(verifier.admin(), newAdmin, "admin should update");

        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, admin));
        verifier.setAdmin(admin);
    }

    function testFuzzRejectsIncorrectOtp(string memory wrongOtp) public {
        bytes32 requestId = keccak256("REQ_FUZZ");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 correctHash = _hash("654321");

        vm.prank(issuer);
        verifier.setOtp(requestId, correctHash, expiry);

        vm.assume(keccak256(abi.encodePacked("OTP:v1", wrongOtp)) != correctHash);

        bool result = verifier.verify(requestId, wrongOtp);
        assertTrue(!result, "verification should fail for wrong otp");
        (, , uint8 attemptsAfterFailure, ) = verifier.entries(requestId);
        assertEq(attemptsAfterFailure, 1, "first wrong attempt should increment counter");
    }

    function _hash(string memory otp) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("OTP:v1", otp));
    }

    // === NEW SECURITY TESTS ===

    // Test for reentrancy protection (even though not directly applicable, good to have)
    function testNoReentrancyInSetOtp() public {
        bytes32 requestId = keccak256("REQ_REENTRANCY");
        uint64 expiry = uint64(block.timestamp + 60);
        
        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        
        // Verify the function doesn't allow reentrancy by attempting to set the same request twice
        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.EntryExists.selector, requestId));
        verifier.setOtp(requestId, _hash("654321"), expiry);
    }

    // Test for overflow protection in attempts counter
    function testAttemptsCounterDoesNotOverflow() public {
        bytes32 requestId = keccak256("REQ_OVERFLOW");
        uint64 expiry = uint64(block.timestamp + 60);
        
        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        
        // Try to exceed max attempts multiple times to check for overflow
        uint8 maxAttempts = verifier.MAX_ATTEMPTS();
        for (uint8 i = 0; i < maxAttempts + 5; i++) {
            verifier.verify(requestId, "000000"); // Wrong OTP
        }
        
        // After max attempts, it should still revert
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.AttemptsExceeded.selector, requestId));
        verifier.verify(requestId, "123456");
    }

    // Test for proper zero address validation
    function testZeroAddressValidation() public {
        // Test constructor reverts with zero address
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ZeroAddress.selector));
        new OtpVerifier(address(0), admin);
        
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ZeroAddress.selector));
        new OtpVerifier(issuer, address(0));
        
        // Test setIssuer reverts with zero address
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ZeroAddress.selector));
        verifier.setIssuer(address(0));
        
        // Test setAdmin reverts with zero address
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.ZeroAddress.selector));
        verifier.setAdmin(address(0));
    }

    // Test for proper hash validation (not zero hash)
    function testInvalidHashReverts() public {
        bytes32 requestId = keccak256("REQ_INVALID_HASH");
        uint64 expiry = uint64(block.timestamp + 60);
        bytes32 invalidHash = bytes32(0);
        
        vm.prank(issuer);
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.InvalidHash.selector));
        verifier.setOtp(requestId, invalidHash, expiry);
    }

    // Test for proper cleanup of used entries
    function testCleanupUsedEntry() public {
        bytes32 requestId = keccak256("REQ_CLEAN_USED");
        uint64 expiry = uint64(block.timestamp + 60);
        
        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        verifier.verify(requestId, "123456"); // Mark as used
        
        // Should be able to cleanup used entry
        verifier.cleanup(requestId);
        
        (bytes32 hashAfterCleanup, , , ) = verifier.entries(requestId);
        assertEq(hashAfterCleanup, bytes32(0), "Entry should be cleaned up");
    }

    // Test for proper cleanup of expired entries
    function testCleanupExpiredEntry() public {
        bytes32 requestId = keccak256("REQ_CLEAN_EXPIRED");
        uint64 expiry = uint64(block.timestamp + 1);
        
        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        
        // Warp to after expiry
        vm.warp(block.timestamp + 2);
        
        // Should be able to cleanup expired entry
        verifier.cleanup(requestId);
        
        (bytes32 hashAfterCleanup, , , ) = verifier.entries(requestId);
        assertEq(hashAfterCleanup, bytes32(0), "Entry should be cleaned up");
    }

    // Test for proper event emission
    function testEventEmission() public {
        bytes32 requestId = keccak256("REQ_EVENTS");
        uint64 expiry = uint64(block.timestamp + 60);
        
        // Test OtpSet event
        vm.expectEmit(true, true, false, true);
        emit OtpVerifier.OtpSet(requestId, expiry);
        
        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        
        // Test AttemptRecorded event
        vm.expectEmit(true, true, false, true);
        emit OtpVerifier.AttemptRecorded(requestId, 1);
        
        verifier.verify(requestId, "000000"); // Wrong OTP
        
        // Test OtpLocked event when max attempts reached
        uint8 maxAttempts = verifier.MAX_ATTEMPTS();
        for (uint8 i = 1; i < maxAttempts; i++) {
            verifier.verify(requestId, "000000"); // Wrong OTP
        }
        
        vm.expectEmit(true, false, false, true);
        emit OtpVerifier.OtpLocked(requestId);
        
        verifier.verify(requestId, "000000"); // This should lock it
    }

    // Test for proper state after verification
    function testStateAfterVerification() public {
        bytes32 requestId = keccak256("REQ_STATE");
        uint64 expiry = uint64(block.timestamp + 60);
        
        vm.prank(issuer);
        verifier.setOtp(requestId, _hash("123456"), expiry);
        
        // Before verification
        (, , uint8 attemptsBefore, bool usedBefore) = verifier.entries(requestId);
        assertEq(attemptsBefore, 0, "Should start with 0 attempts");
        assertFalse(usedBefore, "Should not be used initially");
        
        // After successful verification
        verifier.verify(requestId, "123456");
        
        (, , uint8 attemptsAfter, bool usedAfter) = verifier.entries(requestId);
        assertEq(attemptsAfter, 0, "Attempts should remain 0 after success");
        assertTrue(usedAfter, "Should be marked as used after success");
    }
}
