# Plan — Écarts du SPAD VII : pistes physiques sourcées

- Statut : **validé** par le joueur le 2026-10-23
- Contexte : premier essai en vol du `spad7` (carnet de bord du 2026-10-22) :
  - la vitesse et le régime à 2 000 m passent ;
  - **le modèle monte trop bien** : montées 15 à 22 % trop rapides, plafond +19 %, vitesses à 3 000 et 4 000 m +7 % ;
  - l'historien maintient les cibles (`spad7.md` § 5.5.6). Le joueur a choisi la voie des pistes physiques sourcées.
- Règle : **aucun coefficient réglé sur les cibles du SPAD**. Chaque ajout est mesuré sur un objet d'époque ou de vraie grandeur, porte sa source, est testé et revu par l'aérodynamicien. S'il reste un écart à la fin, on le rapporte.

## Diagnostic

L'excès de vitesse ascensionnelle est d'environ 0,85 m/s, soit environ 6 kW, presque constant en altitude. Ajouter de la traînée **constante** ne le corrige pas sans casser la vitesse à 2 000 m (aérodynamicien). Ce qui manque doit donc peser **en montée** (forte portance) plus qu'**en vitesse maximale** (faible portance), ou bien toucher la puissance.

## Trois pistes, avec leur source et leur ordre de grandeur

| Piste | Source | Mesure | Effet attendu en montée (ordre de grandeur, C) |
|---|---|---|---|
| 1. Traînée induite par la portance de l'avion complet | NACA Report 304 (Sperry Messenger, ailes RAF 15, **vols planés en vraie grandeur**, tableau II lu sur l'image) | polaire de l'avion complet sur 21 vols de C_L 0,23 à 0,81 : C_D = 0,055 + 0,120 C_L², soit un **coefficient d'Oswald de 0,73** pour la cellule biplan (allongement équivalent 3,6 avec le σ de Prandtl) | si le SPAD du modèle est plus efficace, environ −0,5 m/s |
| 2. Traînée parasite qui croît avec l'incidence | NACA Report 17 (S43, maquette de JN-2 en présence des ailes, MIT 1917) | traînée du fuselage et du train : +67 % entre 0° et 12° ; traînée parasite totale : jusqu'à +20 % entre 0° et 9° | environ −0,15 m/s |
| 3. Puissance installée en vol | NACA Report 301, p. 10 | dans trois essais sur quatre, coefficients de puissance en vol supérieurs de 3 à 5 % à la soufflerie : les moteurs donnaient en vol un peu moins que leur puissance au banc | −4 % de puissance : environ −0,45 m/s, et −1,3 % sur la vitesse maximale |

Ensemble, elles sont de l'ordre de l'écart mesuré. Ce chiffrage reste grossier : seul l'essai dira où l'on tombe.

## Étapes

1. **Mesurer la polaire du modèle** : nouvelle commande `fdm_lab polaire <id>`.
   - Vols planés stabilisés au ralenti sur toute la plage de vitesses.
   - C_L et C_D de la cellule **sans l'hélice**, dans la convention du Report 304 (traînée « vraie »).
   - Ajustement C_D = C_D0 + K C_L² et coefficient d'Oswald, repris dans le rapport d'essais.
   - Tests sur le biplan synthétique : ajustement exact sur une polaire connue.
2. **Traînée parasite et incidence** (piste 2) :
   - les éléments de traînée gagnent une loi facultative cd(α) = cd₀ (1 + k sin² α), avec α l'incidence locale de l'élément ;
   - k vient de S43 : environ 15 pour le fuselage, le radiateur et le train, 0 pour les mâts et les câbles ;
   - changement du FDM (`DragItem`), du schéma (`format-avion.md`) et des données, avec tests et revue.
3. **Comparer à la vraie grandeur** (piste 1) :
   - on compare le coefficient d'Oswald du SPAD modélisé, piste 2 comprise, à celui du Report 304 ;
   - s'il est nettement meilleur, le reste est attribué à la traînée de profil d'ailes entoilées réelles, plus forte que celle de la maquette polie du Report 233. On l'ajoute au profil comme un accroissement ΔC_d = k_f C_L² (nouveau champ des fichiers de profil, `C:` d'après le Report 304) ;
   - le calage se fait sur l'**avion de comparaison** mesuré, jamais sur les cibles du SPAD. Confiance C, puisque c'est un autre avion.
4. **Puissance installée** (piste 3) : nouveau champ moteur `installed_power` (fraction de la puissance au banc), 0,96 pour le SPAD (`C:` Report 301, milieu de 3 à 5 %).
5. **Essai et rapport** :
   - `/essai-en-vol spad7`, revue de l'aérodynamicien ;
   - rapport au joueur avec les écarts restants, sans réglage ;
   - s'il reste un écart à 3 000 et 4 000 m, c'est le groupe de vitesses que l'historien juge douteux (`spad7.md` § 5.5.3).

Chaque étape : tests unitaires, `just check`, `just test`, carnet de bord, commit et push.

## Résultats

### Étape 1 (2026-10-23)

`fdm_lab polaire spad7` : coefficient d'Oswald de la cellule du SPAD modélisé 0,890, contre 0,734 pour le Messenger en vraie grandeur (Report 304) ; voir le carnet de bord.

### Étape 2 (2026-10-23) : mesurée, sans changement du modèle

La relecture complète de S43 (NACA Report 17, p. 285-288) corrige deux prémisses du plan :
- la traînée du **train** croît jusqu'à 2° environ (référence : longerons supérieurs), puis reste quasi constante jusqu'à 16° : pas de loi d'incidence pour le train ;
- la hausse de 0,015 à 0,025 lb à 12° est **lente jusqu'à 9°, puis rapide** : la traînée du corps, du train et de l'empennage reste « constante à 20 % près de 0° à 9° », et croît très vite au-delà. Une loi en sin² α (k ≈ 15) surestime donc la hausse aux petits angles.

Par ailleurs, le fuselage du modèle porte déjà une traînée d'écoulement transversal (Allen et Perkins, en sin³ α) : une loi en sin² α ajoutée aux mêmes éléments la compterait deux fois aux grands angles (décrochage, vrille).

Incidence du fuselage du SPAD modélisé (trim, 74 % de carburant) :

| Altitude | Meilleure montée | Incidence | Vitesse maximale | Incidence |
|---|---|---|---|---|
| 0 m | 115 km/h | 2,6° | 205 km/h | −2,0° |
| 2 000 m | 118 km/h | 4,0° | 198 km/h | −1,3° |
| 4 000 m | 127 km/h | 4,9° | 186 km/h | 0,0° |

Effet sur la vitesse ascensionnelle :
- **Majorant**, avec la loi du plan (k = 15,4 en sin² α) appliquée au corps, au poste, aux armes, aux échappements, au train et même au radiateur (0,41 m²) : −0,04 m/s à 0 m, −0,08 m/s à 2 000 m, −0,12 m/s à 4 000 m.
- **Allure de S43** (une loi en sin⁴ α passe par +67 % à 12° et +21 % à 9°) : −0,002 à −0,02 m/s.
- Ordre de grandeur de l'écart à combler : 0,85 m/s.

Conclusion : le fuselage du SPAD ne monte qu'à 3 à 5° d'incidence, là où S43 ne mesure presque rien. La piste 2 n'explique pas l'écart. On ne change pas le modèle (la loi serait un ajout de code sans effet mesurable, avec un risque de double compte). Elle reste notée pour les grands angles, à reprendre avec une source en vraie grandeur : les mesures de S43 sont faites sur une maquette au 1/24 à 30 mph, et le décollement du corps à ces faibles nombres de Reynolds ne se transpose pas tel quel.

## Hors de ce plan (tâches séparées, déjà repérées)

- Effet Munk distinct en tangage et en lacet : deux volumes. En lacet, la stabilité de route est surestimée d'environ 35 %.
- Efficacité des gouvernes à grand braquage (K′ de DATCOM).
- Gains du pilote scripté normalisés par avion. Sa direction oscille pendant la reprise du décrochage du SPAD.
- Chronométrage des montées depuis le lâcher des freins : il faut le modèle de sol du M2.
- Documents d'archives repérés par l'historien : fonds Dorand du SHD, rapport britannique n° 96.

### Étape 3 (2026-10-23) : traînée des ailes entoilées, k = 0,020 ± 0,005

- Avion de comparaison modélisé : le Sperry Messenger du Report 304 (`tools/fdm_lab/comparison/`), sur les mêmes tables et avec les mêmes méthodes que le SPAD. Table polie : Oswald 0,889, contre 0,734 mesuré.
- Nouveau champ de profil `fabric_drag_rise` : ΔC_d = k C_l². Pour retrouver la polaire mesurée, k = 0,020 ± 0,005 ; le calage est tenu par un test. Détail et sensibilité : `spad7-calculs.md` § 6.
- `fdm_lab polaire` plane désormais jusqu'au C_L le plus bas de l'ajustement, comme les piqués du Report 304 : avant, il s'arrêtait à la vitesse maximale en palier (C_L 0,5 pour le Messenger).
- SPAD avec k = 0,020 : Oswald 0,723. Montées −9 / −8 / −12 % (avant −15 / −16 / −22 %), plafond +8 % (avant +19 %), vitesses +1,7 / +5,6 / +4,5 % (avant +2,9 / +7,4 / +7,1 %).

### Étape 4 (2026-10-23) : puissance installée 0,963

- Nouveau champ moteur `installed_power`, appliqué à la puissance indiquée (frottement entier). −4 % au frein (Report 301) donne 0,963 pour le Hispano (`spad7-calculs.md` § 7).
- Campagne du SPAD :
  - vitesses −1,0 / +2,6 / +1,1 % : réussies ;
  - montées +0,8 / +3,1 / +1,1 % : réussies ;
  - plafond +2,1 % : réussi ;
  - **régime à la vitesse maximale −3,8 / −3,7 / −4,0 %, pour une tolérance de ±3 % : en échec**, A à 2 000 m.
- Sensibilité : 0,98 ferait tout passer, mais ce serait un réglage sur les cibles ; la source donne 3 à 5 % de perte. Le régime dépend surtout de l'hélice, un comparable (D' portée à 2,45 m) : piste documentaire, le diamètre et le pas réels de l'hélice du S.VII 150 ch.
