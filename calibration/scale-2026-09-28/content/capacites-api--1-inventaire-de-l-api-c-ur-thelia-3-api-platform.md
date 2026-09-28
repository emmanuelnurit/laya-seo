1. Inventaire de l'API cœur Thelia 3 (API Platform)


Toutes les ressources vivent dans `core/lib/Thelia/Api/Resource/` (121 fichiers). Sauf mention contraire, aucune ressource ne déclare de Provider/Processor personnalisé : le pipeline générique Propel s'applique (`core/lib/Thelia/Api/Bridge/Propel/State/PropelItemProvider.php`, `PropelCollectionProvider.php`, `PropelPersistProcessor.php`, `PropelRemoveProcessor.php`), injecté automatiquement par `PropelResourceCollectionMetadataFactory`.

### 1.1 Coupons / promos

| Ressource | Fichier | Endpoints | Lecture/Écriture | Groupes | Auth |
|---|---|---|---|---|---|
| `Coupon` | `Api/Resource/Coupon.php` | `GET /front/coupons`, `GET /front/coupons/{id}`, `GET /front/coupons/{code}/by-code` | **Lecture seule** — aucun `Post`/`Put`/`Patch`/`Delete` déclaré | `front:coupon:read[:single]` | `ROLE_CUSTOMER` |
| `CouponI18n` | `Api/Resource/CouponI18n.php` | Aucun (sous-objet imbriqué de `Coupon`) | — | — | — |
| `OrderCoupon` | `Api/Resource/OrderCoupon.php` | `POST/GET(collection)/GET(item)/PUT/PATCH/DELETE /admin/order_coupons[/{id}]` | Écriture complète | `admin:order_coupon:read[:single]` / `write` | `ROLE_ADMIN` + permission admin `ORDER` (CREATE/UPDATE/DELETE selon verbe) |

**Détail important** : `Coupon` définit bien des groupes `GROUP_ADMIN_WRITE` sur des propriétés (`serializedConditions`, `serializedEffects`, `couponCountries`, `couponModules`) mais **aucune opération de l'attribut `#[ApiResource]` ne les référence** — ce sont des groupes orphelins, inatteignables via une route réellement exposée. `Coupon::class` est de plus absent de la table `AdminApiResourcePermissions::CORE_RESOURCES`, confirmant qu'aucun endpoint admin n'a jamais été enregistré pour cette ressource.

`OrderCoupon` permet bien un `POST` en écriture, mais il représente une **ligne de coupon appliquée à une commande existante** (`code`, `type`, `amount`, `serializedConditions`, `isCumulative`) — **pas de champ `serializedEffects`**. Ce n'est pas un outil de création de campagne promotionnelle catalogue, mais un moyen de rejouer/forcer manuellement un coupon sur une commande.

La création réelle d'un coupon catalogue (code + conditions + effets) n'existe que via les contrôleurs back-office classiques (`templates/backOffice/default/Controller/Admin/CouponController.php`, `templates/backOffice/default-twig/src/Controller/CouponController.php`), **hors périmètre API Platform**.

### 1.2 Clients

| Ressource | Fichier | Endpoints | Lecture/Écriture | Auth |
|---|---|---|---|---|
| `Customer` | `Api/Resource/Customer.php` | Admin : `POST/GET(coll)/GET(item)/PUT/PATCH/DELETE /admin/customers[/{id}]`. Front : `POST /front/customers` (inscription, ouvert), `GET/PUT /front/account/customers/{id}` (propriétaire uniquement) | Écriture complète admin ; front self-service limité à son propre profil | Admin : `ROLE_ADMIN` + permission `CUSTOMER`. Front : `ROLE_CUSTOMER` + `object.getId() == user.getId()` |
| `Address` | `Api/Resource/Address.php` | Admin CRUD complet sur `/admin/addresses`. Front : `POST/GET(coll)/GET(item)/PUT/DELETE /front/account/addresses[/{id}]` (propriétaire), `POST /front/guest/addresses` (invité, écriture seule) | Écriture | Front : processor `CustomerAddressProcessor` force le rattachement au client authentifié, le champ `customer` n'est jamais accepté depuis le front |
| `CustomerTitle` / `CustomerTitleI18n` | `Api/Resource/CustomerTitle.php` | Admin CRUD, front lecture seule `/front/customer_titles` | Écriture admin, lecture front publique | Référentiel (civilités), pas de données personnelles |
| `GuestCustomer` | `Api/Resource/GuestCustomer.php` | `POST /front/guest-customers`, `POST /front/guest-customers/{id}/convert` | Écriture, endpoints publics par design (protection déléguée aux processors — rate limit + policy) | `GuestCustomerRegistrationProcessor`, `GuestCustomerConversionProcessor` |

**Profil complet d'un client (coordonnées + adresses + historique de commandes)** : possible **uniquement pour un agent admin** disposant du droit `CUSTOMER` (VIEW), et en agrégeant 2-3 appels (`GET /admin/customers/{id}` embarque `addresses`, `GET /admin/orders?customer.id={id}` pour l'historique — l'historique de commandes n'est **pas** une propriété embarquée du `Customer`, c'est une ressource séparée filtrable). Un client front ne voit que son propre profil.

**RGPD** : aucun endpoint d'export de données personnelles ni de suppression de compte en self-service n'existe côté API. Un vrai mécanisme d'anonymisation existe (`core/lib/Thelia/Domain/Customer/Service/CustomerAnonymizer.php`, `CustomerPurger.php`) mais se déclenche uniquement en CLI (`core/lib/Thelia/Command/CustomerAnonymizeCommand.php`), jamais via API Platform. Notable : `DELETE /admin/customers/{id}` utilise le `PropelRemoveProcessor` générique (suppression physique directe), **sans** passer par le service d'anonymisation ni vérifier l'absence de commandes — comportement plus brutal que la route back-office legacy qui refuse la suppression si le client a des commandes (`core/lib/Thelia/Action/Customer.php:328-337`).

### 1.3 Commandes

| Ressource | Fichier | Endpoints clés | Lecture/Écriture | Auth |
|---|---|---|---|---|
| `Order` | `Api/Resource/Order.php` | Admin CRUD `/admin/orders[/{id}]` (processor `OrderProcessor` sur PUT/PATCH). Front lecture seule : `GET /front/account/orders[/{id}]` (propriétaire), `GET /front/guest-orders/{token}` (token signé, provider `GuestOrderProvider`) | Écriture admin uniquement ; **aucun `POST` front** (pas de création de commande via API) | Admin : `ROLE_ADMIN` + permission `ORDER`. Front : ownership |
| `OrderStatus` | `Api/Resource/OrderStatus.php` | Admin CRUD, front lecture seule | Référentiel des statuts possibles, pas le statut d'une commande donnée | — |
| `OrderProduct` | `Api/Resource/OrderProduct.php` | Admin CRUD, front lecture seule (propriétaire) | — | — |
| `OrderAddress` | `Api/Resource/OrderAddress.php` | Admin uniquement, CRUD complet | Le champ `siret` est explicitement commenté comme figé (à corriger via le workflow back-office, pas en écrivant directement la ressource) | Admin |
| `OrderReturn` | `Api/Resource/OrderReturn.php` | Admin : lecture, `POST` (création), `PATCH`, **`POST /admin/order_returns/{id}/transition`** (changement de statut du retour, processor `OrderReturnTransitionProcessor`), `DELETE`. Front : lecture + `POST /front/account/order_returns` (création demande, `OrderReturnFrontCreateProcessor`) | Écriture | Admin : ACL `UPDATE` pour la transition. Front : propriétaire + `ReturnRequestLimiter` anti-abus |
| `OrderReturnStatus` / `OrderReturnReason` | idem | Lecture seule (référentiels) | — | — |

**Changement de statut d'une commande** : **oui, côté admin uniquement**, via `PUT`/`PATCH /admin/orders/{id}` → `OrderProcessor` (`core/lib/Thelia/Api/State/Processor/OrderProcessor.php`) dispatche `TheliaEvents::ORDER_UPDATE_STATUS` (même événement que le back-office, garde-fou de transition inclus, transaction unique). Aucune route front ne le permet.

**Remboursement** : **non, pas de remboursement bancaire réel**. `POST /admin/order_returns/{id}/transition` vers le statut `received` déclenche un **calcul** de `refundAmount` (`core/lib/Thelia/Action/OrderReturn.php`, via `RefundAmountCalculator`) — une valeur comptable enregistrée sur le retour. `PaymentModuleInterface` (`core/lib/Thelia/Module/PaymentModuleInterface.php`) ne définit aucune méthode `refund()`, et aucun processor n'appelle une passerelle de paiement. Le remboursement effectif reste un acte manuel hors API.

### 1.4 Catalogue & stocks

| Ressource | Fichier | Endpoints | Lecture/Écriture | Notes |
|---|---|---|---|---|
| `Product` | `Api/Resource/Product.php` | Admin CRUD `/admin/products`, front lecture `/front/products` | Écriture admin, lecture front seule | Le tableau imbriqué `productSaleElements` est écrivable via `Product::GROUP_ADMIN_WRITE` |
| `ProductSaleElements` (PSE) | `Api/Resource/ProductSaleElements.php` | Admin CRUD `/admin/product_sale_elements`, front lecture `/front/product_sale_elements` | Écriture admin | Champ `quantity` (le **stock**) exposé en lecture admin+front, écriture admin |
| `ProductPrice` | `Api/Resource/ProductPrice.php` | `GET /admin\|front/product_prices/{pse}/currencies/{currency}` (identifiant composite) | **Lecture seule sur cette ressource** | Le prix se modifie de façon **imbriquée**, via le tableau `productPrices` dans `PUT/PATCH /admin/product_sale_elements` ou `/admin/products` |
| `Category`, `Brand`, `Feature`, `Attribute` | fichiers homonymes | Admin CRUD complet, front lecture seule | Écriture admin | Pas de notion de prix/stock |

**Consulter les stocks** : oui, `quantity` de `ProductSaleElements` est exposé en admin (`GET /admin/product_sale_elements[/{id}]`, JWT admin requis) **et en front, anonymement** (`GET /front/product_sale_elements[/{id}]`), ainsi qu'imbriqué dans `GET /front/products/{id}`.

**Modifier prix/stock** : oui côté admin uniquement — stock via `PUT/PATCH /admin/product_sale_elements/{id}` (champ `quantity`), prix via le tableau imbriqué `productPrices` (pas d'endpoint direct d'écriture sur `ProductPrice`). Aucune écriture possible côté front.

### 1.5 Panier / checkout

| Ressource | Fichier | Endpoints | Lecture/Écriture | Notes |
|---|---|---|---|---|
| `Cart` | `Api/Resource/Cart.php` | Front : `POST /front/carts` (processor `CartCreationProcessor`), `GET /front/carts/{id}`, `GET /front/cart` (panier de session), `PUT/DELETE /front/carts/{id}` | Création/suppression fonctionnelles ; **`PUT` sans effet réel** (aucune propriété ne porte le groupe `front:cart:write`) | Aucun champ `deliveryModule`/`paymentModule` sur le panier |
| `CartItem` | `Api/Resource/CartItem.php` | Front : `POST/GET(coll)/GET(item)/PUT/DELETE /front/cart_items[/{id}]` | **Seul point d'écriture réellement fonctionnel du panier front** | Voter `CartItemVoter` (ownership + `MUTABLE`), une ligne offerte par une promo (`isOffered`) n'est ni modifiable ni supprimable |
| `CartAddress` | `Api/Resource/CartAddress.php` | `GET /admin/cart_addresses[/{id}]` | **Lecture seule, admin uniquement** | Commentaire du code : écrite par le checkout, jamais par l'API |
| `DeliveryModule` | `Api/Resource/DeliveryModule.php` | `GET /front/delivery_modules[/{id}]` | Lecture seule | Consultable, mais aucun endpoint pour **sélectionner** un mode de livraison |
| `PaymentModule` | `Api/Resource/PaymentModule.php` | `GET /front/payment/modules` | Lecture seule | Aucune opération d'écriture, aucun moyen de déclencher un paiement |

**Synthèse** : le panier front est pilotable en écriture pour les **lignes d'articles** (`CartItem`) mais pas pour la livraison, l'adresse ou le paiement — ces derniers restent gérés par un mécanisme de checkout legacy hors API Platform. Aucune ressource ne permet de finaliser une commande ou de déclencher un paiement via l'API.

### 1.6 Contenu & CMS, Config

| Ressource | Endpoints | Lecture/Écriture | Remarque |
|---|---|---|---|
| `Content`, `Folder` | Admin CRUD complet, front lecture seule | Écriture admin | i18n via `ContentI18n`/`FolderI18n` |
| `Config` | Admin CRUD complet, **pas d'endpoint front** | Écriture admin | Porte les variables globales clé/valeur du site (`name`/`value`), y compris les champs `secured`/`hidden` qui restent lisibles/écrivables par API pour un admin autorisé |
| `Module` | Admin : `POST/GET(coll)/GET(item)/PUT/DELETE` (pas de `Patch`) | Écriture admin | Champ `activate` : active/désactive **n'importe quel module** (paiement, livraison…) via un simple `PUT`, sans garde-fou métier supplémentaire ni audit |
| `Lang` | Admin CRUD, front lecture | Écriture admin | `byDefault` change la langue par défaut du site |
| `Currency` | Admin CRUD, front lecture | Écriture admin | `rate`, `byDefault` modifiables |

**Danger identifié** : `Module.activate` et `Config.value` sont des leviers puissants sans garde-fou métier (pas de confirmation, pas d'audit log dédié au-delà d'un refus de permission) — un agent avec un jeton admin ayant les droits `MODULE`/`CONFIG` pourrait désactiver un module de paiement en production via un simple `PUT`.

---
