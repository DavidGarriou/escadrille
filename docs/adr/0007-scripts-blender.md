# 0007 — Scripts Blender (Python) pour les sources de contenu dans `art/`

- Statut : Accepté (choix du joueur du 2026-10-26, « Blender par script », plan `docs/plans/m3-modele-spad7.md`)
- Date : 2026-10-26

## Contexte

Le joueur demande un modèle 3D du SPAD VII « le plus réaliste possible, avec une excellente qualité de rendu ». C'est l'élément « Modèle PBR du premier avion » du M3, avancé.

- **L'outil prévu** : `CLAUDE.md` liste Blender parmi les outils de contenu.
- **Pas d'interface dans le conteneur** : Claude ne peut s'en servir que par script. L'API de Blender (`bpy`) est en Python.
- **La règle en vigueur** : `CLAUDE.md` pose « Rust uniquement » parmi les contraintes non négociables.

Le modèle provisoire est fait par un générateur en Rust (`tools/mockup`). Pour un modèle réaliste, il faut des outils qu'un générateur maison n'a pas : surfaces de subdivision, chanfreins, dépliage UV, et précalcul de textures. Le précalcul porte sur l'occlusion, les normales des rubans et des coutures, et l'épaisseur de la toile pour la transmission.

## Options

1. **Scripts Python exécutés par Blender, dans `art/`** (retenue).
   - **Fonctionnement** : un script par avion (`art/<id>/build.py`) construit le modèle à partir des données (`aircraft.ron`, fiches), précalcule les textures avec Cycles et exporte `assets/aircraft/<id>/model.glb`. Le `.blend` produit peut être ouvert et retouché à la main sur la machine du joueur.
   - **Avantages** : la qualité de Blender, une construction reproductible et relue comme du code, et la retouche manuelle reste possible.
   - **Inconvénients** : un second langage dans le dépôt, Blender installé pour reconstruire un modèle, et des sorties lourdes (`.blend`, textures).
2. **Tout en Rust** : `tools/mockup` devient un modeleur procédural.
   - **Avantages** : pas d'exception, sortie déterministe.
   - **Inconvénients** : il faudrait réécrire la subdivision, le dépliage UV et le précalcul. Le résultat serait moins bon sur les formes arrondies (capot, carénages, saumons), et il faudrait une dépendance de plus pour écrire les textures.
3. **Modélisation à la main par le joueur**, Claude fournissant cotes et références. Elle ne répond pas à la demande, et la construction ne serait ni reproductible ni relue.

## Décision

**Option 1.** Le Python est accepté **uniquement** pour les scripts de contenu qui s'exécutent dans Blender, rangés dans `art/`.
- Le jeu, le modèle de vol et les outils (`tools/`) restent en Rust.
- Aucun code Rust n'appelle ces scripts, et ces scripts n'importent que `bpy`, les modules livrés avec Blender et la bibliothèque standard. Aucun paquet `pip`.

## Conséquences

- **Version** : Blender **4.0** minimum. Le conteneur a 4.0.2, paquet Ubuntu, avec `python3-numpy` pour l'exporteur glTF ; blender.org est bloqué par le proxy. Le script vérifie la version au démarrage et doit tourner aussi sur la machine du joueur (Fedora).
- **Licences** : les scripts sont du code, sous MIT (`LICENSE`). Ce qu'ils produisent (`.blend`, textures, `.glb`) est un asset, sous CC-BY-SA 4.0 (`LICENSE-ASSETS`). Aucune texture tierce, ou alors inscrite dans `assets/LICENCES.md`.
- **Contrôle** :
  - la sortie passe par `tools/asset_check`, lancé par `just test` : contrat des nœuds, charnières, dimensions et licence ;
  - toute modification visuelle passe par des captures regardées, et les changements importants par le directeur artistique ;
  - une recette `just modele <avion>` reconstruit le modèle.
- **Repères** : ceux de l'ADR 0006 (nez vers −Y, dessus +Z, aile droite vers −X dans Blender ; origine au `datum`).
- **Fichiers lourds** :
  - le `.blend` et les textures sources passent par **Git LFS** (convention de `pipeline-assets`) ;
  - si le stockage LFS n'est pas joignable depuis le conteneur, le `.blend` n'est pas versionné : le script suffit à le reconstruire ;
  - le `.glb` exporté reste versionné, comme aujourd'hui.
- **`CLAUDE.md`** : la contrainte « Rust uniquement » cite cette exception, et l'arborescence mentionne les scripts de `art/`.
- **À surveiller** :
  - que l'exception ne s'étende pas : aucun script Python ailleurs que dans `art/` ;
  - les écarts entre versions de Blender, sur l'API et sur l'exporteur glTF.
