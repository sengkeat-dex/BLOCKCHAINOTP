// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./DeployOtpVerifier.s.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";

/// @title Enhanced Deployment Script
/// @notice Main deployment script that uses the enhanced DeployOtpVerifier with security layers
contract Deploy is Script {
    // Deployment configuration
    address public constant DEFAULT_ISSUER = 0x1234567890123456789012345678901234567890;
    address public constant DEFAULT_ADMIN = 0x0987654321098765432109876543210987654321;
    
    error DeploymentAborted(string reason);
    
    function run() external {
        // Get private key from environment variable
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);
        
        // Get issuer and admin from environment variables, or use defaults
        address issuer = _getAddressFromEnv("ISSUER", DEFAULT_ISSUER);
        address admin = _getAddressFromEnv("ADMIN", DEFAULT_ADMIN);
        
        console.log("Deploying OtpVerifier contract...");
        console.log("Issuer:", issuer);
        console.log("Admin:", admin);
        
        // Validate addresses before deployment
        if (!_isValidAddress(issuer)) {
            revert DeploymentAborted("Invalid issuer address");
        }
        
        if (!_isValidAddress(admin)) {
            revert DeploymentAborted("Invalid admin address");
        }
        
        if (issuer == admin) {
            revert DeploymentAborted("Issuer and admin must be different addresses");
        }
        
        // Deploy using our enhanced deployment contract
        DeployOtpVerifier deployer = new DeployOtpVerifier();
        OtpVerifier verifier = deployer.deploy(issuer, admin);
        
        console.log("Contract deployed at:", address(verifier));
        
        // Run basic tests
        deployer.runBasicTest(verifier);
        
        // Log deployment information
        console.log("Deployment completed successfully!");
        console.log("Contract address:", address(verifier));
        console.log("Issuer:", verifier.issuer());
        console.log("Admin:", verifier.admin());
        console.log("MAX_ATTEMPTS:", verifier.MAX_ATTEMPTS());
        
        vm.stopBroadcast();
    }
    
    /// @notice Gets an address from an environment variable or returns a default
    /// @param envVar The environment variable name
    /// @param defaultValue The default value to use if env var is not set
    /// @return The address from env var or default
    function _getAddressFromEnv(string memory envVar, address defaultValue) internal returns (address) {
        try vm.envAddress(envVar) returns (address addr) {
            if (addr != address(0)) {
                return addr;
            }
        } catch {
            // If env var is not set or is zero address, use default
        }
        return defaultValue;
    }
    
    /// @notice Validates that an address is not the zero address
    /// @param addr The address to validate
    /// @return True if the address is valid, false otherwise
    function _isValidAddress(address addr) internal pure returns (bool) {
        return addr != address(0);
    }
}