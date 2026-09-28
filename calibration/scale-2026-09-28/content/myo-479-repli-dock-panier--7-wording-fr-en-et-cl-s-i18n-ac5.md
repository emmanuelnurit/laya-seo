7. Wording FR/EN et clés i18n (AC5)


Principe : réutiliser l'existant en priorité (règle du chantier — voir aussi *Reach for what exists
first*). Après revue, une seule chaîne est réellement nouvelle :

| Clé (source EN, `I18n/en_US.php`) | EN | FR (`I18n/fr_FR.php`) | Usage |
|---|---|---|---|
| `New suggestion available` *(nouvelle clé)* | New suggestion available | Nouvelle suggestion disponible | Texte annoncé aux lecteurs d'écran (`aria-live="polite"`, visuellement masqué) quand le point de notification apparaît sur la bulle réduite (§3.4). |

Chaînes **réutilisées sans modification** (aucune nouvelle clé) :

- `openAssistant` → *"Open the shopping assistant"* / *"Ouvrir l'assistant d'achat"* — sert à la
  fois d'`aria-label` de la bulle et de libellé visible au survol desktop (§3.1).
- `assistantName` — variable de config (nom de l'assistant), pas une clé i18n figée, déjà utilisée
  partout ailleurs dans le widget.

Les libellés es_ES/it_IT suivent le même schéma que les clés existantes (mêmes fichiers
`I18n/es_ES.php` / `I18n/it_IT.php`, 4 locales couvertes) ; à charge de l'implémentation d'ajouter
la traduction ES/IT en miroir de la ligne FR ci-dessus, comme pour toute autre clé du module.
