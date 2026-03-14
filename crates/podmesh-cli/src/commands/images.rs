use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum ImageCommand {
    /// List images
    List,
    /// Inspect an image
    Inspect { id: String },
    /// Pull an image
    Pull {
        /// Image reference (e.g. docker.io/library/nginx:latest)
        reference: String,
    },
    /// Remove an image
    Remove { id: String },
}

impl ImageCommand {
    pub async fn run(&self, api: &ApiClient, fmt: &OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let resp: serde_json::Value = api.get("/api/images").await?;
                output::print(&resp, fmt);
            }
            Self::Inspect { id } => {
                let resp: serde_json::Value = api.get(&format!("/api/images/{id}")).await?;
                output::print(&resp, fmt);
            }
            Self::Pull { reference } => {
                let body = serde_json::json!({ "reference": reference });
                let resp: serde_json::Value = api.post("/api/images/pull", &body).await?;
                output::print(&resp, fmt);
            }
            Self::Remove { id } => {
                let resp: serde_json::Value = api.delete(&format!("/api/images/{id}")).await?;
                output::print(&resp, fmt);
            }
        }
        Ok(())
    }
}
