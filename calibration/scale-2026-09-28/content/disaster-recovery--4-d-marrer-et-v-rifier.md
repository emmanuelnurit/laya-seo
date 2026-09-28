4. Démarrer et vérifier


```bash
ddev exec -- php bin/console cache:clear
ddev exec -- php bin/console assets:install public --symlink
ddev exec -- php bin/console tailwind:build          # thème flexy (FO)
ddev exec -- php bin/console importmap:install        # thème flexy (FO)
ddev exec -- bash -c "cd templates/backOffice/default-twig && npm ci && yarn build"  # thème default-twig (BO)
ddev exec -- curl -s -o /dev/null -w '%{http_code}\n' http://localhost/admin/login
ddev exec -- curl -s -o /dev/null -w '%{http_code}\n' http://localhost/
```

**Piège vérifié en pratique (MYO-402)** : `npm ci`, pas `yarn install
--frozen-lockfile`. Le lockfile committé pour `default-twig` est
`package-lock.json` (format npm) ; `yarn` classic (v1, celui utilisé par ce
thème) ne le comprend pas — il ignore silencieusement `package-lock.json`
sans erreur (juste un warning), retombe sur une résolution libre depuis le
registre npm du jour, et reproduit exactement le bug d'origine
(`Cannot find module '@babel/core'`) au `yarn build` qui suit. `--frozen-lockfile`
ne proteste pas non plus, faute de `yarn.lock` à comparer. `npm ci` est la
seule commande qui respecte réellement ce lockfile (échec net si
`package.json`/lockfile divergent) ; une fois `node_modules` peuplé par
`npm ci`, `yarn build` (qui ne réinstalle rien) fonctionne normalement.

Preuve attendue : `200` sur les deux — pas seulement l'absence d'erreur au
clone ou à l'import. Un dump vide/tronqué qui « s'importe sans erreur »
resterait indétectable sans ce test HTTP réel (c'est le même écueil que
l'incident MYO-395/396 sur le filet DB).

**Résultat réel de l'exercice MYO-401 (2026-09-15)** : `/` → `200` ✓ ;
`/admin/login` → `500`. Deux causes rencontrées, aucune liée au
versionnement config/ ni à l'archive secrets (qui ont, eux, fonctionné
comme attendu — la FO en est la preuve, elle dépend du même kernel) :

1. **Module `Comment` actif en base mais absent du code restauré** (son
   code vit dans `local/modules/Comment`, gitignoré, hors des deux
   bundles) → `cache:clear` plantait avec `Class "Comment\Comment" not
   found`. Gap déjà tracké séparément (MYO-379, absence de ce module dans
   un install neuf). Contournement pour les besoins de l'exercice :
   `UPDATE module SET activate=0 WHERE code='Comment';` avant
   `cache:clear`.
2. **Build BO (`yarn build`) non reproductible** : cf. §2.3 et
   [MYO-402](/MYO/issues/MYO-402), gap distinct de MYO-401 (pas de
   lockfile committé pour le thème `default-twig`, dérive npm dans le
   temps). Sans le build, `/admin/login` échoue avec « Could not find the
   entrypoints file from Webpack ».

Aucune des deux causes ne remet en cause le correctif MYO-401 lui-même :
le kernel Symfony démarre et sert du contenu réel dès que `config/` et les
secrets sont en place, ce qui est exactement ce que ce ticket devait
prouver.

**Résultat réel de l'exercice MYO-402 (2026-09-15, même jour, projet DDEV
isolé `myo402restore`)** : protocole rejoué à l'identique depuis un bundle
régénéré après le commit du lockfile (`HEAD=8ed80eab5`), avec la même
archive secrets et le même dump que l'exercice MYO-401 précédent. `/` → `200`
✓ ; `/admin/login` → `200` ✓ (page réelle, titre `Welcome - Thelia`, pas
d'erreur webpack masquée). Un premier essai avec `yarn install
--frozen-lockfile` a d'abord semblé passer (exit 0, juste des warnings) mais
`@babel/core` restait absent de `node_modules` et `yarn build` échouait à
l'identique de l'exercice MYO-401 — cf. piège documenté au §4 ci-dessus :
`yarn` classic ignore silencieusement le lockfile npm. Reproduit avec `npm
ci` à la place : succès net, `@babel/core@7.29.7` installé, `yarn build`
réussit (510 fichiers écrits dans `dist/`). Contournement `Comment` du §
précédent toujours nécessaire (gap MYO-379, non lié). Nettoyage effectué en
fin d'exercice (`ddev delete -O myo402restore`, suppression du répertoire
isolé).
