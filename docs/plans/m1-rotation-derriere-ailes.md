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

(Chiffres à 8 bandes par demi-aile. Avec des bandes convergées, les ailes reprennent 0,79 / 0,67 / 0,93 / 0,48 Q : voir la revue dans les résultats.)

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

### Revue de l'aérodynamicien (2026-10-25)

Verdict : à corriger. Le code est juste : aucune erreur de signe, aucun double compte, aucune discontinuité des efforts. Mais les premiers chiffres publiés n'étaient pas convergés.

- **Bloquant : discrétisation.**
  - Avec 8 bandes par demi-aile (0,44 à 0,49 m), le jet n'était couvert que par 1,5 à 2 bandes par demi-largeur.
  - Or la fraction, 1 − M/Q, est une petite différence entre deux grands nombres. À Vmax, 14 % d'erreur sur M (0,93 Q au lieu de 0,80 Q) triplaient la rotation laissée (0,07 au lieu de 0,21).
  - **Artefacts** de la première version, retirés des documents :
    - la cellule qui reprenait plus que le couple au plané ;
    - la borne 0 atteinte ;
    - le bilan d'énergie à −1 % en montée.
  - **Corrigé** : SPAD à 16 bandes par demi-aile, empennage et dérive doublés (C:, convergence). Nouveau test `the_swirl_left_converges_with_the_strips` : doubler encore change la fraction de moins de 0,02, et les commandes à l'équilibre de moins de 0,005 de la course (0,01 au décrochage plein gaz, équilibré au maximum de portance, où le décrochage de chaque bande est lui aussi discrétisé : 0,0065 mesuré).
- **Important 1 : la marge statique était du bruit de mesure.**
  - La différence centrée de 1e-4 m/s lisait la pente d'un seul segment des tables linéaires par morceaux. La marge sautait de 2 à 6 % selon la vitesse de trim. L'écart 5,9 → 4,9 % ne voulait donc rien dire. Le creux du profil d'empennage existe, mais il est dans la dispersion d'une table à trois décimales tous les 1,5°. La symétrisation l'a recopié de part et d'autre de α = 0.
  - **Corrigé** : marge et dérivées latérales par sécante sur ±1° d'incidence, de dérapage et de déflexion, et ±2 % de régime (`static_stability`).
  - **Reste** la pente avec la vitesse (ci-dessous), surtout due au C_m de l'aile : d'un segment à l'autre, dC_m/dC_L va de −0,046 à +0,044. Lisser les tables (droite des moindres carrés de l'empennage, C_m de l'aile) est un choix de traitement des données, **à décider par le joueur** (ADR).
- **Important 2 : coût.**
  - Le calcul « sans rotation » tournait à chaque passe de l'itération mutuelle, soit 15,6 % des instructions par évaluation (mesure callgrind de la revue). Or l'itération n'utilise que les C_L, que la part de rotation ne change pas.
  - **Corrigé** : l'itération passe l'écoulement sans sa part de rotation ; seule la passe finale la sépare. Un test vérifie que les C_L et les angles mutuels sont identiques. `just test` passe de 145 à 65 s.
  - Reste le vrai poste : environ 12 passes d'itération mutuelle à 1e-13. Un Newton sur les C_L est à faire avant les avions de l'IA (hors plan).
- **Important 3 : test d'énergie.** La borne |P| < 5 % ne teste plus rien de physique, puisque le signe du total est dans le bruit de discrétisation. Ce qui est robuste, c'est le partage : ailes négatives en montée (−0,09 kW), empennage positif. Une estimation à la main de la revue rend ce signe plausible : poussée τ ΔC_L δ ≈ 0,007 q S, contre environ 0,009 de montée de la traînée de profil. **Gardé** comme non-régression ; un test de la décomposition (traînée de profil figée) est noté dans les suites.
- **Important 4 : ce qu'on peut dire de l'équilibre.**
  - Le trim est ailes horizontales, dérapage libre : « sans ailerons ni direction » s'entend avec 0,7° de dérapage à Vmax et 2,0° en montée. Bille au centre, il faudrait environ 120 N·m de lacet en montée.
  - Confiance **D** :
    - c/D = 0,5, au-delà du D/3 de R&M 2368 ;
    - la seule mesure (TN 1146, c/D ≈ 0,8) est sous forte traction, et le modèle y reste en dessous.
  - Le 0,80 Q de croisière est bien la théorie des bandes : la revue trouve 0,77 Q à la main, ce n'est pas un bogue.
  - Le témoignage T8 (Meister, SPAD XIII : « beaucoup de pied ») porte sur un autre avion, à réducteur, à rotation inverse et au couple double. Ce n'est pas une contradiction, mais c'est un signal.
- **Suggestions retenues** :
  - test miroir (hélice à gauche : même fraction, efforts latéraux exactement opposés), ajouté ;
  - borne 0 justifiée par l'argument des grilles d'aubes (une aube sans cambrure dans le plan de la rotation ne sur-dévie pas), et biais de la borne 1 au décrochage (moins de rotation sur l'empennage), écrits dans le code et dans `modele-aero.md`.
- **Réponses** : la différence « avec − sans rotation » est le bon moment pour ce modèle. Le roulis, le dérapage et les ailerons font un sillage latéral sur toute l'envergure, qu'il faudrait modéliser globalement, pas par le seul jet. Les bornes sont défendables en E: ; une fois les bandes convergées, aucun des quatre vols ne les atteint. Pas de retard au M1.

### Rotation laissée et commandes à l'équilibre (bandes convergées)

`fdm_lab souffle spad7`, 16 bandes par demi-aile. « Avant » : le même code, fraction figée à 0,5 (l'ancienne donnée).

| Vol | Fraction laissée | Roulis du tourbillon sur la cellule, avant → après | Ailerons, avant → après | Direction, avant → après |
|---|---|---|---|---|
| Vmax, 2 000 m | 0,21 | 1,08 → 0,91 Q | −0,001 → +0,003 | +0,033 → −0,004 |
| Meilleure montée, 0 m | 0,33 | 0,94 → 0,85 Q | +0,008 → +0,013 | +0,062 → +0,019 |
| Plané, 35 m/s | 0,07 | 1,23 → 0,97 Q | −0,000 → +0,000 | +0,002 → −0,004 |
| Décrochage plein gaz | 0,52 | 0,73 → 0,73 Q | +0,080 → +0,079 | +0,095 → +0,100 |

- Commandes en fraction de la course, positives pour roulis à droite et nez à droite.
- La cellule ne reprend plus nulle part plus que le couple. Puissance reprise au tourbillon : 8 / 1 / 13 / 7 % de l'énergie de rotation (le plané porte presque rien).
- Biplan d'essai (synthétique, non modifié) : 0,09 / 0,31 / 0 / 0,49 à 6 bandes par demi-aile. Il ne converge pas aussi bien : 0 / 0,11 / 0 / 0,38 à 12 bandes, et 0,02 / 0,16 / 0 / 0,39 à 24. Au plané, ses ailes reprennent 1,08 Q à toute finesse : c'est sa géométrie (corde large devant le jet), pas la discrétisation.
- **Lecture**, confiance D : dans le modèle, le couple se sent surtout près du décrochage (8 % d'ailerons, 10 % de direction au décrochage plein gaz) et, au M2, au décollage. En palier et en montée, il en faut 2 % au plus. Même l'ancienne moitié de rotation n'en demandait que 3 à 6 % en montée.

### Campagne du SPAD

| Mesure | Avant le plan (8 bandes, fraction 0,5) | Après (16 bandes, bilan de moment cinétique) |
|---|---|---|
| Vitesses maximales (2 000 / 3 000 / 4 000 m) | −0,2 / +4,1 / +2,8 % | −0,2 / +4,0 / +2,7 % |
| Montées (2 000 / 3 000 / 4 000 m) | −1,7 / 0,0 / −3,3 % | −1,5 / +0,2 / −3,1 % |
| Plafond | +5,0 % | +4,9 % |
| Commandes en croisière (153 km/h, 2 000 m) : ailerons, direction | −0,00, +0,03 | +0,00, −0,00 |
| Départ au décrochage moteur réduit (à 3 s) | à gauche, −54°, nez −15° | à gauche, −55°, nez −17° |

- **Performances** : moins de 0,2 point, comme prévu.
- **Marge statique**, désormais par sécante (Important 1) : 3,8 / 5,0 / 5,1 / 4,7 / 4,0 / 3,2 / 1,2 % à 140 / 145 / 150 / 153 / 156 / 160 / 170 km/h, 5,5 % au trim du rapport (153,5 km/h). Elle baisse avec la vitesse et reste sensible aux tables (C_m de l'aile surtout). On ne cite pas un chiffre seul.
- **Départ au décrochage moteur réduit**, sensibilité (§ 9 de `spad7-calculs.md`), avec une copie du binaire munie de variantes temporaires, jamais commitées :

(en cours de calcul avec les bandes convergées)

- **Robustesse** (boucle ouverte, sensible à la rotation sur l'empennage) :
  - vol sur le dos : 161 km/h et 99 m perdus après 10 s, contre 176 et 136 ;
  - glissade sur la queue : 112 km/h contre 80, assiette 62° contre 69° ;
  - piqué à 60° : 224 km/h au plus, contre 243, sa valeur d'avant le sillage des ailes.

## Hors de ce plan

- **Retard** de la rotation entre les ailes et l'empennage (état auxiliaire, comme la déflexion), à la vitesse du jet V + 2v_i : environ 0,06 s à Vmax, 0,1 s au décrochage plein gaz.
- **Lissage des tables** (droite des moindres carrés de l'empennage, C_m de l'aile) : décide de la marge statique et de sa pente avec la vitesse. À trancher par le joueur, en ADR.
- **Newton sur les C_L de l'induction mutuelle**, à la place d'une douzaine de passes à 1e-13 : le vrai poste de coût, à faire avant les avions de l'IA.
- **Test d'énergie par décomposition** : traînée de profil figée, la puissance des ailes doit être positive et au plus 0,28 à 0,37 de l'énergie de rotation interceptée ; la part de profil, négative.
- **Bornes** : exposer le rapport non borné dans les diagnostics, compter quand la borne joue dans les scénarios de robustesse, envisager une saturation C¹ pour le trim et la linéarisation.
- **Redressement uniforme sur le rayon** (E:) : la contre-rotation des ailes est concentrée près de l'axe et aux bords du jet ; le plan fixe, 0,32 m au-dessus de l'axe, en voit sans doute plus que la moyenne.
- **Oubliés du bilan** : mâts, câbles et fuselage reprennent aussi de la rotation, sans entrer dans M.
- **Questions pour l'historien** : les ailes du S.VII étaient-elles calées différemment à gauche et à droite pour compenser le couple ? Ce serait une mesure indirecte du couple en croisière.
- **Essai manette en main** (M2) : montée plein gaz vers 115 km/h, palonnier lâché ; l'avion doit dériver lentement à gauche. À consigner dans `docs/gdd/retours.md`.
- **Réponse de l'aile au tourbillon en charge légère** (0,9 Q et plus) : aucune mesure trouvée ; seule la ligne portante à matrice d'influence, ou une source nouvelle, pourrait la trancher.
- **Décalage du biplan** : le plan inférieur, en arrière du plan supérieur, voit encore toute la rotation.
- **Empennage du TN 1146** : comparer les forces latérales mesurées sur la dérive (figure 13) contrôlerait la réponse de la dérive à la rotation, sous forte traction. Utile, mais ne départage pas l'ancienne donnée et le bilan, qui y donnent presque la même chose.
