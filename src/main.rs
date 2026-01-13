use anyhow::Result;
use rmcp::{ServerHandler, model::ServerInfo, service::ServiceExt, transport};
use rmcp::model::{CallToolRequestParam, CallToolResult, ListToolsResult, PaginatedRequestParam, Implementation, ServerCapabilities, ToolsCapability};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::ErrorData;
use rmcp::handler::server::tool::ToolCallContext;

mod tools;
mod client;

use crate::tools::search::SearchTool;

impl ServerHandler for SearchTool {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "search-web-mcp".into(),
                version: "0.1.0".into(),
            },
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    async fn call_tool(
        &self, 
        params: CallToolRequestParam, 
        req: RequestContext<RoleServer>
    ) -> Result<CallToolResult, ErrorData> {
        let context = ToolCallContext::new(self, params, req);
        self.tool_router.call(context).await
    }

    async fn list_tools(
        &self, 
        _params: Option<PaginatedRequestParam>, 
        _req: RequestContext<RoleServer>
    ) -> Result<ListToolsResult, ErrorData> {
        let tools = self.tool_router.list_all();
        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let tool = SearchTool::new();
    let transport = transport::stdio();
    
    let service = tool.serve(transport).await?;
    service.waiting().await?;
    
    Ok(())
}
