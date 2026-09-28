# Pilote d'élargissement — 2026-09-28 (ACT/FLAG, cas d'usage 2 & 3)

Suite du pilote `calibration/pilot-2026-09-28/` (qui calibrait uniquement
`policy::INJECTION_BLOCK` sur le README de ce dépôt). Ce second pilote
attaque le seuil générique `policy::ACT`/`policy::FLAG` — celui qui gate
`decide` (cas d'usage 2), `link` (cas d'usage 3), `geo` (cas d'usage 4) et
`audit`/`crawl` (même groupe dans `policy::thresholds`) — sur du contenu
MyOrg réel plus large que le README d'un seul dépôt.

## Pourquoi encore un pilote, pas la calibration finale

La revue CTO du 2026-09-28 sur MYO-536 a débloqué un accès partiel : pas de
DDEV/Docker vivant (`docker.sock` injoignable, confirmé indépendamment par
le CTO lui-même depuis son propre run — ce n'est pas spécifique à cet
agent), pas de credentials Google Search Console, mais **lecture directe du
filesystem `thelia3`** est possible sans DDEV pour du contenu non-rendu.
Le CTO a identifié trois sources concrètes ; ce pilote en a utilisé deux
(la troisième, `messages.fr_FR.yaml`, est un catalogue de chaînes
clé/valeur, pas des pages `.md`/`.html` — `decide`/`link`/`geo` opèrent sur
des pages, donc ça ne s'y prête pas sans travail de conversion hors
périmètre de ce pilote) :

- 8 documents techniques internes réels (`docs/**/*.md` de `thelia3` :
  disaster-recovery, scripts de démo client, checkout, ventes réservées,
  réponse à incident sécurité, design MYO-479, capacités API).
- `local/modules/CommerceAgents/README.md` et `CHANGELOG.md` (module IA
  agentique réel MyOrg — même famille de vocabulaire que la découverte du
  premier pilote).

Chaque fichier a été découpé en sections `## titre` ≥ 40 mots (même méthode
que le premier pilote), copie fidèle sans modification, dans `content/`
(89 sections, voir `manifest.tsv` pour la correspondance section → fichier
source `thelia3`).

## Limite méthodologique à lire avant de faire confiance au résultat

Ce sont des **documents d'ingénierie internes** (runbooks, changelogs,
scripts de démo), pas des pages produit/CMS indexées avec un vrai titre et
une vraie meta description. `content_decision` reçoit `page.title`/
`page.description` vides ou dérivés du nom de fichier, pas les métadonnées
qu'une vraie page web porterait. C'est probablement la cause principale de
la découverte ci-dessous — pas une conclusion générale sur le calibrage
`decide`/`link` en conditions réelles de site.

## Méthode

1. `jev-seo decide calibration/scale-2026-09-28/content --limit 100 --json`
   (bridge local réel, checkpoint multilingual) → 89 décisions.
2. `jev-seo link calibration/scale-2026-09-28/content --limit 100 --json`
   → 9 suggestions de lien (les autres candidats sources n'ont trouvé aucun
   lien naturel — `no_link`, ce qui est un résultat légitime, pas un échec).
3. Vérité terrain posée par l'agent (SEOPlatformEngineer) en relisant
   chaque décision non triviale contre le contenu réel de la section :
   - Les 86 décisions `decide` = `keep` : `correct=true` pour toutes — ce
     sont des docs de référence réels, actuels, non redondants ; aucune
     n'est un candidat légitime à la suppression/fusion. C'est un jugement
     honnête, pas un tamponnage : voir "Limite" ci-dessous sur pourquoi ce
     jeu est structurellement déséquilibré vers `correct=true`.
   - Les 3 décisions `decide` ≠ `keep` (2×`merge`, 1×`delete`) : relues une
     par une contre le contenu réel — **les 3 sont fausses** (aucune n'a de
     cannibalisation détectée en face, ni de raison éditoriale réelle
     d'être fusionnée/supprimée). Toutes les 3 à confiance très basse
     (0.056–0.128), donc déjà correctement gatées "drop" par le seuil
     actuel — le modèle se trompe mais le sait.
   - Les 9 décisions `link` : relues une par une (source + section cible
     lues en entier). 7 correctes, 2 fausses (`readme--installation` →
     section backup hors-machine du dépôt **site**, et `readme--preamble`
     → même section : deux liens forcés entre deux documents sans lien de
     sujet réel). Un des deux faux positifs est à confiance 0.57, **plus
     haute** que plusieurs vrais positifs corrects (0.48, 0.51, 0.54, 0.65)
     — la confiance ne sépare pas proprement correct/incorrect sur ce jeu
     non plus.
4. `jev-seo calibrate decide-labels.csv --default-threshold 0.80` et
   `jev-seo calibrate link-labels.csv --default-threshold 0.80`.

## Résultat

```
decide (n=89, 86 correct):
  Default 0.80:       precision 0.00  recall 0.00  F1 0.00
  Recommandé 0.00:    precision 0.97  recall 1.00  F1 0.98

link (n=9, 7 correct):
  Default 0.80:       precision 1.00  recall 0.43  F1 0.60
  Recommandé 0.49:    precision 0.78  recall 1.00  F1 0.88
```

## Action prise : **aucune** — `policy::ACT`/`policy::FLAG` restent à 0.80/0.45

Contrairement au premier pilote (où la sortie brute de `calibrate` a servi
de point de départ documenté), la sortie de `calibrate` sur `decide` n'est
**pas exploitable telle quelle** ici : recommander ACT=0.00 signifierait
traiter comme des faits actionnables des décisions à 5-15% de confiance,
y compris les 3 décisions `merge`/`delete` erronées de ce même jeu. Le
F1=0.98 à ce seuil est un artefact du déséquilibre du jeu de données
(86/89 exemples `correct=true`, quasi indépendamment de la confiance), pas
un signal de calibration fiable — exactement le genre de sortie que
`policy::INJECTION_BLOCK` documente déjà comme "à ne pas suivre
aveuglément". Le jeu `link` (n=9) montre un signal plus discriminant
(0.49 vs 0.80) mais l'échantillon est trop petit pour justifier de changer
un seuil partagé avec `audit`/`geo`/`crawl`.

**Conclusion honnête** : les deux jeux de données confirment surtout que ce
corpus proxy (docs internes sans métadonnées de page réelles) n'est pas
représentatif du contenu que `decide`/`link`/`geo` verront en production
(pages produit/CMS avec vrai titre, vraie meta, vrai enjeu éditorial). Le
seuil `ACT=0.80`/`FLAG=0.45` (skill rule-of-thumb) reste en place, non
invalidé par ce pilote mais pas non plus positivement confirmé par lui —
la calibration "quelques centaines de décisions réelles" que demande
MYO-536 reste bloquée sur le même accès que le premier pilote avait déjà
identifié (Option A : lecture réseau d'une instance `thelia3`/`flexy`
vivante pour un vrai corpus de pages produit/CMS ; Option B : export GSC
réel ou lot de pages réelles fourni par un humain). Le CTO a vérifié
lui-même depuis son propre run que l'Option A est indisponible pour tout
agent actuellement (pas spécifique à ce sandbox) — ce n'est pas un
blocage que cette itération peut lever.

## Fichiers

- `manifest.tsv` : section → fichier source `thelia3` → titre → nombre de mots.
- `content/*.md` : 89 sections, copie fidèle du contenu réel `thelia3`.
- `decide-labels.csv`, `link-labels.csv` : `id,confidence,correct` labellisés à la main.
