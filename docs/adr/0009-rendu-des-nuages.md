# 0009 — Rendu des nuages : marche de rayons maison, éclairée par l'atmosphère de Bevy

- Statut : Accepté (validé par le joueur le 2026-11-07)
- Date : 2026-11-07

## Contexte

Le M3 commence par les nuages (`docs/plans/m3-beaute.md`, incrément 1). Le joueur a validé le 2026-11-04 la décision 2 du plan : une marche de rayons maison en WGSL, d'après Schneider et Hillaire, plutôt que les volumes de brouillard de Bevy. Ces volumes sont des boîtes, conçues pour un brouillard local et éclairées seulement par les lumières qu'on y marque ; ils ne peuvent pas porter un ciel entier. Reste à décider **comment** cette marche s'insère dans Bevy 0.20 et d'où vient sa lumière.

Ce qu'on a vérifié dans les sources de la version épinglée :
- **Bevy 0.20 n'a pas de nuages.** Son graphe de rendu est devenu un schedule, `Core3d`, dont chaque passe est un système ordonné dans les ensembles `Prepass`, `MainPass`, `EarlyPostProcess` et `PostProcess`.
- **Le ciel et la perspective aérienne** sont rendus par le système `render_sky` de `bevy_pbr`, placé après les opaques et avant les transparents. Ce système lit quatre tables précalculées (transmittance, multidiffusion, vue du ciel, perspective aérienne), qu'il recalcule à chaque image.
  - Tout ce qu'il lie est public : textures, échantillonneurs, tampons d'uniformes et leurs décalages. Seul son groupe de liaisons (`AtmosphereBindGroups`) est réservé à la crate.
  - Ses fonctions WESL (`bevy_pbr::atmosphere::functions`) s'importent, pourvu que le groupe 0 ait la même disposition.
- **La caméra du jeu** : HDR, TAA, sans MSAA, exposition physique.

Contraintes :
- **traverser les nuages** sans saut ni bande ;
- **parité CPU/GPU** de la densité (`CLAUDE.md`). C'est fait : `escadrille_weather` et `assets/shaders/clouds/density.wesl`, avec un test sur le GPU ;
- **couleurs cohérentes** avec le ciel de Bevy, du midi au couchant (skill `rendu-visuel`) ;
- **budget** : 3 ms en 1440p, qualité « haut », sur la RX 7800 XT ; le jeu coûte aujourd'hui 1,1 à 3,3 ms.

## Options

1. **Lumière calculée à part.** On recalcule en Rust, à chaque image, la couleur du soleil et celle du ciel à l'altitude des nuages, et on les passe en uniformes.
   - Avantages : aucune dépendance aux détails internes de Bevy.
   - Inconvénients : une seconde atmosphère, qui divergera de la première, surtout au lever et au coucher. Pas de perspective aérienne par pixel sur les nuages lointains, alors qu'elle donne l'échelle.
2. **Lumière tirée des tables de l'atmosphère de Bevy.** La passe recrée à l'identique le groupe de liaisons de `render_sky` (groupe 0, liaisons 0 à 13) et importe ses fonctions. Nos textures et nos uniformes vont au groupe 1.
   - Avantages : le même soleil, le même ciel et la même perspective aérienne que le reste de l'image, à toute heure, sans code d'atmosphère à nous.
   - Inconvénients : la passe dépend d'une disposition de liaisons que Bevy ne garantit pas d'une version à l'autre.
3. **Implémenter Hillaire 2020 nous-mêmes** pour le ciel comme pour les nuages, et retirer l'atmosphère de Bevy.
   - Avantages : la maîtrise complète.
   - Inconvénients : refaire ce que Bevy fait déjà bien, pour un gain qu'on ne voit pas aujourd'hui.

Pour l'**intégration**, les nuages doivent se poser sur le ciel et les objets opaques, déjà voilés par la perspective aérienne, et sous les objets transparents (haubans) : après `render_sky`, avant `main_transparent_pass_3d`. Mais `render_sky` est un système privé : aucun `.after()` ne peut le nommer. Quatre voies, essayées dans les sources :
- se placer entre les deux passes sans ordre explicite avec `render_sky` : l'ordre est alors indéterminé. Si les nuages passent avant le ciel, celui-ci les recouvre ;
- une passe de construction du schedule (`ScheduleBuildPass`) qui trouve `render_sky` par son nom et ajoute l'arête. Son trait impose de nommer un type de la crate `indexmap`, que Bevy ne réexporte pas : ce serait une dépendance de plus ;
- retrouver l'ensemble implicite (`SystemTypeSet`) de `render_sky` dans le graphe du schedule : on le trouve, mais on ne peut pas reconstruire sa valeur internée, qui se compare par adresse ;
- **deux temps** : la marche écrit la lumière et la transmittance des nuages dans **sa propre image**, entre les deux passes, sans toucher à l'image principale ; l'ordre avec `render_sky` n'a donc plus d'importance. Un **matériau transparent plein écran** les compose ensuite, premier dessiné de la passe des transparents grâce à un biais de tri, donc après le ciel et sous les haubans. Tout passe par des API publiques.

## Décision

**Option 2**, en deux temps : la marche des rayons, éclairée par les tables de l'atmosphère de Bevy, écrit dans sa propre image ; un matériau transparent plein écran la compose (couleur = lumière des nuages + image × transmittance), premier de la passe des transparents. Elle se construit en deux étapes :
1. **Pleine résolution** : l'image des nuages a la taille de l'image principale. Cette étape sert à régler l'aspect et à mesurer le coût.
2. **Quart de résolution** (moitié sur chaque axe) : reconstruction temporelle sur une image d'historique, bruit bleu pour décaler les échantillons, remise à l'échelle qui tient compte de la profondeur. Le but est de tenir le budget. Seule l'image des nuages change de taille ; la composition, elle, reste la même.

L'éclairage :
- extinction de Beer-Lambert, terme « powder » ;
- phase de Henyey-Greenstein à deux lobes, multidiffusion par octaves (Wrenninge) ;
- marche courte vers le soleil, sur la densité grossière ;
- soleil atténué par la table de transmittance, ciel ambiant tiré de la table de vue du ciel ;
- perspective aérienne appliquée à la lumière des nuages selon leur distance moyenne pondérée par la transmittance.

Les **ombres des nuages** (au sol, sur l'avion) viendront ensuite. Elles utiliseront une carte d'ombre vue du dessus, calculée à partir de la même densité.

## Conséquences

- **Montées de version de Bevy** : la disposition du groupe 0 et les fonctions importées sont à revérifier à chaque montée. Si elles changent, le shader ne compile plus, et `just capture` sort en erreur : la régression ne passe pas inaperçue. L'ADR de montée de version le mentionnera. Le biais de tri du matériau de composition est aussi à revérifier.
- **Objets transparents** : ils se dessinent après les nuages. Un hauban ou une traçante situés derrière un nuage restent donc visibles à travers lui. C'est négligeable pour les haubans ; pour les traçantes du M4, à revoir.
- **Coût mémoire** : une image Rgba16Float de la taille de l'écran, soit 29 Mo en 1440p ; le quart de résolution la divisera par quatre.
- **TAA** : les nuages n'ont pas de vecteurs de mouvement. La TAA les reprojette comme s'ils étaient à l'infini. Il faudra surveiller les traînées en vol rapide près d'un nuage ; la reconstruction temporelle de l'étape 2 utilisera sa propre reprojection, à la distance des nuages.
- **Coût** : mesuré dans le `.json` de chaque capture (`gpu_passes_ms`). Les réglages de qualité règleront le nombre de pas et la résolution.
- **Réglages** : la couche de nuages devient une ressource du jeu, que les scènes et, plus tard, les missions décrivent en données.
