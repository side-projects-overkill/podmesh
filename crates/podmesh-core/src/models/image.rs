use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Image {
    pub id: String,
    pub names: Vec<String>,
    pub digest: String,
    pub size: u64,
    pub created: DateTime<Utc>,
    pub labels: HashMap<String, String>,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageInspect {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests: Vec<String>,
    pub size: u64,
    pub config: ImageConfig,
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageConfig {
    pub env: Vec<String>,
    pub cmd: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub exposed_ports: HashMap<String, serde_json::Value>,
    pub working_dir: Option<String>,
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PullImageRequest {
    pub reference: String,
    pub node_id: Option<String>,
}
