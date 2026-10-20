# Jalons

Chaque jalon livre un jeu qui se lance et se termine par un **essai du joueur à la manette**. Claude prépare la liste de ce qu'il faut tester et consigne les retours dans `docs/gdd/retours.md`. Un jalon n'est terminé que si sa définition de « terminé » (DdT) est remplie.

## M0 — Fondations ✅ (clos le 2026-10-20)
- Workspace Cargo (`crates/fdm`, `crates/data`, `crates/game`, `tools/fdm_lab`), versions épinglées, profils de compilation, mold.
- ~~ADR de version de Bevy~~ : fait, ADR 0004 (Bevy 0.20.0, bevy_egui 0.43.1, bevy_hanabi reporté).
- `justfile`, `deny.toml` (liste des licences de `CLAUDE.md`), `rustfmt.toml`, `assets/LICENCES.md`.
- Fenêtre Bevy, états de jeu, plugin `dev`.
- **Labo manette** : axes bruts et traités, boutons, histogramme des valeurs de gâchettes, test des vibrations.
- **Mode capture** opérationnel sur une scène simple.

DdT : `just check` et `just test` verts ; le labo manette affiche toutes les commandes de la SN30 Pro ; questions ouvertes 5 (gâchettes, vibrations) tranchées ; une capture lue et commentée par Claude.

## M1 — Le modèle de vol, sans rendu (en cours ; plan : `docs/plans/m1-modele-de-vol.md`)
- `escadrille_fdm` : ISA, corps rigide, surfaces par bandes, traînées parasites, hélice, moteur en ligne.
- Fiche de recherche et `aircraft.ron` du premier avion (proposition : SPAD VII).
- `fdm_lab` : trim, essais, modes propres, tracés PNG.

DdT : essais quantitatifs de confiance A et B dans leurs tolérances ; tests de robustesse verts ; revue `aerodynamicien` sans point bloquant.
**Amendement du 2026-10-20 (décision du joueur)** : faute d'accès aux sources primaires, le M1 est validé avec des cibles de **confiance C**, étiquetées comme telles dans `essais.ron` et le rapport d'essais ; la confiance sera relevée plus tard (revalidation à prévoir). Premier avion validé : **SPAD VII 150 ch** (Hispano-Suiza 8Aa), le 180 ch ensuite.

## M2 — Premier vol
- Pont FDM ↔ ECS, pas fixe, interpolation, origine flottante.
- Terrain simple, ciel et atmosphère de base, caméras cockpit, poursuite et regard libre.
- Pilotage à la manette, télémétrie egui, gizmos de forces.
- Décollage et atterrissage sur herbe.

DdT : le joueur vole dix minutes à la manette et trouve cela agréable ; décollage et atterrissage possibles en préréglage Pilote.

## M3 — Beauté, première passe
- Nuages volumétriques (on peut les traverser), atmosphère, cycle jour/nuit.
- Terrain réel des Flandres avec LOD, occupation du sol 1917, zone du front.
- Modèle PBR du premier avion : toile translucide, hélice floue, câbles anticrénelés.

DdT : les scènes de référence sont capturées et approuvées par le joueur ; budget de performances tenu sur le GPU de référence.

## M4 — Combat
- Armes, synchronisation, balistique, volumes d'impact, modèle de dégâts par composants.
- Second avion (proposition : Albatros D.V) piloté par une IA qui utilise les mêmes commandes.

DdT : un duel 1 contre 1 gagnable et perdable en préréglage Pilote ; l'IA ne triche pas (vérifié par un test).

## M5 — Niveaux de réalisme
- `RealismSettings`, préréglages du tableau de `docs/vision.md`, instructeur virtuel, écran de réglages.

DdT : un vol scripté « débutant » par préréglage passe les tests de fumée ; un débutant s'amuse en Arcade ; un joueur exigeant ne voit pas d'aide cachée en As.

## M6 — Son et moteurs rotatifs
- Audio procédural (moteurs, vent, câbles, armes, ambiance du front).
- Sopwith Camel et Fokker Dr.I : précession gyroscopique, coupe-contact, mélange.

DdT : essais qualitatifs gyroscopiques conformes aux sources ; le joueur reconnaît le moteur à l'oreille.

## M7 — Missions et carrière
- Briefing, débriefing, carnet de vol, escadrille, revendications et témoins.
- Missions : patrouille, escorte, ballon. Le système hydrogène des ballons (cellules, fuites, inflammation) est conçu générique, pour servir aux dirigeables.

DdT : une campagne courte (5 missions) jouable de bout en bout.

## Ensuite
Chasse aux dirigeables (`docs/gdd/mission-dirigeables.md` : prototype en combat rapide, puis le « Raid silencieux »), autres avions et années (1915 à 1918), météo dynamique, biplaces, éditeur de missions, HOTAS et palonnier, suivi de tête opentrack, Windows, VR (OpenXR), multijoueur.
