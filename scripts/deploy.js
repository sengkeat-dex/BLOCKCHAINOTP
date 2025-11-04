// deploy.js
const { ethers } = require("hardhat");

async function main() {
    const [deployer] = await ethers.getSigners();
    
    console.log("Deploying contracts with the account:", deployer.address);
    console.log("Account balance:", (await deployer.getBalance()).toString());
    
    // Get the contract factory
    const OtpVerifier = await ethers.getContractFactory("OtpVerifier");
    
    // Deploy the contract with the deployer as the initial issuer
    const otpVerifier = await OtpVerifier.deploy(deployer.address);
    
    console.log("OtpVerifier deployed to:", otpVerifier.address);
    
    // Wait for the deployment transaction to be mined
    await otpVerifier.deployed();
    
    console.log("Deployment completed!");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });