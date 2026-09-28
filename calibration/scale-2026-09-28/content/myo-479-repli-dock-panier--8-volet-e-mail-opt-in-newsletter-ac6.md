8. Volet e-mail / opt-in newsletter (AC6)


**Recommandation : ne pas ajouter de nouvel emplacement dédié sur `/panier`. Le point d'entrée
recommandé est le mécanisme de suggestion proactive existant (`.caw-proactive`), positionné comme
en §3.4, sans nouvelle surface.**

Justification :

- Le déclenchement de l'offre newsletter (`NewsletterOptinScenarioResolver`, déjà en dépôt) est déjà
  câblé sur le signal `add_to_cart` et produit un `ProactiveMessage` avec `newsletterOptin: true` —
  c'est-à-dire que la carte MYO-476 (« carte de conversation opt-in ») est déjà conçue pour
  apparaître comme une variante de `.caw-proactive`, pas comme un bloc séparé. Créer un second
  emplacement sur `/panier` dupliquerait un mécanisme qui existe déjà et fonctionnerait en parallèle
  du premier — exactement ce que le ticket demande d'éviter (« ne rien dupliquer »).
- En le laissant dans `.caw-proactive`, le point d'entrée e-mail/opt-in hérite gratuitement de
  toutes les garanties déjà spécifiées ici pour les suggestions proactives sur `/panier` : ancrage
  bas-gauche, largeur plafonnée, jamais au-dessus du CTA, dismissible, respect de
  `prefers-reduced-motion`. Un second emplacement (« dans le dock réduit » ou « ailleurs sur la
  page ») demanderait de respécifier tout cela une seconde fois pour un gain nul.
- Cohérence avec le garde-fou sécurité du ticket (reprise de compte invité, MYO-449/450) : la carte
  proactive est un composant du widget, contextualisé, dismissible, jamais un formulaire injecté à
  même le contenu de la page panier qui pourrait laisser croire à un champ natif du site — réduit le
  risque de confusion entre « je discute avec l'assistant » et « je remplis un formulaire du site ».
  L'implémentation de la carte elle-même (champ e-mail, case décochée par défaut, lien vers
  l'inscription classique) reste entièrement du ressort de MYO-475/476 ; ce ticket ne redéfinit ni
  ne duplique leur contenu.

Si le board souhaite un déclenchement systématique dès l'arrivée sur `/panier` (et non plus
seulement après un `add_to_cart`), c'est un changement de **condition de déclenchement** côté
`NewsletterOptinScenarioResolver`/signal proactif (MYO-475), pas un changement d'emplacement visuel
— cette spec reste valable telle quelle dans les deux cas.
