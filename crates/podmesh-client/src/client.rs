use std::path::Path;

use podmesh_core::error::{PodMeshError, Result};
use tracing::debug;

use crate::containers::ContainerService;
use crate::images::ImageService;
use crate::networks::NetworkService;
use crate::pods::PodService;
use crate::transport::UnixTransport;
use crate::volumes::VolumeService;

const DEFAULT_SOCKET: &str = "/run/podman/podman.sock";
const API_VERSION: &str = "v5.0.0";

pub struct PodmanClient {
    transport: UnixTransport,
}

impl PodmanClient {
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        debug!(
            socket = %socket_path.as_ref().display(),
            "creating podman client"
        );
        Self {
            transport: UnixTransport::new(socket_path),
        }
    }

    pub fn default() -> Self {
        Self::new(DEFAULT_SOCKET)
    }

    pub fn containers(&self) -> ContainerService<'_> {
        ContainerService::new(&self.transport)
    }

    pub fn pods(&self) -> PodService<'_> {
        PodService::new(&self.transport)
    }

    pub fn images(&self) -> ImageService<'_> {
        ImageService::new(&self.transport)
    }

    pub fn volumes(&self) -> VolumeService<'_> {
        VolumeService::new(&self.transport)
    }

    pub fn networks(&self) -> NetworkService<'_> {
        NetworkService::new(&self.transport)
    }

    pub async fn ping(&self) -> Result<bool> {
        let (status, _) = self
            .transport
            .request("GET", &format!("/{API_VERSION}/libpod/_ping"), None)
            .await
            .map_err(|e| PodMeshError::Internal(e.to_string()))?;
        Ok(status == 200)
    }

    pub async fn info(&self) -> Result<serde_json::Value> {
        let (status, body) = self
            .transport
            .request("GET", &format!("/{API_VERSION}/libpod/info"), None)
            .await
            .map_err(|e| PodMeshError::Internal(e.to_string()))?;

        if status != 200 {
            return Err(PodMeshError::PodmanApi {
                status,
                message: String::from_utf8_lossy(&body).to_string(),
            });
        }

        serde_json::from_slice(&body).map_err(Into::into)
    }
}
