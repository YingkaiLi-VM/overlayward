pub mod error;
pub mod extract;
pub mod middleware;
pub mod routes;

use axum::{Extension, middleware as axum_mw};
use middleware::AuthState;
use ow_core::{ServiceRegistry, TokenResolver};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub struct RestServer {
    pub registry: Arc<ServiceRegistry>,
    pub token_resolver: Arc<dyn TokenResolver>,
    pub port: u16,
}

impl RestServer {
    pub fn new(registry: Arc<ServiceRegistry>, token_resolver: Arc<dyn TokenResolver>) -> Self {
        Self { registry, token_resolver, port: 8420 }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub async fn run(self) -> std::io::Result<()> {
        let auth_state = AuthState { resolver: self.token_resolver };
        let app = routes::api_routes(self.registry)
            .layer(axum_mw::from_fn(middleware::auth_middleware))
            .layer(Extension(auth_state))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        tracing::info!("REST API listening on {addr}");
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await
    }
}
