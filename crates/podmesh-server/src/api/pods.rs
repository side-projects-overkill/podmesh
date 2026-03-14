use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

use podmesh_core::models::{ApiResponse, CreatePodRequest};

use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/pods",
    tag = "pods",
    responses((status = 200, description = "Pod list", body = serde_json::Value))
)]
pub async fn list_pods(State(_state): State<AppState>) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!([])))
}

#[utoipa::path(
    get,
    path = "/api/pods/{id}",
    tag = "pods",
    params(("id" = String, Path, description = "Pod ID")),
    responses((status = 200, description = "Pod details", body = serde_json::Value))
)]
pub async fn get_pod(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!({ "id": id })))
}

#[utoipa::path(
    post,
    path = "/api/pods",
    tag = "pods",
    request_body = CreatePodRequest,
    responses((status = 201, description = "Pod created", body = serde_json::Value))
)]
pub async fn create_pod(
    State(_state): State<AppState>,
    Json(req): Json<CreatePodRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(
        serde_json::json!({ "name": req.name, "status": "created" }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/pods/{id}/start",
    tag = "pods",
    params(("id" = String, Path, description = "Pod ID")),
    responses((status = 200, description = "Pod started"))
)]
pub async fn start_pod(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("started {id}")))
}

#[utoipa::path(
    post,
    path = "/api/pods/{id}/stop",
    tag = "pods",
    params(("id" = String, Path, description = "Pod ID")),
    responses((status = 200, description = "Pod stopped"))
)]
pub async fn stop_pod(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("stopped {id}")))
}

#[utoipa::path(
    delete,
    path = "/api/pods/{id}",
    tag = "pods",
    params(("id" = String, Path, description = "Pod ID")),
    responses((status = 200, description = "Pod removed"))
)]
pub async fn remove_pod(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("removed {id}")))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/pods", get(list_pods).post(create_pod))
        .route("/pods/{id}", get(get_pod).delete(remove_pod))
        .route("/pods/{id}/start", post(start_pod))
        .route("/pods/{id}/stop", post(stop_pod))
}
