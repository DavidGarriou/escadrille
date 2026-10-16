# Escadrille — kit de démarrage pour Claude Code

Ce dossier ne contient pas encore de code. C'est le « prompt » du projet : il prépare Claude Code à concevoir et à développer avec vous un simulateur de chasse 1914-1918 en Rust, sur le moteur open source Bevy.

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
chmod +x .claude/hooks/format-rust.sh
```

Outils de contenu (paquets Fedora ou Flathub) : Blender, Krita, GIMP, Inkscape, QGIS, Audacity, Ardour. Profilage : Tracy, RenderDoc.

## Démarrer

1. Placer ce dossier là où vivra le projet et lancer `git init`.
2. Relire `docs/vision.md`, surtout les **questions ouvertes** en fin de fichier, et corriger ce qui ne vous convient pas.
3. Ouvrir Claude Code dans le dossier, passer en **mode plan**, et envoyer :

> Lis CLAUDE.md, docs/vision.md et docs/roadmap.md. Nous commençons le jalon M0.
> Avant tout code :
> 1. Vérifie sur crates.io la dernière version de Bevy et sa compatibilité avec bevy_egui et bevy_hanabi, puis propose la version à épingler dans un ADR.
> 2. Vérifie les licences et l'état de maintenance des autres crates de la stack.
> 3. Propose le plan détaillé du M0 (fichiers, crates, recettes du justfile, labo manette, mode capture) et attends ma validation.

4. À la fin du M0, branchez la SN30 Pro et lancez `just labo-manette` : Claude complète la skill `manette` avec ce que le labo montre (gâchettes progressives ou non, vibrations).

## Adapter le kit

- **Préréglages de réalisme** : le tableau de `docs/vision.md` fait foi.
- **Premier avion et secteur** : proposition SPAD VII contre Albatros D.V, Flandres 1917 ; à changer dans `docs/vision.md`.
- **Ajouter une skill** : `.claude/skills/<nom>/SKILL.md`, avec en en-tête un `name` et une `description` qui dit *quand* la charger.
- **Une règle importante revient souvent dans vos corrections ?** Demandez à Claude de l'ajouter à `CLAUDE.md`.
