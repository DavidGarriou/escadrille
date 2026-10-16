---
name: rendu-visuel
description: Rendu visuel d'Escadrille — ciel et atmosphère, nuages volumétriques, terrain à grande échelle et paysage du front 1917, matériaux des avions (toile translucide, bois, métal), hélice, câbles, effets (traçantes, fumées, DCA), post-traitement, shaders WGSL et budget de performances GPU. À charger pour toute tâche qui touche l'apparence du jeu ou crates/game/src/render, src/world ou assets/shaders.
---

# Rendu visuel

Objectif : **« réalisme lumineux »**, photoréaliste par la lumière et les matières (direction artistique dans `docs/vision.md`). La beauté se **vérifie sur image** : aucune modification visuelle n'est terminée sans captures avant/après regardées (`/capture`).

Référence détaillée : `nuages-volumetriques.md` (la fonctionnalité visuelle la plus importante et la plus difficile).

## Scènes de référence

Chacune est un fichier `assets/scenes/<nom>.ron` (caméra, heure, météo, graine) utilisé par `/capture`.

| Scène | Ce qu'elle vérifie |
|---|---|
| `aube_terrain` | soleil rasant, brume basse, avions alignés sur l'herbe, ombres longues |
| `cumulus_midi` | champ de cumulus à 2 000 m, soleil haut, ombres des nuages au sol |
| `contre_jour_toile` | avion à contre-jour : toile translucide, nervures visibles |
| `no_mans_land` | rase-mottes au-dessus des tranchées et des cratères inondés |
| `combat_couchant` | combat au soleil couchant : traçantes, fumée, reflets |
| `plafond_bas` | couverture basse, pluie, visibilité réduite |
| `haute_altitude` | 5 000 m : perspective aérienne, horizon, ciel profond |
| `cockpit_detail` | instruments, bois, cuir, huile, profondeur de champ |
| `dirigeable_aube` | coque argentée de 200 m au-dessus d'une mer de nuages : échelle lisible, ombre sur les nuages |
| `dirigeable_feu_nuit` | dirigeable en feu la nuit : faisceaux de projecteurs, structure visible, nuages et sol éclairés par le feu |

## Chaîne de rendu

- **HDR** et soleil en unités physiques (lux) ; exposition automatique ou caméra physique ; tonemapping à choisir sur captures (AgX ou TonyMcMapface, selon la version) ; bloom discret.
- **Anticrénelage** : TAA. Les **câbles et haubans** sont plus fins qu'un pixel : leur appliquer la technique « phone-wire AA » (largeur minimale à l'écran d'environ un pixel, avec opacité réduite en proportion) pour éviter le scintillement.
- **Atmosphère** : utiliser l'atmosphère procédurale de Bevy si la version épinglée la fournit (vérifier) ; sinon implémenter Hillaire 2020 (LUT de transmittance, multidiffusion, vue du ciel, perspective aérienne). La **perspective aérienne** donne l'échelle du paysage : elle n'est pas optionnelle.
- **Ombres** : cascades réglées pour l'avion et le cockpit de près, ombres des nuages au sol (carte d'ombre vue du dessus).

## Terrain et paysage de 1917

- **Données** : `tools/terrain_bake` (GDAL) transforme un MNT open data en tuiles de hauteur 16 bits, normales et masques (skill `pipeline-assets`).
- **Rendu** : quadtree avec LOD continu (CDLOD) et géomorphing pour éviter les sauts ; matériaux par splatting, en triplanar dans les pentes ; variations à grande échelle contre la répétition.
- **Occupation du sol 1917** : patchwork de champs, haies, bois, villages, routes, canaux, numérisés depuis des cartes d'époque. **Aucun élément moderne.**
- **Le front** : densité de cratères fonction de la distance à la ligne de front (champ procédural avec graine), tranchées numérisées (vecteurs → masques, décalques ou déplacement), boue, cratères remplis d'eau qui reflètent le ciel.
- Végétation : arbres instanciés, imposteurs au loin.
- La hauteur du terrain a une implémentation CPU de référence (collisions, IA, essais) identique au GPU.

## Avions

- `StandardMaterial` PBR. Vérifier dans la version épinglée la disponibilité de la transmission diffuse, du vernis (clearcoat) et de l'anisotropie.
- **Toile entoilée** : transmission diffuse modulée par une carte d'épaisseur (nervures, lattes, coutures, rubans) : à contre-jour, la structure apparaît. Toile écrue enduite et toile peinte se distinguent.
- **Losanges** (toile imprimée allemande), cocardes, croix, numéros : couches de décalques.
- **Bois vernis** (hélice, mâts, cockpit) : vernis + rugosité variable. **Capots métalliques** : bouchonnage (*engine-turning*) en anisotropie si disponible.
- **Usure** : coulures d'huile de ricin (rotatifs), fumées d'échappement, éclats de peinture, boue sous les ailes inférieures.
- **Hélice** : pales réelles à bas régime ; au-delà d'un seuil, disque flou dont le motif stroboscopique dépend du régime et de la fréquence d'image.
- **Dégâts** : trous de balles en décalques, toile déchirée qui flotte, pièces arrachées, fumée et feu.

## Dirigeables

Mission : `docs/gdd/mission-dirigeables.md`.
- **Échelle** : un objet de près de 200 m doit se lire comme immense et lointain (perspective aérienne, détails de la coque, ombre portée sur les nuages), et rester crédible de très près (toile tendue sur la structure, nacelles, hélices).
- **Coque** : toile enduite (souvent argentée, ou sombre pour les raids de nuit) avec transmission diffuse, comme les ailes ; la structure apparaît à contre-jour.
- **En feu** : lueur qui grandit à l'intérieur de l'enveloppe, flammes et fumée à grande échelle, la structure qui se découpe puis s'effondre ; la lumière du feu éclaire les nuages, le sol et les avions proches (quelques lumières dynamiques, pas une par particule).
- **Nuit** : lune, étoiles, faisceaux de projecteurs volumétriques qui accrochent la coque et les nuages, instruments éclairés au cockpit.

## Effets (bevy_hanabi)

Traçantes (une balle sur N, traînée lumineuse), flammes de bouche, fumée d'échappement (bleutée pour les rotatifs), fumée noire durable d'un avion abattu, éclatements de DCA (noirs ou blancs selon le camp, dérivant avec le vent), embrasement d'un ballon à hydrogène, débris, explosions et terre au sol. Particules « douces » (soft particles) éclairées par le soleil.

## Post-traitement

Flou de mouvement discret ; profondeur de champ dans le cockpit ; reflets d'objectif sobres ; filtre « actualités d'époque » optionnel (grain, sépia ou noir et blanc, scintillement).

## Shaders WGSL

- Dans `assets/shaders/`, modules nommés `escadrille::<domaine>::<module>` et importés.
- Structures partagées Rust ↔ WGSL dérivées de `ShaderType` ; unités documentées (mètres monde, radiance).
- Rechargement à chaud en dev (feature de surveillance des fichiers de Bevy).
- Tout ce qui agit sur le gameplay a une implémentation Rust de référence et un **test de parité** (échantillons comparés entre CPU et GPU).

## Performances

- Cible : 60 images/s minimum en 1440p sur le GPU de référence (à définir dans `docs/vision.md`). Budgets indicatifs à ajuster : nuages ≤ 3 ms, terrain ≤ 3 ms, ombres ≤ 2 ms, post-traitement ≤ 1,5 ms.
- Préréglages de qualité (`QualitySettings`) : bas, moyen, haut, ultra ; chaque fonctionnalité coûteuse a un réglage.
- Mesurer avec Tracy et RenderDoc ; le `.json` de `/capture` sert de suivi de régression.

## Avant de déclarer fini

- [ ] `/capture` avant/après sur les scènes concernées, **images regardées**
- [ ] Revue `directeur-artistique` pour les changements importants
- [ ] Budget GPU mesuré, réglage de qualité ajouté si nécessaire
- [ ] Test de parité CPU/GPU si la fonctionnalité agit sur le jeu
