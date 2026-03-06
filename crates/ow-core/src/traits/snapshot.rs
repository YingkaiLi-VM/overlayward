use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait SnapshotManager: Send + Sync + 'static {
    async fn save(&self, sandbox_id: &str, name: Option<&str>, description: Option<&str>) -> Result<Snapshot, ApiError>;
    async fn restore(&self, sandbox_id: &str, snapshot_id: &str) -> Result<(), ApiError>;
    async fn list(&self, sandbox_id: &str) -> Result<Vec<Snapshot>, ApiError>;
    async fn delete(&self, sandbox_id: &str, snapshot_id: &str) -> Result<(), ApiError>;
    async fn diff(&self, sandbox_id: &str, from: &str, to: &str) -> Result<SnapshotDiff, ApiError>;
}
