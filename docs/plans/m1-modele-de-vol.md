# Plan du M1 — Le modèle de vol, sans rendu

- Statut : **Validé** par le joueur le 2026-10-20 (plan, dépendances `plotters` et `nalgebra` dans `fdm_lab`, ADR 0005, lancement de l'historien)
- Date : 2026-10-20
- Références : `docs/roadmap.md` (M1), ADR 0002 (FDM indépendant, f64), skills `simulation-vol`, `recherche-historique`, `essai-en-vol` ; `.claude/skills/simulation-vol/modele-aero.md` et `format-avion.md`

**But** : le SPAD VII vole dans `escadrille_fdm`, sans image, et ses performances calculées par `fdm_lab` retombent sur les chiffres historiques sourcés.

**Définition de « terminé »** (feuille de route) : essais quantitatifs de confiance A et B dans leurs tolérances ; tests de robustesse verts ; revue `aerodynamicien` sans point bloquant. S'y ajoute l'essai du joueur : lire le rapport d'essais et les tracés, et dire si le SPAD « ressemble » à ce que racontent les pilotes. On pilotera au M2.

Tout le M1 se fait dans le cloud : pas de GPU ni de manette nécessaires.

## Étape 1 — Recherche : le SPAD VII et son moteur (subagent `historien`)

Fiches sourcées, selon `fiche-avion.md` :
- `docs/recherche/spad7.md` : sous-version à retenir pour la SPA 3 à l'été 1917 (moteur de 150 ch ou de 180 ch : à établir, pas à supposer), géométrie complète, masses et centrage, performances (vitesse maximale par altitude, temps de montée, plafond, décrochage), qualités de vol, **témoignages de pilotes en plusieurs langues et réputation**, traits traduits en essais qualitatifs.
- `docs/recherche/hispano-suiza-8a.md` : puissance en fonction du régime, comportement en altitude, hélice (diamètre, pas, sens de rotation).
- Profil d'aile : données de soufflerie publiées (NACA ou rapports britanniques du domaine public), ou profil comparable justifié (confiance C).

Sortie : fourchettes, niveaux de confiance A à D, journal des décisions. **Aucun chiffre n'entre dans un `.ron` avant d'être dans la fiche.**

Cette étape avance **en parallèle** des étapes 2 à 4, qui se testent sans données historiques.

## Étape 2 — Atmosphère et corps rigide (`escadrille_fdm`)

- Atmosphère ISA de 0 à 11 km (température, pression, densité, vitesse du son), vent constant. Trait `Environment` (sol, vent, densité), implémenté plus tard par le jeu et dès maintenant par les outils.
- Corps rigide à 6 degrés de liberté : position dans le repère NED, quaternion d'attitude, vitesses dans le repère corps, tenseur d'inertie avec `ixz`. Intégrateur **RK4 en f64**, à pas fixe, avec quaternion renormalisé.
- Tests : chute libre, conservation de l'énergie et du moment cinétique sans aérodynamique, précession d'une toupie, rotation libre stable autour des axes principaux, déterminisme (deux exécutions identiques au bit près).

## Étape 3 — Aérodynamique

- Surfaces découpées en bandes : vitesse locale avec ω × r, incidence et dérapage locaux, polaires sur 360° (données du profil jusqu'au décrochage, puis plaque plane au-delà, raccord continu).
- Biplan : écartement, décalage horizontal et angulaire, interférence entre les plans. Empennage avec déflexion de l'aile et gouvernes (profondeur, direction, ailerons).
- Traînées parasites, élément par élément, avec leur position : mâts, haubans, câbles, radiateur, train, pilote.
- Tests : symétrie (pas de roulis spontané en vol symétrique), signes des moments (une incidence positive cabre, l'empennage stabilise), continuité des polaires, comparaison à des cas analytiques (aile elliptique, traînée induite).

## Étape 4 — Hélice et moteur en ligne

- Hélice : tables C_T(J) et C_P(J), couple de réaction sur la cellule, effet P simple. Le souffle de l'hélice sur l'empennage est reporté si le modèle se tient sans lui (choix noté dans la fiche).
- Moteur à pistons : puissance en fonction du régime et de la manette, perte en altitude (relation de Gagg-Farrar), équilibre du régime entre le couple moteur et le couple hélice.

## Étape 5 — Données : `crates/data`

- Schémas `aircraft.ron` et `essais.ron` (`format-avion.md`), avec unités d'époque typées (`Kmh`, `Mph`, `Ft`, `Hp`, `Ps`…) converties en SI au chargement.
- Validation : chaque valeur doit avoir une origine (`S:`, `C:`, `E:`), sinon refus ; contrôles de cohérence (masses positives, centre de gravité dans la cellule, surfaces calculées comparées à la fiche).
- `assets/aircraft/spad7/aircraft.ron` et `essais.ron`, remplis **depuis la fiche** de l'étape 1.

## Étape 6 — `fdm_lab` : trim, essais, modes, tracés

- Trim : solveur de Newton (incidence, profondeur, gaz, et symétrie latérale) pour une vitesse et une altitude données.
- Essais : vitesse maximale par altitude, temps de montée par paliers, plafond, vitesse de décrochage, puis les scénarios qualitatifs issus des témoignages.
- Modes propres par linéarisation autour du trim : oscillation d'incidence, phugoïde, roulis hollandais, spirale, roulis.
- Sorties : `target/essais/spad7/rapport.json` et tracés PNG (polaire des vitesses, montée, trim, modes), lus par Claude ; option `--dt-check` (convergence en pas de temps < 1 %).
- `just essai spad7`.

## Étape 7 — Validation

- Tests de robustesse : vitesse nulle, glissade sur la queue, vrille, vol sur le dos, piqué à Vmax : jamais de NaN ni de divergence.
- `/essai-en-vol spad7`, puis rapport dans `docs/essais/spad7.md`.
- Revue du subagent `aerodynamicien`, puis corrections **justifiées** (jamais un coefficient ajusté pour faire passer un essai).
- Essai du joueur : lecture du rapport et des tracés ; retours dans `docs/gdd/retours.md`.

## Hors M1 (reporté)

Train d'atterrissage et sol (M2, nécessaires au décollage), structure et dégâts (M4), moteurs rotatifs (M6), turbulence et sillage, dirigeables.

## Décisions à valider

1. **Dépendances**, toutes indépendantes de Bevy :
   - `glam` (déjà au workspace) dans `escadrille_fdm` ;
   - `plotters` (MIT) pour les tracés PNG et `nalgebra` (Apache-2.0) pour les valeurs propres, **dans `fdm_lab` seulement**, conformément à `CLAUDE.md`.

   Versions et licences vérifiées sur crates.io au moment de l'ajout.
2. **ADR 0005 — Intégration numérique** : RK4 en f64, pas de 1/120 s avec sous-pas si nécessaire, quaternion renormalisé. Les autres options (Euler semi-implicite, RK45 adaptatif) y seront comparées.
3. **Lancement immédiat du subagent `historien`** sur le SPAD VII et l'Hispano-Suiza 8A. C'est une longue recherche web en plusieurs langues ; elle tourne en arrière-plan pendant que le code avance.
