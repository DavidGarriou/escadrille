# 0005 — Intégration numérique du modèle de vol : RK4 à pas fixe, en f64

- Statut : Accepté (validé par le joueur le 2026-10-20, avec le plan du M1)
- Date : 2026-10-20

## Contexte

`escadrille_fdm` intègre les équations du mouvement d'un corps rigide à 6 degrés de liberté (plus, plus tard, le rotor moteur et le train). Contraintes :
- **déterminisme** : à entrées égales, résultats identiques au bit près (replays, tests de régression, IA) ;
- **stabilité** sur des dynamiques raides : oscillation d'incidence rapide d'un petit chasseur peu chargé, amortissement en roulis, contact au sol (M2) ;
- **précision** suffisante pour que les essais en vol mesurent l'avion et non l'intégrateur (critère du skill `simulation-vol` : diviser le pas par deux change les résultats de moins de 1 %) ;
- **coût** compatible avec le temps réel pour plusieurs avions et l'IA, dans `FixedUpdate` à 120 Hz (`CLAUDE.md`).

## Options

1. **Euler semi-implicite (symplectique)** : une évaluation des forces par pas, très bon marché, conserve bien l'énergie d'un oscillateur. Mais il est d'ordre 1 : erreurs de phase et d'amplitude notables sur les modes rapides, ce qui exigerait de petits pas pour passer le critère de 1 %.
2. **Runge-Kutta classique d'ordre 4 (RK4), pas fixe** : quatre évaluations des forces par pas, erreur d'ordre 4, comportement connu et très documenté (Stevens et Lewis l'utilisent pour la simulation d'avions). Déterministe par construction.
3. **RK45 à pas adaptatif (Dormand-Prince)** : précision contrôlée automatiquement, mais le nombre d'évaluations varie d'un pas à l'autre : coût imprévisible en temps réel, et le découpage du temps dépend de l'état, ce qui complique le déterminisme entre configurations et la synchronisation avec `FixedUpdate`.

## Décision

**Option 2 : RK4 à pas fixe, en f64.**
- Le jeu appelle le FDM à 120 Hz (`FixedUpdate`) ; le FDM fait **2 sous-pas par appel, soit 240 Hz** (valeur de départ de `modele-aero.md`), réglable par avion si les essais l'exigent.
- État intégré : position (NED), quaternion d'attitude (monde ← corps), vitesse linéaire et vitesse angulaire **dans le repère corps** ; plus tard, régime du rotor et états moteur.
- Quaternion **renormalisé à la fin de chaque pas** (pas dans les étapes intermédiaires de RK4).
- Commandes et environnement **constants pendant un pas** (bloqueur d'ordre zéro), pour que les quatre évaluations voient les mêmes entrées.
- Aucune source d'aléa ni d'horloge dans l'intégrateur ; ordre des sommes de forces fixe (vecteurs, pas de `HashMap`).

Raison principale : la meilleure précision pour un coût fixe et prévisible, avec un déterminisme garanti.

## Conséquences

- Les tests de `escadrille_fdm` vérifient : chute libre (solution exacte), conservation de l'énergie et du moment cinétique sans aérodynamique, rotation libre autour des axes principaux, précession, déterminisme au bit près, et convergence en pas (`fdm_lab --dt-check`).
- Le contact au sol (M2) pourra demander des sous-pas supplémentaires ou un traitement dédié : à décider avec mesures, par un nouvel ADR si nécessaire.
- Si le profilage montre que le FDM coûte trop cher avec de nombreux avions, piste : pas plus long pour les avions lointains de l'IA, sans changer d'intégrateur pour l'avion du joueur.
