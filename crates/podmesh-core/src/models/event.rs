use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::types::EventKind;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Event {
    pub id: String,
    pub kind: EventKind,
    pub actor_id: String,
    pub actor_name: Option<String>,
    pub node_id: String,
    pub attributes: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EventFilter {
    pub kinds: Option<Vec<EventKind>>,
    pub node_id: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}
