3. Ce que CommerceAgents sait déjà faire


21 tools recensés (pas 25) dans `local/modules/CommerceAgents/Tool/{Admin,Channel,Shopping}/` : **9 Admin, 1 Channel, 11 Shopping**.

### 3.1 Admin (agent marchand / back-office)

| Tool | Fait | Lecture/Écriture | Chemin technique |
|---|---|---|---|
| `get_admin_pages` | Liste les pages du back-office avec URL | Lecture | `TheliaAdminPagesGateway`, table statique + routing Symfony |
| `get_analytics` | CA, nb commandes, top produits, répartition par statut | Lecture | `TheliaAnalyticsGateway` → Propel + SQL brut (`getRevenue()`) |
| `get_campaigns` | Liste coupons + ventes flash | Lecture | `TheliaCampaignGateway` → Propel (`CouponQuery`, `SaleQuery`) |
| `get_inventory` | Variantes + stock, triées, seuil bas | Lecture | `TheliaCatalogAdminGateway` → Propel (`ProductSaleElementsQuery`) |
| `get_listings` | Produits du catalogue, recherche+pagination | Lecture | Propel (`ProductQuery`) |
| `get_pricing` | Prix HT/promo par variante | Lecture | Propel (`ProductQuery`, `ProductSaleElementsQuery`, `ProductPriceQuery`) |
| `open_admin_page` | Redirige le navigateur admin (whitelist `/admin*`) | Navigation | `TheliaSiteUrlValidator` |
| `update_price` | **Propose** un changement de prix (statut `PENDING`) | Écriture en staging, pas d'application directe | `TheliaStagingGateway::stagePriceUpdate()` → `AgentStagedChange` (Propel) |
| `update_stock` | **Propose** un changement de stock (statut `PENDING`) | Écriture en staging | `TheliaStagingGateway::stageStockUpdate()` → `AgentStagedChange` |

### 3.2 Channel

| Tool | Fait | Lecture/Écriture | Chemin technique |
|---|---|---|---|
| `send_to_channel` | Envoie un message sur le(s) canal(aux) configuré(s) pour l'agent (email, webhook) | Écriture (envoi direct ou staging selon `ChannelMode`) | `TheliaChannelGateway` → Propel (`AgentChannelQuery`) + `ChannelConnectorRegistry` |

### 3.3 Shopping (widget front)

| Tool | Fait | Lecture/Écriture | Chemin technique |
|---|---|---|---|
| `add_to_cart` | Ajoute une variante au panier de session | Écriture | `TheliaCartGateway` → `CartFacade::addItem()` (Service Symfony) |
| `get_cart` | Panier courant (items, totaux) | Lecture | `CartFacade::getCartFromSession()` |
| `get_categories` | Catégories visibles + URL | Lecture | Propel (`CategoryQuery`, `ProductCategoryQuery`) |
| `get_my_profile` | Profil du client **connecté uniquement** | Lecture | Propel (`CustomerQuery::findPk()`) |
| `get_orders` | Commandes du client **connecté uniquement** | Lecture | Propel (`OrderQuery::filterByCustomerId()`) |
| `get_policies` | CGV, livraison, retours | Lecture | Propel (`ContentQuery::findPk()`) |
| `get_product_details` | Détail produit + variantes + stock | Lecture | `DataAccessService->resources('/api/front/products', …)` — **appel in-process aux ressources API Platform**, pas de HTTP réel |
| `get_site_pages` | Pages du site avec URL | Lecture | Propel + routing |
| `open_page` | Redirige le visiteur (whitelist domaine) | Navigation | `TheliaSiteUrlValidator` |
| `prepare_checkout` | Résume le panier + lien vers le tunnel (ne déclenche jamais de paiement) | Lecture | `CheckoutUrlProvider` |
| `search_products` | Recherche catalogue (mots-clés, catégorie, option, feature, prix, promo) | Lecture | Mixte : ressources API Platform in-process (`searchProducts`) + Propel direct (`searchVariants`) |
| `suggest_applicable_coupons` | Liste les codes promo **existants**, valides pour le panier courant | Lecture | Propel (`CouponQuery`) + moteur de conditions Thelia — le tool précise explicitement dans son docblock que le LLM ne doit jamais inventer un code hors de cette liste |

### 3.4 Mécanisme ToolRegistry / Capability

- Enregistrement DI : toute classe `ToolInterface` reçoit le tag `commerce_agents.tool` (`local/modules/CommerceAgents/CommerceAgents.php:77-94`), indexée par `ToolRegistry` (`Agent/Tool/ToolRegistry.php`) via `#[TaggedIterator]`.
- **Deux régimes de filtrage** :
  1. Agents configurables autonomes (`AgentRunner`) : filtrage par `Capability` explicite, peuplée depuis les `AgentCapability` liées en base à l'`AgentDefinition` (10 capabilities définies dans `Agent/Tool/Capability.php` : `catalog.read`, `pricing.write`, `inventory.write`, `orders.read`, `analytics.read`, `content.read`, `cart.write`, `checkout.write`, `customer.read`, `channels.send`).
  2. Contextes fixes historiques (chat marchand admin, widget shopping front) : `isAllowed(ToolContext $ctx)` codé en dur par tool (`$ctx->isAdmin` pour Admin, `!$ctx->isAdmin` pour Shopping).
- Un tool refusé n'est **jamais** exposé au LLM (`ToolRegistry::getToolSpecs()` filtre avant envoi).
- `Service/CapabilityCatalog.php` fournit la couche présentation (libellés, groupe read/write, flag `stagedChange` pour `PRICING_WRITE`/`INVENTORY_WRITE`/`CART_WRITE`/`CHECKOUT_WRITE`) utilisée par le wizard de configuration d'agent.

### 3.5 Confirmation des 4 capacités déjà couvertes

- **Créer un coupon** : **non**. `suggest_applicable_coupons` est strictement en lecture (liste les codes existants).
- **Consulter un profil client** : **oui, mais limité au client connecté** (`get_my_profile`). Pas de tool admin pour consulter la fiche d'un client arbitraire.
- **Consulter les stocks** : **oui** (`get_inventory` côté admin, `get_product_details` côté shopping).
- **Consulter les commandes** : **oui côté client connecté** (`get_orders`). Côté admin, seule une vue agrégée existe (`get_analytics`), pas de détail commande par commande.

---
