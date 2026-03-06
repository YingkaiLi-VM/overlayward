use async_trait::async_trait;
use bytes::Bytes;
use ow_types::*;

#[async_trait]
pub trait FileManager: Send + Sync + 'static {
    async fn read(&self, sandbox_id: &str, path: &str, offset: Option<u64>, limit: Option<u64>) -> Result<FileContent, ApiError>;
    async fn write(&self, sandbox_id: &str, path: &str, content: &[u8], mode: Option<&str>) -> Result<(), ApiError>;
    async fn list(&self, sandbox_id: &str, path: &str, recursive: bool) -> Result<Vec<FileEntry>, ApiError>;
    async fn upload(&self, sandbox_id: &str, dest: &str, data: Bytes) -> Result<(), ApiError>;
    async fn download(&self, sandbox_id: &str, path: &str) -> Result<Bytes, ApiError>;
}
