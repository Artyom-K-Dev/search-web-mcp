# WebSearch MCP Server

A Model Context Protocol (MCP) server written in Rust that allows AI agents to search the web using a private SearXNG instance.

## Features

*   **Privacy-focused**: Uses a self-hosted SearXNG instance.
*   **Simple API**: Exposes a single `search` tool.

## Installation & Configuration

### Prerequisites
*   Rust toolchain installed.
*   Docker & Docker Compose (for running the SearXNG instance).

### Build
```bash
cargo build --release
```

### Start SearXNG
This server requires a running SearXNG instance. A `docker-compose.yml` is provided.

```bash
docker-compose up -d
```
*Note: Ensure port 8080 is available.*

### Configuration (Cursor)
Add the server to your `.cursor/mcp.json` (project specific) or your global MCP settings.

**Copy-pasteable entry:**

```json
{
  "mcpServers": {
    "websearch-mcp": {
      "command": "/workspaces/MachineLearning/WebSearchMCP/target/release/websearch-mcp",
      "args": []
    }
  }
}
```

## Usage

The server exposes a single tool: `search`.

### Tool: `search`

**Arguments:**
*   `query`: String. The search query.

**Example:**
*   Query: `rust programming language`
*   Input: `{"query": "rust programming language"}`

## Development

1.  **Start backend**: `docker-compose up -d`
2.  **Run server**:
    ```bash
    cargo run
    ```

## License

MIT
