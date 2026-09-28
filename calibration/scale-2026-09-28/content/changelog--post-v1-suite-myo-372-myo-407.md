Post-V1, suite (MYO-372 → MYO-407)


Le module passe de **0.3.8** à **0.4.0** (pas 0.3.9) : ce lot ajoute `bin/console commerce-agents:rotate-secrets`, une capacité opérationnelle nouvelle exposée aux administrateurs système (rotation de `kernel.secret` sans réécriture manuelle des secrets chiffrés en base), et pas seulement des correctifs — même si le lot en contient aussi (MYO-372, 373, 378, 379, 382, 384, 406, 407). **Aucun changement de schéma** dans ce lot : `Config/update/` s'arrête toujours à `0.3.8.sql`, il n'y a pas de `0.4.0.sql` (voir « Migrations » ci-dessous).

### Garde-fou auto-push étendu aux tags — MYO-372

- `scripts/auto-push-myorg.sh` et `scripts/check-unpushed-myorg.sh` couvrent désormais aussi les tags git, pas seulement les commits sur `myorg` : un tag posé localement (`git tag -a`) sans être poussé sur `origin` est maintenant détecté par le même garde-fou qui protège déjà les commits.

### Fiabilité des runs d'agent — MYO-373, MYO-378, MYO-379, MYO-382

- **MYO-373** — L'outil `report` halluciné par certains modèles (jamais exposé par `ToolRegistry`) provoquait un échec silencieux : `AgentRunner`/`AgentRunsController` distinguent maintenant ce cas et le signalent explicitement au lieu de le laisser disparaître dans les logs.
- **MYO-378** — Les migrations `Config/update/*.sql` sont rendues idempotentes (`CREATE TABLE IF NOT EXISTS`, `ADD COLUMN`/`ADD INDEX IF NOT EXISTS`, `ADD CONSTRAINT` gardé par une procédure `information_schema`), car `CommerceAgents::update()` ne peut pas se fier à `$currentVersion` (fourni par `Thelia\Module\ModuleManagement`) pour savoir quels fichiers ont réellement tourné — voir [MYO-408](/MYO/issues/MYO-408) et la section « Migrations » ci-dessous pour un cas concret. Un test de non-régression dédié (`Tests/Config/UpdateSqlIdempotencyTest.php`, découverte des fichiers par `glob()`) vérifie ces trois garanties sur chaque fichier de `Config/update/`, y compris les futurs.
- **MYO-379** — `ReviewsTabHook` autowirait en dur le modèle `Comment` du module core `Comment` ; sur une installation où ce module n'est pas actif (ou son code absent, cf. [MYO-403](/MYO/issues/MYO-403)), le conteneur d'injection de dépendances refusait de se compiler. Le service est désormais résolu en `nullOnInvalid()`, dégradé proprement (onglet Avis masqué) plutôt que de bloquer tout le DI du module.
- **MYO-382** — `AgentRuntime::runTurn()` ne recouvrait que `ToolException` pendant l'exécution d'un tool ; toute autre exception (`\Throwable` générique) traversait le générateur et faisait échouer le run entier au lieu d'être recouvrée en `tool_result` comme les `ToolException` le sont déjà depuis MYO-373. Recouvrement généralisé, avec stacktrace complète journalisée (nouveau logger injecté dans `AgentRuntime`) puisqu'il ne s'agit pas d'un échec typé attendu.

### Vendor du module désynchronisé — MYO-384

- `vendor/thelia/modules/CommerceAgents` était une seconde copie git indépendante (branche `main`, plusieurs commits en retard sur `local/modules/CommerceAgents`), jamais resynchronisée. `ModuleManagement::updateModules()` scanne les deux répertoires : le `module.xml` vendor périmé pouvait donc réécrire `module.version` avec une valeur obsolète juste après un `module:refresh` pourtant réussi côté local.
- Corrigé en remplaçant `vendor/thelia/modules/CommerceAgents` par un symlink vers `local/modules/CommerceAgents` — une seule copie du code, de `module.xml` et de `Config/update/*.sql` sur le disque, donc plus aucun désaccord possible entre les deux scans. Vérifié en réel dans un projet DDEV isolé : `module:refresh` depuis `module.version = 0.3.1` converge vers `0.3.8`, et deux `module:refresh` consécutifs supplémentaires laissent `module.version` stable à `0.3.8` (voir `README.md` pour la procédure si ce répertoire redevient un jour une copie physique).

### Rotation de `kernel.secret` et audit d'entropie — MYO-406, MYO-407

- Nouvelle commande `bin/console commerce-agents:rotate-secrets --old-secret=<...> --new-secret=<...>` (`--dry-run` disponible) : re-chiffre en base, ligne par ligne, chaque clé API fournisseur et chaque réglage de connecteur de canal (`ChannelSettingsEncryptor`, sodium `secretbox`), une ligne qui ne se déchiffre pas avec l'ancien secret étant journalisée et laissée intacte plutôt que silencieusement perdue. Documente la procédure complète de rotation B6 dans le `README.md`, y compris l'ordre des étapes (rotation en base *avant* d'écraser `APP_SECRET`, sous peine de compiler le kernel contre un secret que la commande ne peut alors plus utiliser pour déchiffrer l'ancien).
- README mis à jour pour marquer **M1** (rate-limit IP sur `/agent/chat*`) et **B1** (plafond de taille de corps de requête) comme en place et prouvés (MYO-406), et **B6** comme prouvé par exécution réelle (MYO-407) : rotation bout-en-bout vérifiée sur worktree + base isolées (jamais la base partagée), et audit d'entropie de l'`APP_SECRET` de cet environnement de dev documenté (jugé non acceptable pour la production, rotation appliquée à titre d'exemple).

### Migrations

Aucun fichier `Config/update/0.4.0.sql` n'est ajouté par ce lot : le diff des 9 commits contre `Config/schema.xml` (`git diff --stat v0.3.8..HEAD -- Config/schema.xml`) ne montre aucun changement de schéma — seuls des fichiers de test et une nouvelle commande console sont ajoutés, et les `Config/update/0.x.sql` existants sont réécrits pour l'idempotence (MYO-378) sans changer leur effet. `Tests/Config/UpdateSqlIdempotencyTest.php` découvre les fichiers par `glob()`, donc un futur `0.4.0.sql` (ou toute version suivante) serait automatiquement couvert par ses trois garanties sans modification du test.
