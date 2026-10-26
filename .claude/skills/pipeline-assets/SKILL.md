---
name: pipeline-assets
description: Production et intégration des assets avec des outils open source uniquement — modèles d'avions Blender exportés en glTF (conventions de nommage, pivots des gouvernes, volumes d'impact, LOD), textures PBR et compression KTX2, livrées, terrain réel (MNT open data, GDAL, QGIS, cartes de 1917), registre des licences. À charger pour toute tâche dans assets/, art/, tools/terrain_bake ou tools/asset_check.
---

# Pipeline d'assets

## Outils (tous open source)

| Besoin | Outil |
|---|---|
| Modélisation, UV, animation, export glTF | Blender |
| Peinture de textures, livrées | Krita, GIMP |
| Matériaux procéduraux | Material Maker |
| Insignes, décalques, UI vectorielle | Inkscape |
| Compression de textures (KTX2, Basis/UASTC) | KTX-Software (`toktx`) |
| Terrain, cartes historiques | QGIS, GDAL |
| Son | Audacity, Ardour |

Les fichiers sources vont dans `art/` (gros fichiers binaires : Git LFS, open source), les exports dans `assets/`.

## Avions : contrat entre le modèle et le code

- Unités : mètres, échelle 1, transformations appliquées.
- **Orientation** (ADR 0006, `docs/adr/0006-reperes-et-conversions.md`) : dans Blender, **nez vers −Y, dessus vers +Z, aile droite vers −X** ; export glTF avec « +Y Up » (option par défaut), transformations appliquées. Le glTF a alors le nez vers +Z, comme le veut la spécification. Le jeu le tourne d'un demi-tour autour de Y (`GLTF_TO_BEVY`, `crates/game/src/flight/bridge.rs`) ; ne pas activer l'option expérimentale `GltfConvertCoordinates` de Bevy, qui compterait ce demi-tour deux fois. **Origine du modèle** : le `datum` d'`aircraft.ron`. Avec le premier modèle, une scène de test vérifie que le nez, l'aile droite et le sens de l'hélice sont ceux du modèle de vol. `asset_check` contrôle chaque export.
- **Nœuds obligatoires** (pivot sur l'axe de rotation réel). Le contrat est calculé depuis `aircraft.ron` par `escadrille_data::model` :
  - **gouvernes** : pivot sur la charnière, **axe local +X le long de la charnière**, orienté pour qu'une rotation positive soit la braque positive du modèle de vol (aileron droit : bord de fuite vers le haut ; profondeur : bord de fuite vers le haut ; gouvernail : bord de fuite vers la droite). Une gouverne qui couvre plusieurs panneaux (le gouvernail du SPAD) n'a qu'un nœud ;
  - **hélice** (`prop`, `prop_blur`) : pivot sur le moyeu, **axe local +X = axe de rotation**, orienté pour que la rotation normale de l'hélice soit positive ;
  - **œil et armes** (`cam_pilot_eye`, `gun_<n>_muzzle`) : regard ou tir vers le **−Z local**, +Y local vers le haut, comme une caméra Bevy.

| Nœud | Rôle |
|---|---|
| `body` | cellule |
| `prop` | hélice (pivot sur l'axe moteur) ; `prop_blur` : disque flou |
| `ctrl_aileron_<l/r>_<upper/lower>`, `ctrl_elevator`, `ctrl_rudder` | gouvernes animées par le FDM |
| `cockpit_stick`, `cockpit_rudder_bar`, `gauge_<nom>_needle` | commandes et aiguilles animées |
| `cam_pilot_eye` | position de l'œil du pilote |
| `gun_<n>_muzzle` | sortie de chaque arme |
| `hit_<composant>` | volumes d'impact (maillages convexes, masqués) |
| `wire_<nom>` | câbles et haubans (rendus avec le shader de câbles) |
| `dmg_<pièce>` | variantes endommagées |
| `<nœud>_LOD1`, `_LOD2`… | niveaux de détail |

- **Dirigeables et ballons** (`assets/airships/<id>/`) : mêmes règles, plus `gas_cell_<n>` (volume de chaque cellule de gaz, masqué), `frame` (structure, visible quand l'enveloppe brûle), `gondola_<n>`, `engine_<n>`, `gunner_<n>` (poste de tir, axe = direction centrale du champ de tir), `ballast_<n>`. LOD poussés : l'objet est visible de très loin.
- **Dimensions** : le modèle respecte la géométrie de la fiche de recherche. `tools/asset_check` compare les boîtes englobantes (envergure, longueur, hauteur) à `aircraft.ron`, à 1 % près. La longueur va du moyeu au point le plus en arrière ; la hauteur est prise dans la plus proche de deux attitudes, ligne de vol ou au sol (sur les roues et la béquille). La source de vérité de la géométrie aérodynamique reste `aircraft.ron`.
- **Charnière du contrat**, pour chaque gouverne :
  - si `aircraft.ron` déclare la vraie charnière (`hinge_m`, voir `format-avion.md`), le contrat la prend telle quelle ;
  - sinon, c'est une droite d'axe parallèle à l'envergure du panneau (verticale pour le gouvernail). Elle passe par le milieu, pondéré par la longueur, des segments de charnière de la gouverne, pris aux bornes exactes `span_from` et `span_to`. Elle ne dépend pas du nombre de bandes.
- **Empennage équivalent** : `aircraft.ron` peut décrire un empennage aérodynamiquement équivalent (sans flèche, surface conservée), dont le contour n'est pas celui de l'avion.
  - Sur un panneau effilé, ses points de charnière ne sont pas sur une droite parallèle à l'envergure. La charnière déduite a le bon axe, mais pas la position de la vraie : un modèle fidèle au dessin peut échouer sur le **pivot**. Pour le SPAD, l'axe réel du gouvernail est à x −4,70, à 2 cm de la droite des bandes ; la profondeur est à 4 mm. Ses charnières sont déclarées (`spad7-calculs.md` § 3.1).
  - **Avant un modèle définitif**, déclarer la vraie charnière de chaque gouverne d'un tel panneau (`hinge_m`, lue sur le dessin). Le garde-fou du contrat la refuse si elle s'écarte trop de celle des bandes.

## Modèle réaliste (Blender, ADR 0007)

- `just modele <avion>` : `mockup <avion> --json` écrit les données du modèle (contrat, surfaces, formes et contours de `maquette.ron`), puis `blender -b … art/<avion>/build.py` construit le `.blend` et le `.glb` dans `target/modele/`, que `asset_check --model` contrôle. Avec `just modele <avion> assets`, le `.glb` remplace `assets/aircraft/<avion>/model.glb` si le contrôle passe.
- **SPAD** : le modèle des assets est celui de Blender depuis le 2026-10-26.
  - L'empennage suit les contours dessinés (`outlines`), et la longueur passe sans écart accepté (6,037 m pour 6,08).
  - La hauteur reste un écart accepté, à la valeur du modèle Blender (2,343 m au sol).
- `mockup` refuse d'écraser un `model.glb` qu'il n'a pas écrit (`--force` pour passer outre).

## Modèle provisoire (`tools/mockup`)

- `just mockup <avion>` écrit `assets/aircraft/<id>/model.glb` à partir d'`aircraft.ron` et de `maquette.ron` (format : skill `simulation-vol`, `format-avion.md`). Boîtes et plaques, sans Blender ; sortie identique à l'octet d'une génération à l'autre.
- Une plaque par bande du modèle de vol, coupée là où la charnière du nœud croise sa corde : la partie mobile va dans le nœud de sa gouverne, et tourne autour de la ligne où elle est coupée. Fuselage par sections, mâts, train, béquille, hélice à deux pales, disque flou.
- Couleurs unies pour lire l'orientation sur une capture : aile droite **verte**, aile gauche **beige**, gouvernes **violettes**.
- Création maison (CC-BY-SA 4.0) : `asset.copyright` porte « Panache blanc », pas d'entrée dans `assets/LICENCES.md`.
- **Écarts acceptés** : une cote que le modèle provisoire manque pour une raison connue est inscrite dans `accepted_deviations` de `maquette.ron`, avec sa valeur mesurée et sa raison, **avec l'accord du joueur** (SPAD : longueur et hauteur, accord du 2026-10-26). Elle ne vaut qu'à 5 mm de la valeur inscrite ; seules les cotes peuvent être acceptées, jamais une charnière, un axe ou une licence. `asset_check` affiche `ok*`.
- Scène de capture `maquette-spad7` (`crates/game/src/world/maquette.rs`) : modèle placé comme le placera le jeu, axes du modèle de vol en gizmos, hélice tournée avec la flèche de son sens.
- Budget indicatif : LOD0 de 50 000 à 150 000 triangles, cockpit compris ; LOD choisis selon la taille à l'écran.

## Textures

- PBR métal-rugosité : couleur de base (sRGB), normales, ORM regroupés (occlusion, rugosité, métal ; linéaire), carte d'épaisseur de la toile (transmission).
- Densité de texels homogène ; 4K pour la cellule, 2K pour les détails.
- Export KTX2 avec mipmaps (`toktx`), espaces colorimétriques corrects.
- **Livrées** : couche de base + décalques (cocardes, croix, numéros, emblèmes). Chaque livrée historique renvoie à sa fiche de recherche.

## Terrain

- **Relief** : SRTM (domaine public) pour la base ; données nationales ouvertes plus fines si la licence est compatible (par exemple RGE ALTI de l'IGN en France, MNT de Flandre en Belgique : vérifier la licence exacte et l'inscrire).
- **Projection** : UTM zone 31N pour les Flandres ; origine de la carte = point de référence documenté.
- **Occupation du sol 1917** : numérisée dans QGIS depuis des cartes d'époque (licences vérifiées), en couches vectorielles : bois, villages, routes, canaux, voies ferrées, aérodromes, ligne de front et tranchées **par date**.
- **Nettoyage** : retirer du relief les éléments modernes (remblais d'autoroute, carrières, terrils récents) ; le relief du front sera retravaillé par les cratères.
- `tools/terrain_bake` : MNT + vecteurs → tuiles de hauteur 16 bits, normales, masques de matériaux, densité de cratères ; sortie déterministe et versionnée.

## Registre des licences

`assets/LICENCES.md` : une ligne par fichier ou lot tiers (chemin, source, auteur, licence, modifications). Licences acceptées : CC0, CC-BY, CC-BY-SA, domaine public. Refusées : « gratuit pour usage personnel », licences non commerciales, provenance inconnue. Les créations maison sont sous la licence d'assets du projet.

## Validation

`tools/asset_check` (lancé par `just test` sur chaque `assets/aircraft/<id>/model.glb`, ou `cargo run -p asset_check [avion…]`) :
- glTF valide, nœuds obligatoires présents, échelle uniforme et positive (un miroir par échelle négative est refusé) ;
- gouvernes : pivot à 1 cm du segment de charnière (un aileron posé sur l'autre aile échoue), axe à 1° ; hélice : pivot à 1 cm du moyeu, axe à 1° ; œil et armes vers l'avant (5° et 1°) ;
- envergure, longueur et hauteur à 1 % ;
- fichier maison ou inscrit dans `assets/LICENCES.md`.

Un modèle en miroir, retourné ou décalé de 5 cm échoue (tests de `tools/asset_check`). À venir : textures (format, espaces colorimétriques).
