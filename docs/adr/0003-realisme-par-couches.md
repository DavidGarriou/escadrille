# 0003 — Réalisme par couches sur un modèle physique unique

- Statut : Accepté
- Date : 2026-10-16

## Contexte
Le jeu doit aller du très fun (*Wings*) au très réaliste (*IL-2*). Deux modèles de vol (arcade et simulation) doubleraient la maintenance et donneraient deux jeux différents.

## Options
1. Deux modèles de vol distincts.
2. **Un modèle unique ; le réalisme se règle par des couches indépendantes** : assistances entre les commandes et le FDM, facteurs d'échelle documentés sur certains effets, règles de jeu, présentation (HUD, vues).

## Décision
Option 2, avec une ressource `RealismSettings` de réglages indépendants et des préréglages (tableau de `docs/vision.md`). Les assistances agissent par les commandes, comme le ferait un pilote. Les réglages non physiques (structure indestructible, munitions illimitées…) sont explicites et signalés dans l'interface.

## Conséquences
- Même sensation d'avion à tous les niveaux : le joueur progresse sans changer de jeu.
- L'instructeur virtuel et l'IA partagent les mêmes contrôleurs de manœuvre.
- Chaque réglage doit être testé seul et dans chaque préréglage (tests de fumée).
- Interdiction des `if arcade` disséminés : chaque réglage est lu par un seul système.
