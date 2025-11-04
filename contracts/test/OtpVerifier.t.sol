// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./utils/TestBase.sol";
import "../src/OtpVerifier.sol";

contract NonIssuerCaller {
    function trySet(OtpVerifier verifier, bytes32 requestId, bytes32 otpHash, uint64 expiry)
        external
        returns (bool success)
    {
        try verifier.setOtp(requestId, otpHash, expiry) {
            success = true;
        } catch {
            success = false;
        }
    }
}

contract OtpVerifierTest is TestBase {
    OtpVerifier private verifier;
    NonIssuerCaller private attacker;

    function setUp() public {
        verifier = new OtpVerifier(address(this));
        attacker = new NonIssuerCaller();
    }

    function testInitialIssuer() public {
        assertEq(verifier.issuer(), address(this), "issuer should default to deployer");
    }

    function testSetOtpAndVerify() public {
        bytes32 requestId = keccak256("REQ1");
        bytes32 otpHash = keccak256(abi.encodePacked("123456"));
        uint64 expiry = uint64(block.timestamp + 60);

        verifier.setOtp(requestId, otpHash, expiry);
        bool ok = verifier.verify(requestId, "123456");
        assertTrue(ok, "otp should verify");
        assertEq(verifier.entries(requestId).used, true, "entry should be marked used");
    }

    function testDuplicateRequestFails() public {
        bytes32 requestId = keccak256("REQ2");
        bytes32 otpHash = keccak256(abi.encodePacked("654321"));
        uint64 expiry = uint64(block.timestamp + 60);

        verifier.setOtp(requestId, otpHash, expiry);
        bool reverted;
        try verifier.setOtp(requestId, otpHash, expiry) {
            reverted = false;
        } catch {
            reverted = true;
        }
        assertEq(reverted, true, "duplicate entries must fail");
    }

    function testVerifyRejectsWrongOtp() public {
        bytes32 requestId = keccak256("REQ3");
        bytes32 otpHash = keccak256(abi.encodePacked("000111"));
        uint64 expiry = uint64(block.timestamp + 60);
        verifier.setOtp(requestId, otpHash, expiry);

        bool reverted;
        try verifier.verify(requestId, "111000") {
            reverted = false;
        } catch {
            reverted = true;
        }
        assertEq(reverted, true, "wrong otp should revert");
    }

    function testPauseBlocksIssuance() public {
        verifier.pause(true);
        bytes32 requestId = keccak256("REQ4");
        bytes32 otpHash = keccak256(abi.encodePacked("222333"));
        uint64 expiry = uint64(block.timestamp + 60);
        bool reverted;
        try verifier.setOtp(requestId, otpHash, expiry) {
            reverted = false;
        } catch {
            reverted = true;
        }
        assertTrue(reverted, "paused contract must reject setOtp");
    }

    function testOnlyIssuerCanSetOtp() public {
        bytes32 requestId = keccak256("REQ5");
        bytes32 otpHash = keccak256(abi.encodePacked("333444"));
        uint64 expiry = uint64(block.timestamp + 60);
        bool success = attacker.trySet(verifier, requestId, otpHash, expiry);
        assertEq(success, false, "non issuer should not set otp");
    }

    function testIssuerRotation() public {
        address newIssuer = address(0xBEEF);
        verifier.setIssuer(newIssuer);
        assertEq(verifier.issuer(), newIssuer, "issuer should rotate");
    }
}
