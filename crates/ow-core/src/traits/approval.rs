use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait ApprovalManager: Send + Sync + 'static {
    async fn list(&self, filter: ApprovalListFilter) -> Result<Vec<Approval>, ApiError>;
    async fn decide(&self, decision: ApprovalDecision) -> Result<(), ApiError>;
}
