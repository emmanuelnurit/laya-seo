Where things live


| Piece | Place |
|---|---|
| Table, seeds, migration | `local/config/schema.xml`, `setup/insert.sql.tpl`, `setup/update/sql/3.1.0.sql` |
| Step contract and core steps | `core/lib/Thelia/Domain/Checkout/Service/Step/` |
| Progression and configuration services | `core/lib/Thelia/Domain/Checkout/Service/` |
| Display form enum | `core/lib/Thelia/Domain/Checkout/Enum/CheckoutDisplayMode.php` |
| Back-office events | `core/lib/Thelia/Core/Event/CheckoutStep/`, handled by `Thelia\Action\CheckoutStep` |
| Back-office screen | `default-twig` theme, `/admin/configuration/checkout-step` |
| Theme rendering, both forms | Flexy theme, `src/Controller/CheckoutController.php` and `checkout-*.html.twig` |
