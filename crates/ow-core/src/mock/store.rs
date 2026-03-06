use ow_types::*;
use papaya::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct InMemoryStore {
    pub sandboxes: HashMap<String, Sandbox>,
    pub snapshots: HashMap<String, Vec<Snapshot>>,
    pub network_policies: HashMap<String, NetworkPolicy>,
    pub audit_events: HashMap<String, Vec<AuditEvent>>,
    pub resource_usage: HashMap<String, ResourceUsage>,
    pub volumes: HashMap<String, Vec<Volume>>,
    pub approvals: HashMap<String, Approval>,
    pub connections: RwLock<Vec<InterConnection>>,
    pub files: HashMap<String, std::collections::HashMap<String, Vec<u8>>>,
    pub event_tx: broadcast::Sender<Event>,
}

impl InMemoryStore {
    pub fn new() -> Arc<Self> {
        let (event_tx, _) = broadcast::channel(256);
        Arc::new(Self {
            sandboxes: HashMap::new(),
            snapshots: HashMap::new(),
            network_policies: HashMap::new(),
            audit_events: HashMap::new(),
            resource_usage: HashMap::new(),
            volumes: HashMap::new(),
            approvals: HashMap::new(),
            connections: RwLock::new(Vec::new()),
            files: HashMap::new(),
            event_tx,
        })
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        let (event_tx, _) = broadcast::channel(256);
        Self {
            sandboxes: HashMap::new(),
            snapshots: HashMap::new(),
            network_policies: HashMap::new(),
            audit_events: HashMap::new(),
            resource_usage: HashMap::new(),
            volumes: HashMap::new(),
            approvals: HashMap::new(),
            connections: RwLock::new(Vec::new()),
            files: HashMap::new(),
            event_tx,
        }
    }
}
