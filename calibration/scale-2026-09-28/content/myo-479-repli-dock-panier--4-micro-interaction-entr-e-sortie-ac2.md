4. Micro-interaction : entrée / sortie (AC2)


Réutilisation stricte des tokens de transition déjà définis dans `chat-widget.css` pour la carte
proactive (`0.22s ease-out` en entrée, `0.15s ease-in` en sortie) — pas de nouveau timing inventé :

- **Barre → bulle** (arrivée sur `/panier`, ou retour à l'état `dock`/`collapsed` depuis `open`) :
  fondu-enchaîné 0.22s ease-out, `opacity 0 → 1` + léger `scale(0.85) → 1)` sur la bulle entrante.
  Pas de « saut » de position : la bulle apparaît directement à sa position finale bas-gauche (pas
  d'animation de translation depuis le centre — ce déplacement horizontal serait distrayant sans
  ajouter d'information).
- **Bulle → ouverture** (clic/tap/Entrée) : reprend exactement la transition existante de l'overlay
  (`.caw-backdrop`/`.caw-shell`, `x-transition.opacity`), aucun changement.
- **`prefers-reduced-motion: reduce`** : comme pour `.caw-proactive-enter/leave` déjà (règle
  `chat-widget.css:1591-1604`), toutes les transitions ci-dessus tombent à `opacity 0.01s linear`,
  sans transform ni scale, et le pulse du point de notification est supprimé (affichage statique du
  point, sans animation).
