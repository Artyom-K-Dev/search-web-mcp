use serde::Deserialize;
use std::env;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub snippet: String,
}

#[derive(Debug, Deserialize)]
pub struct SearxngResponse {
    pub results: Vec<SearchResult>,
}

pub struct SearxngClient {
    base_url: String,
    client: reqwest::Client,
}

impl SearxngClient {
    pub fn new() -> Self {
        let base_url = env::var("SEARXNG_API_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());
        
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        let mut url = Url::parse(&self.base_url)?;
        url.set_path("search");
        
        let params = [
            ("q", query),
            ("format", "json"),
        ];

        let response = self.client
            .get(url)
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("SearXNG request failed: {}", response.status()));
        }

        let searx_resp: SearxngResponse = response.json().await?;
        Ok(searx_resp.results)
    }
}
