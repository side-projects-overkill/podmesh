use podmesh_core::error::{PodMeshError, Result};
use serde_json::Value;

use crate::transport::UnixTransport;

const API: &str = "/v5.0.0/libpod";

pub struct ImageService<'a> {
    transport: &'a UnixTransport,
}

impl<'a> ImageService<'a> {
    pub fn new(transport: &'a UnixTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<Vec<Value>> {
        let (status, body) = self
            .request("GET", &format!("{API}/images/json"), None)
            .await?;
        self.parse_response(status, &body)
    }

    pub async fn inspect(&self, id: &str) -> Result<Value> {
        let (status, body) = self
            .request("GET", &format!("{API}/images/{id}/json"), None)
            .await?;
        self.parse_response(status, &body)
    }

    pub async fn pull(&self, reference: &str) -> Result<Value> {
        let encoded = urlencoding::encode(reference);
        let (status, body) = self
            .request(
                "POST",
                &format!("{API}/images/pull?reference={encoded}"),
                None,
            )
            .await?;
        self.parse_response(status, &body)
    }

    pub async fn remove(&self, id: &str, force: bool) -> Result<Value> {
        let (status, body) = self
            .request(
                "DELETE",
                &format!("{API}/images/{id}?force={force}"),
                None,
            )
            .await?;
        self.parse_response(status, &body)
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
