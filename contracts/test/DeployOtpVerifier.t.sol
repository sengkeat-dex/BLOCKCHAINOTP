// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./utils/TestBase.sol";
import "../script/DeployOtpVerifier.s.sol";

contract DeployOtpVerifierTest is TestBase {
    DeployOtpVerifier private deployer;
    address private issuer = address(0xBEEF);
    address private admin = address(0xCAFE);
    address private deployerAddress = address(this);

    function setUp() public {
        deployer = new DeployOtpVerifier();
    }

    function testDeploymentSuccess() public {
        OtpVerifier verifier = deployer.deploy(issuer, admin);
        
        // Verify the contract was deployed
        assertTrue(address(verifier) != address(0), "Contract should be deployed");
        
        // Verify issuer and admin are set correctly
        assertEq(verifier.issuer(), issuer, "Issuer should be set correctly");
        assertEq(verifier.admin(), admin, "Admin should be set correctly");
        
        // Verify MAX_ATTEMPTS is set correctly
        assertEq(verifier.MAX_ATTEMPTS(), 3, "MAX_ATTEMPTS should be 3");
        
        // Verify contract is not paused
        assertTrue(!verifier.paused(), "Contract should not be paused after deployment");
    }

    function testDeploymentFailsWithZeroIssuer() public {
        vm.expectRevert(abi.encodeWithSelector(DeployOtpVerifier.InvalidIssuerAddress.selector, address(0)));
        deployer.deploy(address(0), admin);
    }

    function testDeploymentFailsWithZeroAdmin() public {
        vm.expectRevert(abi.encodeWithSelector(DeployOtpVerifier.InvalidAdminAddress.selector, address(0)));
        deployer.deploy(issuer, address(0));
    }

    function testDeploymentFailsWithSameIssuerAndAdmin() public {
        address sameAddress = address(0x1234);
        
        vm.expectRevert(
            abi.encodeWithSelector(
                DeployOtpVerifier.SecurityCheckFailedError.selector, 
                "DifferentAddresses", 
                "Issuer and admin should be different addresses"
            )
        );
        deployer.deploy(sameAddress, sameAddress);
    }

    function testDeploymentWithinTimeout() public {
        // This should pass as it's within the timeout
        OtpVerifier verifier = deployer.deploy(issuer, admin);
        assertTrue(address(verifier) != address(0), "Deployment should succeed within timeout");
    }

    function testBasicFunctionalityAfterDeployment() public {
        OtpVerifier verifier = deployer.deploy(issuer, admin);
        
        // Test that issuer can set OTP
        bytes32 requestId = keccak256("test-request");
        bytes32 hash = keccak256(abi.encodePacked("OTP:v1", "123456"));
        uint64 expiry = uint64(block.timestamp + 60);
        
        vm.prank(issuer);
        verifier.setOtp(requestId, hash, expiry);
        
        // Verify the entry was created
        (bytes32 storedHash, uint64 storedExpiry, uint8 attempts, bool used) = verifier.entries(requestId);
        assertEq(storedHash, hash, "Hash should match");
        assertEq(storedExpiry, expiry, "Expiry should match");
        assertEq(attempts, 0, "Attempts should be 0");
        assertTrue(!used, "Entry should not be used");
    }

    function testSecurityAfterDeployment() public {
        OtpVerifier verifier = deployer.deploy(issuer, admin);
        
        // Test that non-issuer cannot set OTP
        bytes32 requestId = keccak256("test-request-2");
        bytes32 hash = keccak256(abi.encodePacked("OTP:v1", "654321"));
        uint64 expiry = uint64(block.timestamp + 120);
        
        vm.prank(address(0x123));
        vm.expectRevert(abi.encodeWithSelector(OtpVerifier.Unauthorized.selector, address(0x123)));
        verifier.setOtp(requestId, hash, expiry);
    }

    function testRunBasicTest() public {
        OtpVerifier verifier = deployer.deploy(issuer, admin);
        deployer.runBasicTest(verifier);
        // If no revert occurs, the test passed
    }

    function testEventsAreEmitted() public {
        // Capture events
        vm.recordLogs();
        
        deployer.deploy(issuer, admin);
        
        Vm.Log[] memory entries = vm.getRecordedLogs();
        
        // Check that DeploymentStarted event was emitted
        bool foundDeploymentStarted = false;
        bool foundDeploymentCompleted = false;
        
        for (uint i = 0; i < entries.length; i++) {
            if (entries[i].topics[0] == keccak256("DeploymentStarted(address,uint256)")) {
                foundDeploymentStarted = true;
            }
            if (entries[i].topics[0] == keccak256("DeploymentCompleted(address,address,address)")) {
                foundDeploymentCompleted = true;
            }
        }
        
        assertTrue(foundDeploymentStarted, "DeploymentStarted event should be emitted");
        assertTrue(foundDeploymentCompleted, "DeploymentCompleted event should be emitted");
    }

    function testSecurityCheckEvents() public {
        vm.recordLogs();
        
        deployer.deploy(issuer, admin);
        
        Vm.Log[] memory entries = vm.getRecordedLogs();
        
        // Count SecurityCheckPassed events
        uint count = 0;
        for (uint i = 0; i < entries.length; i++) {
            if (entries[i].topics[0] == keccak256("SecurityCheckPassed(string)")) {
                count++;
            }
        }
        
        // Should have at least 3 security checks passed:
        // 1. DifferentAddresses
        // 2. DeployerSeparation (if deployer is different)
        // 3. DeploymentTimeout
        assertTrue(count >= 3, "At least 3 security checks should pass");
    }
}