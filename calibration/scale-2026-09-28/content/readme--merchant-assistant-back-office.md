Merchant assistant (back office)


Menu entry **Merchant Agent** (`/admin/merchant-agent`), plus a popup chat available on every admin screen. Restricted to administrators with the `commerceagents` module permission (view to chat, update to approve proposals).

| Tool | Role |
|---|---|
| `get_analytics` | revenue, order count, top sellers, status breakdown over a sliding period |
| `get_listings` | products with visibility, position, category and back-office URL |
| `get_inventory` | variants sorted by stock, optional low-stock threshold |
| `get_pricing` | prices and promo per variant |
| `get_campaigns` | coupons and catalog sales |
| `get_admin_pages`, `open_admin_page` | find back-office screens and navigate to them |
| `update_price`, `update_stock` | **create a pending proposal**, nothing is written |
| `send_email_to_customer` | **create a pending proposal** to e-mail a customer, resolved by `customer_id` only — the model never supplies an address, so it can never reach a hallucinated or copied one |

### Proposed changes

Every write lands in `agent_staged_change` with the before/after payload and the proposing administrator. The **Proposed changes** console (`/admin/merchant-agent/changes`) lists them; approving applies the change through the standard Thelia event (`PRODUCT_UPDATE_PRODUCT_SALE_ELEMENT`), so caches, hooks and listeners run as if the change came from the product form. Rejections and application errors are kept for audit.

Adding a new kind of write: one tool that stages a change, plus one `ChangeApplierInterface` implementation for its `target_type`. Both are auto-registered.

`GET /admin/merchant-agent/changes/agent/{agentDefinitionId}/suggestions` returns pending changes for one configurable agent as pre-formatted JSON (`{"suggestions": [{id, targetType, icon, accent, title, body, ctaLabel, createdAt}, ...]}`) — no business logic ships to the client. This is the data contract the back-office dashboard's "AI agents" card and suggestions popup (`default-twig` theme, outside this module) consume. `POST /admin/merchant-agent/changes/{id}/approve|reject` accept the same AJAX call the popup makes, returning HTTP 409 (`already_handled`) for a double-click and 422 for a real error, so the popup can stay non-blocking.
