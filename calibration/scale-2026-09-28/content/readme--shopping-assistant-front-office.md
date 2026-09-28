Shopping assistant (front office)


- Injected on every page through the Flexy theme hook `layout.body.bottom`; no template change needed. Alpine.js and the CSS ship with the module, without a build step.
- Endpoint `POST /agent/chat`, Server-Sent Events response. The widget keeps the conversation across pages in `sessionStorage` and shows a cart reminder with a checkout call to action.
- Works for guests. Order history and profile require a logged-in customer and never expose another customer's data.

Tools available to the model:

| Tool | Role |
|---|---|
| `search_products` | keyword search with category and price filters, taxed prices in the session currency |
| `get_product_details` | description, variants, availability, prices |
| `add_to_cart`, `get_cart` | current session cart only |
| `prepare_checkout` | cart summary and checkout link; never triggers a payment |
| `get_orders`, `get_my_profile` | logged-in customer only |
| `get_policies` | terms, shipping and return contents |
| `get_site_pages`, `open_page` | find store pages and navigate the visitor's browser to them |

### Proactive assistant

Alongside the reactive chat, the widget's JS (`chat-widget.js`) watches visitor behaviour in `sessionStorage` and calls `POST /agent/chat/proactive-check` to surface an unprompted bubble for one of seven scenarios, each a `ProactiveScenarioResolverInterface` implementation registered under `commerce_agents.proactive_scenario_resolver`:

| Scenario | Signal | What it offers |
|---|---|---|
| Hesitation | repeated cart-drawer opens or idle time on a product | help finding what the visitor is looking for |
| Abandoned cart | cart idle in session | a nudge back to checkout |
| Cross-sell promo | a promoted product viewed | a related item on promotion |
| Stock rupture | a low/out-of-stock product viewed | in-stock alternatives |
| Eligible coupon | an item added to cart that matches an active coupon | the matching coupon |
| Coupon tier (palier) | same as above, ≥2 amount-tiered coupons configured | a progress bar to the next discount tier |
| Welcome coupon | first visit of a session, a coupon that looks like a welcome offer | that coupon |

`Service/ProactiveGuard` gates every bubble, in order: was it dismissed this session → minimum delay since the last one (90s) → not the same scenario repeated → monthly budget not exhausted. `POST /agent/chat/proactive-apply-coupon` and `POST /agent/chat/proactive-dismiss` handle the visitor's response. State lives on `agent_conversation` (`proactive_*` columns), never exposed to another visitor.
