mod tools;

use ow_core::ServiceRegistry;
use rmcp::ServiceExt;
use rmcp::transport::StreamableHttpServerConfig;
use rmcp::transport::StreamableHttpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use std::net::SocketAddr;
use std::sync::Arc;

pub use tools::OverlaywardMcp;

/// Run MCP server over stdio transport.
pub async fn run_stdio(registry: Arc<ServiceRegistry>) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("MCP server starting on stdio");
    let transport = rmcp::transport::io::stdio();
    let server = OverlaywardMcp::new(registry).serve(transport).await?;
    server.waiting().await?;
    Ok(())
}

/// Run MCP server over Streamable HTTP transport (binds to the given port).
///
/// The MCP endpoint will be available at `http://0.0.0.0:{port}/mcp`.
pub async fn run_http(registry: Arc<ServiceRegistry>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let service = StreamableHttpService::new(
        move || Ok(OverlaywardMcp::new(registry.clone())),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );

    let app = axum::Router::new().nest_service("/mcp", service);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("MCP HTTP server listening on {addr} (endpoint: /mcp)");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
