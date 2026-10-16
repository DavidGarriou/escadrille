---
name: audio
description: Son d'Escadrille — synthèse procédurale des moteurs rotatifs et en ligne (kira + fundsp), vent et chant des câbles, mitrailleuses, impacts, DCA, artillerie lointaine, spatialisation, mixage, musique et outils open source. À charger pour toute tâche dans crates/game/src/audio ou assets/audio.
---

# Audio

Le son porte une grande part de l'immersion, et le joueur à la manette s'en sert pour piloter : régime, décrochage, état du moteur.

## Moteurs (procéduraux, pilotés par le FDM)

- Synthèse avec `fundsp`, jouée par `kira` (vérifier comment brancher un flux audio personnalisé dans la version utilisée), mélangée à des échantillons libres pour la texture.
- Paramètres : régime, charge, nombre de cylindres et ordre d'allumage, ratés, coupe-contact, mélange.
- **Rotatifs** (9 cylindres) : le coupe-contact donne le motif haché caractéristique ; ratés si le mélange est mal réglé.
- **En ligne** : caractère propre à chaque moteur (6 cylindres Mercedes, V8 Hispano-Suiza).
- Démarrage à l'hélice, toussotements, arrêt, hélice qui tourne en moulinet.

## Écoulement

- Vent fonction de la vitesse indiquée et de l'incidence.
- **Chant des câbles** : fréquence d'un son éolien f = St × V / d (St ≈ 0,2 pour un cylindre, d = diamètre du câble). La hauteur monte avec la vitesse : un indicateur de vitesse à l'oreille, authentique.
- Tremblement et bruit de buffeting à l'approche du décrochage ; toile déchirée qui claque.

## Armes et impacts

Vickers, LMG 08/15 et Lewis bien distinctes ; mécanique dans le cockpit ; tir lointain ; impacts sur toile, bois et métal ; claquement des balles qui passent près.

## Environnement

DCA (« Archie »), grondement continu de l'artillerie près du front (présence sonore de la guerre), oiseaux et mécaniciens sur le terrain à l'aube, pluie sur la toile. Dirigeables : bourdonnement grave et lointain de leurs moteurs, entendu avant de les voir la nuit ; souffle sourd de l'hydrogène qui s'embrase.

## Spatialisation et mixage

- Son « cockpit » et son « extérieur » différents ; effet Doppler ; atténuation avec la distance et perte des aigus dans l'air ; masquage par la cellule.
- Bus : moteur, armes, ambiance, interface, musique ; atténuation automatique de l'ambiance pendant le combat ; nombre de voix limité.

## Musique

- Discrète en vol ; plus présente dans les menus, le briefing, le carnet. Un gramophone au mess serait une source « dans le monde ».
- Outils open source : MuseScore, LMMS, Ardour, Surge XT.
- Œuvres anciennes : la composition peut être dans le domaine public sans que l'enregistrement le soit. Ne prendre que des enregistrements à licence vérifiée ou des interprétations maison.

## Sources sonores

Échantillons CC0 ou CC-BY uniquement (par exemple Freesound, en vérifiant la licence **de chaque fichier**), inscrits dans `assets/LICENCES.md`.
