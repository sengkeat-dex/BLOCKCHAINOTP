    pub fn new() -> Self {
        let mut resources = HashMap::new();
        resources.insert(
            "system-overview".to_string(),
            r#"Blockchain OTP System Overview:
This system provides time-based one-time passwords (TOTP) using blockchain technology for enhanced security.                                                                  Key components:
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
        
        // Add blockchain testing resources
        resources.insert(
            "test-reports".to_string(),
            "Directory containing test reports and analysis results from blockchain security tools".to_string(),
        );
        
        resources.insert(
            "security-findings".to_string(),
            "Security vulnerabilities and findings from static and dynamic analysis tools".to_string(),
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
            // Add blockchain testing tools from mcp-testing.md
            Tool {
                name: "anvil.start".to_string(),
                description: "Start local EVM (optionally fork mainnet/testnet)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "forkUrl": {
                            "type": "string",
                            "description": "RPC endpoint URL to fork from"
                        },
                        "blockNumber": {
                            "type": "integer",
                            "description": "Block number to fork from"
                        },
                        "chainId": {
                            "type": "integer",
                            "description": "Chain ID to use"
                        }
                    }
                }),
            },
            Tool {
                name: "forge.test".to_string(),
                description: "Run Foundry unit & fuzz tests".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Test pattern to run"
                        },
                        "fuzzRuns": {
                            "type": "integer",
                            "description": "Number of fuzz runs"
                        },
                        "matchContract": {
                            "type": "string",
                            "description": "Contract name to match"
                        }
                    }
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
                description: "Run Mythril on bytecode/solc-output".to_string(),
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
                            "description": "Whether to use Truffle project structure"
                        }
                    },
                    "required": ["contractPath"]
                }),
            },
            Tool {
                name: "get_security_report".to_string(),
                description: "Retrieve a security analysis report".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "report_id": {
                            "type": "string",
                            "description": "ID of the security report"
                        }
                    },
                    "required": ["report_id"]
                }),
            }
        ];

        Self { resources, tools }
    }

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
            // Implement blockchain testing tools from mcp-testing.md
            "anvil.start" => {
                let fork_url = request.arguments.get("forkUrl").and_then(|v| v.as_str()).unwrap_or("");
                let block_number = request.arguments.get("blockNumber").and_then(|v| v.as_u64());
                let chain_id = request.arguments.get("chainId").and_then(|v| v.as_u64());
                
                // In a real implementation, this would start an Anvil node
                let result = json!({
                    "status": "success",
                    "message": "Anvil node would be started with the specified parameters",
                    "forkUrl": fork_url,
                    "blockNumber": block_number,
                    "chainId": chain_id,
                    "rpcEndpoint": "http://localhost:8545"
                });
                
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "forge.test" => {
                let pattern = request.arguments.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
                let fuzz_runs = request.arguments.get("fuzzRuns").and_then(|v| v.as_u64()).unwrap_or(256);
                let match_contract = request.arguments.get("matchContract").and_then(|v| v.as_str()).unwrap_or("");
                
                // In a real implementation, this would run Foundry tests
                let result = json!({
                    "status": "success",
                    "message": "Foundry tests would be run with the specified parameters",
                    "pattern": pattern,
                    "fuzzRuns": fuzz_runs,
                    "matchContract": match_contract,
                    "exampleOutput": "Running 10 tests for contract MyContract...\n[PASS] testBasic() (gas: 20000)\n[PASS] testFuzzing() (runs: 256)\nTest result: ok. 10 passed; 0 failed; 0 skipped"
                });
                
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "echidna.run" => {
                let config = request.arguments.get("config").and_then(|v| v.as_str()).unwrap_or("");
                let target_contracts = request.arguments.get("targetContracts").and_then(|v| v.as_array());
                let contract_name = request.arguments.get("contractName").and_then(|v| v.as_str()).unwrap_or("");
                
                // In a real implementation, this would run Echidna fuzzing
                let result = json!({
                    "status": "success",
                    "message": "Echidna fuzzing would be run with the specified parameters",
                    "config": config,
                    "targetContracts": target_contracts,
                    "contractName": contract_name,
                    "exampleOutput": "echidna: \nContract MyContract:\n  Property property1: FAILED\n  Property property2: PASSED\n  Coverage: 85%"
                });
                
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "mythril.analyze" => {
                let contract_path = request.arguments.get("contractPath").and_then(|v| v.as_str()).unwrap_or("");
                let solc_version = request.arguments.get("solcVersion").and_then(|v| v.as_str()).unwrap_or("");
                let truffle = request.arguments.get("truffle").and_then(|v| v.as_bool()).unwrap_or(false);
                
                // In a real implementation, this would run Mythril analysis
                let result = json!({
                    "status": "success",
                    "message": "Mythril analysis would be run with the specified parameters",
                    "contractPath": contract_path,
                    "solcVersion": solc_version,
                    "truffle": truffle,
                    "exampleFindings": [
                        {
                            "issue": "Integer Overflow",
                            "severity": "High",
                            "location": "MyContract.sol:45"
                        },
                        {
                            "issue": "Unchecked CALL return value",
                            "severity": "Medium",
                            "location": "MyContract.sol:78"
                        }
                    ]
                });
                
                CallToolResult {
                    content: vec![result.to_string()],
                    is_error: false,
                }
            }
            "get_security_report" => {
                let report_id = request.arguments.get("report_id").and_then(|v| v.as_str()).unwrap_or("");
                
                // In a real implementation, this would retrieve a security report
                let result = json!({
                    "status": "success",
                    "message": "Security report would be retrieved with the specified ID",
                    "reportId": report_id,
                    "exampleReport": {
                        "tool": "Mythril",
                        "timestamp": "2025-11-05T10:30:00Z",
                        "findings": [
                            {
                                "issue": "Reentrancy Vulnerability",
                                "severity": "High",
                                "location": "MyContract.sol:120",
                                "description": "The function withdraw() is vulnerable to reentrancy attacks"
                            }
                        ],
                        "recommendations": [
                            "Use the checks-effects-interactions pattern",
                            "Implement a reentrancy guard"
                        ]
                    }
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
