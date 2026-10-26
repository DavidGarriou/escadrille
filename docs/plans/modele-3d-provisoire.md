# Plan — Modèle 3D provisoire du SPAD VII

- Statut : **validé** par le joueur le 2026-10-25, avec les quatre options recommandées : `gltf` et `gltf-json` pour les outils, champ `dimensions` et fichier `maquette.ron`, outil `tools/mockup`, plan entier
- Contexte : étape 8 du parcours `/nouvel-avion spad7`. Le modèle provisoire servira au M2 (premier vol : caméras cockpit et poursuite). Il doit respecter le contrat de `pipeline-assets` et l'ADR 0006 (repères).
- Hors de ce plan :
  - le modèle définitif (Blender, textures, livrées) ;
  - les niveaux de détail (M3), les volumes d'impact et variantes endommagées (M4) ;
  - le poste de pilotage animé.

## Ce que sera le modèle

Des **boîtes et des plaques aux bonnes dimensions**, produites par un outil Rust à partir des données. Blender n'est pas installé dans le conteneur, et un générateur rend le modèle reproductible.

| Nœud | Contenu | Origine des cotes |
|---|---|---|
| `body` | fuselage en boîte effilée, capot, plans supérieur et inférieur, mâts, plan fixe, dérive, train, béquille | `aircraft.ron` (surfaces, positions), `maquette.ron` (fuselage, train : E:) |
| `prop`, `prop_blur` | deux pales et un moyeu ; disque flou. Pivot sur le moyeu, axe local = axe de rotation | `aircraft.ron` (`hub_m`, diamètre) |
| `ctrl_aileron_l_upper`, `ctrl_aileron_r_upper` | ailerons du seul plan supérieur ; pivot sur la charnière, axe local = charnière | `aircraft.ron` (`controls`, part de corde) |
| `ctrl_elevator`, `ctrl_rudder` | profondeur et gouvernail, mêmes règles | `aircraft.ron` |
| `cam_pilot_eye` | œil du pilote | `maquette.ron` (E:) |
| `gun_1_muzzle` | bouche de la Vickers, axe local = axe de tir | `maquette.ron` (E:) |

- Repères : nez vers +Z, aile droite vers −X, haut vers +Y dans le glTF, c'est-à-dire corps (x, y, z) ↦ (−y, −z, x). Origine : le `datum` d'`aircraft.ron` (ADR 0006).
- Matériaux : couleurs unies et distinctes, dont l'aile droite d'une autre couleur que la gauche, pour lire l'orientation sur une capture. Aucune texture.
- Licence : création maison, CC-BY-SA 4.0 (`LICENSE-ASSETS`), inscrite dans `assets/LICENCES.md`.

## Étapes

1. **Données.**
   - Dimensions de référence dans `aircraft.ron`, pour tous les avions : `dimensions: (span, length, height)`, sourcées. SPAD : 7,82 m, 6,08 m (B) et 2,20 m (A), d'après la fiche. **Changement de format, à valider.**
   - Nouveau fichier `assets/aircraft/spad7/maquette.ron`, avec origine de chaque cote :
     - section du fuselage (0,80 m de large, déjà supposé par le modèle de vol ; hauteur E:) ;
     - capot, train, béquille ;
     - œil du pilote, bouche de la Vickers.

   Documentation dans `format-avion.md`.
2. **Conversions sans moteur.** Comme l'ADR 0006 le prévoit, `to_bevy` et `from_bevy` passent dans `escadrille_data` (module `frames`), avec la ligne « corps → glTF ». Le pont du jeu les réutilise : jamais deux copies. Les tests suivent les fonctions.
3. **Générateur `tools/mockup`**, sans Bevy.
   - Il lit `aircraft.ron` et `maquette.ron`, et écrit `assets/aircraft/spad7/model.glb`.
   - Commande : `just mockup spad7`.
   - Tests : dimensions des boîtes, pivots des gouvernes sur leur charnière, sortie déterministe (deux générations identiques à l'octet).
4. **Contrôle `tools/asset_check`**, sans Bevy, lancé par `just test` pour chaque avion qui a un modèle :
   - nœuds obligatoires présents ;
   - gouvernes : pivot sur la charnière d'`aircraft.ron` (à 1 cm près), axe le long de la charnière (à 1° près) ;
   - hélice : pivot sur le moyeu, axe sur l'axe de traction ;
   - boîte englobante comparée aux `dimensions`, à 1 % près ;
   - fichier inscrit dans `assets/LICENCES.md`.

   Tests : le modèle généré passe ; un modèle **en miroir**, **retourné** ou **décalé** échoue. C'est ce qui protège de l'erreur de l'ADR 0006.
5. **Scène de test `maquette-spad7`** pour le mode capture.
   - Le modèle est posé sous une entité au centre de gravité, par `model_transform` et `render_transform`, dans une attitude connue : cap à l'est, inclinaison de 30° à droite.
   - Gizmos des axes du modèle de vol (x rouge vers l'avant, y vert vers la droite, z bleu vers le bas), calculés par `to_bevy`.
   - Hélice tournée d'un angle connu, avec une flèche du sens de rotation.
   - `just capture maquette-spad7`, puis **regarder l'image** :
     - le nez est sur la flèche rouge ;
     - l'aile droite, de couleur distincte, est du côté vert et plus basse ;
     - l'hélice tourne dans le sens horaire vue de derrière.
6. **Documentation et clôture.**
   - `pipeline-assets` (générateur, contrôles) ;
   - `CLAUDE.md`, arborescence : ajout de `tools/mockup` et du rôle d'`asset_check`. **Proposé au joueur, pas modifié d'office** ;
   - carnet, commit, push ;
   - revue de l'aérodynamicien sur les pivots et les axes ; le directeur artistique n'a pas à juger un modèle provisoire.

Chaque étape : `just check`, `just test`, carnet, commit et push.

## Dépendance demandée

| Crate | Version | Licence | Usage | Remarque |
|---|---|---|---|---|
| `gltf-json` | 1.4.1, fonction `names` | MIT ou Apache-2.0 | écrire le glTF (`tools/mockup`) | déjà dans `Cargo.lock` par `bevy_gltf` |
| `gltf` | 1.4.1 | MIT ou Apache-2.0 | lire et valider le glTF (`tools/asset_check`) | idem ; même projet (gltf-rs) |

- Rien de nouveau à compiler ni à vérifier en licence. Seuls les outils en dépendent, pas le jeu.
- Il faut suivre la version de Bevy, pour ne pas compiler deux copies.
- Autre option : écrire le JSON à la main avec `serde_json` (aussi dans `Cargo.lock`). C'est plus de code, et la lecture ne serait plus validée par une bibliothèque qui connaît la spécification. **Non recommandé.**

## Risques

- **Capture dans le conteneur** : rendu logiciel, validé au M0 sur la scène `m0-sol`. Si le chargement d'un glTF y échoue, la scène de test se fera au M2 sur la machine du joueur.
- **Cotes estimées** (fuselage, train, œil, arme) : sans effet sur le modèle de vol. Elles seront remplacées si la demande D4 (`docs/recherche/demandes.md`) aboutit.
- **Chargeur glTF de Bevy** : ne pas activer `GltfConvertCoordinates`, sinon le demi-tour serait compté deux fois (ADR 0006). La scène de test le verrait.

## Décisions demandées au joueur

1. La dépendance `gltf` / `gltf-json` pour les outils.
2. Le nouveau champ `dimensions` d'`aircraft.ron`, et le fichier `maquette.ron`.
3. Le nouvel outil `tools/mockup`, et son ajout à l'arborescence de `CLAUDE.md`.
4. Le plan dans son ensemble.

## Exécution (2026-10-25)

- Étapes 1 à 4 : faites (commits 14f428e et b147874). Le contrat du modèle est calculé par `escadrille_data::model` à partir des bandes du modèle de vol.
- **Deux cotes manquées** par le modèle généré, acceptées provisoirement dans `maquette.ron` (`accepted_deviations`), **en attente de l'accord du joueur** :
  - longueur 6,264 m contre 6,08 m : l'empennage dessiné est l'empennage **équivalent** du modèle de vol (sans flèche, surface conservée) ; son gouvernail finit 0,16 m derrière celui du dessin ;
  - hauteur 2,53 m en ligne de vol, 2,355 m au sol, contre 2,20 m : 2,20 m ne peut pas être mesuré en ligne de vol (dérive et hélice couvrent déjà 2,22 m), et le train du dessin donne 2,30 à 2,36 m au sol. Conflit entre le dessin (C) et les tableaux (A).
- **Point ouvert pour le modèle définitif** (corrigé après la revue de l'aérodynamicien) : sur l'empennage équivalent, l'axe des charnières du contrat est juste (vertical pour le gouvernail, transversal pour la profondeur), mais sa position ne l'est pas. Les points de charnière des bandes penchent (26,8° et 8,0°), et la droite du contrat passe par leur milieu. Un modèle fidèle au dessin aurait pu échouer sur le **pivot** (écart alors estimé à 7 cm sur un étambot supposé à x −4,75 ; la relecture du dessin du 2026-10-26 met l'axe du gouvernail à x −4,70, à 2 cm de la droite des bandes, et l'étambot à x −4,47). Recommandation de l'aérodynamicien : une charnière déclarée, facultative, par gouverne dans `aircraft.ron` (`hinge_m`, sourcée sur le dessin), avec un garde-fou au chargement. **Changement de format, à décider par le joueur.**
- Revue de l'aérodynamicien (pivots et axes) : rien de bloquant ; sens de braquage, axe de l'hélice, repères et capture conformes. Corrigé dans la foulée :
  - le générateur coupe chaque plaque sur la charnière de son nœud, pas sur celle de sa bande (sinon jusqu'à 8,7 cm de jour ou de recouvrement au gouvernail braqué à fond) ;
  - la charnière est prise aux bornes exactes de la gouverne et pondérée par la longueur : le pivot ne dépend plus du nombre de bandes ; la profondeur est groupée par côté ;
  - le pivot est mesuré au segment de charnière, plus à la droite infinie ;
  - gouvernes en violet ; raison de l'écart de hauteur complétée (écrasement statique du train).
- Étape 5 : scène `maquette-spad7` (`crates/game/src/world/maquette.rs`) rendue sous Xvfb (lavapipe) avec le binaire de développement, faute de place disque pour une compilation optimisée. Les trois vérifications passent à l'œil : nez sur la flèche rouge, aile droite verte du côté vert et plus basse, hélice tournée dans le sens horaire vu de derrière.
- Le jeu charge désormais ses assets depuis le dossier `assets/` du dépôt (`ASSETS_DIR`, `crates/game/src/main.rs`) : sous `cargo run`, Bevy les cherchait dans `crates/game/assets`.

## Décisions du joueur (2026-10-26)

1. Les deux écarts (longueur, hauteur) sont **acceptés**, avec leur analyse.
2. La recommandation de l'aérodynamicien est **adoptée** : une charnière déclarée, facultative, par gouverne dans `aircraft.ron` (`hinge_m`), prise telle quelle par le contrat, avec un garde-fou (quart de la corde moyenne de la gouverne, 10°). Faite le même jour ; documentée dans `format-avion.md` et `pipeline-assets`. Les charnières réelles du SPAD restent à lire sur le dessin, avant le modèle définitif.
3. `CLAUDE.md` : `tools/mockup`, le rôle d'`asset_check` et `just mockup` ajoutés.

## Charnières lues sur le dessin (2026-10-26)

- Relecture par l'aérodynamicien (`spad7-calculs.md` § 3.1) :
  - gouvernail : axe vertical à x −4,70 ± 0,03 m, de z −0,31 à −1,29 ;
  - profondeur : charnière droite à x −4,51 ± 0,03 m ;
  - ailerons : la charnière des bandes concorde, pas de `hinge_m`.
- Inscrites dans `aircraft.ron` (`hinge_m`, C:), dans le garde-fou avec de la marge. Modèle régénéré ; fuselage de `maquette.ron` arrêté à l'étambot lu (x −4,47) ; capture refaite et regardée.
- Écarts signalés au joueur, sans rien changer au modèle de vol :
  - partie fixe de dérive absente (0,08 m², −7 % d'empennage vertical) ;
  - ailerons à 29 % au lieu de 32 % ;
  - répartition du gouvernail équivalent ;
  - bord d'attaque du plan fixe à 3,86 m au lieu de 3,78.
