# Escadrille — kit de démarrage pour Claude Code

*Panache blanc* : un simulateur de chasse 1914-1918 en Rust, sur le moteur open source Bevy, développé avec Claude Code. Le dépôt contient le kit de conception (règles, skills, documents) et le code du jalon M0 (workspace Cargo, labo manette, mode capture).

## Contenu

```
CLAUDE.md                         règles du projet, lues à chaque session (court, par choix)
README.md                         ce fichier (pour vous)
docs/
  vision.md                       design : piliers, références, réalisme, avions, théâtre, DA, questions ouvertes
  roadmap.md                      jalons M0 à M7 et leurs définitions de « terminé »
  adr/                            décisions d'architecture (Bevy, FDM indépendant, réalisme par couches)
  gdd/                            documents de design par fonctionnalité (mission-dirigeables.md)
.claude/
  settings.json                   permissions (cargo, just, git en lecture) et hook de formatage
  hooks/format-rust.sh            rustfmt sur chaque fichier .rs modifié par Claude
  hooks/session-start.sh          sessions cloud (Ubuntu) uniquement : installe libs système, just, mold, nextest, deny
  hooks/logbook-*.sh              vérifie que docs/carnet-de-bord.md est tenu, commité et poussé à chaque échange
  skills/                         connaissances chargées à la demande
    simulation-vol/               modèle de vol (+ modele-aero.md, format-avion.md)
    bevy/                         discipline de version, ECS, pas fixe, origine flottante, mode capture
    rendu-visuel/                 ciel, terrain, matériaux, VFX (+ nuages-volumetriques.md)
    manette/                      SN30 Pro, actions, courbes, vibrations, labo
    realisme/                     préréglages, assistances, instructeur virtuel
    combat/                       armes, balistique, dégâts, IA
    recherche-historique/         sources, confiance, droits (+ fiche-avion.md)
    pipeline-assets/              Blender → glTF, textures, terrain, licences
    audio/                        moteurs procéduraux, câbles, armes, ambiance
    game-design/                  missions, carrière, sensations, essais du joueur
    nouvel-avion/                 /nouvel-avion  (workflow lancé par vous)
    essai-en-vol/                 /essai-en-vol  (aussi lancé par Claude après une modif du FDM)
    capture/                      /capture       (aussi lancé par Claude après une modif visuelle)
    adr/                          /adr
  agents/                         subagents relecteurs
    aerodynamicien.md
    historien.md
    directeur-artistique.md
```

Principe : `CLAUDE.md` reste court (il est chargé à chaque fois) ; le détail vit dans les skills, que Claude charge selon la tâche grâce à leur description.

## Prérequis (Fedora)

```bash
sudo dnf install gcc-c++ clang mold jq just git git-lfs \
  libX11-devel libxkbcommon-devel wayland-devel alsa-lib-devel systemd-devel \
  vulkan-tools mesa-vulkan-drivers gdal
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked cargo-nextest cargo-deny
chmod +x .claude/hooks/*.sh
```

Outils de contenu (paquets Fedora ou Flathub) : Blender, Krita, GIMP, Inkscape, QGIS, Audacity, Ardour. Profilage : Tracy, RenderDoc.

## Premier essai sur votre machine (M0, étape 3)

1. **Prérequis** : la section ci-dessus, une seule fois (paquets Fedora, `rustup`). Rust 1.99.1 s'installe ensuite tout seul grâce à `rust-toolchain.toml`. `cargo-nextest` et `cargo-deny` ne servent qu'à `just test` et `just check`.
2. **Récupérer le code** (la première fois : `git clone git@github.com:DavidGarriou/escadrille.git && cd escadrille`) :
   ```bash
   git fetch origin && git checkout main-tcd0yw && git pull   # ou main, une fois la branche fusionnée
   ```
3. **Brancher la SN30 Pro USB**, puis :
   ```bash
   just labo-manette     # la première compilation de Bevy prend de longues minutes
   ```
   - **Gâchettes** : enfoncer lentement L2 et R2 plusieurs fois (le labo dit si elles sont progressives ou tout-ou-rien).
   - **Sticks** : les pousser lentement jusqu'en butée dans chaque direction, faire quelques grands cercles, puis les lâcher et regarder la valeur au repos (elle devrait revenir près de 0).
   - **Vibrations** : essayer chaque bouton, noter ce que vous sentez.
   - Puis « Save report » (écrit `target/labo-manette/rapport.txt`). Échap pour quitter.
4. **Capture de référence** :
   ```bash
   just capture m0-sol   # écrit target/captures/m0-sol.png et .json, puis quitte
   ```
5. **Donner à Claude** : `target/labo-manette/rapport.txt`, vos impressions sur les vibrations, `target/captures/m0-sol.png` et `.json` (les coller dans la conversation ou les copier dans le dépôt).

## Adapter le kit

- **Préréglages de réalisme** : le tableau de `docs/vision.md` fait foi.
- **Premier avion et secteur** : proposition SPAD VII contre Albatros D.V, Flandres 1917 ; à changer dans `docs/vision.md`.
- **Ajouter une skill** : `.claude/skills/<nom>/SKILL.md`, avec en en-tête un `name` et une `description` qui dit *quand* la charger.
- **Une règle importante revient souvent dans vos corrections ?** Demandez à Claude de l'ajouter à `CLAUDE.md`.
