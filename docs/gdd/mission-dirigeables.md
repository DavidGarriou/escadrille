# Mission : chasse aux dirigeables

- État : proposition, à valider par le joueur
- Dernière mise à jour : 2026-10-16
- Skills concernées : `combat`, `simulation-vol`, `rendu-visuel`, `realisme`, `recherche-historique`

## 1. Objectif et pilier

Intercepter un dirigeable ennemi (Zeppelin ou Schütte-Lanz) et l'abattre en flammes, ou le contraindre à se poser pour le capturer.

| Pilier | Ce que la mission lui apporte |
|---|---|
| 3. Le ciel | un objet de près de 200 m au-dessus d'une mer de nuages ; de nuit, un embrasement qui éclaire les nuages et se voit de très loin |
| 1. Sentir le vol | une montée aux limites du plafond, moteur essoufflé, commandes molles dans l'air raréfié |
| 4. Histoire humaine | un équipage d'une vingtaine d'hommes ; une victoire immense et terrible, racontée sobrement |
| 5. Authenticité | l'hydrogène ne s'enflamme pas avec n'importe quelle balle ; munitions spéciales selon la nation et la date |

Risque pour le plaisir de jeu : un dirigeable qui monte hors de portée peut rendre la mission frustrante. La conception doit en faire une **course tendue** (couper la route, anticiper le largage de lest), jamais une impasse sans recours.

## 2. Expérience du joueur

> Octobre 1917, au petit matin. Le téléphone sonne au mess : des Zeppelins dérivent vers le sud, très haut, au retour d'un raid sur l'Angleterre. Décollage dans la brume, puis une montée interminable : 3 000 m, 4 000 m, le moteur s'essouffle et le froid mord les doigts. Au-dessus des nuages apparaît une forme argentée immense, bien plus loin qu'elle n'en a l'air. Les mitrailleurs des nacelles ouvrent le feu. Le dirigeable largue son lest et remonte d'un coup : inutile de le suivre, il faut lui couper la route. Faut-il l'incendier, ou le harceler jusqu'à ce qu'il se pose, à court de gaz et d'essence ?

Variante de nuit : les projecteurs fouillent le ciel, l'un d'eux accroche la coque ; le joueur monte vers ce point argenté, vide un tambour au même endroit, puis une lueur rose grandit à l'intérieur de l'enveloppe.

## 3. Règles et systèmes

### Le dirigeable

- **Simulé, pas scripté** : module `airship` de `escadrille_fdm` (flottabilité, lest, soupapes, altitude de pression, très grande inertie, dérive au vent ; voir skill `simulation-vol`).
- **Piloté par une IA avec ses propres commandes** (gouvernes, moteurs, lest, soupapes, largage des bombes), sans triche, comme les avions.
- **Données sourcées** dans `assets/airships/<id>/airship.ron` : dimensions, volume de gaz, nombre de cellules, masses, lest, moteurs, plafond, armement défensif, postes de tir.
- Les dirigeables « grimpeurs » de 1917 montent très haut, souvent au-delà de ce que les chasseurs atteignent facilement : la mission place l'interception là où elle est possible (approche, dérive, descente pour naviguer, moteurs en panne), comme dans l'histoire.

### Équipage (IA)

- **Commandant** : suit sa route ; sous attaque, largue du lest ou ses bombes pour monter, cherche les nuages, change de cap ; renonce et tente de se poser si le gaz, l'essence ou les moteurs manquent.
- **Mitrailleurs** : postes selon la classe (nacelles, plateforme dorsale, queue), champs de tir et **angles morts** issus de la recherche. Le joueur apprend les bons angles d'approche.
- **État de l'équipage** : froid, manque d'oxygène, fatigue, qui dégradent la précision et les décisions, et provoquent des pannes.

### Dégâts : l'hydrogène

Système générique « enveloppe d'hydrogène », **partagé avec les ballons d'observation** (M7) :
- La coque est divisée en **cellules de gaz** et en structure (anneaux, longerons).
- **Balle ordinaire** : perforation et fuite lente. Le dirigeable s'alourdit peu à peu ; criblé, il peut encore rentrer.
- **Inflammation** : il faut un mélange hydrogène-air (autour des fuites) **et** une source chaude (balle incendiaire ou explosive). La probabilité dépend de la concentration locale et du type de munition. Conséquence émergente et historique : le tir concentré au même endroit crée les fuites puis les enflamme.
- **Propagation** de cellule en cellule, effondrement de la structure, chute lente en flammes, débris ; la structure en feu devient visible.

### Munitions et armes

- **Composition des bandes et tambours** choisie à la préparation : ordinaires, traçantes, incendiaires, explosives, selon la nation et la date.
- **Fusées Le Prieur** (françaises, surtout contre les ballons) : usage contre les dirigeables à vérifier.
- **Bombes larguées d'en dessus** (variante 1915).

### Détection et navigation

- **Réseau de guet** : signalements par téléphone (position approximative, cap, altitude estimée), mis à jour régulièrement et affichés sur la carte du briefing.
- **De nuit** : projecteurs au sol, silhouette contre la lune et les étoiles, lueur des échappements, bourdonnement des moteurs entendu avant d'être vu.
- **Distance trompeuse** : la taille du dirigeable le fait paraître proche ; les débutants tirent trop loin, comme les pilotes de l'époque.

### Issues possibles

| Issue | Résultat |
|---|---|
| Abattu en flammes | victoire facilement confirmée (vue de très loin), honneurs importants |
| Contraint de se poser | capture : récompense plus forte (renseignement), plus difficile à obtenir |
| Échappé | il regagne ses lignes ou la mer |
| Retour difficile | froid, carburant bas après la longue montée, terrain inconnu : atterrissage en campagne possible |

## 4. Variations selon le réalisme

| Aspect | Arcade | Pilote | Vétéran | As |
|---|---|---|---|---|
| Munitions | incendiaires automatiques, inflammation rapide ⚠ | composition automatique | composition à choisir | à choisir, chronologie stricte |
| Altitude du dirigeable | scénario placé à portée | scénario favorable | réelle | réelle |
| Effets de l'altitude (froid, hypoxie) | non | atténués | oui | oui |
| Localisation | marqueur permanent | marqueur à courte distance | carte et signalements | signalements seulement |
| Mitrailleurs de bord | peu précis | précision réduite | réalistes | réalistes |
| Projecteurs (nuit) | accrochent vite | aidants | réalistes | réalistes |

⚠ = non physique. Adapter l'altitude d'un scénario n'est pas non physique : c'est un choix de mission.

## 5. Paramètres de réglage

- `assets/airships/<id>/airship.ron` : classe, géométrie, gaz, cellules, masses, lest, moteurs, armement (sourcé).
- `assets/combat/hydrogene.ron` : débit de fuite par trou, seuil de mélange inflammable, probabilité d'inflammation par type de munition (`ESTIMÉ`, calée sur les témoignages), vitesse de propagation.
- `assets/missions/dirigeable_*.ron` : délai et précision des signalements, météo, heure, nombre de dirigeables, route.
- IA : agressivité du commandant, seuil d'abandon, précision des mitrailleurs.

## 6. Ce qu'on mesure en essai

- Temps jusqu'au contact (accélération du temps comprise).
- Part des sorties où le joueur atteint l'altitude du dirigeable, en Pilote et en As.
- L'émerveillement : le joueur en parle-t-il spontanément ?
- La frustration : un dirigeable qui s'échappe en montant est-il perçu comme juste ?
- La lisibilité : le joueur comprend-il pourquoi ses balles ne l'enflamment pas ?
- Les performances : dirigeable en feu, nuages et nuit dans le budget GPU.

## 7. Questions ouvertes

1. Théâtre de la première variante historique (voir ci-dessous).
2. La capture : objectif à part entière ou bonus ?
3. Ton du carnet face à la mort d'un équipage entier.
4. Jouer un jour l'autre camp (défendre le dirigeable) : hors périmètre proposé.

## Variantes historiques

**À vérifier et à sourcer dans `docs/recherche/dirigeables.md` (subagent `historien`) avant toute intégration.**

| Variante | Contexte | Joueur | Prérequis |
|---|---|---|---|
| **Raid silencieux** (France, octobre 1917) | Au retour d'un raid sur l'Angleterre, des Zeppelins de la Marine sont dispersés par le vent au-dessus de la France ; le L 49 est contraint de se poser près de Bourbonne-les-Bains par des Nieuport de l'escadrille N 152 et capturé intact | pilote français | carte de l'est de la France (Haute-Marne), Nieuport de N 152 (type exact à vérifier), mission de jour |
| **Côte flamande** (1915-1916) | Dirigeables de l'Armée basés en Belgique ; en juin 1915, Warneford (RNAS) détruit le LZ 37 près de Gand en le bombardant par-dessus | pilote du RNAS à Dunkerque, ou français | extension de la carte des Flandres vers Gand, avions de 1915, bombes légères |
| **Défense de Londres** (1916-1918) | Défense de nuit de l'Angleterre ; en septembre 1916, le Schütte-Lanz SL 11 est abattu avec des munitions explosives et incendiaires | pilote britannique | carte du sud-est de l'Angleterre, rendu de nuit, projecteurs |
| **Mer du Nord** (1918) | Chasseur lancé depuis une barge remorquée par un destroyer, contre un dirigeable de reconnaissance | pilote britannique | mer, barge et destroyer, amerrissage |

## Recommandation

1. **Prototyper les mécaniques en « combat rapide »** sur la carte des Flandres existante, scénario libre signalé comme non historique : aérostat, hydrogène, IA d'équipage.
2. **Première variante historique : le Raid silencieux.** Elle s'accorde avec une carrière française et l'année 1917, et se joue de jour (pas besoin du rendu de nuit). Elle demande une seconde carte, produite avec le même outil `terrain_bake`.
3. Ensuite, la nuit (Londres) et la mer.

## Dépendances

- Système hydrogène générique conçu dès les ballons d'observation (M7).
- Module `airship` dans `escadrille_fdm`, avec ses propres essais.
- Rendu d'un objet de 200 m (LOD, ombres sur les nuages), feu à grande échelle qui éclaire la scène, structure visible en feu ; de nuit : faisceaux de projecteurs volumétriques, lune, étoiles.
- Réglage de réalisme « effets de l'altitude » (froid, hypoxie).
- Modèle 3D du dirigeable avec cellules, nacelles et postes de tir nommés (skill `pipeline-assets`).
- Avions et cartes propres à chaque variante.

## Découpage en tâches

1. Fiche de recherche `docs/recherche/dirigeables.md` : classes, dimensions, plafonds, armement défensif et angles morts, tactiques de défense et d'attaque, munitions spéciales par nation et par date.
2. Module `airship` du FDM, avec essais (montée après largage de lest, dérive au vent, altitude de pression).
3. Système hydrogène générique (cellules, fuites, inflammation, propagation) et ses tests.
4. IA d'équipage (commandant, mitrailleurs).
5. Modèle 3D provisoire aux bonnes dimensions, rendu en feu, scènes de référence.
6. Mission « combat rapide ».
7. Variante historique « Raid silencieux » : carte, chronologie, gabarits du carnet.
