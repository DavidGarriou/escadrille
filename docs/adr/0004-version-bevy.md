# 0004 — Version de Bevy : 0.20.0, sans bevy_hanabi pour l'instant

- Statut : Accepté (validé par le joueur le 2026-10-19)
- Date : 2026-10-19

## Contexte

Le M0 commence par le workspace Cargo, qui doit épingler une version **exacte** de Bevy (`CLAUDE.md`, « discipline de version »). La version retenue doit être la plus récente que supportent toutes les dépendances couplées à Bevy prévues dans la stack : `bevy_egui` (UI de debug) et `bevy_hanabi` (particules).

Relevé fait le 2026-10-19 sur l'index et l'API de crates.io (`index.crates.io`, `crates.io/api/v1`), plus la branche `main` de bevy_hanabi sur GitHub :

| Crate | Dernière version | Publiée le | Bevy requis | MSRV (Rust) | Licence |
|---|---|---|---|---|---|
| `bevy` | 0.20.0 | 2026-10-08 | — | 1.97.1 | MIT OR Apache-2.0 |
| `bevy` | 0.19.1 | 2026-08-13 | — | 1.95.0 | MIT OR Apache-2.0 |
| `bevy_egui` | 0.43.1 | 2026-10-10 | `^0.20.0` | 1.95.0 | MIT |
| `bevy_egui` | 0.42.0 | 2026-08-16 | `^0.19` | — | MIT |
| `bevy_hanabi` | 0.19.0 | 2026-06-27 | `^0.19` | 1.95.0 | MIT OR Apache-2.0 |

- **bevy_egui** suit déjà Bevy 0.20 (0.43.0 publiée le lendemain de Bevy 0.20, 0.43.1 deux jours après).
- **bevy_hanabi** n'a pas encore de version pour Bevy 0.20. Sa branche `main` est en `0.20.0-dev` et dépend de `bevy = "0.20"` : le portage est en cours. Délai historique entre une version de Bevy et la version compatible de bevy_hanabi : 10 jours (0.15), 37 jours (0.16), 5 jours (0.17), 19 jours (0.18), 8 jours (0.19). Nous sommes à 11 jours de la sortie de Bevy 0.20.
- **Toolchain** : Bevy 0.20 exige Rust ≥ 1.97.1. La stable actuelle est 1.99.1 (2026-10-13). Le conteneur cloud livre 1.97.0, insuffisant.
- **Besoin réel** : le M0 (fondations, labo manette, mode capture, premier FDM) n'utilise pas de particules. Les effets (fumées, traçantes, DCA) arrivent avec le combat, plus tard dans la feuille de route.

## Options

1. **Bevy 0.19.1 + bevy_egui 0.42 + bevy_hanabi 0.19** : tout l'écosystème est compatible aujourd'hui. Mais on démarre sur une version déjà dépassée, et la première montée de version (0.19 → 0.20) tombera dans quelques semaines, sur du code fraîchement écrit. Coût : une migration certaine à court terme.
2. **Bevy 0.20.0 + bevy_egui 0.43.1, bevy_hanabi ajouté plus tard** : on démarre sur la version actuelle, sans migration à court terme. bevy_hanabi n'est ajouté qu'au moment où le premier effet de particules est nécessaire ; sa version compatible 0.20 sera très probablement publiée d'ici là. Coût : si bevy_hanabi tardait beaucoup, il faudrait attendre, écrire un système de particules minimal maison, ou en choisir un autre (ADR dédié).
3. **Bevy 0.20.0 + bevy_hanabi depuis sa branche Git** : tout de suite complet, mais dépendance non publiée, non versionnée, potentiellement cassée à chaque commit ; contraire à la discipline de version. Rejetée.
4. **Attendre la sortie de bevy_hanabi pour Bevy 0.20** avant de commencer le M0 : bloque le projet pour une dépendance dont le M0 n'a pas besoin. Rejetée.

## Décision

**Option 2.** Épingler `bevy = "=0.20.0"` et `bevy_egui = "=0.43.1"`. Ne pas ajouter `bevy_hanabi` au M0 ; l'ajouter, dans une version compatible avec Bevy 0.20, lors de la tâche qui introduit le premier effet de particules.

Raison principale : partir de la version actuelle évite une migration certaine dans quelques semaines, et le seul manque (les particules) ne concerne pas le M0.

## Conséquences

- **Toolchain** : fixer la version de Rust dans `rust-toolchain.toml` (canal `1.99.1`, composants `rustfmt` et `clippy`) et déclarer `rust-version = "1.97.1"` dans le workspace. Le hook de démarrage cloud (`.claude/hooks/session-start.sh`) devra installer cette toolchain, car le conteneur fournit 1.97.0. Cette modification fait partie du plan du M0, à valider.
- **API** : ne jamais écrire de code Bevy de mémoire ; vérifier dans les sources locales `bevy_*-0.20.0` et les exemples du tag `v0.20.0` (skill `bevy`).
- **À surveiller** : la publication de `bevy_hanabi` 0.20 (et son MSRV) ; une éventuelle `bevy` 0.20.1 (correctifs) ; une nouvelle version de `bevy_egui` 0.43.x. Une montée de correctif (0.20.0 → 0.20.x) reste une tâche dédiée, mais sans ADR si l'API ne change pas.
- **Prochaine montée mineure** (Bevy 0.21, vers 2027 si le rythme d'environ quatre mois se maintient) : tâche dédiée sur une branche, avec un nouvel ADR.
- **Licences** : toutes compatibles avec la liste de `CLAUDE.md` ; `cargo deny check licenses` le confirmera sur l'arbre complet des dépendances lors de la création du workspace.
- **Si bevy_hanabi n'est toujours pas disponible** au moment des premiers effets : nouvel ADR pour choisir entre attendre, un système minimal maison ou une autre crate.

## Features de Bevy activées

Le workspace prend les features par défaut de Bevy (`2d`, `3d`, `ui`, `audio`). S'y ajoutent, pour le jeu seul (`crates/game/Cargo.toml`) :

| Feature | Depuis | Pourquoi |
|---|---|---|
| `pbr_transmission_textures` | 2026-10-27 | Toile translucide à contre-jour (plan `m3-modele-spad7.md`, B5 et B6) : la transmission diffuse varie sur la toile, plus faible sous les rubans, presque nulle sur les nervures et les longerons. |

Le plan B6 prévoit aussi `pbr_anisotropy_texture`, `pbr_clustered_decals` et `ktx2` (déjà dans les features par défaut, avec `zstd_rust`), à ajouter à ce tableau quand ils serviront.
