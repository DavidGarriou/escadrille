# Plan — M3, relais du soleil à la lune et ciel nocturne

- Statut : **proposé** le 2026-11-11, en attente de l'accord du joueur.
- Demande du joueur (2026-11-11) : « Prépare le plan du relais soleil-lune et du ciel nocturne ». C'est la suite de la vision de nuit (`m3-heure-du-jour.md`, avancement) : la planche du crépuscule y a montré un saut d'éclairage à −12° et une nuit sans lune toute noire.
- **Règles** : skills `rendu-visuel` et `bevy`. Captures avant/après regardées, revue du directeur artistique. Chiffres sourcés, ou étiquetés `ESTIMÉ`.

## Ce qui existe

- **Une seule lumière d'astre** (`world::sky::light_the_sky`) : le soleil, puis la lune quand le soleil passe sous −12° (`NIGHT_SUN_DEG`), même si elle est sous l'horizon.
- **Bevy additionne toutes les lumières directionnelles**, jusqu'à 10 : dans le ciel, dans la perspective aérienne et sur les surfaces, avec l'ombre de la Terre.
- **Nos nuages ne prennent que la première** (`clouds/march.wesl`, `clouds/environment.wesl`), et seule la première porte leur carte d'ombre au sol (`render/clouds.rs`, qui prend n'importe quelle lumière directionnelle).
- **Code qui cherche le soleil** par `(DirectionalLight, SunDisk)` : le voile d'éblouissement et l'exposition des scènes du studio (`render/glare.rs`, `render/exposure.rs`). Une seconde lumière les tromperait.
- **L'exposition compte déjà** le crépuscule et la lune ensemble (`exposure::follow_daylight`), avec un plancher à la lumière des étoiles (0,001 lux) : elle est continue.
- **Le ciel de nuit** : la lune et 5 000 étoiles, au-delà de l'atmosphère. Aucune lumière du ciel nocturne n'éclaire les nuages, le sol ni l'avion (`GlobalAmbientLight::NONE`).

### Essai du 2026-11-11 (soleil gardé comme lumière la nuit, le 15 août 1917)

- **À −12,4°**, le crépuscule continue sans aucun saut : lueur, nuages et avion lisibles, comme à −11,8°. Avec la lune pour seule lumière, tout ce qui est sous l'horizon était noir.
- **À −18°**, il reste une lueur vers l'ouest-nord-ouest, coupée par un liseré net en biseau : c'est l'artefact vu par le directeur artistique vers −10°. À −18°, la nuit astronomique commence ; une lueur faible à l'horizon est juste, un bord net ne l'est pas. Notre plancher d'exposition grossit aussi tout reste.
- **À −25°**, le soleil ne compte plus du tout : l'image est la même qu'avec la lune pour lumière.
- **Fond du ciel** : la lumière intégrée de nos étoiles donne déjà un fond gris bleuté au zénith, plus sombre vers l'horizon. Les nuages et le sol, eux, sont noirs.
- **Ombre de la Terre dans la marche des nuages**, essayée lors de la vision de nuit : aucun effet à la hauteur des cumulus, où le rayon rasant ne transmet presque rien. Elle compte pour des nuages plus hauts, avec un soleil gardé toute la nuit.

## Étapes

1. **Deux astres.**
   - Le soleil reste une lumière à toute heure, avec son disque. La lune devient une seconde lumière, allumée quand elle est au-dessus de l'horizon, sans disque de Bevy (le nôtre la dessine).
   - Le ciel, la perspective aérienne et les surfaces les additionnent : la lueur du crépuscule continue au-delà de −12° et s'éteint d'elle-même.
   - Des composants `Sun` et `MoonLight` désignent les deux lumières : le voile d'éblouissement, l'exposition et les nuages cherchent le soleil par son nom, plus par « la lumière qui a un disque ».
   - `NIGHT_SUN_DEG` disparaît ; `Sky::is_night` ne sert plus qu'aux étoiles, s'il sert encore.
   - **Ombres portées** (avion, sol) : sur l'astre dominant seul, comme aujourd'hui une seule carte, donc pas de coût en plus.
   - Tests : les deux lumières à toute heure, la lune éteinte sous l'horizon, l'astre dominant selon l'heure et la phase.
2. **Les nuages et l'astre dominant.**
   - Les nuages ne marchent que vers une lumière : l'astre dominant, celui dont la lumière directe atteint le mieux la couche, l'ombre de la Terre comprise (selon la décision 1).
   - Le processeur le choisit et le passe dans l'uniforme des nuages (`CloudPass`), à la place de `lights.directional_lights[0]`.
   - Le soleil cesse d'éclairer la couche quand il passe sous l'horizon des nuages (environ −1,5° à 2 km). La lune prend alors le relais, quand sa lumière est encore très faible : la bascule ne se voit pas.
   - La carte d'ombre des nuages au sol (0,8 ms) suit l'astre dominant, et le clair de lune y gagne l'ombre des nuages sur les champs. Même chose pour la lumière des bases de nuages sur le sol (`GroundLight`).
   - L'ombre de la Terre entre dans la lumière directe des nuages, calculée une fois par pixel et non à chaque pas, comme Bevy le fait pour ses surfaces.
3. **Le ciel nocturne de base** (selon la décision 2).
   - **Sa valeur** : lumière propre de l'air (luminescence), lumière zodiacale et étoiles non résolues. Au zénith d'un site sombre, environ 22 mag/arcsec², soit environ 2·10⁻⁴ cd/m² (conversion usuelle L = 10,8·10⁴ × 10^(−0,4 m) cd/m², discutée par Bará et al. 2020, à vérifier), plus clair vers l'horizon. Au sol, environ 0,002 lux par nuit claire sans lune (tables d'éclairement usuelles).
     - En 1917, aucune lumière artificielle ne s'y ajoute ; les fusées et les lueurs du front viendront avec le combat.
     - Confiance B pour l'ordre de grandeur. Le maximum du cycle solaire de 1917 éclaircit la luminescence de l'air : à faire sourcer par l'historien.
   - **Dans le ciel rendu** : une sphère de fond, derrière les étoiles et au-delà de l'atmosphère, à cette luminance. L'air de Bevy la voile, ce qui l'assombrit vers l'horizon par la brume. On l'allume la nuit seulement, car elle cacherait au jour le disque du soleil (Bevy ne le dessine que sur le ciel vide).
   - **Dans l'éclairage** :
     - la lumière ambiante globale de Bevy, à cette luminance, pour l'avion et le sol. Elle est négligeable de jour (moins d'un dix-millionième du ciel), donc on peut la garder à toute heure ;
     - un terme ajouté au ciel qui éclaire les nuages dans leur marche.
   - **L'exposition** : le plancher (`STARLIGHT_LUX`) se tire de cette même valeur, pour rester cohérent.
   - Tests : une nuit sans lune où nuages et sol se lisent en masses sombres sur un ciel un peu plus clair ; de jour, rien ne change.
4. **Le liseré du ciel** (−10° à −18°).
   - D'abord en trouver la cause : taille de la table du ciel de Bevy, nombre d'échantillons, ou marche par pixel (`sky_max_samples`, 8 aujourd'hui).
   - Corriger si un réglage suffit à coût raisonnable ; sinon le noter comme limite de Bevy, avec une mesure.
5. **Un outil de planche** (selon la décision 3).
   - Une option `--time` du mode capture, qui remplace l'heure de la scène, et une recette `just planche <scène> <heures…>` qui rend la série et l'assemble en une seule image, en Rust (crate `image`, déjà présente par Bevy).
   - Plus de scènes temporaires à créer et à effacer.
6. **Scènes et revue.**
   - Nouvelles scènes :
     - `nuit_sans_lune`, sur les cumulus ;
     - un lever de lune au crépuscule, les deux astres ensemble.
   - Planche du crépuscule de −6° à −25°, avec et sans lune ; une nuit de juin, où le soleil ne descend qu'à −16° sur Ypres et où la nuit reste en crépuscule astronomique.
   - Toutes les scènes recapturées ; revue du directeur artistique.

## Décisions demandées au joueur

1. **Lumière directe des nuages** :
   - **(A, recommandé)** l'astre dominant seul, choisi par le processeur. La bascule se fait quand le soleil passe sous l'horizon des nuages, où sa lumière est déjà nulle. Aucun coût en plus ;
   - (B) les deux astres dans la marche des nuages : plus général (un lever de lune en plein crépuscule éclairerait les nuages des deux côtés), mais la marche vers la lumière, la part la plus chère des nuages, double pendant la nuit.
2. **Le ciel nocturne** :
   - **(A, recommandé)** sa luminance physique dans le ciel rendu et dans l'éclairage de tout (avion, sol, nuages). La nuit sans lune devient lisible en silhouettes, et le plancher d'exposition s'accorde avec elle ;
   - (B) dans l'éclairage seulement, sans sphère de fond : plus simple, mais le ciel sans lune resterait plus sombre que les nuages qu'il éclaire ;
   - (C) rien : la nuit sans lune reste noire, sauf les étoiles.
3. **L'outil de planche** :
   - **(A, recommandé)** `--time` et `just planche`, en Rust : il servira à chaque passe de lumière ;
   - (B) des scènes temporaires, comme pour la vision de nuit.
4. **Les valeurs de la nuit tassées** (réserve du directeur artistique : noirs à creuser plutôt que tout décaler de −5 EV) :
   - **(A, recommandé)** un chantier suivant, une fois le relais fait, puisque le relais et le ciel nocturne changent les valeurs de la nuit ;
   - (B) dans ce chantier.

## Risques

- **La lueur résiduelle sous −18°** : vue à −18°, nulle à −25°. Si elle dépasse le ciel nocturne de l'étape 3, il faudra l'éteindre plus tôt, en atténuant le soleil entre −18° et −20° par exemple. À juger sur la planche.
- **Le coût de la seconde lumière** : le ciel et la perspective aérienne de Bevy bouclent sur les lumières. Le coût est à mesurer ; la lune éteinte sous l'horizon n'en coûte aucun.
- **La bascule des ombres** (ombre portée et ombre des nuages) au relais : les deux lumières sont alors faibles, mais un saut reste possible. À vérifier sur la planche.
- **La lumière ambiante uniforme** n'est pas masquée, sauf par l'occlusion ambiante : le dessous des ailes recevra autant de ciel nocturne que le dessus. À cette luminance, on ne devrait pas le voir.
- **Les scènes du studio** n'ont ni horloge ni lune : elles gardent leur soleil seul.

## Estimation

Une demi-journée de travail : quatre plugins touchés (ciel, nuages, rendu, capture) et deux shaders. Les étapes 1 à 3 font le chantier ; les étapes 4 et 5 peuvent en sortir si le joueur le préfère.
