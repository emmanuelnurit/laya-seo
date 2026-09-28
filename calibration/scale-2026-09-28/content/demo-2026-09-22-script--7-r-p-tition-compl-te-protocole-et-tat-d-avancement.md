7. Répétition complète — protocole et état d'avancement


### 7.1 État au 21/09 22h30

T6/T3/T4 ([MYO-470](/MYO/issues/MYO-470)/[MYO-472](/MYO/issues/MYO-472)/[MYO-473](/MYO/issues/MYO-473)) sont `done`. T7 ([MYO-471](/MYO/issues/MYO-471)) est `blocked` (post-démo) : décision REPLI actée, l'acte 1 se joue en version F1+F6 uniquement (voir bandeau §Acte 1 et §6). **Plus aucun chantier ne bloque la répétition complète (AC2)** — elle est déléguée à FrontendEngineer, voir le commentaire de délégation sur [MYO-474](/MYO/issues/MYO-474).

**⚠️ Vigilance environnement (voir commentaire [MYO-475](/MYO/issues/MYO-475) sur MYO-474)** : entre ~22h11 et ~22h40 le 21/09, l'environnement DDEV partagé a affiché par erreur le message opt-in T7 sur `add_to_cart` (backend T7 déployé par erreur sur la base partagée `db`). Corrigé (commit `34be01d`). Avant de rejouer l'acte 1, vérifier explicitement qu'`add_to_cart` sur l'environnement isolé de démo (base `myo468_demo`, jamais `db`) renvoie bien la carte coupon F1/F6 normale et non un message opt-in.

**L'acte 5 (copilote, E1) reste le seul déjà testé indépendamment** dans un heartbeat précédent — à revérifier avec le reste lors de la répétition complète.

### 7.2 Protocole de répétition

1. Repartir de l'environnement isolé décrit dans `demo-2026-09-22.md` (worktree + base `myo468_demo`), jamais du site partagé. Vérifier `DATABASE_NAME=myo468_demo` avant d'ouvrir quoi que ce soit.
2. Chronomètre en main, jouer les 5 actes **dans l'ordre**, sans sauter l'acte 1 côté boutique.
3. Acte 1 : jouer uniquement la version repliée **F1+F6** (pas de version opt-in ce soir — décision REPLI actée, voir bandeau ci-dessus).
4. Vérifier le silence après fermeture du widget (2 pages différentes).
5. Noter les durées réelles par acte et ajuster ce document si un acte déborde.
6. Consigner le résultat acte par acte en commentaire sur MYO-474 (AC5), avec preuve d'exécution réelle (capture/sortie), pas une simple déclaration.

### 7.3 Reprise

Répétition jouée, rapport reçu — voir §8.

---
