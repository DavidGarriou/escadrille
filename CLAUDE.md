# CLAUDE.md — Panache blanc

Simulateur de combat aérien de la Première Guerre mondiale, en Rust, sur le moteur open source Bevy.
*Le jeu s'appelle « Panache blanc ». « Escadrille » reste le nom de code technique : dépôt, crates (`escadrille_fdm`…), identifiants.* Code sous licence MIT.

Ce fichier est lu à chaque session : il ne contient que l'essentiel. Le détail est dans `docs/` et dans les skills (`.claude/skills/`), à charger selon la tâche.

## Le jeu en une phrase

Le joueur incarne un pilote de chasse de 1914-1918 et vit sa guerre mission après mission, dans un ciel magnifique, avec un modèle de vol qui va du plaisir immédiat de *Wings* (Amiga) à l'exigence d'*IL-2 Sturmovik*, en passant par la carrière de *Red Baron*.

Vision complète : `docs/vision.md` · Jalons : `docs/roadmap.md` · Décisions : `docs/adr/`.

## Piliers

1. **Sentir le vol.** Même dans le mode le plus simple, on pilote une machine fragile de bois, de toile et de câbles : couple, inertie, vent et décrochage se ressentent.
2. **Un seul monde, un curseur de réalisme.** Un unique modèle physique ; le réalisme se règle par couches d'assistance et de présentation, jamais par un second jeu caché dans des `if`.
3. **Le ciel est le personnage principal.** Nuages, lumière, brume et paysage du front font l'identité visuelle, et sont aussi des éléments de jeu (se cacher dans un nuage, attaquer depuis le soleil).
4. **Une histoire humaine.** Un pilote, une escadrille, des camarades qui ne reviennent pas, un carnet de vol (*Wings*).
5. **Authentique et sourcé.** Chaque chiffre historique a une source et un niveau de confiance.

**Arbitrage** : dans les préréglages réalistes, l'authenticité l'emporte sur le confort. Dans les préréglages fun, le confort l'emporte, sans jamais casser la cohérence physique visible (un avion ne vire pas sans s'incliner et ne monte pas sans perdre de vitesse).

## Contraintes non négociables

- **Rust uniquement** (édition 2024, toolchain stable). Shaders en WGSL. Pas de code C/C++ maison ; les bibliothèques système (Vulkan, ALSA, udev, Wayland) sont acceptées.
- **100 % open source** : moteur, crates, outils, données et assets.
  - Licences de code acceptées : MIT, Apache-2.0, BSD-2/3-Clause, Zlib, ISC, MPL-2.0, BSL-1.0, CC0-1.0, Unicode-3.0. Contrôle automatique : `cargo deny check licenses`.
  - Assets et données : CC0, CC-BY, CC-BY-SA, domaine public, ou création maison. Chaque fichier tiers est inscrit dans `assets/LICENCES.md`.
  - Aucun outil propriétaire dans la chaîne (ni Substance, ni Houdini, ni World Machine, ni Gaea…).
- **Moteur : Bevy.** Voir la discipline de version ci-dessous.
- **Manette d'abord : 8BitDo SN30 Pro.** Tout passe par une couche d'actions remappable ; clavier et souris en secours pour le développement ; HOTAS, palonnier et opentrack plus tard.
- **Beauté vérifiée.** Toute modification visuelle est validée sur captures d'écran des scènes de référence, jamais à l'aveugle.

## Bevy : discipline de version (IMPORTANT)

L'API de Bevy change à chaque version mineure, et ta mémoire d'entraînement a probablement du retard sur la version utilisée.

- Version **exacte** épinglée dans le `Cargo.toml` du workspace (`bevy = "=0.X.Y"`) : la plus récente que supportent toutes les dépendances couplées à Bevy. Repère au 16/10/2026 : Bevy 0.20 est sorti le 08/10/2026 et 0.19 en juin 2026. Vérifier la compatibilité de l'écosystème avant de choisir, et consigner le choix dans un ADR.
- **Ne jamais deviner une API.** Vérifier dans les sources locales de la version épinglée (`~/.cargo/registry/src/*/bevy_*-<version>/`), dans les exemples officiels du tag correspondant et dans les guides de migration.
- **Limiter les dépendances couplées à Bevy**, car chacune freine les montées de version. Préférer des crates indépendantes du moteur (kira, parry3d-f64, glam…) appelées depuis nos plugins.
- Une montée de version est une tâche dédiée, sur une branche, avec un ADR. Détails : skill `bevy`.

## Stack

| Rôle | Choix |
|---|---|
| Moteur, rendu, ECS | `bevy` (version exacte) |
| Modèle de vol | crate maison `escadrille_fdm`, **sans dépendance à Bevy**, f64, déterministe |
| Maths | `glam` (types f64 `DVec3`/`DQuat` dans la simulation) ; `nalgebra` seulement dans les outils (valeurs propres) |
| Requêtes géométriques | `parry3d-f64` (rayons, volumes d'impact) ; pas de moteur physique généraliste |
| Grand monde | origine flottante maison : positions monde en f64, `Transform` relatifs à la caméra |
| Entrées | API manette de Bevy (gilrs) + couche d'actions maison |
| Particules | `bevy_hanabi` |
| Audio | `kira` + `fundsp` (synthèse procédurale des moteurs) |
| UI de debug | `bevy_egui`, `egui_plot` |
| Données | `serde` + `ron` |
| Localisation | `fluent` (fr, en) |
| Tracés d'essais | `plotters` (PNG que Claude peut lire) |
| Profilage | Tracy (feature `trace_tracy`), RenderDoc |
| Outils de contenu | Blender, Krita, GIMP, Inkscape, Material Maker, KTX-Software, QGIS + GDAL, Audacity, Ardour |

Ces choix sont des points de départ : les confirmer ou les remplacer par un ADR pendant le M0, après vérification des versions et des licences.

## Architecture

```
Cargo.toml              workspace, versions épinglées, profils
crates/
  fdm/                  escadrille_fdm : atmosphère, corps rigide 6DDL, aéro, hélice, moteurs, sol, structure
  data/                 schémas des fichiers .ron (avions, armes, essais), chargement, conversions d'unités, validation
  game/                 binaire Bevy, un plugin par domaine :
    src/flight/         pont FDM <-> ECS, pas fixe, interpolation
    src/input/          couche d'actions, profils manette, calibration
    src/realism/        réglages, préréglages, assistances
    src/camera/         cockpit, poursuite, regard libre, padlock
    src/world/          origine flottante, terrain, ciel, nuages, météo
    src/render/         matériaux, post-traitement, shaders
    src/combat/         armes, balistique, dégâts, IA
    src/audio/          moteur, vent, armes, ambiance
    src/ui/             HUD, menus, carnet de vol
    src/campaign/       missions, escadrille, carrière
    src/dev/            mode capture, labo manette, gizmos de forces, télémétrie
tools/
  fdm_lab/              CLI sans Bevy : trim, essais en vol, modes propres, tracés PNG
  terrain_bake/         MNT (GDAL) -> tuiles de hauteur et masques
  asset_check/          vérifie les .glb (nœuds requis, dimensions comparées à la fiche avion)
assets/                 aircraft/<id>/ (glb, aircraft.ron, essais.ron), scenes/, shaders/, terrain/, audio/, i18n/
art/                    fichiers sources (.blend, .kra…)
docs/                   vision, roadmap, adr/, gdd/, recherche/, essais/
```

Règles :
- **`escadrille_fdm` ne dépend ni de Bevy ni d'aucun moteur.** Il se teste en ligne de commande et sert au jeu, à l'IA, aux outils et aux tests.
- **Repères** : le FDM suit les conventions aéronautiques (corps : x avant, y droite, z bas ; monde local NED). Bevy est Y-haut, -Z avant. **Une seule** fonction de conversion, dans `crates/game/src/flight/bridge.rs`, couverte par des tests.
- **Unités SI partout** dans le code (m, kg, s, rad, N, W, Pa). Les unités d'époque (mph, ft, hp, PS) n'apparaissent que dans les fiches de recherche, dans les fichiers de données (valeurs typées, converties au chargement par `crates/data`) et dans l'UI.
- **Pas fixe** : simulation dans `FixedUpdate` (cible 120 Hz, sous-pas du FDM si nécessaire), rendu interpolé. À entrées et graine égales, la simulation est déterministe (replays, tests).
- **L'IA pilote avec les mêmes commandes que le joueur**, sur le même FDM. Aucune triche physique.
- **Réalisme** : une ressource `RealismSettings` de réglages indépendants, plus des préréglages ; jamais de `if arcade` disséminé (skill `realisme`).
- **Parité CPU/GPU** : tout ce qui est rendu *et* agit sur le jeu (densité des nuages, hauteur du terrain) a une implémentation Rust de référence et un test de parité avec le shader.

## Commandes (justfile, créé au M0)

| Commande | Effet |
|---|---|
| `just check` | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check` |
| `just test` | `cargo nextest run --workspace` |
| `just run` | lance le jeu en dev (`bevy/dynamic_linking`) |
| `just essai <avion>` | essais en vol sans rendu via `fdm_lab` ; rapport et tracés dans `target/essais/<avion>/` |
| `just capture <scène>` | rend une scène de référence dans `target/captures/<scène>.png` |
| `just labo-manette` | écran de diagnostic de la manette |
| `just release` | build optimisé |

## Façon de travailler

- **Petits incréments toujours jouables.** Une tâche n'est terminée que si `just check` et `just test` passent.
- **Plan avant code** pour toute tâche de plus d'une heure ou qui touche plusieurs plugins ; faire valider le plan.
- **FDM** : toute modification passe par des tests unitaires, `/essai-en-vol` et une revue du subagent `aerodynamicien`. Ne jamais ajuster un coefficient pour faire passer un essai sans justification physique ou source, notée dans le fichier de données.
- **Visuel** : toute modification passe par `/capture` avant/après. **Regarder les images.** Les changements importants sont revus par le subagent `directeur-artistique`.
- **Histoire** : chiffres, dates, livrées, unités et personnages vont dans des fiches `docs/recherche/` sourcées ; subagent `historien`. Jamais de chiffre inventé sans l'étiquette `ESTIMÉ` et sa justification.
- **Décisions structurantes** : ADR via `/adr`.
- **Demander avant** d'ajouter une dépendance, de changer la version de Bevy, de modifier une convention de ce fichier ou de supprimer des assets.
- **Le fun ne se valide pas par des tests** : chaque jalon se termine par un essai du joueur, manette en main. Proposer ce qu'il faut tester et consigner ses retours dans `docs/gdd/retours.md`.
- **Carnet de bord** : chaque question du joueur, réponse, action, décision et incident est consigné dans `docs/carnet-de-bord.md` (entrées datées, par session), puis commité et poussé avant de clore l'échange.
- **Langue** : discussions et docs en français ; identifiants, commentaires de code et messages de commit en anglais ; textes du jeu dans les fichiers Fluent.

## Skills

| Skill | Quand la charger |
|---|---|
| `simulation-vol` | modèle de vol, fichiers `aircraft.ron` et `essais.ron` |
| `bevy` | ECS, plugins, versions, compilation, mode capture |
| `rendu-visuel` | ciel, nuages, terrain, matériaux, VFX, shaders, performances GPU |
| `manette` | SN30 Pro, actions, courbes de réponse, vibrations |
| `realisme` | préréglages et assistances |
| `combat` | armes, balistique, dégâts, IA |
| `recherche-historique` | données et contenus historiques |
| `pipeline-assets` | Blender → glTF, textures, terrain, licences |
| `audio` | sons moteur, vent, armes, ambiance |
| `game-design` | missions, carrière, sensations, documents de design |
| `/nouvel-avion`, `/essai-en-vol`, `/capture`, `/adr` | workflows lancés à la demande |

Subagents (`.claude/agents/`) : `aerodynamicien`, `historien`, `directeur-artistique`.

## Environnement

Fedora Linux (Wayland), manette en USB-C ou Bluetooth. GPU de référence : AMD Radeon RX 7800 XT 16 Go (RDNA 3, pilote Mesa RADV). Dépendances système et outils : voir `README.md`. Éditeur de liens : mold.

## Jalon en cours

**M0 — Fondations** (voir `docs/roadmap.md`). Mettre à jour cette ligne à chaque changement de jalon.
