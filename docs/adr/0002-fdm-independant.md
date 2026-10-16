# 0002 — Modèle de vol dans une crate indépendante, en f64

- Statut : Accepté
- Date : 2026-10-16

## Contexte
Le modèle de vol (FDM) est utilisé par le joueur, l'IA, les assistances, les outils d'essai et les tests. Il doit être validé contre des données historiques, rejouable et robuste.

## Options
1. FDM écrit comme systèmes Bevy, sur un moteur physique (avian, rapier).
2. **FDM dans une crate Rust pure (`escadrille_fdm`), sans moteur, en f64, avec son propre intégrateur.**

## Décision
Option 2. Le jeu appelle le FDM depuis `FixedUpdate` et convertit son état pour le rendu dans un seul module (`bridge.rs`). Les collisions et requêtes géométriques passent par `parry3d-f64` ou du code maison, pas par un moteur physique généraliste.

## Conséquences
- Essais en vol rapides en ligne de commande (`fdm_lab`), sans GPU, utilisables par Claude.
- Indépendance vis-à-vis des montées de version de Bevy.
- Déterminisme maîtrisé (replays, tests de régression).
- Il faut écrire nous-mêmes l'intégration, le contact au sol et les collisions simples.
