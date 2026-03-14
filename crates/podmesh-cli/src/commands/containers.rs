use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum ContainerCommand {
    /// List containers
    List {
        #[arg(long, default_value = "false")]
        all: bool,
    },
    /// Inspect a container
    Inspect {
        /// Container ID
        id: String,
    },
    /// Start a container
    Start {
        /// Container ID
        id: String,
    },
    /// Stop a container
    Stop {
        /// Container ID
        id: String,
    },
    /// Restart a container
    Restart {
        /// Container ID
        id: String,
    },
    /// Remove a container
    Remove {
        /// Container ID
        id: String,
    },
    /// View container logs
    Logs {
        /// Container ID
        id: String,
    },
    /// View container stats
    Stats {
        /// Container ID
        id: String,
    },
}

impl ContainerCommand {
    pub async fn run(&self, api: &ApiClient, fmt: &OutputFormat) -> Result<()> {
        match self {
            Self::List { all } => {
                let path = format!("/api/containers?all={all}");
                let resp: serde_json::Value = api.get(&path).await?;
                output::print(&resp, fmt);
            }
            Self::Inspect { id } => {
                let resp: serde_json::Value = api.get(&format!("/api/containers/{id}")).await?;
                output::print(&resp, fmt);
            }
            Self::Start { id } => {
                let resp: serde_json::Value =
                    api.post(&format!("/api/containers/{id}/start"), &serde_json::json!({})).await?;
                output::print(&resp, fmt);
            }
            Self::Stop { id } => {
                let resp: serde_json::Value =
                    api.post(&format!("/api/containers/{id}/stop"), &serde_json::json!({})).await?;
                output::print(&resp, fmt);
            }
            Self::Restart { id } => {
                let resp: serde_json::Value =
                    api.post(&format!("/api/containers/{id}/restart"), &serde_json::json!({})).await?;
                output::print(&resp, fmt);
            }
            Self::Remove { id } => {
                let resp: serde_json::Value = api.delete(&format!("/api/containers/{id}")).await?;
                output::print(&resp, fmt);
            }
            Self::Logs { id } => {
                let resp: serde_json::Value =
                    api.get(&format!("/api/containers/{id}/logs")).await?;
                output::print(&resp, fmt);
            }
            Self::Stats { id } => {
                let resp: serde_json::Value =
                    api.get(&format!("/api/containers/{id}/stats")).await?;
                output::print(&resp, fmt);
            }
        }
        Ok(())
    }
}
