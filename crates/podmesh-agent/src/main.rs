use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod registration;

#[derive(Parser)]
#[command(name = "podmesh-agent", about = "PodMesh node agent")]
struct Args {
    /// Address to bind the agent API
    #[arg(long, env = "PODMESH_AGENT_ADDR", default_value = "0.0.0.0:8091")]
    addr: SocketAddr,

    /// Podman socket path
    #[arg(
        long,
        env = "PODMESH_PODMAN_SOCKET",
        default_value = "/run/podman/podman.sock"
    )]
    socket: String,

    /// PodMesh server URL for registration
    #[arg(long, env = "PODMESH_SERVER_URL")]
    server_url: Option<String>,

    /// Node name
    #[arg(long, env = "PODMESH_NODE_NAME", default_value = "default")]
    node_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "podmesh_agent=debug,podmesh_client=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    let podman = podmesh_client::PodmanClient::new(&args.socket);
    let state = api::AgentState::new(podman, args.node_name.clone());

    if let Some(ref server_url) = args.server_url {
        info!(server = %server_url, name = %args.node_name, "registering with podmesh server");
        if let Err(e) = registration::register(&args.node_name, &args.addr, server_url).await {
            tracing::warn!("failed to register with server: {e}");
        }
    }

    let app = api::router(state);

    info!(addr = %args.addr, socket = %args.socket, "podmesh agent starting");

    let listener = tokio::net::TcpListener::bind(args.addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
