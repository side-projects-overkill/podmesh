use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::types::NodeStatus;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub endpoint: String,
    pub status: NodeStatus,
    pub podman_version: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub labels: std::collections::HashMap<String, String>,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NodeStats {
    pub node_id: Uuid,
    pub cpu_usage_percent: f64,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub containers_running: u32,
    pub containers_stopped: u32,
    pub pods_running: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterNodeRequest {
    pub name: String,
    pub endpoint: String,
    pub labels: Option<std::collections::HashMap<String, String>>,
}
