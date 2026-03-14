use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum VolumeCommand {
    /// List volumes
    List,
    /// Inspect a volume
    Inspect { name: String },
    /// Create a volume
    Create {
        #[arg(long)]
        name: String,
    },
    /// Remove a volume
    Remove { name: String },
}

impl VolumeCommand {
    pub async fn run(&self, api: &ApiClient, fmt: &OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let resp: serde_json::Value = api.get("/api/volumes").await?;
                output::print(&resp, fmt);
            }
            Self::Inspect { name } => {
                let resp: serde_json::Value = api.get(&format!("/api/volumes/{name}")).await?;
                output::print(&resp, fmt);
            }
            Self::Create { name } => {
                let body = serde_json::json!({ "name": name });
                let resp: serde_json::Value = api.post("/api/volumes", &body).await?;
                output::print(&resp, fmt);
            }
            Self::Remove { name } => {
                let resp: serde_json::Value = api.delete(&format!("/api/volumes/{name}")).await?;
                output::print(&resp, fmt);
            }
        }
        Ok(())
    }
}
