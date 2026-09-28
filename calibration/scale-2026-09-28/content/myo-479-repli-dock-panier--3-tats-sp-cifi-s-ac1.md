3. États spécifiés (AC1)


Convention : « bulle » = nouveau rendu compact de l'état `dock` **sur `/panier` uniquement**. Sur
toutes les autres pages, `.caw-dock` reste strictement inchangé.

### 3.1 Desktop (≥ 640px)

| État | Rendu |
|---|---|
| **Dock normal** (toute page ≠ `/panier`) | Inchangé : barre pilule 620px, centrée, `bottom: 1.25rem`. |
| **Bulle réduite** (`/panier`, état `dock`) | Cercle plein 56×56px, ancré `bottom: 1.25rem; left: 1.25rem` (au lieu de centré). Avatar `✦` existant (repris de `.caw-avatar`, agrandi), fond `--caw-strong`. Pas de champ de saisie ni de puce panier visibles — voir §3.3 pour la justification de l'absence du badge panier. `aria-haspopup="dialog"`, `aria-expanded="false"`, `aria-label` = `i18n.openAssistant` (« Ouvrir l'assistant d'achat », **clé existante**, aucune nouvelle chaîne). |
| **Survol / focus clavier de la bulle** | Halo `.caw-glow-dock` existant plus marqué + léger `scale(1.04)`. Anneau de focus clavier repris tel quel du pattern existant : `outline: 2px solid var(--caw-strong); outline-offset: 2px` (`chat-widget.css:523-525`). Sur pointeur (desktop uniquement, pas de `hover` tactile), un petit libellé apparaît à gauche de la bulle : même texte que l'`aria-label` (`i18n.openAssistant`), pas de nouvelle chaîne. |
| **Après ouverture puis fermeture** (`close()`/`minimise()`) | Retour à la bulle réduite (pas à la barre pleine) — cohérent avec `close()` qui bascule déjà vers `collapsed` s'il y a une conversation, ou `dock` sinon ; les deux se rendent en bulle sur `/panier`. |
| **Sortie de `/panier`** | Navigation complète (hors tunnel) → nouveau chargement de page → rendu dock normal immédiatement, sans transition inter-page à spécifier. Cas particulier tunnel (Turbo) : voir note de faisabilité §9. |

### 3.2 Mobile (< 640px)

Mêmes règles, offsets alignés sur les valeurs mobiles déjà définies pour `.caw-dock`/`.caw-collapsed`
(`chat-widget.css:1316-1332`) :

| État | Rendu |
|---|---|
| Bulle réduite | 56×56px (cible tactile largement > 44px), `bottom: 0.75rem; left: 0.75rem`. |
| `.caw-collapsed` / `.caw-proactive` en présence de la bulle | Ancrés à gauche au lieu de centrés (`left: 0.75rem`, largeur `calc(100vw - 1.5rem - 4rem)` maxi pour ne pas chevaucher la bulle elle-même horizontalement — au lieu de `left/right: 0.75rem` pleine largeur aujourd'hui), même `bottom` qu'actuellement (`4.75rem`, cf. §3.4). |

Sur mobile le CTA « Commander » est **toujours** le dernier élément de la page (colonne récap
empilée sous le contenu, cf. §1) : c'est le cas où la réserve d'espace bas (§5) est la plus
critique.

### 3.3 Pourquoi pas de badge « panier » sur la bulle

Le dock normal affiche `cart.itemCount` + total (`.caw-dock-cart`) parce qu'ailleurs sur le site
c'est une information que le visiteur n'a pas sous les yeux. Sur `/panier`, le contenu **de la page
elle-même** répond déjà exhaustivement à « qu'est-ce qu'il y a dans mon panier ? » : dupliquer ce
chiffre sur la bulle est un rappel redondant qui ajoute du bruit visuel sans valeur informative
(Occam's Razor). La bulle sur `/panier` porte donc un seul signal, différent : une suggestion
proactive non lue (§3.4), pas le compteur panier.

### 3.4 Suggestion proactive avec bulle réduite (lien avec AC2)

- `.caw-proactive` garde son mécanisme d'affichage actuel (`x-show="proactive && !isOpen"`,
  transitions déjà en place) mais son ancrage passe de centré à gauche sur `/panier`
  (`left: 1.25rem` desktop / `0.75rem` mobile, au lieu de `left: 50%; transform: translateX(-50%)`),
  aligné avec la bulle en dessous. Offset vertical inchangé : `bottom: 5.25rem` desktop
  (`chat-widget.css:1391`) / `4.75rem` mobile — l'empreinte de la bulle (56px) est proche de celle
  de la barre actuelle (~54px), ces valeurs restent valables sans recalcul.
- Largeur de la carte proactive plafonnée à ~360px sur `/panier` (au lieu de `min(620px, 100vw -
  2rem)`) : elle reste un signal secondaire à côté de la bulle, jamais une seconde barre pleine
  largeur qui reproduirait le problème initial un cran plus haut.
- Tant que la carte proactive est visible (dépliée), la bulle sous-jacente affiche en plus un petit
  point animé (réutilise l'animation `caw-proactive-pulse` existante, `chat-widget.css:1581-1587`,
  déclenchée une fois, pas de boucle) — pour que le signal reste perceptible même si le visiteur
  fait défiler la carte hors du champ visuel ou la ferme sans y répondre. Annonce lecteur d'écran
  associée : nouvelle clé `newSuggestionAvailable` (§7).
- Fermeture (« Non merci » / `dismissProactive()` / `Échap`) : comportement strictement inchangé,
  la carte disparaît (transition `leave` existante, 0.15s ease-in), le point animé sur la bulle
  s'efface avec elle.
