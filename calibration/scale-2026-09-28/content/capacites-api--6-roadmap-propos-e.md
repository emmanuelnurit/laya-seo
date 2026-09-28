6. Roadmap proposée


### Lot 1 — Quick wins (faible effort, valeur immédiate)

| Item | Valeur métier | Effort |
|---|---|---|
| Corriger l'auth du CLI MCP (`--admin=<login>` → exiger un jeton/mot de passe, ou restreindre l'exécution à un contexte de confiance) | Sécurité — élimine le contournement de `AdminApiPermissionListener` | S |
| Tool admin « consulter le profil d'un client donné » (`GET /admin/customers/{id}` + adresses) | Support client assisté par agent (« que sait-on de ce client ? ») | S |
| Tool admin « historique de commandes d'un client donné » (`GET /admin/orders?customer.id=`) | Idem, complète le profil 360° | S |
| Tool admin « appliquer un coupon existant à une commande » (`OrderCoupon`) | Service client (geste commercial rapide) | S |
| Audit log dédié des actions d'agent (chat + MCP) — même mécanisme qu'`AdminLog` | Traçabilité, prérequis à toute extension des droits d'écriture | S-M |

### Lot 2 — Structurant (effort moyen, nécessite conception)

| Item | Valeur métier | Effort |
|---|---|---|
| Tool « changer le statut d'une commande » avec `StagedChangeManager` (même pattern que `update_price`) | Automatisation du suivi de commande, sous supervision humaine | M |
| Tool « traiter une demande de retour » (transition de statut `OrderReturn`) | Fluidifie le SAV | M |
| Extension de `Capability` pour couvrir finement les nouveaux tools (ex. `customer.profile.read` distinct de `orders.read`) | Contrôle d'accès précis par agent | S-M |
| Endpoint API Platform dédié à la création de coupon catalogue (`Coupon` + Processor validant conditions/effets), aligné sur le modèle back-office existant | Débloque la capacité n°1 demandée par le board, proprement (pas de contournement Propel direct) | M-L |

### Lot 3 — Ambitieux (effort élevé, à cadrer avec le board avant de lancer)

| Item | Valeur métier | Effort |
|---|---|---|
| Tool « créer un code promo » construit sur le nouvel endpoint du lot 2, avec staging + validation humaine obligatoire | Réponse complète à la demande initiale du board | L |
| Intégration remboursement (nécessite d'ajouter `refund()` à `PaymentModuleInterface` et de l'implémenter module de paiement par module de paiement) | Automatisation SAV avancée | L, risque financier élevé — à ne pas lancer sans revue sécurité dédiée |
| Export RGPD / suppression de compte en self-service via API (aujourd'hui CLI uniquement) | Conformité, pas piloté par un agent a priori mais comble un vrai trou constaté pendant l'étude | M-L, hors scope agents IA au sens strict |

---

*Sources : recherche exhaustive dans `core/lib/Thelia/Api/Resource/` (121 fichiers), `core/lib/Thelia/Api/{State,Security,EventListener}/`, `core/lib/Thelia/Core/TheliaKernel.php`, `local/modules/CommerceAgents/{Tool,Agent,Mcp,Service,Command}/`. Aucune capacité n'a été supposée : toute affirmation d'absence (ex. « pas de `refund()` ») a été vérifiée par recherche négative dans le code.*
