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

**Courbe d'époque : non trouvée.**

Pistes précises :
- Un essai du département moteurs du Royal Aircraft Establishment, daté du **8 août 1916**, sur un Hispano-Suiza de 150 hp. Sa courbe serait reproduite en **figure 10** de l'**ARC R&M n° 586** (juillet 1921). Le même participant du forum note qu'elle donne une puissance maximale un peu plus faible à 2 000 tr/min, mais plus de puissance et un couple maximal plus élevé vers 1 200 à 1 400 tr/min. La référence de comparaison n'est pas claire dans le résumé (H6 [RÉSUMÉ]).
- R&M 586 doit se trouver dans les archives ARC numérisées par Cranfield (AERADE, `reports.aerade.cranfield.ac.uk`), inaccessibles depuis le conteneur.

**Conséquence pour le modèle de vol** : en attendant R&M 586, la forme de la courbe `power_curve` ne peut pas être sourcée. Seul le point nominal (150 ch à 2 000 tr/min, 180 ch à 2 100 tr/min, confiance C) l'est. Toute autre forme est une estimation `E:` à justifier par l'aérodynamicien. Cette fiche n'en propose pas.

Indice d'usage : dans une lettre du 8 septembre 1916, Guynemer décrit son approche d'un groupe ennemi « à 1800 » au compteur (« at 1800 on the speedometer » dans la traduction anglaise ; S1 de `spad7.md` [LU]). Il s'agit **peut-être** du compte-tours (1 800 tr/min en descente vers l'ennemi). Interprétation non vérifiée, inutilisable comme donnée.

## 4. Comportement en altitude
- **Moteur atmosphérique** : aucune source consultée n'attribue un compresseur aux 8Aa et 8Ab de série. Un « Hispano-Suiza suralimenté de 190 ch » essayé sur un SPAD 7 en 1917 est cité comme un cas particulier (résumé de recherche ; page source probablement S5 ou S12 de `spad7.md`, attribution incertaine). Confiance C.
- **Vocabulaire à surveiller** : une hélice Gallia série 3 porte la marque « Sur » pour un Hispano de 150 ch dit « surcompressé » (H8 [RÉSUMÉ]). En français d'époque, « surcomprimé » désigne en général un **taux de compression augmenté** (comme le 8Ab), pas une suralimentation. La traduction anglaise *supercharged* du forum est donc douteuse. À vérifier.
- **Carburateur** : un participant du forum suggère que les premiers 150 ch n'avaient pas de correction altimétrique du mélange, ce qui ferait varier la montée au-dessus de 2 000 m selon le carburateur monté (H7 [RÉSUMÉ]). Hypothèse non vérifiée.
- **Modèle retenu par défaut** : relation de Gagg-Farrar (règle du projet, skill `simulation-vol`). Confiance **C** : aucune mesure d'époque de la puissance en altitude n'a été trouvée.

## 5. Consommation
- Consommation spécifique (g/ch/h) : **non trouvée**.
- Contrôle de cohérence seulement (C, données faibles) : 135 L de carburant pour 1,5 à 2 h d'autonomie (`spad7.md` S13 [RÉSUMÉ]) donnent environ 68 à 90 L/h en moyenne, à un régime non précisé. Ne pas s'en servir comme donnée de modèle.
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
- **2026-10-20 — Sens de rotation.** **Horaire vu du pilote, confiance C** : déduction concordante de H1, H12, H13 et de l'essai de McCook (§ 7.3). À confirmer par une photographie ou une notice d'époque.
