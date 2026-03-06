use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait SandboxManager: Send + Sync + 'static {
    async fn create(&self, req: CreateSandboxRequest) -> Result<Sandbox, ApiError>;
    async fn start(&self, id: &str) -> Result<(), ApiError>;
    async fn pause(&self, id: &str) -> Result<(), ApiError>;
    async fn resume(&self, id: &str) -> Result<(), ApiError>;
    async fn stop(&self, id: &str, force: bool) -> Result<(), ApiError>;
    async fn destroy(&self, id: &str, opts: DestroyOptions) -> Result<(), ApiError>;
    async fn list(&self, filter: ListFilter) -> Result<Vec<Sandbox>, ApiError>;
    async fn info(&self, id: &str) -> Result<Sandbox, ApiError>;
}
