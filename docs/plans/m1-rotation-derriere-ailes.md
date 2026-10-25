# Plan — Rotation du souffle derrière les ailes, par le bilan de moment cinétique

- Statut : demandé par le joueur le 2026-10-25 (« Prépare le plan de la suite 1 et lance le ») ; rédigé et lancé dans la même session.
- Contexte : première suite, « prioritaire », de `m1-souffle-ailes.md` (revue de l'aérodynamicien du 2026-10-25, point important 2). Préalable à toute conclusion sur l'équilibre en roulis et en lacet en croisière.
- Règle inchangée : **aucun coefficient réglé sur les cibles du SPAD**.

## Le problème

Derrière les ailes, l'empennage voit une fraction fixe de la rotation du souffle : la donnée `swirl_left` = 0,5. Mais les ailes du modèle reprennent au tourbillon une part du couple Q qui varie beaucoup selon le vol (`fdm_lab souffle spad7`) :

| Vol | Repris par les ailes | Rotation qui devrait rester | Donnée |
|---|---|---|---|
| Vmax, 2 000 m | 0,93 Q | 0,07 | 0,5 |
| Meilleure montée, 0 m | 0,73 Q | 0,27 | 0,5 |
| Plané, 35 m/s | 1,12 Q | 0 | 0,5 |
| Décrochage plein gaz | 0,41 Q | 0,59 | 0,5 |

Conséquences :
- à Vmax, la cellule reprend 1,2 Q, plus que ce que l'hélice a mis dans l'air ;
- l'équilibre en roulis et en lacet en croisière n'est pas interprétable ;
- quand les ailes décrochent, elles reprennent moins de rotation, mais l'empennage n'en voit pas plus : le départ au décrochage ne sent pas cet effet.

## Ce que disent les sources

| Source | Contenu | Ce qu'on en retient |
|---|---|---|
| Conservation du moment cinétique (équation d'Euler des turbomachines) | Le flux de moment cinétique du jet vaut ṁK derrière l'hélice (K = Q/ṁ) ; le moment que les ailes reçoivent de la rotation lui est retiré. | Derrière les ailes : K′ = K − M_ailes/ṁ (C:). |
| Purser et Spear, NACA TN 1146 (1946), relu le 2026-10-25 | « It has been estimated (reference 11) that a wing behind the propeller may diminish the angle of twist in the slipstream by 50 percent. » Avec cette moitié, les forces latérales calculées sur la dérive « generally agreed with the computed values within 10 to 15 percent », sauf la queue longue à 35°, au bord du souffle dévié par l'aile. Le roulis sans empennage mesuré (50 à 60 % du couple) montre que l'aile en reprenait moins que la moitié supposée. | Sous forte traction, les deux mesures du TN 1146 vont ensemble : l'aile reprend 40 à 50 % du couple, il reste 50 à 60 % de la rotation, et la dérive le confirme à 10-15 % près. Le bilan de moment cinétique donne la même chose : c'est un contrôle, pas un étalonnage. |
| Biermann et Herman, NACA Report 747 (1942), la « référence 11 » du TN 1146 | Attente des auteurs : une aile enlève « environ la moitié » de la rotation ; leur mesure porte sur un gain d'énergie. | Aucune mesure de la rotation elle-même. |
| Squire et Chester, R&M 2368, partie II (1945) | Le moment de roulis d'une aile infinie dans un jet en rotation égale celui de la théorie des bandes, et dépasse Q dès que la corde dépasse 0,375 D. Les auteurs jugent ce résultat « surprenant », et leur théorie peu sûre au-delà d'une corde de D/3 (SPAD : c/D ≈ 0,5). | En charge légère, le moment des ailes du modèle (0,9 Q et plus) est dans la zone peu sûre de la théorie. Le bilan le transmet à l'empennage tel quel : l'erreur éventuelle reste en un seul endroit, la réponse de l'aile. |
| Recherche du 2026-10-25 (Ribner et MacLachlan 1947, fuselage ; NACA RM SL53G10, XF-91) | Rotation du souffle sur le fuselage et sur la dérive. | Aucune mesure de la rotation résiduelle derrière une aile, en charge légère. |

**Correction** du tableau des sources de `m1-souffle-ailes.md` : il disait que les forces de dérive du TN 1146 « tombent à 10-15 % des mesures ». C'est faux : elles s'accordent avec les mesures **à** 10-15 % près.

## Ce qu'on change

1. **Bandes** : la portance que la rotation ajoute (ΔC_L, déjà calculée pour la traînée de rotation) est calculée pour toute bande dans un tourbillon. Sa force, avec la traînée de rotation, est rendue avec les efforts de la surface.
2. **Système portant** : il sait calculer un sous-ensemble de surfaces. Les ailes d'abord, puis le reste. Une paire d'induction mutuelle ne peut pas être à cheval sur les deux groupes (vérifié à la construction).
3. **Avion**, à chaque évaluation :
   - les ailes, dans le souffle de l'hélice avec toute sa rotation ;
   - M_ailes, moment de leur force de rotation autour de l'axe de l'hélice ;
   - fraction laissée : 1 − M_ailes/(ṁK), bornée entre 0 et 1 :
     - pas de contre-rotation, que la théorie, peu sûre là, permettrait (E:) ;
     - pas d'amplification par une aile décrochée, dont la portance baisse quand l'incidence monte (E:) ;
   - l'empennage, les autres surfaces, les traînées et le fuselage dans ce souffle redressé, la chute se faisant sur une corde moyenne comme aujourd'hui ;
   - la fraction laissée passe dans les diagnostics.
4. **Pas de retard** pour l'instant, contrairement à la déflexion.
   - Le transport des ailes à l'empennage prend environ l/V ≈ 0,1 s.
   - Un état retardé ajouterait une inconnue au trim et un état aux modes propres, pour un effet du second ordre.
   - Noté dans les suites.
5. **Données** : `swirl_left` disparaît du bloc `downwash` (SPAD, Messenger, biplan d'essai) et de `format-avion.md`.
6. **`fdm_lab souffle`** : la fraction affichée est celle du modèle. Le contrôle par différence (efforts avec et sans rotation) devient un test.

## Tests

- **Bande** : la force de rotation égale la différence entre la bande avec et sans l'incidence de la rotation (petite rotation, domaine linéaire). Elle est nulle sans rotation et suit la polaire après le décrochage.
- **Système** : la somme des deux sous-ensembles égale le calcul d'un bloc quand les écoulements n'en dépendent pas. Une paire mutuelle à cheval est refusée.
- **Souffle** : ṁK′ = ṁK − M ; fraction bornée ; continuité quand K change de signe (hélice qui freine).
- **Avion** :
  - la fraction du modèle égale 1 − M_ailes/Q mesuré par différence, à 1 % près ;
  - sans sillage (hélice propulsive, sans bloc `downwash`, sans hélice), rien ne change ;
  - souffle continu en T = 0 ;
  - simulation déterministe.
- **`fdm_lab`** : invariant d'énergie (SPAD et biplan d'essai, quatre vols) ; maquette du TN 1146 inchangée, car elle n'a pas d'empennage.

## Essais et rapport

- `/essai-en-vol spad7`.
- Commandes à l'équilibre en roulis et en lacet, à Vmax et en meilleure montée, avant et après. Résultat du modèle, pas une validation : il dépend du moment des ailes en charge légère, peu sûr.
- § 9 de `spad7-calculs.md` : le départ au décrochage est refait. La sensibilité porte sur les bornes (contre-rotation permise) et sur la longueur de chute, la fraction n'étant plus une donnée.
- Revue de l'aérodynamicien.
- Documentation : `modele-aero.md`, `format-avion.md`, § 10 de `spad7-calculs.md`.
- Carnet de bord, commit et push.

## Effets attendus (ordres de grandeur, avant le code)

- **Performances** : presque rien, moins de 0,3 %. La poussée que l'empennage reprend à la rotation vaut 0,2 kW au plus en montée, pour une centaine de kW.
- **Empennage** : rotation divisée par 7 à Vmax et par 2 en montée. Le roulis et la force latérale qu'il en reçoit baissent d'autant. Le réglage de direction en croisière change.
- **Décrochage plein gaz** : un peu plus de rotation sur l'empennage (0,59 au lieu de 0,5), et davantage pendant le départ, à mesure que les ailes décrochent.
- **Biais** : aucune cible chiffrée de `essais.ron` ne dépend de ce changement. Le côté du départ est déclaré non significatif, et les réglages en croisière ne sont pas des cibles.

## Résultats (2026-10-25)

### Mise en œuvre

- **Bandes** (`swirled_strip_loads`) : la part de la rotation dans les efforts d'une bande est la différence entre la bande et la même bande sans la rotation, chacune avec son angle induit résolu.
  - Elle donne la force de rotation (`StripState::swirl_force`) et la portance ajoutée ΔC_L de la traînée de rotation.
  - Première version écartée : ΔC_L lu sur la polaire à l'incidence effective moins δ_s/(1 + k a₀). Elle s'écartait du contrôle par différence de 0,02 à Vmax et de 0,05 au décrochage plein gaz. Deux causes : la pente locale de la polaire n'est pas a₀ (table linéaire par morceaux, décrochage) ; à grande incidence, la rotation change aussi la pression dynamique de la bande au premier ordre, puisqu'elle a une composante le long de l'écoulement.
- **Système** : `LiftingSystem::loads_of` calcule un sous-ensemble de surfaces, et `SystemLoads::with` réunit deux sous-ensembles disjoints. L'avion refuse une paire d'induction mutuelle à cheval entre les ailes et le reste.
- **Souffle** : le sillage des ailes (`WingWake`) ne porte plus que sa géométrie. `Slipstream::swirl_left_after` donne la fraction laissée par le bilan de moment cinétique.
- **Avion** : les ailes d'abord, le reste ensuite ; la fraction passe dans `Diagnostics::swirl_left`. `Aircraft::slipstream` rend le souffle tel qu'il atteint les ailes.
- **Données** : `swirl_left` supprimé (SPAD, Messenger, biplan d'essai, `format-avion.md`).
- **`fdm_lab souffle`** : fraction du modèle, contrôle par différence, et commandes à l'équilibre en roulis et en lacet.
- **Tests ajoutés** :
  - force de rotation d'une bande, en attaché et sur la pente descendante du décrochage ;
  - sous-ensemble de surfaces ;
  - bilan de moment cinétique du souffle (bornes, continuité, hélice en moulinet) ;
  - fraction laissée contre le contrôle par différence, sur le biplan d'essai à quatre incidences, et sur le SPAD et le biplan d'essai à quatre vols (à 0,01 près) ;
  - continuité des efforts quand le couple de l'hélice change de signe ;
  - paire mutuelle à cheval refusée ; hélice propulsive sans fraction.

### Rotation laissée et commandes à l'équilibre

`fdm_lab souffle spad7`. « Avant » : le même code, fraction figée à 0,5 (l'ancienne donnée).

| Vol | Fraction laissée | Roulis du tourbillon sur la cellule, avant → après | Ailerons, avant → après | Direction, avant → après |
|---|---|---|---|---|
| Vmax, 2 000 m | 0,07 | 1,20 → 0,97 Q | −0,004 → +0,000 | +0,031 → −0,024 |
| Meilleure montée, 0 m | 0,28 | 0,97 → 0,86 Q | +0,005 → +0,011 | +0,057 → +0,001 |
| Plané, 35 m/s | 0 | 1,40 → 1,12 Q | −0,001 → −0,000 | +0,002 → −0,005 |
| Décrochage plein gaz | 0,59 | 0,65 → 0,69 Q | +0,099 → +0,092 | +0,098 → +0,128 |

- Commandes en fraction de la course, positives pour roulis à droite et nez à droite.
- La cellule ne reprend plus plus que le couple, sauf au plané : là, les ailes seules en reprennent 1,12 Q, et l'empennage ne voit plus rien.
- Biplan d'essai : fraction 0,09 / 0,31 / 0 / 0,49. À Vmax, l'empennage reprend 0,10 Q d'un souffle qui n'en porte plus que 0,09 : même excès de la théorie des bandes, à l'empennage cette fois.
- **Lecture** : en croisière et en montée, les ailes compensent presque tout le couple, et l'avion tient sans ailerons ni direction. C'est un résultat du modèle, **pas une validation** : il repose sur le moment des ailes en charge légère, que la théorie juge peu sûr et qu'aucune mesure ne confirme.

### Campagne du SPAD

| Mesure | Avant | Après |
|---|---|---|
| Vitesses maximales (2 000 / 3 000 / 4 000 m) | −0,2 / +4,1 / +2,8 % | −0,2 / +4,0 / +2,7 % |
| Montées (2 000 / 3 000 / 4 000 m) | −1,7 / 0,0 / −3,3 % | −1,5 / +0,1 / −3,2 % |
| Plafond | +5,0 % | +5,0 % |
| Marge statique (croisière, 153 km/h, 2 000 m) | 5,9 % | **4,9 %** |
| Départ au décrochage moteur réduit (à 3 s) | à gauche, −54°, nez −15° | à gauche, −55°, nez −17° |

- **Performances** : moins de 0,2 point, comme prévu.
- **Marge statique** : −1 point, expliqué.
  - Sonde à 150 km/h : 5,7 % avec la fraction du modèle (≈ 0,1), 5,7 % figée à 0,1, 6,7 % figée à 0,5. C'est donc le niveau de rotation derrière les ailes qui compte, pas sa variation avec l'incidence.
  - Toute la différence est dans le plan fixe : −683 N·m/(m/s) avec la moitié de la rotation, −645 avec 0,1.
  - Mécanisme : le profil d'empennage (RAF 15 décambré puis symétrisé, `spad_empennage.ron`) a une pente plus faible près de la portance nulle. Elle vaut 0,069 par degré entre −0,75° et +0,75°, contre 0,076 à 0,081 au-delà. Le plan fixe travaille dans ce creux en croisière ; la rotation, de quelques degrés, l'en sortait en partie.
  - Conséquence du modèle, sensible à cette particularité du profil, qui vient d'une mesure de 1918 (Report 233) déplacée et symétrisée (C:). À regarder si la marge devient une cible.
- **Départ au décrochage moteur réduit**, sensibilité (§ 9 de `spad7-calculs.md`), avec une copie du binaire munie de variantes temporaires, retirées depuis :

| Variante | Départ (à 3 s) |
|---|---|
| Modèle | à gauche, −55° |
| Contre-rotation permise (borne basse −1) | à gauche, −55° |
| Chute sur une demi-corde / sur deux cordes | à gauche, −56° / −55° |
| Fraction figée à 0,3 / 0,7 | à gauche, −53° / −56° |
| Fraction figée à 1 (aucun redressement) | à droite, +78° |

  Le côté résiste à toutes les variantes physiques. Seule l'absence de tout redressement, que contredisent les mesures du NACA, le change.
- **Robustesse** : vol sur le dos (164 km/h et 106 m perdus après 10 s, contre 176 et 136), glissade sur la queue (113 km/h contre 80, assiette 62° contre 69°), piqué à 60° (225 km/h au plus, contre 243). Ce sont des scénarios en boucle ouverte, sensibles à la rotation sur l'empennage. Le piqué retrouve sa valeur d'avant le sillage des ailes (225 km/h).

## Hors de ce plan

- **Retard** de la rotation entre les ailes et l'empennage (état auxiliaire, comme la déflexion).
- **Réponse de l'aile au tourbillon en charge légère** (0,9 Q et plus) : aucune mesure trouvée ; seule la ligne portante à matrice d'influence, ou une source nouvelle, pourrait la trancher.
- **Décalage du biplan** : le plan inférieur, en arrière du plan supérieur, voit encore toute la rotation.
- **Empennage du TN 1146** : comparer les forces latérales mesurées sur la dérive (figure 13) contrôlerait la réponse de la dérive à la rotation, sous forte traction. Utile, mais ne départage pas l'ancienne donnée et le bilan, qui y donnent presque la même chose.
