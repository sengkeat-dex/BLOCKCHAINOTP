# MCP (Model Context Protocol) Server for Blockchain OTP

This document explains how to use the MCP server implementation in the Blockchain OTP project.

## Overview

The MCP server provides context about the Blockchain OTP system to AI assistants. It allows AI models to:

1. Access documentation and resources about the system
2. Call tools to get information about OTP requests and system status
3. Understand the architecture and components of the system

## Implementation Details

The MCP server is implemented in [src/mcp_server.rs](src/mcp_server.rs) and provides the following capabilities:

### Resources

The server exposes the following resources:

1. `system-overview` - General information about the Blockchain OTP system
2. `api-endpoints` - List of available API endpoints
3. `blockchain-integration` - Information about blockchain integration

### Tools

The server provides the following tools:

1. `get_otp_status` - Get the status of an OTP request
2. `get_system_info` - Get general information about the system

## How to Use

To use the MCP server:

1. Start the application with `cargo run`
2. The MCP server will be available on port 3002
3. Connect an AI assistant that supports MCP to `localhost:3002`

## Example Usage

An AI assistant can query the system information by calling the `get_system_info` tool:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "call_tool",
  "params": {
    "name": "get_system_info",
    "arguments": {}
  }
}
```

The server will respond with:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": ["{\"name\":\"Blockchain OTP System\",\"version\":\"0.1.0\",\"description\":\"A blockchain-based one-time password system\",\"supported_blockchains\":[\"Ethereum\",\"Solana\"],\"api_endpoints\":[\"/otp/request\",\"/otp/verify\",\"/health\"]}"],
    "is_error": false
  }
}
```

## Integration with AI Assistants

The MCP server can be used with any AI assistant that supports the Model Context Protocol, including:

- Claude Desktop
- Other MCP-compatible tools

To connect an AI assistant, configure it to connect to `localhost:3002` where the MCP server is running.