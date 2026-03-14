use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum NetworkCommand {
    /// List networks
    List,
    /// Inspect a network
    Inspect { name: String },
    /// Create a network
    Create {
        #[arg(long)]
        name: String,
    },
    /// Remove a network
    Remove { name: String },
}

impl NetworkCommand {
    pub async fn run(&self, api: &ApiClient, fmt: &OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let resp: serde_json::Value = api.get("/api/networks").await?;
                output::print(&resp, fmt);
            }
            Self::Inspect { name } => {
                let resp: serde_json::Value = api.get(&format!("/api/networks/{name}")).await?;
                output::print(&resp, fmt);
            }
            Self::Create { name } => {
                let body = serde_json::json!({ "name": name });
                let resp: serde_json::Value = api.post("/api/networks", &body).await?;
                output::print(&resp, fmt);
            }
            Self::Remove { name } => {
                let resp: serde_json::Value =
                    api.delete(&format!("/api/networks/{name}")).await?;
                output::print(&resp, fmt);
            }
        }
        Ok(())
    }
}
