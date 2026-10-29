---
name: combat
description: Combat aérien 1914-1918 — mitrailleuses (Vickers, LMG 08/15, Lewis), synchronisation avec l'hélice, enrayages, balistique, munitions spéciales, détection des impacts, modèle de dégâts par composants, feu, blessures du pilote, aérostats (ballons d'observation et dirigeables, hydrogène), IA de chasse et IA d'équipage de dirigeable, revendications de victoires. À charger pour toute tâche dans crates/game/src/combat ou qui touche aux armes, aux dégâts, aux ballons, aux dirigeables ou à l'IA.
---

# Combat

Principe : **tout passe par la physique et par les commandes**. Une balle est un projectile simulé, un dégât modifie un composant du FDM, l'IA pilote avec le manche.

## Armes

Données par arme dans `assets/weapons/<id>.ron`, sourcées comme les avions : cadence, vitesse initiale, masse et coefficient balistique de la balle, alimentation (bande, tambour de 47 ou 97 cartouches pour la Lewis), dispersion, probabilité d'enrayage selon la chaleur et la longueur des rafales, position et réglage de convergence sur l'avion.

- **Synchronisation** (tir à travers l'hélice) : la cadence effective dépend du régime moteur ; un mécanisme endommagé peut tirer dans sa propre hélice. Avant la synchronisation : plaques déflectrices (Garros) ou Lewis au-dessus de l'aile supérieure (Nieuport).
- **Enrayages** : désenrayage par une action du joueur (durée, réussite non garantie) ; changement de tambour Lewis à la main en réalisme élevé.
- **Munitions spéciales** (incendiaires, explosives, traçantes) : surtout contre les ballons et les dirigeables. Composition des bandes et tambours choisie à la préparation selon le réalisme ; types disponibles selon la nation et la date (fiche de recherche).

## Balistique

- État par balle : position, vitesse (f64). Traînée quadratique (coefficient balistique), gravité, vent ; vitesse héritée du tireur.
- Simulation dans `FixedUpdate` ; détection par **segment** entre deux pas (pas d'effet tunnel).
- Stockage compact (structure de tableaux, pool), **pas une entité ECS par balle**. Le visuel des traçantes est séparé (skill `rendu-visuel`).
- Déterminisme : dispersion tirée d'un générateur avec graine.
- **Visée et réglage de tir** : les viseurs du modèle 3D sont construits sur la ligne de visée qui passe par l'œil du pilote (`pilot_eye_m` de `maquette.ron`), parallèle à l'âme ; sur le SPAD, elle passe 12 cm au-dessus. Le réglage de tir (la ligne de visée croise la trajectoire à une distance donnée, à sourcer) reste à faire avec les armes. Il incline l'arme ou les viseurs, et le script Blender doit suivre.

## Détection des impacts

- **Volumes d'impact** par composant, nommés `hit_<composant>` dans le glTF (skill `pipeline-assets`) : moteur, réservoir, pilote, longerons, câbles de commande, gouvernes, hélice, mitrailleuses, train.
- Phase large par boîtes englobantes, phase fine par `parry3d-f64` (rayon contre forme convexe).
- Résultat : `Hit { component, energy, position, normal }` traité par `DamageSet`.

## Modèle de dégâts

| Composant | Effets |
|---|---|
| Toile des ailes | trous → perte de portance, traînée accrue ; déchirure qui s'agrandit avec la vitesse |
| Longerons, mâts, haubans | résistance réduite → rupture sous charge (skill `simulation-vol`, structure) |
| Câbles de commande | câble coupé → gouverne libre, perte d'une commande |
| Moteur | ratés, perte de puissance, fuite d'huile, grippage ; rotatifs : cylindre arraché, balourd |
| Réservoir | fuite, **feu** (la terreur de l'époque ; pas de parachute côté allié, rares côté allemand en 1918) |
| Pilote | blessure : vision troublée, force réduite sur les commandes, perte de connaissance |
| Hélice | balourd → vibrations → rupture du bâti moteur |
| Armes, train | enrayage, arme détruite ; atterrissage difficile |

Le feu se propage, la fumée se voit de loin ; une glissade ou un piqué peut parfois l'éteindre.

## IA de chasse

- **Même FDM, mêmes commandes que le joueur.** Un test vérifie que l'IA n'a accès qu'aux commandes et à ses perceptions.
- **Perception** : cône de vision, distance, contraste ; masquage par les nuages (fonction de densité CPU), **éblouissement par le soleil** (attaquer depuis le soleil fonctionne contre l'IA comme contre un humain) ; mémoire des contacts.
- **Décision** : IA à utilité ou arbre de comportement. Doctrine inspirée des **Dicta Boelcke** (avantage avant d'attaquer, soleil dans le dos, tirer de près, ne pas lâcher sa cible, se retourner face à l'attaque…).
- **Manœuvre** : contrôleurs partagés avec l'instructeur virtuel (skill `realisme`) : poursuite, mise en virage, ressource, rupture, piqué pour fuir.
- **Personnalités** : temps de réaction, erreur de visée, agressivité, gestion de l'énergie, tolérance au facteur de charge, discipline de formation. Un novice fait des erreurs (fixation sur la cible, décrochage) ; un as se repère à son comportement.
- **Formations** : en V, en échelon, patrouilles d'escadrille ; signaux du chef par gestes et battements d'ailes (pas de radio en 14-18).

## Aérostats : ballons et dirigeables

Conception de la mission : `docs/gdd/mission-dirigeables.md`.

- **Un seul système « enveloppe d'hydrogène »** pour les ballons d'observation et les dirigeables : enveloppe découpée en cellules de gaz (`gas_cell_<n>` dans le glTF), chacune avec volume, pression, fuites et concentration de mélange hydrogène-air autour des trous.
- **Balle ordinaire** : fuite lente, perte de portance progressive. **Inflammation** seulement si une balle incendiaire ou explosive traverse une zone de mélange : probabilité fonction de la concentration et du type de munition. **Jamais** de « points de vie » de dirigeable.
- **Propagation** du feu de cellule en cellule, effondrement de la structure, chute en flammes (le dirigeable reste un corps simulé jusqu'au sol), débris dangereux pour l'attaquant.
- **IA d'équipage** : un commandant (route, largage de lest ou de bombes pour monter, nuages, abandon et atterrissage forcé) et des mitrailleurs par poste, avec champs de tir et angles morts sourcés. Mêmes règles que pour les avions : commandes seulement, perception réelle (froid, manque d'oxygène, nuit, projecteurs).
- **Capture** : un dirigeable posé intact est une issue de mission à part entière.
- Paramètres dans `assets/combat/hydrogene.ron` ; les probabilités d'inflammation sont `ESTIMÉ`, calées sur les témoignages et documentées.

## Aides selon le réalisme

Indicateur de tir (Arcade), dégâts reçus réduits, enrayages coupés, munitions spéciales automatiques : tous via `RealismSettings` (skill `realisme`).

## Revendications et confirmations

Une victoire n'est confirmée qu'avec des **témoins** (ailier, observateurs au sol, autre escadrille) ou une épave dans nos lignes. Le débriefing compare ce que le joueur revendique à ce que la simulation a enregistré, et le carnet garde la trace des victoires « probables ».

## Outils de debug

Trajectoires des balles, volumes d'impact, état des dégâts par composant, perception de l'IA (cône, cibles connues, raison de la décision courante).
