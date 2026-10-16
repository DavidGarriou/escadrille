---
name: capture
description: Rend une ou plusieurs scènes de référence en mode capture, regarde les images, les compare aux captures précédentes et aux critères de direction artistique, puis rédige une critique. À utiliser avant et après toute modification visuelle (rendu, shaders, matériaux, terrain, nuages, éclairage), ou quand le joueur tape /capture.
argument-hint: "[scène | all]"
allowed-tools: Bash(just capture:*), Bash(cargo run -p escadrille:*), Read, Glob
---

# Capture visuelle : $ARGUMENTS

Scènes de référence : skill `rendu-visuel`. Sans argument : les scènes touchées par la modification en cours.

## Déroulé

1. `just capture <scène>` (mode capture, skill `bevy` §7). La recette conserve la capture précédente sous `target/captures/<scène>.avant.png` avant d'écrire la nouvelle. En cas d'échec : lire l'erreur, ne pas présenter de capture partielle.
2. **Ouvrir et regarder chaque image** (outil Read sur le PNG), et l'image « avant » si elle existe.
3. Lire le `.json` associé : temps par passe, comparaison avec la capture précédente.

## Grille de critique

- **Lumière** : exposition, cohérence du soleil (direction, couleur selon l'heure), ombres (acné, peter-panning, cascades visibles).
- **Atmosphère** : perspective aérienne présente, horizon crédible, pas de banding dans le ciel.
- **Nuages** : forme, éclairage (liseré à contre-jour, bases sombres), bords, traînées de reprojection.
- **Terrain** : répétition visible, transitions de LOD, cohérence avec 1917, échelle lisible.
- **Avion** : matières (toile, bois, métal), translucidité de la toile, câbles sans scintillement, hélice.
- **Composition** : l'image donne-t-elle envie de jouer ? Qu'est-ce qui attire l'œil en premier ?
- **Artefacts** : crénelage, scintillement, z-fighting, textures manquantes, NaN (pixels noirs ou blancs).
- **Coût** : écart de temps GPU, budget de la passe.

## Réponse

Pour chaque scène : chemins des images, trois points forts, problèmes classés (bloquant, important, détail) avec la cause probable et une piste de correction, comparaison avant/après. Pour un changement important, proposer une revue par le subagent `directeur-artistique` en lui passant les chemins des images.
