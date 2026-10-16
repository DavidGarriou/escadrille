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

`cargo run -p escadrille --release -- --capture <scène> [--frames 240] [--size 2560x1440] [--out target/captures/<scène>.png]`

- La scène est décrite dans `assets/scenes/<scène>.ron` : caméra (position, cible, focale), date et heure, météo, graine, avions et leur état, préréglage de qualité.
- Déroulé : chargement complet des assets → N images pour stabiliser (TAA, exposition automatique, accumulation temporelle des nuages) → capture avec l'API de capture d'écran de la version épinglée → écriture à côté d'un `.json` (temps CPU et GPU par passe, si disponibles) → sortie de l'application.
- Temps de simulation figé ou déterministe ; UI et debug masqués sauf demande.
- Code de sortie non nul si un asset manque ou si un shader échoue : une capture doit être fiable pour la revue.

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
