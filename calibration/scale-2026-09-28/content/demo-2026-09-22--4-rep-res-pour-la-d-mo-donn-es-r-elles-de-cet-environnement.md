4. Repères pour la démo (données réelles de cet environnement)


- **Acte 2 (avis 2★)** : produit `PROD019` ("Fauteuil Edgar"), client Liam O'Connor, commande `ORD000000000017`. La proposition de réponse est déjà en file (`review_reply`, agent `customer_reviews_replies`).
- **Acte 3 (rupture en pleine promo)** : produit `PROD006` ("Canapé Nigel"), 4 unités restantes, campagne "Semaine promo — Canapé Nigel" active, proposition de réassort (4 → 40 unités) déjà en file (`pse_stock`, agent `stock_watch_restock`).
- **Acte 1 (le Brief)** : 6 propositions en attente au total à l'ouverture de la console (3 `review_reply`, 1 `pse_stock`, 1 `order_coupon`, 1 `customer_email`), tous agents actifs et sans dépendance à un appel LLM en direct.
- Les commandes bloquées précises (référence exacte du paiement en attente et de la commande non expédiée) varient à chaque réinstallation (choisies dynamiquement parmi les commandes du jeu de démo core) — les retrouver via la console "Changements proposés" (propositions `order_coupon` / `customer_email`) ou :
  ```sql
  SELECT ref, created_at FROM `order` o JOIN order_status os ON os.id=o.status_id
  WHERE os.code='not_paid' AND o.created_at < NOW() - INTERVAL 24 HOUR ORDER BY created_at DESC;
  ```
- **Scénarios opt-in visiteur** (si retenus par le board sur [MYO-467](/MYO/issues/MYO-467)) : coupon d'accueil `WELCOME10` (-10%, dès 25€ de panier) déclenche `WelcomeCouponScenarioResolver` à la première visite front ; les paliers `PALIER50` (-10% dès 50€) et `PALIER90` (-15% dès 90€) alimentent la barre de progression de `CartCouponScenarioResolver` après un ajout au panier. Ces scénarios se jouent côté front-office (thème `flexy`), pas dans le BO — tester avec un panier neuf (session anonyme) pour éviter l'effet "déjà commandé" qui désactive le message de bienvenue.
