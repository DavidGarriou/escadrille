# Plan — Train d'atterrissage et sol : fin du M2

- Statut : **validé** par le joueur le 2026-11-01, avec les options recommandées :
  - `just run` démarre au sol, et la pause permet de choisir sol, finale ou en vol ;
  - pré dessiné ;
  - tenue de cap en Arcade et Pilote, envol automatique en Arcade ;
  - accidents par seuils, avec une tolérance selon le préréglage.
- Demande du joueur (2026-11-01) : « Oui, plan du train », pour finir le M2.
- **Jalon** : dernier incrément du **M2 « Premier vol »** (`docs/roadmap.md`). Définition de « terminé » : le joueur vole dix minutes à la manette et trouve cela agréable ; décollage et atterrissage possibles en préréglage Pilote.
- **Règles** : skills `simulation-vol`, `bevy`, `realisme`, `manette`, `rendu-visuel` ; ADR 0002, 0003, 0005, 0006.
- **Hors de cet incrément** :
  - effet de sol (étape 9 du skill `simulation-vol`), à rouvrir si l'essai montre un arrondi trop sec ;
  - vent, rafales, bosses du terrain ;
  - dégâts réels : au M2, un accident fait recommencer le vol ; les variantes `dmg_gear_*` serviront au M4 ;
  - mise en route par le mécanicien, panne moteur, consommation ;
  - son, poste de pilotage, hangars, autres avions.

## Ce que fera la version

`just run` démarre au sol (décision 1). Le SPAD est arrêté sur un pré, queue au sol, moteur au ralenti. On met les gaz, il roule, on lève la queue et il s'envole. On revient, on se pose et on roule jusqu'à l'arrêt, sans freins. Hélice au sol ou posé trop dur : message « Accident », puis nouveau départ après 3 s.

## Ce que l'on sait déjà

**Géométrie**, calculée le 2026-11-01 depuis `aircraft.ron` et `maquette.ron`, réservoirs pleins (C) :

| Grandeur | Valeur | Conséquence |
|---|---|---|
| Masse, centre de gravité | 729 kg ; x −0,467, z −0,432 | |
| Roues | axe à x 0,136, z 0,764 ; 0,61 m de diamètre ; voie 1,54 m | valeurs du dessin de 1917 |
| Assiette trois points | 14,4° queue basse | le plan supérieur est à environ 16,7° d'incidence, au-delà du décrochage (12 à 13,8°) : un posé trois points se fait ailes décrochées |
| Charges au repos, train non comprimé | béquille 150 kg (20,5 %), 290 kg par roue | queue lourde, cohérente avec « s'arrête court » (T8) |
| Centre de gravité à la verticale des roues | 36° piqué depuis l'assiette trois points, soit 22° sous la ligne de vol | capotage difficile, cohérent avec T8 |
| Hélice | 0,27 m de garde en ligne de vol ; touche à 22,7° piqué sous la ligne de vol | elle touche à peu près quand le capotage commence |
| Énergie verticale à absorber | 1,5 kJ à 2 m/s ; 3,3 kJ à 3 m/s | pour 125 mm de course, plus le pneu |

**Propulsion** (`spad7-calculs.md`, § sur le décollage ; C/E) : 4,0 kN au point fixe à 1 593 tr/min. Roulage estimé hors outil : environ 88 m et 6 à 7 s jusqu'à 89 km/h. Au ralenti (environ 500 tr/min), la traction tombe vers 390 N (extrapolée en n²) : elle devrait rester sous le frottement, donc l'avion ne bouge pas sans gaz. Le modèle le dira.

**Sources** : aucune cible chiffrée de confiance A ou B.

- **Train** : jambes fixes, essieu coulissant dans des glissières, sandows, voie de 5 ft (S48, C). Débattement de 125 mm sur le S.XI, comparable de famille. **Inconnus** : raideur et précontrainte des sandows, pneus, charge de rupture.
- **Béquille** : fixe sur le S.XIII (C), non orientable. Pas de freins.
- **Roulage et décollage** (Meister, S.XIII, moteur à réducteur et rotation inverse) : roule facilement, part du côté du couple à l'ouverture des gaz, se corrige en poussant le manche pour lever la queue tôt ; se pose facilement, s'arrête court, ne capote pas.
- **Tallman** (S.VII à moteur américain, années 1960, confiance D) : queue levée presque aussitôt, en l'air après environ 46 m. **Ce n'est pas une cible.**
- **`spad7.md` § 6.2** : à pleine puissance, le S.VII part à gauche, contré au pied droit (confiance C).

## Étapes

Chaque étape : tests, `just check`, `just test`, carnet, commit et push. Une étape terminée laisse le jeu lançable.

### 0. Prérequis et recherche

- `cargo-nextest` et `cargo-deny` : installés par le joueur le 2026-11-01 ; `just check` et `just test` sont verts sur sa machine.
- **Historien**, treizième passe, en parallèle du code :
  - train et béquille du S.VII : sandows, pneus, sabot ;
  - technique de décollage et d'atterrissage, en particulier queue haute ou trois points ;
  - vitesse d'atterrissage ;
  - dimensions d'un terrain d'escadrille en 1917, et usage d'un T au sol.

  Rien ne bloque : toute valeur manquante reste `E:`, avec sa justification.

### 1. Modèle de contact (`crates/fdm/src/ground.rs`)

Un nouveau composant, sans Bevy, qui renvoie une force et un moment comme les autres.

- **Roues** : une sphère du rayon du pneu autour de l'axe, pour que le point de contact reste sous la roue à toute assiette.
- **Suspension** : pneu en série avec le sandow, précontrainte, butée en fin de course, amortissement faible. Le caoutchouc rend presque toute l'énergie : les rebonds doivent émerger.
- **Frottement** :
  - au roulage, résistance au roulement sur l'herbe ;
  - en travers, force de dérive du pneu, plafonnée à µ fois la charge ;
  - la béquille frotte dans toutes les directions : c'est le seul frein, et elle ne dirige pas.
- **Points de structure** : saumons, dos, nez, disque de l'hélice, bas de la dérive. Ressort raide et frottement, pour que le cheval de bois et le capotage émergent, sans script.
- **Vitesse nulle** : frottement régularisé à pente bornée, aucune division par la vitesse.
- **Sorties** : charge et enfoncement par point de contact, dans `Diagnostics`, pour le jeu, les aides et `fdm_lab`.
- **Tests** :
  - **en l'air, résultat identique au bit près** à celui d'aujourd'hui ;
  - charges au repos conformes au tableau ci-dessus, compression en plus ;
  - chute verticale : énergie dissipée bornée, pas de rebond qui gagne de l'énergie ;
  - symétrie, déterminisme, aucun NaN.

Le sol reste plat, et `Environment::ground_altitude` suffit. La pente et la nature du sol viendront avec le terrain réel.

### 2. Sous-pas : remettre le jeu d'accord avec l'ADR 0005

- **Constat** : `fdm_lab` calcule à 240 Hz, comme le veut l'ADR 0005 (120 Hz, deux sous-pas). **Le jeu fait un seul pas RK4 à 120 Hz.** Les essais ne valident donc pas exactement ce que le joueur pilote.
- **Action** : les sous-pas passent dans `Aircraft::step`, pour que le jeu et les outils aient le même pas. Le coût se mesure sur la machine du joueur : 1,7 ms par pas dans le conteneur.
- **ADR 0008** : l'ADR 0005 renvoie à un nouvel ADR le traitement du contact au sol. Il consignera le modèle retenu et le pas, après mesure de convergence : diviser `dt` par deux change la distance de décollage de moins de 1 %.

### 3. Données (`crates/data`, `aircraft.ron`)

- Section `undercarriage` : roues, sandows, pneus, béquille, frottements, points de structure, chaque valeur avec son origine `S:`, `C:` ou `E:`.
- Ligne « Prévus plus tard » de `format-avion.md` remplacée par le vrai schéma.
- Un test garde `aircraft.ron` et `maquette.ron` d'accord sur les roues et la béquille. `extremities()` du jeu, écrit « until the undercarriage is modelled », disparaît.

### 4. Essais dans `fdm_lab`

- **Commande `fdm_lab sol spad7`** : équilibre au repos (assiette, charges, enfoncement), point fixe, gaz nécessaires pour démarrer, tracés PNG lus.
- **`essais.ron`** :
  - mesures rapportées, sans cible historique : distance et temps de décollage, distance d'arrêt ;
  - qualitatifs : « embardée au décollage » reçoit enfin sa manœuvre (départ à gauche, contré au pied droit) ; « roule bien » ; « capote peu, s'arrête court » ;
  - robustesse : posé brutal (déjà exigé par le skill), capotage, cheval de bois, aile au sol.
- **Contrôle grossier** : le décollage reste de l'ordre des 88 m calculés hors outil, sans ajuster un coefficient pour y arriver.
- `/essai-en-vol spad7` : aucun écart en vol par rapport au dernier rapport.
- **Revue de l'aérodynamicien** sur le contact, les valeurs `E:`, la tenue numérique et les essais.

### 5. Dans le jeu (`crates/game`)

- **Départs**, voir la décision 1 : scénarios `terrain` et `finale` à côté de `vol-essai`. Leur état (repos au sol, plané en finale) vient de `fdm_lab`, marqué `C:`, avec un test d'équilibre comme pour `vol-essai.ron`.
- **Neutre de la profondeur au sol** : pris sur la montée, pour qu'un stick relâché ne plaque ni ne cabre l'avion à l'envol. Valeur calculée.
- **Fin du contact = crash** : `touches_ground` et `crashed` cèdent la place à une lecture des contacts du modèle de vol, selon la décision 4.
- **Animation** : roues tournées selon le chemin parcouru, essieu coulissant selon la compression (nœuds `wheel_*` et `axle`). `asset_check` vérifie en plus le pivot des roues.
- **Caméra** : la poursuite reste au-dessus du sol.
- **Télémétrie** : au sol ou en vol, charge sur les roues et la béquille, compression.

### 6. Aides au sol et tolérance (`crates/game/src/realism/`)

- **Passage obligé** : les aides actuelles ne valent pas au sol, et certaines y seraient nuisibles.
  - **Garde anti-décrochage** : à l'arrêt, l'avion est bien au-delà de l'incidence limite de 10°. La garde pousserait le manche en permanence. Elle se coupe au sol **quand la vitesse ne peut plus porter l'avion**, pas au contact : à 103 km/h, un manche tiré au toucher le renverrait en l'air queue basse, jusqu'au décrochage (écart à ce plan, trouvé par l'atterrissage scripté). Elle reste donc active jusqu'à la fin de l'arrondi : **en Pilote, le posé se fait queue haute**, comme l'autorise T8.
  - **Coordination** : le dérapage n'a pas de sens à l'arrêt, et les gains commencent à 90 km/h. Coupée au contact.
  - **Instructeur** : la commande d'attitude ne vaut pas au sol, voir la décision 3.
- **Tenue de cap**, si elle est retenue : un régulateur sur le gouvernail seul, sans force ajoutée. Ses gains viennent de `fdm_lab assist`, souffle de l'hélice compris, et vont dans `assist_gains.ron`. Le palonnier du joueur s'y ajoute et déplace le cap tenu.
- **Tolérance à l'atterrissage** : champ de `RealismSettings`, marqué non physique, voir la décision 4.
- **Tests de fumée par préréglage** : décollage et atterrissage scriptés. L'As reste sans aucune aide.

### 7. Terrain, gizmos, captures

- **Le pré** : un plan posé sur la plaine, et non dans sa texture, car la tuile de 2 km se répète et dessinerait un terrain tous les 2 km. Voir la décision 2.
- **Gizmos de forces** : flèches par composant dans le plugin `dev` (dernier point du M2 dans la feuille de route), utiles pour le contact.
- **Scènes de référence** : `terrain-repos`, `terrain-pilote` et `terrain-roulage`. Images **regardées**. `vol-essai-*` et `avion-studio` comparées avant/après ; revue du directeur artistique sur le pré.

### 8. Vérification

- `just check`, `just test`, `/essai-en-vol spad7`, revue finale de l'aérodynamicien.
- Docs : `format-avion.md`, `modele-aero.md`, skills concernés, ADR 0008, feuille de route, carnet.
- **Ton essai**, ci-dessous. Retours dans `docs/gdd/retours.md`.

## Décisions demandées au joueur

1. **Départ** :
   - **(A, recommandé)** `just run` démarre au sol. L'écran de pause permet de choisir entre **sol**, **finale** et **en vol**. La finale place l'avion à environ 150 m et 1 500 m du pré (ESTIMÉ) : on répète l'atterrissage sans refaire le décollage ;
   - (B) `just run` démarre en vol, comme aujourd'hui ; le sol et la finale se choisissent en pause.
2. **Le terrain** :
   - **(A, recommandé)** un pré d'herbe plus clair, sans haies, d'environ 800 × 300 m (ESTIMÉ, à corriger selon l'historien), orienté est-ouest, départ cap à l'est. Un T blanc seulement si l'historien l'atteste ;
   - (B) rien de dessiné : on se pose où l'on veut, les haies de la plaine n'étant qu'une texture.
3. **Aides au sol** :
   - **(A, recommandé)** :

     | Préréglage | Au sol | Décollage | Atterrissage |
     |---|---|---|---|
     | Arcade | tenue de cap | gaz à fond : l'instructeur lève la queue et s'envole à 1,2 V_s (ESTIMÉ) | instructeur jusqu'au toucher, sans arrondi automatique, tolérance large |
     | Pilote | tenue de cap | à toi de lever la queue et de tirer | garde jusqu'au toucher, posé queue haute |
     | Vétéran | rien | tout à toi | coordination jusqu'au toucher |
     | As | rien | tout à toi | rien |

   - (B) tenue de cap en Arcade seulement. Pilote exigerait alors de tenir le cap à L2/R2, tout-ou-rien, ce qui met en danger le critère du M2.
4. **Accidents** :
   - **(A, recommandé)** le jeu déclare un accident si l'hélice touche, si un point de structure touche fort, ou si le train dépasse sa charge limite (ESTIMÉE avec l'aérodynamicien). Un frottement léger de saumon passe. Un seul réglage de tolérance, multiplicateur non physique de ces limites : Arcade large, Pilote modéré, Vétéran et As à 1 ;
   - (B) mêmes règles pour tous les préréglages jusqu'au M5.

## Risques

- **Aucune donnée sur les sandows** : le comportement au posé repose sur des valeurs `E:`. Il se juge sur des critères qualitatifs et à ton essai, sans cible chiffrée.
- **Stabilité numérique** : le contact est raide et le frottement à l'arrêt aussi. D'où l'étape 2 et la pente bornée. Un frottement régularisé laisse aussi une dérive très lente à l'arrêt : à mesurer, et à traiter par un état dédié seulement si elle se voit.
- **Palonnier tout-ou-rien** : avec L2/R2, tenir le cap sans aide sera difficile en Vétéran et en As. Le critère du M2 porte sur Pilote. Si l'essai le demande, le palonnier analogique du profil « Avancé » (skill `manette`) passera avant le M5.
- **Le sens de l'embardée** repose sur le couple et le souffle, étalonnés en vol seulement. S'il est faux au sol, on cherche la cause physique, sans retoucher un coefficient pour le faire passer.
- **Coût des sous-pas** : deux fois le modèle aérodynamique par pas de jeu. Sinon, seul le contact sera sous-pas, ce que l'ADR 0008 consignera.
- **Chiffres d'époque absents** : le M2 se clôt avec des essais de sol qualitatifs, comme le M1 l'a fait en confiance C.

## Ton essai

Une fois l'incrément fini, en préréglage Pilote d'abord :

1. **Roulage** : combien de gaz pour démarrer ? L'avion va-t-il droit ? Peut-on tourner au palonnier et aux gaz ?
2. **Décollage** : l'embardée se sent-elle ? La queue se lève-t-elle quand on pousse le manche ? L'envol est-il net ?
3. **Vue du pilote au sol** : le capot bouche-t-il trop la vue, queue basse ?
4. **Tour de piste et finale** : le plané, la lecture de la hauteur, la vitesse d'approche.
5. **Posé** : les rebonds, l'arrêt sans freins, le cheval de bois si l'on insiste.
6. **Les autres préréglages** : Arcade, Vétéran et As, et ton avis sur le centrage du 2026-10-31 si tu ne l'as pas donné avant.
7. **Le vol de dix minutes** : critère de fin du M2.

## Réalisation (2026-11-01)

Les étapes 0 à 7 sont faites, ADR 0008 accepté par le joueur le 2026-11-01. Écarts et ajouts par rapport au plan :

- **Ressort de roue** : il dépend de l'assiette. La glissière du SPAD fait 46° avec la verticale en ligne de vol et 61° queue basse (revue de l'aérodynamicien, point bloquant).
- **Béquille** : elle frotte comme une roue, avec une force latérale bien plus raide. Isotrope, elle laissait l'avion partir seul en cheval de bois à 40 km/h, contre le témoignage de Meister.
- **Tenue de cap** : proportionnelle, dérivée et intégrale, l'intégrale tenant le couple permanent de l'hélice.
- **Garde au sol** : voir l'étape 6. Elle se coupe sous la vitesse de vol, pas au contact.
- **Pré** : texture repoussée vers les échelles de 0,6 à 6 m après la revue du directeur artistique ; masqué pour rester dans la brume ; sans T, faute de source de jour.
- **Chiffres, 729 kg, sans cible historique** :
  - repos : 15°, 294 kg par roue, 130 kg sur la béquille ;
  - envol en 11,7 s, 143 m, 96 km/h ; 15 m à 330 m ;
  - arrêt de 82 km/h en 180 m ;
  - posés : 12 kN par roue à 2 m/s, 20 kN à 3 m/s, 28 kN à 4 m/s, limite à 24 kN.
  - cheval de bois : une seconde de pied à 75 km/h, queue haute, plein gaz, 180° de cap et un saumon au sol.
- **Reste ouvert** :
  - relire l'image de S49 pour l'axe de la course ;
  - frottement dans la glissière à la place du visqueux ;
  - message de posé dur à 14 kN ;
  - bord net du voile de l'hélice ;
  - vitesse de rendu sur la machine du joueur en `just run`.
