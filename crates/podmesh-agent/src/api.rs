use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use podmesh_client::PodmanClient;

#[derive(Clone)]
pub struct AgentState {
    pub podman: Arc<PodmanClient>,
    pub node_name: String,
}

impl AgentState {
    pub fn new(podman: PodmanClient, node_name: String) -> Self {
        Self {
            podman: Arc::new(podman),
            node_name,
        }
    }
}

#[derive(Serialize)]
struct AgentInfo {
    name: String,
    version: String,
    podman_reachable: bool,
}

async fn info(State(state): State<AgentState>) -> Json<AgentInfo> {
    let reachable = state.podman.ping().await.unwrap_or(false);
    Json(AgentInfo {
        name: state.node_name.clone(),
        version: env!("CARGO_PKG_VERSION").into(),
        podman_reachable: reachable,
    })
}

async fn health() -> &'static str {
    "ok"
}

async fn containers(State(state): State<AgentState>) -> Json<serde_json::Value> {
    match state.podman.containers().list(true).await {
        Ok(containers) => Json(serde_json::json!({ "containers": containers })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn pods(State(state): State<AgentState>) -> Json<serde_json::Value> {
    match state.podman.pods().list().await {
        Ok(pods) => Json(serde_json::json!({ "pods": pods })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn images(State(state): State<AgentState>) -> Json<serde_json::Value> {
    match state.podman.images().list().await {
        Ok(images) => Json(serde_json::json!({ "images": images })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn volumes(State(state): State<AgentState>) -> Json<serde_json::Value> {
    match state.podman.volumes().list().await {
        Ok(volumes) => Json(serde_json::json!({ "volumes": volumes })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn networks(State(state): State<AgentState>) -> Json<serde_json::Value> {
    match state.podman.networks().list().await {
        Ok(networks) => Json(serde_json::json!({ "networks": networks })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub fn router(state: AgentState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/containers", get(containers))
        .route("/pods", get(pods))
        .route("/images", get(images))
        .route("/volumes", get(volumes))
        .route("/networks", get(networks))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
