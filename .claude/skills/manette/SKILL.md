---
name: manette
description: Entrées du jeu et manette 8BitDo SN30 Pro — couche d'actions, profils de commandes, zones mortes et courbes de réponse, gâchettes analogiques ou tout-ou-rien, palonnier et gaz sur une manette, vibrations, labo de diagnostic, remappage, autres périphériques (clavier, HOTAS, opentrack). À charger pour toute tâche dans crates/game/src/input ou qui ajoute une commande.
---

# Manette et entrées

## La SN30 Pro : ce qui est sûr, ce qui reste à vérifier

Sûr :
- Disposition de type Super Nintendo : **B en bas, A à droite, Y à gauche, X en haut**.
- Deux sticks cliquables (L3, R3), croix, L1/R1, L2/R2, Select, Start, Home, bouton étoile.
- Connexion en USB-C (filaire) ou Bluetooth.

**À vérifier au M0 avec le labo manette** (consigner les résultats ici même) :
- **Gâchettes L2/R2** : en mode X-input, elles remontent comme des axes, mais rien ne garantit une course réellement progressive sur ce modèle (les tests publiés portent surtout sur la SN30 Pro+). Le jeu **ne doit jamais exiger** de gâchettes analogiques.
- Vibrations sous Linux.
- Mode le plus fiable sous Linux (X-input à priori) et combinaison de démarrage correspondante (manuel 8BitDo).
- Correspondance des boutons : en mode X-input, le bouton du bas (B) devrait remonter comme le bouton « sud » (le A d'une manette Xbox).

Conséquences :
- **Toujours mapper par position** (sud, est, ouest, nord), jamais par étiquette.
- **Afficher les étiquettes 8BitDo** (disposition Nintendo) dans l'interface : jeu de pictogrammes « SN30 » sélectionné automatiquement ou dans les options.
- Sticks courts et petits : la précision vient des **courbes de réponse** et des zones mortes bien réglées.
- Sticks à retour au centre : **les gaz ne peuvent pas être une position de stick**. Ils sont incrémentaux.

## Architecture

```
gilrs (via Bevy)
  → échantillonnage (Update)
  → calibration (centre, amplitude, gâchettes analogiques ou non)
  → zone morte radiale + anti-zone morte
  → courbe de réponse (expo) par axe
  → lissage court (optionnel)
  → actions (enum Action → valeur ou impulsion), accumulées jusqu'au pas fixe
  → couche d'assistances (skill `realisme`)
  → commandes du FDM (manche, palonnier, gaz, coupe-contact, mélange…)
```

- Le code de jeu ne lit **que des actions**, jamais un bouton.
- Profils en RON (`assets/input/profils/*.ron`), remappage en jeu, sauvegarde dans la configuration utilisateur.
- Zone morte radiale : x' = (|x| − d) / (1 − d) si |x| > d, sinon 0. Courbe : y = signe(x) × ((1 − k) |x'| + k |x'|³), k réglable par axe.
- Palonnier sur boutons tout-ou-rien : valeur limitée en vitesse vers ±1 (montée réglable, environ 0,3 à 0,5 s), retour au centre plus rapide.
- Gaz incrémentaux : maintien = vitesse de variation (pourcentage par seconde), appui bref = pas de 5 %.
- Option « commandes qui durcissent » : l'autorité maximale du manche diminue avec la pression dynamique, comme les efforts réels ; elle varie d'un avion à l'autre (fiche de recherche).

## Profil par défaut « Pilote » (point de départ, à ajuster en essai)

| Commande | En vol | Avec L2 maintenu (couche « Maj ») |
|---|---|---|
| Stick gauche | manche : tangage (Y), roulis (X) | identique |
| Stick droit | regard libre | Y : gaz ; X : palonnier analogique |
| R3 | recentrer le regard | — |
| L1 / R1 | palonnier gauche / droite (rampe) | — |
| R2 | tir | tir |
| L2 | couche Maj | — |
| Croix ↑ / ↓ | gaz + / − | trim de profondeur, si l'avion en a un |
| Croix ← / → | coup d'œil rapide à gauche / à droite | mélange − / + (rotatifs) |
| A (droite) | coupe-contact (maintenu) | — |
| B (bas) | verrouiller le regard sur une cible (padlock) | — |
| Y (gauche) | changer de vue (cockpit, poursuite, externe) | — |
| X (haut) | désenrayer, changer de tambour Lewis | — |
| Start | pause | — |
| Select | carte et carnet | — |

Variantes :
- **Arcade** : le stick gauche commande l'attitude (instructeur virtuel), gaz automatiques par défaut, palonnier inutile (coordination automatique).
- **Avancé** : palonnier analogique permanent sur le stick droit (X) ; regard libre avec Maj + stick droit.

## Vibrations

Le joueur à la manette n'a pas de manche à retour d'effort : les vibrations le remplacent.
- **Tremblement à l'approche du décrochage** (le plus important : c'est l'alerte principale).
- Moteur : vibration légère modulée par le régime, ratés, coupe-contact.
- Tir des mitrailleuses, impacts reçus, atterrissage dur, rupture.
- Intensité réglable, désactivable. API de vibration de la version de Bevy épinglée : vérifier.

## Labo manette (`just labo-manette`, livrable du M0)

Valeurs brutes et traitées de chaque axe, état des boutons, histogramme des valeurs de gâchette (permet de savoir si elles sont progressives), test de chaque moteur de vibration, réglage des zones mortes et des courbes en direct. Il sert aussi au joueur pour diagnostiquer sa manette.

## Robustesse

- Branchement à chaud ; déconnexion (Bluetooth) en vol → pause automatique et message.
- Plusieurs manettes : la dernière utilisée devient active.
- Test unitaire de la chaîne de traitement (zones mortes, courbes, rampes) sans matériel.

## Plus tard

Clavier et souris (déjà là en secours pour le dev), HOTAS et palonnier (vérifier que gilrs les gère, sinon un autre backend open source), suivi de tête **opentrack** (open source, protocole UDP simple : six valeurs par paquet), retour d'effort.
