# Plan — Aides au pilotage (deuxième incrément du M2)

- Statut : **validé** par le joueur le 2026-10-29, avec les options recommandées :
  - les quatre préréglages ;
  - Pilote au lancement ;
  - le bouton Home pour changer en vol.
- Demande du joueur (2026-10-29, retours du vol d'essai) : « je décroche très facilement sur un virage appuyé car je perds beaucoup de vitesse. Ajouter un mode où l'on aide le pilotage » ; et « pas facile de tenir le palier, il y a une dérive légère sur la gauche ».
- **Jalon** : avance une partie du **M5 « Niveaux de réalisme »** (`docs/roadmap.md`) : les assistances de pilotage seulement, dans le vol d'essai. Règles, présentation et paramètres du modèle viendront plus tard.
- **Règles du projet** (ADR 0003, skill `realisme`) :
  - une aide agit **seulement par les commandes**, comme un pilote : aucune force ajoutée, aucun coefficient changé ;
  - pas de `if arcade` : un champ de `RealismSettings` par réglage, lu par un seul système ;
  - les gains se **calculent** depuis le modèle de vol (linéarisation de `fdm_lab`), sans réglage à la main par avion ;
  - l'aide ne fait pas mieux que l'avion : sans vitesse, il décroche aussi, mais gentiment.

## Pourquoi ces deux symptômes

- **Dérive à gauche** : le palier demande un peu de palonnier et d'aileron à droite, contre le couple et le souffle de l'hélice (palonnier +0,006, aileron +0,003 à 160 km/h). Le neutre réglable ne règle que la profondeur, donc l'avion part lentement à gauche. C'est un comportement réel, que le pilote corrigeait au pied.
- **Décrochage en virage** : à 60° d'inclinaison, il faut 2 g pour tenir l'altitude. La traînée induite est alors multipliée par quatre, et les 150 ch du SPAD ne suffisent plus : l'avion ralentit, et la vitesse de décrochage monte de 41 % (racine de 2). Partant de 160 km/h, il décroche vers 120 km/h dans le virage. C'est le comportement réel de l'avion ; l'aide doit l'en protéger, pas le changer.

## Les préréglages (colonne « Assistance de pilotage » de `docs/vision.md`)

| Préréglage | Aide | Effet en vol d'essai |
|---|---|---|
| **Arcade** « Wings » | **instructeur virtuel** (commande d'attitude) | stick X = inclinaison voulue, stick relâché = ailes à plat ; stick Y = facteur de charge voulu ; coordination et garde anti-décrochage comprises |
| **Pilote** « Red Baron » | **coordination automatique** + **garde anti-décrochage douce** | le palonnier se règle seul pour annuler le dérapage ; le manche ne peut pas tirer au-delà de l'incidence de décrochage moins une marge |
| **Vétéran** | coordination automatique | le palonnier seul |
| **As** « IL-2 » | aucune | le vol d'essai d'aujourd'hui |

Le palonnier du joueur (L2/R2) s'ajoute toujours à celui de la coordination.

## Étapes

### 1. Gains calculés (`fdm_lab`)

- Nouvelle commande `fdm_lab assist <avion>`. À plusieurs vitesses de l'enveloppe, elle linéarise le modèle de vol autour du palier et en tire les dérivées de commande :
  - roulis par aileron ;
  - tangage et facteur de charge par profondeur ;
  - lacet et dérapage par gouvernail.
- **Gains** des régulateurs (proportionnel et dérivé, plus intégral pour la coordination) : choisis pour une réponse donnée, c'est-à-dire une pulsation et un amortissement fixés, puis rapportés à la pression dynamique pour valoir à toute vitesse.
- **Incidence de décrochage** de la cellule : c'est l'incidence du maximum de portance, en vol équilibré.
- **Fichier** `assets/aircraft/spad7/assist_gains.ron`, valeurs marquées `C:` (calculées). Un test de `fdm_lab` vérifie qu'il suit le modèle de vol, comme pour l'état de départ.

### 2. Réglages et préréglages (`crates/game/src/realism/`)

- **`RealismSettings`**, avec un seul champ pour l'instant, `flight_assist` : `Instructor`, `Coordinated { stall_guard }` ou `Off`.
- **Préréglages** : Arcade, Pilote, Vétéran, As. Chaque champ dit ce qu'il change ; un futur réglage non physique sera marqué comme tel.
- **Choix en vol** : voir la décision 3. Le préréglage actif s'affiche dans la télémétrie.

### 3. Les aides (`crates/game/src/realism/assist.rs`)

- **Place** : au pas fixe, entre la couche d'actions et le modèle de vol (`RealismSet::Assist` avant `FlightSet::Step`). Elles lisent l'état de l'avion (incidence, dérapage, inclinaison, vitesses de rotation) et écrivent des positions de commandes.
- **Coordination automatique** : le gouvernail annule le dérapage (régulateur proportionnel et intégral sur β, plus le palonnier du joueur).
- **Garde anti-décrochage douce** : au-delà de l'incidence limite (décrochage moins 2°), la profondeur est ramenée progressivement. Le manche tiré à fond ne fait plus décrocher. En revanche, sans vitesse, l'avion s'enfonce.
- **Instructeur virtuel** :
  - **inclinaison** : le stick en X demande jusqu'à 60° ; l'aileron régule l'inclinaison ;
  - **tangage** : le stick en Y demande un facteur de charge de 0 à 3,5 g autour du palier ; la profondeur le régule, dans la limite de la garde ;
  - **coordination** comprise ;
  - **gaz** : ceux du joueur, sans automate à cet incrément.
- **Tests sans rendu** :
  - tests de fumée par préréglage (skill `realisme`) : manche à fond en arrière, roulis à fond, gaz coupés en montée. En Arcade et Pilote, ni vrille ni décrochage franc ; en As, le comportement réel ;
  - test d'absence de triche : en As, aucune aide active ;
  - test de cohérence : aides coupées, même trajectoire qu'aujourd'hui.

### 4. Vérification et essai

- `just check`, `just test` ; revue de l'**aérodynamicien** sur les gains, l'incidence de décrochage, la vitesse de décrochage de 85 km/h, et l'équilibre manche au neutre à 88 km/h.
- **Ton essai** : virages à 60° en Pilote et en Arcade, palier mains libres en Arcade, comparaison avec As.

## Décisions demandées au joueur

1. **Quelles aides** :
   - **(A, recommandé)** les quatre préréglages de la vision, Arcade, Pilote, Vétéran et As ;
   - (B) seulement Pilote (coordination et garde), en interrupteur ;
   - (C) seulement l'instructeur d'Arcade.
2. **Préréglage au lancement** : **Pilote (recommandé)**, Arcade ou As.
3. **Changer de préréglage en vol** :
   - **(A, recommandé)** le bouton Home de la SN30 Pro fait défiler les préréglages, avec un message à l'écran ;
   - (B) un choix dans l'écran de pause.

## Risques

- **Gains** : un régulateur mal amorti fait osciller l'avion. Les gains calculés seront vérifiés en simulation à plusieurs vitesses avant ton essai.
- **Instructeur et décrochage** : à basse vitesse, la demande de facteur de charge est bornée par la garde ; l'avion perd de l'altitude plutôt que de décrocher.
