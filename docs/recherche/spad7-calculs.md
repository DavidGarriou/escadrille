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

**Comparable** : hélice en bois Navy D' (P/D 0,8, 2,39 m) mesurée sur le VE-7 complet, NACA Report 301, tableau III (`hispano-suiza-8a.md` § 7.4). Les coefficients, sans dimension, sont repris tels quels. **Diamètre** : 2,45 m (Levasseur 485) jusqu'au 2026-10-23, puis **2,40 m, Éclair type 6** (2,40 × 1,95 m, P/D 0,81), choix du joueur le 2026-10-24 : Hartmann présente Éclair (types 6 et 100) comme le fournisseur principal du S.VII (`hispano-suiza-8a.md` § 7.5.10 ; fourchette au § 7). Effet P et inertie du rotor suivent le diamètre (1,08 m ; 6,7 kg·m²).

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

## 5. Valeurs aérodynamiques (aérodynamicien, 2026-10-22)

Calculées par l'aérodynamicien sur la géométrie du § 3 et contrôlées avec le modèle lui-même, **sans réglage** (12 campagnes d'essai dans un dossier de travail). Origine détaillée dans les commentaires de `aircraft.ron`.

- **Traînées parasites**, élément par élément (Hoerner) : radiateur frontal 26 %, câbles 24 %, train et roues 12 %, frottement du fuselage 9 %, renvois d'ailerons 7 %, poste 7 %, mâts 6 %, divers 9 %. Somme en écoulement libre : 0,687 m².
  - **Câbles ronds** toronnés : rien n'atteste des câbles profilés sur le S.VII. Variantes : doublés à lattes (type S.XI) 0,124 m² au lieu de 0,165 ; profilés 0,059 m².
  - **Pas de facteur 3/5** de S43 : il a été mesuré sans hélice, et Relf et Lavender (B.E.2c, ACA 1917-18 vol. II) trouvent les traînées additives.
  - **Contrôle** : en palier à 192 km/h et 2 000 m, le modèle complet a un f équivalent de 0,91 m², profils compris, contre 0,94 à 0,98 m² déduits de S46. L'écart, de 3 à 7 %, reste dans l'incertitude de cette cible.
- **Fuselage en corps élancé** :
  - 14 stations relevées sur le dessin : longueur 5,14 m, volume 2,93 m³, finesse 4,87 ;
  - volume « potentiel » de 0,95 m³ par la méthode de Multhopp (DATCOM), k₂ − k₁ = 0,83 (Lamb) ;
  - aires de dessus 3,30 m² et de côté 4,50 m², η C_dc = 0,75.
- **Déflexion** :
  - `factor` 0,125 : deux nappes planes dans le plan de Trefftz ; monoplan équivalent, 0,122 ;
  - bras de 3,96 m.
- **Allongements** :
  - ailes, e = 0,92. C'est δ du TR 416, préféré à τ parce que les cibles de montée dépendent de la traînée induite ;
  - plan fixe, e = 0,80 (pente de Helmbold) ;
  - dérive, allongement effectif 1,5 (plaque partielle, Hemke) et e = 0,61 (pente DATCOM de 1,9/rad).
- **Empennages sans flèche** : le chargeur oriente les bandes selon la ligne du quart de corde ; une ligne en flèche aurait gonflé la dérive de 0,90 à 1,51 m² et fait décrocher la dérive trop tôt. Les cordes du dessin sont reportées sur une ligne droite, au quart de la corde moyenne.
- **Profil d'empennage** (`assets/airfoils/spad_empennage.ron`) : RAF 15 sans cambrure, maximum plafonné à C_L 0,85 (profil symétrique mince), branche négative en miroir.
- **Hélice** :
  - effet P de 1,1 m, par éléments de pale sur la géométrie de D' (Report 301, fig. 5) au J de montée ;
  - force normale de 0,038, pour J de 0,6 à 0,76, plus 10 % de remontée devant les ailes.
- **Moteur** :
  - ralenti 0,045 (environ 500 tr/min au point fixe), calage à 200 tr/min ;
  - **loi d'altitude `Pressure(0.93)`** : R&M 462 mesure une puissance **au frein** en (p/p₀)^1,04. Le modèle applique l'exposant à la puissance indiquée, puis retranche le frottement. Converti avec le frottement du Report 46, l'exposant vaut 0,93 (à ±0,6 % de 1 000 à 5 000 m, vérifié ici). Avec 1,04, le frottement aurait été compté deux fois : la documentation est corrigée.
- **Débattements** : ±20°, ±25° et ±30° (E:, aucune source SPAD).
- **Calages** : 2,3° (plan supérieur) et 1,8° (plan inférieur), d'après le critère « fuselage horizontal à 170 km/h et 2 000 m ». Vérifié par le trim : incidence du fuselage 0,015°.
- **Hauteur du moyeu** : 0,38 m au-dessus du bord d'attaque inférieur, d'après la relecture du dessin par l'aérodynamicien (0,36 à 0,41 m ; j'avais lu 0,30). Le moteur, le radiateur et l'hélice du bilan de masses ont suivi.

## 6. Traînée des ailes entoilées, calée sur le Sperry Messenger (2026-10-23)

Étape 3 du plan `docs/plans/m1-ecarts-spad7.md`. Le tableau du RAF 15 (Report 233) a été mesuré sur une maquette **polie** ; le champ `fabric_drag_rise` de `spad.ron` ajoute la traînée de l'aile **telle qu'on la construisait**, ΔC_d = k C_l².

**Avion de comparaison.** Le Sperry Messenger à ailes RAF 15 du NACA Report 304 : polaire de l'avion complet en vraie grandeur, par vols planés à traction nulle.
- Ajustement des 21 vols de C_L 0,23 à 0,81 : C_D = 0,055 + **0,120 ± 0,004** C_L² (écart type d'une mesure 0,003).
- Modèle `tools/fdm_lab/comparison/aircraft/messenger_raf15/`, construit sur les mêmes tables que le SPAD (RAF 15 sans recalage de cambrure, empennage du SPAD), avec les mêmes méthodes (e = 0,92, σ de Prandtl, déflexion) :
  - géométrie du Report 304 (texte p. 4 et fig. 2, lue sur l'image) : envergure 20 ft, corde 4 ft (148,5 sq ft au total), entreplan 3 ft 10 in, dièdre 1,5°, calage +2°, plan fixe à +1,5° ;
  - relevés sur la figure 2 : décalage d'environ 16 in, empennage de 6 ft 8 in × 3 ft 7 in (2,12 m²), bord d'attaque 3,16 m derrière celui du plan supérieur, centre de gravité à 29 % de la corde moyenne ;
  - masse 1 008 lb, moyenne du tableau II ;
  - moteur, hélice, inerties et débattements : `E:`. Ils ne jouent pas sur la polaire.

**Mesure** (`fdm_lab`, planés au ralenti au niveau de la mer, forces de l'hélice retranchées, C_L 0,2 à 0,8) :

| | C_D0 | K | Oswald |
|---|---|---|---|
| Messenger mesuré (Report 304) | 0,055 | 0,120 | 0,734 |
| Messenger modélisé, table polie | 0,054 | 0,0997 | 0,889 |
| Messenger modélisé, k = 0,020 | 0,054 | 0,1201 | 0,738 |
| SPAD modélisé, table polie | 0,047 | 0,0801 | 0,899 |
| SPAD modélisé, k = 0,020 | 0,047 | 0,0995 | 0,723 |

K croît de 1,02 par unité de k : **k = (0,120 − 0,0997) / 1,02 = 0,020**.

**Sensibilité** du calage aux valeurs estimées du Messenger :

| Variante | k nécessaire |
|---|---|
| référence | 0,0199 |
| centre de gravité 0,10 m plus en avant / en arrière | 0,0167 / 0,0216 |
| e des ailes 0,95 au lieu de 0,92 | 0,0219 |
| σ 0,47 au lieu de 0,50 | 0,0217 |
| empennage (e 0,60), déflexion (0,12), fuselage (volume × 2), hélice (D 1,8 m) | 0,0198 à 0,0201 |

Avec l'incertitude de K mesuré (±0,004, soit ±0,004 sur k) : **k = 0,020 ± 0,005**, confiance C (un autre avion, une autre époque, des ailes construites de la même façon).

**Ce que k recouvre.** Surtout la traînée de profil des ailes réelles : toile, nervures, rugosité, à des nombres de Reynolds voisins de ceux du Report 233. Mais aussi, faute de mieux, tout ce que le modèle de biplan ne voit pas de la traînée liée à la portance (interférence réelle des plans, mâts, fuselage). Sur une aile, à C_l 0,6, k ajoute 0,007 à la traînée de section, qui passe d'environ 0,0085 (table ramenée en 2D) à 0,016 : presque le double, plus que la seule toile ne l'explique sans doute, d'où la confiance C. L'empennage n'en reçoit pas (portance faible ; même choix sur les deux avions).

**Effet sur le SPAD** (campagne d'essais du 2026-10-23) : montées de −15 / −16 / −22 % à −9 / −8 / −12 % de la cible ; plafond de +19 % à +8 % ; vitesse maximale à 2 000 m de +2,9 % à +1,7 %, à 3 000 m +5,6 %, à 4 000 m +4,5 %. Finesse maximale de 8,3 à 7,4.

**Revue de l'aérodynamicien (2026-10-23) et vérifications.**
- Méthode jugée saine : tables et méthodes identiques sur les deux avions, si bien que les erreurs de méthode s'annulent au premier ordre. Le plané au ralenti équivaut à la traction nulle du Report 304 (hélice en moulinet, forces retranchées). Ajusté sur les 21 C_L du Report 304 au lieu de la grille de `fdm_lab`, k ne bouge pas.
- **Pente de portance** (vérification demandée par la revue) :
  - Messenger mesuré (tableau II, 19 vols de C_L 0,2 à 0,81) : 0,0533 ± 0,0017 par degré, portance nulle à −3,3° ; jusqu'à C_L 0,7 : 0,057 par degré, −2,9° ;
  - Messenger modélisé : 0,0657 par degré, −2,0° ; k ne la change pas ;
  - le modèle est donc 16 à 23 % trop raide, bien au-delà des ±3 à 4 % que le Report 304 donne sur l'incidence.

  Une partie de l'écart tient donc sans doute au rendement d'envergure du biplan réel (σ, e), ou à la pente de section d'une aile entoilée, et non à la seule traînée de profil. Pour les performances, cela ne change rien, puisque k est calé sur la polaire mesurée. Mais la part « induite » varie comme 1/A_eq : si tout l'écart était induit, le SPAD (A_eq 4,42 contre 3,59) recevrait l'équivalent de k ≈ 0,016. C'est dans la marge de ±0,005.

  Piste séparée, ouverte : la pente de portance des biplans du modèle, qui joue sur l'incidence de vol, la déflexion et la réponse aux rafales.
- **Bord d'attaque** : les ailes du Messenger ont du contreplaqué du bord d'attaque au longeron avant (Report 304, p. 4). Si celui du S.VII est entoilé, k serait pour lui une borne basse. Question pour l'historien.
- **Couplage avec les traînées parasites** : k contient la hausse de traînée du corps du Messenger avec l'incidence, pour C_L de 0,2 à 0,8. Si l'on ajoute un jour une loi d'incidence aux éléments de traînée (piste 2), il faudra recaler k.
- **Domaine** : calé jusqu'à C_L 0,8 (bandes jusqu'à C_l 0,9 à 1,0). Au-delà, décrochage et branche négative compris, c'est une extrapolation (E:). Elle ne change ni la vitesse de décrochage (80,6 → 80,7 km/h) ni, sensiblement, les modes.

## 7. Puissance installée (2026-10-23)

Étape 4 du plan `docs/plans/m1-ecarts-spad7.md`. Nouveau champ moteur `installed_power` : fraction de la puissance **indiquée** du banc fournie en vol, le frottement restant entier.

**Source.** NACA Report 301, p. 10 et figures 11 à 13 : avion VE-7, moteur Wright-Hispano (famille du 8Aa). Les références de vol sont le Report 220 (hélices B', D' et I) et le Report 292 (hélice I, dite 3714).
- En vol, les coefficients de puissance, calculés avec l'étalonnage du moteur au banc, dépassent de 3 à 5 % ceux de la soufflerie en vraie grandeur dans trois essais sur quatre.
- Les auteurs en déduisent que le moteur ne fournissait « probablement » pas en vol toute sa puissance de banc. Ils ne donnent pas de cause : ils évoquent seulement l'inclinaison de l'axe de l'hélice en vol, et jugent l'accord « probablement dans la précision des essais en vol ».
- Sur la figure 13, l'hélice D' (le comparable du SPAD, § 2) montre l'écart le plus faible : 0 à 3 % selon J. B' et D' viennent pourtant de la même campagne, sur le même avion. Une perte propre au moteur serait la même pour les deux : une part de l'écart est donc de la dispersion d'essai.

**Double compte avec la loi d'altitude** (point soulevé par la revue, vérifié dans les Reports 220 et 292) :
- les deux rapports ramènent la puissance du banc aux conditions de vol par P_frein ∝ p/√T. Le Report 220 a vérifié la dépendance en 1/√T au banc, à pression constante ;
- les vols de puissance ont lieu à 1 500–3 000 ft (Report 220) et vers 3 000 ft (Report 292 : poids spécifique de l'air de 0,068 à 0,071 lb/ft³) ;
- à ces altitudes, en atmosphère type, leur loi donne **0,6 à 1,3 %** de puissance au frein de plus que la loi du modèle (`Pressure(0.93)` sur la puissance indiquée, tirée de R&M 462) : +0,6 % à 457 m, +0,9 % à 686 m, +1,3 % à 914 m ;
- cette part du « déficit » est déjà dans le modèle.

**Valeur.** Même statistique que le plan (milieu des 3 à 5 %, soit 4 %), moins le double compte (1 %) : **−3 % au frein**. À 1 700 tr/min, P₀ = 175 hp et P_f = 16 hp, d'où 1 − 0,03 × 175 / 191 = **0,973**.
- Plage défendable : 0 à 4 % au frein, soit 0,96 à 1,0.
- Confiance C pour la méthode, D pour la valeur (revue de l'aérodynamicien) : un autre avion, une cause non identifiée, des écarts proches de la précision des essais.
- Une première valeur, 0,963, avait été retenue avant la lecture des Reports 220 et 292. **Transparence** : je savais alors qu'une fraction de 0,98 ferait passer le régime. La correction vient des sources, mais elle va dans ce sens ; à garder en tête en relisant ce chiffre.

**Sensibilité** (vitesse maximale, régime à cette vitesse, temps de montée ; 74 % de carburant ; hélice de 2,45 m) :

| Fraction | Vitesse 2 000 / 3 000 / 4 000 m (km/h) | Régime (tr/min) | Montée à 2 000 / 3 000 / 4 000 m (min) |
|---|---|---|---|
| cible S46 | 192,3 / 179,4 / 173,8 | 1 720 / 1 680 / 1 640 | 6,67 / 11,5 / 19,5 |
| 1,00 | 195,5 / 189,4 / 181,7 | 1 693 / 1 657 / 1 616 | 6,05 / 10,55 / 17,10 |
| 0,98 | 192,8 / 186,6 / 178,5 | 1 672 / 1 636 / 1 593 | 6,39 / 11,22 / 18,42 |
| **0,973** | 191,8 / 185,6 / 177,4 | 1 665 / 1 629 / 1 586 | 6,53 / 11,48 / 18,93 |
| 0,96 | 190,1 / 183,7 / 175,3 | 1 651 / 1 615 / 1 571 | 6,78 / 11,98 / 19,96 |

**Le régime et la montée se lisent ensemble** (revue de l'aérodynamicien, mesures par sonde, fraction 0,963) :

| Hélice | Vitesse (km/h) | Régime (tr/min) | Montée (min) |
|---|---|---|---|
| 2,45 m (données) | 190,5 / 184,1 / 175,7 | 1 654 / 1 618 / 1 574 | 6,72 / 11,86 / 19,72 |
| 2,40 m | 192,7 / 187,5 / 179,4 | 1 725 / 1 696 / 1 652 | 6,30 / 11,01 / 17,96 |
| 2,35 m | 193,6 / 188,3 / 181,8 | 1 790 / 1 760 / 1 726 | 5,95 / 10,34 / 16,69 |

- En meilleure montée, le moteur du modèle tourne à 1 490, 1 465 et 1 440 tr/min (J de 0,53 à 0,62).
- Avec une hélice qui absorberait juste ce qu'il faut pour le régime à la vitesse maximale, il tournerait environ 70 tr/min plus vite en montée, avec +5 % de puissance à l'arbre. **Au bon régime, le modèle monterait encore 4 à 8 % trop vite.**
- Le succès actuel des montées repose donc en partie sur un régime trop bas. L'écart de régime n'est pas un détail isolé : c'est la trace d'un écart encore caché sur la montée.

**Incertitudes à garder avec les résultats** :
- k = 0,020 ± 0,005 : environ ±1,5 à ±2,4 points sur les temps de montée ;
- fraction installée, de 0,96 à 0,98 : environ 5 points ;
- les écarts de montée affichés sont dans ces bandes.

**Pour le régime, sans régler sur les cibles** (revue de l'aérodynamicien) :
1. **Historien** : l'hélice réelle du S.VII 150 ch (fabricant, diamètre, pas et sa définition), et tout régime relevé en montée. Un régime de montée suffit à distinguer « l'hélice absorbe trop » de « le moteur donne trop peu ».
2. **Calcul par éléments de pale** sur la géométrie de la D' (Report 301, fig. 5), contrôlé sur son tableau à 2,39 m, puis appliqué aux hélices documentées : Levasseur 2,45 / 1,95 m, Gallia 2,45 / 2,10 m, Éclair 2,35 / 1,75 m (diamètre / pas). C'est le **pas**, plus que le diamètre, qui sépare le régime à la vitesse maximale (J ≈ 0,77) de celui de la montée (J ≈ 0,53).
3. **Postes à faible J du tableau d'hélice** : les +5 à +6,4 % de traction rendus pour la traînée de souffle du VE-7 reposent sur des surfaces estimées (E:). ±50 % sur ces surfaces font environ ±3 % de traction en montée, soit ±5 % de vitesse ascensionnelle. La géométrie du VE-7 est documentée : on peut les passer en C:.
4. Présenter le résultat comme une enveloppe. Si S46 tombe dedans, on rapporte et on s'arrête. L'hélice se choisit par le document, jamais par le régime.

**Fourchette des hélices autorisées** (2026-10-24 ; tableau Hartmann lu, `hispano-suiza-8a.md` § 7.5.10). Pour le moteur ordinaire, trois hélices ont le P/D de la table D' (0,80 à 0,81) ; on peut les simuler en changeant seulement le diamètre. Puissance installée 0,973, k = 0,020, 74 % de carburant :

| Hélice | Diamètre | Vitesse 2 000 / 3 000 / 4 000 m (km/h) | Régime (tr/min) | Montée à 2 000 / 3 000 / 4 000 m (min) | Plafond |
|---|---|---|---|---|---|
| cible S46 | — | 192,3 / 179,4 / 173,8 | 1 720 / 1 680 / 1 640 | 6,67 / 11,5 / 19,5 | 5 486 m |
| Éclair 6 | 2,40 m | 193,8 / 188,7 / 181,0 | 1 734 / 1 706 / 1 664 (+0,8 à +1,5 %) | 6,13 / 10,69 / 17,33 (−8 / −7 / −11 %) | 5 917 m (+7,9 %) |
| Levasseur 485 (modèle actuel) | 2,45 m | 191,8 / 185,6 / 177,4 | 1 665 / 1 629 / 1 585 (−3,0 à −3,3 %) | 6,52 / 11,47 / 18,93 (−2 / 0 / −3 %) | 5 696 m (+3,8 %) |
| Gallia 3 | 2,50 m | 188,6 / 182,2 / 173,7 | 1 590 / 1 555 / 1 512 (−7,4 à −7,8 %) | 7,05 / 12,52 / 21,07 (+6 / +9 / +8 %) | 5 471 m (−0,3 %) |

- **Chaque cible de S46 est dans la fourchette**, mais **aucune hélice ne tient à la fois le régime et la montée**. Au bon régime (Éclair 6), le modèle monte 7 à 11 % trop vite : c'est l'écart caché que la revue annonçait, maintenant chiffré sur une hélice documentée.
- Les hélices de P/D 0,74 à 0,77 (Éclair 100, Chauvière 2066) demandent une autre table (I à P/D 0,70 du Report 301, ou 3713 à 0,76 du Report 292, mesurée en vol). Avec un pas plus faible, elles tourneraient plus vite encore, et monteraient sans doute mieux.
- **Ce qui reste à chercher est donc du côté de la montée**, pas de l'hélice : traction de la table à faible J (surfaces E: du VE-7, § 2) ; puissance du moteur vers 1 450–1 500 tr/min (forme de la courbe du Report 46) ; convention de chronométrage de S46 (depuis le départ des cales ? le S.VII n'avait pas de freins).

**Hélice de référence : Éclair 6** (décision du joueur, 2026-10-24, sur recommandation). Raisons :
- le texte de Hartmann la désigne ;
- les deux erreurs qui se compensaient avec la Levasseur 485 sont séparées ;
- la cause de la montée se cherche avec l'hélice documentée.

Biais déclaré : on savait avant la lecture que 2,40 m donne le bon régime. Levasseur 485 et Éclair 100 restent des variantes.

Campagne avec l'Éclair 6 :
- **régimes réussis** : +0,8 / +1,5 / +1,5 % ;
- vitesses +0,8 / +5,2 / +4,2 % : 3 000 m en échec de peu (B) ;
- **montées −8,0 / −7,1 / −11,1 %**, plafond +7,8 % : en échec.

L'écart de montée, croissant avec l'altitude, est désormais l'objet des recherches.


## 8. Conditions des essais de 1919 : enveloppe des temps de montée (2026-10-24, revu le même jour)

Le modèle chronomètre ses montées depuis 0 m, en vol, à la vitesse de meilleure montée, en atmosphère type. Les conditions de l'essai de S46 sont inconnues. On encadre donc ce qu'elles ajoutent au temps, sans rien changer au modèle.

**Outil** : `fdm_lab montee spad7 --delta-t-k=-15,-10,0,10,15 --airfield-m 178` (`tools/fdm_lab/src/conditions.rs`). Les altitudes sont des altitudes-pression, comme sur un altimètre. La conversion de la vitesse ascensionnelle (w T_type/T) est vérifiée par un test indépendant, sur une colonne d'air intégrée en altitude géométrique (0,2 % près).

**Moteur et température.** La loi `Pressure(n)` ne dépendait pas de la température : c'était la lecture de R&M 462, qui ne trouve « pas d'effet sensible » en vol. Avec cette loi, par jour chaud, l'air plus léger freine moins l'hélice, le moteur tourne plus vite, et le plafond en altitude-pression bouge à peine. Ce n'est pas une absurdité : c'est la conséquence de cette source. D'autres sources mesurent pourtant un effet. La puissance indiquée suit donc maintenant (T_type/T)^m à pression égale, **m étant une donnée par moteur** (0 par défaut, origine obligatoire). Pour le Hispano, **m = 0,4** (C), au milieu de la plage des sources :
- NACA Report 220, p. 7 : puissance au frein d'un Wright-Hispano « closely as 1/√T » au banc, avec de l'air d'admission à 10 et 26 °C, soit environ 0,46 sur la puissance indiquée ;
- L.C.E. 196, cité par R&M 462 : −5 à −7 % pour +19 % de température absolue, soit 0,27 à 0,38 sur la puissance indiquée ;
- cohérence avec n = 0,93 : 0,37, si la charge suit p à température fixe ;
- R&M 462 : pas d'effet sensible avec 20 °C d'écart en vol ; écarte m = 1, sans départager 0 et 0,5.

La loi `Density` revient à m = 1, que les sources rejettent. En atmosphère type, le facteur vaut 1 : la campagne habituelle est inchangée au chiffre près.

**Biais déclaré** : une première version imposait m = 0,5 à tous les moteurs, dans le commit même qui concluait que « ISA +10 K suffit ». Ce choix allait dans le sens de la conclusion. Avec m = 0, cette conclusion était fausse : à ISA +10 K, depuis 0 m, −6,4 / −5,5 / −9,6 %, contre −2,1 / −0,5 / −3,8 % avec m = 0,5 et −3,0 / −1,5 / −5,0 % avec m = 0,4.

**Enveloppe** (Éclair 6, m = 0,4, 74 % de carburant ; écart aux cibles 6:40 / 11:30 / 19:30 et 5 486 m ; terrain de Villacoublay, 178 m) :

| Jour | Depuis 0 m | Terrain, altimètre sur la mer | Terrain, altimètre à zéro au sol | Plafond (0 m / zéro au sol) |
|---|---|---|---|---|
| ISA −15 K | −15 / −15 / −20 % | −21 / −19 / −22 % | −12 / −11 / −15 % | +10,7 / +7,5 % |
| ISA −10 K | −13 / −12 / −17 % | −19 / −16 / −19 % | −9 / −8 / −12 % | +9,8 / +6,5 % |
| ISA | −8,0 / −7,1 / −11,1 % | −15 / −11 / −13 % | −4,0 / −2,3 / −5,4 % | +7,8 / +4,6 % |
| ISA +10 K | −3,0 / −1,5 / −5,0 % | −10 / −6 / −7 % | +1,3 / +3,6 / +1,4 % | +5,9 / +2,6 % |
| ISA +15 K | −0,5 / +1,3 / −1,8 % | −8 / −3 / −4 % | +4,1 / +6,7 / +5,0 % | +4,8 / +1,6 % |

ISA +20 K jusqu'à 4 000 m sort de la climatologie : la moyenne estivale en altitude est plutôt de +5 à +7 K. Cette ligne est retirée.

**Dimensions qui manquent au tableau, du même ordre** (revue de l'aérodynamicien) :
- **loi de graduation des instruments d'époque**, isotherme ou de Radau : « 4 000 m » lus valent de 3 825 à 4 025 m en atmosphère type selon la température de graduation, soit de +1 à −7 points sur le temps à 4 000 m ;
- **pression au sol du jour** : ±10 hPa font environ ±2 points à 2 000 m ;
- **réduction éventuelle à une atmosphère type** : si les chiffres de 1919 étaient réduits, l'hypothèse du jour chaud tombe. L'atmosphère type du STAé date de 1920 (TN 20), mais une réduction par une méthode inconnue n'est pas exclue : question ouverte ;
- **biais des essais réels**, tous dans le sens « l'avion réel monte moins bien » :
  - montée en spirale au-dessus du terrain : −4 % à 20° d'inclinaison, −9 % à 30° ;
  - vitesse unique au lieu de l'optimum à chaque altitude : environ −1 % ;
  - terme d'énergie cinétique, absent de l'intégration : +0,5 à +0,75 % sur les temps.

**Décollage** (estimation hors outil, C/E ; le modèle de sol viendra au M2) :
- au point fixe, 1 593 tr/min et 4,0 kN de traction (T/W 0,58 ; figure de mérite environ 0,65), comparables aux 1 640 tr/min de la D' au point fixe sur le VE-7 ;
- roulage, queue haute, jusqu'à 89 km/h (1,1 V_s) : 6,3 à 7,0 s et environ 88 m, pour un frottement de 0,05 à 0,10 ;
- accélération jusqu'à la vitesse de meilleure montée (116 km/h) : 3,4 s ;
- à ajouter : la mise en ligne de vol, béquille au sol (+0,3 à 0,6 s), et l'ouverture progressive des gaz (+1 à 2 s) ;
- **10 à 15 s en tout**, soit +2,5 à +3,7 % à 2 000 m, si le chronomètre partait des cales.

**Deux indices contre une explication par les seules conditions** :
- **les vitesses du même tableau tirent en sens inverse** (m = 0,4) : à ISA +10 K, vitesses +1,3 / +5,7 / +4,8 % et régimes +1,5 / +2,2 / +2,3 % ; à ISA −10 K, vitesses +0,2 / +4,5 / +3,5 %. Les vitesses demandent un jour froid, les montées un jour chaud ;
- **la tranche de 3 000 à 4 000 m** : en atmosphère type, −8,0 % de 0 à 2 000 m, −5,6 % de 2 000 à 3 000 m, mais −17 % de 3 000 à 4 000 m. À ISA +10 K (m = 0,4), cette dernière tranche reste à −10,0 %. Départ, terrain et décollage ne touchent que la première tranche. L'arrondi des temps de S46 vaut environ ±6 % sur la dernière tranche : c'est un indice, pas une preuve. La piste de la loi d'altitude reste ouverte (richesse du carburateur Zenith sans correcteur, surchauffe vers 4 000 m selon Hall).

**Lecture** (reformulée d'après la revue). Les conditions inconnues de l'essai déplacent les temps d'environ −22 à +7 % (+8 % avec le décollage), sur ±15 K : température, terrain, zéro et graduation de l'altimètre, réduction éventuelle à une atmosphère type, départ du chronomètre, trajectoire. S46 ne départage donc pas un écart de ±10 % : l'écart actuel n'est **ni la preuve d'un défaut, ni une validation**. Deux indices vont contre une explication par les seules conditions : les vitesses du même tableau, et la tranche de 3 000 à 4 000 m. Les tolérances ne bougent pas.

**Après la dixième passe de l'historien** (`spad7.md` § 5.7) et la lecture des documents qu'il désigne, téléchargés par le coordinateur :
- **départ du chronomètre français : inconnu.** La pratique britannique de 1917 (Tizard, *Flight*, mars 1917) ramène les temps à une montée en vol depuis la hauteur type 0. Un temps britannique corrigé est donc déjà dans la convention du modèle ;
- **altimètres** : le NACA Report 126 (1922, p. 3-5) [LU-OCR] dit que l'échelle des altimètres tourne « pour faire le zéro au départ du vol », et que les altimètres français sont gradués selon la loi de Radau. L'altimètre calé à zéro sur le terrain était donc l'usage courant (C), sans preuve pour l'essai de S46 ;
- **terrain** : Villacoublay, 178 m (585 ft).

**La comparaison qui reste discriminante : le rapport britannique n° 96** (avril 1917 ; chiffres vus en résumé seulement, `spad7.md` S11 ; temps sans doute ramenés à la convention du modèle) :

| Altitude | Rapport n° 96 | Modèle (atmosphère type, depuis 0 m) | Écart |
|---|---|---|---|
| 6 500 ft (1 981 m) | 6,3 min | 6,06 min | −3,8 % |
| 10 000 ft (3 048 m) | 11,5 min | 10,95 min | −4,8 % |
| 15 000 ft (4 572 m) | 24,5 min | 23,04 min | −5,9 % |
| Plafond estimé | 19 000 ft (5 791 m), « probablement un peu faible » | 5 917 m | +2,2 % |

**Bilan.** Avec l'Éclair 6, le modèle monte probablement **4 à 6 % trop vite**. Ce reste est du même ordre que :
- les incertitudes connues : masse de l'avion britannique inconnue ; atmosphère britannique de 1917 un peu plus dense sous 2 000 m ; k = 0,020 ± 0,005 ; puissance installée de 0,96 à 1,0 ;
- et la piste « traction à faible J » (surfaces E: du VE-7, ±5 % de vitesse ascensionnelle).

Le rapport n° 96 lui-même (Kew, AIR 1/1196/204/5/2600) fixerait sa convention et sa masse.

## 9. Décrochage : le sens du départ n'est pas significatif (2026-10-24)

Revue de l'aérodynamicien, essais avec un facteur sur le tourbillon K du souffle (copie hors dépôt) :

| Facteur sur K | SPAD, moteur réduit | Biplan d'essai |
|---|---|---|
| 1 (modèle) | à droite, +78°, nez −21° | à droite, +62°, nez qui monte |
| 0,5 | à gauche, −54°, nez −15° | abattée de 5°, inclinaison au plus 3° |
| 0 | à gauche, −52°, nez −13° | à gauche, −55°, nez qui monte |

- Le départ en roulis existe **sans aucun tourbillon** : c'est la dynamique propre du biplan au décrochage (autorotation, couplage inertiel p·r).
- Le tourbillon ne fait qu'en choisir le **sens**, contre la réaction du couple.
- Or la théorie des bandes surestime son effet sur des ailes dans un souffle étroit : le moment de roulis calculé, environ 40 N·m, dépasse le couple de l'hélice (24,5 N·m). ~~Référence : Smelt et Davies, R&M 1788, 1937.~~ Référence précisée le 2026-10-24 : R&M 1788 traite la hausse de portance, pas le tourbillon. Pour le tourbillon, la mesure est celle du NACA TN 1146 : l'aile et le fuselage reprennent 40 à 50 % du couple. La ligne portante de R&M 2368 (partie II) ne réduit pas, elle, le moment d'une aile infinie. Voir `docs/plans/m1-souffle-ailes.md`.
- **Le côté du départ (« à droite ») n'est donc pas significatif** tant que le souffle sur les ailes n'est pas étalonné. Seuls comptent la présence du départ et l'abattée.
- Cet étalonnage passe en tête des tâches hors plan : avant toute conclusion sur le décrochage ou les effets de couple, et avant l'embardée au décollage du M2.

