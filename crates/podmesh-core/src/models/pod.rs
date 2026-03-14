use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::types::PodState;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Pod {
    pub id: String,
    pub name: String,
    pub state: PodState,
    pub created: DateTime<Utc>,
    pub infra_container_id: Option<String>,
    pub containers: Vec<PodContainer>,
    pub labels: HashMap<String, String>,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PodContainer {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePodRequest {
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
    pub infra_command: Option<Vec<String>>,
    pub share: Option<Vec<String>>,
    pub node_id: Option<String>,
}
