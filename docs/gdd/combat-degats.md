# Combat, dégâts et son : exigences du joueur

- État : exigences du joueur (2026-10-31), à détailler dans le plan du M4, à faire valider avant tout code
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
