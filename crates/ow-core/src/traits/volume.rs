use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait VolumeManager: Send + Sync + 'static {
    async fn mount(&self, req: VolumeMountRequest) -> Result<(), ApiError>;
    async fn unmount(&self, sandbox_id: &str, guest_path: &str) -> Result<(), ApiError>;
    async fn list(&self, sandbox_id: &str) -> Result<Vec<Volume>, ApiError>;
}
