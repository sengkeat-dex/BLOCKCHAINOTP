//! JSON-RPC handler for the MCP (Model Context Protocol) server

use crate::mcp_server::{BlockchainOtpMcpServer, CallToolRequest, ReadResourceRequest};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

/// JSON-RPC request structure
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

/// JSON-RPC success response structure
#[derive(Debug, Serialize)]
pub struct JsonRpcSuccessResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub result: Value,
}

/// JSON-RPC error response structure
#[derive(Debug, Serialize)]
pub struct JsonRpcErrorResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub error: JsonRpcError,
}

/// JSON-RPC error details
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

/// Handle an MCP connection
pub async fn handle_mcp_connection(
    stream: TcpStream,
    server: Arc<BlockchainOtpMcpServer>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = buf_reader.read_line(&mut line).await?;
        
        if bytes_read == 0 {
            // Connection closed
            break;
        }

        // Parse the JSON-RPC request
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                let error_response = JsonRpcErrorResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    error: JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                    },
                };
                let response_str = serde_json::to_string(&error_response)?;
                writer.write_all(response_str.as_bytes()).await?;
                writer.write_all(b"\n").await?;
                continue;
            }
        };

        // Process the request based on the method
        let result = match request.method.as_str() {
            "list_resources" => {
                let resources = server.list_resources();
                json!({
                    "resources": resources.resources
                })
            }
            "read_resource" => {
                if let Some(params) = request.params {
                    if let Ok(read_request) = serde_json::from_value::<ReadResourceRequest>(params) {
                        let resource = server.read_resource(read_request);
                        json!({
                            "contents": resource.contents
                        })
                    } else {
                        json!({
                            "error": "Invalid parameters for read_resource"
                        })
                    }
                } else {
                    json!({
                        "error": "Missing parameters for read_resource"
                    })
                }
            }
            "list_tools" => {
                let tools = server.list_tools();
                json!({
                    "tools": tools.tools
                })
            }
            "call_tool" => {
                if let Some(params) = request.params {
                    if let Ok(tool_request) = serde_json::from_value::<CallToolRequest>(params) {
                        let result = server.call_tool(tool_request);
                        json!({
                            "content": result.content,
                            "is_error": result.is_error
                        })
                    } else {
                        json!({
                            "error": "Invalid parameters for call_tool"
                        })
                    }
                } else {
                    json!({
                        "error": "Missing parameters for call_tool"
                    })
                }
            }
            _ => {
                json!({
                    "error": format!("Method not found: {}", request.method)
                })
            }
        };

        // Send the response
        let response = JsonRpcSuccessResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result,
        };

        let response_str = serde_json::to_string(&response)?;
        writer.write_all(response_str.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}
