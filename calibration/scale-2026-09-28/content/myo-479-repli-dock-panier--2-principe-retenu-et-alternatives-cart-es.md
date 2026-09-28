2. Principe retenu (et alternatives écartées)


| Option | Verdict | Pourquoi |
|---|---|---|
| Masquer complètement le dock sur `/panier` | ❌ écarté | Régression de découvrabilité explicitement interdite par le ticket ; l'assistant est aussi le point d'entrée du parcours opt-in newsletter (§8). |
| Réduire à une bulle bas-**droite** (convention Intercom/Drift) | ❌ écarté | Le CTA est *lui-même* ancré à droite (bloc `flex justify-end`, colonne récap à droite en desktop) — une bulle bas-droite se retrouve juste à côté ou au-dessus du CTA : deux actions concurrentes dans le même coin, à l'endroit précis où l'attention doit converger sur une seule (viole le principe des CTA concurrents / Hick's Law au pire moment du tunnel). |
| Garder la barre pleine largeur mais l'amincir | ❌ écarté | Une barre reste une barre : même réduite en hauteur, sa largeur (jusqu'à 620px / quasi 100vw mobile) continue de traverser tout le bas de l'écran et de couper visuellement le CTA de son contexte. |
| **Bulle bas-gauche + réserve d'espace bas sur la colonne récap** | ✅ retenu | Sépare physiquement les deux actions (bulle vs CTA) dans deux coins opposés, **et** garantit par construction (padding réservé) qu'aucune longueur de panier ne peut faire chevaucher le CTA, sans mesure JS de collision au runtime. |

Le renoncement à la convention « chat bas-droite » est assumé et documenté (§10, sacrifice n°1) :
c'est un choix *spécifique à `/panier`*, pas un changement global de position du dock.

Important : **aucun nouvel état n'est ajouté à la machine à états existante** (`dock` / `collapsed`
/ `open`, déjà implémentée dans `chat-widget.js:168-174` et `minimise()`/`close()`/`expand()`). Le
contexte « page panier » est un **flag d'affichage orthogonal** qui change uniquement le *rendu* de
l'état `dock` (et, par cohérence, l'ancrage de `.caw-collapsed` et `.caw-proactive`) — pas la
logique métier. Voir §9 pour la piste de détection déjà repérée dans le ticket.
