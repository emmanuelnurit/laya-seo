1. Vue d'ensemble


- **Worktree** : `.worktrees/myo468-demo` (créé depuis la racine du dépôt `thelia3/`, jamais ailleurs).
- **Base dédiée** : `myo468_demo` (utilisateur `db`, comme le reste de l'environnement DDEV).
- **Modules actifs** : `CommerceAgents` (0.4.3) et `Comment` (3.1.1), activés automatiquement par `bin/install` (aucune activation manuelle requise).
- **Admin** : login `admin`, mot de passe `DemoMyOrg2026!` (créé par `--with-admin` à l'install ; à changer si l'environnement doit survivre au-delà de la démo).
- **Données** : catalogue + clients + commandes du jeu de démo Thelia core (`thelia:demo:import`, réaliste par construction : noms de produits, descriptions et adresses clients francophones/internationaux crédibles), **enrichi** par la commande `commerce-agents:demo-seed` (avis, campagne promo, propositions IA en attente).
