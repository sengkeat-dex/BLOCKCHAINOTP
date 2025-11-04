# Blockchain OTP System

A blockchain-based one-time password (OTP) system that provides enhanced security through smart contract verification.

## Features

- Time-based one-time passwords (TOTP)
- Blockchain verification on Ethereum and Solana
- Rate limiting and security features
- Web frontend with MetaMask and Phantom wallet integration
- **MCP (Model Context Protocol) server for AI assistant integration**

## Architecture

The system consists of:

1. Smart contracts for OTP verification on Ethereum and Solana blockchains
2. Backend service for OTP generation and management
3. Web frontend for user interaction
4. **MCP server for AI assistant context (running on port 3002)**

## Getting Started

1. Install dependencies
2. Set up environment variables
3. Run the application with `cargo run`

## MCP Server

The system includes an MCP (Model Context Protocol) server that allows AI assistants to access information about the system:

- Running on port 3002
- Provides resources about system architecture
- Offers tools for querying system information
- Compatible with Claude Desktop and other MCP-compatible assistants

See [MCP_SERVER.md](MCP_SERVER.md) for detailed information on using the MCP server.