# Combat, dégâts et son : exigences du joueur

- État : exigences du joueur (2026-10-31) ; catalogue complet des dégâts (§ 3 bis) ; préparation du modèle 3D : plan `docs/plans/m4-degats-modele-3d.md`
- Dernière mise à jour : 2026-10-31
- Skills concernées : `combat`, `simulation-vol`, `rendu-visuel`, `pipeline-assets`, `audio`, `recherche-historique`, `realisme`

## 1. Demande

Le joueur, le 2026-10-31 :

> « Par la suite, il faut penser au modèle de combat. Les dégâts sur les avions et pilotes doivent être particulièrement réalistes : dérive déchirée, ailes endommagées ou coupées, câbles sectionnés, etc. Tous les dégâts doivent être visibles et influencer le vol de l'avion. Le comportement des armes doit être réaliste. Il faut aussi ajouter le son. »

| Pilier | Ce que la demande lui apporte |
|---|---|
| 1. Sentir le vol | une machine fragile de bois, de toile et de câbles, qui se dégrade sous les balles et le fait sentir au manche |
| 4. Histoire humaine | le pilote blessé, l'avion qui rentre en lambeaux, le camarade qui ne revient pas |
| 5. Authenticité | armes et dégâts tirés de sources, comme le modèle de vol |

## 2. Les trois règles

1. **Tout dégât agit sur le vol, par la physique.**
   - Un dégât modifie un composant du modèle de vol : bandes d'une aile, gouverne, câble, moteur, masse.
   - Il ne passe jamais par un malus caché (`combat`, « tout passe par la physique et par les commandes »).
   - Les ailes sont déjà découpées en bandes (16 par demi-plan supérieur) : un trou ou une déchirure peut agir sur les seules bandes touchées.
2. **Tout dégât se voit.**
   - Le modèle 3D montre chaque état du tableau du § 3 : trous en décalques, toile déchirée qui flotte, pièces arrachées qui se détachent, câbles coupés qui pendent, fumée et feu (`rendu-visuel`).
   - Les états sont validés sur captures.
3. **Les armes se comportent comme les vraies.**
   - Balles simulées une à une, avec traînée et gravité.
   - Cadence liée au régime par la synchronisation, dispersion, échauffement et enrayages, réglage de tir.
   - Chaque chiffre est sourcé dans une fiche (`assets/weapons/<id>.ron`).

## 3. Dégâts demandés, et ce qu'ils deviennent

Le tableau complet par composant est dans le skill `combat` (« Modèle de dégâts »). Ici, les cas cités par le joueur :

| Dégât | Effet sur le vol (modèle de vol) | Ce qui se voit (modèle 3D) |
|---|---|---|
| **Dérive ou gouvernail déchirés** | surface et portance latérale réduites sur les bandes touchées : moins de stabilité de route, gouvernail moins efficace ; déchirure qui s'agrandit avec la vitesse | toile trouée puis lambeaux qui flottent ; structure visible à travers |
| **Ailes endommagées** | trous : portance réduite et traînée accrue sur les bandes touchées, d'où roulis et lacet parasites ; longeron touché : résistance réduite | décalques de trous, déchirures, nervures apparentes |
| **Aile coupée ou arrachée** | rupture du longeron ou des haubans sous facteur de charge (structure) : panneau perdu, roulis brutal, souvent fatal | le panneau se détache et tombe ; câbles et mâts rompus |
| **Câbles sectionnés** | câble de commande coupé : gouverne libre, commande perdue ; hauban de vol coupé : la cellule perd de sa résistance et peut rompre en ressource | le câble disparaît ou pend, coupé |
| **Pilote touché** | blessure : vision troublée, force réduite sur les commandes, perte de connaissance ; mort | à l'écran : vision et commandes (selon le réalisme) |

Les autres dégâts (moteur, réservoir et feu, hélice, armes, train) suivent le skill `combat`.

## 3 bis. Catalogue complet des dégâts (SPAD VII, 2026-10-31)

Demande du joueur : « Liste tous les dégâts possibles et intègre-les tout de suite dans le modèle 3D. »

Colonne « Modèle 3D » :
- **pièce** : nœud séparé qui peut se trouer, se détacher ou disparaître ;
- **structure** : nervures et longerons modélisés sous la toile, visibles par les trous ;
- **variante** : nœud `dmg_` masqué, montré quand le dégât survient ;
- **impact** : volume `hit_` masqué ;
- **effet** : point `fx_` où naissent fumée, flammes ou fuite.

Les effets en vol et la logique des dégâts viennent au M4. Le modèle 3D les prépare dès maintenant.

### A. Cellule, toile et structure

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 1 | Toile percée (ailes, empennage, gouvernes, fuselage) | portance réduite et traînée accrue sur les bandes touchées | trous de balles | pièce, impact |
| 2 | Toile déchirée, déchirure qui s'agrandit avec la vitesse | idem, croissant ; vibrations | lambeaux, structure apparente | pièce, structure |
| 3 | Toile arrachée ou brûlée sur un panneau entier | portance perdue sur le panneau | nervures et longerons nus | structure |
| 4 | Nervures cassées | profil déformé, portance locale réduite | nervures brisées sous la toile | structure |
| 5 | Longeron touché | résistance réduite, rupture sous facteur de charge | éclats | impact par longeron |
| 6 | Longeron rompu : aile qui plie | roulis et lacet brutaux, puis perte du panneau | panneau plié | pièce, variante (bout cassé) |
| 7 | Aile coupée ou arrachée | panneau perdu, roulis brutal, souvent fatal | le panneau se détache | pièce par panneau entre mâts, variante (moignon) |
| 8 | Bord d'attaque ou de fuite brisé | traînée, portance locale réduite | bord écrasé | structure |
| 9 | Mât d'aile touché ou rompu | la charge passe aux autres mâts, rupture en chaîne | mât brisé | pièce par mât, variante (deux tronçons) |
| 10 | Mât de cabane rompu | plan supérieur mal tenu, flottement | mât brisé | pièce, variante |
| 11 | Hauban de vol ou d'atterrissage coupé | résistance réduite en ressource ou en piqué | le câble pend, coupé | pièce par câble, variante (deux bouts flottants) |
| 12 | Câble d'incidence coupé | torsion de la cellule, calages modifiés | câble coupé | pièce, variante |
| 13 | Hauban d'empennage coupé | plan fixe ou dérive mal tenus, flottement | câble coupé | pièce, variante |
| 14 | Plan fixe percé, déchiré ou arraché (gauche, droit) | stabilité et équilibre en tangage modifiés | trous, lambeaux, pièce perdue | pièce par côté, structure, variante |
| 15 | Dérive percée, déchirée ou arrachée | stabilité de route réduite ou perdue | trous, lambeaux, pièce perdue | pièce, structure, variante |
| 16 | Gouverne (ailerons, profondeur, gouvernail) percée ou arrachée ; charnière brisée | efficacité réduite ; gouverne bloquée ou qui flotte | trous, gouverne de travers ou absente | pièce, structure |
| 17 | Fuselage arrière rompu (longerons de fuselage) | queue arrachée, perte de contrôle | rupture derrière le poste | pièce (avant, arrière), variante |
| 18 | Capot d'aluminium percé ou panneau arraché | traînée | trous, panneau absent | pièce |
| 19 | Pare-brise brisé | éclats au visage du pilote | verre étoilé ou absent | pièce, variante |

### B. Commandes

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 20 | Câble de commande coupé (ailerons, profondeur ou direction, dans un sens) | gouverne tenue dans un seul sens, ou libre | câble externe qui pend ; gouverne qui flotte | pièce par câble externe, impact sur le parcours interne |
| 21 | Biellette d'aileron coupée ou tordue | aileron libre ou bloqué | biellette brisée | pièce, variante |
| 22 | Manche ou palonnier touché | commande bloquée | (poste, avec la passe du cockpit) | impact |
| 23 | Commande des gaz coupée | régime figé | — | impact |

### C. Moteur et hélice

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 24 | Pale d'hélice ébréchée | balourd, vibrations | pale entaillée | pièce par pale |
| 25 | Pale cassée | vibrations violentes, bâti moteur arraché si on ne coupe pas | moignon de pale | pièce par pale, variante (moignon) |
| 26 | Cylindre ou carter touché | ratés, perte de puissance, fuite d'huile, grippage | fumée, huile sur le pare-brise | impact (moteur), effet |
| 27 | Allumage (magnéto) touché | ratés ou arrêt | — | impact |
| 28 | Carburateur touché | arrêt ou feu | flammes | impact, effet |
| 29 | Radiateur ou durites percés | fuite d'eau, surchauffe, perte de puissance, grippage | vapeur blanche | impact (radiateur), effet |
| 30 | Échappement arraché | bruit, flammes visibles | flammes courtes | pièce, effet |
| 31 | Synchronisation endommagée | la Vickers tire dans sa propre hélice | pales touchées | impact |

### D. Carburant et feu

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 32 | Réservoir principal percé (sous le siège) | fuite, panne sèche | traînée d'essence | impact, effet |
| 33 | Réservoir en charge percé (plan supérieur) | fuite, puis plus de réserve | traînée sous le plan | impact, effet |
| 34 | Pression d'alimentation perdue | alimentation par le seul réservoir en charge, puis arrêt | — | impact |
| 35 | Feu d'essence ou d'huile | fumée, flammes, toile qui brûle, brûlures, dislocation | flammes, fumée noire, structure nue | effet, structure |

### E. Pilote

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 36 | Blessure au bras | force et précision réduites au manche | effets à l'écran | impact (bras) |
| 37 | Blessure à la jambe | palonnier faible d'un côté | effets à l'écran | impact (jambes) |
| 38 | Blessure grave, hémorragie | vision qui se voile, puis perte de connaissance | effets à l'écran | impact (torse) |
| 39 | Tête ou thorax : mort | l'avion n'est plus piloté | effets à l'écran ; pilote affaissé | impact (tête, torse) ; silhouette avec la passe du cockpit |
| 40 | Éclats (verre, bois) | blessure légère | effets à l'écran | — |
| 41 | Brûlures (feu) | blessure qui s'aggrave | effets à l'écran | — |

### F. Armement

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 42 | Enrayage (chaleur, cartouche) | arme muette jusqu'au désenrayage | — | — |
| 43 | Mitrailleuse touchée | arme détruite | culasse ou manchon percés | pièce, impact |
| 44 | Bande de munitions coupée | tir interrompu | — | impact (caisse à munitions) |

### G. Train et sol

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 45 | Pneu crevé, roue voilée | atterrissage dangereux, cheval de bois | roue déformée | pièce par roue |
| 46 | Jambe de train ou essieu rompus | capotage à l'atterrissage | jambe brisée | pièce par jambe, variante |
| 47 | Béquille arrachée | queue au sol | béquille absente | pièce |

### H. Instruments

| # | Dégât | Effet en vol | Ce qui se voit | Modèle 3D |
|---|---|---|---|---|
| 48 | Instrument détruit (compte-tours, altimètre…) | lecture perdue | cadran brisé | avec la passe du cockpit (B7) |

## 4. Ce que cela demande, par domaine

- **Modèle de vol** (`escadrille_fdm`) :
  - un état d'endommagement par bande de surface (portance, traînée, surface restante) ;
  - un état par câble et par gouverne ;
  - la structure : charge limite par longeron et par hauban, rupture sous facteur de charge et pression dynamique.

  Tout se teste dans `fdm_lab`, par un essai par type de dégât.
- **Modèle 3D** (`art/<avion>/build.py`, `pipeline-assets`) :
  - une cellule découpée en pièces endommageables : panneaux d'aile entre les mâts, dérive, gouvernail, plan fixe par côté, chaque câble ;
  - des volumes `hit_<composant>` et des variantes `dmg_<pièce>` ;
  - des décalques de trous et de déchirures sur l'atlas de toile.
- **Armes** :
  - fiches sourcées : Vickers, LMG 08/15, Lewis ; cadence, vitesse initiale, balle, alimentation, synchronisation, enrayages ;
  - recherche historique : historien.
- **Son** (jalon M6, `audio`) :
  - moteur et vent pilotés par le modèle de vol ;
  - mitrailleuses, impacts sur la toile et le bois, toile déchirée qui claque, câbles qui sifflent puis cassent.

## 5. Ordre proposé (à valider)

La feuille de route (`docs/roadmap.md`) place le combat au M4 et le son au M6. Deux remarques :

- **Le modèle 3D gagne à se préparer dès maintenant.** Découper la cellule en pièces endommageables coûte peu pendant le M3, et beaucoup plus après la passe de beauté.
- **Le son du moteur et du vent sert déjà au pilotage**, avant tout combat : régime, approche du décrochage (skill `audio`). Il peut être avancé, juste après le M2, sans attendre le M6.

## 6. Questions ouvertes

1. L'ordre : garder M4 combat puis M6 son, ou avancer un premier son (moteur, vent) ?
2. Le réalisme des blessures et de la mort du pilote selon les préréglages (`realisme`) : la mort définitive, la carrière.
3. Les sources : témoignages d'avions rentrés criblés, de ruptures d'aile, de câbles coupés (historien) ; cadences et balles des armes.
