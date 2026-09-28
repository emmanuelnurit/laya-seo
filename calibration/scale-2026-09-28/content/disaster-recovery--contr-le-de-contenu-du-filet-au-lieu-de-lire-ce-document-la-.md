Contrôle de contenu du filet (au lieu de lire ce document à la main)


Ce document décrit *comment* restaurer si besoin ; le filet automatique ne
dépend pas de sa lecture pour rester fiable :

- `scripts/backup-bundles.sh` régénère un bundle par dépôt, le vérifie
  (`git bundle verify`), vérifie sa taille (plancher `MIN_BUNDLE_BYTES`) puis
  le clone dans un dossier temporaire pour comparer son `HEAD` au `HEAD` du
  dépôt source — échec net si l'un des trois contrôles échoue.
- `scripts/trigger-backup-bundles.sh` journalise chaque run
  (`RESULT OK`/`RESULT FAILED`) et ajoute une ligne
  `ALERT-BUNDLES-BACKUP-STALE` grep-able après `FAIL_ALERT_THRESHOLD` (défaut
  3) échecs consécutifs — même pattern que `ALERT-DB-BACKUP-STALE` sur le
  filet base de données (MYO-396).
- `scripts/backup-secrets.sh` (MYO-401) régénère l'archive de secrets, vérifie
  sa taille plancher (`MIN_ARCHIVE_BYTES`), l'extrait dans un dossier
  temporaire et compare chaque fichier extrait à sa source (`cmp`) — échec
  net si l'un des trois contrôles échoue. `scripts/trigger-backup-secrets.sh`
  journalise chaque run et ajoute `ALERT-SECRETS-BACKUP-STALE` après
  `FAIL_ALERT_THRESHOLD` échecs consécutifs, même pattern que les deux
  filets jumeaux. Cron toutes les 6h (`scripts/install-secrets-backup-cron.sh`)
  — ces secrets ne changent qu'à l'installation initiale ou à une rotation
  de clé JWT délibérée, pas au rythme du travail quotidien.
- `scripts/auto-push-offsite.sh` (MYO-428/433, §6) pousse `twig` vers le
  remote privé `backup` après chaque commit et journalise
  `RESULT OK`/`RESULT FAILED` ; `scripts/check-unpushed-offsite.sh` (cron,
  lecture seule) compare indépendamment `backup/twig` à `twig` local et
  journalise `WARN-UNPUSHED`/`WARN-DIVERGED` en cas de retard — même log
  dédié : `/home/enurit/backups/thelia3-offsite/auto-push.log`.
