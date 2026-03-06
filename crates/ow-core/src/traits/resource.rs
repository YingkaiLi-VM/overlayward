use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait ResourceManager: Send + Sync + 'static {
    async fn usage(&self, sandbox_id: &str) -> Result<ResourceUsage, ApiError>;
    async fn resize(&self, req: ResourceResizeRequest) -> Result<(), ApiError>;
}
