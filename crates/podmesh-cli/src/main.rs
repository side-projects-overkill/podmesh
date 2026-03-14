use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod client;
mod commands;
mod output;

#[derive(Parser)]
#[command(
    name = "podmesh",
    about = "PodMesh CLI — manage your Podman infrastructure",
    version
)]
struct Cli {
    /// PodMesh server URL
    #[arg(
        long,
        env = "PODMESH_URL",
        default_value = "http://localhost:8090",
        global = true
    )]
    url: String,

    /// Output format
    #[arg(long, default_value = "table", global = true)]
    output: output::OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage nodes
    #[command(subcommand)]
    Node(commands::NodeCommand),

    /// Manage containers
    #[command(subcommand)]
    Container(commands::ContainerCommand),

    /// Manage pods
    #[command(subcommand)]
    Pod(commands::PodCommand),

    /// Manage images
    #[command(subcommand)]
    Image(commands::ImageCommand),

    /// Manage volumes
    #[command(subcommand)]
    Volume(commands::VolumeCommand),

    /// Manage networks
    #[command(subcommand)]
    Network(commands::NetworkCommand),

    /// Show server health
    Health,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "podmesh_cli=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let cli = Cli::parse();
    let api = client::ApiClient::new(&cli.url);

    match cli.command {
        Commands::Health => {
            let resp: serde_json::Value = api.get("/api/health").await?;
            output::print(&resp, &cli.output);
        }
        Commands::Node(cmd) => cmd.run(&api, &cli.output).await?,
        Commands::Container(cmd) => cmd.run(&api, &cli.output).await?,
        Commands::Pod(cmd) => cmd.run(&api, &cli.output).await?,
        Commands::Image(cmd) => cmd.run(&api, &cli.output).await?,
        Commands::Volume(cmd) => cmd.run(&api, &cli.output).await?,
        Commands::Network(cmd) => cmd.run(&api, &cli.output).await?,
    }

    Ok(())
}
