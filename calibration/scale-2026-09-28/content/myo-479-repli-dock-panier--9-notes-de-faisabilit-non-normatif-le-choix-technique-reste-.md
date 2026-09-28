9. Notes de faisabilité (non normatif — le choix technique reste à la Tech)


- **Détection « page panier »** : le ticket a déjà repéré la piste la plus simple — `checkoutUrl`
  (passé dans `data-config`, `chat_widget.html.twig:8`) est généré par
  `$this->urlGenerator->generate('checkout_cart')`
  (`Hook/Theme/ChatWidgetThemeHook.php:86`), **qui est précisément la route de `/panier`**
  (`checkout-cart.html.twig` répond à la route `checkout_cart`). Comparer `window.location.pathname`
  au chemin de `checkoutUrl` suffit donc déjà à détecter la page panier, sans nouveau hook ni
  modification du noyau — c'est cohérent avec la contrainte « Plugin rules » du projet. À confirmer/
  trancher par la Tech, cette spec ne l'impose pas.
- **Tunnel Turbo** : `checkout-base.html.twig:18` navigue en Turbo Drive
  (`data-turbo="true"`) entre les étapes du tunnel (panier → livraison → paiement). Si le widget
  Alpine n'est pas réinitialisé à chaque visite Turbo (état persistant au lieu d'un rechargement
  complet), la détection « page panier » doit être réévaluée à **chaque navigation** dans le tunnel,
  pas seulement au chargement initial — sans quoi le retour à l'état bulle→dock normal en quittant
  `/panier` (dernière ligne du tableau §3.1) pourrait ne pas se déclencher. À vérifier en recette
  avant livraison de l'implémentation.
