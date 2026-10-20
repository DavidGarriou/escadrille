# Hispano-Suiza 8A (8Aa, 8Ab) et hélice du SPAD VII — fiche de recherche

- Identifiant : `hispano-suiza-8a`
- Dernière mise à jour : 2026-10-20
- État : **en cours** (première passe ; aucune courbe puissance/régime trouvée)
- Avion concerné : [`spad7.md`](spad7.md) (SPAD S.VII C.1)
- Méthode et statuts des sources (**[LU]**, **[RÉSUMÉ]**, **[PISTE]**) : voir `spad7.md` § 0. En bref, seules des pages GitHub ont pu être lues ; tout le reste n'a été vu qu'en résumé de moteur de recherche.

## 1. Identification et variantes

| Variante | Puissance nominale | Régime | Particularités | Montage | Sources |
|---|---|---|---|---|---|
| 8A (premier type) | 140 hp | 1 900 tr/min | Présenté au ministère de la Guerre en février 1915. Endurance de 15 h à pleine puissance à Chalais-Meudon, puis essai de **50 h** sans interruption réussi le 21 juillet 1915 | — | H1, H2 [RÉSUMÉ] |
| **8Aa** | **150 ch** | 2 000 tr/min (H1) ; 1 700 tr/min (H1, fiche technique d'un miroir) | Taux de compression **4,7** | SPAD S.VII « 150 ch » | H1, H2 [RÉSUMÉ] |
| **8Ab** | **180 ch** | 2 100 tr/min (H1, H5) ; 1 800 tr/min (H4) | Taux de compression porté à **5,3**. Remplace le 8Aa sur les S.VII à partir du début de 1917. Carburateur Zenith (H5). **1 533** construits en 1917-1918, hors licences (H5) | SPAD S.VII « 180 ch » | H1, H4, H5 [RÉSUMÉ] |
| 8B, 8Ba, 8Be (à réducteur) | 200 à 220 ch | — | Réducteur qui **inverse le sens de rotation** de l'hélice (§ 7.3) | SPAD XIII | H1, H12 [RÉSUMÉ] ; S38 de `spad7.md` (notice du XIII « 200 HP ») |
| 8C, 8Cb (à réducteur, moteur-canon) | — | — | Canon tirant à travers l'arbre d'hélice | SPAD XII | H1 [RÉSUMÉ] |
| Wright-Martin « A » (licence américaine) | 150 hp | **1 450 tr/min** | Puissance américaine, autre définition : **ne pas confondre** avec le 8Aa | — | H3 [RÉSUMÉ] |
| Wright-Hispano E (licence américaine) | 180 hp | — | Moteur du S.VII de Tallman (N4727V) | — | `spad7.md` S31 [RÉSUMÉ] |

Licence britannique : Wolseley à Birmingham (H1 [RÉSUMÉ]). Son dérivé à entraînement direct, le Viper, a équipé le S.E.5a (H13 [RÉSUMÉ]).

**Unités.** Les puissances françaises sont en **chevaux-vapeur métriques** (ch, PS). H5 donne pour le 8Ab 132,4 kW à 2 100 tr/min. Or 180 ch × 0,7355 = 132,4 kW (C), soit 177,5 hp impériaux (C). Dans `aircraft.ron`, écrire `Ps(150.0)` et `Ps(180.0)`, pas `Hp(..)`. Les valeurs américaines (H3) sont en hp impériaux : 150 hp = 112 kW (C), ce que donne H3.

## 2. Caractéristiques

| Grandeur | Valeur | Unité d'origine | Source | Confiance | Remarque |
|---|---|---|---|---|---|
| Architecture | V8, refroidissement liquide | — | H3 [RÉSUMÉ] | B | |
| Alésage × course | 120 × 130 mm | mm | H1, H2, H3 [RÉSUMÉ] | B | Trois sources concordantes |
| Cylindrée | 11,76 L (11 760 cm³) | L | H1, H2 [RÉSUMÉ] ; 11,78 L (H3) | B | Contrôle (C) : π/4 × 1,2² × 1,3 × 8 = 11,76 L |
| Taux de compression | 4,7 (8Aa) ; 5,3 (8Ab) | — | H1, H2, H5 [RÉSUMÉ] | B | |
| Carburant | « indice d'octane 80 » | — | H5 [RÉSUMÉ] | **D** | L'indice d'octane n'existait pas en 1917 : mention suspecte |

## 3. Puissance en fonction du régime

### 3.1 Mesure d'époque : NACA Report 46 (V. R. Gage, Bureau of Standards) [LU]
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
- **Régime nominal** : 1 700 tr/min dans ce document américain, contre 2 100 tr/min dans les sources secondaires sur le 8Ab français (H1, H5). La puissance continue de croître jusqu'à 2 100 tr/min (tableau ci-dessus). La « cote » dépend donc de la convention ; voir le journal.
- Le **8Aa** (compression 4,7) n'a pas de mesure équivalente. La courbe ci-dessus n'est transposable qu'avec un facteur d'échelle de puissance (C).

### 3.2 Pistes restantes (inchangées)
- Un essai du département moteurs du Royal Aircraft Establishment, daté du **8 août 1916**, sur un Hispano-Suiza de 150 hp. Sa courbe serait reproduite en **figure 10** de l'**ARC R&M n° 586** (juillet 1921). Le même participant du forum note qu'elle donne une puissance maximale un peu plus faible à 2 000 tr/min, mais plus de puissance et un couple maximal plus élevé vers 1 200 à 1 400 tr/min. La référence de comparaison n'est pas claire dans le résumé (H6 [RÉSUMÉ]).
- R&M 586 doit se trouver dans les archives ARC numérisées par Cranfield (AERADE, `reports.aerade.cranfield.ac.uk`), inaccessibles depuis le conteneur. Une autre voie : le volume annuel *Technical Report of the Aeronautical Research Committee* de 1920-21 ou de 1921-22 sur archive.org (série dont seul le volume `technicalreport1917aero_2` a été repéré). Identifiant à trouver, contenu non lu.
- Un participant du même forum dit utiliser cette courbe dans un modèle de performances du « Spad 7 C1 » à **740 kg**, réglé d'après des données de l'Air Board (H6 [RÉSUMÉ]). Ce n'est pas une source de chiffre, mais une piste vers des données de l'Air Board.

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

**Report 108** (Gage, « Some Factors of Airplane Engine Performance » ; essais en caisson d'altitude d'un Liberty et de trois Hispano-Suiza) (p. 649-651 et conclusions)
- À conditions constantes, la puissance indiquée est proportionnelle à la densité de la charge au début de la compression.
- La puissance décroît **un peu moins vite que la densité** quand c'est la température qui augmente, car le remplissage s'améliore avec la température.
- Le couple de frottement varie linéairement avec le régime et avec la densité ; le pompage en représente environ la moitié.
- Le réglage altimétrique des carburateurs de série est **insuffisant au-dessus d'environ 45 cm de mercure (≈ 15 000 ft)**.

**Report 103** (Sparrow et White, « Performance of a 300-Horsepower Hispano-Suiza Airplane Engine » ; modèle « H » de 300 hp, autre cylindrée) (p. 581-582)
- Au-dessous de 20 000 ft, la puissance au frein suit une **droite en fonction de la densité de l'air au carburateur**.
- À 1 800 tr/min et 0,040 lb/ft³ (contre 0,075 au sol), la puissance au frein vaut **environ 42 %** de celle du sol, et la puissance indiquée **environ 47 %**.
- Le mélange devient trop riche au-delà de 20 000 ft.

### 4.2 Comparaison avec la relation de Gagg-Farrar (C, calculs faits ici)
La relation du projet, g(σ) = σ − (1 − σ)/7,55 (`.claude/skills/simulation-vol/modele-aero.md`, section « Moteurs »), a exactement la forme qu'on obtient avec une **puissance indiquée proportionnelle à σ et un frottement constant** : g(σ) = σ − k(1 − σ), où k = frottement / puissance au frein au sol. Gagg-Farrar revient à **k = 1/7,55 ≈ 0,132**.

- **Report 46** (Hispano de 180 hp) : k = 14,7/179 ≈ 0,082 (1 500 tr/min), 16/203 ≈ 0,079 (1 700), 20/215 ≈ 0,094 (1 900), 24,8/228 ≈ 0,109 (2 100). Ce moteur **perd moins** que Gagg-Farrar ne le prévoit. Exemple à 1 700 tr/min et σ ≈ 0,48 (rapport des puissances indiquées « 20 000 ft » / sol, 105/218) : mesure 89/203 = 0,438 ; Gagg-Farrar 0,413 (environ −6 %).
- **Report 103** (Hispano « H » de 300 hp) : à σ ≈ 0,040/0,075 = 0,533, la mesure donne 0,42, contre 0,471 pour Gagg-Farrar. Ce moteur **perd plus** (environ +11 % d'écart).
- **Conclusion** : Gagg-Farrar se situe **entre** les deux moteurs Hispano mesurés. Pour un 8Ab, le Report 46 est le plus pertinent (même cylindrée et même compression).
- **Proposition** (à valider par l'aérodynamicien) : g(σ) = σ − k(1 − σ), avec k ≈ 0,08 à 0,11 selon le régime (Report 46). Confiance **B** pour un 8Ab, **A** pour le moteur essayé. À défaut, conserver Gagg-Farrar (C), en sachant qu'il est un peu pessimiste pour ce moteur, d'environ 5 % à σ = 0,5.
- **Ce qui manque** : l'origine précise de la relation « Gagg-Farrar » citée par la skill. Les rapports lus sont de **V. R. Gage** (1920-1921), à ne pas confondre avec « Gagg ». La référence Gagg et Farrar n'a été ni trouvée ni lue.

## 5. Consommation
- **Report 46, tableau 1 (p. 550)** [LU], en livres de carburant par hp au frein et par heure :
  - **au sol** : 0,515 (1 300), 0,500 (1 500), 0,503 (1 700), 0,512 (1 900), 0,525 (2 100) ;
  - **à « 30 000 ft »** : 0,649, 0,635, 0,610, 0,650, 0,650 ;
  - rapportée à la puissance indiquée : environ 0,46 à 0,49 lb/(hp·h) à toutes les altitudes.
  
  Confiance A pour le moteur essayé, B pour un 8Ab.
- **Conversion** (C) : 0,503 lb/(hp·h) ≈ 0,228 kg/(hp·h), soit **≈ 0,225 kg/(ch·h)**.
- **Contrôle** (C, données faibles) : à 180 ch, environ 40 kg/h, soit environ 56 L/h avec une masse volumique de 0,72 kg/L (valeur du format de données, non sourcée ici). Les 135 L du SPAD (`spad7.md` S13 [RÉSUMÉ]) donneraient environ 2,4 h à pleine puissance, **plus** que l'autonomie de 1,5 à 2 h citée par la même source. Ces chiffres d'autonomie et de contenance restent à vérifier.
- **Report 46, résumé** : rendement thermique au frein d'environ **24 %** au sol, en baisse vers 20 % en altitude (p. 525).
- Huile : non trouvé.

## 6. Masse

| Grandeur | Valeur | Source | Confiance | Remarque |
|---|---|---|---|---|
| 8Aa, avec huile et eau | 195 kg | H2 [RÉSUMÉ] | C | |
| Wright-Martin « A », à sec | 202 kg (445 lb) | H3 [RÉSUMÉ] | C | Moteur américain |
| 8Aa (tableau comparatif) | 215 kg | H1, miroir [RÉSUMÉ] | D | Tableau tronqué ; définition inconnue |
| 8Ab (tableau comparatif) | 230 kg | H1, miroir [RÉSUMÉ] | D | idem |

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

Témoignage d'époque : le 28 décembre 1916, Guynemer réclame à Béchereau « une meilleure hélice » pour le SPAD de 150 ch (`spad7.md` S1 [LU]). Il trouvait donc les hélices du début de série insuffisantes.

### 7.2 Nombre de pales, matériau
Bipale en bois à pas fixe, d'après les fiches de collectionneurs (H8, H11 [RÉSUMÉ]). Confiance B pour le type ; l'essence de bois n'a pas été relevée.

### 7.3 Sens de rotation (vu du siège du pilote)

**Retenu : sens horaire vu du pilote, confiance C.** Ce n'est pas une mesure ; c'est une déduction tirée de quatre sources non primaires qui concordent :
1. Le Hispano 8B à **réducteur** fait tourner l'hélice dans le sens horaire vu de **face**, ce qu'on appelle une hélice tractive « à gauche » (*left-hand tractor*) (H1, miroir fandom [RÉSUMÉ]).
2. L'hélice du SPAD XIII (8Be à réducteur) est « à gauche » et **tourne en sens inverse** de celle du S.VII à entraînement direct (H12 [RÉSUMÉ]).
3. Sur le S.E.5a, le Hispano à réducteur tourne dans le sens **antihoraire vu du pilote**, et le Viper à entraînement direct dans le sens **horaire vu du pilote** (H13 [RÉSUMÉ], revues de maquettes concordantes ; la formulation de Wikipedia diverge, sans doute par confusion du point de vue).
4. Recoupement physique : le pilote de McCook doit mettre « beaucoup de pied à gauche » à cause du couple sur le SPAD XIII (`spad7.md` S15 [RÉSUMÉ]). C'est cohérent avec une hélice antihoraire vue du pilote sur le XIII (C), donc avec une hélice horaire sur le VII.

**Attention aux conventions** : selon un forum de collectionneurs, les sources britanniques et françaises décrivent la rotation vue du pilote, les sources allemandes vue de face (H8 [RÉSUMÉ]). Pour confirmer, il faudrait une photographie d'époque d'un S.VII dont le bord d'attaque des pales est lisible, ou la notice d'un S.VII. Seule une notice du **XIII** a été repérée (S38 de `spad7.md`).

**Effets attendus en vol** (C, physique élémentaire, à faire émerger du modèle et non à scripter) : à pleine puissance et à basse vitesse, tendance à partir à gauche, contrée au pied droit ; réaction de couple en roulis vers la gauche.

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

**Pistes non consultées**
- ARC R&M n° 586 (juillet 1921), figure 10 : courbe puissance/régime du Hispano 150, d'après l'essai du RAE du 8 août 1916 (archives Cranfield AERADE).
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
