Post-V1, suite (MYO-416 → MYO-426)


Le module passe de **0.4.0** à **0.4.1**. Rappel MYO-408 : chaque ticket ci-dessous est vérifié individuellement, pas seulement la présence globale du chapitre — c'est précisément le défaut que ce lot corrige (le tag `v0.4.0` publié était 6 commits en retard sur le code réellement livré, sans qu'aucun de ces commits ne soit journalisé).

### Branche `main` publique alignée sur `myorg` — MYO-416, MYO-423, MYO-426

- Le tag `v0.4.0` publié pointait sur un commit resté sur `myorg` sans jamais être répercuté sur `origin/main`, la branche par défaut du dépôt public : cloner le dépôt sans préciser de branche donnait un code plus vieux que ce que le tag laissait supposer (récidive MYO-415/420).
- `scripts/auto-push-myorg.sh` fast-forward désormais `origin/main` sur `origin/myorg` automatiquement après chaque push réussi de `myorg`, dès lors que `main` est un ancêtre strict de `myorg` (refspec de push normal, jamais `--force` ; en cas de divergence, un avertissement explicite liste les deux SHA et n'exécute aucun push) — extension du garde-fou existant MYO-372 plutôt qu'un 5ᵉ mécanisme séparé (MYO-426). `scripts/check-unpushed-myorg.sh` détecte aussi cet écart en lecture seule, indépendamment de tout commit local en attente.
- Suppression de la note d'installation et de la puce « Known limitations » du README qui renvoyaient encore à `git checkout myorg` (MYO-416) : ce détour n'a plus lieu d'être une fois `main` tenu à jour automatiquement.

### Repli i18n vérifié en conditions réelles — MYO-418, MYO-419

- Le repli vers l'anglais des locales non couvertes par `I18n/` (4 des 21 locales du BO) a été vérifié par appel HTTP réel — pas par lecture de code — sur `de_DE`, `cs_CZ` et `nl_NL` : les écrans Agents IA et configuration du module retombent proprement sur un texte lisible, sans clé brute ni texte vide (MYO-419, ticket enfant de MYO-418). Décision produit : ne pas traduire les 17 locales manquantes, le sujet portait uniquement sur la qualité du repli.
- Découverte au passage : deux libellés (« URL du webhook entrant », « adresse e-mail destinataire ») étaient codés en dur en français dans les connecteurs de canal `AbstractWebhookChannelConnector`/`MailChannelConnector`, donc affichés en français même sous `en_US`/`de_DE`. Corrigés pour passer par `Translator::trans()`, avec mise à jour des 4 catalogues existants (`fr_FR` garde le texte français d'origine).
- Ce correctif a rendu `Translator` obligatoire dans les constructeurs de ces connecteurs, ce qui a cassé 17 des 19 tests PHPUnit de 6 fichiers de test instanciant encore l'ancienne signature — régression détectée en revue et corrigée avant clôture (nouvelle instanciation avec `Translator`, convention déjà en usage ailleurs dans le module). Suite complète rejouée : 674 tests, 1829 assertions, 0 échec. MYO-418 (parent) clos après cette double vérification.

### Wizard multi-fournisseur pour les agents paramétrables — MYO-417, MYO-421

- Deux bugs corrigés dans le même flux : le picker de modèles de l'assistant de création n'affichait que les modèles du fournisseur actif du module (jamais « tous les fournisseurs »), et la sauvegarde de l'agent réécrivait systématiquement son fournisseur sur `mistral` dès qu'un modèle était choisi — un agent créé sur un modèle Anthropic ou compatible OpenAI s'exécutait donc silencieusement avec le mauvais fournisseur, ou échouait faute de clé Mistral.
- Le picker liste désormais tous les fournisseurs disposant d'une clé API configurée (plus le fournisseur actif du module par défaut, pour ne jamais afficher un picker vide avant configuration), poste une valeur composite `provider:modelId`, et affiche les prix dans la devise propre à chaque fournisseur (EUR pour Mistral, USD pour les autres) au lieu d'une devise unique. 37 tests / 117 assertions ajoutés ou mis à jour, suite verte.
- Limite documentée en contrepartie (AC5 du ticket) : le pied du picker de modèles restait câblé sur « Mistral rates (EUR) as of... » alors que plusieurs fournisseurs sont désormais listés — corrigée séparément par MYO-424 ci-dessous, pour ne pas mélanger la correction fonctionnelle et un correctif i18n à travers 4 locales dans le même ticket.

### Libellé du pied du picker de modèles rendu générique — MYO-424

- Le pied du picker de modèles restait câblé sur « Mistral rates (EUR) as of... » même pour un modèle d'un autre fournisseur sélectionné — le résidu documenté par MYO-421 ci-dessus. Nouvelle clé i18n générique (sans mention de devise fixe), traduite dans les 4 locales du module (`en_US`, `fr_FR`, `es_ES`, `it_IT`) ; la devise par option de modèle, déjà correcte, est inchangée.
- Vérifié en HTTP réel sur `/admin/module/CommerceAgents/agents/{new,1/edit}` avec un modèle Mistral puis un modèle Anthropic sélectionnés : même texte correct dans les deux cas. Suite PHPUnit du module toujours au vert (674 tests / 1829 assertions). Limitation correspondante retirée du README.

### Migrations

Aucun fichier `Config/update/0.4.1.sql` : `Config/schema.xml` est identique entre `v0.4.0` et ce lot (`git diff --stat v0.4.0..HEAD -- Config/schema.xml` ne retourne rien) — seuls des scripts d'exploitation (fast-forward `main`), des correctifs i18n/BO et de la documentation sont livrés.
