use std::net::SocketAddr;

use anyhow::Result;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod state;
mod ws;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "podmesh_server=debug,podmesh_client=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::ServerConfig::load()?;
    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port).parse()?;

    let app_state = state::AppState::new(cfg);
    let app = api::router(app_state);

    info!(%addr, "podmesh server starting");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
