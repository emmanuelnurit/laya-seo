0.4.2 (MYO-452, MYO-453)


Le module passe de **0.4.1** à **0.4.2**. 3e récidive du même écart (MYO-408/409 pour 0.3.8→0.4.0, MYO-425 pour 0.4.0→0.4.1) : les 8 commits ci-dessous étaient déjà mergés sur `myorg` sans que `Config/module.xml` ni le tag ne suivent — chacun est listé individuellement (leçon [MYO-408](/MYO/issues/MYO-408) : une mention globale du chapitre ne suffit pas, chaque ligne doit apparaître).

- `79a451b` fix(i18n): sort le bandeau de repli et le fil d'ariane du domaine module (MYO-435)
- `103eef1` docs(scripts): précise que check-unpushed-myorg n'avait pas de trou credential (MYO-443)
- `eaffc6a` fix(scripts): fast-forward main sur le HEAD local, pas la ref origin périmée (MYO-443)
- `aa7ff20` feat(scripts): bascule auto-push-myorg vers deploy key SSH, PAT en repli (MYO-443)
- `11ee4ad` feat(bo): bandeau explicite quand la locale BO n'est pas couverte (MYO-435)
- `c506dec` feat(demo): seed thematic agents cart_abandoned_relaunch/welcome_new_customer
- `31fb9bf` fix(i18n): ajoute 33 libellés absents des 4 catalogues du module (MYO-436)
- `48d7f26` fix(a11y): navigation clavier flèches + focus visible sur le sélecteur de modèle (MYO-434)

### Migrations

Aucun fichier `Config/update/0.4.2.sql` : `Config/schema.xml` est identique entre `v0.4.1` et ce lot (`git diff --stat v0.4.1..HEAD -- Config/schema.xml` ne retourne rien, vérifié en lisant les diffs des 8 commits — le seul ajout à surface de persistance, `Service/Demo/ThematicAgentsDemoImporter.php`, réutilise les tables `agent_definition`/`agent_trigger` existantes via `AgentDefinitionManager::save()`, aucune DDL).

### Mécanisme anti-récidive version/tag — MYO-452, MYO-453

- Nouveau script `scripts/check-version-drift.sh`, même modèle que `check-unpushed-myorg.sh` (MYO-372) : filet cron `*/15`, `LOG_DIR` partagé, idempotent, sortie silencieuse quand tout est propre. Compare l'état **local** (aucun accès réseau nécessaire) : le dernier tag `vX.Y.Z` local, le nombre de commits au-dessus (`WARN-VERSION-COMMITS-AHEAD`), et si `Config/module.xml` est resté figé au numéro du tag alors que des commits sont livrés au-dessus (`WARN-VERSION-MODULE-XML-STALE`).
- Détecte et alerte uniquement (log `WARN`) : ne pose jamais de tag, ne modifie jamais `module.xml` — le versionnement reste une décision humaine/agent, même contrat que `check-unpushed-myorg.sh`.
- Branché sur le même mécanisme d'installation cron que MYO-372 : `scripts/install-push-guard.sh` installe désormais aussi cette ligne crontab (marqueur dédié, ne duplique pas les lignes existantes).
