5. API REST vs serveur MCP — recommandation


### 5.1 Constat : un serveur MCP existe déjà dans le module

Contrairement à ce que suggère le libellé de l'étude, **CommerceAgents a déjà un serveur MCP natif** :

- `local/modules/CommerceAgents/Mcp/Server/McpServer.php` : implémente le protocole JSON-RPC MCP (`initialize`, `tools/list`, `tools/call`) **en proxifiant directement le `ToolRegistry` existant** — même tools, mêmes gardes (`ToolContext`), aucune duplication de logique métier.
- `local/modules/CommerceAgents/Command/McpServeCommand.php` : commande console `commerceagents:mcp:serve --admin=<login>`, transport stdio, destinée à des clients MCP externes (Claude Desktop, Claude Code).
- `local/modules/CommerceAgents/Mcp/McpToolCatalog.php` : sous-ensemble de tools exposés (masque `open_admin_page`/`open_page`, navigation non pertinente hors chat).
- Doc utilisateur déjà en place : `local/modules/CommerceAgents/Controller/Admin/McpDocController.php`, template `merchant-chat/mcp.html.twig`.

Ce n'est donc **pas** un choix "MCP natif vs proxy API Platform vs tools-in-module" à trancher ex nihilo : l'architecture **tools-in-module + registre + MCP en façade optionnelle** est déjà celle du projet.

### 5.2 Compatibilité Mistral

Les agents tournent via `Agent/Llm/MistralClient.php`, qui utilise le **function-calling au format OpenAI-compatible** (`body['tools']`, `tool_choice: 'auto'`, `local/modules/CommerceAgents/Agent/Llm/OpenAiMessageConverter.php`). Ce n'est **pas** le protocole MCP : MCP est un protocole client-serveur (JSON-RPC sur stdio/HTTP) utilisé par des *clients* MCP (IDE, Claude Desktop) pour découvrir et appeler des tools — l'API d'inférence Mistral elle-même ne "parle" pas MCP, elle consomme des spécifications de tools au format function-calling classique, que `ToolRegistry::getToolSpecs()` fournit déjà. **Aucune incompatibilité** : le chat marchand/shopping appelle `ToolRegistry` directement (function-calling Mistral), le serveur MCP stdio sert un usage différent — un opérateur humain outillé d'un client MCP (Claude Desktop/Code) qui veut piloter la boutique depuis son IDE.

### 5.3 Avantages / inconvénients

**Tools-in-module (`ToolRegistry`, déjà en place)**
- + Contrôle total sur la logique métier (staging, capabilities, i18n des résultats)
- + Un seul chemin d'exécution à sécuriser et auditer
- + Déjà éprouvé (21 tools, tests `ToolRegistryTest`, `CapabilityFilteringTest`)
- − Ne profite pas de l'écosystème MCP (découverte par des clients tiers) sans la façade dédiée

**Serveur MCP en façade du même registre (déjà en place)**
- + Zéro duplication : mêmes tools, mêmes gardes, juste un transport JSON-RPC supplémentaire
- + Ouvre l'usage à des clients MCP génériques (Claude Desktop/Code) pour un opérateur humain
- − **Le garde-fou d'authentification est cassé** : `--admin=<login>` sans mot de passe contourne JWT + `AdminApiPermissionListener` (section 2.5) — c'est un vrai risque de sécurité, pas un problème d'architecture MCP en soi
- − Pas d'audit log des actions passées par ce chemin

**Proxy MCP direct sur l'API Platform (hypothétique, non construit)**
- + Réutiliserait les 121 ressources déjà exposées, y compris celles sans tool dédié (ex. `OrderCoupon`, profil client arbitraire)
- − Redonnerait accès brut aux ressources d'écriture dangereuses (`Module.activate`, `Config.value`) sans le filtrage `Capability`/staging que `ToolRegistry` apporte aujourd'hui
- − Double le travail de sécurisation (JWT + permissions admin fines + nouveau mapping MCP), pour un existant fonctionnel

### 5.4 Recommandation

**Garder l'architecture actuelle (tools-in-module + `ToolRegistry` + MCP en façade).** Ne pas construire de second serveur MCP, ni de proxy MCP direct sur l'API Platform. Deux actions concrètes prioritaires, indépendantes du choix d'architecture :

1. **Corriger l'authentification du CLI MCP** (`McpServeCommand`) : `--admin=<login>` sans vérification de mot de passe/jeton est le vrai trou de sécurité identifié dans cette étude, pas un problème de "REST vs MCP". À traiter avant toute extension du périmètre MCP (voir roadmap, lot 1).
2. **Étendre les capacités via de nouveaux tools dans `ToolRegistry`**, pas via un nouveau canal — chaque nouveau besoin métier (section 4) doit passer par le même registre, avec sa `Capability` dédiée et, pour les écritures sensibles, le même mécanisme `StagedChangeManager` que `update_price`/`update_stock`.

---
