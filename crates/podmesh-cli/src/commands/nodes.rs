use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum NodeCommand {
    /// List registered nodes
    List,
    /// Register a new node
    Add {
        /// Node name
        #[arg(long)]
        name: String,
        /// Node agent endpoint
        #[arg(long)]
        endpoint: String,
    },
}

impl NodeCommand {
    pub async fn run(&self, api: &ApiClient, fmt: &OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let resp: serde_json::Value = api.get("/api/nodes").await?;
                output::print(&resp, fmt);
            }
            Self::Add { name, endpoint } => {
                let body = serde_json::json!({
                    "name": name,
                    "endpoint": endpoint,
                });
                let resp: serde_json::Value = api.post("/api/nodes", &body).await?;
                output::print(&resp, fmt);
            }
        }
        Ok(())
    }
}
