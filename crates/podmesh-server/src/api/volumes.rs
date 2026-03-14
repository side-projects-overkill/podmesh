use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};

use podmesh_core::models::{ApiResponse, CreateVolumeRequest};

use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/volumes",
    tag = "volumes",
    responses((status = 200, description = "Volume list", body = serde_json::Value))
)]
pub async fn list_volumes(State(_state): State<AppState>) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!([])))
}

#[utoipa::path(
    get,
    path = "/api/volumes/{name}",
    tag = "volumes",
    params(("name" = String, Path, description = "Volume name")),
    responses((status = 200, description = "Volume details", body = serde_json::Value))
)]
pub async fn get_volume(
    State(_state): State<AppState>,
    Path(name): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!({ "name": name })))
}

#[utoipa::path(
    post,
    path = "/api/volumes",
    tag = "volumes",
    request_body = CreateVolumeRequest,
    responses((status = 201, description = "Volume created", body = serde_json::Value))
)]
pub async fn create_volume(
    State(_state): State<AppState>,
    Json(req): Json<CreateVolumeRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(
        serde_json::json!({ "name": req.name, "status": "created" }),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/volumes/{name}",
    tag = "volumes",
    params(("name" = String, Path, description = "Volume name")),
    responses((status = 200, description = "Volume removed"))
)]
pub async fn remove_volume(
    State(_state): State<AppState>,
    Path(name): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("removed {name}")))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/volumes", get(list_volumes).post(create_volume))
        .route(
            "/volumes/{name}",
            get(get_volume).delete(remove_volume),
        )
}
