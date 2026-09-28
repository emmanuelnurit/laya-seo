Configuration


Everything lives in the module configuration page (`/admin/module/configure?module_code=CommerceAgents`). The top of the page is always visible: a **Mistral hero card** (API key, default model, connection test, key-configured status badge) and a **spend follow-up card** (spend this month, budget bar, AI calls this month). The gear icon opens a **General settings** panel with five tabs:

| Tab | Settings |
|---|---|
| Providers | One API key, base URL and model per provider; the active provider; a connection test against the configured model |
| Models | Catalog of models with prices and context window (prices are per-provider currency — EUR for Mistral, USD for Anthropic and OpenAI-compatible; see below). Bundled from `Config/models.php`, refreshable from the provider API, editable by hand. Only enabled models are offered in the provider tab |
| Assistants | Assistant name, front chat on/off, cart / checkout / orders capabilities, daily message limit per visitor session, ids of the contents used as policies |
| Budget | Monthly budget for both assistants together, warning threshold, optional hard pause when the budget is reached |
| Usage | Calls, tokens and cost today, this month and per model over 30 days |

Settings are stored as module config values (`CommerceAgents::getConfigValue()`); API keys never reach the browser after being saved.

Mistral publishes its tariffs in EUR; `Service/ModelCatalog` converts them to USD (rate maintained in `Config/models.php`) so the shopping/merchant assistants' budget, spend and usage figures stay in one currency (USD) internally, regardless of which provider is active. The **Models** tab still shows each model's price in the currency its provider bills in (EUR for Mistral, USD for Anthropic and OpenAI-compatible). Configurable agents (below) are the one place a merchant enters a budget directly in EUR — it is converted to USD on save for the same internal accounting.
