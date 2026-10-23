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
