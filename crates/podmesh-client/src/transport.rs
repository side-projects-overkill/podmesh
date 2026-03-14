use std::path::{Path, PathBuf};

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::UnixStream;
use tracing::debug;

pub struct UnixTransport {
    socket_path: PathBuf,
}

impl UnixTransport {
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
        }
    }

    pub async fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
    ) -> Result<(u16, Vec<u8>), TransportError> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| TransportError::Connection(e.to_string()))?;

        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io)
            .await
            .map_err(|e| TransportError::Handshake(e.to_string()))?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                tracing::error!("connection error: {e}");
            }
        });

        let body_bytes = body.unwrap_or_default();
        let req = Request::builder()
            .method(method)
            .uri(format!("http://localhost{path}"))
            .header("Host", "localhost")
            .header("Content-Type", "application/json")
            .body(Full::new(Bytes::copy_from_slice(body_bytes)))
            .map_err(|e| TransportError::Request(e.to_string()))?;

        debug!(method, path, "podman request");

        let response = sender
            .send_request(req)
            .await
            .map_err(|e| TransportError::Send(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response
            .into_body()
            .collect()
            .await
            .map_err(|e| TransportError::Body(e.to_string()))?
            .to_bytes()
            .to_vec();

        Ok((status, body))
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("failed to connect to socket: {0}")]
    Connection(String),

    #[error("HTTP handshake failed: {0}")]
    Handshake(String),

    #[error("failed to build request: {0}")]
    Request(String),

    #[error("failed to send request: {0}")]
    Send(String),

    #[error("failed to read response body: {0}")]
    Body(String),
}
