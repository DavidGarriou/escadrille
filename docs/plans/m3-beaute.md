# Plan — M3, beauté, première passe

- Statut : **validé** par le joueur le 2026-11-04, avec les options recommandées :
  - nuages d'abord, puis heure du jour, relief, paysage de 1917, finition du SPAD ;
  - nuages par une marche de rayons maison en WGSL ;
  - relief des MNT nationaux à 5 m, RGE ALTI (Licence Ouverte 2.0) et DHMV II (licence flamande), **licences acceptées** par le joueur ;
  - paysage procédural guidé par des couches numérisées depuis des sources libres de droits ;
  - relief sur 80 × 80 km, détail d'abord sur 30 km autour d'Ypres et du terrain de la SPA 3.
- Demande du joueur (2026-11-04) : « on continue avec le M3 ».
- **Jalon** (`docs/roadmap.md`) :
  - nuages volumétriques, qu'on peut traverser ;
  - atmosphère et cycle jour/nuit ;
  - terrain réel des Flandres avec niveaux de détail, occupation du sol de 1917 et zone du front ;
  - modèle PBR du premier avion.
- **Critère de fin** :
  - les scènes de référence sont capturées et **approuvées par le joueur** ;
  - le budget de performances est tenu sur la RX 7800 XT : 60 images/s en 1440p, qualité « haut ».
- **Règles** : skills `rendu-visuel` (et `nuages-volumetriques.md`), `pipeline-assets`, `bevy`. Chaque étape passe par des captures avant/après **regardées**. Les changements importants passent par la revue du directeur artistique.
- **Pilier en jeu** : « Le ciel est le personnage principal ». Les nuages et le relief servent aussi au jeu : se cacher dans un nuage, lire sa hauteur, retrouver son terrain. Ils ont donc une version Rust de référence, avec un test de parité contre le shader (`CLAUDE.md`).
- **Hors de ce jalon** :
  - traçantes, fumées, explosions et effets du combat, avec `bevy_hanabi` (M4) ;
  - pluie, neige et météo dynamique ;
  - nuit complète avec projecteurs et feu (mission des dirigeables) ;
  - son ;
  - autres avions.

## Ce qui existe déjà

| Élément | État |
|---|---|
| Atmosphère de Bevy 0.20 | en place : ciel physique et perspective aérienne (`render_sky`). **Bevy n'a pas de nuages** : vérifié dans les sources de la version épinglée |
| Soleil | fixe, au sud-ouest à 40° |
| Sol | plaine plate de 200 km, texture de champs répétée tous les 2 km, et le pré du M2 |
| Modèle du SPAD | B4 fait ; B5 première passe (toile peinte, translucide) ; B6 (mipmaps, studio, câbles) |
| Restes du plan `m3-modele-spad7.md` | bois et métal, rugosité, seconde livrée (S.254), poste de pilotage (B7) |
| Coût actuel | 3 à 5,5 ms par image en 1080p, sans nuages ni relief |
| Outils | GDAL 3.12 installé ; ni QGIS ni KTX-Software |

## Incréments

Chacun laisse le jeu lançable, avec tests, `just check`, `just test`, captures regardées, carnet, commit et push. Ordre recommandé : voir la décision 1.

### 0. Scènes et mesure

- **Scènes en données** : `assets/scenes/<nom>.ron` (caméra, date et heure, météo, graine, avion et son état). Les scènes du M3 n'existent pas encore, et leur forme commune est connue : c'est le moment de le faire. Les scènes actuelles, construites en code, y passent une à une.
- **Mesure GPU** : temps par passe, par les requêtes d'horodatage de Bevy (à vérifier dans les sources), écrit dans le `.json` de la capture. Sans cela, le budget ne se mesure pas.
- **Réglages de qualité** : ressource `QualitySettings` (bas, moyen, haut, ultra), lue par chaque fonction coûteuse.
- **Captures en 1440p** sur la RX 7800 XT, la définition du critère.

### 1. Nuages volumétriques

- **Densité** : fonction `cloud_density(position, temps)` écrite **deux fois**, en WGSL et en Rust, avec un test de parité :
  - carte météo 2D (couverture, type), qui dérive avec le vent ;
  - bruit 3D de forme, Perlin-Worley d'environ 128³, et bruit de détail, Worley d'environ 32³, **générés en Rust** au démarrage, de façon déterministe ;
  - profils verticaux de cumulus, stratocumulus et stratus.
- **Rendu** : marche de rayons en quart de résolution, reconstruction temporelle, bruit bleu, pas adaptatifs.
  - Extinction de Beer-Lambert, terme « powder », phase de Henyey-Greenstein à deux lobes, multidiffusion par octaves.
  - Lumière du soleil et du ciel tirée de l'atmosphère de Bevy, perspective aérienne sur les nuages lointains.
  - Composition avec la profondeur : un avion devant, dans ou derrière un nuage.
- **Ombres des nuages** sur le sol et sur l'avion.
- **Dans le nuage** : sans saut ni bande à l'entrée.
- **Côté jeu** : `Environment` peut donner la densité. La turbulence et la visibilité de l'IA viendront avec leurs jalons. La fonction sert dès maintenant aux tests.
- **ADR 0009** : la technique de rendu des nuages (décision 2).
- **Scènes** : `cumulus_midi` ; `traversee_nuage`, une suite de captures en entrant dans un nuage ; `plafond_bas`, sans pluie.

### 2. Heure du jour

- **Soleil calculé** à partir de la date, de l'heure et de la latitude des Flandres : algorithme astronomique en Rust, avec un test sur des éphémérides publiées.
- **Exposition** : caméra physique dont l'EV suit la luminosité, de l'aube à midi, ou exposition automatique de Bevy si elle convient (à vérifier).
- **Ciel nocturne simple** : la lune et quelques étoiles, sans plus ; la vraie nuit viendra avec les dirigeables.
- **Scènes** : `aube_terrain` (soleil rasant, brume basse, ombres longues) et un couchant sans combat.

### 3. Relief réel

- **Données** : un MNT ouvert, voir la décision 3. Projection UTM 31N, origine de la carte documentée.
- **`tools/terrain_bake`** :
  - GDAL en ligne de commande pour reprojeter et découper ;
  - Rust pour produire les tuiles de hauteur 16 bits, les normales et les masques ;
  - sortie déterministe, et aucune dépendance nouvelle : GDAL reste un outil externe, appelé hors du jeu.
- **Rendu** : quadtree à niveaux de détail continus (CDLOD) avec géomorphing, matériaux par splatting.
- **Hauteur de référence en Rust** : `Environment::ground_altitude` lit les tuiles. Le modèle de vol, le contact au sol du M2 et l'IA voient donc le même relief que l'image, avec un test de parité CPU/GPU.
- **Terrain d'aviation** : le pré du M2 posé sur le vrai relief, à l'emplacement historique de la SPA 3 en 1917 (historien).
- **ADR 0010** : relief et données.

### 4. Paysage de 1917

- **Occupation du sol** : champs, haies, bois, villages, routes, canaux, voies ferrées, sans aucun élément moderne. La méthode est la décision 4.
- **Le front** : ligne de front à la date choisie, densité de cratères selon la distance à la ligne (champ procédural avec graine), tranchées, boue, cratères inondés qui reflètent le ciel.
- **Végétation** : arbres instanciés, imposteurs au loin.
- **Historien** : cartes d'époque et leurs droits, tracé du front à la date retenue, emplacement des terrains de la SPA 3.
- **Scènes** : `no_mans_land` (rase-mottes au-dessus des tranchées) et `haute_altitude` (5 000 m, perspective aérienne sur le front).

### 5. Finition du SPAD

Les restes du plan `m3-modele-spad7.md`, déjà décidés :
- **B5** : bois vernis (mâts, hélice), métal (capot, ferrures), rugosité ;
- **seconde livrée** : les marques de S.254 au 20 octobre 1917 ;
- **B7, premier poste de pilotage** : siège, manche, palonnier, planche de bord avec les instruments de la fiche (cadrans en décalques), Vickers vue de l'intérieur ;
- **voile du disque d'hélice** : son bord net, relevé par la revue du 2026-11-01.

Scènes : `cockpit_detail`, `avion-studio` et `contre_jour_toile` recapturées. Les aiguilles animées viendront plus tard.

### 6. Budget et revue finale

- Mesure de chaque passe en 1440p : nuages ≤ 3 ms, terrain ≤ 3 ms, ombres ≤ 2 ms, post-traitement ≤ 1,5 ms (budgets indicatifs du skill), réglages de qualité ajustés.
- Revue du directeur artistique sur toutes les scènes du M3.
- **Ton essai** : les scènes regardées une à une, et un vol libre au-dessus du front.

## Décisions demandées au joueur

1. **Ordre des incréments** :
   - **(A, recommandé)** nuages, heure du jour, relief, paysage de 1917, finition du SPAD. Les nuages font l'identité du jeu et ne dépendent de rien ; le relief et le paysage demandent des données et de la recherche, qui avancent en parallèle ;
   - (B) le relief d'abord : il change le jeu (le sol réel sous les roues) avant de changer l'image ;
   - (C) la finition du SPAD d'abord, le poste de pilotage en tête.
2. **Rendu des nuages** :
   - **(A, recommandé)** une marche de rayons maison en WGSL (Schneider, Hillaire). C'est le plus de code, mais la seule voie vers des cumulus qu'on traverse, avec ombres et parité CPU ;
   - (B) les volumes de brouillard de Bevy (`FogVolume`, avec une texture 3D). Plus rapide à brancher, mais ce sont des boîtes, pensées pour un brouillard local et éclairées seulement par les lumières marquées : ils ne tiendront pas un ciel entier.
3. **Données de relief** (toute source passe par le registre des licences ; le téléchargement se fait avec ton accord, fichier par fichier) :
   - **(A, recommandé)** les MNT nationaux à 5 m : RGE ALTI de l'IGN (Licence Ouverte 2.0) côté français, DHMV II (licence flamande de réutilisation gratuite) côté belge. Ces licences demandent une attribution, comme le CC-BY. Elles ne figurent pas mot pour mot dans la liste de `CLAUDE.md` : il te faudrait les accepter ;
   - (B) SRTM à 30 m, domaine public. Aucune question de licence, mais trop grossier pour le rase-mottes, et son téléchargement officiel demande un compte NASA Earthdata ;
   - (C) commencer en B, passer en A plus tard.
4. **Occupation du sol de 1917** :
   - **(A, recommandé)** un paysage procédural, guidé par quelques couches que je numérise depuis des sources libres de droits : ligne de front, canaux et rivières, forêts principales, villes, routes majeures. C'est crédible vite, et précis là où cela se voit ;
   - (B) une numérisation complète dans QGIS, depuis les cartes de tranchées (1:10 000) et les plans directeurs. C'est le plus juste, mais c'est un long travail à la main, donc le tien ;
   - (C) A d'abord, puis B secteur par secteur.

   Les données modernes d'OpenStreetMap sont exclues : leur licence (ODbL) n'est pas dans la liste, et elles décrivent le paysage d'aujourd'hui.
5. **Étendue** :
   - **(A, recommandé)** relief sur les 80 × 80 km de la vision dès le départ (c'est peu coûteux), paysage détaillé d'abord sur une bande de 30 km autour d'Ypres et du terrain de la SPA 3 ;
   - (B) tout en détail sur 80 × 80 km dès ce jalon.

## Risques

- **Volume de travail** : c'est le plus gros jalon jusqu'ici, sur de nombreuses sessions. Chaque incrément livre une image regardée.
- **Performances** : nuages et relief se disputent le budget. Les réglages de qualité existent dès l'étape 0, et chaque passe est mesurée.
- **Données d'époque** : les cartes de 1917 et leurs droits ne sont pas encore réunis (historien). L'option A de la décision 4 n'en demande qu'une partie.
- **Sources d'altitude** : les MNT modernes portent des remblais, des carrières et des terrils récents, à effacer.
- **Parité CPU/GPU** : deux implémentations de la densité et du relief, tenues égales par des tests.
- **Version de Bevy** : 0.20 au long du jalon, sans montée de version en cours de route.
