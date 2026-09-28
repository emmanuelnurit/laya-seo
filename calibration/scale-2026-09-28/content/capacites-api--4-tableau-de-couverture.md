4. Tableau de couverture


| Capacité | Tool existant | API cœur dispo | Effort | Risque |
|---|---|---|---|---|
| Créer un code promo avec conditions/effets | Aucun | Non (API Coupon lecture seule ; création réelle seulement via contrôleurs back-office legacy) | **Élevé** — il faut soit exposer une nouvelle opération API Platform sur `Coupon` avec un Processor dédié (validation des conditions/effets), soit appeler les services métier du `CouponController` legacy depuis un nouveau tool Propel-direct | **Élevé** — création de campagnes promotionnelles = impact financier direct, exige a minima un `StagedChange` avec validation humaine |
| Appliquer un coupon existant à une commande | Aucun | Oui, partiel (`OrderCoupon` CRUD admin) | Faible — nouveau tool Propel/API classique sur une ressource déjà écrivable | Moyen — modifie une commande existante |
| Consulter le profil d'un client donné (pas soi-même) | Aucun | Oui (`GET /admin/customers/{id}`, droit `CUSTOMER`) | Faible — nouveau tool Admin, même pattern que `get_inventory` | Moyen — données personnelles, à restreindre par `Capability` dédiée |
| Consulter l'historique de commandes d'un client donné | Aucun (côté Admin) | Oui (`GET /admin/orders?customer.id=`) | Faible | Moyen — données personnelles |
| Voir les stocks | `get_inventory`, `get_product_details` | Oui | — (déjà fait) | Faible (lecture) |
| Modifier le stock | `update_stock` (staging, approbation requise) | Oui (API admin directe aussi) | — (déjà fait, avec garde-fou) | Faible (déjà « staged ») |
| Voir les commandes du client connecté | `get_orders` | Oui | — (déjà fait) | Faible |
| Changer le statut d'une commande | Aucun | Oui, admin uniquement (`PUT/PATCH /admin/orders/{id}` → `OrderProcessor`, garde-fou de transition métier déjà en place côté cœur) | Moyen — nouveau tool + wrapper `StagedChange` par cohérence avec `update_price`/`update_stock` | Moyen-élevé — annulation/expédition erronée impacte le client, garde-fou de transition existe déjà côté cœur mais pas de confirmation humaine côté agent |
| Déclencher un remboursement | Aucun | **Non** — aucune méthode `refund()` dans `PaymentModuleInterface`, aucun appel à une passerelle de paiement | Élevé — nécessite une intégration paiement propre à chaque module de paiement, hors du périmètre du cœur | Élevé — argent réel, ne pas construire sans revue sécurité dédiée |
| Traiter une demande de retour (transition de statut) | Aucun | Oui (`POST /admin/order_returns/{id}/transition`) | Faible-Moyen — nouveau tool + staging | Moyen |
| Activer/désactiver un module (ex. paiement) | Aucun | Oui (`PUT /admin/modules/{id}`, champ `activate`) | Faible techniquement | **Élevé** — aucun garde-fou métier côté cœur (pas de confirmation, pas d'audit dédié) ; à ne pas exposer sans staging + confirmation humaine forte |
| Modifier une variable de config globale | Aucun | Oui (`PUT/PATCH /admin/configs/{id}`) | Faible | **Élevé** — mêmes réserves que Module |
| Envoyer une notification sur un canal (Slack/email) | `send_to_channel` | — (accès direct services, pas d'API) | — (déjà fait) | Faible-Moyen selon mode (`ChannelMode`) |

---
