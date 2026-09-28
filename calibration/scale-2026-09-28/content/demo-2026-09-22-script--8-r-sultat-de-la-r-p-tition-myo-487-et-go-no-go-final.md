8. Résultat de la répétition (MYO-487) et go/no-go final


**Répétition jouée intégralement, chronomètre en main, sur `myo468_demo` (jamais `db`)** — rapport complet, preuves et captures sur [MYO-487](/MYO/issues/MYO-487). Pré-vérification pollution [MYO-475](/MYO/issues/MYO-475) : confirmée saine (flag opt-in bien off par défaut sur `myo468_demo`).

**3 écarts trouvés, tous corrigés et revérifiés réels sur `myo468_demo` le 21/09 au soir ([MYO-502](/MYO/issues/MYO-502), 3/3 AC vérifiés) :**

| Acte | Écart initial | État final | Preuve |
|---|---|---|---|
| 1 (F1) | `first_visit` sur panier vide renvoie 204 muet — `WELCOME10` portait une condition ≥25 € ajoutée par le commit opt-in de la veille | **Corrigé — GO ferme** | `WelcomeCouponScenarioResolver`/`findConfiguredCoupons()` ajustés, `POST /agent/chat/proactive-check` sur session anonyme neuve renvoie de nouveau la carte coupon WELCOME10, 66 tests verts |
| 3 | La proposition `review_reply` en file ne répondait à aucun avis ≤2★ | **Corrigé — GO ferme** | `selectReviewTargets()` corrigé, `agent_staged_change` id=10 (`review_reply`, `pending`) cible bien `comment.id=1` / Fauteuil Edgar / ORD000000000017, texte de réponse reconnaît le retard |
| 5 | Aucune clé API LLM configurée sur `myo468_demo` | **Résolu — GO ferme** | Clé Mistral existante sur `db` reportée sur `myo468_demo` ([MYO-503](/MYO/issues/MYO-503)/[MYO-504](/MYO/issues/MYO-504)), appel réel confirmé : `POST /admin/merchant-agent/chat` → SSE `tool_call get_listings` → réponse texte générée par le fournisseur, pas de bannière « non configuré » |

Les replis testés en §6/§7 restent écrits et prêts (F6 seul, coupure de l'acte 3, saut de l'acte 5) mais **ne sont plus requis pour l'ouverture** : les trois correctifs sont livrés et vérifiés avant l'heure prévue.

**Écarts non bloquants, acceptés tels quels pour cette démo** (pas d'action requise) :
- Acte 2 : budget affiché `$0.00` sur cet environnement (aucun run réel) — les chiffres cités en §Acte 2 doivent être revérifiés à la dernière minute contre l'environnement réellement ouvert en salle.
- Acte 4 : texte de la carte de rupture générique (pas de nom produit/vélocité/date affichés) — jouable en paraphrasant le texte à voix haute prévu au script, sans le lire tel quel depuis l'écran.

**Décision finale : GO ferme pour les 5 actes**, tels qu'écrits en §Acte 0 à 5. Les replis de la table §6 restent la conduite à tenir si un imprévu survenait malgré tout en salle (modèle lent, e-mail non reçu), mais aucun n'est plus déclenché par un écart connu.
