use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait AuditManager: Send + Sync + 'static {
    async fn query(&self, query: AuditQuery) -> Result<AuditQueryResult, ApiError>;
    async fn detail(&self, sandbox_id: &str, event_id: &str) -> Result<AuditEvent, ApiError>;
    async fn replay(&self, req: AuditReplayRequest) -> Result<Vec<AuditEvent>, ApiError>;
}
