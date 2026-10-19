# Plan du M0 — Fondations

- Statut : **Validé** par le joueur le 2026-10-19 (plan et dépendances) ; étapes 1 et 2 faites (2026-10-19) ; étape 3 à faire sur la machine du joueur
- Date : 2026-10-19
- Références : `docs/roadmap.md` (M0), ADR 0004 (versions), skills `bevy` et `manette`

Le M0 est découpé en trois étapes. Les deux premières se font dans le cloud (compilation et tests sans écran) ; la troisième demande la machine du joueur (GPU, manette).

## Étape 1 — Outillage et workspace (cloud)

| Fichier | Contenu |
|---|---|
| `rust-toolchain.toml` | canal `1.99.1`, composants `rustfmt` et `clippy` |
| `Cargo.toml` | workspace : `crates/fdm`, `crates/data`, `crates/game`, `tools/fdm_lab` ; `edition = "2024"`, `rust-version = "1.97.1"`, `license = "MIT"` ; dépendances communes dans `[workspace.dependencies]` avec versions exactes ; profils : `dev` optimise les dépendances (`opt-level = 3`) et peu notre code (`opt-level = 1`), `release` avec LTO léger |
| `.cargo/config.toml` | édition de liens avec `clang` + `mold` sous Linux |
| `deny.toml` | licences autorisées = liste de `CLAUDE.md` ; refus des sources Git non déclarées ; alerte sur les doublons de versions |
| `rustfmt.toml` | réglages minimaux (édition 2024) |
| `justfile` | `check`, `test`, `run`, `release`, `labo-manette`, `capture <scène>` ; `essai` en attente du M1 |
| `assets/LICENCES.md` | registre vide des fichiers tiers, avec le format d'une entrée |
| `.claude/hooks/session-start.sh` | installe la toolchain de `rust-toolchain.toml` (le conteneur n'a que 1.97.0) |

Crates créées à l'état de squelette, chacune avec au moins un test :
- `escadrille_fdm` : sans aucune dépendance à Bevy ; constantes et unités SI. Le vrai modèle de vol est le M1.
- `escadrille_data` : chargement d'un fichier `.ron` d'exemple, avec erreur claire en cas de fichier invalide.
- `fdm_lab` : interface en ligne de commande qui affiche sa version et la liste des sous-commandes prévues.

Vérification : `just check` et `just test` verts dans le cloud.

## Étape 2 — Le jeu, compilé dans le cloud

Crate `escadrille_game` (binaire Bevy 0.20.0) :
- **Fenêtre et états de jeu** : `Démarrage`, `LaboManette`, `Capture`, `Vol` (vide au M0). Un plugin par domaine, comme dans `CLAUDE.md`.
- **Plugin `dev`** :
  - **Labo manette** (écran egui) :
    - manettes détectées (nom, identifiant) ;
    - pour chaque axe, la valeur brute et la valeur après zone morte ;
    - l'état de chaque bouton ;
    - un **histogramme des valeurs prises par L2 et R2**, pour voir si les gâchettes sont progressives ou tout-ou-rien ;
    - des boutons de **test des vibrations** (moteur faible, moteur fort, durée).
  - **Mode capture** : `just capture <scène>` lance le jeu, charge la scène, laisse passer quelques images pour stabiliser, enregistre `target/captures/<scène>.png` et un `.json` (temps par image, version, GPU), puis quitte.
- **Scène de référence `m0-sol`** : un sol, deux repères de taille (un cube de 1 m, un poteau de 10 m), un soleil, un ciel uni. Elle sert à vérifier la chaîne de capture, pas la beauté. (Première version du plan : « un avion factice à l'échelle d'un SPAD VII », abandonnée tant que les dimensions ne sont pas sourcées.)
- Toute l'API Bevy utilisée est vérifiée dans les sources 0.20.0 et ses exemples (skill `bevy`).

Vérification : compilation et tests verts dans le cloud ; tests unitaires de la logique sans fenêtre (zones mortes, histogramme, nommage des captures).

**Expérience proposée** : tenter le mode capture dans le cloud avec un rendu logiciel (Mesa lavapipe et un écran virtuel). Si cela marche, Claude pourra produire et regarder des captures sans votre machine, pour les scènes simples. Si cela ne marche pas, on abandonne sans regret : les captures se feront chez vous.

## Étape 3 — Sur votre machine (Fedora, RX 7800 XT, SN30 Pro)

Mode d'emploi :
```bash
git fetch origin && git checkout main-tcd0yw && git pull   # ou main, une fois fusionnée
just labo-manette        # première compilation : ~20 min ; rustup installe Rust 1.99.1 seul
just capture m0-sol      # écrit target/captures/m0-sol.png et .json
```
Dans le labo : tirer lentement L2 et R2 plusieurs fois, essayer chaque bouton de vibration, puis « Save report ». Envoyer à Claude `target/labo-manette/rapport.txt`, ce que vous avez ressenti pour chaque vibration, et `target/captures/m0-sol.png` et `.json`.


1. `just labo-manette` avec la manette en USB-C, puis en Bluetooth : relever le comportement des gâchettes L2 et R2 (progressives ou tout-ou-rien) et des vibrations.
2. `just capture m0-sol` : Claude lit l'image et le `.json`, et les commente.
3. Claude consigne les résultats : question ouverte 5 tranchée dans `docs/vision.md`, skill `manette` complétée, retours dans `docs/gdd/retours.md`.

## Dépendances à valider (règle « demander avant d'ajouter une dépendance »)

| Crate | Pourquoi | Licence | Couplée à Bevy |
|---|---|---|---|
| `bevy` =0.20.0 | moteur | MIT OR Apache-2.0 | — |
| `bevy_egui` =0.43.1 | labo manette, futurs écrans de debug | MIT | oui |
| `glam` (version utilisée par Bevy 0.20) | maths f64 du FDM, sans Bevy | MIT OR Apache-2.0 | non |
| `serde`, `ron` | fichiers de données | MIT OR Apache-2.0 | non |
| `clap` | arguments de `fdm_lab` et du mode capture | MIT OR Apache-2.0 | non |
| `thiserror` | erreurs typées de `escadrille_data` | MIT OR Apache-2.0 | non |

Versions exactes et licences vérifiées sur crates.io au moment de l'étape 1 ; `cargo deny` contrôle tout l'arbre.

Volontairement absents du M0 : `bevy_hanabi` (ADR 0004), `kira`, `fundsp`, `parry3d-f64`, `plotters`, `fluent` (ajoutés au jalon qui en a besoin, chacun avec validation).

## Ce qui reste hors de portée du cloud

Lancer le jeu dans une fenêtre, tester la manette, mesurer les performances sur le GPU de référence. Le cloud compile et teste ; votre machine joue.
