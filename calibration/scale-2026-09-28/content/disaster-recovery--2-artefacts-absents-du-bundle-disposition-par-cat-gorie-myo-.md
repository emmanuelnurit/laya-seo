2. Artefacts absents du bundle — disposition par catégorie (MYO-401)


`git bundle` ne contient que l'historique committé. Ce qui en manque se
répartit maintenant en trois catégories tranchées par MYO-401 (enfant de
MYO-400), au lieu d'un unique « à copier depuis une install existante » —
constatées par comparaison directe (`diff -rq`) entre le clone restauré et
le dépôt de travail.

### 2.1 Versionné en dépôt depuis MYO-401 (piste 1) — n'apparaît plus ici

Ces fichiers sont **dans le bundle git** depuis MYO-401 (exception
`.gitignore` ciblée, même modèle que `default-twig`/MYO-214 et
`flexy`/MYO-311, précédés d'un grep systématique pour écarter tout secret
en clair — seuls des placeholders `%env(...)%` étaient présents) : ils
n'ont donc plus besoin d'être fournis à la main pour ce plan de
restauration.

| Chemin | Avant MYO-401 | Depuis MYO-401 |
|---|---|---|
| `bin/console`, `bin/phpunit` | à copier depuis une install existante | versionné |
| `config/services.yaml`, `config/preload.php`, `config/packages/*.yaml` (~20 fichiers) | à copier depuis une install existante (trou réel : sans accès réseau à `flex.symfony.com`, `composer install` ne les régénère pas, cf. § plus bas) | versionné |
| `templates/backOffice/default`, `templates/pdf/default`, `templates/email/default` | à copier depuis une install existante | versionné |

**Addendum MYO-402** (même principe « source à suivre pour la
reproductibilité », appliqué après MYO-401) :

| Chemin | Avant MYO-402 | Depuis MYO-402 |
|---|---|---|
| `templates/backOffice/default-twig/package-lock.json` | gitignoré (`.gitignore` imbriqué du thème) → dérive npm dans le temps, build BO non reproductible | versionné |

### 2.2 3ᵉ volet du filet — secrets, jamais versionnés (piste 2)

Ces fichiers restent délibérément **hors dépôt**, y compris hors bundle :
`origin` de `thelia3` est le dépôt public upstream Thelia (jamais poussé,
MYO-387/MYO-341), mais un bundle est une copie complète de l'historique
commité — un secret commité une fois y resterait de façon quasi
irréversible même sans push. Ils sont couverts par un filet automatisé
séparé (`scripts/backup-secrets.sh`, cron 6h, archive tar.gz `chmod 600`
hors de tout dépôt git dans `/home/enurit/workspace/secrets-backups/`,
même classe que `/home/enurit/workspace/db-backups`) :

| Chemin | Pourquoi c'est un secret | Comment le restaurer |
|---|---|---|
| `config/jwt/private.pem`, `config/jwt/public.pem` | clés JWT (auth API) | `tar -xzf thelia3-secrets-<horodatage>.tar.gz -C <racine du clone>` |
| `local/config/database.yml` | contient les creds DB en clair | idem, puis adapter `dbname` à la base dédiée à la restauration |

Étape exécutée telle quelle lors de l'exercice MYO-401 :

```bash
tar -xzf /home/enurit/workspace/secrets-backups/thelia3-secrets-<horodatage>.tar.gz -C thelia3-site
# → produit config/jwt/private.pem, config/jwt/public.pem, local/config/database.yml
```

Vérification faite : les trois fichiers extraits sont bien présents et
identiques (byte à byte, `cmp`) à ceux du dépôt de travail — c'est
exactement le contrôle que `scripts/backup-secrets.sh` fait déjà sur
lui-même à chaque run (extraction de test + `cmp`), donc une archive qui a
passé ce script est déjà prouvée extractible avec ce contenu précis.

### 2.3 Hors périmètre — justifié explicitement (rien à versionner ni à sauvegarder)

| Chemin | Justification |
|---|---|
| `vendor/` | régénérable via `composer install` — réseau `packagist.org` vérifié joignable (HTTP 200, 2026-09-15) — **sauf `vendor/thelia/modules/Comment/`, exception ci-dessous** |
| `vendor/thelia/modules/Comment/` | **exception au régénérable ci-dessus, trou accepté et documenté (MYO-403, 2026-09-15).** Vestige manuel : absent de `composer.json`, absent de `vendor/composer/installed.json` (confirmé par comparaison des 19 dossiers de `vendor/thelia/modules/` contre les paquets installés composer) — un `composer install` frais ne le restaure pas. Seul module dans ce cas : les 17 autres dossiers de `vendor/thelia/modules/` sont bien des paquets composer trackés (`thelia/cheque-module`, `thelia/page-module`, etc.) ; `CommerceAgents` est à part, c'est le module maison, hors périmètre ici et déjà couvert par `local/modules/` (cf. [MYO-289](/MYO/issues/MYO-289)). Aucun correctif de fond retenu ici, priorité basse : en cas de sinistre réel, recopier `vendor/thelia/modules/Comment/` depuis une install donneuse avant tout `cache:clear`. Le risque de *crash* associé (`ClassNotFoundError` observé pendant l'exercice MYO-401 AC2) est neutralisé indépendamment par le correctif noyau de [MYO-403](/MYO/issues/MYO-403) : un module actif dont le code manque au boot est désormais journalisé et sauté proprement (même en debug), pas fatal — donc l'oubli de ce module dégrade la fonctionnalité Comment, il ne casse plus le BO. |
| `.env.local` | rien à préserver de l'original : ce n'est pas une copie mais une création à la main pour la base dédiée de chaque exercice de restauration (identifiants/`DATABASE_NAME` propres à cet exercice, jamais ceux de l'install source) |
| `templates/backOffice/default-twig/dist/` (build Webpack Encore) | supposé "build reproductible" — **infirmé en pratique le 2026-09-15** (exercice MYO-401 AC2) : sans lockfile committé, un install frais résolvait des versions npm plus récentes qu'au dernier build connu et échouait (`@babel/core` exige Node ≥ 22.18, le conteneur DDEV tourne en Node 20). **Corrigé par MYO-402** (2026-09-15, même jour) : `package-lock.json` du thème committé (cf. §2.1 addendum) — voir §4 pour la commande d'install correcte, `npm ci`, pas `yarn install --frozen-lockfile` |
| `var/tailwind/` (binaire `tailwindcss`, ~114 Mo) | téléchargé par `symfonycasts/tailwind-bundle` depuis `github.com/tailwindlabs/tailwindcss/releases` — URL de téléchargement vérifiée joignable (HTTP 200, 2026-09-15, `curl` direct sur l'asset de release) ; sans lui, `bin/console tailwind:build` échoue et la FO (`flexy`) plante en 500, mais le réseau suffit à le régénérer, donc rien à copier ni sauvegarder |
| `templates/frontOffice/flexy/assets/vendor/` (AssetMapper) | déjà hors périmètre avant MYO-401, inchangé : régénéré par `bin/console importmap:install` (réseau `cdn.jsdelivr.net` vérifié OK) |

**Points confirmés comme non nécessaires à copier — se régénèrent tout seuls
à partir du code + de la donnée restaurés, sans intervention manuelle :**

- `var/propel/<env>/model/` — généré automatiquement par
  `PropelInitService::init()` au premier boot du kernel (Symfony ou CLI), dès
  que la base cible contient une table `config` peuplée (voir §3). Rien à
  copier ni à commander explicitement pour ce cas précis — `bin/console
  cache:clear` (ou toute première requête HTTP) le déclenche.
- `templates/frontOffice/flexy/assets/vendor/` (AssetMapper) — cf. §2.3,
  régénéré par `bin/console importmap:install`.
- `public/assets`, `public/bundles`, etc. — régénérés par `bin/console
  assets:install public --symlink`.
- `.ddev/` (hors `.ddev/config.yaml`, seul fichier tracké) — régénéré par
  `ddev start` lui-même.
