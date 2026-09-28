Post-V1 (MYO-297 → MYO-348)


Cinq migrations de schéma (`Config/update/0.3.4.sql` → `0.3.8.sql`) portent le module de **0.3.3** à **0.3.8**. Ce chapitre couvre tout ce qui a été livré depuis la clôture de la V1 ci-dessus.

### Panneau de canaux centralisé et connecteurs dédiés Mattermost/Slack — MYO-300, MYO-332

- Remplace le connecteur `webhook` générique unique par trois connecteurs dédiés et autonomes : `Channel/Connector/MailChannelConnector.php`, `MattermostChannelConnector.php`, `SlackChannelConnector.php` (plus `AbstractWebhookChannelConnector.php` pour la base commune Mattermost/Slack).
- Nouvel onglet « Canaux » de la configuration du module (`Controller/Admin/ChannelsConfigController.php`, routes `commerceagents_channels_test` / `commerceagents_channels_save`) : réglages saisis une fois pour toute la boutique par connecteur (adresse d'expédition mail, URL de webhook Mattermost/Slack…), testables sans enregistrer, persistés chiffrés (`ChannelConnectorConfigService`, toujours sodium `secretbox`).
- L'étape « Canaux » de l'assistant de création d'agent (`_step-channels.html.twig`) ne fait plus que cocher les connecteurs à utiliser pour cet agent ; elle renvoie vers ce panneau centralisé pour le réglage détaillé (lien « Configure channel connectors »).
- Correctif MYO-332 : `store_email` vide ou absent devient un `ChannelException` explicite (au lieu d'un « From » silencieusement absent laissé au transport), à la fois dans `send()` et dans `test()` — le connecteur mail refuse d'envoyer sans expéditeur configuré.

### Presets de catalogue d'agents par spécialité — MYO-301, MYO-326

- Le créateur d'agent propose désormais **six** presets (`Service/AgentPresets.php`), deux de plus que les quatre de la V1 : `STOCK_WATCH_RESTOCK` (surveillance de stock avec proposition de réassort) et `CUSTOMER_REVIEWS_REPLY` (réponse aux avis clients).
- Le preset `CUSTOMER_REVIEWS_REPLY` a été reformulé (4 locales) pour ne plus promettre une publication automatique de la réponse — l'agent reste au stade de proposition à valider, jamais de publication directe (MYO-348).

### Écran d'historique des exécutions — MYO-319, MYO-321, MYO-325

- Nouvel écran dédié à l'historique des `agent_run` (`Controller/Admin/AgentRunsController.php`, routes `commerceagents_agents_runs` liste et `commerceagents_agents_runs_show` détail), qui n'existait pas en V1 : le seul repère était alors le badge « Dernière exécution » sur la carte de l'agent.
- Nouvelle page par agent (`commerceagents_agents_show`) avec un aperçu compact des dernières exécutions et un lien « Ouvrir l'historique » vers l'écran complet.

### Panes par spécialité et traçabilité des envois — MYO-327, MYO-328, MYO-334 → MYO-338

- Nouveau socle `Service/SpecialtyPane/` (`SpecialtyResultsPaneRegistry`, `SpecialtyResultsPaneInterface`) : chaque preset a désormais son rendu de résultats dédié sur la page de l'agent — `DailySalesSummaryResultsPane`, `CartAbandonedResultsPane`, `WelcomeNewCustomerResultsPane`, `StockWatchRestockResultsPane`, `CustomerReviewsReplyResultsPane`, et `GenericResultsPane` en repli pour un agent « from scratch » ou dont le preset ne peut pas être résolu sans ambiguïté.
- Nouvelle table `agent_outbound_message` (`Config/schema.xml`, `Model/AgentOutboundMessageQuery.php`) : trace chaque message sortant (canal, destinataire, statut, référence métier, extrait du corps, erreur) rattaché à son `agent_run`/`agent_definition`, base des panes « Propositions à valider » (stock/avis, MYO-336/MYO-338) et « Messages envoyés » (MYO-340).
- Nouvel outil `send_email_to_customer` (`Tool/Admin/SendCustomerEmailTool.php`) : premier chemin d'envoi d'e-mail *au client* (jusqu'ici les agents ne pouvaient écrire que vers les canaux internes mail/Mattermost/Slack côté marchand) — couvre notamment la relance de réponse à avis client (MYO-337/MYO-340).

### Garde-fous des runs automatiques et de l'exécution des agents — MYO-330, MYO-343, MYO-345, MYO-347

- Retrait du garde-fou self-approval de `StagedChangeManager` (MYO-330/MYO-333) : une proposition est toujours créée par un agent, jamais par l'administrateur qui l'approuve, donc bloquer `proposedBy === adminId` n'avait plus de sens.
- Correctif d'un trou majeur : `AgentRunQueue::enqueue()` (déclencheurs `event`/`cron`/`abandoned_cart`/`low_stock`) ne renseigne jamais `admin_id` — seule l'exécution manuelle (« Exécuter maintenant ») le fait. Les quatre `stageXxx()` de `TheliaStagingGateway` (MYO-343) et `TheliaChannelGateway::stageMessage()` (MYO-345) exigeaient `admin_id` en plus de l'identifiant de conversation et refusaient donc silencieusement toute proposition issue d'un run automatique : les panes stock/avis (MYO-336) et le renvoi de rapport restaient vides en usage réel hors clic manuel. `admin_id` reste désormais nullable sur la proposition (distinct de `approved_by`, toujours renseigné par l'admin qui approuve).
- Garde-fou système explicite (`Service/SystemPromptFactory`, MYO-347) : sur refus (capacité manquante) ou échec d'un appel outil, l'agent ne doit jamais deviner la valeur que l'outil aurait renvoyée ni la réutiliser dans un autre appel outil d'écriture — il doit s'arrêter sur cette partie de la mission et signaler précisément quel outil a été refusé et pourquoi.
- Correctif du mappage des déclencheurs du wizard (MYO-344) : `parseTriggers()` écrivait un `type`/`event_name` faux pour cinq des six déclencheurs proposés ; `TriggerCatalogMapping` devient la source unique de vérité entre le formulaire et `agent_trigger`.

### Autres correctifs de fiabilité — MYO-297, MYO-346

- Écran « Agents IA » : le picker de modèles restait vide tant que son `init()` JS n'était pas appelé explicitement (MYO-297).
- Garde-fou anti-désync entre le JS source et le JS publié des étapes du wizard (MYO-346) : l'asset publié pouvait rester obsolète (ancien libellé « canal e-mail/webhook ») après une modification du JS source ; test de garde comparant un hash source/publié.
