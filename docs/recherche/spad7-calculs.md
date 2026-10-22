# SPAD S.VII 150 ch — note de calcul des données de vol

- Avion : `spad7` (S.VII C.1, Hispano-Suiza 8Aa de 150 ch, une Vickers), décision du joueur du 2026-10-22.
- Rôle : justifier chaque valeur `C:` et `E:` des fichiers `assets/aircraft/spad7/` et `assets/airfoils/spad.ron`. Les valeurs sourcées (`S:`) renvoient directement aux fiches [`spad7.md`](spad7.md), [`hispano-suiza-8a.md`](hispano-suiza-8a.md) et [`profil-spad7.md`](profil-spad7.md).
- Méthode générale : la liste de l'aérodynamicien (revue de `fdm_lab` du 2026-10-22, consignée dans le carnet de bord) ; chaque estimation est une valeur de départ, jamais ajustée pour faire passer un essai.

## 1. Profil d'aile (`assets/airfoils/spad.ron`)

**Base mesurée** : RAF 15 du NACA Report 233, tableau XXX (Re = 3,58 × 10⁶, maquette rectangulaire d'allongement 6, soufflerie fermée de 60 in, sans correction de paroi), recopié tel quel (`profil-spad7.md` § 3.1).

**Passage à l'allongement infini** (`profil-spad7.md` § 4.2, TR 416) : le chargeur applique α∞ = α − C_L (1 + τ)/(π R_e) et C_D∞ = C_D − C_L² (1 + δ)/(π R_e), avec τ = 0,175 et δ = 0,05 (TR 416, fig. 11, allongement 6). L'allongement effectif de la soufflerie fermée, R_e = R / [1 − ½ (b/D)²] = 6 / [1 − ½ (30/60)²] = **6,857**, fait la correction de paroi et la réduction en une seule formule. Contrôle : le terme de paroi vaut 0,380° par unité de C_L, celui que le Report 331 applique à ces mêmes mesures. La seconde correction du TR 669 (+7 % sur le C_L max, −4 % sur la pente) **n'est pas appliquée** : elle a été établie sur la soufflerie reconstruite des années 1930 ; à trancher en revue.

**Du RAF 15 au « Spad »** : théorie du profil mince sur les deux lignes moyennes (ligne moyenne = demi-somme des ordonnées, ramenée à la corde qui joint le bord d'attaque au bord de fuite ; intégrales de Glauert calculées numériquement sur une interpolation monotone des ordonnées).

| Profil | Ordonnées | Cambrure | Épaisseur | α de portance nulle (théorie) | C_m ¼ c (théorie) | Mesure |
|---|---|---|---|---|---|---|
| RAF 15 | Report 233, tableau XXXVI, p. 82, lu sur l'image | 2,67 % à 30 % | 6,38 % | −2,58° | −0,0545 | α₀ ≈ −2,25° ; C_m −0,045 à −0,054 |
| « Spad » | Report 93, réf. 49 (`profil-spad7.md` § 2.2) | 3,77 % à 40 % | 5,36 % | −3,18° | −0,0690 | — |

Ordonnées du RAF 15 lues (% de corde ; x = 0, 1,25, 2,5, 5, 7,5, 10, 15, 20, 30 … 100) :
- extrados 0,20 ; 1,90 ; 2,75 ; 3,93 ; 4,55 ; 5,06 ; 5,58 ; 5,77 ; 5,85 ; 5,68 ; 5,34 ; 4,78 ; 4,10 ; 3,30 ; 2,26 ; 1,63 ; 0,30 ;
- intrados 0,20 ; −0,45 ; −0,71 ; −0,94 ; −1,00 ; −1,00 ; −0,80 ; −0,50 ; −0,04 ; −0,03 ; −0,22 ; −0,46 ; −0,69 ; −0,59 ; −0,32 ; −0,18 ; +0,30.

La théorie retrouve le RAF 15 mesuré à 0,3° et 0,005 près ; on n'en garde que les **écarts** : toute la courbe est décalée de **−0,60°** en incidence et de **−0,0145** en C_m (`camber_correction`). Pente, traînée et décrochage restent ceux du RAF 15 (confiance C pour le « Spad »).

**Branche négative (E:)** : aucune mesure du RAF 15 au-delà de −6° (`profil-spad7.md` § 3.7). Pente conservée jusqu'à −7,5° (le Report 309 est encore linéaire à −6°), minimum arrondi C_L = −0,42 vers −9° (profil mince à intrados creux : décollement précoce de l'intrados), puis chute douce ; traînée qui monte vers les valeurs du décrochage positif ; C_m qui tend vers zéro. Seuls les vols dos et les ressources négatives en dépendent.

## 2. Hélice (`assets/aircraft/spad7/propeller.ron`)

**Comparable** : hélice en bois Navy D' (P/D 0,8, 2,39 m) mesurée sur le VE-7 complet, NACA Report 301, tableau III (`hispano-suiza-8a.md` § 7.4). Le diamètre du S.VII retenu est **2,45 m** (trois documents sur quatre, `hispano-suiza-8a.md` § 7.1) ; à P/D 0,8, le pas vaut 1,96 m, celui de la Levasseur série 485 (1,95 m). Les coefficients, sans dimension, sont repris tels quels.

**De la traction effective à la traction de l'hélice** : le Report 301 donne une traction « effective », diminuée du surcroît de traînée que le souffle impose à l'avion. Le modèle de vol calcule lui-même cette traînée sur les éléments placés dans le souffle : la reprendre compterait deux fois. On rajoute donc la traînée de souffle du VE-7 :
- ΔC_T = ½ Σ (f/D²) [(J + s·ν)² − J²], avec ν = v_i/(nD) solution de C_T = (π/2) ν (J + ν) (Froude) et s = 1 + x/√(x² + R²), la loi du souffle du modèle à la distance x derrière le disque ;
- éléments du VE-7 dans le souffle (E:, avion non coté dans les documents lus) : radiateur frontal, f = 0,15 m² à 0,3 m ; fuselage, f = 0,20 m² à 1,5 m ;
- résolu par point fixe, puisque ν dépend de la traction réelle.

Résultat : +5 à +6,4 % de traction selon J ; le rendement maximal passe de 0,775 à 0,83, valeur d'une hélice isolée.

**Effet des ailes** (Report 301, hélice I, avion complet contre avion sans ailes ni empennage, tableaux III p. 19 et 22 lus sur l'image) : aux J de croisière (0,6 à 0,8), la traction effective ne change pas, mais la puissance absorbée est plus forte de 3 à 5 % avec les ailes (blocage). Le modèle ne représente pas ce blocage : le C_P du D' est donc **gardé avec les ailes**. Aux faibles J, les ailes coûtent 3 à 5 % de traction effective, dont une partie est la traînée des ailes dans le souffle, que le modèle calcule aussi : petit double compte en montée, à juger en revue.

**J = 0** : extrapolation linéaire des points J = 0,10 et 0,15 (C_T 0,1335 effectif, C_P 0,0650).

**Points d'appui des quatre quadrants** : estimation par éléments de pale de l'aérodynamicien (revue du 2026-10-22), avec contrôle d'énergie : arrêt (−0,065 ; −0,008), écoulement inversé (+0,07 ; +0,008), rotation inverse (−0,04 ; −0,007).

**Contrôle attendu** : à 192 km/h et 1 720 tr/min à 2 000 m, J = 53,3 / (28,67 × 2,45) ≈ 0,76, au voisinage du rendement maximal. Le modèle doit retrouver vitesse **et** régime (S46).

## 3. Géométrie mesurée sur le dessin de 1917

**Document** : dessin trois vues « The type VII 160 HP Hispano-Suiza motored Spad Scout », *Flight*, 16 août 1917, p. 841 (reprise d'*Aerial Age*, S48 de `spad7.md`), PDF d'archive.org (`Flight_International_Magazine_1917-08-16-pdf`, page 15) rendu à 600 points par pouce. L'article qualifie ses cotes d'« approximatives » ; le dessin a les mêmes proportions que son texte (corde de 4 ft 7 in pour 25 ft 6 in d'envergure). **Confiance C pour les proportions, D pour la corde.**

**Mise à l'échelle** : la barre d'échelle est illisible à cette définition. L'échelle vient des cotes officielles de S46 : envergure de 7,83 m en vue de dessus (1 885 px) et longueur hors tout de 6,10 m en vue de côté. Les deux vues donnent **240,7 et 240 px/m** : elles concordent à 0,3 %. Mesures faites sur une grille de 20 px (±10 px, soit ±4 cm).

| Grandeur | Dessin | Retenu pour les données | Remarque |
|---|---|---|---|
| Corde des plans | 1,37 m (330 px) | **1,21 m (C)** | Le dessin reprend la corde de S48, incompatible avec la surface officielle (A). Avec 1,21 m, plan supérieur sur toute l'envergure et plan inférieur depuis le flanc du fuselage (0,40 m de l'axe), la surface des bandes vaut 17,96 m², à +0,6 % des 17,85 m² ; c'est le rapport 0,94 entre surface publiée et rectangle brut du XIII (aérodynamicien). **Conflit à trancher en revue** |
| Écartement | 1,26 m (302 px) | 1,27 m (S48, C) | Concordant |
| Décalage horizontal | ≈ 0 : mâts verticaux, bords d'attaque alignés | 0 (C) | XIII : 1¼°, soit 3 cm |
| Mâts interplans | avant à 17 %, arrière à 49 % de la corde | positions des traînées | |
| Ailerons (plan supérieur seul) | 1,95 m d'envergure depuis le saumon, 0,44 m de corde (32 %) | de 50 % à 100 % de la demi-envergure, 32 % de la corde (C) | XIII : 0,495 m sur 1,40 m (35 %, S15 vu en résumé) |
| Plan fixe et profondeur | envergure 2,81 m ; surface 2,30 m² dont profondeur 1,00 m² (44 %) ; corde à l'emplanture 1,14 m ; bord d'attaque à 3,78 m derrière celui des ailes | panneau trapézoïdal de même surface : cordes 1,14 et 0,50 m, quart de corde de x = −4,07 à −4,51 m | Contours numérisés point par point, surface par la formule du polygone |
| Dérive et gouvernail | dérive 0,37 m², gouvernail 0,53 m² (59 %) ; hauteur totale 1,09 m ; le gouvernail descend jusqu'au bas du fuselage ; la dérive avance de 0,2 à 0,27 m devant le plan fixe | deux panneaux : sous le plan fixe (gouvernail seul, 0,39 m de haut, corde 0,53 m), au-dessus (0,70 m de haut, cordes 1,61 et 0,39 m) | S48 : la dérive avance d'environ 12 in |
| Plan fixe : hauteur | sur les longerons supérieurs, 0,70 m au-dessus de la corde du plan inférieur | z = −0,70 m | |
| Fuselage | hauteur maximale ≈ 1,1 à 1,17 m ; ouverture circulaire du radiateur 0,71 m ; largeur peu lisible (renflements des carters d'arbres à cames) | 0,80 m de large, 1,10 m de haut (E:) | |
| Hélice | moyeu 0,78 m devant le bord d'attaque, 0,30 m au-dessus de la corde du plan inférieur ; diamètre dessiné 2,29 m | moyeu (0,78 ; 0 ; −0,30) ; diamètre 2,45 m (§ 2) | |
| Roues | diamètre dessiné 0,58 m, axe 0,12 m devant le bord d'attaque, 0,78 m sous la corde | positions du train | Dimension « 700 × … » lue sur le S.XI par l'historien |
| Poste | ouverture de 0,9 à 1,4 m derrière le bord d'attaque | pilote à x = −1,15 m | |

**Repère des données** : origine au bord d'attaque du plan inférieur, dans l'axe (point virtuel, dans le fuselage) ; x vers l'avant, y à droite, z vers le bas ; ligne de traction horizontale (le dessin est tracé en ligne de vol, queue haute).

## 4. Masses, centrage et chargement d'essai

**Bilan à vide** (`assets/aircraft/spad7/masses.ron`, calculé par `fdm_lab inertia spad7`) : aucune masse de composant du S.VII n'est sourcée (`spad7.md` § 3). Dix composants comparables ou estimés, fermés sur la masse à vide de S46 (1 100 lb = 498,95 kg) par le reste du fuselage :

| Composant | Masse | Origine |
|---|---|---|
| Moteur 8Aa à sec | 447 lb (202,8 kg) | moyenne des Hispano de 150 hp de H19 (440 et 455 lb) et de H20 (445 lb) |
| Radiateur et eau | 40 kg | E: |
| Hélice et moyeu | 22 kg | E: (20 à 25 kg) |
| Plans supérieur et inférieur | 58 + 52 kg | 22 % de la masse à vide comme le S.XI (S49), au prorata des surfaces |
| Mâts, câbles, renvois | 12 kg | E: |
| Empennage | 15 kg | E: (19,9 kg pour les seules gouvernes du S.XI, plus grand) |
| Train | 25 kg | E: |
| Réservoir et circuits vides | 10 kg | E: |
| Fuselage, capots, poste, commandes | 62,4 kg | fermeture sur 1 100 lb |

Résultat : 499,2 kg, centre de gravité à vide à x = −0,313 m (26 % de la corde depuis le bord d'attaque) et z = −0,406 m ; inertie au centre de gravité à vide (785 ; 778 ; 1 360) kg·m², ixz = 40 kg·m². L'inertie n'a aucun recoupement mesuré : le NACA TR 467 (rayons de giration mesurés d'avions de l'époque), proposé par l'aérodynamicien, reste à lire.

**Chargement des essais de S46** (charge totale 451 lb, lue sur l'image, `spad7.md` § 3) : pilote 176 lb ; armement 88 lb ; divers 11 lb ; essence et huile 176 lb. L'huile est mise à part (8 kg, E: environ 2 h à 2,8 L/h de H20 plus le contenu du circuit) ; il reste 71,8 kg d'essence, soit 99,7 L à 0,72 kg/L, c'est-à-dire **74 % des 135 L** du réservoir (contenance C, S13 vue en résumé). Masse totale : 703,5 kg (S46 : 1 551 lb).
