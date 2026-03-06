use async_trait::async_trait;
use ow_types::{ApiError, CallerIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardianVerdict {
    Allow,
    Deny,
}

#[async_trait]
pub trait Guardian: Send + Sync + 'static {
    async fn check(
        &self,
        operation: &str,
        params: &sonic_rs::Value,
        caller: &CallerIdentity,
    ) -> Result<GuardianVerdict, ApiError>;
}
