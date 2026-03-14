use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Network {
    pub name: String,
    pub id: String,
    pub driver: String,
    pub network_interface: Option<String>,
    pub subnets: Vec<Subnet>,
    pub ipv6_enabled: bool,
    pub internal: bool,
    pub dns_enabled: bool,
    pub labels: HashMap<String, String>,
    pub created: DateTime<Utc>,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Subnet {
    pub subnet: String,
    pub gateway: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNetworkRequest {
    pub name: String,
    pub driver: Option<String>,
    pub subnets: Option<Vec<Subnet>>,
    pub ipv6_enabled: Option<bool>,
    pub internal: Option<bool>,
    pub dns_enabled: Option<bool>,
    pub labels: Option<HashMap<String, String>>,
    pub node_id: Option<String>,
}
