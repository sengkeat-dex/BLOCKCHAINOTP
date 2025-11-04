use blockchain_otp::{create_app, mcp_server::BlockchainOtpMcpServer, mcp_handler::handle_mcp_connection};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Start the REST API server
    let api_app = create_app();
    let api_addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Blockchain OTP service listening on http://{}", api_addr);

    // Start the MCP server
    let mcp_server = Arc::new(BlockchainOtpMcpServer::new());
    let mcp_addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    println!("MCP server listening on http://{}", mcp_addr);

    // Create listeners for both servers
    let api_listener = TcpListener::bind(api_addr).await.unwrap();
    let mcp_listener = TcpListener::bind(mcp_addr).await.unwrap();

    // Run both servers concurrently
    tokio::select! {
        result = run_api_server(api_app, api_listener) => {
            if let Err(e) = result {
                eprintln!("API server error: {}", e);
            }
        }
        result = run_mcp_server(mcp_server, mcp_listener) => {
            if let Err(e) = result {
                eprintln!("MCP server error: {}", e);
            }
        }
    }
}

async fn run_api_server(
    app: axum::Router,
    listener: TcpListener,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let std_listener = listener.into_std()?;
    std_listener.set_nonblocking(false)?;
    axum::Server::from_tcp(std_listener)?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn run_mcp_server(server: Arc<BlockchainOtpMcpServer>, listener: TcpListener) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let (socket, _) = listener.accept().await?;
        let server_clone = server.clone();
        
        tokio::spawn(async move {
            if let Err(e) = handle_mcp_connection(socket, server_clone).await {
                eprintln!("Error handling MCP connection: {}", e);
            }
        });
    }
}
