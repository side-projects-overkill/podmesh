use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};

use podmesh_core::models::{ApiResponse, PullImageRequest};

use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/images",
    tag = "images",
    responses((status = 200, description = "Image list", body = serde_json::Value))
)]
pub async fn list_images(State(_state): State<AppState>) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!([])))
}

#[utoipa::path(
    get,
    path = "/api/images/{id}",
    tag = "images",
    params(("id" = String, Path, description = "Image ID or name")),
    responses((status = 200, description = "Image details", body = serde_json::Value))
)]
pub async fn get_image(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!({ "id": id })))
}

#[utoipa::path(
    post,
    path = "/api/images/pull",
    tag = "images",
    request_body = PullImageRequest,
    responses((status = 200, description = "Image pulled", body = serde_json::Value))
)]
pub async fn pull_image(
    State(_state): State<AppState>,
    Json(req): Json<PullImageRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(
        serde_json::json!({ "reference": req.reference, "status": "pulling" }),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/images/{id}",
    tag = "images",
    params(("id" = String, Path, description = "Image ID or name")),
    responses((status = 200, description = "Image removed"))
)]
pub async fn remove_image(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("removed {id}")))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/images", get(list_images))
        .route("/images/{id}", get(get_image).delete(remove_image))
        .route("/images/pull", post(pull_image))
}
