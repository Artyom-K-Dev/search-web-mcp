# Search Web MCP Server

A Model Context Protocol (MCP) server that provides web search capabilities using a SearXNG instance.

## Features

- **Web Search**: Perform privacy-respecting web searches via SearXNG
- **Snippet Extraction**: Returns titles, URLs, and text snippets from search results

## Usage

### Docker (Recommended)

Add this to your `mcp.json` (requires a running SearXNG instance):

```json
{
  "mcpServers": {
    "search-web": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "-e", "SEARXNG_API_URL=http://host.docker.internal:8080",
        "--add-host=host.docker.internal:host-gateway",
        "ghcr.io/artyom-k-dev/search-web-mcp:latest"
      ]
    }
  }
}
```

Or run manually:

```bash
docker run -i --rm \
  -e SEARXNG_API_URL=http://host.docker.internal:8080 \
  --add-host=host.docker.internal:host-gateway \
  ghcr.io/artyom-k-dev/search-web-mcp:latest
```

### Local Development

1. Ensure you have a running SearXNG instance (e.g., via Docker):
   ```bash
   docker-compose up -d
   ```

2. Run the server:
   ```bash
   export SEARXNG_API_URL=http://localhost:8080
   cargo run --release
   ```

## Tools Available

- `search`: Search the web using SearXNG.
  - Arguments:
    - `query`: The search query string

## Configuration

| Environment Variable | Description | Default |
|----------------------|-------------|---------|
| `SEARXNG_API_URL` | Base URL of the SearXNG instance | `http://localhost:8080` |
