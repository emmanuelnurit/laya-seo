6. Accessibilité (AC4)


- **Cible tactile** : bulle 56×56px — au-delà du minimum 44×44px (WCAG 2.5.5 / cible tactile).
- **`aria-label`** : `i18n.openAssistant` sur la bulle fermée (clé existante). Une fois ouverte,
  reprend le `aria-label` déjà utilisé sur le panneau plein (`{{ assistantName }}`, inchangé).
- **`aria-expanded`** : `false` sur la bulle quand `state !== 'open'`, `true` sinon — même logique
  binaire que le bouton d'ouverture actuel, portée sur le nouvel élément bulle.
- **`aria-haspopup="dialog"`** sur la bulle (le clic ouvre un panneau modal plein écran — cohérent
  avec le rôle déjà implicite de `.caw-shell`).
- **Focus visible** : anneau repris tel quel de `.caw-quickreply:focus-visible`
  (`outline: 2px solid var(--caw-strong); outline-offset: 2px`) — pas de nouveau style de focus à
  inventer.
- **Clavier** : la bulle est un `<button>` natif, donc focusable/activable au clavier par
  construction (Tab + Entrée/Espace). **Échap** : comportement déjà en place et suffisant —
  `dismissProactive()` sur `keydown.escape.window` quand la carte proactive est visible et le
  widget n'est pas ouvert (`chat_widget.html.twig:13`), `close()` sur Échap quand le panneau plein
  est ouvert (`chat_widget.html.twig:146`). Rien de nouveau à spécifier pour la bulle elle-même :
  elle n'est pas une surface dismissible, c'est un bouton.
- **`prefers-reduced-motion`** : voir §4.
- **Contraste** : la bulle reprend `--caw-strong` (`#282828`) sur fond clair — déjà validé ailleurs
  dans le composant (barre AA de MYO-210/211 mentionnée dans les tokens CSS) ; aucune nouvelle
  combinaison de couleur introduite.
