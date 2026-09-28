1. Cloner le code depuis les bundles


```bash
git clone --branch twig  <bundle-site>   thelia3-site
git clone --branch myorg <bundle-module> thelia3-site/local/modules/CommerceAgents
```

`git bundle create --all` ne capture que les refs commitées : si le dépôt
source a des modifications non commitées au moment du bundle, elles sont
perdues (vérifié par `git status --porcelain` avant l'exercice — le dépôt
source était propre, donc rien perdu ce jour-là, mais ce n'est pas garanti en
général).

Vérification d'intégrité minimale : `git rev-parse HEAD` du clone doit être
strictement égal au `git rev-parse twig` (ou `myorg`) du dépôt source. C'est
exactement ce que `scripts/backup-bundles.sh` vérifie déjà à chaque run (clone
de test + comparaison de HEAD), donc un bundle qui a passé ce script est déjà
prouvé clonable jusqu'à ce commit précis.
