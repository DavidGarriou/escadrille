# Plan — Partie fixe de la dérive du SPAD VII, sous le plan fixe

- Statut : **validé** par le joueur le 2026-10-26, avec l'option (a) pour le sillage du fuselage (aucun facteur, écart documenté).
- Contexte : relecture du dessin de 1917 du 2026-10-26 (`docs/recherche/spad7-calculs.md` § 3.1).
  - **Ce que montre le dessin** : sous le plan fixe, un petit panneau fixe fermé d'environ 0,08 m² entre l'étambot (x −4,47) et l'axe du gouvernail (x −4,70).
  - **Ce que fait le modèle de vol** : son panneau inférieur de dérive est tout entier gouvernail (corde 0,53 m, de x −4,75 à −5,28), et rien ne remplit l'espace entre l'étambot et ce panneau.
  - **Écart de surface** : l'empennage vertical fait 0,98 m² sur le dessin, contre 0,907 m² dans le modèle, soit −7 %. La `reference_area` de 0,90 m² (« fin 0.37 + rudder 0.53 ») oublie elle aussi cette partie : le contrôle de surface à ±2 % ne pouvait pas le voir.
- **Hors de ce plan**, décisions à part :
  - ailerons à 29 % de la corde sur le dessin contre 32 % ;
  - répartition du gouvernail équivalent au-dessus du plan fixe ;
  - moyeu à x 0,83 contre 0,78.

## Ce qui change

Les données seulement, sans code du modèle de vol à écrire. Tout se passe dans le panneau inférieur de la dérive (`panels[0]` de `fin` dans `assets/aircraft/spad7/aircraft.ron`).

| Grandeur | Aujourd'hui | Proposé (à confirmer sur le contour numérisé) |
|---|---|---|
| Bord d'attaque | x −4,75 | **x −4,47** (étambot du dessin) |
| Bord de fuite | x −5,28 | bord de fuite dessiné du gouvernail, vers −5,25 |
| Corde | 0,53 m | environ 0,78 m |
| Quart de corde | x −4,8825 | environ −4,665 |
| Part de gouvernail (`chord_ratio`) | 1,0 | environ 0,70, pour que la charnière des bandes tombe sur l'axe dessiné (−4,70) |
| Surface du panneau | 0,207 m² | environ 0,30 m², dont gouvernail environ 0,21 (inchangé) et partie fixe environ 0,09 |
| `reference_area` de la dérive | 0,90 m² | surface dessinée, tout l'empennage vertical, partie fixe comprise |
| `aspect_ratio` (E: 1,5, plaque partielle de Hemke) et `span_efficiency` (C: 0,61, pente DATCOM de 1,9 par radian) | — | **recalculés** pour la nouvelle géométrie : envergure inchangée, surface plus grande |
| `hinge_m` du gouvernail | x −4,70 | inchangé ; à environ 4 mm de la charnière des bandes, dans le garde-fou |

- Toutes les valeurs viennent du contour numérisé par l'aérodynamicien (scripts de la relecture). Chacune garde son origine `C:` dans le fichier.
- Le panneau supérieur ne change pas.

### Sillage du fuselage

Le panneau ajouté est derrière le fuselage. Il y reçoit moins de pression dynamique qu'en écoulement libre : c'est le η_v de DATCOM, de l'ordre de 0,8 à 0,9 pour une dérive basse.

Le modèle de vol n'a pas de facteur de pression dynamique par panneau. Deux options :
- **(a) Aucun facteur** (recommandé pour ce plan) : l'effet du panneau est surestimé, d'au plus 10 à 20 % de son propre apport, soit environ 1 % de la stabilité de route. L'écart est documenté dans le fichier de données. On ne compense pas en touchant à `span_efficiency`, ce qui serait un réglage caché.
- **(b) Une nouvelle fonction du modèle de vol** : un facteur de pression dynamique par panneau, sourcé (DATCOM, η_v). C'est un chantier à part, avec tests et revue. Il ne vaut la peine que si d'autres avions en ont besoin (dérives basses, empennages masqués).

## Ce qu'on attend, et qu'on mesurera

| Grandeur (rapport `just essai spad7`) | Attendu | Pourquoi |
|---|---|---|
| Stabilité de route N_v (+111,5 N·m/(m/s)) | en hausse, de l'ordre de +5 à +9 % | surface ajoutée à environ 4,4 m derrière le centre de gravité |
| Effet de dièdre L_v (−18,0) | presque inchangé | panneau près de la hauteur du centre de gravité |
| Roulis hollandais (période 3,46 s, amortissement 0,193) | un peu plus rapide et plus amorti | |
| Spirale (×2 en 17,5 s) | un peu plus divergente | plus de stabilité de route à dièdre égal ; à comparer au trait « spirale peu divergente » (T8, S5), **sans rien régler pour le retrouver** |
| Vrille (établie, sortie franche) | sortie au moins aussi franche | dans une vraie vrille, c'est la surface sous le plan fixe qui reste efficace, celle au-dessus étant masquée. Le modèle de vol ne simule pas ce masquage : il ne verra que la surface ajoutée |
| Trim : dérapage 0,79°, direction −0,00 | léger changement | le panneau est dans le souffle de l'hélice et sa rotation |
| Performances (Vmax, montée, plafond, décrochage) | inchangées à moins de 1 % | traînée de profil d'environ 0,1 m² de surface mouillée de plus |
| Départ au décrochage | à observer | déjà noté comme non significatif (`spad7-calculs.md` § 9) |

## Étapes

1. **Géométrie** (aérodynamicien) : sur le contour numérisé, pour le nouveau panneau inférieur :
   - bord d'attaque, bord de fuite, corde et quart de corde ;
   - `chord_ratio` du gouvernail, et la vérification que sa charnière tombe sur l'axe dessiné ;
   - `reference_area` de la dérive ;
   - `aspect_ratio` et `span_efficiency`, recalculés avec la même méthode qu'au § 5 de `spad7-calculs.md` ;
   - le nombre de bandes du panneau : convergence dans le souffle, test `the_swirl_left_converges_with_the_strips`.
2. **Campagne avant** : `just essai spad7` sur l'état actuel ; le rapport sert de référence.
3. **Données** : nouveau panneau dans `aircraft.ron`, avec les origines `C:` ; `just test` (le contrôle de surface à ±2 % s'applique au chargement).
4. **Campagne après** et comparaison point par point avec le tableau ci-dessus. Tout écart inattendu est expliqué avant d'aller plus loin ; rien n'est réglé pour retrouver l'ancien comportement.
5. **Modèle 3D** : `just mockup spad7`, puis `asset_check`.
   - Le panneau ajouté remplit l'espace entre l'étambot et le gouvernail.
   - La longueur acceptée (6,264 m) ne devrait pas bouger : elle est donnée par la bande d'emplanture du panneau supérieur. Si elle bouge, l'écart accepté est revu.
   - Capture `maquette-spad7` avant et après, **images regardées**.
6. **Revue de l'aérodynamicien** sur les données et la campagne.
7. **Documentation et clôture** :
   - `spad7-calculs.md` § 3.1 et § 5 (valeurs de la dérive) ;
   - rapport d'essai ;
   - carnet, commit et push.

`just check` et `just test` à chaque étape.

## Risques

- **Spirale** : si elle devient nettement plus divergente que « peu divergente », c'est un résultat à rapporter, pas à corriger. Les causes possibles sont ailleurs : dièdre effectif, effet de dièdre du fuselage.
- **Le contrôle de surface à ±2 %** se servira de la nouvelle `reference_area`. Les deux valeurs viennent du même contour : il ne vérifie plus le dessin, seulement la cohérence des bandes avec lui.
- **Souffle de l'hélice** : le panneau ajouté est dans le jet et dans sa rotation, donc le palonnier de trim en montée bouge. C'est attendu.

## Décisions demandées au joueur

1. Le plan.
2. Le sillage du fuselage : (a) aucun facteur, écart documenté (recommandé) ; ou (b) nouvelle fonction du modèle de vol.

## Exécution (2026-10-26)

- **Géométrie** (aérodynamicien, contour numérisé) : panneau inférieur **rectangulaire**, de l'étambot (x −4,47) au bord de fuite dessiné (x −5,245).
  - Corde 0,775 m, gouvernail à 70 % : la charnière des bandes tombe à x −4,703, sur l'axe dessiné.
  - `reference_area` 1,00 m², `aspect_ratio` 1,36, `span_efficiency` inchangé (0,61) ; 2 bandes suffisent (test de convergence relancé).
  - Un panneau effilé, plus proche de l'arrondi du dessin, était refusé au chargement (−3,4 %) : le panneau supérieur avait été construit comme la surface dessinée moins un rectangle sous le plan fixe.
- **Deux estimations du plan corrigées** :
  - N_v gagne **+4,2 %**, et non +5 à +9 % : à envergure égale, la corde ajoutée fait baisser l'allongement ;
  - le sillage du fuselage surestime N_v de **5 à 9 %**, et non d'environ 1 % : tout le panneau inférieur est derrière le fuselage, gouvernail compris. La partie ajoutée, elle, ne compte que pour moins de 1 %. L'option (a) est appliquée, avec ce chiffre dans le commentaire de `span_efficiency`.
- **Campagne** avant et après (`spad7-calculs.md` § 3.1) :
  - N_v de +111,5 à +116,2 ; L_v de −18,0 à −17,2 ;
  - roulis hollandais de 3,46 à 3,39 s ; spirale de ×2 en 17,50 s à 17,40 s ;
  - performances à 0,2 % près ; vrille et manœuvres à la marge.
  
  Aucun écart inexpliqué.
- **Modèle 3D** : régénéré ; `asset_check` passe, écarts acceptés inchangés. Captures avant et après **regardées** : la partie fixe comble l'espace entre l'étambot et le gouvernail du bas.
- **Test du générateur** : il figeait le pivot des bandes du gouvernail (x −4,6785). Il recalcule maintenant la valeur attendue (−4,6628) depuis les panneaux d'`aircraft.ron`, milieu des segments de charnière pondéré par la longueur, à 1 mm près. Il suit donc les données, et refuse une moyenne non pondérée ou une pondération par la hauteur. Ma première version, à 5 cm près, était trop lâche : relevé par l'aérodynamicien.
- **Revue de l'aérodynamicien** : conforme, rien de bloquant ; données et origines vérifiées ; campagne conforme à ses prévisions (N_v +4,3 % prévu, +4,2 % mesuré), sans écart inexpliqué.
