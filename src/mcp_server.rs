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
            // Blockchain testing tools
            Tool {
                name: "anvil.start".to_string(),
                description: "Start local EVM (optionally fork mainnet/testnet), configure block/time, chainId".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "forkUrl": {
                            "type": "string",
                            "description": "URL to fork from"
                        },
                        "blockNumber": {
                            "type": "integer",
                            "description": "Block number to fork from"
                        },
                        "chainId": {
                            "type": "integer",
                            "description": "Chain ID to use"
                        }
                    },
                    "required": []
                }),
            },
            Tool {
                name: "forge.test".to_string(),
                description: "Run Foundry unit & fuzz tests, return JUnit/JSON".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Test pattern to match"
                        },
                        "fuzzRuns": {
                            "type": "integer",
                            "description": "Number of fuzz runs"
                        },
                        "matchContract": {
                            "type": "string",
                            "description": "Contract name to match"
                        }
                    },
                    "required": []
                }),
            },
            Tool {
                name: "echidna.run".to_string(),
                description: "Property-based fuzzing against user predicates".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "config": {
                            "type": "string",
                            "description": "Path to Echidna config file"
                        },
                        "targetContracts": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Target contracts to test"
                        },
                        "contractName": {
                            "type": "string",
                            "description": "Name of the contract to test"
                        }
                    },
                    "required": ["config"]
                }),
            },
            Tool {
                name: "mythril.analyze".to_string(),
                description: "Run Mythril on bytecode/solc-output; return detected vulns".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "contractPath": {
                            "type": "string",
                            "description": "Path to the contract file"
                        },
                        "solcVersion": {
                            "type": "string",
                            "description": "Solidity compiler version"
                        },
                        "truffle": {
                            "type": "boolean",
                            "description": "Whether to use Truffle"
                        }
                    },
                    "required": ["contractPath"]
                }),
            },
            Tool {
                name: "get_security_report".to_string(),
                description: "Get a security report by ID".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "report_id": {
                            "type": "string",
                            "description": "ID of the security report to retrieve"
                        }
                    },
                    "required": ["report_id"]
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
            "anvil.start" => {
                let result = json!({
                    "rpcUrl": "http://localhost:8545",
                    "forkId": "fork_1234567890"
                });
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "forge.test" => {
                let result = json!({
                    "passed": 15,
                    "failed": 0,
                    "coverage": "85%"
                });
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "echidna.run" => {
                let result = json!({
                    "counterexamples": [],
                    "stats": {
                        "tests_run": 1000,
                        "time_elapsed": "15s"
                    }
                });
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "mythril.analyze" => {
                let result = json!({
                    "findings": [],
                    "severityCounts": {
                        "critical": 0,
                        "high": 0,
                        "medium": 0,
                        "low": 0
                    }
                });
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "get_security_report" => {
                let result = json!({
                    "report_id": request.arguments.get("report_id"),
                    "status": "completed",
                    "findings": [],
                    "generated_at": "2025-11-05T12:00:00Z"
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