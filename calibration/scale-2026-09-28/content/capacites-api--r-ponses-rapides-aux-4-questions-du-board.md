Réponses rapides aux 4 questions du board


| Question | Réponse | Preuve |
|---|---|---|
| Un agent peut-il **créer un code promo** par API ? | **Non**, côté API Platform (`Coupon`). **Partiel** via `OrderCoupon` (application d'un coupon existant à une commande, sans système d'effets). **Non** aujourd'hui via un tool CommerceAgents. | `core/lib/Thelia/Api/Resource/Coupon.php` (3 `Get`, 0 `Post`) ; `core/lib/Thelia/Api/Resource/OrderCoupon.php` (CRUD complet mais pas d'effets structurés) |
| Un agent peut-il **consulter le profil d'un client** ? | **Oui, partiel** : un tool existant (`GetMyProfileTool`) ne donne accès qu'au profil du client connecté. Consulter le profil de *n'importe quel* client nécessite un jeton admin (API Platform), aucun tool ne le fait aujourd'hui. | `local/modules/CommerceAgents/Tool/Shopping/GetMyProfileTool.php` ; `core/lib/Thelia/Api/Resource/Customer.php` |
| Un agent peut-il **voir les stocks** ? | **Oui**, déjà en place. | `local/modules/CommerceAgents/Tool/Admin/GetInventoryTool.php`, `local/modules/CommerceAgents/Tool/Shopping/GetProductDetailsTool.php` ; API : `core/lib/Thelia/Api/Resource/ProductSaleElements.php` (champ `quantity`) |
| Un agent peut-il **voir les commandes** ? | **Oui**, côté client connecté (tool) et côté admin (API, pas de tool dédié à une recherche multi-clients). | `local/modules/CommerceAgents/Tool/Shopping/GetOrdersTool.php` ; API : `core/lib/Thelia/Api/Resource/Order.php` |

Recommandation section 5 en une phrase : **garder l'architecture tools-in-module actuelle, ne pas construire de second serveur MCP** — un serveur MCP stdio existe déjà et proxifie le même `ToolRegistry` ; le sujet réel n'est pas MCP vs API mais le garde-fou d'authentification du CLI MCP (`--admin=<login>` sans mot de passe).

---
