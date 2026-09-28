0.4.3 (MYO-464, MYO-465)


Le module passe de **0.4.2** à **0.4.3**. 4e récidive du même écart (MYO-408/409 pour 0.3.8→0.4.0, MYO-425 pour 0.4.0→0.4.1, MYO-452/453 pour 0.4.1→0.4.2) : les 4 commits fonctionnels ci-dessous (779 lignes) étaient déjà mergés sur `myorg` sans que `Config/module.xml` ni le tag ne suivent — chacun est listé individuellement (leçon [MYO-408](/MYO/issues/MYO-408) : une mention globale du chapitre ne suffit pas, chaque ligne doit apparaître).

- `2f14c3c` feat(scripts): check-version-drift.sh, mécanisme anti-récidive version/tag (MYO-452/453)
- `b6a52bc` fix(scripts): discrimination fonctionnelle + dedup 24h + routine de notification version-drift (MYO-455)
- `5fbdbb8` fix(scripts): dérive fonctionnelle basée sur le diff net, pas le compte brut de commits (MYO-455)
- `c4a2ed7` feat(scripts): filet check-module-db-drift.sh, 2e maillon version (MYO-463)

### Migrations

Aucun fichier `Config/update/0.4.3.sql` : `Config/schema.xml` est identique entre `v0.4.2` et ce lot (`git diff --stat v0.4.2..HEAD -- Config/schema.xml` ne retourne rien) — ce lot n'ajoute que des scripts d'exploitation (`scripts/check-version-drift.sh`, `scripts/check-module-db-drift.sh`, `scripts/notify-version-drift.sh`) et de la documentation, aucune surface de persistance.

### 2e maillon de la chaîne de version + routine de notification — MYO-454/455/463

- `check-version-drift.sh` distingue désormais les commits *fonctionnels* (chemins de code/ressources livrées) des commits d'outillage (scripts/docs/CHANGELOG), avec dédup 24h par HEAD pour ne pas spammer le log à chaque passage cron, et se base sur le diff **net** entre le tag et HEAD plutôt que sur un compte brut par commit (un commit fonctionnel intégralement revert par un commit postérieur ne compte plus indéfiniment).
- Nouveau script `scripts/check-module-db-drift.sh` (MYO-463) : 2e maillon de la chaîne de version, complémentaire au premier — compare `Config/module.xml` (le code) à `module.version` réellement stocké en base (l'instance installée) via `ddev`, résolu par chemin absolu pour fonctionner en cron nu. Corrige l'incident constaté MYO-459→MYO-461 : le BO affichait `0.3.8` alors que `Config/module.xml` était déjà à `0.4.2` depuis 3 releases, faute de `module:refresh` rejoué après mise à jour du code.
- Nouveau script `scripts/notify-version-drift.sh` (MYO-455/463) : seul maillon des deux filets à disposer de `PAPERCLIP_API_KEY` (exécuté depuis une routine Paperclip, jamais en cron nu) — lit les marqueurs JSON posés par les deux scripts ci-dessus et crée/met à jour, de façon idempotente, un ticket Paperclip par type de dérive, assigné au CTO pour arbitrage.
- **Garde-fou diffstat (MYO-464/465)** : le marqueur JSON de `check-version-drift.sh` embarque désormais le diff net `git diff --stat <latest_tag> HEAD` (fichiers touchés, lignes ajoutées/supprimées), affiché dans le corps du ticket créé par `notify-version-drift.sh`. Corrige le trou qui a permis à 5 arbitrages successifs (MYO-456/457/458/460/462) de clore la dérive « artefact de test » sans voir que 779 lignes réelles montaient dessous — le ticket ne dépend plus de la seule lecture du nombre de commits.
