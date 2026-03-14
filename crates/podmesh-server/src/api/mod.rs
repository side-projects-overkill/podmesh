mod containers;
mod events;
mod health;
mod images;
mod networks;
mod nodes;
mod pods;
mod volumes;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "PodMesh API",
        version = "0.1.0",
        description = "Container management control plane for Podman"
    ),
    paths(
        health::health,
        health::readiness,
        nodes::list_nodes,
        nodes::register_node,
        containers::list_containers,
        containers::get_container,
        containers::create_container,
        containers::start_container,
        containers::stop_container,
        containers::restart_container,
        containers::remove_container,
        containers::container_logs,
        containers::container_stats,
        pods::list_pods,
        pods::get_pod,
        pods::create_pod,
        pods::start_pod,
        pods::stop_pod,
        pods::remove_pod,
        images::list_images,
        images::get_image,
        images::pull_image,
        images::remove_image,
        volumes::list_volumes,
        volumes::get_volume,
        volumes::create_volume,
        volumes::remove_volume,
        networks::list_networks,
        networks::get_network,
        networks::create_network,
        networks::remove_network,
        events::list_events,
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "nodes", description = "Node management"),
        (name = "containers", description = "Container operations"),
        (name = "pods", description = "Pod operations"),
        (name = "images", description = "Image operations"),
        (name = "volumes", description = "Volume operations"),
        (name = "networks", description = "Network operations"),
        (name = "events", description = "Event stream"),
    )
)]
struct ApiDoc;

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .merge(health::routes())
        .merge(nodes::routes())
        .merge(containers::routes())
        .merge(pods::routes())
        .merge(images::routes())
        .merge(volumes::routes())
        .merge(networks::routes())
        .merge(events::routes());

    Router::new()
        .nest("/api", api_routes)
        .merge(crate::ws::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
