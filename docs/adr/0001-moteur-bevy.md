# 0001 — Moteur : Bevy

- Statut : Accepté
- Date : 2026-10-16

## Contexte
Le jeu doit être écrit en Rust, n'utiliser que des logiciels open source et être visuellement de très haut niveau (ciel, nuages, terrain à grande échelle).

## Options
1. **Bevy** : ECS, rendu PBR et HDR moderne sur wgpu, communauté très active, licences MIT/Apache-2.0. Pas d'éditeur complet ; API qui change à chaque version mineure.
2. **Fyrox** : moteur Rust plus « classique », avec éditeur. Communauté et écosystème plus petits.
3. **wgpu directement** (moteur maison) : contrôle total, coût de développement très élevé.

## Décision
Bevy, version exacte épinglée (choisie par un ADR dédié au M0).

## Conséquences
- Discipline de version (voir `CLAUDE.md` et skill `bevy`) : ne jamais se fier à la mémoire pour l'API.
- Passes de rendu sur mesure pour les nuages volumétriques et le terrain : il faut maîtriser le graphe de rendu de Bevy et WGSL.
- Logique critique (FDM, balistique, données) hors de Bevy pour limiter le coût des montées de version.
- Contenu édité dans Blender et des fichiers RON plutôt que dans un éditeur de moteur.
