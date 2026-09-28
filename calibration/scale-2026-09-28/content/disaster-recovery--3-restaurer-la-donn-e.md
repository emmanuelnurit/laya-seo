3. Restaurer la donnée


Ne **jamais** cibler les bases `db`/`test` du projet DDEV partagé
`thelia3` — c'est le garde-fou MYO-387/MYO-393 (`scripts/safe-install.sh`)
et il reste valable ici même si l'exercice a lieu ailleurs. Concrètement,
pour un exercice de restauration isolé, la manière la plus sûre est de faire
tourner un **second projet DDEV entièrement séparé** (conteneurs et volumes
propres, aucun risque de toucher `db`/`test` du projet partagé, quel que soit
le nom donné à la base à l'intérieur) :

```bash
cd thelia3-site        # le clone du §1, avec les artefacts du §2 en place
ddev start
ddev mysql -uroot -proot -e "CREATE DATABASE myo399_restore;"
gunzip -c <dump.sql.gz> | ddev mysql -uroot -proot myo399_restore
```

Adapter `.env.local` (`DATABASE_NAME`, `DATABASE_URL`) **et** — piège vérifié
en pratique — `.ddev/config.yaml` (`web_environment: DATABASE_NAME=...`) pour
qu'ils pointent tous les deux sur le même nom de base : DDEV injecte
`DATABASE_*` comme de vraies variables d'environnement du conteneur, et
Symfony Dotenv ne les écrase jamais silencieusement — `.env.local` seul ne
suffit pas si `.ddev/config.yaml` dit autre chose.

**Piège vérifié en pratique** : l'utilisateur MySQL par défaut de DDEV (`db`)
n'a de droits que sur la base par défaut du projet — créer une base
supplémentaire par un autre nom nécessite un `GRANT` explicite, sinon
l'application se connecte silencieusement mais reçoit un `Access denied`, ce
qui se traduit côté Thelia par un `TheliaKernel::isInstalled() === false`
alors que la donnée est bien là :

```bash
ddev mysql -uroot -proot -e "GRANT ALL PRIVILEGES ON myo399_restore.* TO 'db'@'%'; FLUSH PRIVILEGES;"
```

**Piège supplémentaire découvert lors de MYO-401** : `ddev start` gère
automatiquement les settings Symfony et **écrase `.env.local`** créé à la
main au §2 (il y réinjecte `DATABASE_NAME=db`/`DATABASE_URL=...db`, les
valeurs par défaut du projet DDEV, pas celles de la base dédiée à
l'exercice). Contournement vérifié : ajouter
`disable_settings_management: true` dans `.ddev/config.yaml` avant
`ddev start`, puis (re)créer `.env.local` — stable après `ddev restart`.

On ne repasse **pas** par `bin/install` ici : restaurer un dump, c'est
importer la donnée réelle telle quelle, pas régénérer une base neuve avec des
données de démo (qui écraserait justement ce qu'on cherche à restaurer).
`bin/install`/`scripts/safe-install.sh` restent le seul chemin autorisé pour
une **installation neuve** (jamais `bin/install` en direct, jamais cibler
`db`/`test` sans `--i-know-what-i-am-doing`) ; ce n'est pas l'opération faite
ici.
