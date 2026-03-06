use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait ExecManager: Send + Sync + 'static {
    async fn run(&self, req: ExecRequest) -> Result<ExecResult, ApiError>;
}
