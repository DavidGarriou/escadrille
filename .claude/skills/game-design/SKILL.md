---
name: game-design
description: Game design d'Escadrille — piliers, boucle de mission, carrière façon Wings et Red Baron (carnet de vol, escadrille, revendications), conception des missions, sensations de jeu (retours, vibrations, caméra), prise en main et tutoriels, accessibilité, rédaction des documents de design et préparation des sessions d'essai du joueur. À charger pour concevoir ou modifier une fonctionnalité de jeu, une mission, la progression, ou pour préparer un essai.
---

# Game design

Vision et piliers : `docs/vision.md`. Toute proposition se rattache à au moins un pilier et précise son comportement à chaque niveau de réalisme.

## Évaluer une idée

1. Quel pilier sert-elle ? Lequel risque-t-elle d'abîmer ?
2. Que vit le joueur, en une phrase, manette en main ?
3. Comment se comporte-t-elle en Arcade et en As ?
4. Est-elle historiquement plausible ? Sinon, est-ce assumé et signalé ?
5. Quel est le plus petit prototype qui permet de la juger en essai ?

## Sensations de jeu

- Chaque événement important a un retour sur **plusieurs canaux** : image, son, vibration, carnet.
- Coup au but : éclats de toile, fumée, son d'impact ; avion abattu : chute qui dure, fumée visible de loin, entrée au carnet.
- Caméra : secousses légères (moteur, tir, impacts), proportionnées au réalisme ; jamais au détriment de la visée.
- En réalisme élevé, les retours restent sobres mais présents : c'est l'information du pilote.

## Missions

- Structure : objectif principal, complications possibles (météo, panne, ennemi supérieur en nombre, ailier perdu), condition de retour.
- Durée de 10 à 25 minutes ; accélération du temps et saut jusqu'au contact en option.
- Chaque mission produit une **histoire courte** que le carnet peut raconter.
- Types de mission : tableau de `docs/vision.md`.

## Carrière

- Carnet de vol généré par des gabarits Fluent à partir des événements de la mission (lieu, météo, ennemis, pertes, revendications) ; ton sobre, première personne.
- Escadrille vivante : camarades avec nom, caractère, palmarès ; pertes et remplaçants ; relations simples (le vétéran qui veille sur le nouveau).
- Progression : grades, décorations, nouveaux avions selon la chronologie, liberté de choisir sa livrée personnelle quand on devient as.

## Prise en main

- Première question « Comment voulez-vous voler ? » (trois portes, `docs/vision.md`).
- Tutoriels intégrés à l'histoire : école de pilotage, puis premiers vols en escadrille sous la surveillance d'un ancien.
- Les aides proposent d'elles-mêmes de baisser d'un cran quand le joueur progresse ; jamais imposé.

## Accessibilité

Remappage complet, sous-titres, interface lisible pour les daltoniens, pause à tout moment en solo, vibrations réglables, tailles de texte.

## Documents de design

Un document par fonctionnalité dans `docs/gdd/<fonctionnalité>.md` :
1. Objectif et pilier
2. Expérience du joueur (récit d'une minute de jeu)
3. Règles et systèmes
4. Variations selon le réalisme
5. Paramètres de réglage (et où ils vivent)
6. Ce qu'on mesure en essai
7. Questions ouvertes

## Essais du joueur

Claude ne peut pas juger du plaisir de jeu : à la fin d'un jalon ou d'une fonctionnalité importante, préparer une **fiche d'essai** (quoi tester, dans quel ordre, avec quel préréglage, questions à se poser) et consigner les retours dans `docs/gdd/retours.md`, avec la date et les actions décidées.
