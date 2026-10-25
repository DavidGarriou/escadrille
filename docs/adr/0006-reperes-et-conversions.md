# 0006 — Repères et conversions : Blender → glTF → Bevy → modèle de vol

- Statut : Accepté (validé par le joueur le 2026-10-25)
- Date : 2026-10-25

## Contexte

Quatre outils voient l'avion, chacun dans son repère. Tous sont directs (main droite), mais aucun n'oriente ses axes de la même façon :

| Repère | Avant (nez) | Droite (aile droite) | Haut | Source |
|---|---|---|---|---|
| Modèle de vol, corps | +x | +y | −z | conventions aéronautiques, `CLAUDE.md` |
| Modèle de vol, monde (NED local) | nord = +x | est = +y | −z | `CLAUDE.md` ; positions en f64 |
| Blender | −Y | −X | +Z | vue « Face » de Blender : on regarde le nez en direction de +Y |
| glTF 2.0 | +Z | −X | +Y | spécification Khronos (« front faces +Z ») |
| Bevy 0.20, repère local | −Z | +X | +Y | `Transform::forward`, `right`, `up` |

Ce qui oblige à décider maintenant :
- `CLAUDE.md` impose **une seule fonction de conversion**, dans `crates/game/src/flight/bridge.rs`, couverte par des tests. Elle n'existe pas encore.
- Le modèle 3D provisoire du SPAD (parcours `/nouvel-avion`, étape 8) et l'outil `asset_check` ont besoin d'une convention écrite.
- Une erreur de signe se voit mal et coûte cher. Un modèle en miroir gauche-droite inverserait à l'écran le sens de l'hélice et du couple ; un nez à l'envers ferait voler l'avion en marche arrière. `pipeline-assets` demande pour cela une scène de test.

Contraintes :
- le modèle de vol reste sans Bevy (ADR 0002) ;
- positions monde en f64 jusqu'au rendu, `Transform` relatifs à l'origine flottante (skill `bevy`, § 5) ;
- une hélice à droite est horaire vue du poste : ce sens doit se retrouver à l'écran.

Vérifié dans les sources épinglées :
- Bevy 0.20 et le modèle de vol utilisent la même version de glam (0.33) : les types `DVec3` et `DQuat` passent tels quels.
- Le chargeur glTF de Bevy 0.20 ne convertit rien par défaut. Il propose une conversion, `GltfConvertCoordinates` (`rotate_scene_entity`, `rotate_meshes`), marquée « expérimentale, le comportement peut changer » (`bevy_gltf-0.20.0/src/convert_coordinates.rs`). C'est un demi-tour autour de Y, appliqué à l'entité racine de la scène ou aux maillages.

## Options

**A. Monde : où mettre le nord dans Bevy.**
1. **Nord = −Z, est = +X, haut = +Y.**
   - Avantages : le « devant » par défaut de Bevy regarde le nord ; l'est suit l'abscisse UTM de la carte (skill `bevy`, § 5).
   - Conséquence : le corps et le monde se convertissent par **la même** rotation.
2. Nord = +X, est = +Z, haut = +Y.
   - Repère direct lui aussi, mais il faudrait une seconde rotation pour le corps, puisque le devant de Bevy est −Z. Deux fonctions au lieu d'une.

**B. glTF → Bevy : retourner le modèle.**
1. **Demi-tour fixe autour de Y, appliqué par notre pont** (`GLTF_TO_BEVY`, quaternion (0, 1, 0, 0)) à l'entité qui porte la scène glTF.
   - Avantages : explicite et testé chez nous ; il ne dépend d'aucune option expérimentale ; il est identique au demi-tour de Bevy, au cas où l'on y passerait plus tard.
2. Option expérimentale `GltfConvertCoordinates::rotate_scene_entity`.
   - Même effet, sans code, mais le comportement « peut changer », alors que chaque montée de version de Bevy est déjà coûteuse.
3. Exporter le nez vers −Z de glTF.
   - Contraire à la spécification : tout autre outil (visionneuses, `asset_check`) verrait l'avion à l'envers, et le modélisateur devrait penser contre Blender.

**C. Blender → glTF : modéliser l'avion.**
1. **Nez vers −Y, dessus vers +Z, aile droite vers −X**, export glTF avec « +Y Up » (option par défaut de l'exportateur, (X, Y, Z) ↦ (X, Z, −Y)), transformations appliquées, mètres, échelle 1.
   - Avantage : c'est la convention « Face » de Blender, et elle donne exactement le nez +Z de glTF.
2. Nez vers +X, habitude de certains simulateurs.
   - Une rotation de plus à l'export ou au chargement, source d'erreurs.

**D. Origine du modèle 3D.**
1. **La référence des cotes (`datum`) d'`aircraft.ron`** (pour le SPAD : bord d'attaque du plan inférieur, dans l'axe).
   - Le modèle ne dépend pas du chargement. L'entité de l'avion est au centre de gravité du modèle de vol ; le modèle 3D en est un enfant, décalé de −`cg_from_datum`, converti.
2. Le centre de gravité.
   - Il change avec le carburant et les munitions : il faudrait réexporter ou décaler quand même. Rejeté.

**E. Attitudes.**
1. **Conjuguer le quaternion par la rotation de changement de repère** : (w ; v) ↦ (w ; M v).
   - Exact, sans singularité.
2. Passer par des angles d'Euler : blocage de cardan et conventions d'ordre à tenir. Rejeté.

**« Ne rien décider »** : chaque plugin convertirait à sa façon ; c'est exactement ce que `CLAUDE.md` interdit.

## Décision

Options A1, B1, C1, D1 et E1. Le changement de repère est **une seule rotation propre M**, la même pour le corps et pour le monde.

| De | Vers | Formule |
|---|---|---|
| modèle de vol (corps ou NED) | Bevy (local ou monde) | (x, y, z) ↦ (y, −z, −x) — `bridge::to_bevy` |
| Bevy | modèle de vol | (X, Y, Z) ↦ (−Z, X, −Y) — `bridge::from_bevy` |
| quaternion du modèle de vol (corps → NED) | rotation Bevy (local → monde) | (w ; v) ↦ (w ; M v) |
| glTF | Bevy local | demi-tour autour de Y : (X, Y, Z) ↦ (−X, Y, −Z), quaternion (0, 1, 0, 0) |
| Blender | glTF | (X, Y, Z) ↦ (X, Z, −Y), fait par l'exportateur (« +Y Up ») |
| corps | glTF | (x, y, z) ↦ (−y, −z, x), composition des deux premières lignes, pour `asset_check` |
| corps | Blender | (x, y, z) ↦ (−y, −x, −z), pour le modélisateur |

- Position de l'entité d'un avion : `to_bevy(position NED)`, en f64, moins l'origine de rendu, convertie en f32 à la toute fin.
- Modèle 3D : enfant de l'entité, de translation `to_bevy(−cg_from_datum)` et de rotation `GLTF_TO_BEVY`.
- Vitesses, vent, forces et vitesses angulaires : même fonction `to_bevy`. Une rotation propre transforme de la même façon les vecteurs et les pseudo-vecteurs.
- Raison principale : une seule formule, déduite des conventions de chaque outil et non choisie à la main. Elle est vérifiable par des cas simples (cap à l'est, cabré, inclinaison à droite), testés dans `bridge.rs`.

## Conséquences

- **Plus facile.**
  - Un seul endroit où chercher une erreur de signe : `crates/game/src/flight/bridge.rs` et ses tests (axes, repère direct, aller-retour, cas de vol, origine du modèle).
  - Le modélisateur a une règle simple : nez vers −Y, dessus vers +Z, aile droite vers −X, export par défaut.
- **À faire.**
  - **`asset_check`** devra lire les dimensions et les pivots dans les axes glTF, par la ligne « corps → glTF ». Si un outil sans Bevy a besoin de la conversion elle-même, elle passera dans une crate sans moteur (`escadrille_data`) et le pont la réexportera : jamais deux copies.
  - **Scène de test** (`pipeline-assets`) : avec le premier modèle 3D, une capture vérifiera que le nez, l'aile droite et le sens de l'hélice sont ceux du modèle de vol.
  - **Pont du M2** : interpolation des états, origine flottante (`WorldPos`, `RenderOrigin`), gizmos de forces. Tous passent par `to_bevy`.
- **À surveiller.**
  - Si l'option expérimentale de Bevy devient la norme et change le comportement par défaut du chargeur, le demi-tour serait compté deux fois. Le test de la scène de référence le verrait.
  - Toute montée de version de Bevy relit cet ADR.
- **Hors de cet ADR** : la projection de la carte (UTM 31N, point de référence : skill `pipeline-assets`) et la courbure de la Terre (terre plate locale, skill `bevy`, § 5).
