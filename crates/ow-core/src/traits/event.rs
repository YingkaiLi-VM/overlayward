use async_trait::async_trait;
use ow_types::*;
use tokio::sync::broadcast;

#[async_trait]
pub trait EventManager: Send + Sync + 'static {
    fn subscribe(&self) -> broadcast::Receiver<Event>;
    async fn emit(&self, event: Event);
}
