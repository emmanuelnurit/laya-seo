3. La commande `commerce-agents:demo-seed`


Fichier : `local/modules/CommerceAgents/Command/DemoSeedCommand.php`. S'appuie sur `thelia:demo:import` (core, lancé par `bin/install --with-demo`) pour le catalogue/clients/commandes de base, puis ajoute ce que celui-ci ne couvre pas :

| Étape | Ce qu'elle fait | AC couvert |
|---|---|---|
| Avis produits | Insère 6 avis (`Comment`, module `Comment`) sur des produits/clients réels du catalogue démo, dont un avis 2★ rattaché dynamiquement à une vraie commande `sent` (retard de livraison mentionné avec la référence de commande) | AC3 |
| Campagne + stock | Fige le produit `PROD006` ("Canapé Nigel") à 4 unités de stock, l'inscrit dans une `Sale` dédiée active (-15 %), et génère 12 commandes réelles sur 4 semaines pour ~6 ventes/semaine | AC4 |
| Agents + propositions | Active 2 presets supplémentaires (`customer_reviews_reply`, `stock_watch_restock`) + un agent "Suivi des commandes bloquées" construit sur mesure (aucun preset existant ne couvre ce cas), puis pré-génère 6 `StagedChange` en attente (3 réponses aux avis, 1 réassort stock, 1 coupon de geste commercial, 1 e-mail d'excuse) via les mêmes services que les tools réels (`StagingGatewayInterface`) — **aucun appel LLM** | AC6 |
| Scénarios coupons opt-in | Ajoute une condition de montant minimum (≥ 25€) sur le coupon d'accueil `WELCOME10` déjà importé par le core, et crée 2 coupons à paliers (`PALIER50` -10% dès 50€, `PALIER90` -15% dès 90€) pour la barre de progression de `CartCouponScenarioResolver` | demande board [MYO-467](/MYO/issues/MYO-467) du 2026-09-21 soir |

AC1/AC2/AC5/AC7 sont déjà couverts nativement :
- **AC1** (install isolée + procédure) : ce document + `scripts/safe-install.sh`.
- **AC2** (catalogue réaliste) : `thelia:demo:import` importe déjà 30 produits avec noms/descriptions/prix/images réalistes (ex. "Chaise Horatio", "Fauteuil Edgar" — jamais "Produit 1"), fichier source `core/lib/Thelia/Command/Import/data/products.csv`.
- **AC5** (commandes bloquées) : `thelia:demo:import` (`OrdersImporter`) date déjà ses commandes sur 12 mois avec un mix de statuts réaliste ; toute base fraîchement importée contient des commandes `not_paid`/`processing` vieilles de plusieurs jours, donc largement > 24 h / 48 h.
- **AC7** (rejouable par script) : `commerce-agents:demo-seed` + `thelia:demo:import`, aucune manipulation manuelle.

### Idempotence

Chaque insertion est gardée (vérification d'existence avant écriture) :
- avis : par couple (client, produit) ;
- campagne : par présence d'une `Sale` exclusivement liée au produit héros ;
- historique de vélocité : par préfixe de token de panier dédié (`demo-seed-velocity-`) ;
- agents : par `preset_code` (presets) ou par titre exact (agent sur mesure) ;
- `StagedChange` : par (`target_type`, `target_id`, `status = pending`) déjà en attente.
