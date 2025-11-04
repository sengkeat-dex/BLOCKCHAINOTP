// Ethereum deployment script for OtpVerifier contract
const { ethers } = require("hardhat");

async function main() {
    // Get the deployer account
    const [deployer] = await ethers.getSigners();
    
    console.log("Deploying contracts with the account:", deployer.address);
    console.log("Account balance:", (await deployer.getBalance()).toString());
    
    // Get the contract factory
    const OtpVerifier = await ethers.getContractFactory("OtpVerifier");
    
    // Deploy the contract with the deployer as the initial issuer
    console.log("Deploying OtpVerifier contract...");
    const otpVerifier = await OtpVerifier.deploy(deployer.address);
    
    // Wait for the deployment transaction to be mined
    await otpVerifier.deployed();
    
    console.log("OtpVerifier deployed to:", otpVerifier.address);
    
    // Verify the deployment by calling a view function
    const issuer = await otpVerifier.issuer();
    console.log("Contract issuer set to:", issuer);
    
    // Save the contract address to a file for later use
    const fs = require("fs");
    const deploymentInfo = {
        network: "ethereum",
        contractAddress: otpVerifier.address,
        issuer: deployer.address,
        deployTimestamp: new Date().toISOString()
    };
    
    fs.writeFileSync(
        "ethereum_deployment.json", 
        JSON.stringify(deploymentInfo, null, 2)
    );
    
    console.log("Deployment information saved to ethereum_deployment.json");
    console.log("Deployment completed successfully!");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });