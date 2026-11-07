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
- **En place** (2026-11-07) : la référence Rust est dans la crate `escadrille_weather` (`crates/weather/src/clouds.rs`), sans Bevy. Elle suit le schéma de *Nubis* : la carte météo place chaque nuage et fixe sa hauteur, le bruit de forme érode son enveloppe. Couper le bruit de forme par la couverture dessinait une grille de nuages identiques, à la période du bruit.
- **Jumeau WGSL** : `assets/shaders/clouds/density.wesl`, en WGSL pur sans import. Textures et échantillonneur y sont des paramètres de fonction : chaque passe les lie où elle veut. Le test `render::clouds::tests::the_gpu_density_is_the_reference` le compile tel quel, le lance sur le GPU sur 65 536 points et compare. Écart au pire de 3,5·10⁻³ sur la RX 7800 XT, dû à l'arrondi des poids du filtrage matériel.

## Éclairage

**En place** (2026-11-07) : le modèle de `escadrille_weather::scattering`, jumeau des fonctions du shader `render.wesl`, dont les tests vérifient l'énergie :
- **diffusion simple** : phase de Henyey-Greenstein à deux lobes × e^−τ vers le soleil ;
- **diffusion multiple, en deux parts** : une part rapide près de la face éclairée, 1,8·e^−τ, qui donne les creux entre les bourgeons ; puis la traîne de diffusion, 2,1/(1 + 0,055τ), qui garde la lumière des bases ; le tout divisé par 4π. Les constantes viennent d'un balayage contre la théorie à deux flux (Bohren 1987) : à τ = 50, sommet à +8 % et base à −2 % ; un bourgeon de 50 m dans son ombre est 2 fois plus sombre. Deux impasses : les octaves de Wrenninge en e^−τ/2ⁿ éteignaient les bases, la diffusion seule ne laissait aucun modelé ;
- **marche vers le soleil** : premier pas de 8 m, puis ×3, soit 2,9 km en 6 pas ; les deux premiers pas portent sur la densité détaillée, pour que les bourgeons s'ombrent entre eux. À 900 m de portée, la face sombre d'une tour face à un soleil à 9° gardait trois fois la lumière que laisse passer la théorie à deux flux, et virait au brun orangé ;
- **ciel et sol**, chacun sur sa demi-sphère, atténués par la même décroissance :
  - le ciel, par la profondeur jusqu'au sommet de l'enveloppe, ou par la profondeur déjà traversée le long du rayon de l'œil quand celui-ci regarde un flanc, qui voit le ciel devant lui. Ce second chemin ne vaut pas quand l'œil regarde une base par en dessous : de ce côté, la base voit le sol ;
  - le sol, par la profondeur jusqu'à la base ;
- **vue de débogage par terme** : remplacer, le temps de quelques captures, la somme `scattered` de `render.wesl` par un seul terme (simple, multiple, ciel ou sol), sans voile de distance. Les shaders se chargent au lancement : sans recompiler le jeu, chaque capture prend une minute. C'est elle qui a montré que la diffusion multiple, et non le ciel, rendait le contre-jour orangé. Le sol a un albédo de 0,18 (`ESTIMÉ`, champs) et reçoit le soleil hors des ombres des nuages ;
- **limite** : le modèle est local. Une frange mince au soleil renvoie autant que le sommet d'un nuage épais.

Repères visés, d'après le directeur artistique : un cumulus au soleil est l'objet le plus lumineux du ciel, 2 à 4 fois plus que le ciel voisin ; sa base et ses creux sont 3 à 5 fois plus sombres que ses sommets ; son bord passe du clair à l'opaque en une dizaine de mètres (d'où le gain de 4 sur la densité finale).

Principes de départ :

- Beer-Lambert pour l'extinction, terme « powder » pour l'assombrissement des bords côté soleil.
- Fonction de phase de Henyey-Greenstein à deux lobes (liseré argenté à contre-jour).
- Approximation de la multidiffusion par octaves (Wrenninge) ; lumière ambiante issue du ciel.
- Marche vers le soleil courte (environ 6 pas) ; ombres des nuages sur le terrain et sur les avions.
- Couleurs cohérentes avec l'atmosphère (lever et coucher du soleil, perspective aérienne sur les nuages lointains).

## Performances

- **Marche en place** : pas proportionnels à la distance (2 %). Elle enjambe trois pas à la fois l'air situé hors de toute enveloppe, avec une gigue étalée sur toute l'enjambée ; elle recule à l'entrée d'un nuage. Sans cela, des stries apparaissaient sur les dômes, et jusqu'à l'horizon.

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
