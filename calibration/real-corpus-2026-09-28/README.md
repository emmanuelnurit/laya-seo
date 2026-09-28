# Pilote sur pages réelles — 2026-09-28 (ACT/FLAG, catalogue thelia3 complet)

Troisième pilote de calibration de `policy::ACT`/`policy::FLAG` pour MYO-536, et
le premier sur du **vrai contenu produit/CMS avec vraies métadonnées** — la
limite méthodologique documentée dans `calibration/scale-2026-09-28/README.md`
("proxy = docs d'ingénierie internes, pas de vrai titre/meta") est levée ici.

## Origine du corpus — le catalogue publié entier, pas un échantillon

[MYO-537](/MYO/issues/MYO-537) (BackendEngineer, accès filesystem au dump SQL
`thelia3`, lecture seule) a exporté `thelia3/var/export/myo536-seo-corpus/corpus.csv` :
**41 pages, l'intégralité du catalogue FR publié** (`SELECT COUNT(*) FROM
product` = 30, `FROM content` = 11), pas un sous-ensemble de 200-400 comme visé
par l'AC de MYO-536. Il n'existe pas de plus grand corpus réel à échantillonner
dans cet environnement actuellement — voir le commentaire de clôture de
MYO-537 pour le détail (contournement du blocage Docker/DDEV via un dump SQL
existant, `product`/`content` seulement, `rewriting_url` non redirigée
`fr_FR`).

Chaque ligne (`url,type,title,meta_description,body`) a été convertie en
fichier `.md` avec frontmatter YAML (`title`, `description` = les vraies
métadonnées du site, pas des valeurs vides ou dérivées du nom de fichier comme
dans le pilote précédent) + corps de page réel. 30 fiches produit, 11 pages
CMS. Voir `manifest.tsv` pour la correspondance fichier → URL source.

## Méthode

1. Bridge `laya-bridge` démarré localement (`scripts/run-laya-bridge.sh`,
   checkpoint multilingue, `LAYA_LOCAL_URL=http://127.0.0.1:8791`).
2. `jev-seo decide calibration/real-corpus-2026-09-28/content --limit 50 --json`
   → 41 décisions (une par page).
3. `jev-seo link calibration/real-corpus-2026-09-28/content --limit 50 --json`
   → 11 suggestions de lien.
4. Vérité terrain posée par l'agent (SEOPlatformEngineer), page par page :
   - **`decide`** : le détecteur de cannibalisation déterministe
     (`audit::cannibalization_pairs`) n'a trouvé **aucune paire** parmi les 41
     pages — normal, ce sont 30 produits distincts (mobilier différent à
     chaque fois : nom, matière, marque) et 11 pages CMS de sujets distincts.
     Conséquence directe : les 8 décisions `merge`/`update` rendues par le
     modèle sans aucun candidat de fusion identifié (`cannibalization_with:
     null` dans les 8 cas, vérifié dans `~/.jev-seo/decisions.jsonl`) sont
     **structurellement fausses** — il n'y a rien avec quoi fusionner. Relu
     chaque cas individuellement contre le contenu réel pour confirmer
     (exemples : `fauteuil-cuir-jaune-leela` vs `fauteuil-cuir-wilson`, deux
     fauteuils cuir de marques et styles différents — pas de doublon ; canapé
     `capitonne-chrome-kyle`, page complète et à jour, aucune raison
     éditoriale de la marquer `update`). Les 33 décisions `keep` restantes :
     `correct=true` pour toutes — 30 fiches produit distinctes et 11 pages CMS
     de sujets distincts, aucun candidat légitime à la suppression/fusion.
   - **`link`** : les 11 sources et pages cibles lues intégralement. Résultat
     net et non aléatoire : **les 5 suggestions correctes pointent toutes vers
     `cms--livraison`** (infos pratiques d'expédition — garantie, nouveaux
     horaires, bon de réduction (mention du seuil de livraison offerte),
     mentions légales, annonce produit : cohérence thématique "infos
     pratiques/politiques" réelle). **Les 6 suggestions fausses pointent
     toutes vers `cms--nouveaute-scarlett`** (une brève annonce produit sur
     "la chaise Scarlett") depuis 6 produits sans aucun rapport (chaise
     campagne bois, fauteuil rétro, chaise design blanc, tabouret bar,
     fauteuil cuir jaune, canapé vintage) — un biais net d'"aimant" vers une
     page cible courte et générique, indépendant du sujet réel de la page
     source.
5. `jev-seo calibrate` sur les deux jeux, seuls puis combinés avec les labels
   du pilote précédent (`calibration/scale-2026-09-28/`, docs proxy).

## Résultat

```
decide (n=41 réel, 33 correct) :
  Default 0.80:       precision 0.00  recall 0.00  F1 0.00
  Recommandé 0.00:    precision 0.80  recall 1.00  F1 0.89

link (n=11 réel, 5 correct) :
  Default 0.80:       precision 0.50  recall 0.80  F1 0.62
  Recommandé 0.52:    precision 0.50  recall 1.00  F1 0.67

decide combiné (n=130 = 89 proxy + 41 réel, 119 correct) :
  Default 0.80:       precision 0.00  recall 0.00  F1 0.00
  Recommandé 0.00:    precision 0.92  recall 1.00  F1 0.96

link combiné (n=20 = 9 proxy + 11 réel, 12 correct) :
  Default 0.80:       precision 0.64  recall 0.58  F1 0.61
  Recommandé 0.51:    precision 0.65  recall 0.92  F1 0.76
```

## Action prise : **aucune** — `policy::ACT=0.80`/`FLAG=0.45` restent inchangés

Ce troisième pilote **confirme sur du vrai contenu produit/CMS** exactement le
diagnostic déjà posé sur le corpus proxy — ce n'était donc pas un artefact du
choix de corpus :

- **`decide`** : la sortie brute de `calibrate` recommande toujours ACT=0.00,
  et c'est toujours un artefact de déséquilibre (33/41 `keep` triviaux), pas
  un signal fiable. Suivre cette recommandation traiterait comme des faits
  actionnables les 8 décisions `merge`/`update` erronées de ce même jeu
  (confiance 0.10–0.21, déjà correctement gatées `drop` par le seuil actuel).
- **`link`** : le signal est réel mais **pas de nature à corriger par un
  seuil**. Les 6 faux positifs ne sont pas concentrés aux confiances basses —
  ils couvrent presque tout le spectre (0.49 à 1.00, dont deux à >0.97) parce
  que c'est un biais structurel du modèle (attraction vers une page cible
  courte/générique), pas une incertitude de score. Le seuil recommandé (0.52)
  n'exclut qu'1 des 6 faux positifs (F1 0.67, précision toujours 0.50) : même
  optimisé, la précision reste une pièce jetée à pile ou face. Un vrai
  correctif porterait sur le prompt/les critères de `link` (pénaliser
  explicitement les pages cibles "hub" courtes sans lien thématique
  vérifiable), pas sur `policy::ACT`/`FLAG` — hors périmètre de ce pilote de
  calibration, à traiter comme un suivi produit séparé si `link` doit un jour
  sortir du mode "assistance humaine relit tout" documenté au §11b du README
  principal.

## Total labellisé à la main sur les trois pilotes MyOrg réels

24 (injection, README de ce dépôt) + 130 (decide, proxy+réel) + 20 (link,
proxy+réel) = **174 décisions**, encore en dessous des « quelques centaines »
visées par l'AC de MYO-536, mais c'est désormais le **plafond réel** de cet
environnement, pas un choix d'échantillonnage :

- Le catalogue thelia3 publié ne contient que 41 pages au total (confirmé par
  requête directe sur `product`/`content`, cf. clôture MYO-537) — il n'existe
  pas 200-400 pages réelles à extraire ici sans en fabriquer, ce qui
  recréerait exactement le biais "corpus non représentatif" déjà rejeté.
- Aucun accès Google Search Console n'a jamais été débloqué pour cette série
  de pilotes (cas d'usage 1, classification d'intention, reste donc non
  calibré empiriquement — le workflow existe et fonctionne, `jev-seo intent
  --csv <export>`, mais sans jeu de requêtes réelles MyOrg à labelliser).
- Le cas d'usage 4 (scoring GEO) n'a pas non plus de vérité terrain
  disponible dans cet environnement (pas d'accès aux moteurs IA pour vérifier
  une citation réelle).

Débloquer davantage suppose soit un accès GSC réel (cas d'usage 1), soit la
croissance organique du catalogue thelia3 (cas d'usage 2/3), ni l'un ni
l'autre du ressort d'un agent dans cet environnement — décision de laisser
`policy::ACT`/`FLAG` en l'état, documentée sur trois pilotes indépendants
(proxy interne, réel, combiné) qui pointent tous vers la même conclusion :
remonté au CTO en revue plutôt que tranché unilatéralement ici.

## Fichiers

- `manifest.tsv` : fichier → URL source → type → nombre de mots.
- `content/*.md` : 41 pages, frontmatter = vraies métadonnées `title`/`meta_description` thelia3, corps = vrai texte produit/CMS.
- `decide-raw.json`, `link-raw.json` : sortie brute `jev-seo decide`/`link --json`.
- `decide-labels.csv`, `link-labels.csv` : `id,confidence,correct` labellisés à la main sur ce corpus réel seul.
- `combined-decide-labels.csv`, `combined-link-labels.csv` : mêmes labels + ceux de `calibration/scale-2026-09-28/` (proxy), pour la calibration combinée ci-dessus.
