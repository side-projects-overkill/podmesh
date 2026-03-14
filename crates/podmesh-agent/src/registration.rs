use std::net::SocketAddr;

use anyhow::Result;
use tracing::info;

pub async fn register(node_name: &str, agent_addr: &SocketAddr, server_url: &str) -> Result<()> {
    let endpoint = format!("http://{agent_addr}");
    let payload = serde_json::json!({
        "name": node_name,
        "endpoint": endpoint,
    });

    let url = format!("{server_url}/api/nodes");

    // Uses a simple TCP connection to register; in production this would use reqwest
    // with TLS and auth tokens. For now we keep dependencies minimal in the agent.
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let body = serde_json::to_vec(&payload)?;

    let req = hyper::Request::builder()
        .method("POST")
        .uri(&url)
        .header("Content-Type", "application/json")
        .body(http_body_util::Full::new(hyper::body::Bytes::from(body)))?;

    let resp = client.request(req).await?;

    info!(status = %resp.status(), "registered with server");
    Ok(())
}
