# Claude Desktop Configuration for Blockchain OTP MCP Server

This document explains how to configure Claude Desktop to use the Blockchain OTP MCP server.

## Prerequisites

1. Claude Desktop installed
2. Blockchain OTP system running (MCP server on port 3002)

## Configuration Steps

1. Open Claude Desktop
2. Go to Settings > Developer > MCP Servers
3. Click "Add Server"
4. Enter the following configuration:

```
Name: Blockchain OTP Server
Command: tcp
Args: ["localhost:3002"]
```

5. Click "Save"

## Usage

Once configured, Claude can access information about the Blockchain OTP system:

1. Ask questions about the system architecture
2. Request information about API endpoints
3. Get details about blockchain integration

## Example Prompts

Try these prompts with Claude:

- "What is the Blockchain OTP system?"
- "How does the OTP verification work?"
- "What API endpoints are available?"
- "Tell me about the blockchain integration"

Claude will use the MCP server to retrieve accurate information about the system.