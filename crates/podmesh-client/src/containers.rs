use podmesh_core::error::{PodMeshError, Result};
use serde_json::Value;

use crate::transport::UnixTransport;

const API: &str = "/v5.0.0/libpod";

pub struct ContainerService<'a> {
    transport: &'a UnixTransport,
}

impl<'a> ContainerService<'a> {
    pub fn new(transport: &'a UnixTransport) -> Self {
        Self { transport }
    }

    pub async fn list(&self, all: bool) -> Result<Vec<Value>> {
        let path = format!("{API}/containers/json?all={all}");
        let (status, body) = self.request("GET", &path, None).await?;
        self.parse_response(status, &body)
    }

    pub async fn inspect(&self, id: &str) -> Result<Value> {
        let path = format!("{API}/containers/{id}/json");
        let (status, body) = self.request("GET", &path, None).await?;
        self.parse_response(status, &body)
    }

    pub async fn create(&self, spec: &Value, name: Option<&str>) -> Result<Value> {
        let body = serde_json::to_vec(spec)?;
        let name_query = name
            .map(|n| format!("?name={n}"))
            .unwrap_or_default();
        let path = format!("{API}/containers/create{name_query}");
        let (status, resp) = self.request("POST", &path, Some(&body)).await?;
        self.parse_response(status, &resp)
    }

    pub async fn start(&self, id: &str) -> Result<()> {
        let path = format!("{API}/containers/{id}/start");
        let (status, body) = self.request("POST", &path, None).await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(())
    }

    pub async fn stop(&self, id: &str, timeout: Option<u32>) -> Result<()> {
        let t = timeout.unwrap_or(10);
        let path = format!("{API}/containers/{id}/stop?timeout={t}");
        let (status, body) = self.request("POST", &path, None).await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(())
    }

    pub async fn restart(&self, id: &str, timeout: Option<u32>) -> Result<()> {
        let t = timeout.unwrap_or(10);
        let path = format!("{API}/containers/{id}/restart?timeout={t}");
        let (status, body) = self.request("POST", &path, None).await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(())
    }

    pub async fn remove(&self, id: &str, force: bool) -> Result<()> {
        let path = format!("{API}/containers/{id}?force={force}");
        let (status, body) = self.request("DELETE", &path, None).await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(())
    }

    pub async fn logs(&self, id: &str, tail: Option<u32>) -> Result<Vec<u8>> {
        let lines = tail.unwrap_or(100);
        let path =
            format!("{API}/containers/{id}/logs?stdout=true&stderr=true&tail={lines}");
        let (status, body) = self.request("GET", &path, None).await?;
        if status >= 300 {
            return Err(self.api_error(status, &body));
        }
        Ok(body)
    }

    pub async fn stats(&self, id: &str) -> Result<Value> {
        let path = format!("{API}/containers/{id}/stats?stream=false");
        let (status, body) = self.request("GET", &path, None).await?;
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
