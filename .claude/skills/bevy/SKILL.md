---
name: bevy
description: Travail avec le moteur Bevy dans ce projet — vérification de l'API de la version épinglée, organisation en plugins, ordonnancement des systèmes (pas fixe, interpolation), origine flottante, temps de compilation, mode capture d'écran, débogage, montée de version. À charger avant d'écrire ou de modifier du code dans crates/game ou dans un outil qui dépend de Bevy.
---

# Bevy dans Escadrille

## 1. La vérité de l'API est dans la version épinglée, pas dans ta mémoire

Ta connaissance de Bevy date de ton entraînement, et les noms changent souvent d'une version à l'autre (bundles remplacés par des composants requis, événements renommés, requêtes « single » qui renvoient un `Result`, etc.). Avant d'écrire un appel :
1. Lire la version : `grep -n '^bevy' Cargo.toml` ou `cargo tree -p bevy --depth 0`.
2. Chercher dans les sources locales : `rg -n "fn nom_cherché" ~/.cargo/registry/src/*/bevy_*-<version>/src`.
3. Pour un usage complet, lire l'exemple officiel du même tag. Cloner une seule fois, **hors du projet** (répertoire temporaire de session) : `git clone --depth 1 --branch v<version> https://github.com/bevyengine/bevy`, puis lire `examples/`.
4. `cargo check` tôt et souvent ; lire l'erreur en entier avant de corriger.

Mêmes règles pour `bevy_egui` et `bevy_hanabi`.

## 2. Choisir ou changer de version

- Critère : la version la plus récente que supportent **toutes** les dépendances couplées à Bevy (vérifier leur `Cargo.toml` sur crates.io ou docs.rs, ou leur tableau de compatibilité).
- Montée de version : branche dédiée → guide de migration officiel → corrections de compilation → `just test` → `/capture all` et comparaison avec les captures précédentes → ADR. Jamais mélangée avec une fonctionnalité.
- L'ADR de version liste aussi les features Bevy activées et pourquoi.

## 3. Organisation

- Un `Plugin` par domaine (arborescence dans `CLAUDE.md`). Chaque plugin expose des `SystemSet` publics ; les autres plugins s'ordonnent par rapport à eux, jamais par rapport à des systèmes internes.
- États de jeu : `Boot`, `MainMenu`, `Briefing`, `InFlight`, `Debrief`, avec `Paused` en sous-état de `InFlight`. Systèmes conditionnés par `run_if(in_state(..))`.
- Composants petits et nombreux ; ressources pour la configuration globale (`RealismSettings`, `InputProfile`, `QualitySettings`).
- Configuration de jeu en fichiers RON dans `assets/`, rechargeable à chaud en dev.

## 4. Pipeline de simulation

```
FixedUpdate (cible 120 Hz)
  InputSet::Apply  →  RealismSet::Assist  →  FlightSet::Step (FDM, sous-pas)
  →  CombatSet (tirs, balistique, impacts)  →  DamageSet  →  AiSet::Decide (pour le pas suivant)

Update
  InputSet::Sample  →  FlightSet::Interpolate  →  OriginSet::Rebase
  →  CameraSet  →  AudioSet  →  UiSet
```

- L'état qui fait foi vit dans les composants du FDM (f64) ; `Transform` n'est qu'une vue pour le rendu.
- Conserver l'état précédent et l'état courant de chaque corps simulé ; interpoler avec la fraction de pas restante du temps fixe (méthode de `Time<Fixed>` : vérifier son nom dans la version épinglée).
- Les entrées sont échantillonnées dans `Update` et accumulées jusqu'au pas fixe suivant, sinon des appuis brefs se perdent.

## 5. Origine flottante

- Composant `WorldPos(DVec3)` en axes Bevy (Y haut), f64, origine au point de référence de la carte (projection UTM).
- Ressource `RenderOrigin(DVec3)` : position de la caméra active, recalée par paliers (par exemple 1 km) pour ne pas réécrire chaque image les transforms des objets statiques.
- Système `Rebase` : `Transform.translation = (WorldPos − RenderOrigin).as_vec3()`.
- Terrain et décor statiques : positionnés par tuiles, rebasés uniquement quand l'origine change.
- Terre plate locale (carte ≤ 100 km) ; la courbure peut être simulée dans le shader du terrain lointain.

## 6. Temps de compilation

`.cargo/config.toml` :
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

`Cargo.toml` du workspace :
```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
lto = "thin"
codegen-units = 1
```

- `bevy/dynamic_linking` uniquement via `just run` (feature de dev), jamais en release ni pour les captures.
- `escadrille_fdm` reste sans Bevy : ses tests compilent et tournent en quelques secondes.
- Désactiver les features Bevy inutiles dès que le besoin est connu.

## 7. Mode capture (vérification visuelle par Claude)

`just capture <scène> [--frames N] [--size 2560x1440] [--quality haut] [--out target/captures/<scène>.png]` (code : `crates/game/src/dev/capture.rs`)

- **Scènes d'un vol** : fichiers `assets/scenes/<scène>.ron` (`escadrille_data::scene`) : description, scénario de départ (`assets/scenarios`), caméra (`Chase` ou `Pilot`, lacet et tangage), commandes tenues, soleil, heure (`time`, heure légale française de 1917 : soleil, lune, étoiles, brume de l'heure), nuages, visibilité au sol (la brume ; 30 km par défaut), nombre d'images, et une **séquence** facultative : des images antérieures du même vol, enregistrées en plus sous `<scène>-<image>.png` (`traversee_nuage`). La recette garde aussi les précédentes en `.avant.png`. Une scène fichier se pilote aussi manette en main : `just run --scene <scène>` (ressource `FlightScene`). Les manettes partent des commandes de la scène, et le redémarrage ramène à son départ ; les poses construites en code ne se pilotent pas. Les scènes qui posent l'avion à la main (`m0-sol`, `maquette-spad7`, `avion-studio`, `contre_jour_toile`, `degats-spad7`, `degats-spad7-proche`) restent construites en code (`world::scenes::CODE_SCENES`). Le format grandit avec le M3 : météo, graine.
- **Rendu hors écran** : les caméras rendent dans une image à la taille demandée (1440p par défaut, la définition du budget), jamais dans la fenêtre, que le compositeur rogne à la zone utile de l'écran.
- **Temps déterministe** : l'horloge du jeu attend la fin du chargement, puis chaque image avance de 1/60 s (deux pas du modèle de vol). `--frames` compte donc du temps de jeu : 120 images, 2 s. Deux captures de la même scène ne diffèrent que de quelques pixels, sauf avec des nuages : la gigue de leur marche suit le compteur d'images de Bevy, qui inclut les images de chargement, et 0,8 à 1,2 % des pixels de `cumulus_midi` changent de plus de 8 niveaux d'une capture à l'autre (mesuré le 2026-11-08). Une régression se juge au-delà de ce bruit.
- **Aucune entrée en direct** : pendant une capture, la manette et le clavier sont ignorés (`input::read_controls`). Les commandes sont celles du départ ou celles que tient la scène. Une manette restée branchée faisait tourner la caméra de quelques degrés d'une capture à l'autre, par la dérive de son stick. Contrôle : deux captures de suite d'une même scène doivent différer de moins de 10⁻⁴ en moyenne (`magick compare -metric MAE`).
- Déroulé : chargement complet des assets → N images pour stabiliser (TAA, accumulation temporelle des nuages) → `Screenshot::image` → `.json` à côté → sortie de l'application.
- **Le `.json`** : définition, images, `mean_frame_ms` (temps réel, moyenne de la seconde moitié des images), `quality`, adaptateur, `gpu_passes_ms` (temps GPU de chaque passe, mesuré par `RenderDiagnosticsPlugin`, ajouté en mode capture seulement) et `gpu_frame_ms` (somme des passes de premier niveau ; le travail non chronométré n'y est pas).
- La recette garde la capture précédente en `<scène>.avant.png` et `.avant.json`.
- UI et debug masqués sauf demande.
- Code de sortie non nul si un asset manque ou si un shader échoue : une capture doit être fiable pour la revue.

## 7 bis. Passes de rendu maison (Bevy 0.20)

- **Le graphe de rendu est un schedule**, `Core3d`, dont chaque passe est un système ordonné dans les ensembles `Prepass`, `MainPass`, `EarlyPostProcess` et `PostProcess` (`bevy_core_pipeline::schedule`). Une passe s'y ajoute avec `.after(main_opaque_pass_3d)`, `.before(main_transparent_pass_3d)`, etc. ; elle lit sa vue avec `ViewQuery` (sautée si la vue n'a pas les composants) et enregistre avec `RenderContext`. Modèle : l'exemple `shader_advanced/custom_post_processing.rs` du tag.
- **Un système privé de Bevy ne se nomme pas** : ni `.after()`, ni l'ensemble implicite `SystemTypeSet` (son identifiant interné se compare par adresse), ni une passe de construction (`ScheduleBuildPass`), dont le trait impose un type d'`indexmap`. C'est le cas de `render_sky`. Parade des nuages (ADR 0009) : écrire dans sa propre image, à un endroit où l'ordre avec ce système n'importe pas, puis composer par un matériau transparent plein écran, premier dessiné de la passe des transparents grâce à `Material::depth_bias` (tri croissant, de l'arrière vers l'avant).
- **Réutiliser l'atmosphère** : `crates/game/src/render/clouds/pass.rs` recrée la disposition du groupe de `render_sky`, dont toutes les ressources sont publiques, et le shader importe `bevy_pbr::atmosphere::functions`. À revérifier à chaque montée de version.
- **Un GPU dans un test**, sans application : `bevy::render::renderer::initialize_renderer` (test de parité de `render/clouds.rs`).

## 8. Débogage

- Gizmos (feature `dev`, touches de debug) : portance et traînée par bande, poussée, vent relatif, CG, trajectoires des balles, volumes d'impact.
- `bevy_egui` : télémétrie (α, β, vitesses indiquée et vraie, facteur de charge, régime, gaz, carburant), réglages de réalisme et de qualité modifiables à chaud.
- Télémétrie : `--telemetry out.csv` enregistre l'état du FDM à chaque pas fixe ; `fdm_lab plot` le trace.
- Profilage : feature `trace_tracy` + Tracy ; RenderDoc pour les passes GPU.

## 9. Pièges

- Ordre de systèmes non spécifié : le comportement dépend de l'ordonnanceur. Toujours ordonner explicitement ce qui touche au même état.
- `GlobalTransform` n'est à jour qu'après la propagation des transforms (PostUpdate).
- Commandes différées : une entité créée n'existe qu'après l'application des commandes.
- Jamais de position monde en f32 en dehors du rendu.
- Espaces colorimétriques : sRGB pour la couleur de base uniquement ; normales, rugosité et masques en linéaire.
- **Capturer une version antérieure** : la construire dans un worktree avec le même `CARGO_TARGET_DIR` écrase les artefacts des crates du projet, qui ont les mêmes empreintes (chemins relatifs à l'espace de travail). Cargo croit ensuite la version courante à jour et ne refait pas le binaire, qui pointe encore vers les assets du worktree (`ASSETS_DIR` est fixé à la compilation). Après coup : `cargo clean --release -p escadrille -p escadrille_weather -p escadrille_data -p escadrille_fdm`, puis reconstruire.
- `NormalPrepass` (exigée par la SSAO) : la passe principale lit les normales de la prépasse, ce qui fausse les matériaux transparents et le clearcoat. Voir `render/own_normals.rs`.
- Toute `DirectionalLight` est un soleil pour l'atmosphère (diffusion, sol de la planète) : pas de lumière directionnelle d'appoint en extérieur.
- Les cartes d'environnement générées sont filtrées dans `Render` avant le graphe, sur ce que l'image précédente a laissé dans leur cube : une passe du graphe peut y dessiner (`render/clouds/pass.rs`, `light_environment`).
