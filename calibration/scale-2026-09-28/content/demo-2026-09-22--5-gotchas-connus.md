5. Gotchas connus


- **`URL::getInstance()` non initialisé en CLI** : toute commande console qui sauvegarde un objet avec URL réécrite (ex. `Sale`) doit instancier `new \Thelia\Tools\URL($router)` en tout début d'`execute()`, comme le fait déjà `Thelia\Command\Import\DemoImportCommand`. `DemoSeedCommand` le fait.
- **`ProductPrice::setPromoPrice()` attend une chaîne**, pas un float — comme les autres colonnes `DECIMAL` générées par Propel dans ce dépôt.
- Le worktree doit garder son `.ddev/` renommé en `.ddev.disabled` **avant tout premier `ddev exec` lancé depuis son propre dossier**, sous peine de faire naître un second projet DDEV complet (cf. mémoire `thelia3-isolated-install-worktree-technique`).
