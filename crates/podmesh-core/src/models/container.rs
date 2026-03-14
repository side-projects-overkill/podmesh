use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::types::ContainerState;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub image_id: String,
    pub state: ContainerState,
    pub status: String,
    pub created: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ports: Vec<PortMapping>,
    pub labels: HashMap<String, String>,
    pub pod_id: Option<String>,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PortMapping {
    pub host_ip: Option<String>,
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContainerStats {
    pub container_id: String,
    pub cpu_percent: f64,
    pub memory_usage_bytes: u64,
    pub memory_limit_bytes: u64,
    pub memory_percent: f64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub block_read_bytes: u64,
    pub block_write_bytes: u64,
    pub pids: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContainerInspect {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub config: ContainerConfig,
    pub network_settings: serde_json::Value,
    pub mounts: Vec<Mount>,
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContainerConfig {
    pub hostname: Option<String>,
    pub env: Vec<String>,
    pub cmd: Vec<String>,
    pub entrypoint: Option<Vec<String>>,
    pub working_dir: Option<String>,
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Mount {
    pub source: String,
    pub destination: String,
    pub r#type: String,
    pub mode: String,
    pub rw: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateContainerRequest {
    pub name: Option<String>,
    pub image: String,
    pub cmd: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub ports: Option<Vec<PortMapping>>,
    pub volumes: Option<Vec<String>>,
    pub labels: Option<HashMap<String, String>>,
    pub pod: Option<String>,
    pub node_id: Option<String>,
}
