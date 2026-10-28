# Plan — Vol d'essai : premier incrément jouable du M2

- Statut : **validé** par le joueur le 2026-10-28, avec les options recommandées :
  - gaz sur la croix ↑/↓, stick droit au regard libre ;
  - neutre de la profondeur réglable ;
  - départ à 1 000 m et 160 km/h ;
  - sans aides.
- Demande du joueur (2026-10-28) : « Avant la seconde livrée, je veux une première version jouable, un vol d'essai qui démarre en l'air », avec :
  - le modèle de vol du SPAD branché dans le jeu ;
  - la manette : stick gauche pour le manche, gâchettes pour le palonnier, stick droit ou boutons pour les gaz ;
  - une vue au choix, poursuite derrière l'avion ou pilote dans le poste ;
  - le SPAD au-dessus de la plaine, avec ciel et ombres.
- **Jalon** : premier incrément du **M2 « Premier vol »** (`docs/roadmap.md`). Le M1 n'est pas clos : son essai joueur (lecture du rapport d'essais) reste à faire, et le joueur choisit d'avancer.
- **Hors de cet incrément** :
  - train d'atterrissage et sol, donc décollage et atterrissage (incrément suivant du M2) ;
  - son ;
  - poste détaillé (B7 du plan `m3-modele-spad7.md`) ;
  - profils de manette en RON et remappage ;
  - vibrations ;
  - aides au pilotage et préréglages de réalisme (M5) ;
  - seconde livrée.

## Ce que fera la version

`just run` démarre en l'air. Le SPAD vole en palier à **1 000 m** et **160 km/h**, cap à l'est, au-dessus d'une plaine sous le soleil d'après-midi. On le pilote à la manette, on passe de la vue de poursuite à la vue du pilote, on affiche la télémétrie. Si l'avion touche le sol, le vol recommence au point de départ.

## Étapes

Chaque étape : tests, `just check`, `just test`, carnet, commit et push. Une étape terminée laisse le jeu lançable.

### 1. État de départ (données)

- **Fichier** `assets/scenarios/vol-essai.ron`, schéma `crates/data/src/scenario.rs` :
  - avion, position, cap, altitude, vitesse ;
  - état d'équilibre du modèle de vol à ce point : incidence, gaz, régime de l'hélice, déflexion de la profondeur.
- **Calcul** : les valeurs d'équilibre viennent du solveur de `fdm_lab`, puisque le jeu ne peut pas l'appeler (`nalgebra` est réservé aux outils). Elles sont marquées `C:` (calculées).
- **Contrôle** : un test de `fdm_lab` vérifie que le fichier est encore un équilibre du modèle actuel. Si le modèle de vol change, le test échoue et dit quelles valeurs reprendre.
- **Valeurs actuelles**, sondées le 2026-10-28 :
  - à 160 km/h et 1 000 m : incidence 0,0°, gaz 0,56, manche à −0,108 (poussé de 11 %) ;
  - **manche au neutre**, l'avion s'équilibre à **88 km/h** et 10,7° d'incidence, près du décrochage. Voir la décision 2.

### 2. Le modèle de vol dans le jeu (`crates/game/src/flight/`)

- **Avion** : construit depuis `aircraft.ron` par `escadrille_data`, comme dans `fdm_lab`.
- **Pas fixe** : simulation dans `FixedUpdate` à **120 Hz**, intégration RK4 (ADR 0005). L'état précédent et l'état courant sont gardés.
- **Environnement** : atmosphère type, sans vent, sol plat à 0 m.
- **Rendu interpolé** entre les deux derniers pas, avec la fraction de pas de `Time<Fixed>`. Les conversions passent par `bridge.rs` et lui seul (ADR 0006).
- **Origine flottante minimale** : l'origine du rendu est recalée sous l'avion par paliers de 1 km ; le sol suit. Positions du modèle de vol en f64.
- **Animation du modèle** :
  - gouvernes tournées de leur déflexion, autour de leur charnière (nœuds du contrat) ;
  - hélice tournée à son régime ;
  - au-dessus d'un régime seuil, disque flou translucide à la place des pales.
- **Sol** : si un point bas de la cellule passe sous 0 m, message « Écrasé » et redémarrage après 3 s. Pas de roulage avant le train.
- **Pause** : le temps fixe s'arrête.

### 3. Couche d'actions et manette (`crates/game/src/input/`)

- **Principe** : le jeu ne lit que des actions, jamais un bouton (skill `manette`). Mapping par position (sud, est, ouest, nord), étiquettes de la SN30 Pro dans les textes.
- **Profil « Vol d'essai »** :

| Commande | Action |
|---|---|
| Stick gauche | manche : roulis (X), tangage (Y) |
| L2 / R2 | palonnier à gauche / à droite |
| Gaz | voir la décision 1 |
| L1 / R1 | neutre de la profondeur : piqué / cabré, par pas de 1 % (décision 2) |
| Y (gauche, ouest) | changer de vue : poursuite ↔ pilote |
| R3 | recentrer le regard |
| X (haut, nord) | afficher ou masquer la télémétrie |
| Select | recommencer le vol |
| Start | pause |

- **Traitement des axes** : zone morte radiale (8 %), puis courbe y = signe(x) × ((1 − k)|x| + k|x|³) avec k = 0,4. Ce sont les valeurs de départ du skill, à régler en essai.
- **Palonnier sur gâchettes** : tes gâchettes sont **tout-ou-rien** (labo du 2026-10-20). Le palonnier monte donc en rampe vers ±1 en 0,4 s et revient au centre en 0,2 s. Le palonnier analogique viendra avec le profil « Avancé ».
- **Clavier de secours**, pour le développement : flèches, Q/D, Page préc./suiv., V, R.
- **Profil** : écrit en code pour cet incrément ; le format RON et le remappage viendront ensuite.
- **Tests sans matériel** : zone morte, courbe, rampe du palonnier, gaz, neutre.

### 4. Vues (`crates/game/src/camera/`)

- **Poursuite** : à 10 m derrière et 2,5 m au-dessus. Elle suit le cap et l'assiette avec un amortissement (environ 0,3 s) et garde l'horizon peu incliné, pour qu'on lise l'inclinaison de l'avion. Le regard libre la fait tourner autour de l'avion.
- **Pilote** : à l'œil du pilote (nœud `cam_pilot_eye` du modèle), champ vertical de 60°.
  - **Regard libre** : ±150° en lacet, −30° à +80° en tangage, retour au centre sur R3.
  - **Ce qu'on voit** : capot, Vickers, pare-brise, plan supérieur et sa découpe, câbles. Le poste est encore vide (ni planche de bord ni manche, B7 du plan M3) ; la télémétrie en tient lieu.
- **Rendu** : celui des scènes de référence. Atmosphère de Bevy, ombres en cascades, TAA, TonyMcMapface, câbles anticrénelés, toile translucide.

### 5. La plaine (`crates/game/src/world/`)

- **Sol** : grand plan sous l'avion, recentré avec l'origine flottante. Sa texture de champs, générée au lancement sans image tierce, mêle parcelles de verts et d'ocres, chemins et haies sombres, à deux échelles. Elle sert à lire la hauteur et la vitesse. Mipmaps et filtrage anisotrope (plugin `render`).
- **Ciel et lumière** : atmosphère, soleil d'après-midi au sud-ouest à 40°.
- **Limite** : pas de relief, d'arbres ni de villages à cet incrément. Le vrai terrain des Flandres viendra avec `terrain_bake`.

### 6. Télémétrie (`bevy_egui`, déjà au workspace)

Petit panneau en coin, qui s'affiche ou se masque. Il donne :
- vitesse indiquée et vraie (km/h) ;
- altitude (m), variomètre (m/s) ;
- régime (tr/min), gaz (%) ;
- incidence, dérapage (°) ;
- facteur de charge ;
- position du manche, du palonnier et du neutre.

### 7. Vérification

- **Tests** :
  - le départ est un équilibre (étape 1) ;
  - la chaîne d'entrée (étape 3) ;
  - **une boucle de vol sans rendu** : application Bevy minimale, 60 s de vol, commandes au neutre décalé. Pas de NaN ; l'avion reste en vol, à quelques dizaines de mètres de son altitude.
- **Scènes de capture** `vol-essai-poursuite` et `vol-essai-pilote` : trois secondes de vol simulé, puis une image. Les images sont regardées.
- **Non vérifiable ici** : la manette (le conteneur n'a ni manette ni écran) et la fluidité (rendu logiciel). Ce sera ton essai.
- **Ton essai** : une liste de choses à essayer te sera donnée (tenue du palier, virages coordonnés au palonnier, décrochage, effet du couple aux gaz, vues). Les retours vont dans `docs/gdd/retours.md`.

## Décisions demandées au joueur

1. **Gaz** :
   - **(A, recommandé)** croix ↑/↓ : maintien = 30 % par seconde, appui bref = 5 %. Le stick droit sert au regard libre, utile dans le poste ;
   - (B) stick droit en Y, dont la déflexion règle la vitesse de variation des gaz. Pas de regard libre à cet incrément.
2. **Neutre de la profondeur** :
   - **(A, recommandé)** au départ, le neutre du stick correspond au manche qui tient le palier à 160 km/h (−0,108), et L1/R1 le déplacent. C'est un réglage de la manette, pas de la physique : la gouverne prend la position que le pilote lui donnerait en maintenant sa pression ;
   - (B) manche brut : stick au neutre, l'avion se cabre et ralentit vers 88 km/h, et il faut pousser en permanence.

   Dans les deux cas, ce neutre à 88 km/h sera soumis à l'aérodynamicien à la prochaine revue du modèle de vol. Le calage de l'empennage est estimé, et aucune source ne dit si le S.VII avait un compensateur réglable en vol. Aucun coefficient ne sera changé sans justification.
3. **Départ** : 1 000 m, 160 km/h, cap à l'est. À changer si tu préfères une autre vitesse ou altitude.
4. **Sans aides** : le SPAD tel que le modèle de vol le donne (couple de l'hélice, effet du souffle, spirale, décrochage), en attendant les préréglages du M5. D'accord ?

## Risques

- **Comportement du modèle de vol en temps réel** : il est testé en robustesse dans `fdm_lab`, mais jamais encore piloté. Le test de boucle de vol et ton essai le diront.
- **Coût du pas** : RK4 sur une centaine de bandes, 120 fois par seconde. À mesurer ; sous-pas ou parallélisme si besoin, sans changer la physique.
- **Rendu logiciel** dans le conteneur : lent, mais les captures restent justes.

## Avancement

- **2026-10-28 : réalisé**, toutes étapes.
  - **Départ** : `assets/scenarios/vol-essai.ron`, équilibre du modèle de vol contrôlé par `fdm_lab`.
  - **Modèle de vol dans le jeu** (`crates/game/src/flight/`) :
    - pas fixe à 120 Hz et rendu interpolé ;
    - origine flottante sur une grille de 2 km ;
    - gouvernes et hélice animées, disque flou au-dessus de 400 tr/min ;
    - contact avec le sol puis nouveau départ après 3 s ;
    - pause.
  - **Manette** (`crates/game/src/input/`) : profil « Vol d'essai » du skill `manette`, clavier de secours.
  - **Vues** (`crates/game/src/camera/`) : poursuite amortie, œil du pilote, regard libre.
  - **Plaine** (`crates/game/src/world/plain.rs`) : îlots de 500 m bordés de haies, découpés en parcelles de 25 à 70 m, tuile de 2 km générée au lancement.
  - **Télémétrie et aide** (`crates/game/src/ui/`).
  - **Scènes de capture** : `vol-essai-poursuite` et `vol-essai-pilote`.
- **Corrections vues sur les captures** :
  - la vitre du pare-brise, exportée en transmission, s'affichait opaque : elle passe en mélange alpha ;
  - la plaine en polygones faisait camouflage : elle devient un bocage de parcelles.
- **Mesuré** : le modèle de vol coûte environ 1,7 ms par pas dans le conteneur, en optimisation complète (itérations des plans et des angles induits). Acceptable pour un avion ; à profiler avant d'en faire voler plusieurs, sans changer la physique.
- **Pour la prochaine revue du modèle de vol** (aérodynamicien) : l'équilibre manche au neutre, à 88 km/h.

## Essai du joueur

Sur ta machine : `just run`. Le vol commence en l'air ; l'aide des commandes s'affiche 20 s, et à chaque pause (Start).

1. **Tenue du palier** : sans toucher au manche, l'avion tient-il à peu près son altitude ? Combien de temps avant qu'il parte en spirale ? Le neutre (L1/R1) suffit-il à le régler ?
2. **Roulis et virages** : incline à 30° puis à 60°. Faut-il du palonnier pour garder la bille au centre (télémétrie : dérapage) ? Le lacet inverse se sent-il ?
3. **Gaz** : réduis à 30 %, puis remets plein gaz. L'avion se met-il en lacet et en roulis (couple de l'hélice, souffle) ?
4. **Décrochage** : réduis les gaz et tire doucement jusqu'au décrochage. Vitesse indiquée à ce moment ? L'avion abat-il, et sur une aile ?
5. **Les vues** (Y) : la poursuite suit-elle bien sans secousses ? Dans la vue du pilote, que voit-on devant, et le regard libre (stick droit, R3) est-il assez rapide ?
6. **La manette** : zone morte, sensibilité du manche, rampe du palonnier sur L2/R2, gaz sur la croix : trop lent, trop rapide ?
7. **Fluidité** : images par seconde ressenties en `just run` (profil de développement).

Les retours vont dans `docs/gdd/retours.md`.

