// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../src/OtpVerifier.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";

/// @title Enhanced DeployOtpVerifier
/// @notice Secure deployment wrapper with validation, logging, and testing capabilities
contract DeployOtpVerifier is Script {
    // Events for deployment tracking
    event DeploymentStarted(address indexed deployer, uint256 indexed timestamp);
    event DeploymentCompleted(address indexed contractAddress, address indexed issuer, address indexed admin);
    event SecurityCheckPassed(string checkName);
    event SecurityCheckFailed(string checkName, string reason);
    
    // Errors for deployment issues
    error InvalidIssuerAddress(address issuer);
    error InvalidAdminAddress(address admin);
    error DeploymentFailed(string reason);
    error SecurityCheckFailedError(string checkName, string reason);
    
    // Constants for security parameters
    uint256 public constant DEPLOYMENT_TIMEOUT = 1 hours;
    uint8 public constant MAX_ATTEMPTS_LIMIT = 3;
    
    // Storage for deployment parameters
    address public deployer;
    uint256 public deploymentStartTime;
    
    /// @notice Deploys the OtpVerifier contract with enhanced security checks
    /// @param issuer The address authorized to issue OTPs
    /// @param admin The address authorized to administer the contract
    /// @return The deployed OtpVerifier contract instance
    function deploy(address issuer, address admin) external returns (OtpVerifier) {
        // Record deployment start
        deployer = msg.sender;
        deploymentStartTime = block.timestamp;
        
        emit DeploymentStarted(deployer, deploymentStartTime);
        
        // Validate input parameters
        _validateAddresses(issuer, admin);
        
        // Perform security checks
        _performSecurityChecks(issuer, admin);
        
        // Deploy the contract
        OtpVerifier verifier = _deployContract(issuer, admin);
        
        // Verify deployment
        _verifyDeployment(verifier, issuer, admin);
        
        emit DeploymentCompleted(address(verifier), issuer, admin);
        
        return verifier;
    }
    
    /// @notice Validates that addresses are not zero addresses
    /// @param issuer The issuer address to validate
    /// @param admin The admin address to validate
    function _validateAddresses(address issuer, address admin) internal pure {
        if (issuer == address(0)) {
            revert InvalidIssuerAddress(issuer);
        }
        
        if (admin == address(0)) {
            revert InvalidAdminAddress(admin);
        }
        
        console.log("Address validation passed");
    }
    
    /// @notice Performs comprehensive security checks before deployment
    /// @param issuer The issuer address
    /// @param admin The admin address
    function _performSecurityChecks(address issuer, address admin) internal {
        // Check 1: Ensure issuer and admin are different addresses
        if (issuer == admin) {
            emit SecurityCheckFailed("DifferentAddresses", "Issuer and admin should be different addresses");
            revert SecurityCheckFailedError("DifferentAddresses", "Issuer and admin should be different addresses");
        }
        emit SecurityCheckPassed("DifferentAddresses");
        
        // Check 2: Ensure deployer is not the same as issuer or admin (optional security)
        if (deployer == issuer || deployer == admin) {
            console.log("Warning: Deployer is the same as issuer or admin");
        } else {
            emit SecurityCheckPassed("DeployerSeparation");
        }
        
        // Check 3: Verify deployment is not expired
        if (block.timestamp > deploymentStartTime + DEPLOYMENT_TIMEOUT) {
            emit SecurityCheckFailed("DeploymentTimeout", "Deployment window has expired");
            revert SecurityCheckFailedError("DeploymentTimeout", "Deployment window has expired");
        }
        emit SecurityCheckPassed("DeploymentTimeout");
        
        console.log("All security checks passed");
    }
    
    /// @notice Deploys the OtpVerifier contract
    /// @param issuer The issuer address
    /// @param admin The admin address
    /// @return The deployed contract instance
    function _deployContract(address issuer, address admin) internal returns (OtpVerifier) {
        try new OtpVerifier(issuer, admin) returns (OtpVerifier verifier) {
            console.log("Contract deployed at:", address(verifier));
            return verifier;
        } catch Error(string memory reason) {
            emit SecurityCheckFailed("ContractDeployment", reason);
            revert DeploymentFailed(reason);
        } catch (bytes memory /*lowLevelData*/) {
            emit SecurityCheckFailed("ContractDeployment", "Low-level deployment error");
            revert DeploymentFailed("Low-level deployment error");
        }
    }
    
    /// @notice Verifies the deployment was successful
    /// @param verifier The deployed contract
    /// @param issuer The expected issuer address
    /// @param admin The expected admin address
    function _verifyDeployment(OtpVerifier verifier, address issuer, address admin) internal view {
        // Verify issuer
        address actualIssuer = verifier.issuer();
        if (actualIssuer != issuer) {
            revert DeploymentFailed("Issuer verification failed");
        }
        
        // Verify admin
        address actualAdmin = verifier.admin();
        if (actualAdmin != admin) {
            revert DeploymentFailed("Admin verification failed");
        }
        
        // Verify MAX_ATTEMPTS
        uint8 maxAttempts = verifier.MAX_ATTEMPTS();
        if (maxAttempts != MAX_ATTEMPTS_LIMIT) {
            revert DeploymentFailed("MAX_ATTEMPTS verification failed");
        }
        
        // Verify contract is not paused
        bool isPaused = verifier.paused();
        if (isPaused) {
            revert DeploymentFailed("Contract should not be paused after deployment");
        }
        
        console.log("Deployment verification successful");
    }
    
    /// @notice Runs a basic test on the deployed contract
    /// @param verifier The deployed contract
    function runBasicTest(OtpVerifier verifier) external {
        address issuer = verifier.issuer();
        address admin = verifier.admin();
        
        // Test that issuer can set OTP
        vm.prank(issuer);
        bytes32 testRequestId = keccak256("test-request");
        bytes32 testHash = keccak256(abi.encodePacked("OTP:v1", "123456"));
        uint64 testExpiry = uint64(block.timestamp + 60);
        
        try verifier.setOtp(testRequestId, testHash, testExpiry) {
            console.log("Basic test passed: Issuer can set OTP");
        } catch {
            revert DeploymentFailed("Basic test failed: Issuer cannot set OTP");
        }
        
        // Test that non-issuer cannot set OTP
        vm.prank(address(0x123));
        bytes32 testRequestId2 = keccak256("test-request-2");
        bytes32 testHash2 = keccak256(abi.encodePacked("OTP:v1", "654321"));
        uint64 testExpiry2 = uint64(block.timestamp + 120);
        
        try verifier.setOtp(testRequestId2, testHash2, testExpiry2) {
            revert DeploymentFailed("Security test failed: Non-issuer can set OTP");
        } catch {
            console.log("Security test passed: Non-issuer cannot set OTP");
        }
        
        console.log("All basic tests passed");
    }
}