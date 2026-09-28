6. Copie hors-machine du dépôt site (MYO-428/433)


Les bundles du §1 (`scripts/backup-bundles.sh`) sont un filet local : ils
vivent sur `/home/enurit/backups/thelia3-bundles`, **le même disque
(`/dev/sdd`) que le projet lui-même** (constat MYO-427). Une perte de ce
disque emporte donc le dépôt de travail ET tous ses bundles de secours en
même temps — trou identifié par MYO-427/428, fermé pour le dépôt **site**
par ce ticket (le dépôt module CommerceAgents a son garde-fou équivalent
depuis MYO-372/426, `origin` public `myorg`+`main`, non concerné ici).

### 6.1 Où est la copie

- Dépôt GitHub **privé** `emmanuelnurit/thelia3-site` (créé via l'API
  GitHub pour MYO-433, `"private": true` vérifié à la création).
- Remote dédié dans le dépôt site : `backup` (jamais `origin` — `origin`
  reste `https://github.com/thelia/thelia.git`, l'upstream public en
  lecture seule, MYO-387, inchangé).
- Une seule branche synchronisée : `twig` → `backup/twig`. Pas de
  `--all`/`--mirror`, pas d'autre branche (donc `main` local et
  `backup/twig-avant-maj-20260914`, tous deux des vestiges, n'y apparaissent
  pas).

### 6.2 Mécanisme (même modèle que MYO-372/426, remote et repo différents)

- **Credentials — deploy key SSH dédiée, pas un token HTTPS.** Root cause
  d'un premier essai en HTTPS (via `$GITHUB_TOKEN`, même fallback que
  `auto-push-myorg.sh`) : le dépôt site suit `.github/workflows/*.yml`
  (héritage `thelia/thelia`), et GitHub rejette tout push HTTPS qui touche
  ce chemin si le PAT n'a pas le scope OAuth `workflow` — restriction propre
  à l'auth PAT/OAuth App. Décision CEO (MYO-433) : remote `backup` en SSH
  (`git@github.com:emmanuelnurit/thelia3-site.git`), authentifié par une
  deploy key ed25519 dédiée à ce seul dépôt (écriture, portée repo-only,
  créée via `POST /repos/emmanuelnurit/thelia3-site/keys`). Une deploy key
  SSH n'est pas soumise au contrôle de scope OAuth — vérifié par un push
  manuel réel avant d'automatiser.
  - Clé privée : `$HOME/.ssh/thelia3_site_backup_deploy_key`, permissions
    `600`, **jamais commitée, jamais dans un log**. `StrictHostKeyChecking=yes`
    avec `$HOME/.ssh/known_hosts` pré-rempli (clé d'hôte github.com) — jamais
    de désactivation de la vérification d'hôte.
- `scripts/git-hooks/post-commit` (déjà branché via `core.hooksPath` pour
  MYO-369) appelle en tâche de fond `scripts/auto-push-offsite.sh` après
  chaque commit sur `twig` : `git push backup twig` via cette deploy key
  (`GIT_SSH_COMMAND` positionné dans le script + `core.sshCommand` du dépôt),
  jamais `--force`, jamais de retry en boucle. Log dédié :
  `/home/enurit/backups/thelia3-offsite/auto-push.log`.
- `scripts/check-unpushed-offsite.sh` (cron, toutes les 15 min, lecture
  seule) compare `backup/twig` à `twig` local et journalise
  `WARN-UNPUSHED`/`WARN-DIVERGED` en cas de retard — **divergence par
  rapport à `check-unpushed-myorg.sh`** : `emmanuelnurit/thelia3-site` est
  privé, donc même une lecture (`git ls-remote`) a besoin d'un credential
  ici (même deploy key que le push, jamais de push depuis ce script).
- `scripts/install-offsite-backup.sh` installe/actualise ces volets de façon
  idempotente (hook déjà versionné, remote `backup` normalisé en SSH,
  `core.sshCommand` configuré, ligne crontab avec marqueur
  `# MYO-428 check-unpushed-offsite ...`, remplacée et non dupliquée à
  chaque exécution — vérifié par un second run consécutif : occurrence
  unique du marqueur). Il vérifie la présence et les permissions de la
  deploy key mais ne la génère pas (opération ponctuelle avec l'API GitHub,
  hors de ce script réexécutable).

### 6.3 RPO réel

- Cas nominal (hook réussit) : quasi temps réel — le push est déclenché en
  tâche de fond immédiatement après chaque `git commit` sur `twig`, avant
  même la fin du heartbeat qui l'a produit.
- Cas de rattrapage (hook a échoué — réseau, credentials absents hors run
  agent) : jusqu'à `CRON_INTERVAL_MIN` (défaut 15 min) pour que le filet
  cron le *détecte* (`WARN-UNPUSHED`) ; le push effectif ne repart lui-même
  qu'au prochain commit qui redéclenche le hook, ou via un lancement manuel
  de `scripts/auto-push-offsite.sh manual`. `check-unpushed-offsite.sh` ne
  pousse jamais — il alerte seulement.

### 6.4 Restaurer depuis cette copie

Le dépôt étant privé, un clone nécessite un credential en lecture. La
restriction de scope OAuth `workflow` (§6.2) ne bloque que les **push** qui
modifient `.github/workflows/*` — un **clone** (lecture) n'est pas concerné,
donc le token GitHub existant (compte `emmanuelnurit`, scope `repo`, secret
Paperclip `github_token`) suffit pour restaurer, pas besoin de récupérer la
deploy key privée :

```bash
git clone https://<token>@github.com/emmanuelnurit/thelia3-site.git thelia3-site-restore
# ou, sans mettre le token dans l'historique shell :
GIT_ASKPASS=<script imprimant le token sur stdout> git -c core.askPass=... clone \
  https://github.com/emmanuelnurit/thelia3-site.git thelia3-site-restore
cd thelia3-site-restore && git checkout twig
```

Alternative si le token n'est pas disponible : régénérer une deploy key
(§6.2) et cloner en SSH — une deploy key en lecture (`read_only: true`)
suffit pour cet usage, pas besoin de la variante en écriture utilisée par le
push.

Vérification faite lors de la recette MYO-433 (exécutée dans un répertoire
scratch, hors de `/home/enurit/workspace/thelia3`, avec `$GITHUB_TOKEN`) : le
clone contient bien `templates/backOffice/default-twig`,
`templates/frontOffice/flexy` et `scripts/`, et son `git rev-parse HEAD` est
strictement égal au `HEAD` du dépôt source au moment du test.

Cette copie ne remplace pas les autres volets du filet (§1-§4 : bundles,
dump SQL, secrets) — elle couvre spécifiquement la perte du disque qui
héberge le dépôt de travail ET ses bundles locaux au même endroit. Le reste
de la procédure de restauration (§2-§4) s'applique à l'identique une fois le
code récupéré par ce clone au lieu du bundle du §1.
