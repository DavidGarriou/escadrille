# Vision — Panache blanc

*Panache blanc* : d'après la phrase attribuée à Henri IV à Ivry (1590), « Ralliez-vous à mon panache blanc ». Le pilote seul là-haut, qui charge en tête et se fait remarquer. « Escadrille » reste le nom de code technique (dépôt, crates).

Document de référence du design. Les questions ouvertes sont en fin de document : les trancher avec le joueur-auteur, puis mettre à jour ce fichier.

## Pitch

Été 1917, Flandres. Vous êtes un jeune pilote de chasse qui rejoint une escadrille. Chaque matin : la brume sur le terrain, le mécanicien qui lance l'hélice à la main, puis la montée vers un ciel immense où l'ennemi peut surgir du soleil. Vous rentrerez, ou pas. Votre carnet de vol raconte votre guerre.

## Ce que nous prenons à nos références

| Référence | Ce que nous gardons |
|---|---|
| *Wings* (Cinemaware, 1990, Amiga) | Le récit : carnet de vol, camarades nommés qui disparaissent, progression du pilote. La variété des missions. La prise en main immédiate. |
| *Red Baron* (Dynamix, 1990 ; *Red Baron 3D*, 1998) | La carrière historique, la rencontre des as, les réglages de réalisme à la carte, le replay de mission. |
| *IL-2 Sturmovik* et *Flying Circus* | La profondeur du modèle de vol, la gestion moteur, le modèle de dégâts, le son, l'immersion dans le cockpit. |

Écueils à éviter : une barrière d'entrée raide sans accompagnement ; des missions répétitives ; des séquences de jeu déconnectées de la simulation.

## Les dix premières minutes

1. **Menu** sur fond de terrain d'aviation à l'aube, avions alignés, brume basse.
2. **Une seule question** : « Comment voulez-vous voler ? » Trois portes : *Pour le plaisir* (Arcade), *Comme un pilote* (Pilote), *Comme en 1917* (As). Tout reste modifiable ensuite, réglage par réglage.
3. **Premier vol guidé** à l'école de pilotage. Piste à étudier : le « Pingouin » des écoles françaises (avion aux ailes rognées qui ne décolle pas, pour apprendre le roulage), puis un premier vol court.
4. **Arrivée à l'escadrille** : le capitaine, deux ou trois camarades nommés, un avion à soi.
5. **Première patrouille** en formation, sans combat garanti : apprendre à regarder le ciel.

## Boucle de mission

Briefing (carte, objectif, météo, formation) → préparation (avion, livrée) → décollage (terrain en herbe, vent) → transit (formation, navigation à vue, accélération du temps possible) → engagement → retour (carburant, dégâts, retrouver son terrain) → atterrissage → débriefing (revendications, témoins, entrée du carnet).

Une mission dure de 10 à 25 minutes en temps réel. L'accélération du temps et le « saut jusqu'au contact » sont des options.

## Méta-jeu : la carrière

- **Chronologie historique** : la carrière avance date par date ; les avions disponibles, amis et ennemis, suivent l'histoire.
- **Escadrille vivante** : camarades avec nom, visage, caractère et palmarès ; pertes et remplaçants ; as historiques croisés dans le ciel.
- **Victoires revendiquées** et **confirmées** seulement avec témoins ou épave dans nos lignes.
- **Promotions et décorations** selon la nation.
- **Carnet de vol** généré après chaque mission (gabarits Fluent), ton sobre.
- **Blessure et mort** selon le réglage : blessure et convalescence, ou mort permanente qui clôt la carrière.

Autres modes : vol libre, combat rapide, et plus tard un éditeur de missions.

## Types de mission

| Mission | Intérêt de jeu |
|---|---|
| Patrouille offensive | chasser au-delà des lignes, gérer carburant et vent dominant |
| Patrouille de barrage | défendre un secteur, intercepter |
| Escorte de biplaces (réglage d'artillerie, photo) | protéger plus lent que soi, discipline de formation |
| Attaque de ballon d'observation (*Drachen*, saucisse) | cible très défendue (DCA, chasse), approche à basse altitude |
| Chasse aux dirigeables (Zeppelin, Schütte-Lanz) | cible immense, très haute, défendue par ses mitrailleurs ; l'enflammer ou la forcer à se poser ; de jour ou de nuit (`docs/gdd/mission-dirigeables.md`) |
| Interception de biplaces | ennemi qui riposte (mitrailleur arrière) |
| Mitraillage de tranchées (1918) | vol à basse altitude au-dessus du no man's land |
| Rencontre avec un as | événement scénarisé rare |
| Vol d'essai, convoyage | vol calme, contemplation, apprentissage |

## Niveaux de réalisme

Un modèle physique unique. Les préréglages ne sont que des combinaisons de réglages indépendants (implémentation : skill `realisme`). Ce tableau est la **source de vérité** des préréglages.

| Réglage | Arcade « Wings » | Pilote « Red Baron » | Vétéran | As « IL-2 » |
|---|---|---|---|---|
| Assistance de pilotage | instructeur virtuel (commande d'attitude) | coordination auto + garde anti-décrochage douce | coordination auto | aucune |
| Effets gyroscopiques et couple | 20 % ⚠ | 60 % ⚠ | 100 % | 100 % |
| Gestion moteur | automatique | manette des gaz seule | gaz + coupe-contact | complète (mélange, coupe-contact, températures) |
| Pannes et enrayages | non | rares | oui | oui (chaleur, rafales longues) |
| Structure | indestructible ⚠ | renforcée ×1,5 ⚠ | réelle | réelle |
| Munitions et carburant | illimités ⚠ | réels | réels | réels |
| Vent et turbulence | léger | oui | oui | oui |
| Munitions spéciales (incendiaires, explosives) | automatiques, inflammation rapide ⚠ | composition automatique | composition à choisir | à choisir, chronologie stricte |
| Aide de visée | indicateur de tir | non | non | non |
| HUD et marqueurs | complets | à courte distance | aucun | aucun |
| Vues | toutes | toutes | cockpit + externe limitée | cockpit seul |
| Effets du facteur de charge, blessures | non | atténués | oui | oui |
| Effets de l'altitude (froid, manque d'oxygène) | non | atténués | oui | oui |
| Atterrissage | tolérant | normal | réel | réel |
| Mort du pilote | non | blessure | au choix | permanente |

⚠ = réglage non physique, signalé comme tel dans l'interface.

## Avions

Lot visé (à confirmer par la recherche, dates de service approximatives) :

| Entente | Empires centraux |
|---|---|
| Nieuport 17 (1916, rotatif Le Rhône) | Fokker E.III (1915, gauchissement, mitrailleuse synchronisée) |
| SPAD VII (1916-1917, Hispano-Suiza V8) | Albatros D.III / D.V (1917, Mercedes D.III) |
| SPAD XIII (1917-1918) | Fokker Dr.I (1917-1918, rotatif Oberursel) |
| Sopwith Camel (1917-1918, rotatif Clerget) | Fokker D.VII (1918) |
| S.E.5a (1917-1918) | |

**Premier couple jouable (validé le 17/10/2026)** : SPAD VII de l'escadrille SPA 3 « Cigognes » contre Albatros D.V, Flandres, été 1917. Deux moteurs en ligne (FDM plus simple à valider d'abord), deux avions bien documentés qui se sont affrontés. Les rotatifs (Camel, Dr.I) arrivent au M6.

## Théâtre

- **Flandres 1917** : de la côte (Nieuwpoort, Dunkerque) à Lille et Armentières, environ 80 × 80 km.
- **Relief réel** issu d'un MNT open data ; **occupation du sol reconstituée pour 1917** d'après les cartes d'époque (pas de routes ni de zones industrielles modernes).
- **Le front** : bande dévastée, champs de cratères, cratères inondés (Passchendaele, automne 1917), villes en ruines (Ypres), forêts (Houthulst), canaux (Yser), aérodromes historiques à documenter.
- **Météo** : nuages bas, pluie et brouillard font partie du jeu (missions annulées, plafond bas, visibilité).

## Direction artistique

- **Réalisme lumineux** : photoréaliste par la lumière et les matières. Palette et atmosphère inspirées des **autochromes Lumière** (photographies couleur de l'époque) ; photos aériennes de reconnaissance comme référence du paysage du front.
- **Moments-clés** (scènes de référence, voir skill `rendu-visuel`) : aube sur le terrain, champ de cumulus, contre-jour à travers la toile des ailes, no man's land en rase-mottes, combat au soleil couchant, plafond bas sous la pluie, haute altitude.
- **Détails qui font l'époque** : toile translucide qui laisse voir les nervures, huile de ricin projetée par les rotatifs, câbles qui vibrent, hélice en bois vernie, camouflage à losanges imprimé, rubans de commandant.
- **Filtre d'ambiance optionnel** « actualités d'époque » : grain, sépia ou noir et blanc, scintillement.

## Son

Le son porte la moitié de l'immersion : moteur rotatif coupé par le coupe-contact, chant des câbles qui monte avec la vitesse, claquement des mitrailleuses, grondement continu de l'artillerie près du front. Voir skill `audio`.

## Plateformes et performances

- Linux (Fedora) d'abord, Windows ensuite (Bevy est multiplateforme).
- Cible : 60 images/s minimum en 1440p sur le GPU de référence, en préréglage de qualité « haut » ; préréglages de qualité de « bas » à « ultra ».
- **GPU de référence** : AMD Radeon RX 7800 XT 16 Go GDDR6 (RDNA 3 ; modèle XFX Speedster MERC319, RX-78TMERCB9), sous Fedora avec le pilote Vulkan libre Mesa RADV. Budget d'image : 16,6 ms à 60 images/s. Mémoire vidéo : viser au plus 12 Go en « haut », pour laisser de la marge au système et au compositeur Wayland.

## Hors périmètre initial

Multijoueur ; réalité virtuelle (OpenXR plus tard) ; biplaces jouables (mitrailleur arrière) ; bombardiers ; fronts d'Italie et d'Orient ; campagne stratégique dynamique.

## Questions ouvertes

1. ~~Nom définitif du jeu~~ — **tranché le 17/10/2026** : *Panache blanc* (remplace *Ricin*, retenu puis abandonné le même jour). Aucun jeu homonyme trouvé ; recherche de marque (INPI, EUIPO) à faire avant toute publication. Titre anglais à décider (garder le français ou traduire).
2. ~~GPU de référence~~ — **tranché le 16/10/2026** : AMD Radeon RX 7800 XT 16 Go (voir « Plateformes et performances »).
3. ~~Nation et secteur du premier jalon jouable~~ — **tranché le 17/10/2026** : France, SPA 3, Flandres, été 1917 ; SPAD VII contre Albatros D.V. Dates et dotations à faire vérifier par l'historien.
4. ~~Licences~~ — **tranché le 17/10/2026** : code et fichiers de données (`.ron`, `.ftl`) sous MIT (`LICENSE`) ; assets (`assets/`), sources artistiques (`art/`) et documentation (`docs/`) sous CC-BY-SA 4.0 (`LICENSE-ASSETS`). Les fichiers tiers gardent leur licence, inscrite dans `assets/LICENCES.md`.
5. SN30 Pro : gâchettes L2/R2 analogiques ou tout-ou-rien ? Vibrations sous Linux ? (vérification au M0 avec le labo manette).
6. ~~Hébergement du dépôt~~ — **tranché le 17/10/2026** : GitHub (`DavidGarriou/escadrille`), parce que les sessions cloud de Claude Code et le suivi des pull requests en dépendent ; miroir Codeberg possible plus tard.
7. ~~Ton du récit~~ — **tranché le 17/10/2026** : sobre et réaliste (esprit des *Croix de bois*, de *La Grande Illusion*).
8. ~~Théâtre de la première mission de chasse aux dirigeables~~ — **tranché le 17/10/2026** : le « Raid silencieux », France, octobre 1917 (voir `docs/gdd/mission-dirigeables.md`). Faits à sourcer par l'historien dans `docs/recherche/dirigeables.md`.
