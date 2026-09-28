2. Relancer l'environnement de zéro


Depuis la racine du dépôt (`/home/enurit/workspace/thelia3`), sur le worktree existant ou un nouveau :

```bash
# 1. Worktree isolé (à sauter si .worktrees/myo468-demo existe déjà)
git worktree add .worktrees/myo468-demo HEAD
mv .worktrees/myo468-demo/.ddev .worktrees/myo468-demo/.ddev.disabled

# 2. Dépendances non versionnées à copier en dur (jamais en symlink — cf. mémoire
#    thelia3-isolated-install-worktree-technique : un vendor/ symlinké fait
#    doublonner les classes générées par Composer)
cp -a vendor .worktrees/myo468-demo/vendor
mkdir -p .worktrees/myo468-demo/local/modules
cp -a local/modules/CommerceAgents .worktrees/myo468-demo/local/modules/CommerceAgents
cp composer.lock .worktrees/myo468-demo/composer.lock
mkdir -p .worktrees/myo468-demo/templates/backOffice/default-twig
cp -a templates/backOffice/default-twig/dist .worktrees/myo468-demo/templates/backOffice/default-twig/dist
mkdir -p .worktrees/myo468-demo/config/jwt
cp -a config/jwt/. .worktrees/myo468-demo/config/jwt/

# 3. Base dédiée (le rôle 'db' n'a pas CREATE DATABASE global — cf. mémoire
#    thelia3-isolated-install-worktree-technique, piège n°3)
ddev mysql -uroot -proot -e "DROP DATABASE IF EXISTS myo468_demo; \
  CREATE DATABASE myo468_demo CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci; \
  GRANT ALL PRIVILEGES ON myo468_demo.* TO 'db'@'%';"

# 4. Install isolée — jamais bin/install directement, toujours safe-install.sh
EXEC_DIR=/var/www/html/.worktrees/myo468-demo \
  ./scripts/safe-install.sh --database_name=myo468_demo \
  --with-demo --with-admin \
  --admin_login=admin --admin_password='DemoMyOrg2026!' \
  --admin_first_name=Admin --admin_last_name=Demo --admin_email=admin-demo@myorg.test \
  --frontoffice_theme=flexy --backoffice_theme=default-twig

# 5. Jeu de données démo CommerceAgents (avis, campagne promo, propositions IA)
env -u DATABASE_HOST -u DATABASE_PORT -u DATABASE_NAME -u DATABASE_USER -u DATABASE_PASSWORD \
  APP_ENV=dev ddev exec -d /var/www/html/.worktrees/myo468-demo -- \
  php bin/console commerce-agents:demo-seed
```

`commerce-agents:demo-seed` est **idempotent** : la relancer sur un environnement déjà peuplé ne duplique rien (vérifié sur 3 exécutions consécutives, voir preuve d'exécution en commentaire du ticket [MYO-468](/MYO/issues/MYO-468)). Elle peut donc aussi être rejouée seule, sans tout réinstaller, si seul le jeu de données CommerceAgents doit être rafraîchi.

### Servir le site pendant la démo

```bash
ddev exec -d /var/www/html/.worktrees/myo468-demo -- \
  php -S 0.0.0.0:<port> -t public public/router.php
```

`public/router.php` est un routeur PHP standard Symfony (jetable, jamais committé — voir mémoire `thelia3-isolated-install-worktree-technique`) : sans lui, `public/index.php` nu casse le `Content-Type` des assets déjà publiés dès qu'un vrai navigateur envoie `Accept-Encoding` (invisible avec un simple `curl`, visible dans un vrai navigateur — wizards BO et boutons "Tester la connexion" alors muets).

```php
<?php
if (PHP_SAPI === 'cli-server') {
    $path = urldecode(parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH));
    if ($path !== '/index.php' && is_file(__DIR__ . $path)) {
        return false;
    }
}
// PHP_CLI_SERVER_WORKERS>1 reuses this same process across requests; without
// pinning SCRIPT_FILENAME, a request served by an already-used worker makes
// vendor/autoload_runtime.php's `require $_SERVER['SCRIPT_FILENAME']`
// recurse into router.php itself (int(1) return) instead of index.php,
// throwing "Invalid return value: callable object expected, int returned
// from public/router.php" -- intermittent (found during the MYO-487
// rehearsal: every FO product page failed roughly 1 request in 4).
$_SERVER['SCRIPT_FILENAME'] = __DIR__ . '/index.php';
require __DIR__ . '/index.php';
```

### Nettoyage après la démo

```bash
git worktree remove .worktrees/myo468-demo
ddev mysql -uroot -proot -e "DROP DATABASE IF EXISTS myo468_demo;"
```
