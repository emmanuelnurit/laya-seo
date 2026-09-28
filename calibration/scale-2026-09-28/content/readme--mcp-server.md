MCP server


```bash
php Thelia commerceagents:mcp:serve --admin=<login> [--locale=fr_FR] [--base-url=https://store.tld] [--debug]
```

- Transport: stdio, newline-delimited JSON-RPC 2.0. Protocol versions 2024-11-05 through 2025-11-25.
- Exposes the merchant tools above except the navigation ones. Writes go through the same proposal flow.
- `--admin` is required: the client acts as that administrator. `--base-url` fixes generated links, since the CLI has no request (defaults to the `url_site` setting).
- No LLM call is made by the store: the MCP client brings its own model.
- The back office documents it, with ready-to-paste Claude Desktop and Claude Code configuration, under **Merchant Agent › MCP server**.

Claude Desktop entry:

```json
{
  "mcpServers": {
    "thelia": {
      "command": "php",
      "args": ["/path/to/thelia/Thelia", "commerceagents:mcp:serve", "--admin=thelia", "--base-url=https://store.tld"]
    }
  }
}
```

Security model: whoever can run the command acts as the given administrator. There is no HTTP transport and no API key; keep it to local or containerized access.
