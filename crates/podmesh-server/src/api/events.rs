use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use podmesh_core::models::ApiResponse;

use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/events",
    tag = "events",
    responses((status = 200, description = "Recent events", body = serde_json::Value))
)]
pub async fn list_events(
    State(_state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::ok(serde_json::json!([])))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/events", get(list_events))
}
