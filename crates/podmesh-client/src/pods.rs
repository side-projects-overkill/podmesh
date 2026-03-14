use podmesh_core::error::{PodMeshError, Result};
use serde_json::Value;

use crate::transport::UnixTransport;

const API: &str = "/v5.0.0/libpod";

pub struct PodService<'a> {
    transport: &'a UnixTransport,
}

impl<'a> PodService<'a> {
    pub fn new(transport: &'a UnixTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<Vec<Value>> {
        let (status, body) = self.request("GET", &format!("{API}/pods/json"), None).await?;
        self.parse_response(status, &body)
    }

    pub async fn inspect(&self, id: &str) -> Result<Value> {
        let (status, body) = self
            .request("GET", &format!("{API}/pods/{id}/json"), None)
            .await?;
        self.parse_response(status, &body)
    }

    pub async fn create(&self, spec: &Value) -> Result<Value> {
        let body = serde_json::to_vec(spec)?;
        let (status, resp) = self
            .request("POST", &format!("{API}/pods/create"), Some(&body))
            .await?;
        self.parse_response(status, &resp)
    }

    pub async fn start(&self, id: &str) -> Result<()> {
        let (status, body) = self
            .request("POST", &format!("{API}/pods/{id}/start"), None)
            .await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(())
    }

    pub async fn stop(&self, id: &str) -> Result<()> {
        let (status, body) = self
            .request("POST", &format!("{API}/pods/{id}/stop"), None)
            .await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(())
    }

    pub async fn remove(&self, id: &str, force: bool) -> Result<()> {
        let (status, body) = self
            .request("DELETE", &format!("{API}/pods/{id}?force={force}"), None)
            .await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(())
    }

    async fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
    ) -> Result<(u16, Vec<u8>)> {
        self.transport
            .request(method, path, body)
            .await
            .map_err(|e| PodMeshError::Internal(e.to_string()))
    }

    fn parse_response<T: serde::de::DeserializeOwned>(
        &self,
        status: u16,
        body: &[u8],
    ) -> Result<T> {
        if status >= 300 {
            return Err(self.api_error(status, body));
        }
        serde_json::from_slice(body).map_err(Into::into)
    }

    fn api_error(&self, status: u16, body: &[u8]) -> PodMeshError {
        PodMeshError::PodmanApi {
            status,
            message: String::from_utf8_lossy(body).to_string(),
        }
    }
}
