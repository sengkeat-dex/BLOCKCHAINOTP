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
}
