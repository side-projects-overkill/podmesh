use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "health",
    responses(
        (status = 200, description = "Server is healthy", body = HealthResponse)
    )
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

#[utoipa::path(
    get,
    path = "/api/ready",
    tag = "health",
    responses(
        (status = 200, description = "Server is ready", body = HealthResponse)
    )
)]
pub async fn readiness(State(state): State<AppState>) -> Json<HealthResponse> {
    let nodes = state.nodes.read().await;
    let status = if nodes.is_empty() {
        "no_nodes"
    } else {
        "ready"
    };
    Json(HealthResponse {
        status: status.into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(readiness))
}
