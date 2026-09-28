Configurable agents


Menu entry **AI agents** (`/admin/module/CommerceAgents/agents`) lets a merchant create standing agents that run outside any chat. Each agent (`agent_definition`) has a name, a free-text role prompt, a model (picked from any provider with a configured API key — defaults to the shop's active provider), a monthly budget in EUR, one or more capabilities, one or more triggers, and one or more channels (mail, Mattermost, Slack).

Creating one starts from either a blank form or one of six presets (`Service/AgentPresets`) that pre-fill the role prompt, model tier, trigger and channel — nothing is persisted until the merchant saves:

| Preset | Role | Default trigger | Channel |
|---|---|---|---|
| Abandoned cart relaunch | Gentle e-mail reminder 24h after a cart is abandoned | `cart_abandoned`, 24h | mail |
| Welcome new customers | Personalised welcome message on sign-up | `new_customer` | mail |
| Daily sales summary | End-of-day report on revenue, orders, best seller | `schedule`, 19:00 | webhook |
| Stock watch & restock | Flags stockouts and low stock, proposes a restock quantity for approval | `low_stock`, threshold 5 | mail |
| Customer reviews replies | Drafts a reply to product reviews for approval, never publishes automatically (requires the `Comment` module) | `schedule`, 10:00 | — |
| Start from scratch | Everything blank | — | — |

Each preset gets its own results view on the agent's page (`Service/SpecialtyPane/`): `DailySalesSummaryResultsPane`, `CartAbandonedResultsPane`, `WelcomeNewCustomerResultsPane`, `StockWatchRestockResultsPane`, `CustomerReviewsReplyResultsPane`, and a `GenericResultsPane` fallback for "start from scratch" agents or a preset the registry can't resolve unambiguously.

Capabilities (`agent_capability`, catalog in `Service/CapabilityCatalog`) are grouped **read** (catalog, content, customers, orders, analytics — no approval needed) and **write** (prices, stock, cart, checkout — every write still lands as a pending `agent_staged_change`, same approval flow as the merchant chat). `channels.send` is its own capability, ungrouped.

The card list shows each agent's last run with a status badge: queued (hourglass), running (spinner), done (green check), failed or skipped for budget (red warning), or "never run yet". **Run now** executes an agent immediately, bypassing its triggers, from the same card.

Each agent also has its own page (`/admin/module/CommerceAgents/agents/{id}`) with a compact preview of its last runs and a link to the full execution history (see **Execution history** below).

### Triggers

Triggers (`agent_trigger`) decide when a configurable agent runs. No trigger ever executes an agent inline in an HTTP request: it only inserts a queued `agent_run`, deduplicated by `dedup_key`. A separate drain step, `commerce-agents:run-due`, turns queued runs into LLM calls.

| Trigger `type` | Fires on | `conditions` JSON |
|---|---|---|
| `event` | One of the whitelisted Thelia events (`event_name`): order paid, order status changed, new customer account | `target_statuses` (status ids, order status change only), `min_amount` |
| `cron` | Its own `cron_expression` (« Planification ») | — |
| `abandoned_cart` | `cron_expression`, then a query for carts with items and no order, older than `delay_hours` | `delay_hours` (default 24) |
| `low_stock` | `cron_expression`, then a query for visible sale elements at or under `threshold` | `threshold` (default 5) |

**Install a real cron** (recommended):

```cron
* * * * * php /path/to/thelia Thelia commerce-agents:run-due >> var/log/commerce-agents-run-due.log 2>&1
```

Without one, a **pseudo-cron fallback** drains the queue from back-office traffic once the real cron has not ticked for 10 minutes, at most once every 5 minutes — enough to keep triggers moving on a store with no crontab access, never a substitute for one at any real volume.

### Execution history

A dedicated screen (`/admin/module/CommerceAgents/agents/runs`) lists every `agent_run`, paginated and filterable by agent and status. Each run's detail page (`/admin/module/CommerceAgents/agents/runs/{id}`) shows its trigger, duration, the tool calls it made (`agent_action_log`) and the staged changes it produced. Each agent's own page (`/admin/module/CommerceAgents/agents/{id}`) shows a compact preview of its last 5 runs with a link into this screen; the card list's "Last run" badge stays the quick-glance summary.

### Guardrails on automated runs

Runs enqueued by a trigger (`event`, `cron`, `abandoned_cart`, `low_stock`) never carry an administrator id — only a manual "Run now" does. `admin_id` is nullable throughout the staging and channel gateways for that reason; it is distinct from `approved_by`, which is always set by the human who approves a proposal. Self-approval is not blocked: a staged change is always proposed by an agent, never by the approving administrator, so there is nothing to self-approve. If a tool call is refused (missing capability) or fails mid-run, the agent is instructed to stop that part of its mission and report exactly which tool was refused and why — never to guess the value the tool would have returned and reuse it in a further write call.
