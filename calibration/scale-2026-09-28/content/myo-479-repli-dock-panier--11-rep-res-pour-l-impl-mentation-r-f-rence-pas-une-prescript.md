11. Repères pour l'implémentation (référence, pas une prescription)


| Élément | Fichier |
|---|---|
| Bloc Dock / Collapsed / Proactive (Twig) | `local/modules/CommerceAgents/templates/theme-hook/chat_widget.html.twig` |
| Styles dock/collapsed/proactive | `local/modules/CommerceAgents/templates/frontOffice/default/assets/css/chat-widget.css` (dock : lignes 85-160 ; collapsed : 162-261 ; proactive : 1370-1608 ; responsive mobile : 1316-1336) |
| État `dock`/`collapsed`/`open`, `minimise()`/`close()`/`expand()` | `local/modules/CommerceAgents/templates/frontOffice/default/assets/js/chat-widget.js:168-292` |
| `checkoutUrl` déjà disponible côté widget | `local/modules/CommerceAgents/Hook/Theme/ChatWidgetThemeHook.php:86,153` |
| Page panier / CTA « Commander » | `templates/frontOffice/flexy/checkout-cart.html.twig`, `templates/frontOffice/flexy/checkout-base.html.twig:45-73`, `templates/frontOffice/flexy/components/Organisms/NextButton/Base.html.twig` |
| Grille panier/récap (breakpoints) | `templates/frontOffice/flexy/assets/styles/layout.css:38-64` |
| Catalogues i18n module (4 locales) | `local/modules/CommerceAgents/I18n/{en_US,fr_FR,es_ES,it_IT}.php` |
| Newsletter opt-in (existant, ne pas dupliquer) | `local/modules/CommerceAgents/Service/NewsletterOptinScenarioResolver.php` |
