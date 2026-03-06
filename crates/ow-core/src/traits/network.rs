use async_trait::async_trait;
use ow_types::*;

#[async_trait]
pub trait NetworkManager: Send + Sync + 'static {
    async fn get(&self, sandbox_id: &str) -> Result<NetworkPolicy, ApiError>;
    async fn allow(&self, req: AddNetworkRuleRequest) -> Result<AddRuleResult, ApiError>;
    async fn deny(&self, sandbox_id: &str, rule_id: &str) -> Result<(), ApiError>;
    async fn set_default(&self, sandbox_id: &str, default: &str) -> Result<(), ApiError>;
}
