use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum PodCommand {
    /// List pods
    List,
    /// Inspect a pod
    Inspect { id: String },
    /// Create a pod
    Create {
        #[arg(long)]
        name: String,
    },
    /// Start a pod
    Start { id: String },
    /// Stop a pod
    Stop { id: String },
    /// Remove a pod
    Remove { id: String },
}

impl PodCommand {
    pub async fn run(&self, api: &ApiClient, fmt: &OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let resp: serde_json::Value = api.get("/api/pods").await?;
                output::print(&resp, fmt);
            }
            Self::Inspect { id } => {
                let resp: serde_json::Value = api.get(&format!("/api/pods/{id}")).await?;
                output::print(&resp, fmt);
            }
            Self::Create { name } => {
                let body = serde_json::json!({ "name": name });
                let resp: serde_json::Value = api.post("/api/pods", &body).await?;
                output::print(&resp, fmt);
            }
            Self::Start { id } => {
                let resp: serde_json::Value =
                    api.post(&format!("/api/pods/{id}/start"), &serde_json::json!({})).await?;
                output::print(&resp, fmt);
            }
            Self::Stop { id } => {
                let resp: serde_json::Value =
                    api.post(&format!("/api/pods/{id}/stop"), &serde_json::json!({})).await?;
                output::print(&resp, fmt);
            }
            Self::Remove { id } => {
                let resp: serde_json::Value = api.delete(&format!("/api/pods/{id}")).await?;
                output::print(&resp, fmt);
            }
        }
        Ok(())
    }
}
