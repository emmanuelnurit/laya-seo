Surfaces


| Surface | What changed |
| --- | --- |
| `/api/front/sales` | New read-only resource: settings, remaining seconds, `productIds` (batch-preloaded), rewritten URL. The customer list never leaves the back office. Reserved sales are absent for anyone not named, 404 on item access. |
| `/api/front/products`, PSE | Hidden products filtered out (collection and item); reserved price served to entitled customers. |
| Product/PSE loops, `sale` loop | Same filters; the `sale` loop also exposes `URL` and the countdown outputs. Price *sorting* still uses the raw columns. |
| Rewritten URL | `sale` is a rewriting view; the URL is generated when the sale is created. |
| Product view | The view check answers 404 for a hidden product, like an invisible one. |
| Theme sitemap | Hidden products excluded. |
| Back office | Targeting block (public / named customers), shared customer picker (gated by the CUSTOMER right), countdown settings (refused without an end date), reserved badge with the recipient count. |
