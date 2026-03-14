use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};

use podmesh_core::models::{ApiResponse, CreateNetworkRequest};

use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/networks",
    tag = "networks",
    responses((status = 200, description = "Network list", body = serde_json::Value))
)]
pub async fn list_networks(
    State(_state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!([])))
}

#[utoipa::path(
    get,
    path = "/api/networks/{name}",
    tag = "networks",
    params(("name" = String, Path, description = "Network name")),
    responses((status = 200, description = "Network details", body = serde_json::Value))
)]
pub async fn get_network(
    State(_state): State<AppState>,
    Path(name): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!({ "name": name })))
}

#[utoipa::path(
    post,
    path = "/api/networks",
    tag = "networks",
    request_body = CreateNetworkRequest,
    responses((status = 201, description = "Network created", body = serde_json::Value))
)]
pub async fn create_network(
    State(_state): State<AppState>,
    Json(req): Json<CreateNetworkRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(
        serde_json::json!({ "name": req.name, "status": "created" }),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/networks/{name}",
    tag = "networks",
    params(("name" = String, Path, description = "Network name")),
    responses((status = 200, description = "Network removed"))
)]
pub async fn remove_network(
    State(_state): State<AppState>,
    Path(name): Path<String>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::ok(format!("removed {name}")))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/networks", get(list_networks).post(create_network))
        .route(
            "/networks/{name}",
            get(get_network).delete(remove_network),
        )
}
