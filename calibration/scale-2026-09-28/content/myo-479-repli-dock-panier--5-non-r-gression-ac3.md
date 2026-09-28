5. Non-régression (AC3)


**Garde-fou 1 — position.** Bulle en bas-**gauche** du viewport, CTA toujours ancré à droite de sa
colonne (`flex justify-end`, inchangé). Séparation physique systématique.

**Garde-fou 2 — réserve d'espace (le vrai filet).** La position seule ne suffit pas à garantir
l'AC3 dans tous les cas (ex. un panier très court où la colonne récap entière tient au-dessus du
pli) : la colonne récap (`checkout-base.html.twig:53-66`, le conteneur `bg-lightest`) reçoit un
`padding-bottom` additionnel **sur `/panier` uniquement**, égal à l'empreinte de la bulle + marge de
respiration (`56px + 1.25rem` desktop, `56px + 0.75rem` mobile — soit un peu plus de `4.5rem`).
Puisque le CTA est en flux normal (jamais `fixed`), cette réserve garantit par construction que son
bord inférieur reste toujours au-dessus du bord supérieur de la bulle, quelle que soit la longueur
du panier ou la hauteur du viewport — sans mesure de collision au runtime. C'est une modification
CSS côté page panier, pas côté widget : hors du gel du 22/09 (cf. « Périmètre » du ticket).

**Découvrabilité.** La bulle reste un bouton `56×56px` explicitement libellé
(`aria-label`), ouvrable en **un geste** (clic/tap/Entrée) — pas de sous-menu, pas de double
interaction. Elle est toujours visible (jamais masquée par défaut), contrairement à un
« replié dans un menu » qui aurait été une vraie régression de découvrabilité.

**Sacrifice nommé** (explicitement demandé par l'AC3) : la position du dock n'est plus la même sur
`/panier` que partout ailleurs sur le site (bas-gauche au lieu de bas-centre). Un visiteur qui a
pris l'habitude de chercher le dock au centre en bas devra, sur cette seule page, le retrouver dans
le coin gauche. Compensé par : (a) le glyphe/avatar reste visuellement identique (reconnaissance
immédiate), (b) c'est la seule page du site où ce changement s'applique, et (c) c'est le prix
explicitement accepté pour ne jamais risquer de masquer le CTA de conversion.
