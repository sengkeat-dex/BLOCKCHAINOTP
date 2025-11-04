# Smart Contracts

This folder contains the on-chain anchor used by the OTP platform. It is a Foundry project with the
following useful commands:

```bash
cd contracts
forge fmt
forge build
forge test
forge script script/DeployOtpVerifier.s.sol:DeployOtpVerifier \
  --sig "deploy(address)" <issuer_address> --rpc-url $RPC_URL --private-key $PK --broadcast
```

`foundry.toml` pins Solidity to 0.8.20 for parity with the rest of the workspace.
