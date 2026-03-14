use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::Deserialize;

use podmesh_core::models::{ApiResponse, CreateContainerRequest};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListParams {
    #[serde(default)]
    pub all: bool,
    pub node_id: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/containers",
    tag = "containers",
    params(
        ("all" = Option<bool>, Query, description = "Show all containers including stopped"),
        ("node_id" = Option<String>, Query, description = "Filter by node ID"),
    ),
    responses(
        (status = 200, description = "Container list", body = serde_json::Value)
    )
)]
pub async fn list_containers(
    State(_state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<ApiResponse<serde_json::Value>> {
    // In production, this iterates over nodes and aggregates results.
    // For now, return a placeholder that shows the API shape.
    Json(ApiResponse::ok(serde_json::json!([])))
}

#[utoipa::path(
    get,
    path = "/api/containers/{id}",
    tag = "containers",
    params(("id" = String, Path, description = "Container ID")),
    responses(
        (status = 200, description = "Container details", body = serde_json::Value)
    )
)]
pub async fn get_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!({ "id": id })))
}

#[utoipa::path(
    post,
    path = "/api/containers",
    tag = "containers",
    request_body = CreateContainerRequest,
    responses(
        (status = 201, description = "Container created", body = serde_json::Value)
    )
)]
pub async fn create_container(
    State(_state): State<AppState>,
    Json(req): Json<CreateContainerRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "image": req.image,
        "status": "created"
    })))
}

#[utoipa::path(
    post,
    path = "/api/containers/{id}/start",
    tag = "containers",
    params(("id" = String, Path, description = "Container ID")),
    responses((status = 200, description = "Container started"))
)]
pub async fn start_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("started {id}")))
}

#[utoipa::path(
    post,
    path = "/api/containers/{id}/stop",
    tag = "containers",
    params(("id" = String, Path, description = "Container ID")),
    responses((status = 200, description = "Container stopped"))
)]
pub async fn stop_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("stopped {id}")))
}

#[utoipa::path(
    post,
    path = "/api/containers/{id}/restart",
    tag = "containers",
    params(("id" = String, Path, description = "Container ID")),
    responses((status = 200, description = "Container restarted"))
)]
pub async fn restart_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("restarted {id}")))
}

#[utoipa::path(
    delete,
    path = "/api/containers/{id}",
    tag = "containers",
    params(("id" = String, Path, description = "Container ID")),
    responses((status = 200, description = "Container removed"))
)]
pub async fn remove_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("removed {id}")))
}

#[utoipa::path(
    get,
    path = "/api/containers/{id}/logs",
    tag = "containers",
    params(("id" = String, Path, description = "Container ID")),
    responses((status = 200, description = "Container logs", body = serde_json::Value))
)]
pub async fn container_logs(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(
        serde_json::json!({ "container_id": id, "logs": [] }),
    ))
}

#[utoipa::path(
    get,
    path = "/api/containers/{id}/stats",
    tag = "containers",
    params(("id" = String, Path, description = "Container ID")),
    responses((status = 200, description = "Container stats", body = serde_json::Value))
)]
pub async fn container_stats(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(
        serde_json::json!({ "container_id": id, "stats": {} }),
    ))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/containers", get(list_containers).post(create_container))
        .route(
            "/containers/{id}",
            get(get_container).delete(remove_container),
        )
        .route("/containers/{id}/start", post(start_container))
        .route("/containers/{id}/stop", post(stop_container))
        .route("/containers/{id}/restart", post(restart_container))
        .route("/containers/{id}/logs", get(container_logs))
        .route("/containers/{id}/stats", get(container_stats))
}
