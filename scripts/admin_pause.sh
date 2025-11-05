#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 <network-key> <true|false>" >&2
  exit 64
fi

NETWORK="$1"
PAUSED="$2"
CONFIG_PATH="${DEPLOYMENTS_CONFIG:-config/deployments.toml}"

if [[ -z "${ADMIN_PRIVATE_KEY:-}" ]]; then
  echo "ADMIN_PRIVATE_KEY environment variable must be set." >&2
  exit 65
fi

FORGE_BIN="${FORGE_BIN:-forge}"
if ! command -v "$FORGE_BIN" >/dev/null 2>&1; then
  if [[ -x "$HOME/.foundry/bin/forge" ]]; then
    FORGE_BIN="$HOME/.foundry/bin/forge"
  else
    echo "Could not locate forge binary; ensure Foundry is installed or set FORGE_BIN." >&2
    exit 66
  fi
fi

if ! python3 -c "import sys" >/dev/null 2>&1; then
  echo "python3 is required to resolve deployment configuration." >&2
  exit 67
fi

mapfile -t INFO < <(python3 scripts/deployment_info.py "$CONFIG_PATH" "$NETWORK")
STATUS=$?
if [[ $STATUS -ne 0 ]]; then
  echo "Failed to resolve deployment metadata for '$NETWORK' (exit code $STATUS)." >&2
  exit $STATUS
fi

RPC_URL="${INFO[0]}"
CONTRACT_ADDRESS="${INFO[1]}"
NETWORK_LABEL="${INFO[2]:-$NETWORK}"

echo "Pausing contract on $NETWORK_LABEL ($NETWORK) via $RPC_URL"
"$FORGE_BIN" script contracts/script/AdminActions.s.sol:AdminActions \
  --sig "pause(address,bool)" "$CONTRACT_ADDRESS" "$PAUSED" \
  --rpc-url "$RPC_URL" \
  --broadcast \
  --private-key "$ADMIN_PRIVATE_KEY" \
  --slow
