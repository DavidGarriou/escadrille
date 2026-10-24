# Plan — Souffle d'hélice sur les ailes et l'empennage

- Statut : **validé** par le joueur le 2026-10-24
- Contexte : première tâche « prioritaire » hors plan de `m1-ecarts-spad7.md`. Le tourbillon du souffle choisit le côté du départ au décrochage (`spad7-calculs.md` § 9), et le modèle le croit plus fort que le couple de l'hélice. À régler avant toute conclusion sur le décrochage ou les effets de couple, et avant l'embardée au décollage du M2.
- Règle inchangée : **aucun coefficient réglé sur les cibles du SPAD**. On étalonne sur un objet mesuré (ici une maquette motorisée du NACA), on source, on teste, l'aérodynamicien relit, et on rapporte l'écart qui reste.

## Ce que disent les sources (lues le 2026-10-24)

| Source | Contenu | Ce qu'on en retient |
|---|---|---|
| Smelt et Davies, R&M 1788 (1937) | Introuvable en ligne (absent d'AERADE). Sa formule est citée par R&M 2732, éq. 6 : ΔC_L = (S_souffle / S) × s × C_L0, avec 1 + s le rapport de vitesse dans le souffle. | Hausse de **portance** due à la vitesse du souffle : environ la **moitié** de la théorie des bandes. R&M 1788 ne traite **pas** le tourbillon : l'attribution faite au § 9 est corrigée. |
| Squire et Chester, R&M 2368, partie I (1945) | Aile infinie traversée par un jet plus rapide que l'écoulement, ligne portante avec les tourbillons images au bord du jet. | Tableau 2 : rapport « hausse de portance calculée / théorie des bandes » de 0,13 à 0,89, selon la vitesse du jet (V/v) et sa largeur rapportée à la corde (μ = 8πR / a₀c). Ce rapport tend vers 1 quand le jet ralentit vers la vitesse de l'écoulement (Koning). |
| R&M 2368, partie II | Aile infinie dans un jet en rotation de même vitesse. | Le moment de roulis de l'aile **égale** celui de la théorie des bandes, et dépasse le couple de l'hélice dès que la corde dépasse 0,375 D. Les auteurs jugent ce résultat « surprenant » et leur théorie peu fiable au-delà d'une corde de D/3, le cas courant. L'aile ne récupère que **28 à 37 %** de l'énergie de rotation. |
| Purser et Spear, NACA TN 1146 (1946) | Maquette motorisée au 1/5, aile basse, D/c = 1,7 (SPAD : 2,0), forte traction (T_c′ = 1,25), incidence 0°, trois calages d'hélice. | **Mesure** : sans empennage, le roulis net vaut **50 à 60 % du couple**. L'aile et le fuselage n'en reprennent donc que **40 à 50 %**. Derrière l'aile, le NACA suppose le tourbillon réduit de moitié ; les forces de dérive calculées ainsi tombent à 10-15 % des mesures. |
| Biermann et Herman, NACA Report 747 (1942) | Hélices simples et contrarotatives, avec et sans aile dans le souffle. | Une aile dans le souffle récupère environ la **moitié** de ce que gagne la contrarotation : elle « enlève environ la moitié de la rotation ». |
| Stüper, NACA TM 874 (1938) | Répartitions de portance mesurées dans un souffle d'hélice. | La rotation domine l'asymétrie de portance jusqu'à 8° d'incidence. Dans le jet, l'écoulement reste attaché au-delà du maximum de portance ; le décollement part des bords du jet. |
| R&M 2732, R&M 2701 | Stabilité longitudinale avec hélice. | Seule l'éq. 6 de R&M 2732 sert ici. |

## Ce que fait le modèle aujourd'hui

Mesure du 2026-10-24 sur le SPAD équilibré, avec un test temporaire non commité. Q est le couple de l'hélice. V/v est le rapport entre la vitesse de l'avion et celle du souffle au droit des ailes.

| Vol | V/v | Roulis du tourbillon | dont ailes | dont empennage | Poussée récupérée sur le tourbillon / énergie de rotation du souffle | Portance ajoutée par le souffle axial |
|---|---|---|---|---|---|---|
| Vmax, 2 000 m | 0,92 | 1,57 Q | 1,00 Q | 0,57 Q | 2,07 / 1,37 kW (**151 %**) | +5,2 % |
| Meilleure montée, 0 m | 0,77 | 1,46 Q | 0,86 Q | 0,60 Q | 3,15 / 2,98 kW (**106 %**) | +11,0 % |
| Plané, 33 m/s | 1,00 | 1,67 Q | 1,08 Q | 0,60 Q | — | +0,1 % |
| Décrochage, plein gaz | 0,59 | 1,18 Q | 0,51 Q | 0,68 Q | 1,69 / 3,62 kW (47 %) | +15,8 % |
| Décrochage, ralenti | 0,98 | 1,12 Q | 0,50 Q | 0,62 Q | — | +0,8 % |

Le niveau du tourbillon lui-même est juste : son énergie vaut 2,5 % de la puissance sur l'arbre en montée, contre « environ 3 % » selon R&M 2368.

Trois défauts :
1. **Énergie** : la cellule reprend au tourbillon plus de puissance qu'il n'en porte (151 % à Vmax, 106 % en montée). C'est physiquement impossible : la théorie donne au plus 37 % pour une aile, la mesure « environ la moitié ». Conséquence directe : l'hélice installée est trop efficace, de 1,5 à 2,5 % de la puissance sur l'arbre.
2. **L'empennage voit tout le tourbillon**, comme si les ailes n'en prenaient rien. Les deux sources du NACA disent : la moitié.
3. **Les ailes répondent au souffle comme en théorie des bandes.** La hausse de portance due à la vitesse du souffle est trop forte : de 7 % de cette hausse à Vmax jusqu'à 35 % au décrochage plein gaz, selon le tableau 2 de R&M 2368. La réponse au tourbillon l'est sans doute aussi sous forte traction : le TN 1146 mesure 40 à 50 % du couple. Pour une maquette semblable, notre modèle donnerait de l'ordre de 100 % ; c'est une estimation grossière, à confirmer à l'étape 1.

## Étapes

1. **Avion de comparaison : la maquette du TN 1146**, sur le modèle du Messenger (`tools/fdm_lab/comparison/`).
   - Géométrie : tableau I et figure 1 (aile basse de 7,51 ft, allongement 6, effilement 2). Hélice de 2,27 ft, à la traction et au couple mesurés (figure 7), pour les trois calages.
   - Un test de `fdm_lab` calcule le roulis sans empennage à incidence nulle et le compare à la mesure (50 à 60 % du couple).
   - Le FDM ne change pas à cette étape : elle mesure l'écart actuel.
2. **Largeur du souffle sur les bandes portantes** (R&M 2368, tableau 2) :
   - un facteur η(V/v, μ), interpolé dans le tableau, réduit ce que le souffle ajoute à la portance de chaque bande immergée. Il s'applique à la hausse due à la vitesse, et à l'incidence due au tourbillon ;
   - η tend vers 1 quand le souffle ralentit vers la vitesse de l'avion (limite de Koning), si bien qu'au ralenti rien ne change ;
   - les éléments de traînée et le fuselage gardent tout le souffle, et l'empennage reçoit la même correction que les ailes ;
   - appliquer au tourbillon le facteur calculé pour un jet sans rotation est une hypothèse (E:). La comparaison avec le TN 1146 la vérifie : la maquette doit retomber dans la plage mesurée.
   - S'il reste un écart, on le rapporte. Le suspect déjà connu est la limite documentée dans `aero/surface.rs` : une charge antisymétrique répond trop fort, et l'amortissement en roulis est surestimé d'environ 27 %. On n'ajoute aucun facteur sans source ni accord de l'aérodynamicien.
3. **Tourbillon derrière les ailes** : divisé par deux pour tout ce qui est en arrière des ailes (Report 747 et TN 1146 ; C:).
   - Ce sera une donnée de l'avion, avec sa source : un triplan ou un avion à hélice propulsive auront une autre valeur.
   - Version plus physique, laissée pour plus tard : une aile décrochée reprend moins de tourbillon. Elle demanderait un bilan de moment cinétique calculé à chaque pas.
4. **Vitesse induite de Froude pour une traction négative**, rattachée à ce plan : v_i = ½(√(V² + 2T/ρA) − V), prolongée sous T = 0 et bornée à −V/2, où commence le régime de sillage turbulent (E:). L'élargissement du tube est borné lui aussi.
   - Effet : derrière une hélice qui freine (piqué, ralenti à grande vitesse), le souffle est plus lent. Le centre de l'aile porte un peu moins et l'empennage est un peu moins efficace.
5. **Tests**, en plus de ceux des étapes 1 à 4 :
   - η tend vers 1 quand v tend vers V, varie de façon monotone et continue, et le bord du jet reste lisse ;
   - **nouvel invariant d'énergie** : la puissance reprise au tourbillon ne dépasse jamais l'énergie de rotation du souffle (SPAD et biplan d'essai, plusieurs vols) ;
   - le souffle reste continu en T = 0 et la simulation déterministe ;
   - les tests existants sont adaptés ;
   - la maquette du TN 1146 retombe dans la plage mesurée (test calqué sur celui du Messenger).
6. **Essai et rapport** :
   - `/essai-en-vol spad7` ;
   - le § 9 est refait, en faisant varier les termes dans les plages des sources ; le côté du départ ne devient significatif que s'il résiste à cette variation ;
   - revue de l'aérodynamicien ;
   - documentation : `modele-aero.md`, `format-avion.md`, nouveau § 10 de `spad7-calculs.md` ;
   - rapport au joueur, sans réglage.

Chaque étape : `just check`, `just test`, carnet de bord, commit et push.

## Résultats

### Étape 1 (2026-10-24) : la maquette du TN 1146 mesure l'écart

- Maquette modélisée dans `tools/fdm_lab/comparison/aircraft/naca_tn1146`. Commande : `fdm_lab --assets tools/fdm_lab/comparison souffle`.
  - Géométrie : tableau I et figure 1. Aile basse en W inversé (−0,73° puis 7,75°), plan de corde 4,3 à 4,5 in sous l'axe de traction, quart de corde à 0,65 D derrière l'hélice.
  - Profil : NACA 66(215)-216, l'équivalent du 66,2-216, relu sur l'image du NACA Report 824 (p. 236) au plus petit nombre de Reynolds de la page, 2,9 × 10⁶. La grille a été repérée par analyse d'image ; le scan est tourné de 0,7°.
  - Hélice : une table par calage, tirée de la figure 7 et passant exactement par le point mesuré T_c′ = 1,25.
  - Vérification de la lecture : le roulis « calculé » du NACA (−Q_c′/2) retombe exactement sur leur courbe de la figure 13.
- Mesure (figure 13, points sans empennage) contre le modèle actuel :

| Calage | Repris par l'aile, mesure | Repris par l'aile, modèle | Rapport |
|---|---|---|---|
| 15° | 47,5 à 50 % | 71 % | 1,4 à 1,5 |
| 25° | 39 à 45 % | 70 % | 1,6 à 1,8 |
| 35° | 43,5 à 46 % | 57 % | 1,2 à 1,3 |

- À 35°, le tourbillon donne des incidences de 15 à 20° près du moyeu : les bandes décrochent, ce qui limite déjà la réponse du modèle.
- Sensibilité :
  - aile 0,3 in plus haute ou plus basse : ±3 à 4 points ;
  - pente du profil réduite de 10 %, pour un Reynolds plus bas : −6 points ;
  - même en cumulant les deux dans le sens favorable, le modèle reste au-dessus de la mesure (51 à 61 %).
- Réserves :
  - profil relu à 2,9 × 10⁶, alors que la maquette volait à 0,5 × 10⁶ (0,8 × 10⁶ effectif) ;
  - T_c′ « effectif » (traction moins traînée du souffle) ;
  - fuselage non modélisé.
- **Conclusion** : la surestimation est confirmée, d'un facteur 1,2 à 1,8. C'est moins que le facteur 2 estimé dans le plan.

## Effets attendus sur le SPAD (ordres de grandeur, à vérifier)

- **Montée plus lente, d'environ 5 %**, parce que la cellule reprend moins d'énergie au tourbillon.
  - Aujourd'hui, le modèle met 7 à 11 % de temps en moins que S46 pour monter, et monte 4 à 6 % trop vite par rapport au rapport britannique n° 96 (vu en résumé seulement). La correction irait donc **dans le sens des deux cibles**.
  - **Biais déclaré** : je le sais avant d'écrire le code. Les facteurs viennent des sources (tableau 2, Report 747, TN 1146) et de la maquette de comparaison, jamais des temps de montée du SPAD. Si l'essai tombe juste, ce ne sera pas une preuve ; s'il dépasse, on le rapporte sans retoucher.
- **Vitesse maximale** : environ −0,5 %.
- **Décrochage plein gaz** : environ 2 à 3 % plus rapide, parce que le souffle ajoute moins de portance.
- **Équilibre en roulis** : aujourd'hui, le tourbillon annule presque la réaction du couple (somme presque nulle à Vmax). Après correction, il resterait un roulis net vers la gauche, de l'ordre d'un tiers à la moitié du couple, à tenir à l'aileron en montée.
  - Question pour l'historien, pas une cible : que disent les pilotes de SPAD Hispano du couple, et les ailes étaient-elles réglées de façon dissymétrique pour le compenser ?
- **Côté du départ au décrochage** : le § 9 est à refaire. Au ralenti, la correction de largeur ne joue presque pas ; seul compte le tourbillon divisé par deux sur l'empennage. Il se peut donc que le côté reste non significatif.

## Hors de ce plan

- **Direction du souffle** : le modèle le fait partir tout droit le long de l'axe de traction (E:). En réalité, l'incidence et la déflexion des ailes le dévient (Katzoff, NACA Report 690 ; Stüper, TM 874). C'est le prochain candidat, car cela décide si l'empennage est dans le souffle au décrochage.
- **Ligne portante à matrice d'influence** (charges antisymétriques, amortissement en roulis) : limite connue, cible déjà notée dans `aero/surface.rs`.
- **Répartition du tourbillon** : tourbillon libre avec un cœur de 0,25 R (E:), à comparer au calcul par éléments de pale du TN 1146 (figure 11).

## Décision demandée au joueur

Valider ce plan, ou le modifier. Point à savoir avant de valider : la correction la plus sûre (l'énergie) rendra probablement le SPAD plus lent en montée d'environ 5 %, ce qui le rapprocherait de S46 et du rapport n° 96. Les facteurs ne seront pas choisis pour cela ; le biais est déclaré plus haut.
