# OTP Verifier Admin Runbooks

These runbooks wire incident playbooks into the code paths provided by the Axum backend (`/admin/*` routes) and the Foundry automation script (`contracts/script/AdminActions.s.sol`). Use them whenever you need to pause the verifier contract or rotate privileged keys.

## Prerequisites
- `ADMIN_PRIVATE_KEY` exported with the current admin EOA private key (hex, with or without `0x`).
- `config/deployments.toml` populated with the target network(s). Override with `DEPLOYMENTS_CONFIG=/path/to/file` if needed.
- Foundry installed (`forge` available in `$PATH` or at `~/.foundry/bin/forge`).
- Backend running (`cargo run`) if you plan to trigger actions through HTTP.

Verify automation wiring:
```bash
cargo run --bin blockchain-otp &           # launches the Axum API on :3001
curl -s http://127.0.0.1:3001/admin/pause \
  -H 'content-type: application/json' \
  -d '{"network":"ethereum_sepolia","paused":false}'
# → {"error":"admin automation not configured"} if ADMIN_PRIVATE_KEY is missing
```

Start the service with automation enabled by exporting the private key before `cargo run`. The server will announce how many networks were loaded once configuration succeeds.

## Pause / Resume (HTTP)
```bash
curl -sS http://127.0.0.1:3001/admin/pause \
  -H 'content-type: application/json' \
  -d '{"network":"ethereum_sepolia","paused":true}'
```
Response mirrors the on-chain receipt:
```json
{
  "action": "pause",
  "network_key": "ethereum_sepolia",
  "network_label": "Ethereum Sepolia",
  "tx_hash": "0x…",
  "status": "success",
  "block_number": 4761231
}
```
Toggle `paused` back to `false` to resume interactions.

## Rotate Issuer / Admin (HTTP)
```bash
curl -sS http://127.0.0.1:3001/admin/rotate-issuer \
  -H 'content-type: application/json' \
  -d '{"network":"ethereum_sepolia","new_issuer":"0xIssuer…"}'

curl -sS http://127.0.0.1:3001/admin/rotate-admin \
  -H 'content-type: application/json' \
  -d '{"network":"ethereum_sepolia","new_admin":"0xAdmin…"}'
```

Errors return JSON with `error` and appropriate HTTP status (`503` when automation is disabled, `400/404` for malformed input, `502` for upstream RPC failures).

## Pause / Rotate (Foundry scripts)
The shell helpers wrap `forge script` so playbooks can run from incident automation without copying arguments.

```bash
export ADMIN_PRIVATE_KEY=0xabc…

# Pause contract
./scripts/admin_pause.sh ethereum_sepolia true

# Rotate issuer
./scripts/admin_rotate_issuer.sh ethereum_sepolia 0xIssuer…

# Rotate admin
./scripts/admin_rotate_admin.sh ethereum_sepolia 0xAdmin…
```

Each helper resolves RPC URLs and contract addresses via `scripts/deployment_info.py`, calls [`AdminActions`](../../contracts/script/AdminActions.s.sol), and broadcasts the transaction with the supplied admin key.

## Post-Action Verification
1. Watch for the transaction hash in the HTTP/CLI output and confirm it finalised.
2. Subscribe to `Paused`, `IssuerChanged`, or `AdminChanged` events (e.g., `cast logs`) if you require immutable evidence.
3. Record the action, outcome, and hash in the incident ticket/runbook trail.
