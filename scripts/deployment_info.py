#!/usr/bin/env python3
"""
Utility helper that extracts deployment metadata for a configured network.
Outputs three lines: RPC URL, contract address, and human-readable network label.
"""

import sys
from pathlib import Path


def parse_config(path: Path) -> dict:
    data = {}
    current = None
    try:
        lines = path.read_text().splitlines()
    except FileNotFoundError:
        return {}

    for raw_line in lines:
        line = raw_line.split("#", 1)[0].strip()
        if not line:
            continue
        if line.startswith("[") and line.endswith("]"):
            current = line[1:-1].strip()
            data.setdefault(current, {})
            continue
        if "=" in line and current:
            key, value = line.split("=", 1)
            data[current][key.strip()] = value.strip().strip('"')
    return data


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: deployment_info.py <config-path> <network-key>", file=sys.stderr)
        return 64

    config_path = Path(sys.argv[1])
    network_key = sys.argv[2]
    entries = parse_config(config_path)

    entry = entries.get(network_key)
    if not entry:
        print(f"network '{network_key}' not found in {config_path}", file=sys.stderr)
        return 65

    rpc = entry.get("rpc_url")
    verifier = entry.get("otp_verifier")
    label = entry.get("network", network_key)

    if not rpc or not verifier:
        print(
            f"network '{network_key}' is missing 'rpc_url' or 'otp_verifier' fields",
            file=sys.stderr,
        )
        return 66

    print(rpc)
    print(verifier)
    print(label)
    return 0


if __name__ == "__main__":
    sys.exit(main())
