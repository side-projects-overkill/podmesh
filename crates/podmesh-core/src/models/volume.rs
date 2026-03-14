use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Volume {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub labels: HashMap<String, String>,
    pub options: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub node_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVolumeRequest {
    pub name: String,
    pub driver: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub options: Option<HashMap<String, String>>,
    pub node_id: Option<String>,
}
