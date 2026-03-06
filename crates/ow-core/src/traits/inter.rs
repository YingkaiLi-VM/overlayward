use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait InterManager: Send + Sync + 'static {
    async fn connect(&self, req: InterConnectRequest) -> Result<(), ApiError>;
    async fn send(&self, msg: InterMessage) -> Result<(), ApiError>;
    async fn disconnect(&self, sandbox_a: &str, sandbox_b: &str) -> Result<(), ApiError>;
}
