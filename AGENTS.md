# Repository Guidelines

## Project Structure & Module Organization
Specs sit at the root (`layout.md`, `rule.md`, `security-protection-testing-layer.md`) and every code folder should mirror them: the Foundry project lives in `contracts/`, Rust + Axum code in the workspace crates, and wallet or dashboard work in `frontend/`. Keep tests beside their modules (`contracts/test`, `tests/otp_flow.rs`, `frontend/src/**`) so the Register → Get OTP → Verify path stays traceable. Non-code artifacts belong in `docs/assets/`.

## Build, Test, and Development Commands
`forge build` (run inside `contracts/`) is mandatory for Solidity 0.8.20+ and should pass before every commit; `forge test` exercises the bundled `OtpVerifier` spec. Rust workflows rely on Cargo: `cargo test` for backend + integration coverage and `cargo run` to expose the local issuance API against a devnet RPC. Frontend artifacts are wired into the pipeline via the root `Makefile`: run `make frontend-build` (prefers `wasm-pack`, falls back to `cargo build -p otp-frontend --target wasm32-unknown-unknown`) and `make frontend-serve` to launch `trunk` when available. Replay full flows via the curl scripts in `docs/examples/` or `just e2e` when available.

## Coding Style & Naming Conventions
Use 4-space indentation in Solidity, explicit visibility, and NatSpec on public/external functions. Contracts stay PascalCase (`OtpVerifier`), storage vars camelCase, constants ALL_CAPS. Rust code must pass `cargo fmt` plus `clippy --all-targets --all-features`, with snake_case items and UpperCamelCase types. Keep hashing paired (`keccak256(abi.encodePacked(otp))` ↔ `Keccak256`) to avoid drift.

## Testing Guidelines
Unit tests must cover every state outcome (valid, expired, used, wrong, unknown requestId) plus pause-mode. Integration suites replay Register → Get OTP → Verify while asserting throttling (≤3 attempts) and issuer rotation. Add adversarial runs for replay, rate-limit evasion, and WAF bypass. Publish coverage artifacts in `target/coverage/` and gate CI on `forge coverage` plus `cargo tarpaulin`.

## Commit & Pull Request Guidelines
History shows imperative subjects (`Implement StableSwap AMM`, `Add rules and commit summary`); keep that tone, stay under 72 characters, and include a short body listing rationale and executed tests. Every PR needs a problem statement, linked issue, command log (e.g., `forge test`, `cargo test`), and rollback plan. Provide screenshots/log snippets for UX or observability changes and request security review whenever hashing, access control, or rate limits move.

## Security & Configuration Tips
Never persist plaintext OTPs; store only hashes + metadata on-chain and encrypted seeds off-chain. Protect issuer keys with a multisig or managed relayer and rehearse `setIssuer` rotation before release tags. Enforce TLS 1.2+, signed OTP requests, WAF-backed rate limits, and the attempt ceilings from `rule.md`. Track all runtime defaults (OTP TTL, RPC URLs, throttle windows) inside `.env.example` and call out any change inside the PR description.

## Chain Deployment Matrix
`config/deployments.toml` tracks the currently supported testnets (Ethereum Sepolia, Polygon Amoy, Arbitrum Sepolia, and Solana Devnet) along with non-zero sample contract/program IDs. Update it whenever you broadcast a new `OtpVerifier` or Solana program so clients can pin deterministic addresses. The `crates/otp-contract` crate exposes an `OtpChain` trait with an EVM implementation (default feature) and a Solana integration that can be enabled via `--no-default-features --features solana`—note that upstream `ethers` and Solana SDKs currently require separate builds because of `zeroize` version constraints. Agents adding more networks should implement that trait and extend the config + docs accordingly.
