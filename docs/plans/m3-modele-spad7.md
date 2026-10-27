# Plan — Modèle 3D réaliste du SPAD VII

- Statut : **validé** par le joueur le 2026-10-26 :
  - plan complet, partie A comprise ;
  - Blender piloté par script (ADR 0007) ;
  - livrée : l'avion réel le mieux documenté, à choisir sur la fiche de l'historien ;
  - premier poste de pilotage.
- Demande du joueur (2026-10-26) : « Je suis d'accord avec les suggestions. Réalise un modèle 3D le plus réaliste possible, avec une excellente qualité de rendu. »
- **Jalons** : c'est l'élément « Modèle PBR du premier avion » du **M3**, avancé alors que le M1 n'est pas clos et le M2 pas commencé. Le modèle se verra d'abord dans les scènes de capture, puis dans les caméras du M2.
- Lecture de « les suggestions » (à confirmer) : les trois points encore ouverts du dessin de 1917 (`spad7-calculs.md` § 3.1) :
  - ailerons à 29 % de la corde ;
  - empennage vertical réparti comme dessiné ;
  - moyeu à x 0,83.

  Ils passent dans le modèle de vol avant le modèle 3D, qui doit avoir la même géométrie.

## Partie A — Le modèle de vol suit le dessin

Données seulement, sans code du modèle de vol. Chaque point : valeurs calculées par l'aérodynamicien sur le contour numérisé, campagne `just essai spad7` avant et après, revue.

1. **Ailerons** : `chord_ratio` de 0,32 à 0,29 (dessin : 27 à 31 %). Environ −4 % d'efficacité en roulis attendu.
2. **Empennage vertical tel que dessiné** :
   - panneau inférieur effilé, qui suit l'arrondi du gouvernail ;
   - panneau supérieur ramené à la surface dessinée au-dessus du plan fixe (0,73 m²).
   - **La répartition du gouvernail** : sur le dessin, sa corde est presque constante (0,55 m) ; dans le modèle, elle va de 0,74 à 0,18 m. Le chargeur oriente les bandes selon le quart de corde, ce qui interdit une dérive en flèche. L'aérodynamicien proposera le découpage en panneaux qui reproduit le mieux la surface du gouvernail et la hauteur de son centre, dans ces contraintes. Un seul `chord_ratio` par panneau, donc plusieurs panneaux si nécessaire.
3. **Moyeu** : la lecture rapide donne x 0,83 au lieu de 0,78. À confirmer par une vraie mesure avant de changer `hub_m`, qui touche l'effet P et le souffle.

## Partie B — Le modèle 3D réaliste

### B1. Références (historien)

- **Description visuelle détaillée du S.VII**, dans une fiche `docs/recherche/spad7-aspect.md`, sourcée. Rien n'est inventé sans l'étiquette `ESTIMÉ`. Elle couvre notamment :
  - capot aluminium, radiateur circulaire et ses volets, persiennes, bossages des arbres à cames, échappements ;
  - cabane, mâts et leurs ferrures ; haubans doublés, câbles d'atterrissage et d'incidence ;
  - renvois d'ailerons extérieurs, guignols, câbles de gouvernes ;
  - pare-brise, ouverture du poste, bourrelet ;
  - Vickers et son support ;
  - roues avec voiles, pneus, sandows, béquille ;
  - nombre et espacement des nervures, rubans, coutures, trappes de visite ;
  - hélice Éclair, forme de pale.
- **Photos et musées** (S.VII conservés) : cités comme références, avec leurs droits ; aucune reproduction n'entre dans le dépôt.
- **Lacunes** : elles vont dans `docs/recherche/demandes.md`, en particulier D4 (plans du Musée de l'Air).

### B2. Livrée (historien, décision du joueur)

Un avion précis, à une date précise.
- Couleurs du camouflage 1917, dessous, cocardes (diamètres, positions), bandes du gouvernail, numéro de série, insigne d'escadrille.
- Chaque teinte et chaque cote sourcée ou `ESTIMÉ`.
- Les livrées sont des couches de décalques : on pourra en ajouter d'autres sans toucher au modèle.

### B3. Outil (décision du joueur)

- **(1) Blender, piloté par script** (recommandé).
  - **Installation** : le paquet Ubuntu 4.0.2 est disponible dans le conteneur ; blender.org est bloqué par le proxy.
  - **Construction** : un script Python (`art/spad7/build.py`) construit le modèle à partir des données (`aircraft.ron`, fiche d'aspect, dessin). Il enregistre le `.blend` dans `art/spad7/` et exporte `assets/aircraft/spad7/model.glb`. Le modèle est reproductible ; tu peux ouvrir le `.blend` sur ta machine et retoucher à la main.
  - **Ce que Blender apporte** : surfaces de subdivision, chanfreins, dépliage UV, et surtout le **précalcul Cycles** : occlusion ambiante, normales des rubans et des coutures depuis une version détaillée, épaisseur de la toile pour la transmission.
  - **Ce qu'il faut accepter** : du Python dans le dépôt, alors que `CLAUDE.md` dit « Rust uniquement ». C'est une exception, limitée aux scripts de contenu dans `art/`, jamais dans le jeu ni dans les outils. ADR et ligne dans `CLAUDE.md`.
  - **Fichiers volumineux** : `.blend` et textures sources passent par Git LFS (convention de `pipeline-assets`), à mettre en place.
- **(2) Tout en Rust** : `tools/mockup` devient un modeleur procédural.
  - **Ce qu'on garde** : pas d'exception, sortie déterministe, données en entrée.
  - **Ce qu'on perd** : beaucoup plus de code à écrire (subdivision, dépliage UV, précalcul) pour un résultat moins bon sur les formes arrondies (capot, carénages, saumons).
  - **Ce qu'il faut ajouter** : une dépendance pour écrire les textures (`png` ou `image`), à demander.

### B4. Géométrie

- **Budget** de `pipeline-assets` : 50 000 à 150 000 triangles pour le niveau de détail le plus fin, poste compris.
- **Ce qui est modélisé** :
  - **ailes** : profils réels (fiche `profil-spad7.md`), saumons arrondis, découpe du plan supérieur au-dessus du poste ;
  - **fuselage** : section arrondie sur les quatre faces, lofté sur les 14 stations du dessin ;
  - **empennages** : contour réel, avec l'échancrure en V de la profondeur ;
  - **capot et radiateur**, avec leurs détails (persiennes, bossages, échappements) ;
  - **mâts profilés, haubans, train et roues**, Vickers ;
  - **hélice** : pales vrillées, disque flou.
- **Contrat** : mêmes nœuds qu'aujourd'hui, plus `wire_*` pour les câbles. Les charnières viennent de `hinge_m` et des bandes.
- **`asset_check`** :
  - il doit passer sans écart accepté de longueur : le contour réel donne environ 6,10 m, à 0,3 % des 6,08 m ;
  - **la hauteur reste en écart** (2,30 m au sol contre 2,20 m) ;
  - les écarts acceptés vivent aujourd'hui dans `maquette.ron`, propre au modèle provisoire. Ils iront dans un fichier de description du modèle définitif.
- **Poste de pilotage** : voir B7.

### B5. Matériaux et textures

- **Toile enduite** :
  - transmission diffuse, modulée par une carte d'épaisseur (nervures, rubans, coutures) : à contre-jour, la structure apparaît ;
  - toile peinte (dessus camouflé) et toile écrue enduite (dessous) distinctes.
- **Bois vernis** (hélice, mâts) : vernis et veinage.
- **Aluminium du capot** : peint ou brut, selon la fiche.
- **Autres** : acier des ferrures, caoutchouc, cuir du bourrelet.
- **Textures** :
  - 4K pour la cellule, 2K pour les détails ;
  - PNG d'abord ; compression KTX2 ensuite (KTX-Software, à installer, open source) ;
  - textures sources générées ou précalculées dans l'outil retenu, sans aucune texture tierce, ou alors inscrite dans `assets/LICENCES.md`.

### B6. Rendu dans le jeu

- **Fonctions de Bevy 0.20** (vérifiées dans les sources) : transmission diffuse avec texture, vernis et anisotropie (extensions glTF lues), décalques en grappes.
  - Plusieurs demandent des features Bevy hors défaut : `pbr_transmission_textures`, `pbr_anisotropy_texture`, `pbr_clustered_decals`, `ktx2`. Leur activation est à consigner dans l'ADR des versions et features (ADR 0004).
- **Câbles plus fins qu'un pixel** : technique « phone-wire AA » du skill `rendu-visuel`, sans quoi les câbles scintillent. Petit shader WGSL.
- **Scènes de référence** :
  - `maquette-spad7` est gardée pour les repères ;
  - `avion-studio` : trois quarts avant, soleil, atmosphère de base, ombres, anticrénelage temporel ;
  - `contre_jour_toile` (déjà listée dans `rendu-visuel`).

### B7. Poste de pilotage (périmètre, décision du joueur)

- **Recommandé** : un premier poste, dont la caméra cockpit du M2 a besoin. Il comprend le siège, le manche, le palonnier, la planche de bord avec les instruments du S.VII d'après la fiche (cadrans en décalques), la Vickers vue de l'intérieur et le pare-brise.
- **Plus tard** : aiguilles animées, détails fins.

### B8. Validation

- `asset_check` et `just test` verts.
- Captures des scènes de référence, **images regardées** à chaque étape.
- Comparaison avec les photos d'époque, en référence seulement.
- Revue du **directeur artistique**.
- Revue de l'**aérodynamicien** sur les charnières et les axes.
- **Performances** : à mesurer sur ta machine (RX 7800 XT). Le conteneur rend en logiciel : il juge l'image, pas la vitesse.

## Ordre

1. Partie A, avec essais et revue.
2. B1 et B2 en parallèle (historien).
3. B3 : installation de l'outil retenu, ADR.
4. B4 et B5 par passes : forme générale, puis détails, puis matériaux et textures, puis livrée. Captures regardées après chaque passe.
5. B6 : fonctions de rendu et scènes.
6. B7 : poste de pilotage.
7. B8 : revues, docs, carnet.

Chaque étape : `just check`, `just test`, carnet, commit et push. C'est un chantier de plusieurs sessions. Une étape terminée laisse toujours un modèle utilisable.

## Risques

- **Disque** : il reste environ 3,7 Go ; Blender et les textures demandent de la place. Prévoir le ménage dans `target/`.
- **Rendu logiciel dans le conteneur** : lent, mais l'image est juste (validé au M0). Les effets coûteux se jugent sur ta machine.
- **Données d'aspect lacunaires** : sans les plans du Musée de l'Air (D4), certaines formes seront `ESTIMÉ` (capot, ferrures, poste).
- **Hauteur** : l'écart au sol de 2,30 contre 2,20 m restera tant que le train ne sera pas mieux connu.

## Décisions demandées au joueur

1. Le plan, et la lecture de « les suggestions » (partie A).
2. L'outil : (1) Blender piloté par script, avec une exception Python limitée à `art/` ; ou (2) tout en Rust.
3. La livrée : quel avion, à quelle date. L'historien proposera les mieux documentés.
4. Le périmètre du poste de pilotage.

## Décisions du joueur (2026-10-26, suite)

- **Livrée (B2)** : S.254 « Vieux Charles » dans son état du 20 octobre 1917, en deux couches de décalques (finition d'usine, puis marques de Guynemer). Moteur de 180 ch accepté.
- **Usage réservé** : studio, as de l'IA, livrée historique au choix ; l'avion du joueur prend la finition d'usine avec un numéro fictif.
- **Partie A** : faite, revue de l'aérodynamicien conforme.

## Avancement

- **B4** (2026-10-26) : quatre passes de forme et de détails ; le modèle Blender est dans les assets.
- **B5, première passe** (2026-10-27) : toile peinte dans deux textures 4K générées par le script, couleur et normales, sur un atlas de projections planes.
  - Peints : la finition d'usine de 1917 (`livrees.ron`), avec les cocardes des ailes et le gouvernail tricolore ; les rubans et coutures des nervures, les fausses nervures et le creux de la toile entre les nervures.
  - **Reste en B5** :
    - les marques de S.254 (cigogne, « 2 », « Vieux Charles », « S 254 »), en seconde livrée ; la recherche est faite (`spad7-aspect.md` § 12.6) ;
    - le bois et le métal ;
    - la rugosité.
  - **Mipmaps** (début de B6, 2026-10-27) : Bevy ne les génère pas pour les images d'un glTF. Le jeu les calcule au chargement, avec un filtrage anisotrope ; les KTX2 faites hors ligne restent prévues.
- **Toile translucide** (2026-10-27) : la transmission est peinte dans l'alpha de la couleur, et le jeu l'applique (feature `pbr_transmission_textures`). Scène `contre_jour_toile`.
- **B6** (2026-10-27) :
  - mipmaps calculées au chargement ;
  - scène `avion-studio` et choix du tonemapping ;
  - anticrénelage des câbles.

  Restent la compression KTX2 et les décalques en grappes, s'ils servent.
