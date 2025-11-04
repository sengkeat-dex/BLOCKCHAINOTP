# Deployment Guide: EVM + Solana OTP Anchors

This repository ships the on-chain anchor contract (`contracts/src/OtpVerifier.sol`) and the Rust
clients that speak to it. The steps below walk through deploying to the major EVM testnets, updating
config files, and wiring up the Solana parity program.

## Prerequisites
- Metamask (already installed) configured for your target network (e.g., Sepolia, Polygon Amoy,
  Arbitrum Sepolia, or mainnet). Fund the account with test ETH/MATIC.
- Foundry toolchain (`forge`, `cast`) installed locally.
- Infura/Alchemy (or other RPC) keys for each EVM network you plan to deploy to.
- Git + access to this repo so you can commit deployment artifacts.
- For Solana: `solana-install init` plus the Anchor CLI (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli`).

## 1. Build the Solidity contracts
```bash
cd contracts
forge build
```
This compiles `OtpVerifier.sol` and emits ABI/bytecode under `contracts/out/OtpVerifier.sol/OtpVerifier.json`.
Commit the JSON file after deployment so clients can import the ABI.

## 2. Deploy to an EVM testnet (Sepolia example)
```bash
cd contracts
forge script script/DeployOtpVerifier.s.sol:DeployOtpVerifier \
  --sig "deploy(address)" <ISSUER_ADDRESS> \
  --rpc-url https://sepolia.infura.io/v3/<INFURA_KEY> \
  --private-key <DEPLOYER_PRIVATE_KEY> \
  --broadcast
```
- `<ISSUER_ADDRESS>` is the wallet allowed to call `setOtp` and administration functions.
- `<DEPLOYER_PRIVATE_KEY>` controls the funding account (Metamask export).
- Repeat for Polygon Amoy, Arbitrum Sepolia, mainnet, etc. by swapping `--rpc-url` and the funding key.

## 3. Record deployment metadata
After each broadcast:
1. Copy the deployed `OtpVerifier` address and tx hash.
2. Update `config/deployments.toml` under the appropriate section:
   ```toml
   [ethereum_sepolia]
   network = "Ethereum Sepolia"
   rpc_url = "https://sepolia.infura.io/v3/<INFURA_KEY>"
   otp_verifier = "0x..."     # deployed address
   tx_hash = "0x..."          # optional but helpful
   ```
3. Commit the updated `config/deployments.toml` and the ABI JSON.

## 4. Frontend/backend integration
- The Rust client in `crates/otp-contract` automatically consumes the ABI and addresses via config.
- Add any new network entries into your runtime configuration so the backend selects the right RPC
  endpoint and contract address per tenant.

## 5. Solana parity program
1. Install the Solana toolchain and Anchor:
   ```bash
   solana-install init
   cargo install --git https://github.com/coral-xyz/anchor anchor-cli
   ```
2. Scaffold an Anchor program mirroring the Solidity logic (PDA seed `b"otp" || request_id`, fields
   `{hash, expiry, used}`). Suggested layout:
   ```text
   programs/otp_verifier/src/lib.rs
   Anchor.toml
   ```
3. Build and deploy to Devnet:
   ```bash
   anchor build
   anchor deploy
   ```
   Anchor prints the `program id` and tx hash.
4. Update `config/deployments.toml` `solana_devnet` section with:
   ```toml
   program_id = "<ProgramID>"
   payer_pubkey = "<DeployerPubkey>"
   last_deploy_tx = "<Signature>"
   ```
5. Export the payer secret (Base58) so the backend can instantiate `SolanaOtpClient::try_new`.
   Store secrets in your vault; do **not** commit them.

## 6. Test the Solana client locally
The Solana support is behind a feature flag to avoid dependency conflicts with `ethers`. Run tests in
a Solana-enabled checkout:
```bash
cargo test -p otp-contract --no-default-features --features solana
```
Prerequisites: Solana libraries, running `solana-test-validator` if you want to mock on-chain state.

## 7. Optional: Automated scripts
- Add a `deploy.<network>.sh` script that wraps the `forge script` commands and writes a JSON artifact
  containing address + tx hash. Keep these scripts in `contracts/scripts/`.
- Add CI jobs that run `forge test` and (optionally) `cargo test -p otp-contract --features solana`
  to gate merges.

## 8. Checklist per deployment
- [ ] Run `forge build` and `forge test`.
- [ ] Deploy via `forge script` (capture tx hash).
- [ ] Update `config/deployments.toml` + ABI JSON.
- [ ] Tag/commit with a clear message (`Deploy OtpVerifier to Sepolia`).
- [ ] If Solana: `anchor build`, `anchor deploy`, update config, run solana-feature tests.
- [ ] Notify downstream teams of the new addresses.

Following these steps keeps the repo synchronized with on-chain state and gives client teams the ABI
+ addresses they need to integrate. Reach out if you hit RPC/Anchor errors and we can troubleshoot.
