use clap::Parser;
use ow_cli::{Cli, Commands};
use ow_core::{MockTokenResolver, ServiceRegistry, mock::{InMemoryStore, MockBackend, MockGuardian}};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve(args) => run_serve(args.rest_port, args.grpc_port, args.mcp_port).await,
        Commands::McpServer => run_mcp().await,
        _ => {
            let code = ow_cli::run(cli).await;
            std::process::exit(code);
        }
    }
}

async fn run_serve(rest_port: u16, grpc_port: u16, mcp_port: u16) {
    let registry = build_registry();
    let resolver: Arc<dyn ow_core::TokenResolver> = Arc::new(MockTokenResolver);

    let rest_reg = registry.clone();
    let rest_resolver = resolver.clone();
    let rest = tokio::spawn(async move {
        ow_rest::RestServer::new(rest_reg, rest_resolver)
            .with_port(rest_port)
            .run()
            .await
            .expect("REST server failed");
    });

    let grpc_reg = registry.clone();
    let grpc = tokio::spawn(async move {
        ow_grpc::GrpcServer::new(grpc_reg)
            .with_port(grpc_port)
            .run()
            .await
            .expect("gRPC server failed");
    });

    let mcp_reg = registry.clone();
    let mcp = tokio::spawn(async move {
        ow_mcp::run_http(mcp_reg, mcp_port)
            .await
            .expect("MCP HTTP server failed");
    });

    tracing::info!("Overlayward servers started — REST :{rest_port} | gRPC :{grpc_port} | MCP :{mcp_port}");

    tokio::select! {
        r = rest => { if let Err(e) = r { tracing::error!("REST: {e}"); } }
        r = grpc => { if let Err(e) = r { tracing::error!("gRPC: {e}"); } }
        r = mcp => { if let Err(e) = r { tracing::error!("MCP: {e}"); } }
    }
}

async fn run_mcp() {
    let registry = build_registry();
    ow_mcp::run_stdio(registry).await.expect("MCP server failed");
}

fn build_registry() -> Arc<ServiceRegistry> {
    let store = InMemoryStore::new();
    let backend = Arc::new(MockBackend::new(store.clone()));
    Arc::new(ServiceRegistry {
        guardian: Arc::new(MockGuardian),
        sandbox: backend.clone(),
        snapshot: backend.clone(),
        network: backend.clone(),
        exec: backend.clone(),
        file: backend.clone(),
        volume: backend.clone(),
        audit: backend.clone(),
        resource: backend.clone(),
        inter: backend.clone(),
        approval: backend.clone(),
        event: backend,
    })
}
