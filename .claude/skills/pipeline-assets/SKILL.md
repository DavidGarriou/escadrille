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
- **Orientation** : la convention d'axes Blender → glTF → Bevy → repère corps du FDM est fixée une fois par un ADR, avec une scène de test qui vérifie que le nez du modèle pointe dans l'axe avant du FDM. `asset_check` contrôle chaque export.
- **Nœuds obligatoires** (pivot sur l'axe de rotation réel, axe local = axe de charnière) :

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
- **Dimensions** : le modèle respecte la géométrie de la fiche de recherche. `tools/asset_check` compare les boîtes englobantes (envergure, longueur, hauteur) à `aircraft.ron`, à 1 % près. La source de vérité de la géométrie aérodynamique reste `aircraft.ron`.
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

`tools/asset_check` (lancé par `just test`) : nœuds obligatoires présents, pivots cohérents, dimensions conformes, textures au bon format, toutes les entrées présentes dans `assets/LICENCES.md`.
