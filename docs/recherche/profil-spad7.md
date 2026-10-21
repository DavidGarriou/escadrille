# Profil d'aile du SPAD VII — fiche de recherche

- Identifiant : `profil-spad7` (fichier de polaires visé : `airfoils/spad.ron`, voir `format-avion.md`)
- Dernière mise à jour : 2026-10-21 (quatrième passe : conventions des coefficients établies par un document britannique d'époque ; incidences négatives et facteurs τ, δ **non trouvés** dans les documents disponibles ; § 2.3, 3.7, 4.2 et 4.5)
- État : **en cours**.
  - Un profil nommé **« Spad »** a été trouvé dans le NACA Report 93 (ordonnées et polaire du MIT de 1918). Confiance **B** pour le S.VII : le document ne précise pas de quel SPAD il s'agit.
  - Des polaires de **RAF 15** ont été mesurées par la NACA jusqu'au Reynolds de vraie grandeur. Confiance **A** pour ce profil, **C** comme substitut.
- Avion concerné : [`spad7.md`](spad7.md)
- Méthode et statuts des sources (**[LU]**, **[RÉSUMÉ]**, **[PISTE]**) : voir `spad7.md` § 0. Pour cette passe, **[LU]** désigne les rapports NACA téléchargés depuis NTRS par le coordinateur et lus ici : texte OCR, puis vérification visuelle des tableaux sur les pages PDF. Ce sont des copies locales non versionnées, dont l'adresse NTRS est donnée au § 7.

## 1. La question
Quel profil portaient les ailes du SPAD S.VII ? Existe-t-il, pour ce profil ou pour un profil comparable, des données de soufflerie publiées que le modèle de vol puisse utiliser ?

## 2. Le profil « Spad » du NACA Report 93 (découverte de la troisième passe)

### 2.1 Ce que dit le document [LU]
- Le Report 93 (A1) compile des profils issus de plusieurs laboratoires. Son index range, parmi les profils essayés au **MIT**, une section nommée **« Spad »**, sous la référence **n° 49** (A1, index p. 262 et p. 264).
- **Ordonnées** : tableau « Ref. No. 49, Spad », p. 268 (PDF p. 12). Elles sont lues à l'image et recoupées avec l'OCR et avec le tableau en médaillon de la planche (p. 291).
- **Conditions d'essai**, sur la planche « Reference No. 49 », p. 291 (PDF p. 35) : section « Spad » ; maquette de **18 × 3 in** (allongement 6) ; vitesse **55,7 ft/s** (lecture visuelle) ; essai au **M.I.T.** ; date **1918** (lecture visuelle). La planche donne les courbes L_c, D_c, L/D et centre de poussée en fonction de l'incidence, de −4° à 18°.
- **Ce que le document ne dit pas** : quel SPAD a servi (S.VII, S.XIII ou autre), ni comment les ordonnées ont été obtenues (relevé sur une aile ou sur un plan).

### 2.2 Ordonnées (en % de corde, au-dessus de la ligne de référence) [LU]

| x (%) | 0 | 1,25 | 2,5 | 5 | 7,5 | 10 | 15 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 | 95 | 100 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Extrados | 0,26 | 1,55 | 2,14 | 3,00 | 3,75 | 4,37 | 5,31 | 5,95 | 6,60 | 6,67 | 6,28 | 5,53 | 4,51 | 3,26 | 1,80 | 1,06 (OCR : 1,04) | 0,32 |
| Intrados | 0,26 | 0,07 | 0,03 | 0,08 | 0,16 | 0,26 | 0,55 | 0,85 | 1,25 | 1,31 | 1,10 | 0,82 | 0,51 | 0,29 | 0,15 | 0,07 | 0,0 |

**Caractéristiques déduites** (C, calcul fait ici) :
- **Intrados creux** : ses ordonnées sont positives de 5 % à 95 % de la corde.
- **Épaisseur maximale ≈ 5,4 %** vers 30 à 40 % (6,67 − 1,31 = 5,36 à 40 %).
- **Cambrure ≈ 3,8 %** vers 40 % de la corde. Ligne moyenne à 40 % : (6,67 + 1,31) / 2 = 3,99 au-dessus de la référence, moins environ 0,2 pour la droite qui joint le bord d'attaque (0,26) au bord de fuite (environ 0,16).

C'est donc un profil **mince et nettement cambré, à intrados creux**. Il est plus cambré que le RAF 15 et ressemble davantage au RAF 6 tel que le donne le même rapport (§ 4.2).

### 2.3 Polaire du MIT (1918)
- **Convention des coefficients (point essentiel).** Le Report 93 emploie un coefficient « absolu » défini par **P = C·ρ·V²**, sans le facteur ½ (A1, p. 259-260 [LU]). Les rapports ultérieurs le rappellent : les coefficients modernes C_L = L/(qS) sont « deux fois plus grands que les anciens coefficients L_c, D_c » (A21, pages des symboles [LU]). **Donc C_L = 2·L_c et C_D = 2·D_c** ; la finesse L/D ne change pas.
- **Convention britannique d'époque (quatrième passe) [LU-OCR]** : Glauert et Gates écrivent l'équilibre en palier W = 0,0051 K_L A V², avec V en mph et A la surface en ft² (ARC R&M 324, mars 1917, dans le rapport ACA 1917-18, vol. II, p. 578-579 ; A22). Or 0,0051 = 0,00237 slug/ft³ × (88/60)², c'est-à-dire ρ₀ × (conversion des mph en ft/s)², calcul C fait ici. La formule est donc **L = K_L ρ A V², sans le facteur ½**. Le **K_L britannique vaut C_L/2**, comme le L_c du Report 93. Les tableaux du NPL du même volume (profils d'hélice, A23) ont d'ailleurs des coefficients de portance maximaux d'environ 0,5, cohérents avec cette convention. Le rapprochement, seulement probable en troisième passe, est maintenant **établi par un document d'époque**.
- **Nombre de Reynolds** (C, calcul fait ici) : corde de 3 in (0,0762 m) et 55,7 ft/s (17,0 m/s) donnent environ **9 × 10⁴**, soit **50 à 100 fois moins** que le Reynolds du S.VII en vol (§ 4.4).
- **Valeurs** : la planche est petite. Ma lecture graphique est **grossière** et ne vaut qu'à titre indicatif (confiance **D**) : L_c max d'environ 0,5 vers 14 à 16°, soit un C_L max d'environ 1,0 ; finesse maximale de l'ordre de 16 vers 3 à 4°. **À numériser proprement** à partir du PDF, p. 291, avec un outil de relevé de courbes, avant tout usage.

### 2.4 Autres constats (inchangés)
- Différents profils d'aile auraient été essayés sur le S.VII sans être adoptés en série (A11 [RÉSUMÉ], wiki). Le « Spad » du MIT pourrait donc ne pas être le profil de série du S.VII : à vérifier.
- Les plans originaux du SPAD XIII auraient été détruits pendant la Seconde Guerre mondiale (A10 [RÉSUMÉ]).
- Réputation de « profil mince » et de décrochage brutal (A12 [RÉSUMÉ]). Elle est **compatible** avec l'épaisseur trouvée (5,4 %), sans être démontrée par elle.
- *Flugsport* 1919 : comparaison de profils français, dont le « Spad » (A8 [RÉSUMÉ]). C'est désormais une source de **recoupement** de la forme.

## 3. Polaires mesurées du RAF 15 (profil de référence à Reynolds de vraie grandeur)

### 3.1 NACA Report 233 (Munk et Miller, 1927) : la source principale [LU]
- **Laboratoire et méthode** (A15, p. 73-74) :
  - soufflerie à densité variable de Langley ;
  - maquette rectangulaire en duralumin de **30 × 5 in**, soit un **allongement de 6** (et non une aile infinie) ;
  - vitesse d'environ 76 ft/s, pression du caisson de 1 à 20 atmosphères ;
  - correction appliquée : traînée des fils et pièces de suspension retranchée ; **aucune correction de paroi** (le Report 331 l'ajoute plus tard, § 3.2) ;
  - coefficients absolus modernes C_L = L/(qS), C_D = D/(qS) ;
  - **C_M rapporté au quart de corde, positif à cabrer**.
- **Mise en garde des auteurs** : ces essais sont parmi les premiers de cette soufflerie ; certaines séries ne concordent pas avec les autres ; l'effet général du Reynolds est jugé donné « avec une précision raisonnable » (p. 73).
- **Ordonnées** : elles ont été mesurées sur la maquette finie (tableau XXXVI, p. 82). Les auteurs signalent que les tables d'ordonnées du RAF 15 publiées à différentes époques diffèrent entre elles (p. 74). Ma lecture visuelle de ce tableau (C) donne une épaisseur maximale d'environ 6,4 % vers 15 %, proche du fichier UIUC (6,5 %).

**Tableau XXX (p. 80)** : RAF 15, 20 atmosphères, **Re = 3 580 000**, allongement 6, sans correction de paroi.

| α (°) | C_L | C_D | C_M (¼ c) |
|---|---|---|---|
| −4,5 | −0,162 | 0,0139 | −0,054 |
| −3,0 | −0,052 | 0,0087 | −0,052 |
| −1,5 | 0,052 | 0,0083 | −0,053 |
| 0,0 | 0,166 | 0,0088 | −0,048 |
| 1,5 | 0,285 | 0,0119 | −0,045 |
| 3,0 | 0,398 | 0,0154 | −0,044 |
| 4,5 | 0,507 (OCR : 0,557 ; image et pente : 0,507) | 0,0205 | −0,049 |
| 6,0 | 0,629 | 0,0280 | −0,045 |
| 9,0 | 0,850 | 0,0477 | −0,048 |
| 12,0 | 1,063 | 0,0734 | −0,049 |
| 15,0 | 1,209 | 0,0997 | −0,046 |
| « 16 ± » | 1,127 | 0,1527 | −0,077 |
| 18,0 | 1,004 | 0,2181 | −0,138 |
| 21,0 | 0,924 | 0,2872 | −0,137 |

**Tableau XXXVII (p. 82)** : effet du Reynolds sur le RAF 15. La valeur du C_L max est recoupée par le produit (C_L max / C_D min) × C_D min.

| Pression (atm) | Re | C_L max | C_D min |
|---|---|---|---|
| 1,00 | 185 000 | 1,067 | 0,0101 |
| 2,51 | 461 000 | 1,061 | 0,0093 |
| 5,10 | 950 000 | 1,063 | 0,0094 |
| 10,75 | 2 060 000 | 1,095 | 0,0099 |
| 20,00 | 3 580 000 | 1,216 | 0,0083 |

Les tableaux complets aux pressions intermédiaires sont aux tableaux XXVI à XXIX (p. 79-80).

**Lecture physique** (C, interprétation faite ici) :
- À **Re = 3,58 × 10⁶**, le C_L tombe d'environ 1,21 (15°) à 1,13 (environ 16°) puis à 1,00 (18°) : la **chute après le maximum est marquée**.
- À **Re = 1,85 × 10⁵** (tableau XXVI), le maximum est plus bas (environ 1,07) et la chute plus douce (1,066 à 15°, 1,018 à 18°).
- Le C_M au quart de corde vaut environ −0,045 à −0,054 et reste presque constant jusqu'à 15°. Il plonge ensuite (−0,138 à 18°) : c'est la signature du décrochage.

### 3.2 NACA Report 331 (Louden) : les mêmes mesures corrigées de l'effet de paroi [LU]
- **Tableau XI (p. 617)** : RAF 15, essai de Langley, maquette de 5 × 30 in, allongement 6, Re = 3 580 000, « données de la référence 2 corrigées de l'interférence de paroi ». La référence 2 est le Report 233, et les C_L sont identiques.
- La **correction de Prandtl** déplace les incidences : −1,48 / 0,06 / 1,61 / 3,15 / 4,69 / 6,24 / 9,32 / 12,41 / 15,46 / 16,42 / 18,38 / 21,35° au lieu de −1,5 / 0 / 1,5 / … / 21°.
- Le rapport donne aussi la **traînée de profil C_D0 = C_D − C_L²/(π·6)** (A18, p. 592) :

| α corrigé (°) | −4,56 | −3,02 | −1,48 | 0,06 | 1,61 | 3,15 | 4,69 | 6,24 | 9,32 | 12,41 | 15,46 | 16,42 | 18,38 | 21,35 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| C_D0 | 0,0127 | 0,0085 | 0,0081 | 0,0075 | 0,0081 | 0,0079 | 0,0085 | 0,0095 | 0,0141 | 0,0203 | 0,0317 | 0,0937 | 0,1712 | 0,2474 |

- C_M rapporté au **bord d'attaque**, positif à cabrer. Contrôle (C) : C_M(bord d'attaque) ≈ C_M(¼ c) − C_L/4, vérifié à 0,005 près à 0°, 6°, 12° et 15°.
- Les deux C_D corrigés lus à 12,41° et à 15,46° (0,0809 et 0,1098) s'écartent d'environ 0,0007 de C_D0 + C_L²/(6π). C'est une lecture incertaine du dernier chiffre.
- Le Report 331 reprend 15 profils courants aux États-Unis (Clark Y, Göttingen, M-6, M-12, RAF 15, USA 27…). **Le RAF 6 n'y figure pas** (A18, p. 591).

### 3.3 Dispersion entre souffleries : NACA Report 309 (Diehl, rapport de juin 1928, imprimé en 1929) [LU]
- **Maquette** : RAF 15 de 6 × 36 in (allongement 6), en aluminium, construite par le NPL pour les essais internationaux de l'Aeronautical Research Committee. Elle a été essayée en 1923-1924 au Bureau of Standards, à Langley, au MIT et à McCook Field, entre 29,3 et 100 ft/s, donc à bas Reynolds. Elle était abîmée par ses voyages (56 trous, voilée) (A16, p. 5-8).
- **Correction de paroi de Prandtl** appliquée (p. 10). Résultats corrigés (tableau XI, p. 20 ; discussion p. 13-15) :
  - C_L max de **1,040 à 1,093**, sans les valeurs de McCook (1,110 et 1,153), jugées douteuses ;
  - C_D min de **0,0138 à 0,0147** (MIT : 0,0123, sans doute trop bas) ;
  - finesse maximale de **17,30 à 19,20** ;
  - incidence de portance nulle de **−1,62 à −2,00°**.
- **Conclusions** : l'accord entre souffleries dépend surtout du soin apporté à l'essai ; la correction de paroi est indispensable (p. 15).

### 3.4 Effet d'échelle sur une cellule de chasseur biplan : NACA Report 279 (Higgins, Diehl et DeFoe, 1928) [LU]
- Maquette de **Bristol Fighter au 1/15 à ailes RAF 15**, sans empennage, essayée en soufflerie à densité variable de Re ≈ 150 000 à plus de 3 000 000 (A17, résumé et introduction).
- **C_L max de la maquette** : 0,99 à 1 atmosphère, 1,11 à 2,5 atmosphères, puis baisse lente jusqu'à **1,032** à 20 atmosphères (A17, discussion). Valeurs non corrigées de l'effet de paroi.
- L'incidence de portance nulle reste constante. L'effet d'échelle sur la traînée vient surtout des parties autres que les ailes.
- Utile comme **ordre de grandeur du C_L max d'une cellule biplan à profil mince** (confiance C pour le SPAD).

### 3.5 Vraie grandeur : NACA Report 304 (Crowley et Green, 1929) [LU]
- **Sperry Messenger** biplan muni d'ailes RAF 15, USA 5, USA 27 et Göttingen 387, en vol plané à « traction nulle » de l'hélice, de 45 à 105 mph (A4, p. 3-5) :
  - envergure de 20 ft, écartement de 3 ft 10 in ;
  - dièdre de 1½°, calage de +2°.
- **Les coefficients sont ceux de l'avion complet**, avec sa traînée parasite, pas ceux du profil. Le **C_L max n'a pas été atteint** en vol. Précision : ±2 % sur les courbes lissées, ±3 à 4 % sur l'incidence (p. 6-7).
- **Conclusion utile** : le RAF 15, mince, donne le C_L max le plus faible et la traînée minimale la plus faible des quatre (p. 7). Le choix du profil change peu les performances, hormis le C_L max.

### 3.6 Données basse vitesse d'époque du Report 93 (RAF 6 et RAF 15) [LU]
- **RAF 6** (réf. n° 61) : NPL, maquette de 15 × 2,5 in, 30 ft/s, **1912** (planche p. 294).
  - Ordonnées p. 268 : extrados 7,59 et intrados 0,77 à 30 %, intrados positif (creux).
  - Épaisseur ≈ **6,8 %** et cambrure ≈ **3,9 %** (C, calcul fait ici).
  - **Ce n'est pas la géométrie du fichier UIUC « raf6 »** (≈ 10 % d'épaisseur et 4,6 % de cambrure ; ancienne § 4.2). Il existe donc au moins deux « RAF 6 » différents : se fier au Report 93.
- **RAF 15** (réf. n° 72) : NPL, maquette de 18 × 3 in, 40 ft/s, **1916** (planche p. 296). Convention L_c = C_L/2. Le maximum lu grossièrement (L_c ≈ 0,51, soit C_L ≈ 1,0) est cohérent avec les essais à bas Reynolds des Reports 233 et 309.

### 3.7 Incidences négatives : ce qui existe et ce qui manque (quatrième passe)

**Rectification.** Le tableau XXX du Report 233 ne s'arrête pas à −1,5° : il commence à **−4,5°** (C_L = −0,162 ; § 3.1). Le Report 331 reprend ces points jusqu'à −4,56° corrigé. Aucun des deux n'approche le décrochage négatif.

**Point le plus négatif trouvé pour le RAF 15** : Report 309 (A16), essais de McCook Field, mars 1924, à bas Reynolds [LU] :
- tableau IX, 29,3 ft/s : C_L = −0,337 à −6° (−6,19° corrigé), C_D = 0,0598 ;
- tableau X, 58,7 ft/s : C_L = −0,329 à −6° (−6,18° corrigé), C_D = 0,0589 ;
- les autres laboratoires du même rapport s'arrêtent à −4°.

Contrôle (C, calcul fait ici) : entre −4° et −6°, la pente vaut environ 0,08 par degré, la même qu'aux incidences positives. **À −6°, l'écoulement n'a donc pas encore décroché** côté intrados. Le C_L minimal et l'incidence du décrochage négatif du RAF 15 restent **inconnus**.

**Autres données lues, non représentatives** : les profils d'hélice du NPL publiés dans le volume II du rapport ACA 1917-18 (A23, p. 277 et suivantes) sont essayés jusqu'à −6°, et l'un d'eux de −30° à +25°. Mais ce sont des profils à intrados **plat** et extrados en arc de cercle, de 7,5 à 20 % d'épaisseur, très différents du « Spad » à intrados creux de 5,4 %. Un constat qualitatif reste utile : pour le profil le plus cambré de la série, la portance reste positive jusqu'à −25°, preuve que le comportement aux grandes incidences négatives dépend fortement de la forme.

**Où chercher** (§ 5) :
- volume I du même rapport ACA 1917-18 : essai « Forces and moments on R.A.F. 14 for negative angles of attack » (titre lu dans la table des matières du volume II). Le RAF 14 n'est pas le RAF 15, mais c'est un profil mince britannique de la même époque ;
- à défaut, un profil mince cambré à intrados creux essayé aux incidences négatives à Reynolds élevé. Le NACA Report 460 (profils NACA, pas RAF) est une **piste non vérifiée** : j'ignore jusqu'à quelle incidence négative ses essais descendent.

**Conséquence pour le modèle** (C) : tant qu'aucune mesure n'existe, la branche négative au-delà de −6° est une **extrapolation** à marquer `E:`. La forme du décrochage négatif d'un profil à intrados creux ne doit pas être copiée sur la branche positive.

## 4. Choix pour le modèle de vol

### 4.1 Proposition (révisée)
1. **Géométrie : le profil « Spad » du Report 93** (§ 2.2). Confiance **B** pour le S.VII : document primaire de 1918-1921, profil explicitement nommé, mais type de SPAD et origine des ordonnées non précisés.
2. **Polaires : calcul `C:`** (panneaux et couche limite) à partir de ces ordonnées, au Reynolds du vol (environ 2 à 5 × 10⁶). Recalage et contrôle sur :
   - (a) le **RAF 15 en vraie grandeur** (Reports 233 et 331, confiance A pour ce profil), pour la pente, les traînées et le comportement au maximum ;
   - (b) la **polaire « Spad » du MIT**, une fois numérisée (Report 93, confiance A comme mesure mais à Re ≈ 9 × 10⁴), pour la forme générale et l'incidence de portance nulle.
3. **Repli** si le calcul n'est pas prêt pour le M1 : **polaires du RAF 15** des Reports 233 et 331 (confiance C comme substitut). Elles sont probablement trop peu cambrées par rapport au « Spad » : incidence de portance nulle moins négative, C_M moins piqueur. Le **RAF 6** du Report 93 donne une variante plus cambrée, mais sa seule polaire disponible est à très bas Reynolds (1912).

### 4.2 Passage aux coefficients de profil (allongement infini)
Toutes les données ci-dessus sont des **maquettes d'allongement 6**. Formules de Prandtl, celles qu'utilise le Report 331 (A18, p. 592) : C_D0 = C_D − C_L²/(π·A), et α∞ = α − (C_L/(π·A))·(180/π), avec A = 6, en supposant une répartition elliptique (approximation).

Ordres de grandeur pour le RAF 15 à Re = 3,58 × 10⁶ (C, calcul fait ici sur les valeurs corrigées du Report 331, entre −1,48° et 6,24°) :
- pente de la maquette ≈ 0,0747 par degré, soit ≈ 0,0967 par degré (5,5 par radian) en allongement infini ;
- incidence de portance nulle ≈ −2,25° par interpolation (le Report 309 donne −1,6 à −2,0° à bas Reynolds) ;
- C_L max ≈ 1,21, peu modifié par le passage en 2D.

**Facteurs τ et δ d'une aile rectangulaire d'allongement 6 (quatrième passe) : non trouvés.**
- Les formules demandées par l'aérodynamicien, α∞ = α − C_L(1 + τ)/(π·A) (en radians) et C_D∞ = C_D − C_L²(1 + δ)/(π·A), corrigent l'hypothèse elliptique pour une forme en plan rectangulaire.
- **Aucun document disponible ne donne τ ni δ.** Le Report 331 calcule sa traînée de profil avec la formule elliptique pure, c'est-à-dire **δ = 0** (A18, p. 592) ; il en va de même pour les incidences corrigées. Les Reports 309 et 331 ne citent Glauert que pour la correction de paroi (le Report 331 rapporte que le coefficient de paroi passe de 0,125 à 0,137 pour une veine carrée, p. 592). Le Report 362 renvoie à Glauert, *The Elements of Aerofoil and Airscrew Theory* (Cambridge, 1926), non disponible. La TN 182 de Prandtl (`spad7.md` S61) suppose des répartitions elliptiques et ne donne pas non plus ces facteurs.
- Les valeurs τ ≈ 0,17 et δ ≈ 0,05 avancées par l'aérodynamicien restent donc **non vérifiées**. Documents à obtenir (§ 5) : NACA Report 460 (1933 ; NTRS 19930091108 d'après le moteur de recherche, à confirmer), NACA Report 669 (Jacobs et Abbott, 1939 ; NTRS 19930091744), qui traitent de la réduction des essais d'allongement 6 de la soufflerie à densité variable en caractéristiques de profil ; à défaut, le livre de Glauert.

**Sensibilité** (C, calcul fait ici avec les valeurs **non vérifiées** τ = 0,17 et δ = 0,05, pour l'aérodynamicien) :
- incidence induite à C_L = 1,0 et A = 6 : 3,04° en elliptique, 3,56° avec τ, soit **0,52°** d'écart ; 0,26° à C_L = 0,5 ;
- traînée induite à C_L = 1,0 : 0,0531 en elliptique, 0,0557 avec δ, soit **0,0027** d'écart, c'est-à-dire 13 à 27 % d'une traînée de profil de 0,02 à 0,01 ; 0,0007 à C_L = 0,5 ;
- pente en allongement infini du RAF 15 (pente de maquette 4,28 par radian) : 5,54 par radian en elliptique, 5,83 avec τ, soit **+5 %**.

À forte portance, l'écart de traînée dépasse la dispersion entre souffleries des traînées minimales du RAF 15 (0,0138 à 0,0147, Report 309, § 3.3). L'emploi de τ et δ mérite donc une source avant d'être figé dans les données.

### 4.3 Justification
- Le « Spad » est le seul profil trouvé qui porte ce nom dans un document d'époque. Sa finesse (5,4 %) concorde avec la réputation de profil mince.
- Le RAF 15 est le seul profil mince de la Grande Guerre mesuré à Reynolds de vraie grandeur dans les documents disponibles, d'où son rôle de **référence de calage**.

### 4.4 Précautions pour l'aérodynamicien
- **Échelle** : la corde du S.VII n'est pas connue, mais elle est de l'ordre du mètre : environ 1,15 à 1,2 m d'après la surface, 1,40 m d'après un article de 1917 incompatible avec la surface (`spad7.md` § 2.1). Avec plusieurs dizaines de m/s, le Reynolds du vol dépasse le million. Les données MIT et NPL à 6 × 10⁴ – 9 × 10⁴ sous-estiment le C_L max et surestiment la traînée (Reports 233 et 279 : le C_L max varie de façon irrégulière avec le Reynolds).
- **Décrochage** : les données du RAF 15 à Reynolds élevé montrent une chute après le maximum. C'est une **mesure**, et non un réglage : elle est cohérente avec la réputation (`spad7.md` § 6.2), sans la prouver pour le SPAD.
- **Biplan** : l'interférence entre plans relève du modèle, pas du profil. Facteurs et méthodes : `spad7.md` § 2.4, dont le facteur σ de Prandtl (TN 182) pour la traînée induite de la cellule.

### 4.5 Convention de chaque tableau utilisé (quatrième passe ; demande de l'aérodynamicien)

| Source | Coefficients de force | Moment ou centre de poussée | Corrections déjà appliquées | Vérification |
|---|---|---|---|---|
| Report 93 (A1), dont le « Spad » (MIT, 1918), le RAF 6 et le RAF 15 du NPL | L_c, D_c définis par P = C·ρ·V², **sans ½** : C_L = 2 L_c | **centre de poussée** en fraction de corde depuis le bord d'attaque (planches) | aucune (maquettes d'allongement 6) | p. 259-260 [LU] ; A21 |
| Données britanniques d'époque (NPL, RAE ; A22, A23) | K_L = L/(ρ A V²), **sans ½** : C_L = 2 K_L | centre de poussée depuis le bord d'attaque, mesuré le long de l'intrados ; moments autour du bord d'attaque (A23) | selon l'essai | R&M 324, p. 578-579 ; A23, p. 279 [LU-OCR] |
| Report 233 (A15), RAF 15 à Re élevé | C_L, C_D modernes, L/(qS) | **C_M au quart de corde**, positif quand il tend à **augmenter l'incidence** (cabrer) | aucune correction de paroi ; traînée des suspensions retranchée | p. 73 [LU, revérifié] |
| Report 331 (A18) | modernes | **C_M au bord d'attaque**, positif quand il tend à **lever le bord d'attaque** ; centre de poussée en fraction de corde | **paroi corrigée** (Prandtl) ; C_D0 calculé en **elliptique (δ = 0)** | p. 591-592 [LU] |
| Report 309 (A16), RAF 15 dans cinq souffleries | modernes (« deux fois les anciens L_c, D_c ») | **centre de poussée** en fraction de corde depuis le bord d'attaque ; moments pris au bord d'attaque à Langley, à 1/8 in sous le bord d'attaque au Bureau of Standards | colonnes non corrigées **et** corrigées de la paroi | pages des symboles et p. 9-10 [LU] |
| Report 304 (A4), Sperry Messenger en vol | modernes, **avion complet** (traînée parasite comprise) | — | — | A21 |

**À retenir** : tout coefficient britannique d'époque ou du Report 93 doit être **doublé** ; tout moment du Report 331 doit être **ramené au quart de corde** (C_M¼ ≈ C_M,BA + C_L/4, contrôle du § 3.2) avant d'être comparé au Report 233.

## 5. Ce qu'il faudrait pour aller plus loin
1. **Numériser la planche n° 49 du Report 93** (p. 291) : L_c, D_c, centre de poussée en fonction de l'incidence. C'est le plus urgent.
2. **Identifier le SPAD mesuré au MIT** en 1918 : rapports d'essais du MIT de 1917-1918, ou bibliographie aéronautique de 1917-1921 (`spad7.md` § 9.6).
3. ***Flugsport* 1919** (A8) et **Eiffel 1919** (A5) : forme et polaires françaises ou allemandes du profil SPAD.
4. **Relevé de nervure sur un S.VII original** (Musée de l'Air, S.254, etc. ; `spad7.md` P17).
5. **τ et δ** (quatrième passe) : NACA Report 460 (NTRS 19930091108 selon le moteur de recherche, à confirmer) et NACA Report 669 (NTRS 19930091744), à télécharger en PDF ; à défaut, H. Glauert, *The Elements of Aerofoil and Airscrew Theory*, Cambridge University Press, 1926.
6. **Incidences négatives** (quatrième passe) : volume I du *Technical Report of the ACA* 1917-18 (identifiant archive.org non trouvé ; `spad7.md` § 9.6), pour le RAF 14 aux incidences négatives, l'effet biplan sur le RAF 15 et la comparaison du RAF 15 avec un profil Albatros.

## 6. Sources (consultées le 2026-10-20)

### 6.1 Lues [LU] : rapports NACA téléchargés depuis NTRS par le coordinateur (copies locales), lus en OCR et vérifiés sur les pages PDF
- **A1** — NACA, Report No. 93, *Aerodynamic Characteristics of Aerofoils*, dans le rapport annuel de la NACA, p. 257 et suivantes, 1921. https://ntrs.nasa.gov/citations/19930091156. Pages utilisées : index p. 262 et 264 ; ordonnées p. 268 (réf. 49 « Spad », 61 RAF 6) ; transformation des coefficients p. 259-260 ; planches p. 291 (réf. 49), p. 294 (réf. 61), p. 296 (réf. 72). Primaire (compilation).
- **A2** — NACA, Report No. 124, *Aerodynamic Characteristics of Aerofoils II*. https://ntrs.nasa.gov/citations/19930091187. Seulement parcouru par recherche dans l'OCR : aucune occurrence « R.A.F. » ni « Spad » trouvée. Non exploité.
- **A4** — J. W. Crowley Jr. et M. W. Green, NACA Report No. 304, *An Investigation of the Aerodynamic Characteristics of an Airplane Equipped with Several Different Sets of Wings*, publié en février 1929. https://ntrs.nasa.gov/citations/19930091373. Primaire.
- **A15** — M. M. Munk et E. W. Miller, NACA Report No. 233, *The Aerodynamic Characteristics of Seven Frequently Used Wing Sections at Full Reynolds Number*, Langley, 1927. https://ntrs.nasa.gov/citations/19930091301. Pages 73-82 (tableaux XXVI à XXX et XXXVI-XXXVII). Primaire.
- **A16** — W. S. Diehl, NACA Report No. 309, *Joint Report on Standardization Tests on N.P.L. R.A.F. 15 Airfoil Model*, Bureau of Aeronautics, rapport de juin 1928, imprimé en 1929. https://ntrs.nasa.gov/citations/19930091378. Pages 3-20. Primaire.
- **A17** — G. J. Higgins, W. S. Diehl et G. L. DeFoe, NACA Report No. 279, *Tests on Models of Three British Airplanes in the Variable Density Wind Tunnel*, 1928. https://ntrs.nasa.gov/citations/19930091347. Primaire.
- **A18** — F. A. Louden, NACA Report No. 331, *Collection of Wind-Tunnel Data on Commonly Used Wing Sections*, Bureau of Aeronautics. https://ntrs.nasa.gov/citations/19930091401. Pages 591-593 et 617 (tableau XI). Primaire (compilation corrigée).
- **A21** — Pages « Aeronautical Symbols » des Reports 271, 301, 304 et 309 : note sur les coefficients absolus, « deux fois plus grands que les anciens coefficients L_c, D_c » [LU].
- **A9** — UIUC Airfoil Coordinates Database, miroir GitHub `kennyjensen/uiuc_airfoil_database`, fichiers raf15.dat et raf6.dat [LU]. Sa version du RAF 6 diffère de celle du Report 93 (§ 3.6).

**Lues en quatrième passe**
- **A20** (statut changé) — *Technical Report of the Advisory Committee for Aeronautics for the Year 1917-18*, vol. II, « Airscrews and Full Scale Work on Aeroplanes », HMSO, Londres, 1921. archive.org `technicalreport1917aero_2`, texte OCR téléchargé par le coordinateur. **[LU-OCR]** : table des matières des deux volumes ; A22, A23 ; R&M 462 et 492 (`hispano-suiza-8a.md` H21, `spad7.md` S60).
- **A22** — H. Glauert et S. B. Gates, « Prediction of the Performance and Longitudinal Stability of an Aeroplane… », ARC R&M n° 324, mars 1917, dans A20, p. 576-580 environ (formule W = 0,0051 K_L A V², p. 578-579). Primaire.
- **A23** — Essais du NPL de profils pour hélices (intrados plat, séries A et B, cambrures de 0,075 à 0,20), dans A20, p. 277-284 environ ; auteurs lus partiellement dans l'OCR (W. L. Cowley et H. Levy, à confirmer). Primaire ; cité pour la convention des coefficients et pour un constat qualitatif aux incidences négatives.
- Prandtl, NACA TN 182 (1924), facteur σ du biplan : voir `spad7.md` S61 et § 2.4 [LU sur l'image].

### 6.2 Vues seulement en résumé de recherche ou pistes (inchangées depuis la première passe)
- **A3** — NACA Reports 182, 244 et 286 (suites de la série) [RÉSUMÉ].
- **A5** — G. Eiffel, *Résumé des principaux travaux exécutés pendant la guerre au Laboratoire aérodynamique Eiffel, 1915-1918*, E. Chiron, Paris, [1919], https://catalog.hathitrust.org/Record/001035741 [RÉSUMÉ].
- **A6** — G. Eiffel, *Nouvelles recherches sur la résistance de l'air et l'aviation…* (1911-1913) [RÉSUMÉ].
- **A7** — M. Munk et E. Hückel, « Systematische Messungen an Flügelprofilen », 1917 ; A. S. Engels (comp.) [RÉSUMÉ].
- **A8** — *Flugsport*, 1919, comparaison de profils français, signalée par https://ww1aeroinc.org/ww1-aero-drawing-index/ [RÉSUMÉ].
- **A10** — Museum of Flight, « SPAD XIII S13 » [RÉSUMÉ].
- **A11** — Wikipedia (en), « SPAD S.VII » [RÉSUMÉ].
- **A12** — Memorial Flight ou NASM (attribution incertaine) ; forum [RÉSUMÉ].
- **A13** — Pankhurst, ARC R&M 3311 [RÉSUMÉ].
- **A14** — L. Bairstow, *Applied Aerodynamics*, 1920 [PISTE].
- **A19** — L. J. Briggs et H. L. Dryden, NACA Report No. 319, *Aerodynamic Characteristics of Twenty-Four Airfoils at High Speeds*, 1929 ou 1930 (date de la page de titre mal lue par l'OCR ; 1930 selon l'index du coordinateur). Profils d'hélice. Téléchargé, mais **non exploité** dans cette passe (= H18 de `hispano-suiza-8a.md`).
- **A20** — ACA, *Technical Report* 1917-18, vol. 2 : lu en quatrième passe (§ 6.1).

## 7. Journal des décisions
- **2026-10-20 (passe 1) — Profil réel** inconnu ; comparables RAF 15 et RAF 6 proposés en confiance C.
- **2026-10-20 (passe 2)** — Numéros des rapports NACA identifiés ; rien de lu.
- **2026-10-20 (passe 3) — Profil réel.** Le Report 93 contient une section nommée **« Spad »** (réf. 49, MIT, 1918), avec ordonnées et polaire. **Retenu pour la géométrie, confiance B** : le type de SPAD n'est pas précisé, et un autre profil a pu équiper la série (A11). L'hypothèse de la première passe (« profil mince ») est confirmée pour l'épaisseur (5,4 %) ; la cambrure (environ 3,8 %, intrados creux) est **plus forte** que celle du RAF 15.
- **2026-10-20 (passe 3) — Polaires du RAF 15.** Valeurs des Reports 233 (tableau XXX) et 331 (tableau XI) lues et recoupées par trois contrôles : moments au bord d'attaque contre quart de corde, traînée de profil contre C_D, et C_L max contre le rapport C_L max / C_D min. **Confiance A pour le RAF 15 lui-même**, avec ces réserves :
  - maquette d'allongement 6 ;
  - mesures de 1926-1927 dans une soufflerie dont les auteurs reconnaissent des séries discordantes ;
  - Report 233 sans correction de paroi, celle-ci étant apportée par le Report 331.
  
  **Confiance C** pour leur emploi comme profil du SPAD.
- **2026-10-20 (passe 3) — Convention.** Les coefficients du Report 93 sont de la forme P = C·ρ·V². Il faut les **doubler** pour obtenir les C_L et C_D modernes. Les Reports 233, 309, 331 et 304 emploient déjà la convention moderne.
- **2026-10-20 (passe 3) — RAF 6.** La géométrie du Report 93 (6,8 % d'épaisseur) diffère du fichier UIUC (10 %). **Retenue : celle du Report 93.** L'ancienne ligne « RAF 6 ≈ 10 % » de la première passe est donc **caduque**.
- **2026-10-20 (passe 3) — Choix pour le M1.** Polaires calculées sur la géométrie « Spad », calées sur le RAF 15 de vraie grandeur. En repli, polaires du RAF 15 en substitut (C). Décision finale à l'aérodynamicien.
- **2026-10-21 (passe 4) — Convention britannique.** K_L = C_L/2, établi par la formule de Glauert et Gates (A22). **A** pour la convention. Elle s'applique à toutes les données britanniques d'époque et au Report 93.
- **2026-10-21 (passe 4) — Tableau des conventions** (§ 4.5) dressé pour chaque source utilisée. Point de vigilance : le C_M du Report 331 est au bord d'attaque, celui du Report 233 au quart de corde.
- **2026-10-21 (passe 4) — τ et δ.** Non trouvés. **Aucune valeur retenue** : les valeurs τ = 0,17 et δ = 0,05 restent celles de l'aérodynamicien, **non vérifiées**. Les données d'allongement infini du § 4.2 restent en correction elliptique (τ = δ = 0), comme dans le Report 331. Sensibilité chiffrée pour aider la décision (§ 4.2).
- **2026-10-21 (passe 4) — Incidences négatives.** Le Report 233 descend à −4,5° (et non −1,5°) ; le point le plus négatif mesuré pour le RAF 15 est −6° (Report 309, bas Reynolds), sans décrochage. **Décrochage négatif : non sourcé.** La branche au-delà de −6° sera une extrapolation `E:`.
