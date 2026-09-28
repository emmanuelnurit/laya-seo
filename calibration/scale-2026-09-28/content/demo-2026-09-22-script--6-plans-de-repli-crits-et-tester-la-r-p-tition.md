6. Plans de repli — écrits et à tester à la répétition


| Risque | Parade | Test à faire à la répétition |
|---|---|---|
| **Modèle lent ou indisponible en direct** | Les propositions des actes 2, 3, 4 sont **pré-générées et déjà en file** (`commerce-agents:demo-seed`, sans appel LLM). Le direct ne sert **qu'à l'acte 5**. | Vérifier avant ouverture que les 6 `StagedChange` sont bien `pending` en base, sans déclencher `commerce-agents:run-due` en salle. |
| **E-mail de code promo non reçu (acte 1)** | Boîte de démo préparée et testée à l'avance, **jamais un service de mail public**. Second envoi possible en un clic depuis le widget si le premier n'arrive pas. | À la répétition : chronométrer le délai réel de réception, et rejouer volontairement un échec pour vérifier que le bouton de second envoi fonctionne. |
| **T7 (MYO-471, opt-in) REPLI (décision actée)** | L'acte 1 se replie sur les scénarios **1 (bienvenue, F1) et 4 (hésitation, F6)** — ne demandent que le signal émis par MYO-470 (T6, `done`). Le récit tient, l'e-mail en moins : on montre la boutique qui personnalise, pas le canal e-mail. | Décision définitive pour ce soir : seule la version F1+F6 est répétée et jouée en salle. |
| Client perçoit l'opt-in comme du harcèlement | Le geste de fermeture du widget (acte 1, étape 7) prouve le silence en direct. Plafond abaissé à 2 messages/session. | Vérifier à la répétition qu'aucun message ne revient après fermeture, sur 2 pages différentes. |
| Trop de fonctions montrées | 5 actes, pas 7 (voir « écarté volontairement » ci-dessus). | — |
| **Dock de discussion recouvrant « Commander » sur `/panier`** | Défaut connu, **non corrigé pour cette démo** : correctif spécifié ([MYO-479](/MYO/issues/MYO-479)) mais mise en œuvre gelée jusqu'après la démo ([MYO-491](/MYO/issues/MYO-491) → [MYO-493](/MYO/issues/MYO-493)/[MYO-480](/MYO/issues/MYO-480)). La collision est spécifique à `/panier` ; or la **seule visite scriptée de la page panier est l'étape 6 de l'acte 1, coupée par le repli F1+F6**. Ne pas s'y rendre hors script. | À la répétition : confirmer qu'aucun des 5 actes ne passe par `/panier`. Si un acte y passe malgré tout, le signaler sur MYO-491 — cela change l'arbitrage du gel. |
| Base de démo écrasée par erreur | Installation isolée uniquement (`scripts/safe-install.sh`, base `myo468_demo`), jamais `db`/`test`. | Vérifier `DATABASE_NAME` avant ouverture. |

**Aucun scénario non terminé ne part en démo — décision ferme, pas de discussion en salle.**

---
