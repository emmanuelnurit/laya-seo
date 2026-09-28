1. Constat actuel


- `.caw-dock` (`local/modules/CommerceAgents/templates/frontOffice/default/assets/css/chat-widget.css:87-94`)
  est `position: fixed`, centré (`left: 50%`), `bottom: 1.25rem`, large de `min(620px, 100vw - 2rem)`.
  En mobile (`<640px`, règle `@media (max-width: 639px)` ligne 1316), il devient quasi plein
  écran : `left/right: 0.75rem`.
- Sur `/panier` (route `checkout_cart`, gabarit `templates/frontOffice/flexy/checkout-cart.html.twig`),
  le bouton « Commander » (`twig:Organisms:NextButton:Base`) est rendu dans
  `checkout-base.html.twig:60-64`, à l'intérieur de la colonne récap
  (`bg-lightest`). C'est un élément de **flux normal** (pas `fixed`/`sticky`), aligné à droite
  (`class="flex justify-end"`, `NextButton/Base.html.twig:9`).
- Layout `container-double-1` (`templates/frontOffice/flexy/assets/styles/layout.css:38-59`) :
  - `< lg` (1024px) : une seule colonne, contenu du panier puis récap+CTA **empilés en dessous** —
    le CTA est donc littéralement le dernier élément de la page, exactement là où un dock fixe en
    bas de viewport s'invite quand on scrolle jusqu'au bout.
  - `≥ lg` : grille `58% / 42%`, contenu à gauche, récap+CTA à droite. Pour un petit panier, le bas
    de la colonne récap peut arriver près du bas du viewport, avec le même effet de recouvrement.
- Le dock est donc au bon endroit sur toutes les autres pages (il ne gêne aucun CTA en flux
  fixe) mais entre en collision précisément sur la page dont l'unique but est « lire le récap, puis
  cliquer Commander ».
