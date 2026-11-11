# Plan — M3, heure du jour : soleil calculé, exposition, brume du matin, nuit simple

- Statut : **validé** par le joueur le 2026-11-11 (« Oui, recommandées »), avec les quatre options recommandées.
- Demande du joueur (2026-11-11) : « lance le chantier "Heure du jour" ». C'est l'incrément 2 du plan `m3-beaute.md`.
- Ce que l'incrément doit apporter, d'après `m3-beaute.md` et les revues du chantier d'atmosphère :
  - le soleil à sa place pour une date et une heure de 1917 ;
  - une exposition qui suit la lumière, de l'aube à midi : c'est la réserve principale du directeur artistique sur le contre-jour (à −1 EV, le liseré se détache du ciel) ;
  - les scènes `aube_terrain` et un couchant ;
  - une nuit simple, la lune et quelques étoiles ; la vraie nuit viendra avec les dirigeables.
- **Règles** : skills `rendu-visuel` et `bevy`. Captures avant/après regardées, revue du directeur artistique. Les chiffres historiques passent par l'historien ; les autres ont une source ou l'étiquette `ESTIMÉ`.

## Ce qui existe

- **Soleil** : une direction fixe par scène (azimut et hauteur, champ `sun`), l'après-midi (225°, 40°) par défaut. Sa lumière est l'éclairement hors atmosphère (`lux::RAW_SUNLIGHT`) ; l'atmosphère de Bevy l'atténue et le colore selon sa hauteur. Le ciel, la brume et les nuages en tirent déjà leur lumière.
- **Exposition** : fixe, EV100 = 13 (`DAYLIGHT_EV100`, choisi sur `avion-studio` le 2026-10-27). L'essai du 2026-11-11 l'a montrée trop ouverte face au soleil.
- **Exposition automatique de Bevy 0.20** (`AutoExposurePlugin`, crate `bevy_post_process`, déjà dans nos dépendances) : un histogramme de la luminance de l'image, une courbe de compensation, une vitesse d'adaptation. Elle n'est pas activée. Elle demande des compute shaders, ce que la RX 7800 XT fournit.
- **Brume** : une couche de mélange, 30 km de visibilité par défaut (`m3-atmosphere.md`). La fiche de l'historien décrit ses variations dans la journée (`brume-ete-1917.md` § 6) : brume au sol le matin, qui se lève vers 8 à 10 h (confiance B pour le phénomène, D pour les chiffres) ; couche plus épaisse l'après-midi ; horizon roussi le soir.
- **Pas encore de géographie** : la plaine n'a ni latitude ni longitude. Le relief réel des Flandres vient avec l'incrément 3.

## Étapes

1. **Le soleil calculé.**
   - Un module de position du soleil, en Rust, sans Bevy, dans `escadrille_weather` : l'algorithme de la NOAA tiré de Meeus (« Astronomical Algorithms »), précis au centième de degré, largement assez pour l'image et les ombres.
   - Le lieu par défaut : le secteur d'Ypres (environ 50,85° N, 2,89° E, `ESTIMÉ` en attendant le relief réel).
   - L'heure : l'heure légale française de 1917, heure d'été comprise, à sourcer par l'historien (dates du changement d'heure en 1917, côté français, britannique et allemand), selon la décision 2.
   - Tests : la hauteur du soleil à midi aux équinoxes et aux solstices pour la latitude d'Ypres, l'heure du midi solaire (équation du temps), et quelques valeurs d'éphémérides publiées.
   - Les scènes gagnent un champ `time` (date et heure). Le champ `sun` reste pour les scènes qui veulent un angle précis (le contre-jour, le liseré).
   - Le vol d'essai (`just run`) prend une date et une heure d'été qui redonnent à peu près le soleil actuel.
2. **L'exposition qui suit la lumière**, selon la décision 1 :
   - une base physique, tirée de l'éclairement du sol (soleil et ciel) selon la hauteur du soleil, et calée pour redonner EV100 = 13 à l'après-midi d'aujourd'hui ;
   - plus l'adaptation de l'œil de Bevy, bornée à ±1,5 EV autour de cette base, comme le propose le directeur artistique : face au soleil, l'œil ferme, et le liseré se détache du ciel ;
   - à vérifier dans les sources de Bevy 0.20 : les unités de l'histogramme, la façon de borner la correction, et que les captures restent déterministes (l'adaptation doit avoir convergé pendant les images de stabilisation).
3. **La brume selon l'heure**, selon la décision 4 :
   - le sommet de la couche de mélange monte dans la journée : bas le matin, 1 500 m l'après-midi (`ESTIMÉ`, à étayer) ;
   - au lever du jour, une brume au sol de quelques dizaines de mètres, dans les creux et sur le terrain, qui se lève vers 8 à 10 h ; il la faut pour `aube_terrain`.
4. **La nuit simple**, selon la décision 3 :
   - la lune : position et phase (algorithme de Meeus de basse précision), un disque éclairé par le soleil, et son clair de lune, faible ;
   - des étoiles ;
   - l'exposition de la nuit, par l'étape 2.
5. **Scènes et revue.**
   - Nouvelles scènes : `aube_terrain` (soleil rasant, brume au sol, ombres longues sur le terrain), un couchant en vol au-dessus des cumulus (sans combat), une nuit de lune.
   - Toutes les scènes d'extérieur recapturées, avec l'exposition qui suit la lumière.
   - Points à surveiller : l'éclairage des nuages au soleil rasant et sous l'horizon ; les cascades d'ombre avec des ombres longues ; le crépuscule de Bevy quand le soleil est sous l'horizon ; le coût du GPU.
   - Revue du directeur artistique.

## Décisions demandées au joueur

1. **Exposition** :
   - **(A, recommandé)** une base physique tirée de la hauteur du soleil, plus l'adaptation de l'œil de Bevy bornée à ±1,5 EV. Les grandes variations (aube, midi, nuit) suivent la physique, et l'adaptation ne rattrape que le contraste du cadre, comme le contre-jour ;
   - (B) l'exposition automatique de Bevy seule, sans borne. C'est le plus simple, mais une image toute de ciel ou toute de nuage pomperait, et la nuit deviendrait aussi claire que le jour ;
   - (C) la base physique seule, sans adaptation. C'est déterministe et simple, mais le contre-jour resterait trop clair, à corriger scène par scène.
2. **L'heure du jeu** :
   - **(A, recommandé)** l'heure légale française de 1917, heure d'été comprise, sourcée par l'historien : c'est celle des rapports et des carnets de vol, que les missions citeront ;
   - (B) l'heure solaire locale : plus simple, mais midi tomberait près d'une heure plus tôt que l'heure des documents.
3. **Les étoiles** :
   - **(A, recommandé)** des étoiles procédurales, avec une graine : rien à télécharger, assez pour « quelques étoiles ». Le vrai ciel viendra avec les dirigeables ;
   - (B) un catalogue réel, le Yale Bright Star Catalogue (domaine public) : les vraies constellations, mais un fichier à télécharger, avec ton accord.
4. **La brume selon l'heure** :
   - **(A, recommandé)** dans cet incrément : le sommet de la couche qui monte dans la journée, et la brume au sol du matin, qu'il faut pour `aube_terrain` ;
   - (B) plus tard, avec la météo des missions : `aube_terrain` se ferait sans brume au sol.

## Risques

- **Les nuages au soleil bas** : leur éclairage n'a été réglé qu'au soleil de 9° et plus. À l'aube et au couchant, et sous l'horizon, il faudra le revoir sur image.
- **L'exposition dans les captures** : l'adaptation dépend du temps ; les captures doivent la laisser converger, sinon elles ne seraient plus comparables d'une fois à l'autre.
- **Toutes les scènes changent** : l'exposition touche chaque image, celles déjà approuvées comprises.
- **La nuit** : la lune et son clair de lune demandent une seconde lumière directionnelle ; Bevy dessine un disque pour chaque lumière qui a un `SunDisk`, à vérifier pour la lune.

## Avancement

- **2026-11-11** : étapes 1 à 4 faites, scènes capturées, revue du directeur artistique en cours.
  - **Soleil calculé** (`escadrille_weather::sun`, NOAA d'après Meeus) et **heure légale** (`escadrille_data::legal_time`, fiche de l'historien `heure-legale-1917.md`) ; ressource `Sky` ; le vol d'essai le 1er septembre 1917 à 15 h.
  - **Exposition** : base, adaptation de Bevy, rendu (crépuscule, éblouissement). Écart à la décision 1 : la base reste fixe en plein jour, et l'éblouissement passe par l'étalonnage, après la mesure ; mesures dans le skill `rendu-visuel`.
  - **Brume selon l'heure** : brume du matin jusqu'à 7 h solaire, levée à 9 h ; couche de mélange qui monte jusqu'à 13 h.
  - **Nuit** : lune et sa phase, étoiles procédurales, une seule lumière d'astre (la lune en pleine nuit). Lumière ambiante uniforme de Bevy retirée.
  - **Scènes** : `aube_terrain`, `couchant_cumulus`, `nuit_lune`.
