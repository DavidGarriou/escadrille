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
- **Forme des cumulus** (2026-11-08, variante v6 retenue par le directeur artistique) :
  - **sommet** : la position du point entre 0,1 de la rampe de couverture et la valeur la plus forte de la carte, à la puissance 0,7, abaissée jusqu'à 25 % par le bruit de type. Seule la tache la plus forte atteint le haut de la couche, et chaque nuage a son sommet. Avant, le sommet était la couverture elle-même, saturée au cœur des taches : tous les cœurs montaient au haut de la couche, plats à une même hauteur ;
  - **profil** : le cumulus s'arrondit dès la moitié de sa hauteur (avant : plein jusqu'à 65 %, d'où des parois verticales) ;
  - **couvert** : vers une couverture de 1, les sommets remontent au haut de la couche (couverture puissance 8), pour qu'un couvert reste une nappe sans trou ;
  - **pièges** : borner le dénominateur (couverture sous 0,04) et sortir avant `pow(0, …)`, que WGSL ne définit pas ; le test de parité couvre ces cas ;
  - **essai rejeté** : un dôme 1 − h² sur toute la hauteur abaissait les parois sur le banc de mesure, mais vidait le haut des nuages, que l'érosion mangeait : les tours du contre-jour devenaient une mer de nuages. Juger la forme sur image, pas sur la seule mesure des parois ;
  - **mesure** : hauteur médiane de la colonne au bord visible, de 150-250 m à 110-190 m ; le bord reste raide sur 100 à 200 m ;
  - **couverture** : à 0,55, l'empreinte des nuages approche le seuil de percolation (50 %) et les taches se soudent en crêtes de plusieurs kilomètres. Pour des cumulus séparés, rester vers 0,45.
- **Jumeau WGSL** : `assets/shaders/clouds/density.wesl`, en WGSL pur sans import. Textures et échantillonneur y sont des paramètres de fonction : chaque passe les lie où elle veut. Le test `render::clouds::tests::the_gpu_density_is_the_reference` le compile tel quel, le lance sur le GPU sur 65 536 points et compare. Écart au pire de 3,5·10⁻³ sur la RX 7800 XT, dû à l'arrondi des poids du filtrage matériel.

## Éclairage

**En place** (2026-11-07) : le modèle de `escadrille_weather::scattering`, jumeau des fonctions du shader `march.wesl` (la marche, partagée par la vue et la carte d'environnement), dont les tests vérifient l'énergie :
- **diffusion simple** : phase de Henyey-Greenstein à deux lobes × e^−τ vers le soleil ;
- **diffusion multiple, en deux parts** : une part rapide près de la face éclairée, 1,8·e^−τ, qui donne les creux entre les bourgeons ; puis la traîne de diffusion, 2,1/(1 + 0,055τ), qui garde la lumière des bases ; le tout divisé par 4π. Les constantes viennent d'un balayage contre la théorie à deux flux (Bohren 1987) : à τ = 50, sommet à +8 % et base à −2 % ; un bourgeon de 50 m dans son ombre est 2 fois plus sombre. Deux impasses : les octaves de Wrenninge en e^−τ/2ⁿ éteignaient les bases, la diffusion seule ne laissait aucun modelé ;
- **marche vers le soleil** : premier pas de 8 m, puis ×3, soit 2,9 km en 6 pas ; les trois premiers pas (104 m) portent sur la densité détaillée, pour que les bourgeons s'ombrent entre eux (deux, soit 32 m, jusqu'au 2026-11-08 : la lumière au-delà ne voyait que l'enveloppe lisse ; avec trois, les bords éclairés de la crête du contre-jour passent de 1,45 à 1,63 fois le ciel voisin, pour environ 0,05 ms). À 900 m de portée, la face sombre d'une tour face à un soleil à 9° gardait trois fois la lumière que laisse passer la théorie à deux flux, et virait au brun orangé ;
- **ciel et sol**, chacun sur sa demi-sphère, atténués par la même décroissance :
  - le ciel, par une courte marche vers le haut penchée vers le côté de l'œil (10, 30, 90 puis 270 m, le premier pas sur la densité complète), ou jusqu'au sommet de l'enveloppe, le plus court (2026-11-09 ; voir « Part de ciel ») ;
  - le sol, par la profondeur jusqu'à la base ;
- **vue de débogage par terme** : remplacer, le temps de quelques captures, la somme `scattered` de `march.wesl` par un seul terme (simple, multiple, ciel ou sol), sans voile de distance. Les shaders se chargent au lancement : sans recompiler le jeu, chaque capture prend une minute. C'est elle qui a montré que la diffusion multiple, et non le ciel, rendait le contre-jour orangé. Le sol a un albédo de 0,18 (`ESTIMÉ`, champs) et reçoit le soleil hors des ombres des nuages ;
- **limite** : le modèle est local. Une frange mince au soleil renvoie autant que le sommet d'un nuage épais.
- **Ce que la vue par terme a montré** (2026-11-08) : la traîne de diffusion et le ciel, presque uniformes, font l'essentiel de la lumière d'une face à l'ombre ou d'un flanc vu de près ; d'où l'aspect « coton ». Le relief vient du terme rapide et de la diffusion simple.
  - **Limite de calibration** : contre les tests d'énergie (deux flux, τ de 5 à 100, ciel, sommet/base), seul le réglage actuel passe ; le rapport bourgeon éclairé / à l'ombre plafonne vers 2,1, même avec une troisième composante.
  - **Essais écartés** : une troisième composante (e^−τ/8) ; la « poudre » de Schneider (dômes plus gris et moins détaillés) ; une frange de densité adoucie pour le liseré (rien de plus, bords émoussés).
  - **Limites physiques acceptées** (directeur artistique) : un cumulus éclairé de face, le soleil dans le dos, est plat ; un flanc nord en plein soleil de midi, où la traîne fait 87 % de la lumière, est gris et doux. Ne pas chercher à les creuser.
  - **Part de ciel** (2026-11-09) : elle était prise jusqu'au sommet de l'enveloppe, ou le long du rayon de l'œil sur un flanc, chemin nul à la surface : tout point visible recevait le ciel entier, et les faces à l'ombre du contre-jour étaient un aplat. Une courte marche vers le haut, penchée vers le côté de l'œil (ouvert pour un flanc visible), la mesure maintenant (`optical_depth_to_sky`, référence Rust `CloudTextures::optical_depth_to_sky`, test : 21 au pied de la tour de la traversée, 6 à mi-flanc, 0,8 au sommet). Les lobes du contre-jour gagnent leurs bourgeons ; ils deviennent plus chauds que bleutés, la traîne orangée y dominant une fois le ciel retiré. Elle remplace aussi la profondeur de l'œil dans le nuage (une valeur par image, retirée) : continue dans toutes les directions, elle ne trace ni ligne à l'horizon ni forme qui suive le regard.
    - **Gigue** : lue au milieu de chaque pas, la marche imprimait les cellules du bruit en taches rondes et pâles (« pommelé », directeur artistique). Le décalage de ses pas suit une gigue par pixel, `fract(first + SECOND_JITTER)`, que la passe temporelle moyenne ; la référence Rust prend le décalage en paramètre (0,5 pour le milieu des pas). Vérifiée en mouvement (virages de `cumulus_contre_jour` et de `cumulus_midi`, approche de la traversée) : le bruit d'une image à l'autre ne varie que de ±3 % avec ou sans gigue, sans grouillement sur les faces à l'ombre.
  - **Liseré à contre-jour** (2026-11-09) : le modèle le fait quand le soleil est au bord d'une tour ou juste derrière. Avec le soleil de `cumulus_contre_jour` déplacé à 156° (élévation 9°), une bande de 10 à 30 px s'allume juste à l'intérieur du bord près du soleil : 0,84 à 0,86 en luminance linéaire, contre 0,50 pour le corps ; à 200 px du soleil, elle disparaît. Avec le soleil à 6–10° des bords, comme dans la scène, il n'y en a pas : c'est une affaire de composition, plus de shader. Scène de référence : `cumulus_lisere`.
    - **Il reste doux** : le bord ne dépasse le corps juste en dessous que de 15 % (0,84 contre 0,73), quand un vrai liseré éblouit. La double Henyey-Greenstein n'a pas le pic de diffraction des gouttelettes, de un à deux ordres de grandeur plus fort entre 1 et 3° du soleil (directeur artistique). Piste : un lobe avant étroit, par exemple la phase approchée HG + Draine (Jendersie et d'Eon, 2023), normalisée pour garder l'énergie ; à régler avec l'auréole de l'atmosphère, qui joue dans les mêmes degrés autour du soleil.

Repères visés, d'après le directeur artistique : un cumulus au soleil est l'objet le plus lumineux du ciel, 2 à 4 fois plus que le ciel voisin ; sa base et ses creux sont 3 à 5 fois plus sombres que ses sommets ; son bord passe du clair à l'opaque en une dizaine de mètres (d'où le gain de 4 sur la densité finale).

Principes de départ :

- Beer-Lambert pour l'extinction, terme « powder » pour l'assombrissement des bords côté soleil.
- Fonction de phase de Henyey-Greenstein à deux lobes (liseré argenté à contre-jour).
- Approximation de la multidiffusion par octaves (Wrenninge) ; lumière ambiante issue du ciel.
- Marche vers le soleil courte (environ 6 pas) ; ombres des nuages sur le terrain et sur les avions.
- Couleurs cohérentes avec l'atmosphère (lever et coucher du soleil, perspective aérienne sur les nuages lointains).

## Performances

- **En place (2026-11-07)** : marche à la moitié de la résolution sur chaque axe (« haut »), puis reconstruction temporelle et suréchantillonnage selon la profondeur. Trois passes : `render.wesl` (qui appelle la marche de `march.wesl`), `temporal.wesl`, `composite.wesl`.
  - **Marche** : elle écrit la lumière, la transmittance, la distance où elle s'est arrêtée (l'objet le plus proche du bloc de pixels) et la distance moyenne des nuages.
  - **Temporel** : reprojection à la distance moyenne, découpage dans la moyenne du voisinage ± 1,25 écart-type, 12 % de poids à l'image courante, remise à zéro quand l'origine de rendu saute.
    - **Voisinage de même arrêt** (2026-11-08) : la moyenne et l'écart-type ne comptent que les voisins dont la marche s'est arrêtée à la même distance (à 25 % près). Au bord d'une silhouette, les texels arrêtés sur l'avion (brouillard mince) et ceux passés par le ciel (nuage opaque) élargissaient tant la plage que l'historique fuyait de l'un à l'autre : liseré bleu du ciel le long des silhouettes dans le brouillard, et halo dédoublé des mâts et des bords d'aile sur fond de nuages. Le TAA de Bevy et la composition n'y étaient pour rien (vérifié sans TAA, et en pleine résolution, où le liseré restait sur un pixel). 0,07 ms au lieu de 0,03.
  - **Composition** : des quatre texels voisins, ceux dont la distance d'arrêt s'accorde avec la profondeur du pixel pèsent le plus ; un nuage situé derrière l'objet du pixel ne passe pas devant.
  - **Coût en 1440p** : 0,3 à 0,44 ms pour la marche et 0,02 ms pour le temporel, contre 0,9 à 1,6 ms en pleine résolution. Deux images consécutives diffèrent de 0,04 % : pas de scintillement.
- **Transmittance de l'air jusqu'aux nuages** (2026-11-08) : intégrée sur 12 pas le long du segment, à partir de la table de densité de Bevy (`air_transmittance`, `march.wesl`). La fonction de Bevy `sample_transmittance_lut_segment` divise deux valeurs de sa table, toutes deux minuscules près de l'horizon de la planète, et change de formule là où le rayon touche le sol : bornée à 1, elle laissait une bande de nuages lointains sans voile, un trait jaune à l'horizon de `cumulus_midi`. Le terme du sol de la planète (`environment.wesl`) la garde, pour retrancher exactement ce que Bevy ajoute.
- **Piège de précision** : reconstruire la direction d'un rayon en soustrayant l'œil, à 2 000 m, d'un point du plan proche situé à 5 cm perd quelques pixels à l'arrondi f32. La reprojection glissait alors vers le bas, en rideaux. Travailler en axes du monde centrés sur l'œil, avec le petit déplacement de l'œil d'une image à l'autre.

- **Marche en place** : pas proportionnels à la distance (2 %). Elle enjambe trois pas à la fois l'air situé hors de toute enveloppe, avec une gigue étalée sur toute l'enjambée ; elle recule à l'entrée d'un nuage. Sans cela, des stries apparaissaient sur les dômes, et jusqu'à l'horizon.
  - **La tranche avant le premier pas** (2026-11-08, `traversee_nuage`) : la gigue place le premier pas jusqu'à une enjambée de l'œil (36 m de près). Un échantillon tiré dans cette tranche, à une seconde gigue, la représente tout entière ; le dernier pas est coupé où le rayon s'arrête. Sans cela, caméra dans le nuage, le brouillard devant l'avion à 10 m était tantôt nul, tantôt d'un pas entier.

- Raymarching à résolution réduite (quart de résolution, par exemple) avec reprojection temporelle et reconstruction ; pas adaptatifs (grands pas dans le vide, petits dans la densité).
- Bruit bleu pour décaler les échantillons, accumulé par la reprojection.
- Composition avec la profondeur de la scène (avions devant et dans les nuages).
- Réglages de qualité : nombre de pas, résolution, détail.

## Ombres des nuages

**En place (2026-11-07)** : une carte de transmittance vue du soleil (`shadow.wesl`, R8, 2 048² sur 48 km, soit 23 m par texel), posée sur le soleil comme texture de lumière (`DirectionalLightTexture`, feature `pbr_light_textures` ; voir ADR 0004). Bevy la projette le long du soleil sur tout ce que la lumière éclaire : sol et avion. La carte est répétée : hors de sa portée, une texture de lumière non répétée éteindrait le soleil.
- **Placement** : la carte est avancée de 0,35 fois sa largeur dans la direction horizontale de la vue, et alignée sur sa grille de texels dans le plan du soleil, sans quoi ses bords ondulent en vol. Le soleil porte ce centre et une échelle égale à sa demi-largeur ; ses cascades d'ombre n'utilisent que sa rotation. Ses 20 % extérieurs se fondent dans la transmittance moyenne de la couche (1 − 0,76 × couverture, `ESTIMÉ`), pour que la répétition se lise comme un voile uniforme.
- **Une valeur par rayon de soleil** : sans précaution, un avion au soleil au-dessus d'un nuage prendrait l'ombre que ce nuage jette au sol. Autour de la ligne de soleil de l'avion joueur (un texel, puis un fondu sur un autre), la carte ne compte que les nuages au-dessus de lui. Les avions de l'IA (M4) poseront la même question.
- **Lumière des bases sur le sol** : quand la caméra survole la couche, la carte d'environnement (ci-dessous), vue de la caméra, montre un ciel clair au-dessus. Le sol, lui, voit les bases. Son matériau (`render/ground.rs`, `shaders/ground.wesl`) ajoute donc leur lumière, 0,6 × couverture × E_sol / π (`ESTIMÉ`), dans la part de la couche que la caméra survole. E_sol est l'éclairement du soleil sur un sol horizontal, à travers l'air (épaisseur optique 0,25 au zénith). Sans elle, l'ombre ne recevait que la lumière bleue d'un ciel clair, et se lisait comme un lac. Du 2026-11-07 au 2026-11-08, c'était une `AmbientLight` uniforme, qui rendait l'avion plat.
- **Coût** : 0,37 à 0,40 ms en 1440p ; 24 pas sur la densité grossière par texel, à chaque image.
- **Scène de contrôle** : `cumulus_sous_la_base`, l'avion à l'ombre d'un nuage.
- **À suivre** : la même carte, en profondeur optique, pour l'ombre lointaine d'un nuage sur son voisin, que la marche vers le soleil (2,9 km) ne voit pas sous un soleil bas ; une pénombre qui croît avec la distance au nuage, par soleil bas.

## Lumière des nuages sur la scène (carte d'environnement)

**En place (2026-11-08, complément de l'ADR 0009)** : la passe de calcul `environment.wesl` écrit dans le cube de la carte d'environnement de la caméra (`GeneratedEnvironmentMapLight`, 128² par face) :
- le ciel de l'atmosphère ;
- les nuages, marchés par la fonction de la vue (`march.wesl`) avec moins de pas : de 24 à 64 selon le niveau, avec un bruit fixe par texel ;
- sous l'horizon, les champs à la place de la planète grise de Bevy : albédo moyen de leur palette, soleil atténué par l'ombre moyenne des nuages, ciel.

Bevy filtre ce cube au début de l'image suivante en lumière diffuse et en reflets. L'avion prend ainsi le gris des bases par-dessus, le sol vert-ocre par-dessous, et le blanc de la mer de nuages quand il la survole.
- **Piège** : l'albédo de l'`Atmosphere` sert aussi à sa multidiffusion. Mis aux couleurs des champs, il rendait la brume de l'horizon olive : il reste au gris de la Terre, et seule la carte montre les champs.
- **Mesure** (`cumulus_sous_la_base`, lumière linéaire) : à l'ombre, les faces du dessus de l'avion reçoivent 0,27 à 0,31 fois la lumière au soleil, et les flancs 0,30. Le directeur artistique vise 0,25 à 0,4.
- **Coût** : 0,09 à 0,13 ms en 1440p.
- **Limites** : une seule carte, vue de la caméra ; sans nuages, la planète grise de Bevy reste sous l'horizon.

## Dans le nuage

- Caméra dans le volume : brouillard volumétrique local, perte de repères, gouttelettes sur le pare-brise en option, turbulence (le FDM lit la densité via le trait `Environment`).
- La transition extérieur → intérieur ne doit montrer ni « pop » ni bande.
- **En place (2026-11-08)**, vérifié sur la séquence `traversee_nuage` :
  - **brouillard jusqu'à l'avion** : la tranche avant le premier pas de la marche est intégrée (section Performances) ;
  - **pas de flanc vu de dedans** : la lumière du ciel par le côté de l'œil supposait que le rayon était entré dans le nuage par un flanc ; de l'intérieur, une ligne coupait le brouillard à l'horizon, puis une valeur de profondeur de l'œil par pixel dessinait des formes qui tournaient avec le regard. Réglé en deux temps, d'abord par une profondeur de l'œil par image (2026-11-08), puis par la marche vers le ciel, qui la remplace (2026-11-09).
  - **surfaces transparentes voilées** (`render/clouds/fog.rs`, `shaders/clouds/fog.wesl`) : câbles, disque de l'hélice et verre sont dessinés après la composition des nuages, donc sans leur brouillard. Leurs shaders (`wire_aa.wesl`, `own_normals.wesl`) lisent l'image des nuages à leur pixel. Si la marche s'y est arrêtée à leur distance, sur l'avion derrière eux, ils prennent sa lumière et sa transmittance. Sinon, ils prennent l'extinction du nuage à l'œil (calculée en Rust, dans un tampon partagé écrit une fois par image), et la lumière de la marche dans la même part. Hors des nuages, pas de voile, quels que soient les nuages derrière ;
  - **mesures** : aucun saut d'une image à l'autre à l'entrée ni à la sortie (séquence dense, variation lisse sur une dizaine d'images) ; l'avion passe de 0,65 de luminance d'affichage au soleil à 0,50 au cœur, un peu plus sombre que le brouillard (0,58).
- **Vue du pilote** (`traversee_nuage_pilote`, regard à 40° à droite) : les mâts et les câbles de l'aile droite, à 2 ou 3 m, se voilent avant ceux du poste.
- **Limites** : le bout d'aile et le capot ne tiennent pas dans une même vue ; la poursuite à 10 m ne montre jamais l'avion englouti par le nez (il faudrait une poursuite longue ou un observateur fixe).

## Usages de jeu (même fonction de densité, côté CPU)

- **Visibilité de l'IA** : lancer de rayon dans la densité, avec un seuil de détection.
- **Turbulence** près et dans les cumulus.
- **Missions** : plafond, trouées, couches, météo qui interdit le vol.

## Validation

- Scènes `cumulus_midi`, `cumulus_contre_jour`, `cumulus_sous_la_base`, `traversee_nuage` (un vol capturé en séquence), puis `combat_couchant` et `plafond_bas`.
- Vérifier : pas de banding (bruit bleu), pas de traînées de reprojection derrière un avion en mouvement rapide, bords nets mais pas découpés, lumière cohérente avec le soleil.
