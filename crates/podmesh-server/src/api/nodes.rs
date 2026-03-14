use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use uuid::Uuid;

use podmesh_core::models::{ApiResponse, Node, RegisterNodeRequest};
use podmesh_core::types::NodeStatus;

use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/nodes",
    tag = "nodes",
    responses(
        (status = 200, description = "List of registered nodes", body = Vec<Node>)
    )
)]
pub async fn list_nodes(State(state): State<AppState>) -> Json<ApiResponse<Vec<Node>>> {
    let nodes = state.nodes.read().await;
    let list: Vec<Node> = nodes.values().cloned().collect();
    Json(ApiResponse::ok(list))
}

#[utoipa::path(
    post,
    path = "/api/nodes",
    tag = "nodes",
    request_body = RegisterNodeRequest,
    responses(
        (status = 201, description = "Node registered", body = Node)
    )
)]
pub async fn register_node(
    State(state): State<AppState>,
    Json(req): Json<RegisterNodeRequest>,
) -> Json<ApiResponse<Node>> {
    let node = Node {
        id: Uuid::new_v4(),
        name: req.name,
        endpoint: req.endpoint,
        status: NodeStatus::Online,
        podman_version: None,
        os: None,
        arch: None,
        labels: req.labels.unwrap_or_default(),
        last_seen: Utc::now(),
        created_at: Utc::now(),
    };

    let mut nodes = state.nodes.write().await;
    nodes.insert(node.id, node.clone());

    Json(ApiResponse::ok(node))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/nodes", get(list_nodes).post(register_node))
}
