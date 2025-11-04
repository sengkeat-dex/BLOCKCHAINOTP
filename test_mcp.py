#!/usr/bin/env python3
"""
Test script for the Blockchain OTP MCP server
This script demonstrates how to communicate with the MCP server
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
    
    print("Testing Blockchain OTP MCP Server")
    print("=" * 40)
    
    # Test list_resources
    print("\n1. Testing list_resources:")
    try:
        response = send_mcp_request(host, port, "list_resources")
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")
    
    # Test list_tools
    print("\n2. Testing list_tools:")
    try:
        response = send_mcp_request(host, port, "list_tools")
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")
    
    # Test call_tool with get_system_info
    print("\n3. Testing call_tool (get_system_info):")
    try:
        response = send_mcp_request(host, port, "call_tool", {
            "name": "get_system_info",
            "arguments": {}
        })
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")
    
    # Test call_tool with get_otp_status
    print("\n4. Testing call_tool (get_otp_status):")
    try:
        response = send_mcp_request(host, port, "call_tool", {
            "name": "get_otp_status",
            "arguments": {
                "request_id": "example-request-id"
            }
        })
        print(json.dumps(response, indent=2))
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()