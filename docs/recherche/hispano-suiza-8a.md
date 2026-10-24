# Hispano-Suiza 8A (8Aa, 8Ab) et hélice du SPAD VII — fiche de recherche

- Identifiant : `hispano-suiza-8a`
- Dernière mise à jour : 2026-10-23
- État : **en cours**.
  - Troisième passe : courbe puissance/régime, loi d'altitude et consommation du 180 hp sourcées par des rapports NACA lus ; hélice d'époque comparable mesurée (VE-7).
  - **Quatrième passe** : la contradiction sur la cote du 8Ab est **expliquée** par plusieurs conventions de cote (§ 3.2). Une **loi d'altitude mesurée en vol** sur un Hispano de 150 hp sans réducteur est trouvée (§ 4.3). L'hélice réelle du SPAD et le sens de rotation restent sans source primaire.
  - **Cinquième passe (2026-10-21)** : tableaux des moteurs et des avions relus **sur l'image**. Puissance maximale autorisée sur le S.VII : 175 hp à 1 700 tr/min (8Aa), 205 hp à 1 800 tr/min (8Ab) ; régimes en palier ; consommation, masses et couples (§ 3.2, 4.4, 5, 6, 7.4).
  - **Sixième passe (2026-10-23), hélice du S.VII 150 ch** (§ 7.5) : liste de sept hélices autorisées pour le « Type VII · Hispano 150 ch », avec diamètre et pas (vue en résumé seulement) ; cotes britanniques AB 723, AB 759 et AB 764 précisées ; deux rapports NACA lus qui donnent des hélices de la famille de D' **mesurées en vol** sur VE-7, avec leurs régimes en montée, en palier et au point fixe ; titre exact du R&M 586 établi. La définition française du pas, l'hélice exacte de l'avion de S46 et un régime en montée sur S.VII restent **non trouvés**.
- Avion concerné : [`spad7.md`](spad7.md) (SPAD S.VII C.1)
- Méthode et statuts des sources (**[LU]**, **[LU-OCR]**, **[RÉSUMÉ]**, **[PISTE]**) : voir `spad7.md` § 0. Première et deuxième passes : seules des pages GitHub ont pu être lues, le reste n'a été vu qu'en résumé de moteur de recherche. Troisième passe : rapports NACA téléchargés par le coordinateur (PDF et texte extrait) et **lus** (§ 3.1, 4.1, 5, 7.4), chiffres douteux de l'OCR vérifiés sur l'image des pages.

## 1. Identification et variantes

| Variante | Puissance nominale | Régime | Particularités | Montage | Sources |
|---|---|---|---|---|---|
| 8A (premier type) | 140 hp | 1 900 tr/min | Présenté au ministère de la Guerre en février 1915. Endurance de 15 h à pleine puissance à Chalais-Meudon, puis essai de **50 h** sans interruption réussi le 21 juillet 1915 | — | H1, H2 [RÉSUMÉ] |
| **8Aa** | **150 ch** | 2 000 tr/min (H1) ; 1 700 tr/min (H1, fiche technique d'un miroir) | Taux de compression **4,7** | SPAD S.VII « 150 ch » | H1, H2 [RÉSUMÉ] |
| **8Ab** | **180 ch** | 2 100 tr/min (H1, H5) ; 1 800 tr/min (H4) ; ligne française de H19 : 208 hp à 2 000 tr/min en régime normal, 225 hp à 2 100 tr/min au maximum (p. 201) [LU-IMG] ; puissance maximale autorisée sur le S.VII : 205 hp à 1 800 tr/min (`spad7.md` S46, p. 205) [LU-IMG] | Taux de compression porté à **5,3**. Remplace le 8Aa sur les S.VII à partir du début de 1917. Carburateur Zenith (H5). **1 533** construits en 1917-1918, hors licences (H5) | SPAD S.VII « 180 ch » | H1, H4, H5 [RÉSUMÉ] |
| 8B, 8Ba, 8Be (à réducteur) | 200 à 220 ch | — | Réducteur qui **inverse le sens de rotation** de l'hélice (§ 7.3) | SPAD XIII | H1, H12 [RÉSUMÉ] ; S38 de `spad7.md` (notice du XIII « 200 HP ») |
| 8C, 8Cb (à réducteur, moteur-canon) | — | — | Canon tirant à travers l'arbre d'hélice | SPAD XII | H1 [RÉSUMÉ] |
| Wright-Martin « A » (licence américaine) | 150 hp | **1 450 tr/min** | Puissance américaine, autre définition : **ne pas confondre** avec le 8Aa | — | H3 [RÉSUMÉ] ; confirmé par H19 : 150 hp « normaux » à 1 450 tr/min, 170 hp au maximum à 1 700 tr/min [LU-OCR] |
| Wright-Hispano E (licence américaine) | 180 hp | 1 700 tr/min en régime normal ; maximum **205** hp à 1 800 tr/min (H19, p. 201 [LU-IMG] ; « 203 » était une erreur d'OCR) | Moteur du S.VII de Tallman (N4727V). Identification de la ligne « Hispano-Suiza 180 » de H19 avec le modèle E : **déduite** (C) | — | `spad7.md` S31 [RÉSUMÉ] ; H19 [LU-OCR] |
| Hispano « type V » de 160 hp | 160 hp vers 1 500 tr/min | — | Moteur annoncé pour le S.VII par un article américain de 1917 ; cotes et masse identiques à celles du moteur américain de 150 hp (§ 6) | — | H20 [LU-OCR] |

Licence britannique : Wolseley à Birmingham (H1 [RÉSUMÉ]). Son dérivé à entraînement direct, le Viper, a équipé le S.E.5a (H13 [RÉSUMÉ]).

**Unités.** Les puissances françaises sont en **chevaux-vapeur métriques** (ch, PS). H5 donne pour le 8Ab 132,4 kW à 2 100 tr/min. Or 180 ch × 0,7355 = 132,4 kW (C), soit 177,5 hp impériaux (C). Dans `aircraft.ron`, écrire `Ps(150.0)` et `Ps(180.0)`, pas `Hp(..)`. Les valeurs américaines (H3) sont en hp impériaux : 150 hp = 112 kW (C), ce que donne H3.

## 2. Caractéristiques

| Grandeur | Valeur | Unité d'origine | Source | Confiance | Remarque |
|---|---|---|---|---|---|
| Architecture | V8, refroidissement liquide | — | H3 [RÉSUMÉ] | B | |
| Alésage × course | 120 × 130 mm | mm | H1, H2, H3 [RÉSUMÉ] | B | Trois sources concordantes |
| Cylindrée | 11,76 L (11 760 cm³) | L | H1, H2 [RÉSUMÉ] ; 11,78 L (H3) | B | Contrôle (C) : π/4 × 1,2² × 1,3 × 8 = 11,76 L |
| Taux de compression | 4,7 (8Aa) ; 5,3 (8Ab) | — | H1, H2, H5 [RÉSUMÉ] ; H19, p. 201 [LU-IMG] : 4,8 pour les Hispano de 150 hp américain et britannique, 5,3 pour tous les 180 | B | Écart de 0,1 sur le 150 : non tranché, sans effet notable pour le modèle |
| Carburant | « indice d'octane 80 » | — | H5 [RÉSUMÉ] | **D** | L'indice d'octane n'existait pas en 1917 : mention suspecte |

## 3. Puissance en fonction du régime

### 3.1 Mesure d'époque : NACA Report 46 (V. R. Gage, Bureau of Standards ; source H14) [LU]
Le moteur essayé est décrit page 527 :
- Hispano-Suiza de série « coté **180 hp à 1 700 tr/min** » ;
- 8 cylindres de **4,72 × 5,12 in** (120 × 130 mm) en V à 90° ;
- **taux de compression de 5,3** ;
- carburateur **Claudel**.

Ce sont exactement l'alésage, la course et la compression du **8Ab** (§ 1-2). Le constructeur (français ou licence américaine) n'est **pas** précisé.

Conditions d'essai :
- caisson d'altitude, pleins gaz ;
- de 1 300 à 2 100 tr/min ;
- pressions correspondant à environ 5 500, 11 500, 19 200 et 29 500 ft ;
- **air maintenu vers 0 °F (−18 °C)** à toutes les « altitudes » ;
- mélange réglé pour la puissance maximale, côté pauvre ;
- 390 essais moyennés.

**Tableau 1 (p. 550)**, puissance au frein en hp, valeurs lissées. Les étiquettes de lignes sont confirmées par le produit rendement mécanique × puissance indiquée du même tableau.

| tr/min | Sol | 5 000 ft | 10 000 ft | 15 000 ft | 20 000 ft | 25 000 ft | 30 000 ft |
|---|---|---|---|---|---|---|---|
| 1 500 | 179 | 146 | 119 | 96,7 | 78,6 | 63,5 | 50,8 |
| 1 700 | 203 | 165 | 134 | 109 | 89 | 71,6 | 56,2 |
| 1 900 | 215 | 177 | 144 | 117 | 94,6 | 75 | 58,2 |
| 2 100 | 228 | 185,5 | 150 | 123 | 97 | 78 | 59,3 |

Données complémentaires du même tableau :
- **Puissance indiquée au sol** : 164,5 (1 300), 195 (1 500), 218 (1 700), 235 (1 900) et 253 hp (2 100).
- **Puissance de frottement**, à peu près constante quelle que soit l'altitude : environ 13,1 (1 300), 14,7 (1 500), 16 (1 700), 20 (1 900) et 25 hp (2 100).
- **Rendement mécanique au sol** : 0,90 à 0,93.

**Courbe proposée** (A pour le moteur essayé, **B** pour un 8Ab : même définition, constructeur non précisé). Puissance au frein à pleins gaz, au sol, rapportée à 2 100 tr/min :

| tr/min | 1 500 | 1 700 | 1 900 | 2 100 |
|---|---|---|---|---|
| Facteur | 0,785 | 0,890 | 0,943 | 1,000 |

Calcul (C) sur les valeurs du tableau. À 1 300 tr/min, rendement mécanique × puissance indiquée ≈ 151 hp, soit un facteur d'environ 0,66 (C).

**Précautions**
- Les valeurs sont mesurées en **air froid** (−18 °C), donc dense. Contrôle (C) : 203 hp × 255,4 K / 288,15 K ≈ 180 hp à 1 700 tr/min en air standard, si la puissance suit la densité. C'est bien la cote annoncée, mais l'hypothèse est simplificatrice (§ 4).
- **Régime nominal** : 1 700 tr/min dans ce document américain, contre 2 100 tr/min dans les sources secondaires sur le 8Ab français (H1, H5). La puissance continue de croître jusqu'à 2 100 tr/min (tableau ci-dessus). La « cote » dépend donc de la convention : explication au § 3.2 (quatrième passe) et journal.
- Le **8Aa** (compression 4,7) n'a pas de mesure équivalente. La courbe ci-dessus n'est transposable qu'avec un facteur d'échelle de puissance (C).

### 3.2 La cote du 8Ab : plusieurs conventions (quatrième passe, relue sur l'image en cinquième passe) [LU-IMG]

**Source.** Tableau « Complete technical specifications of important American and foreign aeroplane engines », compilé par le service aéronautique américain et reproduit par *Flight* le 13 février 1919, **p. 201** (H19). Le tableau a trois sections :
- une première section de moteurs utilisés aux États-Unis (Curtiss, Liberty, Hispano-Suiza, Le Rhône…) ;
- une deuxième, sans astérisque, qui regroupe des moteurs britanniques (B.R., Beardmore, Siddeley, Rolls-Royce, Sunbeam…) et deux « Hispano » : je l'appelle **section britannique** (attribution déduite, C) ;
- une troisième de moteurs étrangers, où les deux Hispano portent l'astérisque « * French ».

Le tableau donne pour chacun une puissance au frein « normale » et une puissance « maximale », chacune avec son régime, puis la pression moyenne au frein (« M.B.P. ») en lb/in².

**Lignes Hispano** (alésage 120 mm, soit 4,72 in, course 130 mm, soit 5,12 in, rapport 1,08 pour toutes) :

| Ligne (p. 201) | Puissance « normale » | Puissance « maximale » | Compression | Vitesse de piston (ft/min) | Pression moyenne (lb/in²) |
|---|---|---|---|---|---|
| Hispano-Suiza 150 (section américaine) | 150 hp à **1 450** tr/min | 170 hp à 1 700 tr/min | 4,8 | 1 240 | 114 |
| Hispano-Suiza 180 (section américaine) | 180 hp à **1 700** tr/min | **205** hp à 1 800 tr/min | 5,3 | 1 450 | 122 |
| Hispano 150 (section britannique) | 156 hp à 1 500 tr/min | 165 hp à 1 600 tr/min | 4,8 | 1 280 | 114,75 |
| Hispano 180 (section britannique) | **220 hp à 2 000** tr/min | **225 hp à 2 100** tr/min | 5,3 | 1 707 | 121,8 |
| Hispano 180 * (**français**) | **208 hp à 2 000** tr/min | **225 hp à 2 100** tr/min | 5,3 | 1 707 | 121,8 |
| Hispano 200 * (français) | 220 hp à 2 000 tr/min | 215 hp à 2 100 tr/min | 4,8 | 1 707 | 115 |

Les « 203 hp » et « 223 hp » de la quatrième passe étaient des erreurs d'OCR : l'image porte 205 et 225.

**Contrôles** (C, calculs faits ici) :
- **Vitesse de piston** = 2 × course × régime : 1 450 tr/min donnent 1 237 ft/min, 1 500 tr/min 1 280, 1 700 tr/min 1 451 et 2 000 tr/min 1 707. Toutes les lignes tombent juste : **les régimes « normaux » sont validés.**
- **Pression moyenne** = 792 000 × puissance / (cylindrée de 718 in³ × régime), formule du moteur à quatre temps. Elle redonne exactement la puissance « normale » pour le 150 américain (114,1), le 150 britannique (114,7) et le 180 britannique (121,3). Elle **ne colle pas** pour le 180 américain (116,8 calculé contre 122 imprimé) ni pour les deux lignes françaises.
- **Indice d'une inversion entre les deux lignes françaises** : 121,8 correspond à 220 hp à 2 000 tr/min et 115 à 208 hp. Les pressions imprimées vont donc avec des puissances **échangées**. Si l'on échange les puissances « normales » des deux lignes, la ligne 200 * devient cohérente (208 hp normaux, 215 hp maximaux, au lieu de 220 et 215). La ligne 180 * devient alors identique à la ligne britannique (220 et 225 hp, 121,8), comme l'est déjà toute sa ligne de la p. 203. C'est une **hypothèse** (C) : le tableau imprimé dit 208.

**Ce que cela explique.**
- Le moteur du Report 46 (H14), « coté 180 hp à 1 700 tr/min », suit la convention de la section **américaine**. C'était donc **probablement un moteur construit aux États-Unis** (déduction C). Réserve : H14 décrit un carburateur Claudel, alors que H19 attribue un Zenith Duplex 58 D.C. à tous les Hispano de 180 hp (p. 202).
- Les 1 450 tr/min du Wright-Martin A (H3) correspondent à la ligne américaine de 150 hp.
- Les sources françaises secondaires (H1, H5 : 180 ch à 2 100 tr/min) suivent la convention européenne, où le régime normal est de 2 000 tr/min et le maximal de 2 100 tr/min.
- **Il n'y a pas de contradiction sur le moteur**, mais plusieurs façons de le coter. La forme de la courbe de H14 reste valable de 1 300 à 2 100 tr/min.

**La puissance du 8Ab sur le S.VII** (`spad7.md` S46, p. 205, colonne « Maximum Power Permitted ») [LU-IMG] :
- **8Ab : 205 hp à 1 800 tr/min**, la puissance maximale de la section américaine ;
- **8Aa : 175 hp à 1 700 tr/min**, un peu plus que les 170 hp maximaux de la section américaine.

Recoupement (C, calcul fait ici) : avec la forme de H14 (facteur d'environ 0,917 à 1 800 tr/min par interpolation entre 1 700 et 1 900 tr/min), 205 hp à 1 800 tr/min correspondent à **environ 224 hp à 2 100 tr/min**. C'est la puissance maximale de 225 hp des lignes britannique et française de H19. Trois chiffres d'origines différentes (S46, H19, forme de H14) sont donc cohérents entre eux, et tous au-dessus de la cote nominale de « 180 ch ». Le « 180 » est une **classe** de moteur, pas une puissance mesurée.

Remarque (C) : les puissances de H19 à 2 000 et 2 100 tr/min (220 et 225 hp) sont très proches des valeurs **brutes** du Report 46, mesurées en air froid (environ 221 et 228 hp), et non de ses valeurs ramenées en air standard (environ 196 et 202 hp). Je ne sais pas si c'est une coïncidence.

**Régimes en palier du S.VII** (S46, p. 205) [LU-IMG] : 8Ab, 1 870 à 1 710 tr/min de 2 000 à 5 000 m ; 8Aa, 1 720 à 1 640 tr/min de 2 000 à 4 000 m. Le moteur tourne donc en vol au-dessus de son régime de « puissance maximale autorisée », mais bien au-dessous de 2 100 tr/min (`spad7.md` § 4 ; contrôle de l'hélice au § 7.4).

**Tableau de la p. 203, « Performances of important American and foreign aeroplane engines »** [LU-IMG] : consommations, masses, couples et **puissance estimée en altitude** (§ 4.4, § 5, § 6).

**Laboratoire d'altitude du Bureau of Standards.** Un article de H. C. Dickinson et H. G. Boutell sur ce laboratoire, repris par *Flight* le 23 octobre 1919 (p. 1402-1403, H22), indique que les essais en caisson d'altitude, de janvier 1918 à janvier 1919, ont porté sur **plusieurs moteurs Hispano-Suiza de 150 hp** et de 180 hp. Des mesures d'altitude sur le 150 hp existent donc ; le rapport qui les publie n'est pas identifié (§ 9, pistes).

### 3.3 Pistes restantes (inchangées)
- Un essai du département moteurs du Royal Aircraft Establishment, daté du **8 août 1916**, sur un Hispano-Suiza de 150 hp. Sa courbe serait reproduite en **figure 10** de l'**ARC R&M n° 586** (juillet 1921). Le même participant du forum note qu'elle donne une puissance maximale un peu plus faible à 2 000 tr/min, mais plus de puissance et un couple maximal plus élevé vers 1 200 à 1 400 tr/min. La référence de comparaison n'est pas claire dans le résumé (H6 [RÉSUMÉ]).
- R&M 586 doit se trouver dans les archives ARC numérisées par Cranfield (AERADE, `reports.aerade.cranfield.ac.uk`), inaccessibles depuis le conteneur. Une autre voie : le volume annuel *Technical Report of the Aeronautical Research Committee* de 1920-21 ou de 1921-22 sur archive.org (série dont seul le volume `technicalreport1917aero_2` a été repéré). Identifiant à trouver, contenu non lu.
- Un participant du même forum dit utiliser cette courbe dans un modèle de performances du « Spad 7 C1 » à **740 kg**, réglé d'après des données de l'Air Board (H6 [RÉSUMÉ]). Ce n'est pas une source de chiffre, mais une piste vers des données de l'Air Board.

- **Sixième passe (2026-10-23) : titre exact du R&M 586.** Le NACA Report 292 (H25, liste de références, p. 14) [LU-IMG] le cite ainsi : Aerodynamics Staff of the R.A.E., « Report on Various Airscrews Designed for SE-5, with 150 H.P. Ungeared Hispano-Suiza Engine », R. & M. 586, 1921. L'index des R&M publié en 1968 (R&M 3555, archives AERADE de Cranfield) donne le même titre (vu en résumé). Ce rapport porte donc sur **plusieurs hélices essayées sur le même moteur que le 8Aa** (classe 150 hp, sans réducteur), et pas seulement sur la courbe du moteur. C'est le seul document repéré qui puisse dire, sur ce moteur, si un écart de régime vient de l'hélice ou du moteur (§ 7.5.9).

**Conséquence pour le modèle de vol** : la forme de `power_curve` peut désormais être sourcée, pour le 180 ch, par le Report 46 (§ 3.1). Le R&M 586 reste utile pour le 150 ch.

Indice d'usage : dans une lettre du 8 septembre 1916, Guynemer décrit son approche d'un groupe ennemi « à 1800 » au compteur (« at 1800 on the speedometer » dans la traduction anglaise ; S1 de `spad7.md` [LU]). Il s'agit **peut-être** du compte-tours (1 800 tr/min en descente vers l'ennemi). Interprétation non vérifiée, inutilisable comme donnée.

## 4. Comportement en altitude
- **Moteur atmosphérique** : aucune source consultée n'attribue un compresseur aux 8Aa et 8Ab de série. Un « Hispano-Suiza suralimenté de 190 ch » essayé sur un SPAD 7 en 1917 est cité comme un cas particulier (résumé de recherche ; page source probablement S5 ou S12 de `spad7.md`, attribution incertaine). Confiance C.
- **Vocabulaire à surveiller** : une hélice Gallia série 3 porte la marque « Sur » pour un Hispano de 150 ch dit « surcompressé » (H8 [RÉSUMÉ]). En français d'époque, « surcomprimé » désigne en général un **taux de compression augmenté** (comme le 8Ab), pas une suralimentation. La traduction anglaise *supercharged* du forum est donc douteuse. À vérifier.
- **Carburateur** : un participant du forum suggère que les premiers 150 ch n'avaient pas de correction altimétrique du mélange, ce qui ferait varier la montée au-dessus de 2 000 m selon le carburateur monté (H7 [RÉSUMÉ]). Hypothèse non vérifiée.
### 4.1 Mesures d'époque sur la famille Hispano (troisième passe) [LU]

**Report 46** (Gage ; moteur de 180 hp, compression 5,3 ; § 3.1)
- La **puissance indiquée décroît en proportion directe de la densité de l'air** : sur papier logarithmique, c'est une droite de pente 1 (p. 525).
- La **puissance de frottement est à peu près constante** à régime donné, quelle que soit l'altitude (p. 525-526).
- Vers 30 000 ft, la puissance ne croît presque plus au-delà de 1 700 tr/min (p. 526).
- **Réserve de méthode** : la puissance indiquée a été calculée en supposant un frottement constant (p. 525, 527). La loi « indiquée ∝ densité » est donc en partie une hypothèse de calcul, jugée justifiée a posteriori par les auteurs.

**Report 108** (H15 ; Gage, « Some Factors of Airplane Engine Performance » ; essais en caisson d'altitude d'un Liberty et de trois Hispano-Suiza) (p. 649-651 et conclusions)
- À conditions constantes, la puissance indiquée est proportionnelle à la densité de la charge au début de la compression.
- La puissance décroît **un peu moins vite que la densité** quand c'est la température qui augmente, car le remplissage s'améliore avec la température.
- Le couple de frottement varie linéairement avec le régime et avec la densité ; le pompage en représente environ la moitié.
- Le réglage altimétrique des carburateurs de série est **insuffisant au-dessus d'environ 45 cm de mercure (≈ 15 000 ft)**.

**Report 103** (H16 ; Sparrow et White, « Performance of a 300-Horsepower Hispano-Suiza Airplane Engine »). Moteur décrit p. 584 : Hispano-Suiza de 300 hp, V8 à refroidissement liquide, **alésage 140 mm, course 150 mm**, compression 5,3, carburateur Stromberg à correcteur manuel. C'est un autre moteur que le 8A, de plus grande cylindrée ; le rapport ne lui donne pas de désignation de type. Résultats p. 581-582 :
- Au-dessous de 20 000 ft, la puissance au frein suit une **droite en fonction de la densité de l'air au carburateur**.
- À 1 800 tr/min et 0,040 lb/ft³ (contre 0,075 au sol), la puissance au frein vaut **environ 42 %** de celle du sol, et la puissance indiquée **environ 47 %**.
- Le mélange devient trop riche au-delà de 20 000 ft.

**Ce que le Report 103 apporte au 8A** : aucun chiffre directement transposable. La cylindrée, le carburateur et la puissance diffèrent ; seul le taux de compression est le même. Le rapport confirme deux choses : la forme quasi linéaire de la puissance en fonction de la densité, et le fait que la perte relative en altitude **varie d'un moteur Hispano à l'autre** (§ 4.2). Il ne sert que de **borne** : confiance C pour le 8A.

### 4.2 Comparaison avec la relation de Gagg-Farrar (C, calculs faits ici)
La relation du projet, g(σ) = σ − (1 − σ)/7,55 (`.claude/skills/simulation-vol/modele-aero.md`, section « Moteurs »), a exactement la forme qu'on obtient avec une **puissance indiquée proportionnelle à σ et un frottement constant** : g(σ) = σ − k(1 − σ), où k = frottement / puissance au frein au sol. Gagg-Farrar revient à **k = 1/7,55 ≈ 0,132**.

- **Report 46** (Hispano de 180 hp) : k = 14,7/179 ≈ 0,082 (1 500 tr/min), 16/203 ≈ 0,079 (1 700), 20/215 ≈ 0,094 (1 900), 24,8/228 ≈ 0,109 (2 100). Ce moteur **perd moins** que Gagg-Farrar ne le prévoit. Exemple à 1 700 tr/min et σ ≈ 0,48 (rapport des puissances indiquées « 20 000 ft » / sol, 105/218) : mesure 89/203 = 0,438 ; Gagg-Farrar 0,413 (environ −6 %).
- **Report 103** (Hispano de 300 hp, 140 × 150 mm) : à σ ≈ 0,040/0,075 = 0,533, la mesure donne 0,42, contre 0,471 pour Gagg-Farrar. Ce moteur **perd plus** (environ +11 % d'écart).
- **Conclusion** : Gagg-Farrar se situe **entre** les deux moteurs Hispano mesurés. Pour un 8Ab, le Report 46 est le plus pertinent (même cylindrée et même compression).
- **Proposition** (à valider par l'aérodynamicien) : g(σ) = σ − k(1 − σ), avec k ≈ 0,08 à 0,11 selon le régime (Report 46). Confiance **B** pour un 8Ab, **A** pour le moteur essayé. À défaut, conserver Gagg-Farrar (C), en sachant qu'il est un peu pessimiste pour ce moteur, d'environ 5 % à σ = 0,5.
- **Ce qui manque** : l'origine précise de la relation « Gagg-Farrar » citée par la skill. Les rapports lus sont de **V. R. Gage** (1920-1921), à ne pas confondre avec « Gagg ». La référence Gagg et Farrar n'a été ni trouvée ni lue.

### 4.3 Mesure en vol sur un Hispano de 150 hp : ARC R&M 462 (quatrième passe) [LU-OCR]

**Source** (H21) : D. H. Pinsent et H. A. Renwick (Royal Aircraft Establishment), « The Variation of Engine Power with Height », R&M n° 462, mars 1918, dans le *Technical Report of the ACA* 1917-18, vol. II, p. 748-757.

**Méthode** (p. 749-750 et annexe I) :
- l'hélice sert de dynamomètre : à pleins gaz, on relève régime, vitesse indiquée, altitude et température, de 50 à 100 mph ;
- les vols sont répétés par temps chaud et par temps froid, avec jusqu'à 20 °C d'écart à une même altitude ;
- la puissance en altitude est rapportée à la **puissance au banc au même régime**, banc à p = 1 et +16 °C ;
- cinq moteurs essayés, dont un **Hispano-Suiza de 150 hp sans réducteur**, refroidi par eau, sur S.E.5 (trois vols, observations à 3 000, 6 000, 9 000 et 12 000 ft), et un **Hispano de 170 hp à haute compression**, sans réducteur, sur S.E.5a (essais de six hélices, montées partielles jusque vers 15 000 ft).

**Résultats** :
- Pour tous les moteurs, la puissance s'exprime mieux en fonction de la **pression seule** qu'en fonction de la densité : à régime et altitude barométrique donnés, elle ne dépend pas sensiblement de la température de l'air (p. 748, conclusions). L'hypothèse d'une loi en densité est jugée « sérieusement en défaut » (p. 755).
- **Hispano de 150 hp** : puissance en altitude / puissance au banc ≈ (p/p₀)^**1,04** (p. 752 ; exposant lu deux fois dans l'OCR, à confirmer sur l'image).
- **Hispano de 170 hp à haute compression** : loi en pression d'exposant voisin de 1, « un peu plus défavorable » que celle du RAF 4D ; les exposants exacts sont **illisibles** dans l'OCR.
- **Explication proposée** (p. 753-754) : la charge s'échauffe au contact des parois jusque vers 300 °C avant la fermeture de la soupape d'admission ; la température extérieure ne change donc presque pas la densité de la charge, d'où une puissance indiquée proportionnelle à la pression. Le frottement constant fait ensuite décroître la puissance au frein un peu plus vite que la pression. Cela recoupe H15 (puissance qui décroît moins vite que la densité quand la température monte).
- **Au banc** (rapport L.C.E. 196, cité p. 754) : une hausse de 19 % de la température absolue de l'air ne fait perdre que 5 à 7 % de puissance au frein.

**Comparaison en atmosphère type** (C, calcul fait ici ; ISA, p/p₀ et σ standard) :

| Altitude | (p/p₀)^1,04 (H21) | Gagg-Farrar, k = 0,132 | Forme de H14, k = 0,08 |
|---|---|---|---|
| 1 000 m | 0,883 | 0,895 | 0,900 |
| 2 000 m | 0,777 | 0,798 | 0,807 |
| 3 000 m | 0,682 | 0,708 | 0,722 |
| 4 000 m | 0,597 | 0,625 | 0,642 |
| 5 000 m | 0,520 | 0,548 | 0,569 |

La mesure **en vol** de H21 perd **plus** que les deux lois tirées du banc : environ 5 % de moins que Gagg-Farrar et 9 % de moins que la forme de H14 à 5 000 m.

**Portée.**
- H21 porte sur un 150 hp sans réducteur, c'est-à-dire la classe du 8Aa. Le constructeur du moteur essayé (français ou britannique) n'est pas précisé. **B pour un 8Aa, sous réserve de l'exposant à confirmer sur l'image.**
- Le rapport mesure la puissance **installée**, relevée en vol par l'hélice. H14 et H15 sont des mesures au banc en caisson. L'écart entre les deux familles de résultats croît avec l'altitude ; aucun document lu ne l'explique. Ce n'est pas le même phénomène que l'écart constant de 3 à 5 % entre vol et soufflerie relevé par H17 (§ 7.4).
- Pour le jeu, qui vole en atmosphère type, une loi en pression s'applique directement : P(h) = P_banc(N) × (p/p₀)^1,04 pour le 150 ch.

### 4.4 Puissance « estimée » en altitude du tableau américain (H19, p. 203 ; cinquième passe) [LU-IMG]

La dernière partie du tableau des performances des moteurs donne une « puissance estimée » à 6 000, 10 000, 15 000 et 20 000 ft :

| Ligne | 6 000 ft (1 829 m) | 10 000 ft (3 048 m) | 15 000 ft (4 572 m) | 20 000 ft (6 096 m) | Référence au sol |
|---|---|---|---|---|---|
| Hispano 150 (section britannique) | 129 hp | 114,2 hp | 97,5 hp | 83,8 hp | 156 hp normaux à 1 500 tr/min |
| Hispano 180 (section britannique) **et** Hispano 180 * (français) : lignes identiques | 182 hp | 161 hp | 137,5 hp | 118 hp | 220 hp normaux (britannique) |

Les deux lignes américaines du Hispano n'ont pas d'estimation.

**Lecture** (C, calculs faits ici) :
- Les rapports à la puissance au sol sont **les mêmes** pour le 150 et le 180 : 0,827, 0,732, 0,625 et 0,537. C'est donc une **loi conventionnelle** appliquée par les compilateurs, pas une mesure. Le tableau dit d'ailleurs « estimée ».
- Ces rapports suivent la **densité** de l'atmosphère type à 1 % près : 0,836, 0,738, 0,629 et 0,533.
- La mesure en vol de H21, (p/p₀)^1,04, donne aux mêmes altitudes 0,794, 0,678, 0,552 et 0,445. À 20 000 ft, l'estimation américaine est donc **plus optimiste de 20 %** que la mesure britannique.

**Usage** : à ne pas retenir comme loi d'altitude ; c'est une convention de calcul de 1918. Elle peut expliquer pourquoi des performances calculées à l'époque surestiment le plafond. Confiance C.

## 5. Consommation
- **Report 46, tableau 1 (p. 550)** [LU], en livres de carburant par hp au frein et par heure :
  - **au sol** : 0,515 (1 300), 0,500 (1 500), 0,503 (1 700), 0,512 (1 900), 0,525 (2 100) ;
  - **à « 30 000 ft »** : 0,649, 0,635, 0,610, 0,650, 0,650 ;
  - rapportée à la puissance indiquée : environ 0,46 à 0,49 lb/(hp·h) à toutes les altitudes.
  
  Confiance A pour le moteur essayé, B pour un 8Ab.
- **Conversion** (C) : 0,503 lb/(hp·h) ≈ 0,228 kg/(hp·h), soit **≈ 0,225 kg/(ch·h)**.
- **Contrôle** (C, données faibles) : à 180 ch, environ 40 kg/h, soit environ 56 L/h avec une masse volumique de 0,72 kg/L (valeur du format de données, non sourcée ici). Les 135 L du SPAD (`spad7.md` S13 [RÉSUMÉ]) donneraient environ 2,4 h à pleine puissance, **plus** que l'autonomie de 1,5 à 2 h citée par la même source. Ces chiffres d'autonomie et de contenance restent à vérifier.
- **Report 46, résumé** : rendement thermique au frein d'environ **24 %** au sol, en baisse vers 20 % en altitude (p. 525).
- **Quatrième passe** : l'article américain de 1917 sur le S.VII (H20) donne pour son Hispano de « 160 hp » une consommation d'environ ½ lb d'essence par hp et par heure (le signe est rendu « j » par l'OCR : lecture à confirmer) et **3 quarts d'huile par heure**, soit 2,8 L/h en quarts américains (C). La consommation d'essence concorde avec H14 (0,50 à 0,53 lb/(hp·h) au sol) : **B** pour l'ordre de grandeur.
- Huile : environ 2,8 L/h (H20, moteur américain, C).
- **Cinquième passe, H19, p. 203 [LU-IMG]** : consommation d'essence de **0,52 lb par hp et par heure** pour **toutes** les lignes Hispano (américaines, britanniques, françaises), soit 0,236 kg/(hp·h) (C). Consommation d'huile : 0,043 lb/(hp·h) pour le 150 britannique, 0,05 pour le 150 américain, 0,056 pour les 180. Par heure, au régime normal : 81,45 lb d'essence et 6,75 lb d'huile pour le 150 britannique ; 114,8 lb d'essence et 12,4 lb d'huile pour les 180 britannique et français. **Confiance A** pour la valeur de 0,52 lb/(hp·h), concordante avec H14 (0,50 à 0,525 au sol) et H20.

## 6. Masse

| Grandeur | Valeur | Source | Confiance | Remarque |
|---|---|---|---|---|
| 8Aa, avec huile et eau | 195 kg | H2 [RÉSUMÉ] | C | |
| Wright-Martin « A », à sec | 202 kg (445 lb) | H3 [RÉSUMÉ] ; H20 [LU-OCR] : 445 lb pour le Hispano « type V » de 160 hp, avec carburateur, magnétos, magnéto de départ, manivelle et moyeu d'hélice, sans radiateur, eau, huile ni échappements | **B** (deux sources concordantes, dont une d'époque) | Moteur américain. Cylindrée annoncée par H20 : 718 in³ (11,77 L, C), concordante avec le § 2 |
| 8Aa (tableau comparatif) | 215 kg | H1, miroir [RÉSUMÉ] | D | Tableau tronqué ; définition inconnue |
| 8Ab (tableau comparatif) | 230 kg | H1, miroir [RÉSUMÉ] | D → **B** | Recoupé par H19 (ligne suivante) |
| **H19, p. 203, masse à sec** [LU-IMG] | 150 américain : 440 lb (200 kg) ; 180 américain : 485 lb (220 kg) ; 150 britannique : 455 lb (206 kg) ; 180 britannique et 180 français : **510 lb (231 kg)** | H19 | **A** pour les valeurs imprimées | Le Hispano de 180 hp européen pèse 55 lb (25 kg) de plus que le 150 britannique (C). Couples imprimés : 546 lb·ft (150 britannique, exactement 156 hp à 1 500 tr/min) et 583 lb·ft (180, soit environ 222 hp à 2 000 tr/min), calculs C |

## 7. Hélice du SPAD VII

### 7.1 Fabricants et dimensions relevées

| Hélice | Moteur visé | Diamètre | Pas | Autres cotes | Source | Confiance |
|---|---|---|---|---|---|---|
| Gallia série 3 | Hispano 150 ch « standard et surcompressé », SPAD VII | 2 450 mm | 2 100 mm | largeur de pale 190 mm ; moyeu de 150 mm d'épaisseur ; alésage central de 70 mm ; 8 boulons de 11 mm sur un cercle de 150 mm | H8 [RÉSUMÉ] (données dites « officielles ») | C |
| Levasseur série 485 | Hispano 150 ch, SPAD VII | 2,45 m | 1,95 m | moyeu de 150 mm | H8 [RÉSUMÉ] | C |
| Éclair série 100 | Hispano 150 ch, SPAD VII | 2,35 m | 1,75 m | moyeu de 150 mm | H8 [RÉSUMÉ] | C |
| Plan britannique AB 759 | SPAD à Hispano 150 | 2,45 m | 1,91 m (une autre valeur citée : 1,85 m) | — | H8 [RÉSUMÉ] | C |
| Régy type 353 | homologuée sur le SPAD VII, homologation annulée ensuite au profit d'hélices mieux adaptées au monoplace à moteur Hispano | 2,35 m (forum H10) ; le texte d'H9 parle d'homologations à 2,50 m pour d'autres avions, d'où une confusion possible | — | — | H9, H10 [RÉSUMÉ] | D |
| Levasseur série 586 | **SPAD XIII** (pour comparaison) | 2,55 m | 2,30 m | — | H11 [RÉSUMÉ] | C (XIII) |
| « Les trois meilleures hélices » pour le SPAD VII 180 ch | document de 1917 cité par un forum | non détaillé | — | — | H10 [RÉSUMÉ] | — |

Bilan : pour le 150 ch, le **diamètre de 2,45 m** revient dans trois documents sur quatre, et le **pas** varie de 1,75 à 2,10 m selon le fabricant. Pour le 180 ch : **non trouvé**.

> **Sixième passe (2026-10-23)** : ce tableau est complété et en partie corrigé au § 7.5. En bref : la Gallia série 3 figure à 2,50 × 2,00 m dans la liste reprise par Hartmann (contre 2,45 × 2,10 m sur le forum) ; le pas de 1,91 m appartient bien à AB 759, et la valeur de 1,85 m à un autre plan, AB 764 (2,414 m) ; la Régy 353 est cotée 2,35 × 1,80 m avec un moyeu de 170 mm. La Levasseur 485 (2,45 × 1,95 m) et l'Éclair 100 (2,35 × 1,75 m) sont recoupées. Aucune de ces hélices n'a de mesure aérodynamique connue ; la seule mesure d'époque d'hélices en bois comparables trouvée est celle du VE-7 (§ 7.4).

Témoignage d'époque : le 28 décembre 1916, Guynemer réclame à Béchereau « une meilleure hélice » pour le SPAD de 150 ch (`spad7.md` S1 [LU]). Il trouvait donc les hélices du début de série insuffisantes.

### 7.2 Nombre de pales, matériau
Bipale en bois à pas fixe, d'après les fiches de collectionneurs (H8, H11 [RÉSUMÉ]). Confiance B pour le type ; l'essence de bois n'a pas été relevée.

### 7.3 Sens de rotation (vu du siège du pilote)

**Retenu : sens horaire vu du pilote, confiance C.** Ce n'est pas une mesure ; c'est une déduction tirée de quatre sources non primaires qui concordent :
1. Le Hispano 8B à **réducteur** fait tourner l'hélice dans le sens horaire vu de **face**, ce qu'on appelle une hélice tractive « à gauche » (*left-hand tractor*) (H1, miroir fandom [RÉSUMÉ]).
2. L'hélice du SPAD XIII (8Be à réducteur) est « à gauche » et **tourne en sens inverse** de celle du S.VII à entraînement direct (H12 [RÉSUMÉ]).
3. Sur le S.E.5a, le Hispano à réducteur tourne dans le sens **antihoraire vu du pilote**, et le Viper à entraînement direct dans le sens **horaire vu du pilote** (H13 [RÉSUMÉ], revues de maquettes concordantes ; la formulation de Wikipedia diverge, sans doute par confusion du point de vue).
4. Recoupement physique : le pilote de McCook doit mettre « beaucoup de pied à gauche » à cause du couple sur le SPAD XIII (`spad7.md` S15 [RÉSUMÉ]). C'est cohérent avec une hélice antihoraire vue du pilote sur le XIII (C), donc avec une hélice horaire sur le VII.
5. **Troisième passe, indice d'époque [LU]** : les trois hélices en bois de forme standard de la Navy essayées sur le Vought VE-7 à moteur **Wright 180 hp E-2** portent toutes la mention « *Rotation: Right hand* » (NACA Report 301, p. 5, légendes des figures 3, 4 et 5 ; vérifié sur l'image de la page). Deux maillons manquent pour en tirer le sens d'un S.VII :
   - le rapport **ne définit pas** la convention « right hand ». La lecture usuelle américaine (hélice tractive « à droite » = horaire vue de l'arrière, donc du pilote) est une connaissance générale que je n'ai pas retrouvée dans les documents lus ;
   - le rapport ne dit **ni** que le E-2 est un Hispano sous licence, **ni** qu'il est à entraînement direct. L'identification « Wright-Hispano E » repose sur S31 de `spad7.md` [RÉSUMÉ].

   Si ces deux maillons se confirment, l'indice concorde avec les points 1 à 4.

**Bilan** : la déduction reste de confiance **C**, mais elle est renforcée (cinq indices concordants, dont un document d'époque). Pour passer en B ou A, il faut :
- une définition écrite de « right hand » dans une publication NACA ou SAE des années 1920 ;
- ou, mieux, une notice ou une photographie d'un S.VII.

**Attention aux conventions** : selon un forum de collectionneurs, les sources britanniques et françaises décrivent la rotation vue du pilote, les sources allemandes vue de face (H8 [RÉSUMÉ]). Pour confirmer, il faudrait une photographie d'époque d'un S.VII dont le bord d'attaque des pales est lisible, ou la notice d'un S.VII. Seule une notice du **XIII** a été repérée (S38 de `spad7.md`).

**Quatrième passe : rien de nouveau.** Aucun des 208 numéros de *Flight* lus (1916-1919), ni le volume II du rapport ACA 1917-18, ne donne le sens de rotation d'un Hispano à entraînement direct. Le tableau des moteurs H19 a une colonne « tractive ou propulsive » (« Either »), mais pas de colonne de sens. Documents à obtenir, repérés dans la bibliographie Brockett (`spad7.md` S56) :
- Wright-Martin Aircraft Corporation, *Hispano-Suiza aeronautical engines, Birkigt patents ; instruction book*, juillet 1918, série 4A, 216 p., et septembre 1918, série 6H, 168 p. Une notice d'entretien donne en principe le sens de rotation du vilebrequin et le point de vue adopté ;
- *L'Aérophile*, 1er-15 avril 1919, p. 112-114, « Description du moteur Hispano-Suiza (150) horsepower » (source française) ;
- *The Aeroplane*, 4 avril 1917, p. 856-858, « The Hispano-Suiza motor » ;
- *Air Service Information Circular*, vol. 1, n° 5 (14 février 1920) : instructions de calage de l'allumage des Hispano-Suiza modèles E et I.

**Sixième passe (2026-10-23) : un indice de plus, sans changement de confiance.** Une hélice britannique dessinée pour le S.VII porte au moyeu « 140 HP Hispano Suiza », « R H Tractor S7 », « D 2450 », « P 2132 » et, sur l'autre face, le numéro de plan de l'Air Board « AB 723 » (woodenpropeller.com et deux fiches de vente Bonhams, H26 [RÉSUMÉ]). Comme pour les hélices du VE-7 (point 5), le marquage « right hand » n'est défini dans aucun document lu. L'indice concorde avec les points 1 à 5 ; la confiance reste **C**.

**Effets attendus en vol** (C, physique élémentaire, à faire émerger du modèle et non à scripter) : à pleine puissance et à basse vitesse, tendance à partir à gauche, contrée au pied droit ; réaction de couple en roulis vers la gauche.

### 7.4 Mesure d'époque d'hélices en bois comparables : NACA Report 301 (VE-7 ; source H17) [LU]

**Ce n'est pas l'hélice du SPAD.** Ce sont trois hélices en bois de forme standard de la Navy (« standard Navy wood type »), présentées comme hélices expérimentales pour le Vought VE-7 à moteur Wright 180 hp E-2. Le VE-7 est un biplace : un observateur occupe le poste arrière pendant les essais (p. 6). Le nombre de pales n'est pas écrit dans le texte lu. Elles sont essayées en vraie grandeur, montées sur l'avion complet, dans la soufflerie de 20 ft (6,1 m) de Langley. **Confiance A pour la mesure, C pour l'usage sur le SPAD.**

**Conditions** (p. 3, 6-7, 10) :
- veine ouverte de 20 ft de diamètre, jusqu'à 110 mph ; les ailes du VE-7 (34 ft d'envergure) dépassent la veine d'environ 7 ft ;
- axe de l'hélice toujours horizontal ;
- couple lu sur un bâti dynamométrique, régime au tachymètre (lecture à ±5 tr/min près) ;
- rapport daté du 18 juin 1928, publié en 1929.

**Définitions** (p. 7) :
- C_T = poussée effective / (ρ n² D⁴), où la poussée effective est la poussée moins l'augmentation de traînée de l'avion due au souffle ;
- C_P = puissance / (ρ n³ D⁵) ;
- J = V/(nD) ;
- η est un **rendement propulsif**, interférence hélice-cellule comprise.

**Les trois hélices** (p. 5, figures 3 à 5, cotes lues sur l'image ; conversions C) :

| Hélice | Diamètre | Pas | P/D | η max (tableau III, avion complet) | Source |
|---|---|---|---|---|---|
| B' | 8 ft 6 in (2,59 m) | 5 ft 1,2 in (1,55 m) | 0,6 | 0,715 à J = 0,60-0,65 | p. 5, p. 18 |
| I (3714) | 8 ft 2 in (2,49 m) | 5 ft 8,6 in (1,74 m) | 0,7 | 0,773 à J = 0,75 | p. 5, p. 19 |
| **D'** | **7 ft 10 in (2,39 m)** | **6 ft 3,2 in (1,91 m)** | **0,8** | **0,775 à J = 0,85** | p. 5, p. 20 |

Contrôle de cohérence (C) : C_T = 0,1215 au point quasi statique de D' (tableau II, p. 15 : 19,7 mph, 1 655 tr/min, 801 lb, ρ = 0,00230 slug/ft³) se retrouve avec D = 7,83 ft. Le diamètre dessiné est donc bien celui de l'hélice essayée.

**Tableau III, hélice D', VE-7 complet, coefficients finaux ajustés** (p. 20, vérifié sur l'image) :

| J | C_T | C_P | η |
|---|---|---|---|
| 0,10 | 0,1243 | 0,0668 | 0,186 |
| 0,15 | 0,1197 | 0,0677 | 0,265 |
| 0,20 | 0,1149 | 0,0683 | 0,336 |
| 0,25 | 0,110 | 0,0689 | 0,399 |
| 0,30 | 0,1051 | 0,0690 | 0,457 |
| 0,35 | 0,1002 | 0,0690 | 0,508 |
| 0,40 | 0,0951 | 0,0689 | 0,554 |
| 0,45 | 0,0900 | 0,0682 | 0,594 |
| 0,50 | 0,0847 | 0,0671 | 0,631 |
| 0,55 | 0,0789 | 0,0654 | 0,664 |
| 0,60 | 0,0730 | 0,0631 | 0,694 |
| 0,65 | 0,0671 | 0,0603 | 0,724 |
| 0,70 | 0,0608 | 0,0570 | 0,745 |
| 0,75 | 0,0544 | 0,0534 | 0,765 |
| 0,80 | 0,0478 | 0,0497 | 0,770 |
| 0,85 | 0,0418 | 0,0457 | 0,775 |
| 0,90 | 0,0351 | 0,0416 | 0,759 |
| 0,95 | 0,0288 | 0,0372 | 0,735 |
| 1,00 | 0,0224 | 0,0325 | 0,689 |
| 1,05 | 0,0160 | 0,0272 | 0,618 |
| 1,10 | 0,0096 | 0,0216 | 0,489 |

Les tableaux III de B' (p. 18) et de I (p. 19) sont lus et vérifiés sur l'image. Ils ne sont pas recopiés ici. Ils serviront si l'aérodynamicien veut encadrer le pas.

**Autres résultats utiles** :
- **Ailes** : sur l'hélice I, la présence des ailes **réduit le rendement propulsif maximal d'environ 5 %** et augmente un peu la puissance absorbée. L'empennage n'a **pas d'effet sensible** (p. 3, 10). Tableaux III, p. 19, 21 et 22, vérifiés sur l'image : η max vaut **0,773** avec l'avion complet, **0,770** avec les ailes sans empennage et **0,807** sans ailes ni empennage, chaque fois à J = 0,75.
- **Vol et soufflerie** : dans trois essais sur quatre, les coefficients de puissance mesurés en vol (rapports NACA 220 et 292) sont **3 à 5 % plus élevés** qu'en soufflerie. L'auteur en conclut que les moteurs délivraient en vol un peu moins que leur puissance au banc (p. 10). Pour le jeu, c'est un ordre de grandeur de l'écart possible entre puissance au banc et puissance installée (C).
- **Vrillage** : la déformation des pales en fonctionnement est faible, de 0,2 à 0,5° (p. 7).
- **Puissance absorbée** (calcul C sur le tableau II, p. 15) : vers 99,5 mph et 1 740 tr/min, couple de 552 lb·ft, soit environ **183 hp**. Au point quasi statique (19,7 mph, 1 655 tr/min, 556 lb·ft), environ **175 hp** pour **801 lb** de poussée effective (≈ 3,56 kN). Ces valeurs donnent un ordre de grandeur de la poussée statique avec un moteur de 180 hp, **pas une valeur SPAD**.

**Usage proposé pour le modèle** (C, à valider par l'aérodynamicien) :
- **D'** est la plus proche des cotes relevées pour le SPAD (§ 7.1) : diamètre 2,39 m contre 2,35-2,45 m, pas 1,91 m contre 1,75-2,10 m, P/D 0,8 contre environ 0,74 à 0,86 (calcul C : Éclair 1,75/2,35 ; Gallia 2,10/2,45). Le pas de D' coïncide avec celui du plan britannique AB 759 (1,91 m, H8 [RÉSUMÉ]), ce qui est probablement une coïncidence.
- Deux options :
  - **calibrer le calcul BEMT** de l'hélice Gallia ou Levasseur pour qu'il retrouve la table de D' quand on lui donne la géométrie de D' (forme en plan et calages lisibles sur la figure 5, ordonnées des profils au tableau I, p. 12) ;
  - ou **reprendre directement la table de D'** comme hélice de secours, mise à l'échelle par le diamètre (les coefficients sont sans dimension), avec une étiquette `C:` dans les données.
- **Domaine couvert** (calcul C) : en palier à 208 km/h et 2 000 tr/min avec D = 2,45 m, J ≈ 0,71. La table (J de 0,10 à 1,10) couvre le palier, la montée et un piqué modéré.
- **Contrôle par les régimes en palier** (cinquième passe ; `spad7.md` S46, p. 205, lu sur l'image ; calculs C avec D = 2,45 m) :
  - 8Ab à 2 000 m : 212 km/h à 1 870 tr/min, soit J ≈ 0,77 ;
  - 8Aa à 2 000 m : 192 km/h à 1 720 tr/min, soit J ≈ 0,76.
  
  Les deux versions volent donc près du **rendement maximal** de l'hélice D' (0,77 à 0,775 pour J de 0,75 à 0,85). **Contrôle fort** pour l'aérodynamicien : avec la puissance de S46 (205 hp à 1 800 tr/min pour le 8Ab, 175 hp à 1 700 tr/min pour le 8Aa) et une seule hélice, le modèle doit retrouver **à la fois** la vitesse et le régime de chaque altitude. Un pas trop faible ferait monter le régime au-delà de la valeur lue ; un pas trop fort le ferait baisser. (La quatrième passe donnait par erreur 1 720 tr/min pour le 8Ab.)
- **Ce que la table ne donne pas** : J = 0 (point fixe au sol), l'effet de l'incidence de l'axe (toujours horizontal ici) et l'effet du nombre de Mach en bout de pale. Pour ce dernier, le Report 319 (profils d'hélice à grande vitesse, téléchargé, non exploité) est une piste.

### 7.5 Sixième passe (2026-10-23) : hélice réelle du S.VII 150 ch, définition du pas, régimes mesurés en vol

**Objet.** Le modèle du S.VII 150 ch (8Aa, hélice D' portée à 2,45 m) retrouve la vitesse, la montée et le plafond de S46 (`spad7.md`). Il tourne en revanche trop lentement à la vitesse maximale : 1 665 tr/min au lieu de 1 720 tr/min à 2 000 m. L'hélice doit être choisie d'après les documents, pas d'après le régime.

#### 7.5.1 Accès et méthode

- **Outil de lecture** : échec DNS le 2026-10-23 pour woodenpropeller.com (avec et sans « www »), archive.org, gallica.bnf.fr, francearchives.gouv.fr, theaerodrome.com, catalog.hathitrust.org et reports.aerade.cranfield.ac.uk. **Aucune page web n'a pu être ouverte.** Tout ce qui vient du web est donc **[RÉSUMÉ]** : une quarantaine de requêtes, en français, anglais, allemand et italien.
- **Lus dans le dossier de travail** :
  - les NACA Reports 220 et 292, déjà téléchargés mais absents de `ntrs/INDEX.md` (fichiers `19930091287.pdf` et `19930091361.pdf`), lus sur l'image des pages **[LU-IMG]** ;
  - le volume II du rapport ACA 1917-18, les 208 numéros de *Flight* 1916-1919 et la bibliographie de Brockett **[LU-OCR]**.
- **Rien n'a été téléchargé** : l'outil de l'historien ne le permet pas. Aucun sous-dossier n'a donc été créé dans `sources/`. Les documents à récupérer sont listés au § 7.5.9.
- **Rien de nouveau dans les sources locales d'époque** : aucun passage de *Flight* ni du volume ACA ne donne l'hélice d'un SPAD. Le volume ACA ne cite que des hélices britanniques du S.E.5 à Hispano. Par exemple, l'hélice T.28086 du S.E.5 a un rayon de 47 in, soit 2,39 m de diamètre (C). Source : A. A. Griffith et B. Hague (second nom restitué d'après un OCR déformé), « On the Shape of Propeller Blades », R&M n° 452, juin 1918, même volume [LU-OCR] ; page imprimée non repérée dans l'OCR.

#### 7.5.2 Hélices autorisées pour le « Type VII · Hispano 150 ch » (H9 [RÉSUMÉ])

**Le tableau.** Le dossier de Gérard Hartmann sur les héliciers français contient un tableau dont chaque ligne donne : le type d'avion, le moteur, la série, le fabricant, deux cotes en mètres, une cote en millimètres et une observation.
- Les extraits vus ne donnent ni les intitulés des colonnes, ni la date, ni l'origine du tableau.
- Le même tableau porte, pour une hélice Régy du SPAD XIII, la mention « STAé 7 sept 1917 ». Il s'agit donc sans doute de la transcription d'une liste officielle d'hélices homologuées (déduction, C).

**Lecture des colonnes proposée** (C) : diamètre, pas, épaisseur du moyeu. Arguments :
- la Gallia série 3 du forum H8 a un moyeu de 150 mm d'épaisseur ;
- les premières cotes vont de 2,35 à 2,50 m et les secondes de 1,75 à 2,05 m. Cela donne des P/D de 0,74 à 0,82, valeurs ordinaires pour un chasseur (D' : 0,80).

| Série | Fabricant | Diamètre | Pas (lecture C) | Moyeu | Observation imprimée | P/D (C) |
|---|---|---|---|---|---|---|
| 485 | Levasseur | 2,45 m | 1,95 m | 150 mm | « Aucune tolérance en dessous » | 0,80 |
| 561 | Levasseur | 2,50 m | 2,05 m | 150 mm | « Moteur surcomprimé » | 0,82 |
| 3 | Gallia | 2,50 m | 2,00 m | 150 mm | « Moteur ordinaire ou surcompressé » | 0,80 |
| 100 | Éclair | 2,35 m | 1,75 m | 150 mm | « Moteur ordinaire ou surcomprimé » | 0,74 |
| 6 | Éclair | 2,40 m | 1,95 m | 150 mm | « Approuvé » | 0,81 |
| 2066 | Chauvière | 2,35 m | 1,80 m | 150 mm | non vue | 0,77 |
| 353 | Régy | 2,35 m | 1,80 m | **170 mm** | « Autorisation annulée » | 0,77 |

**Remarques.**
- **Statut** : chaque ligne a été vue dans un résumé distinct du moteur de recherche, qui la cite en entier. Le tableau lui-même n'a pas été lu. **Confiance C** pour chaque chiffre.
- **« Aucune tolérance en dessous »** : le sens n'est pas expliqué. D'après les résumés du même dossier, Levasseur fabriquait à 0,5 mm près sur la longueur, **0,5° près sur l'angle de pas** et 0,5 mm près sur l'épaisseur des pales.
- **Régy 353** : c'est la seule ligne à 170 mm de moyeu. Hartmann écrit qu'elle a été homologuée sur le SPAD VII, puis que l'autorisation a été annulée quand des concurrents ont présenté des hélices mieux adaptées au monoplace et à son moteur. Un lien entre le moyeu et l'annulation n'est **pas** établi.
- **« Surcomprimé »** : trois lignes sur sept visent un Hispano de 150 ch « surcomprimé ».
  - Un résumé de Wikipedia (H1) donne au 8Ab la désignation française « Type 34-S », le S signifiant « surcomprimé ».
  - Si c'est exact, ces lignes concernent le **8Ab**, encore rangé sous « Hispano 150 ch » dans cette liste, et la Levasseur 561 (2,50 × 2,05 m) serait une hélice du 180 ch.
  - **Hypothèse C**, à vérifier sur la liste elle-même. Elle s'accorde avec le marquage « Sur » d'une Gallia série 3 (§ 4).
- **Divergences avec les forums** :
  - Gallia série 3 : 2,45 m sur le forum aviation-ancienne t8351 (H10), et 2,45 × 2,10 m avec une pale de 190 mm sur le forum woodenpropeller (H8, données dites « officielles », sans source) ; ici, 2,50 × 2,00 m ;
  - Levasseur 561 : 2,45 m selon le forum t8351, qui écrit « et non 2,50 indiquée sur certains sites ».
  - **Non tranché.**
- **Même tableau, autre avion** : une Levasseur 486 de 2,70 m pour le « Type H » (hydravion F.B.A.) à Hispano de 150 ch.
- **Forme des pales** : selon Hartmann, les hélices Levasseur se reconnaissent à un bord d'attaque à courbure négative. Sans cote, cela ne sert pas au modèle.

#### 7.5.3 Hélices britanniques et pièces conservées (H8, H26, H27, H10 [RÉSUMÉ])

| Repère | Moteur marqué | Diamètre | Pas | Remarques | Source |
|---|---|---|---|---|---|
| AB 723 (numéro de plan de l'Air Board) | « 140 HP Hispano Suiza », « R H Tractor S7 » | 2 450 mm | 2 132 mm | D et P frappés au moyeu ; « dessin de style Levasseur » ; acajou selon woodenpropeller, hêtre selon Bonhams ; un fil du forum attribue un exemplaire à Mann Egerton | H8, H26 |
| AB 759 | SPAD S7, « Hisso 150hp » | 2,45 m | 1,91 m (« et non 1,85 m ») | attribution par Bob Gardner, spécialiste des hélices britanniques, sur le forum woodenpropeller | H8 |
| AB 764 | SPAD (moteur non précisé) | 2 414 mm | 1 850 mm | frappée « Levasseur Design » | H8 |
| Levasseur, NASM A19320047000 | « 180 hp Hispano-Suiza » selon la fiche | 243,8 cm (96 in) | non donné | corde de 21,6 cm (8,5 in) ; le musée juge le fabricant et l'attribution au S.VII **non confirmés** | H27 |
| Éclair série 100 du S.VII du Musée de l'Air | — | 2,35 m | non donné | selon le forum t8351 ; rien ne dit que cette hélice soit celle d'origine | H10 |

- Conversions (C) : P/D de 0,87 pour AB 723, 0,78 pour AB 759 et 0,77 pour AB 764.
- Sur les hélices de plan AB, D et P sont frappés au moyeu, en mm, selon un participant du forum H8. La convention britannique de mesure du pas n'est pas donnée.
- AB 723 vise le Hispano de 140 hp des tout premiers moteurs. C'est la plus forte en pas de toutes les hélices relevées.

#### 7.5.4 Quelle hélice sur le S.VII 150 ch en service en 1917 ?

**Non établi.** Aucun document lu ou résumé ne relie une hélice à une escadrille, à un appareil ou à l'essai de S46. Indices :
- **Un parc mêlé** : sept modèles français étaient autorisés pour le même avion et le même moteur (§ 7.5.2). Il n'y avait donc pas une « hélice du S.VII ».
- **Éclair** : l'hélice Éclair (types 6 et 100) est citée comme équipant le SPAD VII, « dont celui de Guynemer » (H9 ; Wikipédia « Hélice Éclair », H35 [RÉSUMÉ]).
- **Document du GQG du 27 décembre 1917** : destiné à réduire le nombre de modèles d'hélices en service, il désignerait les trois meilleures hélices du SPAD VII **180 ch**.
  - Le forum t8351 les marque en gras, mais le résumé a perdu la mise en forme. La Levasseur 485 n'en ferait pas partie (H10 [RÉSUMÉ]).
  - Ni la cote du document ni les trois noms ne sont connus.
- **Guynemer** : sa demande d'« une meilleure hélice » du 28 décembre 1916 (`spad7.md` S1 [LU]) porte sur le 150 ch, un an plus tôt.
- **Hélice de l'avion de S46** : **non trouvée**. S46 ne mentionne aucune hélice (`spad7.md` § 5.5.2).

#### 7.5.5 Définition du pas

- **Convention française de 1916-1918 : non trouvée.** Aucune source lue ou résumée ne dit :
  - si le « pas » frappé sur les moyeux ou imprimé dans la liste est géométrique ou effectif ;
  - à quel rayon il est mesuré.

  Seul indice (H9 [RÉSUMÉ]) : la tolérance de Levasseur s'exprimait en **degrés d'angle de pas**. Le pas se contrôlait donc sans doute par les calages de pale à des stations données (C).
- **Convention américaine, lue** : les hélices de la famille de D' sont cotées par leur « mean geometrical pitch », c'est-à-dire leur **pas géométrique moyen**. Sources [LU-IMG] : H24, p. 4 ; H25, p. 4. La liste des symboles du NACA, en fin de H24, distingue ce pas (p_g) du pas effectif (p_e), du pas « standard » et des pas de poussée nulle et de couple nul.
- **Contrôle (C, calculs faits ici sur les calages lus sur l'image)** :
  - hélice 3712 (H25, figure 1) : le pas géométrique 2πr·tan β vaut 79,0 à 79,1 in à 0,15, 0,30, 0,45, 0,60 et 0,75 R, puis 77,0 in à 0,90 R. Le pas annoncé est de 6 ft 7,1 in, soit 79,1 in ;
  - hélice D' (H24, figure 4) : 75,1 à 75,2 in à 0,56, 0,72, 0,89 et 0,97 R, pour 6 ft 3,2 in, soit 75,2 in ;
  - ces hélices sont donc **à pas géométrique constant** le long de la pale, au moins jusqu'aux trois quarts du rayon. Pas moyen, pas à 2/3 R et pas à 0,75 R y coïncident à 1 % près ;
  - l'accord de ces calculs avec les pas imprimés vaut aussi contrôle de ma lecture des calages, imprimés en petits caractères.
- **Sensibilité (C)** : à 0,75 R, où 3712 est calée à 18,9°, 0,5° de calage change le pas local d'environ 2,8 %. La seule tolérance de fabrication citée pour Levasseur suffit donc à déplacer le pas d'environ ±3 %.
- **Conséquence pour le modèle** : en attendant une définition écrite, traiter le pas français comme un **pas géométrique constant**, comme celui de D', avec l'étiquette `E:`. Si le pas français était mesuré autrement (en bout de pale, ou comme pas effectif), la comparaison des P/D avec D' serait faussée de quelques pour cent.

#### 7.5.6 Hélices de la famille de D' mesurées en vol (H24, H25) [LU-IMG]

**Les essais.** Deux rapports NACA mesurent **en vol** les hélices en bois standard de la Navy sur le Vought VE-7 : la série de 1924 avec un moteur Wright E-4 (H24, Langley, du 1er mai au 24 août 1924), la série de 1926 avec un Wright E-2 (H25). Méthode commune :
- des planés pour obtenir la polaire de l'avion ;
- des vols à pleins gaz de 50 à 135 mph, en montée, en palier ou en piqué selon la vitesse ;
- le régime relevé par un compteur de tours entraîné par l'arbre à cames ;
- la puissance tirée de l'étalonnage du moteur au banc, corrigée de la pression et de la température d'admission.

**Confiance A pour les mesures, C pour le SPAD** : autre avion, autre moteur. Les rapports ne disent pas que ces Wright sont des Hispano sous licence (identification C, comme au § 7.3).

**a) Géométrie**

| Hélice | Diamètre | Pas géométrique moyen | P/D (C) | Source |
|---|---|---|---|---|
| D' | 7 ft 10 in (2,388 m) | 6 ft 3,2 in au dessin ; 6 ft 2,8 in (1,900 m) mesurés après les essais | 0,80 | H24, p. 4-5 |
| 3712 | 8 ft 2 in (2,489 m) | 6 ft 7,1 in (2,009 m) | 0,81 | H25, p. 4 |
| 3713 | 8 ft 2 in | 6 ft 2,0 in (1,880 m) | 0,76 | H25, p. 4 |
| 3714 (doublon de l'hélice I) | 8 ft 2 in | 5 ft 8,8 in (1,748 m) | 0,70 | H25, p. 4 |
| 3715 | 8 ft 2 in | 5 ft 3,3 in (1,608 m) | 0,65 | H25, p. 4 |

Les hélices 3712 à 3715 ne diffèrent que par l'épaisseur et le pas, avec la même forme en plan (allongement 6). Elles sont en bouleau lamellé entoilé de coton, sans bord métallique (H25, p. 3-4).

**Géométrie complète de 3712**, utilisable pour un calcul par éléments de pale. Source : H25, figure 1 p. 4 (calages) et tableau I p. 14 (cordes). Rayon R = 49 in.

| r (in) | r/R | Corde (in) | Corde (m, C) | Calage (°) |
|---|---|---|---|---|
| 7,35 | 0,15 | 7,35 | 0,187 | 59,71 |
| 14,70 | 0,30 | 7,72 | 0,196 | 40,55 |
| 22,05 | 0,45 | 8,17 | 0,208 | 29,73 |
| 29,40 | 0,60 | 7,72 | 0,196 | 23,17 |
| 36,75 | 0,75 | 6,47 | 0,164 | 18,90 |
| 44,10 | 0,90 | 4,41 | 0,112 | 15,53 |

Les ordonnées des profils (extrados et intrados en pouces, de 2,5 à 90 % de la corde, rayons de bord d'attaque et de fuite) sont au tableau I, p. 14. Elles ne sont pas recopiées ici.

**b) Coefficients en vol (valeurs lissées), comparés à la table de soufflerie déjà utilisée** (D', H17, avion complet)

| J | C_P de D' en soufflerie (H17) | C_P de D' en vol (H24, tableau III, p. 24) | C_P de 3712 en vol (H25, tableau IV, p. 23) |
|---|---|---|---|
| 0,45 | 0,0682 | 0,0673 | 0,0682 |
| 0,50 | 0,0671 | 0,0662 | 0,0662 |
| 0,55 | 0,0654 | 0,0648 | 0,0638 |
| 0,60 | 0,0631 | 0,0630 | 0,0611 |
| 0,65 | 0,0603 | 0,0605 | 0,0583 |
| 0,70 | 0,0570 | 0,0576 | 0,0547 |
| 0,75 | 0,0534 | 0,0545 | 0,0510 |
| 0,80 | 0,0497 | 0,0510 | 0,0471 |
| 0,85 | 0,0457 | 0,0471 | — |

- **Tables complètes**, avec C_T et η : D' en vol de J = 0,30 à 0,85 (η max 0,760 à J = 0,80) ; 3712 en vol de J = 0,35 à 0,80 (η max 0,705 à J = 0,70).
- **Contrôle** : chaque ligne a été vérifiée par η = C_T·J/C_P. C'est ce contrôle qui fixe la lecture 0,0471 pour 3712 à J = 0,80.

**Ce qu'en disent les auteurs.**
- **H24** (p. 3 et 12) : en vol, poussée et puissance absorbée dépassent de **6 à 10 %** (8 % en moyenne) celles des maquettes de Stanford ; les rendements restent proches.
- **H25** (p. 13) :
  - en vol, aucun rendement ne dépasse 71 %, soit au moins 4 à 5 % de moins que ne le prévoient les maquettes ; les auteurs l'attribuent surtout aux vitesses plus élevées en bout de pale ;
  - une comparaison précise entre hélices exige de connaître les calages réels en fonctionnement, à cause de la déformation des pales.
- **H24** (p. 8) : une fois installé, le moteur n'entraînait pas les hélices au point fixe aux régimes attendus des essais de maquettes. Il a fallu reprendre l'installation (fuite à l'admission, magnétos, mélange) pour obtenir 1 580 tr/min au point fixe avec l'hélice I.

**c) Régimes à pleins gaz mesurés en vol** (extraits ; vitesses vraies). σ est la masse volumique relative, calculée ici à partir du poids spécifique de l'air imprimé (C).

| Hélice, moteur | Condition | Vitesse | Pente | Régime (tr/min) | J | σ | Essai |
|---|---|---|---|---|---|---|---|
| D', Wright E-4 (H24, tableau II, p. 22) | point fixe | 0 | — | 1 650 ; 1 640 | 0 | 1,00 | 31-8 ; 12-6 |
| | montée | 103,7 ft/s (114 km/h) | +11,5° | 1 640 | 0,484 | 0,97 | 31-4 |
| | montée | 116,9 ft/s (128 km/h) | +9,7° | 1 660 | 0,539 | 0,98 | 31-5 |
| | palier | 167,5 ft/s (184 km/h) | 0,0° | 1 800 | 0,713 | 1,00 | 32-1 |
| | palier | 171,2 ft/s (188 km/h) | 0,0° | 1 808 | 0,726 | 0,99 | 32-8 |
| 3712, Wright E-2 (H25, tableau III, p. 18) | point fixe | 0 | — | 1 480 à 1 504 | 0 | 1,00 | 2-10, 14-11, 15-11 |
| | montée | 100,4 ft/s (110 km/h) | +8,3° | 1 500 | 0,492 | 0,91 | 1-1 |
| | montée | 94,3 ft/s (103 km/h) | +8,2° | 1 504 | 0,461 | 0,89 | 15-1 |
| | palier | 175,5 ft/s (193 km/h) | 0,0° | 1 710 | 0,754 | 0,92 | 1-7 |
| | palier | 171,5 ft/s (188 km/h) | −0,1° | 1 668 | 0,756 | 0,90 | 15-2 |

**Rapport des régimes montée/palier à pleins gaz (C).**
- D' : 1 640-1 660 contre 1 800-1 808 tr/min, soit **0,91 à 0,92**.
- 3712 : 1 500-1 504 contre 1 668-1 710 tr/min, soit **0,88 à 0,90**. Chaque vol comparé à lui-même donne 0,877 (vol 1) et 0,902 (vol 15).

Ce sont des régimes **de VE-7**. Le moteur, la cellule, la masse (2 040 à 2 213 lb, soit 925 à 1 004 kg) et les J de montée (0,46 à 0,54) diffèrent de ceux du SPAD : ces chiffres ne se transposent pas tels quels. Ils donnent l'ordre de grandeur de la chute de régime entre palier et montée, pour un V8 Wright à prise directe et une hélice en bois de P/D voisin de 0,8.

#### 7.5.7 Ce que cela change pour le modèle (C ; arbitrage de l'aérodynamicien)

- **Les documents encadrent D'.** Dans la liste française, les P/D vont de 0,74 à 0,82, avec une médiane de 0,80 ; sur les plans britanniques, de 0,77 à 0,87. D', à 0,80, est au milieu : aucun document ne désigne une hélice de pas nettement plus faible.
- **Forme de la courbe C_P(J).** Le rapport C_P(J = 0,53) / C_P(J = 0,77) règle l'écart entre le régime de montée et celui du palier. Calculé ici par interpolation linéaire, il vaut :
  - 1,273 pour D' en soufflerie (H17) ;
  - 1,231 pour D' en vol (H24), soit 3,3 % de moins ;
  - 1,310 pour 3712 en vol (H25), soit 2,9 % de plus.
- **Ordre de grandeur (C, hypothèses simples).**
  - Près de 1 700 tr/min, la puissance du moteur croît à peu près comme le régime : la forme de H14 donne un facteur de 0,785 à 1 500 tr/min et de 0,890 à 1 700 tr/min. À J donné, le régime d'équilibre varie alors comme C_P^(−1/2).
  - Pour gagner 3,3 % de régime en palier (de 1 665 à 1 720 tr/min) sans changer celui de la montée, il faudrait un rapport C_P(montée)/C_P(palier) plus fort d'environ 6,7 % que celui de D' en soufflerie.
  - La courbe la plus favorable mesurée en vol (3712) n'apporte que 2,9 %, soit environ 1,4 % de régime. Celle de D' en vol va dans l'autre sens, d'environ −1,7 %.
- **Lecture.**
  - Mesurées en vol, des hélices en bois de pas voisin diffèrent d'environ ±1,5 % sur l'écart de régime entre montée et palier. La tolérance de fabrication (±0,5° de calage) déplace à elle seule le pas d'environ ±3 %.
  - L'écart du modèle (−3,2 %) est donc **du même ordre que l'incertitude des sources sur l'hélice**. Selon cet ordre de grandeur, il est toutefois **plus grand que ce que la seule forme de la courbe d'hélice peut expliquer**.
  - Les documents disponibles ne permettent pas de trancher entre « l'hélice absorbe trop » et « le moteur donne trop peu ». Pour un Hispano de 150 hp, seul le R&M 586 le pourrait (§ 7.5.9).

#### 7.5.8 Hélice recommandée pour le modèle (S.VII 150 ch, 8Aa)

| Rôle | Hélice | Diamètre | Pas | P/D | Définition du pas | Confiance | Raison |
|---|---|---|---|---|---|---|---|
| **Géométrie de référence** | **Levasseur série 485** | **2,45 m** | **1,95 m** | 0,80 | pas géométrique supposé constant le long de la pale (`E:`) | C | seule hélice dont les deux cotes reviennent dans deux transmissions distinctes (liste reprise par Hartmann, H9 ; forum H8) ; même diamètre qu'AB 759 et que la liste du forum t8351 ; même P/D que D' |
| Table aérodynamique de base | D' en soufflerie (H17), mise à l'échelle de 2,45 m | — | — | 0,80 | pas géométrique constant (vérifié, § 7.5.5) | A pour la mesure, C pour le SPAD | **inchangée** : même P/D que la Levasseur 485 |
| Variante 1 | 3712 en vol (H25), mise à l'échelle | 2,489 m à l'origine | 2,009 m | 0,81 | pas géométrique moyen, constant à 1 % près jusqu'à 0,75 R | A pour la mesure, C pour le SPAD | mesurée en vol ; géométrie complète pour caler un calcul par éléments de pale ; courbe C_P(J) la plus raide |
| Variante 2 | D' en vol (H24) | 2,388 m à l'origine | 1,900 m (mesuré) | 0,80 | idem | A pour la mesure, C pour le SPAD | même hélice que la base, mesurée en vol ; courbe la plus plate |
| Variante historique | Éclair série 100 (série 6) | 2,35 m (2,40 m) | 1,75 m (1,95 m) | 0,74 (0,81) | inconnue | C | type d'hélice cité pour les SPAD VII, dont celui de Guynemer |

- **Non recommandées** :
  - la **Gallia série 3**, retenue comme référence le 2026-10-20 : ses cotes changent selon la source (2,45 × 2,10 m ou 2,50 × 2,00 m) ;
  - **AB 723**, dessinée pour le Hispano de 140 hp.
- **Pour le 180 ch** : toujours aucune hélice attestée. Si l'hypothèse « surcomprimé = 8Ab » se vérifie (§ 7.5.2), les candidates seraient la Levasseur 561 (2,50 × 2,05 m), la Gallia série 3 et l'Éclair 100, toutes en C.

#### 7.5.9 Documents à obtenir (pour la session principale)

| Priorité | Document | Où | Ce qu'on y cherche |
|---|---|---|---|
| 1 | ARC R&M 586 (1921), Aerodynamics Staff of the R.A.E., « Report on Various Airscrews Designed for S.E.5, with 150 H.P. Ungeared Hispano-Suiza Engine » | archives AERADE de Cranfield, collection des R&M de l'Aeronautical Research Committee, `https://reports.aerade.cranfield.ac.uk/handle/1826.2/11`, à parcourir par numéro ou par titre (fichiers sur le modèle `arc-rm-NNNN.pdf`, comme `arc-rm-3555.pdf`) ; à défaut, *Technical Report of the Aeronautical Research Committee* 1920-21 (HMSO) | diamètres, pas, calages et rendements de plusieurs hélices sur un Hispano de 150 hp sans réducteur ; régimes en montée et en palier ; courbe du moteur (figure 10, § 3.3) |
| 1 | G. Hartmann, « Les héliciers français », PDF | `https://www.hydroretro.net/etudegh/heliciersfrancais.pdf` | le tableau « Type VII » complet : intitulés des colonnes, origine et date de la liste, page, lignes « Hispano 180 ch » s'il y en a |
| 1 | G. Eiffel, *Études sur l'hélice aérienne faites au laboratoire d'Auteuil*, Paris, Librairie aéronautique E. Chiron, 1921, 2 vol. (texte XVI-304 p. ; XXVII planches) | notice BnF (CCFr) `https://ccfr.bnf.fr/portailccfr/ark:/16871/00111293469` ; Smithsonian Libraries, cote TL705 .E5X 1921 (`https://www.si.edu/object/siris_sil_549981`) ; à chercher sur Gallica | un participant du forum The Aerodrome a publié un dessin d'hélice du « SPAD à Hispano-Suiza de 150 hp », numérisé dans « un ensemble d'ouvrages d'Eiffel » (H28 [RÉSUMÉ]) : géométrie, calages, peut-être essais de maquette |
| 2 | G. Eiffel, *Résumé des principaux travaux exécutés pendant la guerre au laboratoire aérodynamique Eiffel, 1915-1918*, Paris, E. Chiron, 212 p. (daté [1919] par HathiTrust, 1920 par Brockett) | HathiTrust, notice 001035741 (exemplaire de l'université du Michigan, en accès intégral selon la notice) ; peut-être Bibliothèques numériques de la Défense, document `75bed661-8e23-4f70-942a-0e0608fa5971` (contenu non confirmé) | essais d'hélices et de maquettes de SPAD |
| 2 | The Aerodrome, fil « Propeller for Hispano Suiza 150 hp SPAD » | `https://theaerodrome.com/forum/showthread.php?p=785680` | le dessin et sa référence exacte |
| 2 | Forum aviation-ancienne, fil t8351 | `https://aviation-ancienne.forumactif.com/t8351-dimensions-de-l-helice-pour-le-spad-vii` | les trois hélices en gras du document du GQG du 27 décembre 1917, et la référence de ce document |
| 2 | Forum woodenpropeller : fils 950-, 3469- et 3326- (archive `t-3247.html`) ; pages `SPAD_VII.html` et `SPADs7.html` | `https://www.woodenpropeller.com/forumvB/forum/wooden-propeller-identification/early-wooden-propellers/<numéro>-` | origine des données « officielles » de la Gallia série 3 ; AB 759 et AB 764 (Bob Gardner) |
| 2 | S. Drzewiecki, *Théorie générale de l'hélice : hélices aériennes et hélices marines*, Paris, Gauthier-Villars, 1919, XII-184 p. (S56) | Gallica, à chercher | la définition française du pas à l'époque |
| 3 | A. Lapresle, « Résultats acquis sur la question de l'hélice aérienne », *L'Aérophile*, 29e année, n° 19-20 (1er-15 octobre 1921), p. 305-312 ; P. Levasseur, « Les avions et les hélices Pierre Levasseur », *L'Aéronautique*, 3e année, n° 30 (novembre 1921), p. 463 (S56) | Gallica | conventions et cotes de l'époque |
| 3 | SHD, département Air : listes d'homologation des hélices de la Section technique de l'aéronautique | **cote non trouvée** ; chercher sur FranceArchives « hélices homologuées », « Section technique de l'aéronautique », « hélice » avec « Spad » ; voir aussi le fonds Dorand (`spad7.md` S69, DE 2012 ZL 245) | la liste originale : date, colonnes, définition du pas |
| 3 | Image à 300 dpi de la p. 528 de *Flight* du 16 mai 1918 (`Flight_International_Magazine_1918-05-16-pdf`), déjà demandée dans `spad7.md` § 9.6 | archive.org | la légende du dessin coté du S.XI mentionne l'hélice (« Screw », « Hub ») et peut-être un régime ; illisible dans l'OCR |

#### 7.5.10 Septième passe (2026-10-24) : le dossier Hartmann lu en entier, le R&M 586 introuvable en ligne

**Accès.** Le joueur a autorisé `www.hydroretro.net` et `reports.aerade.cranfield.ac.uk` dans l'environnement. Le dossier a été téléchargé par le coordinateur (`sources/hartmann/heliciersfrancais.pdf`, 23 pages, auteur Gérard Hartmann, créé le 24 mai 2008), puis lu sur le texte extrait et vérifié sur l'image des pages 19 et 20 **[LU-IMG]**.

**Le tableau** (annexe, p. 19-20) a pour colonnes « Type, Moteur, série, Hélice, Diam., Pas, Moyeu épaisseur, Commentaires ». Le texte (p. 13) le présente ainsi : il existait 253 types d'hélices, et l'inspection du matériel décida de ne garder que trois fournisseurs, dont Éclair. L'origine officielle de la liste n'est pas écrite ; seules deux lignes portent « STAé 7 sept 1917 ». Secondaire, confiance **C**, mais lue cette fois.

Lignes « SPAD », moteur Hispano 150 ch (p. 19-20) :

| Type | Série | Fabricant | Diamètre | Pas | P/D (C) | Moyeu | Commentaire imprimé |
|---|---|---|---|---|---|---|---|
| Type VII | 353 | Régy | 2,35 m | 1,80 m | 0,77 | 170 mm | Autorisation annulée |
| Type VII | 6 | Éclair | 2,40 m | 1,95 m | 0,81 | 150 mm | Approuvé |
| Type VII | 367 | Régy | 2,45 m | 1,90 m | 0,78 | 140 mm | Moteur surcomprimé |
| Type VII | 100 | Éclair | 2,35 m | 1,75 m | 0,74 | 150 mm | Moteur ordinaire ou surcomprimé |
| Type VII | 2066 | Chauvière | 2,35 m | 1,80 m | 0,77 | 150 mm | — |
| Type VII | 485 | Levasseur | 2,45 m | 1,95 m | 0,80 | 150 mm | Aucune tolérance en dessous |
| Type VII | 561 | Levasseur | 2,50 m | 2,05 m | 0,82 | 150 mm | Moteur surcomprimé |
| Type VII | 3 | Gallia | 2,50 m | 2,00 m | 0,80 | 150 mm | Moteur ordinaire ou surcompressé |
| VII 19 m² | 2117 | Chauvière | 2,45 m | 1,85 m | 0,76 | 150 mm | Autorisation annulée |
| VII 19 m² | 2117 | Chauvière | 2,40 m | 1,95 m | 0,81 | 150 mm | Moteur surcomprimé |
| VII 19 m² | 2116 | Chauvière | 2,40 m | 1,80 m | 0,75 | 150 mm | Remplace la 2117 moteurs surcomprimés |
| Type VII | 23211 | Chauvière | 2,60 m | 2,05 m | 0,79 | 150 mm | (Hispano 200 ch) |
| Type XII | 336 | Régy | 2,45 m | 1,90 m | 0,78 | 140 mm | **Pas mesuré aux 2/3 de la pale** |

Ce que la lecture complète ajoute ou corrige par rapport au § 7.5.2 :
- **Colonnes confirmées** : diamètre, pas, épaisseur du moyeu.
- **Lignes nouvelles** : Régy 367 (moteur surcomprimé) ; trois Chauvière pour un « VII 19 m² », variante non identifiée ici ; une Chauvière 23211 pour le S.VII à Hispano 200 ch.
- **Moteur ordinaire (8Aa)** : six hélices de 2,35 à 2,50 m et de P/D 0,74 à 0,81 (Régy 353 annulée comprise) ; trois d'entre elles ont le P/D de D' (0,80 à 0,81) : Éclair 6 (2,40 m), Levasseur 485 (2,45 m), Gallia 3 (2,50 m).
- **Définition du pas** : une seule ligne la précise, pour une Régy du SPAD XII : « pas mesuré aux 2/3 de la pale ». C'est le premier indice écrit d'une convention française. Il ne dit pas si toutes les autres lignes suivent la même. Pour une hélice à pas constant comme D', pas à 2/3 et pas moyen coïncident à 1 % près (§ 7.5.5).
- **Quelle hélice sur le S.VII ?** Le texte (p. 13) dit des hélices Éclair : « elles propulsent [...] les avions Letord LA3 et surtout le SPAD type VII (hélice Eclair type 6 et 100) ». Il dit aussi que Levasseur parvient à faire homologuer ses produits sur les avions les plus populaires, dont le SPAD VII (p. 15). **Éclair est donc présentée comme le fournisseur principal du S.VII**, sans dire lequel des deux types dominait. Secondaire, confiance C.

**R&M 586 : introuvable en ligne depuis l'environnement.** D'après l'index R&M 3555 (téléchargé, `sources/aerade/arc-rm-3555.pdf`, tables 3 et 5) [LU-OCR], c'est une monographie publiée en brochure et jamais reprise dans les volumes annuels. Les archives de Cranfield n'en ont pas de copie : leur fonds de 1919-1922 compte une dizaine de rapports seulement (dirigeables, théorie), et les recherches par titre, par mot et par date ne le trouvent pas. HathiTrust refuse les accès automatiques (réponse 403 du site). Reste une demande manuelle : HathiTrust, la bibliothèque de la Royal Aeronautical Society, ou les National Archives (série DSIR 23).

## 8. Fiabilité et défauts connus (pour les pannes et l'ambiance, pas pour le M1)
- Surchauffe par temps chaud, mise en température difficile par temps froid, bâti moteur à renforcer sur les premiers S.VII (`spad7.md` S5 [RÉSUMÉ]).
- Surchauffe vers 4 000 m d'un moteur reconstruit : il faut réduire les gaz et piquer pour refroidir (Hall, `spad7.md` S3 [LU]).
- Sur le XIII, le moteur refroidit vite en plané : le pilote doit manœuvrer sans cesse les volets du radiateur (`spad7.md` S15 [RÉSUMÉ]).
- Fuites d'huile, entretien plus lourd que sur les rotatifs des Nieuport (`spad7.md` S32 [RÉSUMÉ]).
- Guynemer demande dès le 18 novembre 1916 un moteur « renforcé » (`spad7.md` S1 [LU]).
- Flandre, 10 septembre 1917 : commande de pompe à eau en panne sur un SPAD à deux mitrailleuses, essence qui jaillit sur un carburateur (`spad7.md` S1 [LU]).

## 9. Sources (consultées le 2026-10-20)
- **H1** — Wikipedia (en), « Hispano-Suiza 8 », https://en.wikipedia.org/wiki/Hispano-Suiza_8, et ses miroirs : https://www.wikiwand.com/en/articles/Hispano-Suiza_8A ; https://military-history.fandom.com/wiki/Hispano-Suiza_8 ; https://wikipedia.classicistranieri.com/en/articles/h/i/s/Hispano-Suiza_8_5f1b.html. [RÉSUMÉ] Wiki : piste.
- **H2** — hydroretro.net, dossier « Hispano-Suiza », https://www.hydroretro.net/etudegh/hispanosuiza.pdf. [RÉSUMÉ] Attribué aux « études GH » du site (Gérard Hartmann ?) : **auteur à vérifier**. Secondaire.
- **H3** — National Air and Space Museum, « Hispano-Suiza A (Wright-Martin) V-8 Engine », https://si.edu/object/hispano-suiza-wright-martin-v-8-engine:nasm_A19710885000. [RÉSUMÉ] Secondaire institutionnelle.
- **H4** — aircraftinvestigation.info, « Hispano Suiza 8Ab », https://www.aircraftinvestigation.info/airplanes/Hispano_Suiza_8Ab.html. [RÉSUMÉ] Piste.
- **H5** — armedconflicts.com, « Hispano-Suiza 8Ab », https://www.armedconflicts.com/topic/view/66611. [RÉSUMÉ] Forum : piste.
- **H6** — The Aerodrome (forum), « Hispano-Suiza V8 aero engines », https://theaerodrome.com/forum/threads/hispano-suiza-v8-aero-engines.77166/. [RÉSUMÉ] Forum ; il cite l'essai du RAE du 8 août 1916 et l'ARC R&M 586 (juillet 1921), figure 10.
- **H7** — The Aerodrome (forum), « Climbing performance of the SPAD VII » (= S11 de `spad7.md`). [RÉSUMÉ]
- **H8** — woodenpropeller.com, forum « Early wooden propellers » (fils 373-, 950-, 2340-, 3326-, 3469-, 3847-), https://www.woodenpropeller.com/forumvB/forum/wooden-propeller-identification/early-wooden-propellers/, et page https://woodenpropeller.com/SPAD_VII.html. [RÉSUMÉ] Forum de collectionneurs : piste.
- **H9** — hydroretro.net, « Les héliciers français », https://www.hydroretro.net/etudegh/heliciersfrancais.pdf. [RÉSUMÉ] Secondaire.
- **H10** — Forum Aviation ancienne, « dimensions de l'hélice pour le SPAD VII », https://aviation-ancienne.forumactif.com/t8351-dimensions-de-l-helice-pour-le-spad-vii, et « Hélice Levasseur ou Hispano 150 HP », https://aviation-ancienne.forumactif.com/t8056-helice-levasseur-ou-hispano-150-hp. [RÉSUMÉ] Forum : piste.
- **H11** — ratier.org, fiche « 1917 Hélice tractive Levasseur série 586 n° 25016 », http://ratier.org/archives/fichelices/Leva586WEB.pdf. [RÉSUMÉ]
- **H12** — Wikipedia (en), « SPAD S.XIII », https://en.wikipedia.org/wiki/SPAD_S.XIII. [RÉSUMÉ] Wiki : piste.
- **H13** — IPMS USA, « SE.5a Propeller Two-Blade (right) and Two-Blade (Left) », https://reviews.ipmsusa.org/review/se5a-propeller-two-blade-right-and-two-blade-left ; Wikipedia (en), « Royal Aircraft Factory S.E.5 » ; notices de maquettes Eduard. [RÉSUMÉ] Pistes.
- Sources partagées avec `spad7.md` : S1 (Bordeaux, 1918) [LU] ; S3 (Hall, 1918) [LU] ; S5, S13, S15, S31, S32, S38 [RÉSUMÉ].

**Sources primaires lues (troisième passe)**

Fichiers PDF et texte extrait de NASA NTRS, téléchargés par le coordinateur le 2026-10-20 et lus dans le dossier de travail. Les chiffres douteux de l'OCR ont été vérifiés sur l'image des pages. Pages citées : pagination imprimée des rapports.
- **H14** — Victor R. Gage (Bureau of Standards), « A Study of Airplane Engine Tests », NACA Report No. 46, rapport annuel du NACA, p. 525 et suivantes ; 1920 d'après l'index NTRS. NTRS 19930091075, https://ntrs.nasa.gov/citations/19930091075. **[LU]** : résumé p. 525-527, tableau 1 p. 550 (vérifié sur l'image).
- **H15** — Victor R. Gage (Bureau of Standards), « Some Factors of Airplane Engine Performance », NACA Report No. 108, rapport annuel du NACA, p. 649 et suivantes, 1921. NTRS 19930091171, https://ntrs.nasa.gov/citations/19930091171. **[LU]** : résumé et conclusions.
- **H16** — S. W. Sparrow et H. S. White (Bureau of Standards), « Performance of a 300-Horsepower Hispano-Suiza Airplane Engine », NACA Report No. 103, rapport annuel du NACA, p. 579 et suivantes, 1921. NTRS 19930091166, https://ntrs.nasa.gov/citations/19930091166. **[LU]** : p. 581-582 et 584.
- **H17** — Fred E. Weick (Langley Memorial Aeronautical Laboratory), « Full Scale Tests of Wood Propellers on a VE-7 Airplane in the Propeller Research Tunnel », NACA Report No. 301, Government Printing Office, 1929 ; texte daté du 18 juin 1928. NTRS 19930091370, https://ntrs.nasa.gov/citations/19930091370. **[LU]** : texte entier, images des p. 5, 15 et 18 à 22.
- **H18** — L. J. Briggs et H. L. Dryden, « Aerodynamic Characteristics of Twenty-Four Airfoils at High Speeds », NACA Report No. 319, 1929 ou 1930 (date de la page de titre mal lue par l'OCR). NTRS 19930091389. Téléchargé, **non exploité** : piste pour les profils de pale et l'effet du nombre de Mach en bout de pale.
- Pistes citées par H17 : NACA Report 220 (Durand et Lesley, 1926, comparaison des essais d'hélices en vol et sur maquette) et NACA Report 292 (Crowley et Mixson, essais en vol de cinq hélices), sur les mêmes hélices du VE-7. Non lus.

**Sources lues en quatrième passe** (texte OCR d'archive.org téléchargé par le coordinateur ; **[LU-OCR]**)
- **H19** — « Complete technical specifications of important American and foreign aeroplane engines », « Performances of important American and foreign aeroplane engines » et « Further details… », tableaux compilés par le service aéronautique américain, d'après *Automotive Industries*, reproduits dans *Flight*, 13 février 1919, p. 201-203. archive.org `Flight_International_Magazine_1919-02-13-pdf`. Primaire (compilation officielle). OCR désordonné ; **relu sur l'image en cinquième passe** [LU-IMG] (images PNG à 150 dpi fournies par le coordinateur le 2026-10-21) : caractéristiques p. 201, accessoires p. 202 (carburateurs Zenith Duplex 48 D.C. pour les 150, 58 D.C. pour les 180 ; hélice en prise directe, « E.S. » ; aucun sens de rotation), performances p. 203. Voir aussi `spad7.md` S46 (tableau des avions, p. 204-205).
- **H20** — « The Spad Scout », d'après *Aerial Age Weekly* (2 juillet 1917), *Flight*, 16 août 1917, p. 840-841, paragraphe sur le moteur. `Flight_International_Magazine_1917-08-16-pdf`. Secondaire d'époque (presse américaine).
- **H21** — D. H. Pinsent et H. A. Renwick, « The Variation of Engine Power with Height », ARC R&M n° 462, mars 1918, dans *Technical Report of the Advisory Committee for Aeronautics for the Year 1917-18*, vol. II, HMSO, 1921, p. 748-757. archive.org `technicalreport1917aero_2`. Primaire (essais en vol du RAE).
- **H22** — H. C. Dickinson et H. G. Boutell, « The Altitude Laboratory for the Testing of Aircraft Engines », repris par *Flight*, 23 octobre 1919, p. 1402-1403 (suite d'un article commencé p. 1380). `Flight_International_Magazine_1919-10-23-pdf`. Secondaire d'époque ; rapport NACA d'origine à identifier.
- **H23** — H. Glauert et S. B. Gates, « Prediction of the Performance and Longitudinal Stability of an Aeroplane », ARC R&M n° 324, mars 1917, même volume ACA, p. 576 et suivantes : phrase d'époque sur une puissance au frein variant comme une puissance de la densité (exposant illisible dans l'OCR, p. 577). Cité pour mémoire.
- Bibliographie Brockett : `spad7.md` S56.

**Sources de la sixième passe (2026-10-23)**

*Lues dans le dossier de travail* (PDF de NTRS déjà téléchargés par le coordinateur, absents de `ntrs/INDEX.md`). Pages : pagination imprimée.
- **H24** — W. F. Durand et E. P. Lesley (Stanford University), « Comparison of Tests on Air Propellers in Flight with Wind Tunnel Model Tests on Similar Forms », NACA Report No. 220, publié en mars 1926 (réimpression du Government Printing Office, 1927). Fichier local `ntrs/19930091287.pdf` ; NTRS 19930091287, https://ntrs.nasa.gov/citations/19930091287 (adresse construite sur le modèle des autres rapports, non ouverte). **[LU-IMG]** : p. 3-13 et 16-29 (figures 1 à 5 et 14, tableaux II, III, VI, VII et VIII, liste des symboles en dernière page). Primaire.
- **H25** — J. W. Crowley Jr. et R. E. Mixson (Langley), « Characteristics of Five Propellers in Flight », NACA Report No. 292, publié en novembre 1928 ; texte daté du 8 novembre 1927. Fichier local `ntrs/19930091361.pdf` ; NTRS 19930091361, https://ntrs.nasa.gov/citations/19930091361 (même remarque). **[LU-IMG]** : p. 3-23 (figures 1 à 22, tableaux I à IV, références p. 14). Primaire.
- R&M 452 du volume ACA 1917-18 (`technicalreport1917aero_2`) [LU-OCR] : rayon de l'hélice T.28086 du S.E.5 (§ 7.5.1).

*Vues en résumé de recherche* [RÉSUMÉ], le 2026-10-23 (pages non ouvertes : échec DNS de l'outil de lecture) :
- **H9 (complément)** — G. Hartmann, « Les héliciers français », https://www.hydroretro.net/etudegh/heliciersfrancais.pdf : sept lignes « Type VII · Hispano 150 ch » (§ 7.5.2) ; tolérances de Levasseur ; homologation puis annulation de la Régy 353 ; Éclair types 6 et 100 sur le SPAD VII ; Levasseur 486 du « Type H » ; mention « STAé 7 sept 1917 » pour une Régy du SPAD XIII. Secondaire. L'auteur et la page restent à vérifier sur le PDF.
- **H8 (complément)** — woodenpropeller.com :
  - pages « SPAD s7 Wooden Propeller » (https://woodenpropeller.com/SPADs7.html) et « SPAD VII » (https://woodenpropeller.com/SPAD_VII.html) : AB 723 ;
  - fils du forum « Early wooden propellers » 3326- (archive : https://woodenpropeller.com/forumvB/archive/index.php/t-3247.html ; Gallia série 3), 950- et 3469- (AB 759, AB 764, d'après Bob Gardner), et 46746-spad-vii-prop (exemplaire attribué à Mann Egerton).
  - Forum de collectionneurs : piste.
- **H10 (complément)** — Forum Aviation ancienne :
  - fil t8351 (liste de séries et de diamètres ; Éclair 100 du S.VII du Musée de l'Air ; document du GQG du 27 décembre 1917) ;
  - fil t8336, https://aviation-ancienne.forumactif.com/t8336-helice-spad-vii : hélice de S.VII à 180 ch refaite en 1928 sur plans d'époque, marquée Contrôle Veritas.
  - Forum : piste.
- **H26** — Bonhams :
  - vente 25448, lot 132, « A two bladed 'Eddington' type wooden propeller as fitted to a SPAD VII, circa 1916 », https://cars.bonhams.com/auction/25448/lot/132/ ;
  - vente 20931, lot 117, « A twin bladed wooden propeller for 140Hp Hispano-Suiza engined SPAD VII », https://cars.bonhams.com/auction/20931/lot/117/.
  - Marquages relevés : « 140 HP Hispano Suiza », « R H Tractor S7 », « D2450 », « P2132 », « AB 723 ». Fiches de vente : piste.
- **H27** — Smithsonian, National Air and Space Museum, « Levasseur Propeller, fixed-pitch, two-blade, wood », A19320047000, https://airandspace.si.edu/collection-objects/levasseur-propeller-fixed-pitch-two-blade-wood/nasm_A19320047000. Institutionnelle. Le musée lui-même ne confirme ni le fabricant ni l'attribution au S.VII.
- **H28** — The Aerodrome (forum), « Propeller for Hispano Suiza 150 hp SPAD », message de « machinist », https://theaerodrome.com/forum/showthread.php?p=785680 : dessin d'hélice numérisé, dit tiré d'« un ensemble d'ouvrages » d'Eiffel. Piste vers H30.
- **H29** — Aeronautical Research Council, R&M n° 3555 (index des R&M, HMSO, 1968), https://reports.aerade.cranfield.ac.uk/bitstream/handle/1826.2/2824/arc-rm-3555.pdf : titre du R&M 586. Primaire (index).
- **H35** — Wikipédia (fr), « Hélice Éclair », https://fr.wikipedia.org/wiki/H%C3%A9lice_%C3%89clair. Wiki : piste.
- **H36** — enginehistory.org, « UKNA DSIR Propeller Records », https://www.enginehistory.org/References/UKNA/Prop-DSIR.shtml : liste de cotes DSIR consacrées aux hélices ; aucune entrée « Spad » dans le résumé. Piste.
- **H1 (complément)** — Wikipedia (en), « Hispano-Suiza 8 » : désignation française « Type 34-S » du 8Ab, avec le « S » de « surcomprimé » selon le résumé. Wiki : piste.

*Pistes bibliographiques* [PISTE] (repérées dans S56 ou dans des catalogues, non consultées) :
- **H30** — G. Eiffel, *Études sur l'hélice aérienne faites au laboratoire d'Auteuil*, Paris, Librairie aéronautique E. Chiron, 1921, 2 vol. (texte XVI-304 p. ; XXVII planches). Notice BnF, CCFr ark:/16871/00111293469 ; Smithsonian Libraries TL705 .E5X 1921.
- **H31** — G. Eiffel, *Résumé des principaux travaux exécutés pendant la guerre au laboratoire aérodynamique Eiffel (1915-1918)*, Paris, E. Chiron, [1919] selon HathiTrust (notice 001035741), 1920 selon Brockett ; 212 p.
- **H32** — S. Drzewiecki, *Théorie générale de l'hélice : hélices aériennes et hélices marines*, Paris, Gauthier-Villars, 1919, XII-184 p.
- **H33** — A. Lapresle, « Résultats acquis sur la question de l'hélice aérienne », *L'Aérophile*, 29e année, n° 19-20, 1er-15 octobre 1921, p. 305-312.
- **H34** — P. Levasseur, « Les avions et les hélices Pierre Levasseur », *L'Aéronautique*, 3e année, n° 30, novembre 1921, p. 463.

**Pistes non consultées**
- ~~Images des pages 201-203 de *Flight* du 13 février 1919 (H19)~~ : **lues en cinquième passe** (§ 3.2, 4.4, 5, 6). Reste à trouver l'original d'*Automotive Industries* (fin 1918 ou début 1919), d'où *Flight* tire ces tableaux, pour écarter une erreur de recopie (inversion possible des lignes françaises, § 3.2).
- Rapport NACA du Bureau of Standards sur les Hispano de 150 hp en caisson d'altitude (H22) : numéro à trouver dans les rapports annuels NACA de 1919-1921.
- Notices Wright-Martin de 1918 et *L'Aérophile* d'avril 1919 (§ 7.3).
- ARC R&M n° 586 (juillet 1921), figure 10 : courbe puissance/régime du Hispano 150, d'après l'essai du RAE du 8 août 1916 (archives Cranfield AERADE). **Sixième passe** : le rapport porte sur plusieurs hélices dessinées pour le S.E.5 à Hispano de 150 hp sans réducteur (titre au § 3.3) ; priorité 1 (§ 7.5.9).
- Rapport du département moteurs du RAE, 8 août 1916 (National Archives).
- Rapports d'essais de Chalais-Meudon de 1915 (endurance de 50 h) et fiches STAé des moteurs (SHD).
- Notices d'époque des héliciers (Levasseur, Gallia, Éclair, Régy, Ratier) et le dossier de Gérard Hartmann sur les héliciers français, en entier.

## 10. Journal des décisions
- **2026-10-20 — Puissance et régime du 8Aa.** 2 000 tr/min (H1) ou 1 700 tr/min (miroir de H1) ; 1 450 tr/min chez H3 (licence américaine, autre puissance de référence). **Retenu : 150 ch à 2 000 tr/min, confiance C.** La valeur porte le nom de la variante française dans la source principale ; 1 450 tr/min concerne le Wright-Martin.
- **2026-10-20 — Puissance et régime du 8Ab.** 2 100 tr/min (H1, H5, concordants) contre 1 800 tr/min (H4). **Retenu : 180 ch à 2 100 tr/min, confiance C.**
- **2026-10-20 — Unités.** Puissances françaises en ch métriques : `Ps(..)` dans les données. Justification : H5 donne 132,4 kW pour 180 ch, ce qui correspond exactement au ch métrique (C).
- **2026-10-20 — Courbe puissance/régime.** Non trouvée. Aucune forme proposée ; la priorité de recherche est l'ARC R&M 586.
- **2026-10-20 — Altitude.** Modèle de Gagg-Farrar par défaut (règle du projet), confiance C. Pas de suralimentation pour les 8Aa et 8Ab de série.
- **2026-10-20 — Masse du 8Aa.** **Retenu : 195 kg avec huile et eau (H2), confiance C.** C'est la seule valeur dont le contenu est précisé ; à préférer pour le centrage quand le radiateur et l'eau sont comptés à part.
- **2026-10-20 — Hélice de référence pour le calcul BEMT.** **Retenu : Gallia série 3** (2 450 mm, pas de 2 100 mm, pale de 190 mm), confiance C. C'est la fiche la plus complète, qui donne aussi la largeur de pale. **Variante** : Levasseur série 485 (2,45 m, pas de 1,95 m). Le pas réel d'un S.VII donné reste inconnu, et le profil de pale n'a pas été trouvé : la table C_T(J), C_P(J) sera un calcul `C:` à partir de ces cotes.
- **2026-10-20 — Hélice du 180 ch.** Non trouvée. Faute de mieux, garder les cotes du 150 ch et le signaler en `E:` dans les données.
- **2026-10-20 — Deuxième passe : rien de relevé.** archive.org et NTRS ne sont pas lisibles par l'outil de l'historien (DNS en échec ; `spad7.md` § 0). Ni l'ARC R&M 586, ni une consommation, ni une hélice d'époque n'ont pu être consultées. Pour les profils de pale d'hélice, NACA Report 319 (famille « R.A.F. » d'hélices, NTRS 19930091389, `profil-spad7.md` A19) est une piste pour le calcul BEMT.
- **2026-10-20 — Sens de rotation.** **Horaire vu du pilote, confiance C** : déduction concordante de H1, H12, H13 et de l'essai de McCook (§ 7.3). À confirmer par une photographie ou une notice d'époque.

**Troisième passe (rapports NACA lus)**
- **2026-10-20 — Courbe puissance/régime (remplace l'entrée « non trouvée »).** Retenu : facteurs du Report 46 (H14), rapportés à 2 100 tr/min, soit 0,785 / 0,890 / 0,943 / 1,000 pour 1 500 / 1 700 / 1 900 / 2 100 tr/min (§ 3.1). Confiance A pour le moteur essayé, **B pour un 8Ab** : même alésage, course et compression, constructeur non précisé. Pour le 8Aa, même forme mise à l'échelle, confiance C. Le R&M 586 reste à trouver pour le 150 ch.
- **2026-10-20 — Cote du 8Ab : 1 700 ou 2 100 tr/min.** H14 cote le moteur à 180 hp à 1 700 tr/min ; H1 et H5 donnent 180 ch à 2 100 tr/min. Ramené en air standard par la seule densité (C, hypothèse simplificatrice), le moteur de H14 donne environ 180 hp à 1 700 tr/min, mais **environ 202 hp (≈ 205 ch) à 2 100 tr/min**. Ce dernier chiffre dépasse la cote française.
  - Explications possibles, non tranchées : cote française prise à un autre régime ; correction de température moins forte que la densité (H15), qui creuserait encore l'écart ; moteur américain différent du 8Ab français.
  - **Retenu pour les données** : 180 ch à 2 100 tr/min (C) comme puissance de référence, avec la forme de H14. **Variante à essayer** : la puissance absolue de H14 corrigée en air standard (C).
  - L'arbitrage revient à l'aérodynamicien, par les essais de montée (STAé et rapport britannique n° 96, `spad7.md` § 5).
- **2026-10-20 — Loi d'altitude.** Fourchette : k ≈ 0,08 à 0,11 (H14, Hispano de 180 hp) ; k = 0,132 (Gagg-Farrar de la skill) ; perte plus forte sur le 300 hp (H16).
  - **Proposition** : g(σ) = σ − k(1 − σ), avec k de H14 selon le régime. Confiance B pour un 8Ab.
  - Je ne modifie pas la skill (hors de `docs/recherche/`). Gagg-Farrar reste le défaut, en C, tant que l'aérodynamicien n'a pas validé le changement.
  - L'origine de la référence « Gagg-Farrar » n'est toujours pas trouvée.
- **2026-10-20 — Consommation.** **Retenu : 0,503 lb/(hp·h) à 1 700 tr/min au sol, soit ≈ 0,225 kg/(ch·h)** (H14, tableau 1), confiance B pour un 8Ab, à pleins gaz et au mélange de puissance maximale. Elle augmente en altitude jusqu'à environ 0,61-0,65 lb/(hp·h) à « 30 000 ft ». L'écart avec l'autonomie citée par S13 (§ 5) est signalé, non résolu.
- **2026-10-20 — Report 103 (300 hp).** Utilisé seulement comme borne pour la perte en altitude, confiance C pour le 8A. **Correction** : j'avais d'abord qualifié ce moteur de modèle « H ». Cette désignation n'apparaît pas dans le rapport ; je l'ai retirée et remplacée par les cotes lues p. 584.
- **2026-10-20 — Hélice : table mesurée de secours.** Retenu : la **table C_T/C_P/η de l'hélice D' du VE-7** (H17, tableau III, p. 20) sert d'**étalon** au calcul BEMT et d'**hélice de secours** mise à l'échelle du diamètre. Confiance **A pour la mesure, C pour l'usage sur le SPAD** : ce n'est pas l'hélice du SPAD, l'avion et le moteur diffèrent, et le rendement inclut l'interférence propre au VE-7. La Gallia série 3 (§ 7.1) reste la géométrie de référence. D' est choisie parmi les trois hélices parce que son diamètre (2,39 m) et son rapport pas/diamètre (0,8) sont les plus proches des cotes relevées pour le SPAD.
- **2026-10-20 — Sens de rotation (troisième passe).** Indice nouveau : « Rotation: Right hand » sur les trois hélices du VE-7 (H17, p. 5). **Pas de relèvement de la confiance** : la convention n'est pas définie dans le rapport, et H17 ne dit pas que le moteur E-2 est un Hispano à entraînement direct. Reste C, renforcée (§ 7.3).

**Quatrième passe (2026-10-21)**
- **Cote du 8Ab : contradiction expliquée.** H19 cote le Hispano de 180 hp construit aux États-Unis à 1 700 tr/min et le Hispano de 180 français à 2 000 tr/min en régime normal et 2 100 au maximum ; les régimes sont validés par la vitesse de piston. Les « 180 hp à 1 700 tr/min » de H14 relèvent de la convention américaine. **Retenu** : pour le 8Ab français, régime de référence **2 100 tr/min**, confiance relevée de C à **B** pour le régime (H1, H5 et la ligne française de H19). La **puissance** de référence reste **180 ch, C**, avec deux variantes à essayer par l'aérodynamicien : environ 200 hp (H14 corrigé en air standard, C) et 223 hp au maximum (H19, lecture OCR, C). Les cibles B de vitesse du S.VII 180 ch (`spad7.md` § 5.4) serviront d'arbitre.
- **Loi d'altitude du 150 ch.** Retenu pour le 8Aa : P(h) = P_banc × (p/p₀)^1,04 (H21, mesure en vol), **B sous réserve** de l'exposant lu dans l'OCR. Elle remplace, pour le 150 ch, Gagg-Farrar (C), plus optimiste d'environ 5 % à 5 000 m.
- **Loi d'altitude du 180 ch.** Deux familles de données : banc en caisson (H14, k ≈ 0,08 à 0,11, B) et vol (H21 sur le 170 hp à haute compression, exposant voisin de 1, illisible). **Non tranché** ; à l'aérodynamicien. Si l'on veut une seule loi pour les deux moteurs, la loi en pression de H21 est la seule mesurée en vol.
- **Consommation et masse.** Consommation d'essence d'environ ½ lb/(hp·h) dans un document de 1917 (H20) : concorde avec H14, B. Masse à sec de 445 lb pour le moteur américain : B (H3, H20).
- **Sens de rotation.** Aucune donnée nouvelle ; reste **C**. Documents à obtenir listés au § 7.3.

**Cinquième passe (2026-10-21 ; tableaux de *Flight* relus sur l'image)**
- **Corrections d'OCR** : maximum du 180 américain, 205 hp (et non 203) ; maximum du 180 français, 225 hp (et non 223). La section britannique du tableau, absente de la quatrième passe, cote le 180 à 220 hp à 2 000 tr/min et 225 hp à 2 100 tr/min.
- **Puissance de référence du 8Ab pour le modèle (remplace l'entrée « Cote du 8Ab » de la quatrième passe).** **Retenu : 205 hp à 1 800 tr/min**, la « puissance maximale autorisée » associée aux performances du S.VII dans S46 (A pour la valeur imprimée), avec la forme de H14 (B). Cela donne environ 224 hp à 2 100 tr/min, en accord avec les 225 hp maximaux de H19 (C). La cote de « 180 ch » reste le nom de la classe, pas une puissance à mettre dans les données.
- **Puissance de référence du 8Aa.** **Retenu : 175 hp à 1 700 tr/min** (S46, A pour la valeur imprimée). La forme de la courbe reste celle de H14 mise à l'échelle (C, faute de mesure sur un 150 hp).
- **Lignes françaises de H19.** Les pressions moyennes imprimées suggèrent une inversion des puissances normales entre les lignes 180 * et 200 * (§ 3.2). **Non corrigé** dans les données : simple signalement, C.
- **Loi d'altitude.** La « puissance estimée » de H19 suit la densité à 1 % près : c'est une convention de calcul, **non retenue** (§ 4.4). La loi mesurée en vol de H21 reste la référence proposée pour le 150 ch.
- **Consommation : 0,52 lb/(hp·h), A** (H19, H14, H20 concordants).
- **Masse à sec du 8Ab : 231 kg (510 lb), A** pour la valeur imprimée dans H19 ; recoupe les 230 kg de H1.

**Sixième passe (2026-10-23 ; hélice du S.VII 150 ch)**
- **Géométrie de référence de l'hélice (remplace l'entrée du 2026-10-20 « Gallia série 3 »).**
  - Fourchette : sept hélices autorisées pour le « Type VII · Hispano 150 ch », de 2,35 à 2,50 m de diamètre et de 1,75 à 2,05 m de pas (P/D de 0,74 à 0,82) ; plans britanniques de 0,77 à 0,87 (§ 7.5.2 et 7.5.3).
  - **Retenu : Levasseur série 485, 2,45 × 1,95 m (P/D 0,80), confiance C.** C'est la seule hélice dont les deux cotes reviennent dans deux transmissions distinctes. Son diamètre est celui d'AB 759 ; son P/D est celui de D'.
  - La Gallia série 3 est abandonnée comme référence : 2,45 × 2,10 m sur un forum, 2,50 × 2,00 m dans la liste reprise par Hartmann. La pale de 190 mm n'a pas d'autre source.
  - Les deux tableaux de 7.5.2 et 7.5.3 sont en **[RÉSUMÉ]** : aucune valeur ne dépasse C.
- **Table aérodynamique de l'hélice.** **Maintenu** : D' en soufflerie (H17) mise à l'échelle, puisque la Levasseur 485 a le même P/D. **Ajouté** : deux variantes mesurées en vol, A pour la mesure et C pour le SPAD : 3712 (H25), à géométrie complète, et D' (H24).
  - Le rapport C_P(J = 0,53)/C_P(J = 0,77) vaut 1,231 (D' en vol), 1,273 (D' en soufflerie) et 1,310 (3712 en vol).
  - Le choix revient à l'aérodynamicien. Je n'ai retenu aucune variante d'après le régime qu'elle donnerait.
- **Définition du pas.**
  - Convention française : **non trouvée**.
  - Hélices de la Navy : pas géométrique moyen, constant le long de la pale à 1 % près jusqu'à 0,75 R (vérifié par le calcul sur 3712 et D').
  - **Retenu, en `E:`** : traiter le pas français comme un pas géométrique constant. Justification : c'est la seule convention écrite trouvée pour des hélices de même famille. Le contrôle de la fabrication en degrés de calage, chez Levasseur (H9 [RÉSUMÉ]), est compatible avec cette lecture.
- **Régime en montée d'un S.VII : non trouvé.**
  - Comparable, en A pour la mesure et C pour l'usage : sur VE-7 à pleins gaz, le régime de montée vaut 0,88 à 0,92 fois celui du palier (H24, H25, § 7.5.6). Ce rapport sert de **contrôle de vraisemblance** du modèle, pas de cible.
  - Ordre de grandeur (C, § 7.5.7) : la forme de la courbe d'hélice explique au mieux la moitié de l'écart de 3,2 % entre le régime du modèle et S46 en palier. Les sources lues ne permettent pas de dire si l'autre moitié vient du moteur ou de S46.
  - **Signalé à l'aérodynamicien** : une tolérance de ±3 % sur le régime est plus serrée que l'incertitude documentaire sur l'hélice. À elle seule, la tolérance de fabrication de ±0,5° de calage fait varier le pas d'environ ±3 %, soit environ ±2 % de régime (C). Le choix de la tolérance lui revient.
- **AB 759 précisé.** 2,45 × 1,91 m pour le SPAD à Hispano de 150 hp ; la valeur de 1,85 m citée en première passe appartient à un autre plan, AB 764 (2,414 × 1,85 m, « Levasseur Design »). AB 723 (2,45 × 2,132 m) vise le 140 hp. Tout cela vient d'un forum, en résumé : C.
- **Hélice du 180 ch.** Toujours non attestée.
  - Hypothèse C : « Hispano 150 surcomprimé » désignerait le 8Ab (« Type 34-S »).
  - Candidates si l'hypothèse se vérifie : Levasseur 561 (2,50 × 2,05 m), Gallia série 3, Éclair 100. **Rien n'est changé dans les données.**
- **Sens de rotation.** Le marquage « R H Tractor S7 » d'AB 723 concorde avec le § 7.3. Confiance inchangée (C) : la convention n'est pas définie.
- **Priorité documentaire.** Le R&M 586 passe en tête (§ 7.5.9) : ses essais de plusieurs hélices sur un Hispano de 150 hp sans réducteur sont le seul document repéré qui puisse séparer l'hélice du moteur.

**Septième passe (2026-10-24 ; dossier Hartmann lu, coordinateur)**
- Le tableau des hélices homologuées est lu sur l'image (§ 7.5.10) : colonnes confirmées, quatre lignes de plus, un indice sur la définition du pas (« aux 2/3 de la pale », une ligne).
- **Éclair est présentée comme le fournisseur principal du S.VII** (types 6 et 100). La référence Levasseur 485 du 2026-10-23 reste une hélice autorisée, mais le texte pèse désormais en faveur de l'Éclair 6 (2,40 × 1,95 m, même P/D que D') ou de l'Éclair 100 (2,35 × 1,75 m). Choix laissé au joueur (voir `spad7-calculs.md` § 7). **Décision du joueur (2026-10-24) : Éclair 6 (2,40 × 1,95 m) devient l'hélice de référence du modèle** ; Levasseur 485 et Éclair 100 restent des variantes.
- R&M 586 : pas en ligne (Cranfield, HathiTrust) ; à demander à la main.

