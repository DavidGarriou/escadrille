# 0008 — Contact au sol : efforts de pénalité dans le RK4, à 240 Hz

- Statut : Accepté (validé par le joueur le 2026-11-01)
- Date : 2026-11-01

## Contexte

Le plan `docs/plans/m2-train-sol.md` ajoute le train et le sol au modèle de vol. L'ADR 0005 renvoyait à un nouvel ADR le traitement du contact, « à décider avec mesures ».

Contraintes :
- pas de moteur physique généraliste (`CLAUDE.md`) ;
- déterminisme au bit près (ADR 0005) ;
- le cheval de bois, le rebond et le capotage doivent émerger, sans script (skill `simulation-vol`) ;
- le même pas pour le jeu et les essais : `fdm_lab` calculait à 240 Hz, le jeu à 120 Hz ;
- coût mesuré : 1,0 ms par pas RK4 sur le Ryzen 7 7800X3D de référence.

## Options

1. **Efforts de pénalité dans le RK4** : chaque point de contact est un ressort-amortisseur, avec un frottement régularisé. Simple, déterministe, sans état ajouté, et ordinaire pour le modèle de vol : le contact n'est qu'un composant de plus.
   - Limite : un contact raide et un frottement sec se heurtent à la stabilité de l'intégrateur explicite. Le frottement sec n'existe pas vraiment : sous une force constante, l'avion dérive très lentement à l'arrêt.
2. **Solveur de contraintes ou impulsions** : vrai frottement statique, contact sans enfoncement.
   - Limites : un second intégrateur dans le modèle de vol, des événements dans les étapes de RK4 alors que `loads` doit rester sans effet de bord, et beaucoup plus de code pour une physique de roue où l'enfoncement a un sens (le pneu et le sandow cèdent vraiment).
3. **États de poils** (modèle de Dahl ou LuGre) : un état par contact pour le vrai frottement statique, avec l'option 1.
   - Limites : deux états par contact, à intégrer et à sauvegarder. Rien ne le justifie tant que la dérive ne se voit pas.
4. **Sous-pas réservés au contact**, à 480 Hz ou plus, le vol restant à 240 Hz.
   - Limite : inutile d'après les mesures ci-dessous.

## Décision

**Option 1 : efforts de pénalité dans le RK4, à 240 Hz**, dans `escadrille_fdm::ground`.

- `Aircraft::advance` fait les deux sous-pas de l'ADR 0005, et le jeu l'appelle à 120 Hz. `Aircraft::step` reste un pas RK4 simple, pour les outils.
- Le frottement s'annule linéairement sous `CREEP_SPEED` (0,03 m/s), sans aucune division par une vitesse. Sa pente croît avec la charge : le mode couplé des roues, du roulis et du lacet vaut 2,1 au pas de 240 Hz, sous la limite de 2,785 du RK4 ; il valait 3,2 à 0,02 m/s, avec un état au repos incohérent.
- L'amortissement entre sur 2 cm de compression : un pneu ne donne aucune charge à l'instant où il touche.
- Un environnement peut ne pas avoir de sol (`NO_GROUND`) : c'est le cas de tous les essais en vol, dont les états au niveau de la mer n'ont pas de sens avec des roues.

Raison principale : le contact reste un composant comme les autres, déterministe et sans état, et la mesure montre qu'il tient au pas du vol.

## Mesures (SPAD VII, 2026-11-01)

- **Convergence** : diviser le pas par deux change la distance d'envol et la charge maximale d'un posé à 2 et 4 m/s de moins de 1 % (test de `fdm_lab`).
- **Dérive à l'arrêt** : 1,4 cm/s au ralenti, soit 84 cm par minute.
- **Coût** : 2 ms par pas de jeu pour un avion, soit un quart de cœur.

## Conséquences

- Le jeu calcule enfin comme les essais : ce qui est validé est ce que le joueur pilote.
- Une dérive à l'arrêt existe. Si elle se voit, l'option 3 s'ajoutera sans changer le reste.
- Le coût double pour chaque avion : à revoir au M4 avec l'IA. L'ADR 0005 prévoit un pas plus long pour les avions lointains.
- Le sol reste plat, et sa normale verticale. Le terrain réel demandera une pente et une nature de sol dans `Environment`.
