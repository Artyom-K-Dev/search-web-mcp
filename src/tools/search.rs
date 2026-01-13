use rmcp::{tool, tool_router, handler::server::tool::{ToolRouter, Parameters}, ErrorData, model::ErrorCode};
use schemars::JsonSchema;
use rmcp::serde::Deserialize;
use std::future::Future;
use std::sync::Arc;
use crate::client::SearxngClient;

#[derive(Clone)]
pub struct SearchTool {
    client: Arc<SearxngClient>,
    pub tool_router: ToolRouter<Self>,
}

#[derive(Deserialize, JsonSchema)]
struct SearchArgs {
    query: String,
}

#[tool_router]
impl SearchTool {
    pub fn new() -> Self {
        Self {
            client: Arc::new(SearxngClient::new()),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Search the web using SearXNG (WebSearch).")]
    pub async fn search(&self, params: Parameters<SearchArgs>) -> Result<String, ErrorData> {
        let query = params.0.query;
        let results = self.client.search(&query).await.map_err(|e| ErrorData { 
            code: ErrorCode::INTERNAL_ERROR, 
            message: e.to_string().into(), 
            data: None 
        })?;
        
        let text_results: Vec<String> = results.iter().map(|r| {
            format!("Title: {}\nLink: {}\nSnippet: {}\n", 
                r.title, r.url, r.snippet)
        }).collect();
        
        let content = text_results.join("\n---\n\n");
        Ok(content)
    }
}
