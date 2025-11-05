#!/usr/bin/env python3
"""
Test script for the enhanced Blockchain OTP MCP server with blockchain testing tools
This script demonstrates how to communicate with the MCP server to run blockchain security tests
"""

import json
import socket

def send_mcp_request(host, port, method, params=None, request_id=1):
    """Send an MCP request to the server and return the response"""
    # Create the JSON-RPC request
    request = {
        "jsonrpc": "2.0",
        "id": request_id,
        "method": method
    }

    if params is not None:
        request["params"] = params

    # Convert to JSON string
    request_str = json.dumps(request) + "\n"

    # Connect to the server
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((host, port))

        # Send the request
        sock.sendall(request_str.encode('utf-8'))

        # Receive the response
        response_data = sock.recv(4096)

        # Parse and return the response
        response_str = response_data.decode('utf-8').strip()
        return json.loads(response_str)

def main():
    host = "localhost"
    port = 3002

    print("Testing Enhanced Blockchain OTP MCP Server with Blockchain Testing Tools")
    print("=" * 70)

    # Test list_tools to see our new blockchain testing tools
    print("\n1. Testing list_tools:")
    try:
        response = send_mcp_request(host, port, "list_tools")
        tool_names = [tool['name'] for tool in response.get('result', {}).get('tools', [])]
        print(f"Available tools: {', '.join(tool_names)}")
    except Exception as e:
        print(f"Error: {e}")

    # Test anvil.start
    print("\n2. Testing anvil.start:")
    try:
        response = send_mcp_request(host, port, "call_tool", {
            "name": "anvil.start",
            "arguments": {
                "forkUrl": "https://mainnet.infura.io/v3/YOUR_PROJECT_ID",
                "blockNumber": 15000000,
                "chainId": 1
            }
        })
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")

    # Test forge.test
    print("\n3. Testing forge.test:")
    try:
        response = send_mcp_request(host, port, "call_tool", {
            "name": "forge.test",
            "arguments": {
                "pattern": "testTransfer",
                "fuzzRuns": 1000,
                "matchContract": "TokenContract"
            }
        })
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")

    # Test echidna.run
    print("\n4. Testing echidna.run:")
    try:
        response = send_mcp_request(host, port, "call_tool", {
            "name": "echidna.run",
            "arguments": {
                "config": "/path/to/echidna_config.yaml",
                "targetContracts": ["MyToken", "MyVault"],
                "contractName": "MyContract"
            }
        })
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")

    # Test mythril.analyze
    print("\n5. Testing mythril.analyze:")
    try:
        response = send_mcp_request(host, port, "call_tool", {
            "name": "mythril.analyze",
            "arguments": {
                "contractPath": "/path/to/MyContract.sol",
                "solcVersion": "0.8.20",
                "truffle": False
            }
        })
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")

    # Test get_security_report
    print("\n6. Testing get_security_report:")
    try:
        response = send_mcp_request(host, port, "call_tool", {
            "name": "get_security_report",
            "arguments": {
                "report_id": "mythril_report_20251105_120000"
            }
        })
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()