Data model


Two things changed in the schema: four settings columns on `sale`, and the
`sale_customer` link table. The reserved price is deliberately absent from this
model — see the next section.

```mermaid
erDiagram
    sale ||--o{ sale_customer : "audience (mode 1)"
    sale ||--o{ sale_product : "covered products"
    sale ||--o{ sale_offset_currency : "discount per currency"
    sale ||--|| sale_i18n : "texts + rewritten URL"
    customer ||--o{ sale_customer : ""
    product ||--o{ sale_product : ""

    sale {
        tinyint audience_mode "0 public, 1 named customers, 2 groups (reserved for a later story)"
        bool hide_products "hide covered products from everyone else"
        tinyint countdown_mode "0 never, 1 lead hours before the end, 2 from the opening"
        int countdown_lead_hours "nullable, read only when countdown_mode = 1"
    }
    sale_customer {
        int sale_id FK "ON DELETE CASCADE"
        int customer_id FK "ON DELETE CASCADE"
    }
```

Both foreign keys cascade: deleting the last targeted customer (a GDPR purge,
for instance) silently empties the audience. The back office list therefore
shows the recipient count on every reserved sale and turns it into an alert at
zero. The `(active, audience_mode)` index carries the "is any reserved sale
running" short-circuit that keeps shops without reserved sales on their
historical query plans.

`audience_mode = 2` (customer groups) is modeled but not implemented: customer
groups do not exist in the core yet.
