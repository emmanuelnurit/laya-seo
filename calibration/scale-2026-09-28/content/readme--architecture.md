Architecture


```
Agent/          AgentRuntime (LLM loop + tool calls, streaming events), LLM clients, ToolRegistry, ToolContext, Proactive/ resolver interface + 2 resolvers
Tool/           Shopping/*, Admin/* and Channel/* tools; each depends on a gateway interface
Channel/        ChannelConnectorInterface, registry, Connector/ (mail, Mattermost, Slack)
Service/        Thelia gateways (Propel, DataAccessService, events), config, budget, cost, conversations, streaming,
                 configurable agents (AgentDefinitionManager, presets, capability/trigger catalogs, run queue),
                 SpecialtyPane/ (per-preset results views + registry), channel connector config service,
                 proactive guard/session and the remaining scenario resolvers (Proactive/, plus two at the Service/ root)
StagedChange/   proposal manager, appliers, repository contract
Mcp/            JSON-RPC framing, MCP server, stdio transport, tool catalog
Controller/     Front chat + proactive endpoints; admin chat, agents CRUD, run history, proposals console, channels config, MCP page, configuration actions
Hook/           Back-office hooks (menu, popup chat, configuration page) and the front theme hook
Command/        commerceagents:mcp:serve, commerce-agents:run-due
Config/         module.xml, Propel schema, SQL, bundled model catalog
templates/      Twig for the back office (default-twig) and the front widget assets
Tests/          PHPUnit unit tests with fake gateways
docs/design/    Committed HTML mockups (e.g. the dashboard suggestions popup)
```

Key points:

- **One tool catalog, three consumers.** The chat runtimes and the MCP server all call `ToolRegistry::getToolSpecs()` and `ToolRegistry::execute()`. A tool is exposed to the model only if `isAllowed(ToolContext)` says so, and its arguments are validated against its JSON schema before execution.
- **ToolContext** carries who is talking (admin id or customer id), the conversation, the locale and the currency. Tools never read the session themselves.
- **Gateways** isolate Thelia access behind interfaces (`Tool/*/Gateway/*Interface.php`), which keeps tools unit-testable with fakes.
- **Streaming**: `ChatStreamer` freezes a copy of the session before opening the SSE stream, so mid-stream readers (tax engine, security context) never restart the PHP session after headers are sent.
- **LLM clients** normalize Anthropic, Mistral and OpenAI-style tool calling to one `LlmEvent` stream; `HistorySanitizer` keeps tool call / result pairs consistent across providers.
- **Costs**: every assistant message stores tokens, model and cost; `BudgetGuard` blocks new turns when the monthly budget is exhausted and blocking is enabled.

### Database

| Table | Content |
|---|---|
| `agent_conversation` | type (`shopping` / `merchant`), customer or admin id, session reference, locale, proactive bubble state |
| `agent_message` | role, content, tool calls, tokens in/out, model, cost |
| `agent_staged_change` | conversation or agent definition, admin, target type and id, before/after payloads, status, approver, error |
| `agent_model` | provider, model id, name, prices, currency, context window, enabled, source |
| `agent_definition` | title, role prompt, model, monthly budget, auto-apply, enabled — a configurable agent |
| `agent_capability` | agent definition, capability code (read/write) |
| `agent_trigger` | agent definition, type (`event`/`cron`/`abandoned_cart`/`low_stock`), cron expression, `conditions` JSON |
| `agent_run` | agent definition, trigger, status (`queued`/`running`/`done`/`failed`/`skipped_budget`), `dedup_key`, timestamps |
| `agent_channel` | agent definition, connector code, encrypted settings, mode (`draft`/`direct`), enabled |
| `agent_outbound_message` | agent run, agent definition, channel, recipient, status, business reference, body excerpt, error, sent-at — one row per outbound message, feeding the execution history and the specialty result panes |

### Schema migrations

`Config/update/*.sql` are applied by `CommerceAgents::update()` when `bin/console module:refresh` (or module activation) detects a version bump. **`module.version` in the database is not sufficient proof that the schema is actually up to date** — do not use it as evidence in an incident, a deploy check, or a CI gate without also checking `information_schema`. Two independent ways it can go stale, both observed in production (see [MYO-378](../../../../MYO/issues/MYO-378)):

1. **Implicit commit inside `ModuleManagement::updateModule()`.** That core Thelia method calls `$module->setVersion($newVersion)->save($con)` *before* `$instance->update($currentVersion, $newVersion, $con)`, all inside one Propel transaction. MySQL/MariaDB implicitly commit the current transaction as soon as a DDL statement (`CREATE TABLE`, `ALTER TABLE`) runs — so the first DDL statement in `update()` permanently commits the version bump too, regardless of whether a later statement in the same run then fails. `$con->rollBack()` in the `catch` block cannot undo it. Mitigation here: every `Config/update/*.sql` file is written to be idempotent/replayable (`CREATE TABLE IF NOT EXISTS`, `ADD COLUMN IF NOT EXISTS`, `ADD INDEX IF NOT EXISTS`, and a guarded `information_schema` check for foreign keys, which have no `IF NOT EXISTS` form in MariaDB). `update()` replays *all* of them on every call instead of trusting `$currentVersion` to skip files, so the schema converges to what the current code describes even if the recorded version is wrong or several versions behind.
2. **Stale vendor copy — fixed, see below.** `ModuleManagement::updateModules()` scans both `local/modules/` and `vendor/thelia/modules/` for `module.xml`, and this module used to be tracked as two *independent* copies (two separate `.git` checkouts of the same repo, `local/modules/CommerceAgents` on `myorg`, `vendor/thelia/modules/CommerceAgents` stuck on `main`, several commits behind). If the vendor copy fell behind — an older declared `<version>`, missing `Config/update/*.sql` files — a `module:refresh` run could process the vendor copy *after* the local one and write that older version number back into `module.version`, even though the schema and the local copy were already current. This was a distinct, real bug from #1 above (not fixed by SQL idempotency); found live while verifying that fix, tracked and resolved in [MYO-384](../../../../MYO/issues/MYO-384).

If you ever need to *prove* the schema matches the code (not just trust the version column), compare `information_schema.columns` / `information_schema.tables` for the tables in the table above against what a fresh replay of every `Config/update/*.sql` file produces.

### Keeping `vendor/thelia/modules/CommerceAgents` in sync (MYO-384)

Nothing in this repo (`composer.json`, `bin/install`, DDEV provisioning) actually populates or refreshes `vendor/thelia/modules/CommerceAgents` — it isn't a composer dependency (no `path` repository declares it) and it is entirely `.gitignore`d, like `local/modules/`. On this shared machine it existed only because someone had, at some point, manually cloned the module a second time into `vendor/thelia/modules/`, and nothing ever kept that second clone in sync with `local/modules/CommerceAgents` afterwards — which is exactly how it silently drifted to a `module.xml` several versions behind.

Since there is no install-time process that recreates this directory, the fix is not a "resync as part of a release checklist" step (there is no such step to hook into, and adding a manual one would just recreate the same class of drift the next time someone forgets it). Instead, **`vendor/thelia/modules/CommerceAgents` is now a symlink to `local/modules/CommerceAgents`**, so there is exactly one copy of the module's code, `module.xml`, and `Config/update/*.sql` on disk — the vendor-side scan in `ModuleManagement::updateModules()` reads the exact same `module.xml` as the local-side scan, so it can never disagree with it again. This was verified with `Symfony\Component\Finder\Finder` (which the core scan uses) actually following the symlink, and with a real `module:refresh` run in an isolated DDEV project: starting from `module.version = 0.3.1`, a single `module:refresh` correctly catches the schema up to `0.3.8`, and two further consecutive `module:refresh` runs leave `module.version` at `0.3.8` (previously, the second — vendor — scan would silently write it back down).

If this directory ever shows up again as a real directory instead of a symlink (e.g. after a manual `git clone` for some ad hoc reason), replace it with `ln -s ../../../local/modules/CommerceAgents vendor/thelia/modules/CommerceAgents` from the project root — do not leave a second physical copy in place.

### Security

- Tool calls are validated server-side: schema, then context gate. The model never sees data outside the caller's scope.
- Merchant writes always go through human approval; the console needs the update permission on the module.
- Provider keys stay server-side; the front widget only talks to `/agent/chat`.
- Message length is capped, visitors get a daily message limit, and the monthly budget caps spend.
