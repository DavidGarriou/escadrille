# Plan — Dégâts dans le modèle 3D du SPAD VII (préparation du M4)

- Statut : **validé** par le joueur le 2026-11-01 : périmètre complet du modèle 3D ; toile trouée par un masque au M4, sur une structure modélisée sous la toile.
- Demande du joueur (2026-10-31) : « Liste tous les dégâts possibles et intègre-les tout de suite dans le modèle 3D. »
- Catalogue : `docs/gdd/combat-degats.md` § 3 bis, 48 dégâts en huit familles.
- **Périmètre.** Le modèle 3D est préparé pour que chaque dégât du catalogue puisse se voir. Au M4 viendront :
  - la logique des dégâts (impacts, propagation, ruptures) ;
  - leurs effets sur le modèle de vol ;
  - les effets visuels animés (trous en décalques, fumée, flammes).
- **Règles** : skills `pipeline-assets` (nommage des nœuds), `rendu-visuel` (captures regardées), `combat`.

## Étapes

### 1. Découper la cellule en pièces nommées (`art/spad7/build.py`)

Aujourd'hui, toute la cellule sort en un seul maillage, `airframe`. Chaque face recevra désormais une pièce, comme elle reçoit déjà sa région de toile, et l'écriture produira un nœud par pièce :

- **ailes** : `wing_<upper|lower>_<l|r>_<root|mid|tip>`, coupées aux mâts intermédiaires et extérieurs (y 1,71 et 3,03) ;
- **empennage** : `tailplane_<l|r>`, `fin` ;
- **fuselage** : `fuselage_front` (capot, moteur, poste) et `fuselage_rear` (derrière le poste) ;
- **mâts** : `strut_<l|r>_<inner|outer>_<front|rear>` et `cabane_<l|r>_<front|rear>` ;
- **train** : `gear_<l|r>`, `axle`, `wheel_<l|r>`, `skid` ;
- **le reste** : `gun_1`, `windscreen`, `exhaust_<l|r>`, `linkage_<l|r>` (biellettes d'aileron) ;
- **hélice** : les pales séparées, `prop_blade_<1|2>`, sous `prop` ;
- **câbles** : un nœud par câble, `wire_<famille>_<l|r>_<n>`, au lieu d'un par famille. Le nom garde le préfixe `wire_` et le diamètre en extras, pour le shader anticrénelé.

### 2. Structure sous la toile

- Nervures, longerons, bords d'attaque et de fuite de chaque surface entoilée : ailes, plan fixe, dérive, gouvernes.
- Longerons et montants du fuselage arrière.
- Modélisés en bois, à l'intérieur de la toile, dans le nœud de leur pièce. Ils se voient par les trous et quand la toile brûle (dégâts 2, 3, 4, 35).
- Positions : pas des nervures (`ribs`), longerons à 12 et 46 % de la corde (`strut_spars`), données déjà dans `maquette.ron`.

### 3. Volumes d'impact, points d'effets

- **Volumes `hit_<composant>`** : boîtes ou enveloppes convexes, masquées.
  - Moteur, radiateur, réservoirs principal et en charge, huile, magnéto et carburateur.
  - Pilote : tête, torse, bras, jambes.
  - Mitrailleuse et caisse à munitions, synchronisation.
  - Câbles de commande (parcours interne), longerons de chaque panneau (avant, arrière), mâts, empennage, train.
- **Points `fx_<nom>`** : fumée et vapeur du moteur et du radiateur, fuites des réservoirs, flammes d'échappement, feu du poste.

### 4. Variantes endommagées `dmg_<pièce>_<état>` (masquées)

- **Panneaux d'aile** : moignon cassé à l'emplanture (`_stub`), avec longerons éclatés.
- **Fuselage arrière** : rupture.
- **Mâts** : deux tronçons.
- **Câbles** : coupés, deux bouts qui flottent vers l'arrière.
- **Hélice** : moignon de pale.
- **Pare-brise** : brisé.
- **Train** : jambe rompue.

### 5. Contrat et contrôle (`crates/data`, `tools/asset_check`)

- **Le contrat d'un avion liste en plus** :
  - les pièces attendues, déduites des données : surfaces, mâts, train ;
  - les volumes d'impact obligatoires ;
  - les variantes obligatoires.
- **`asset_check` vérifie** :
  - leur présence ;
  - que chaque `hit_` est fermé et convexe ;
  - qu'aucune pièce ne manque à la cellule : la somme des pièces couvre l'ancien `airframe`.

### 6. Dans le jeu

- Au chargement, les nœuds `hit_` et `dmg_` sont masqués.
- Nouvelle scène de référence **`degats-spad7`**, une « galerie » au studio :
  - un panneau d'aile sans toile, pour montrer sa structure ;
  - un panneau détaché ;
  - des câbles coupés ;
  - un mât brisé ;
  - une pale cassée ;
  - le pare-brise brisé.

  Elle est capturée et **regardée**, avec la revue du directeur artistique.
- Le vol d'essai garde son allure : captures avant/après comparées.

### 7. Vérification

- `just check`, `just test`.
- Captures `vol-essai-pilote`, `vol-essai-poursuite`, `avion-studio`, `degats-spad7`.
- Coût de rendu mesuré : plus de nœuds, et une structure interne.
- Docs : skill `pipeline-assets` (nouveaux nœuds), `format-avion.md` si le contrat change. Carnet.

## Risques

- **Coût de rendu** : une centaine de nœuds au lieu d'une dizaine, plus la structure interne. À mesurer sur les captures. Si besoin, la structure ne se rend que derrière la toile endommagée (M4).
- **Coutures** : des pièces coupées entre deux mâts laissent des jointures. Les mêmes sommets sont partagés au découpage, pour que rien ne se voie.
- **Atlas de toile** : il garde ses régions par surface et par côté ; les pièces ne changent que le maillage.
