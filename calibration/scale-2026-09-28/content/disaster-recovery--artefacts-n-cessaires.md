Artefacts nécessaires


- Le dernier bundle site : `/home/enurit/backups/thelia3-bundles/thelia3-site-*.bundle`
  (filet MYO-360/369, cron toutes les 15 min).
- Le dernier bundle module : `/home/enurit/backups/thelia3-bundles/commerceagents-module-*.bundle`.
- Le dernier dump SQL : `/home/enurit/workspace/db-backups/thelia3-db-*.sql.gz`
  (filet MYO-380/396, cron toutes les 30 min).
- La dernière archive de secrets : `/home/enurit/workspace/secrets-backups/thelia3-secrets-*.tar.gz`
  (filet MYO-401, cron toutes les 6h — cf. §2.2).

`origin` du dépôt `thelia3` est le dépôt public upstream Thelia — on n'y
pousse jamais (MYO-387/MYO-341). Ces bundles sont donc la **seule** copie du
code hors machine ; ce dump est la **seule** copie de la donnée hors machine ;
cette archive est la **seule** copie hors machine des secrets indispensables
au boot.
