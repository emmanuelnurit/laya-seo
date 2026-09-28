2. Authentification et permissions


### 2.1 Mécanisme JWT

- Bundle **LexikJWTAuthenticationBundle**, clés lues depuis l'environnement (`config/packages/lexik_jwt_authentication.yaml`).
- Firewalls injectés dynamiquement au boot par `core/lib/Thelia/Core/TheliaKernel.php::loadDefaultSecurityConfig()` (lignes ~766-848) : `frontLogin` (`POST /api/front/login`), `adminLogin` (`POST /api/admin/login`), `api` (`^/api`, garde JWT standard).
- `access_control` : `^/api/admin` → `ROLE_ADMIN` ; `^/api/front/account` → `ROLE_CUSTOMER` ; le reste de `/api/front` (catalogue, produits, catégories…) est **accessible anonymement**.
- Rafraîchissement : `core/lib/Thelia/Controller/Api/RefreshTokenController.php`, routes `/api/admin/token/refresh` et `/api/front/token/refresh`. Refresh token opaque à usage unique (rotation), stocké en cache (pas en base), TTL par défaut **30 jours** (`env(JWT_REFRESH_TOKEN_TTL)`, `core/lib/Thelia/Config/Resources/services/core/security.php:30-35`).
- Rate limiting dédié : `core/lib/Thelia/Core/Security/RateLimiter/ApiLoginRateLimiter.php`, `core/lib/Thelia/Api/EventListener/ApiRateLimitListener.php`.

### 2.2 Clés API

**Aucun mécanisme de clé API n'existe.** Recherche exhaustive (`grep -ril "ApiKey"`) : rien en dehors de fichiers de cache générés par API Platform pour la doc Swagger. La seule authentification possible est le JWT décrit ci-dessus — un agent doit se comporter comme un utilisateur humain (login/mot de passe).

### 2.3 Scopes front vs admin

Deux niveaux cumulatifs :
1. **Firewall/rôle** (grossier) : `ROLE_ADMIN` ou `ROLE_CUSTOMER` selon le préfixe de route.
2. **`AdminApiPermissionListener`** (`core/lib/Thelia/Api/EventListener/AdminApiPermissionListener.php`, priorité 6) : c'est le **vrai** garde-fou admin. Un token `ROLE_ADMIN` seul ne dit rien des droits réels du profil — ce listener résout la ressource métier via `AdminApiResourcePermissions::resolve()` et vérifie le même système de permissions fines que le back-office (VIEW/CREATE/UPDATE/DELETE par ressource). **Refuse par défaut** si aucune permission n'est déclarée pour l'opération. Le superadministrateur court-circuite ce contrôle. Chaque refus est journalisé dans `AdminLog::append()`.

### 2.4 Ressources dangereuses / à garde-fou explicite

- **Écriture admin sans contrainte métier forte** : `Module.activate` (désactivation d'un module de paiement en un `PUT`), `Config.value` (variables système), `DELETE /admin/customers/{id}` (suppression physique sans anonymisation ni vérification de commandes).
- **Données personnelles** : `Customer`, `Address`, `Order` (adresse, email, historique) — exposées en lecture admin dès que le profil a le droit `CUSTOMER`/`ORDER`.
- **Recommandation** : toute action d'écriture admin déclenchée par un agent devrait passer par une confirmation humaine et/ou un audit log dédié — le module CommerceAgents a déjà ce réflexe via `StagedChangeManager` (voir section 3), mais ce n'est **pas** un mécanisme de la plateforme API elle-même : rien n'empêche un appel direct à `PUT /admin/modules/{id}` avec `activate: false` en dehors du module.

### 2.5 Comment CommerceAgents s'authentifie-t-il aujourd'hui ?

**Il n'appelle jamais l'API HTTP de Thelia.** Il tourne en process, par injection directe de services Symfony/Propel (ex. `local/modules/CommerceAgents/Service/Shopping/TheliaCartGateway.php` injecte `CartFacade`, `local/modules/CommerceAgents/Service/Merchant/TheliaCatalogAdminGateway.php` utilise `ProductQuery`/`CategoryQuery` Propel directement). Deux points d'entrée :

1. **Chat back-office** (`local/modules/CommerceAgents/Controller/Admin/MerchantChatController.php`) — protégé par la session admin standard (`SecurityContext`/`AccessManager`), pas de JWT.
2. **Serveur MCP en CLI** (`local/modules/CommerceAgents/Command/McpServeCommand.php`) — **point critique de sécurité** : la commande accepte `--admin=<login>` **sans aucun mot de passe ni jeton** (`AdminQuery::create()->findOneByLogin($login)`, ligne 60), puis construit un `ToolContext(isAdmin: true, adminId: $admin->getId(), ...)` utilisé par tous les outils. Quiconque a un accès shell au serveur obtient donc les capacités de l'admin indiqué, **sans repasser par le firewall JWT ni par `AdminApiPermissionListener`**. Le contrôle d'accès aux tools MCP est un simple booléen (`$ctx->isAdmin && $ctx->adminId !== null`, ex. `Tool/Admin/UpdatePriceTool.php:49-51`) — **pas** le profil de permissions fin du back-office : un admin aux droits restreints obtiendrait potentiellement des capacités plus larges via MCP si l'appelant fournit son login.

**Aucune action de CommerceAgents (chat marchand ou MCP) n'écrit dans `AdminLog`** — pas d'audit log exhaustif des actions d'agent.

---
