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
