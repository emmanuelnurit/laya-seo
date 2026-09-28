# CommerceAgents

AI assistants for a Thelia 3 store, powered by LLM agents with tool calling:

- **Shopping assistant** (front office): a chat widget that searches the catalog, explains products, fills the cart, points to the checkout, tracks the customer's orders and answers policy questions.
- **Merchant assistant** (back office): a chat for administrators that reads sales analytics, listings, stock, prices and campaigns, and proposes price or stock changes that a human approves before anything is written.
- **MCP server**: the merchant tools exposed to Claude Desktop, Claude Code or any Model Context Protocol client, with the same gates and approval flow.

Providers: **Mistral (`ministral-3b-latest`, default)**, Anthropic, and any OpenAI-compatible API (OpenAI, OpenRouter…). The chat assistants use one active provider at a time, picked in the **Providers** configuration tab; configurable agents (below) can each be set to any provider that has a configured API key, independently of the shop's active one. Prompts and UI in English, French, Spanish and Italian; the assistants answer in the session language.

Inspired by the [anthropics/commerce-agents](https://github.com/anthropics/commerce-agents) blueprint, ported natively to PHP so it runs wherever Thelia runs.
