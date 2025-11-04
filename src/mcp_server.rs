//! MCP (Model Context Protocol) server implementation for the Blockchain OTP system.
//! This module provides context about the blockchain OTP system to AI agents.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// MCP server implementation for the Blockchain OTP system
#[derive(Clone)]
pub struct BlockchainOtpMcpServer {
    resources: HashMap<String, String>,
    tools: Vec<Tool>,
}

/// Represents a resource that can be accessed via MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

/// Represents a tool that can be called via MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// Result of listing resources
#[derive(Debug, Serialize, Deserialize)]
pub struct ListResourcesResult {
    pub resources: Vec<Resource>,
}

/// Request to read a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceRequest {
    pub uri: String,
}

/// Result of reading a resource
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResourceResult {
    pub contents: Vec<String>,
}

/// Result of listing tools
#[derive(Debug, Serialize, Deserialize)]
pub struct ListToolsResult {
    pub tools: Vec<Tool>,
}

/// Request to call a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolRequest {
    pub name: String,
    pub arguments: Value,
}

/// Result of calling a tool
#[derive(Debug, Serialize, Deserialize)]
pub struct CallToolResult {
    pub content: Vec<String>,
    pub is_error: bool,
}

impl BlockchainOtpMcpServer {
    /// Create a new MCP server instance
    pub fn new() -> Self {
        let mut resources = HashMap::new();
        resources.insert(
            "system-overview".to_string(),
            r#"Blockchain OTP System Overview:
This system provides time-based one-time passwords (TOTP) using blockchain technology for enhanced security.
Key components:
1. Smart contracts for OTP verification on Ethereum and Solana blockchains
2. Backend service for OTP generation and management
3. Web frontend for user interaction with MetaMask and Phantom wallet integration
4. Rate limiting and security features"#
                .to_string(),
        );

        resources.insert(
            "api-endpoints".to_string(),
            r#"Available API Endpoints:
- POST /otp/request - Generate a new OTP for a user
- POST /otp/verify - Verify an OTP submission
- GET /health - Health check endpoint
- GET /cors-test - CORS testing endpoint"#
                .to_string(),
        );

        resources.insert(
            "blockchain-integration".to_string(),
            r#"Blockchain Integration:
The system supports two major blockchains:
1. Ethereum - Using Solidity smart contracts
2. Solana - Using Rust-based smart contracts

Smart contracts handle the verification of OTPs on-chain, providing immutable proof of authentication."#
                .to_string(),
        );

        let tools = vec![
            Tool {
                name: "get_otp_status".to_string(),
                description: "Get the status of an OTP request".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "request_id": {
                            "type": "string",
                            "description": "The ID of the OTP request"
                        }
                    },
                    "required": ["request_id"]
                }),
            },
            Tool {
                name: "get_system_info".to_string(),
                description: "Get information about the blockchain OTP system".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
        ];

        Self { resources, tools }
    }

    /// List all available resources
    pub fn list_resources(&self) -> ListResourcesResult {
        let resources: Vec<Resource> = self
            .resources
            .keys()
            .map(|key| Resource {
                uri: format!("resource://{}", key),
                name: key.clone(),
                description: Some(format!("Resource for {}", key)),
                mime_type: Some("text/plain".to_string()),
            })
            .collect();

        ListResourcesResult { resources }
    }

    /// Read a specific resource
    pub fn read_resource(&self, request: ReadResourceRequest) -> ReadResourceResult {
        let resource_id = request.uri.strip_prefix("resource://").unwrap_or("");
        let content = self
            .resources
            .get(resource_id)
            .cloned()
            .unwrap_or_else(|| "Resource not found".to_string());

        ReadResourceResult { contents: vec![content] }
    }

    /// List all available tools
    pub fn list_tools(&self) -> ListToolsResult {
        ListToolsResult {
            tools: self.tools.clone(),
        }
    }

    /// Call a specific tool
    pub fn call_tool(&self, request: CallToolRequest) -> CallToolResult {
        match request.name.as_str() {
            "get_otp_status" => {
                // In a real implementation, this would check the actual OTP status
                let result = json!({
                    "request_id": request.arguments.get("request_id"),
                    "status": "example_status",
                    "expires_at": 1234567890u64
                });
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "get_system_info" => {
                let result = json!({
                    "name": "Blockchain OTP System",
                    "version": "0.1.0",
                    "description": "A blockchain-based one-time password system",
                    "supported_blockchains": ["Ethereum", "Solana"],
                    "api_endpoints": ["/otp/request", "/otp/verify", "/health"]
                });
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            _ => CallToolResult {
                content: vec!["Tool not found".to_string()],
                is_error: true,
            },
        }
    }
}