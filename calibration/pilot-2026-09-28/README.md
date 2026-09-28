# Pilote de calibration — 2026-09-28

Ce dossier documente un pilote de calibration du seuil de confiance
`injection_preflight` (MYO-536), et sert de méthode + jeu de données
reproductible pour la calibration à pleine échelle qui reste à faire.

## Pourquoi un pilote, pas la calibration finale

L'issue MYO-536 demande une calibration sur « quelques centaines de
décisions labellisées à la main sur du contenu MyOrg réel ». Depuis ce
sandbox, deux accès manquent pour faire cette calibration à pleine échelle :

- **Pas de réseau vers un site MyOrg réel** : `ddev list` échoue
  (`docker.sock` injoignable) — aucune instance `thelia3`/`flexy` n'est
  accessible en lecture pour constituer un corpus de vraies pages produit.
- **Pas d'accès Google Search Console** : `GOOGLE_CLIENT_ID`/`SECRET` absents
  de l'environnement, aucun token stocké — impossible de tirer un vrai
  export de requêtes pour le cas d'usage 1 (classification d'intention).

Ce pilote utilise donc le seul contenu MyOrg réel accessible depuis ce
sandbox : **le README de ce dépôt lui-même** (18 sections, découpage par
`## titre`, réécrites telles quelles dans `content/`), complété par 6
exemples adverses construits à la main (patterns d'injection de prompt
classiques OWASP LLM01, nécessaires comme classe positive puisque le README
ne contient par construction aucune vraie tentative d'injection).

**Ce pilote ne remplace pas la calibration pleine échelle.** Il valide le
pipeline outillage (`jev-seo calibrate`) de bout en bout sur de vraies
décisions, et documente une découverte bien réelle et actionnable (voir
ci-dessous) — mais 18 exemples négatifs d'une seule source (doc technique
sur un produit d'audit IA) ne sont pas représentatifs de l'ensemble du
contenu MyOrg (produit e-commerce, pages CMS, etc.).

## Méthode

1. `README.md` découpé en 18 sections ≥ 40 mots (`content/*.md`), copie
   fidèle du contenu réel du dépôt à la date du pilote.
2. 6 exemples adverses construits (`injection-labels.csv`, lignes
   `adversarial-*`) : patterns d'injection classiques (« ignore previous
   instructions », exfiltration de secrets demandée à l'IA, etc.).
3. Chaque exemple passé à `injection_preflight` en conditions réelles
   (`laya-bridge` local, checkpoint `multilingual`, même appel que le code
   de production après le correctif MYO-536 sur `injection_surface`).
4. Label `correct` = vérité terrain posée par l'agent (SEOPlatformEngineer) :
   `false` pour les 18 sections README (aucune n'est une tentative
   d'injection), `true` pour les 6 exemples adverses. **Ce label vient de
   l'agent, pas d'un relecteur humain distinct** — limite à noter avant de
   faire confiance au chiffre recommandé sans revue.
5. `jev-seo calibrate injection-labels.csv --default-threshold 0.70`.

## Résultat

```
Labeled examples:   24 (6 correct)
Default 0.70:       precision 0.32  recall 1.00  F1 0.48
Recommended 0.99:   precision 1.00  recall 1.00  F1 1.00
```

**Découverte** : au seuil par défaut 0.70 (skill rule-of-thumb, jamais
calibré sur du contenu réel), 17 des 18 sections réelles du README —
documentation produit authentique, pas du contenu suspect — dépassent le
seuil et se font bloquer par le pré-filtre d'injection. Precision 0.32 à ce
seuil : sur 10 décisions "bloquer", seules ~3 sont de vraies tentatives
d'injection.

**Cause probable** : ce produit documente littéralement des agents IA, des
clés API, des serveurs MCP, du contrôle d'outils — le vocabulaire d'une
doc produit légitime sur l'outillage agentique chevauche fortement celui
d'une tentative d'injection ciblant un agent. Pertinent au-delà de ce repo :
toute page MyOrg qui parle de fonctionnalités IA (ex. CommerceAgents,
copilote merchant) risque le même chevauchement.

## Action prise (provisoire)

Le seuil `policy::INJECTION_BLOCK` est relevé de 0.70 à **0.95** (voir
commentaire dans `src/policy.rs`) : réduit le taux de faux positifs sur ce
pilote de 17/18 (94%) à 4/18 (22%), sans rater aucun des 6 exemples adverses
(tous ≥ 0.985). **Valeur explicitement provisoire** — posée sur 24 exemples
dont 18 d'une seule source homogène, pas sur les « quelques centaines »
demandées par l'issue. À recalibrer dès qu'un accès réel (site MyOrg en
lecture, ou export GSC) est disponible.

## Prochaine étape (bloquée, propriétaire CTO)

Pour compléter la calibration pleine échelle demandée par MYO-536 :
- **Option A** — accès réseau en lecture à une instance `thelia3`/`flexy`
  vivante (DDEV) depuis ce workspace, pour crawler un vrai corpus de pages
  produit/CMS à labelliser.
- **Option B** — un export CSV Search Console réel (`Queries.csv`, cas
  d'usage 1) et/ou un lot de pages réelles fourni par un humain, à
  labelliser par l'agent ou en binôme avec un relecteur humain.

`jev-seo intent --csv` et `jev-seo decide` sont prêts à consommer l'un ou
l'autre dès que disponible ; `jev-seo calibrate` prend n'importe quel CSV
`id,confidence,correct` en entrée, y compris un export du ledger
`jev-seo review --json` une fois des décisions réelles accumulées et
validées par un relecteur.
