# Nuages volumétriques

Dans un simulateur de 14-18, les nuages sont à la fois le décor principal et un élément tactique : on s'y cache, on en surgit, on s'y perd. Ils doivent être beaux de loin, de près **et de l'intérieur**.

## Références techniques

- Schneider et Vos, *The Real-time Volumetric Cloudscapes of Horizon Zero Dawn*, SIGGRAPH 2015 (Advances in Real-Time Rendering).
- Schneider, *Nubis: Authoring Real-Time Volumetric Cloudscapes with the Decima Engine*, SIGGRAPH 2017.
- Schneider, *Nubis³* (nuages en voxels que l'on traverse en vol), SIGGRAPH 2023.
- Hillaire, *Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite*, SIGGRAPH 2016.
- Hillaire, *A Scalable and Production Ready Sky and Atmosphere Rendering Technique*, EGSR 2020.

## Modèle de densité

- **Carte météo 2D** (couverture, type de nuage, précipitations), défilant avec le vent ; générée par le système météo de la mission.
- **Bruits 3D** : Perlin-Worley basse fréquence (forme, environ 128³) et Worley haute fréquence (détail, environ 32³), **générés en Rust** de façon déterministe (au build ou au démarrage), en tuiles répétables.
- **Profil vertical** par type (stratus, cumulus, cumulonimbus) ; couches distinctes possibles (cirrus en 2D ou en couche fine).
- Fonction `cloud_density(pos_monde, temps) -> f32` écrite **deux fois** : en WGSL pour le rendu, en Rust pour le jeu. Test de parité obligatoire.

## Éclairage

- Beer-Lambert pour l'extinction, terme « powder » pour l'assombrissement des bords côté soleil.
- Fonction de phase de Henyey-Greenstein à deux lobes (liseré argenté à contre-jour).
- Approximation de la multidiffusion par octaves (Wrenninge) ; lumière ambiante issue du ciel.
- Marche vers le soleil courte (environ 6 pas) ; ombres des nuages sur le terrain et sur les avions.
- Couleurs cohérentes avec l'atmosphère (lever et coucher du soleil, perspective aérienne sur les nuages lointains).

## Performances

- Raymarching à résolution réduite (quart de résolution, par exemple) avec reprojection temporelle et reconstruction ; pas adaptatifs (grands pas dans le vide, petits dans la densité).
- Bruit bleu pour décaler les échantillons, accumulé par la reprojection.
- Composition avec la profondeur de la scène (avions devant et dans les nuages).
- Réglages de qualité : nombre de pas, résolution, détail.

## Dans le nuage

- Caméra dans le volume : brouillard volumétrique local, perte de repères, gouttelettes sur le pare-brise en option, turbulence (le FDM lit la densité via le trait `Environment`).
- La transition extérieur → intérieur ne doit montrer ni « pop » ni bande.

## Usages de jeu (même fonction de densité, côté CPU)

- **Visibilité de l'IA** : lancer de rayon dans la densité, avec un seuil de détection.
- **Turbulence** près et dans les cumulus.
- **Missions** : plafond, trouées, couches, météo qui interdit le vol.

## Validation

- Scènes `cumulus_midi`, `combat_couchant`, `plafond_bas` et une scène de traversée (séquence de captures en entrant dans un nuage).
- Vérifier : pas de banding (bruit bleu), pas de traînées de reprojection derrière un avion en mouvement rapide, bords nets mais pas découpés, lumière cohérente avec le soleil.
