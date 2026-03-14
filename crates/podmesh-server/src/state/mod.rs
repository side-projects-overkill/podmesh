use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use podmesh_core::models::{Event, Node};

use crate::config::ServerConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub nodes: Arc<RwLock<HashMap<Uuid, Node>>>,
    pub event_tx: broadcast::Sender<Event>,
}

impl AppState {
    pub fn new(config: ServerConfig) -> Self {
        let (event_tx, _) = broadcast::channel(1024);
        Self {
            config,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
        }
    }
}
