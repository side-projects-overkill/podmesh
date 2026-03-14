use anyhow::Result;
use serde::de::DeserializeOwned;

pub struct ApiClient {
    base_url: String,
    http: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: reqwest::Client::new(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{path}", self.base_url);
        let resp = self.http.get(&url).send().await?;
        let body = resp.json().await?;
        Ok(body)
    }

    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: &serde_json::Value) -> Result<T> {
        let url = format!("{}{path}", self.base_url);
        let resp = self.http.post(&url).json(body).send().await?;
        let result = resp.json().await?;
        Ok(result)
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{path}", self.base_url);
        let resp = self.http.delete(&url).send().await?;
        let body = resp.json().await?;
        Ok(body)
    }
}
