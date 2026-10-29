# Retours du joueur

Retours d'essai manette en main, par jalon. Les tests automatiques ne disent pas si c'est agréable : ce fichier, si.

## M0 — Fondations (2026-10-20, Fedora, RX 7800 XT, SN30 Pro USB)

- Le jeu compile et se lance ; le labo manette reconnaît la manette (« Xbox 360 Controller »).
- Gâchettes L2/R2 : tout-ou-rien. Décision : on conclut sans autre relevé.
- Vibrations : rien de ressenti sur aucun des boutons de test.
- Bouton B (en bas) = `South` : confirmé.
- Capture `m0-sol` en 1920×1080 réussie (image et JSON dans `docs/essais/captures/`).

## M2 — Vol d'essai, premier incrément (2026-10-29, Fedora, SN30 Pro USB)

Version : `just run`, plan `docs/plans/m2-vol-essai.md`. Le joueur : « Ce n'est pas facile de faire tous ces tests, voici mon ressenti » :

| Point essayé | Ressenti du joueur | Suite |
|---|---|---|
| Tenue du palier, spirale | « pas facile de tenir le palier, il y a une dérive légère sur la gauche » | Attendu : l'équilibre demande un peu de palonnier et d'aileron à droite (couple et souffle de l'hélice), que le neutre ne compense pas. Le 2026-10-29, aides au pilotage : seul l'instructeur virtuel (Arcade) tient les ailes à plat. La coordination (Pilote, Vétéran) annule le dérapage mais pas la spirale lente |
| Virages à 30° et 60° | « je décroche très facilement sur un virage appuyé car je perds beaucoup de vitesse. Ajouter un mode où l'on aide le pilotage » | Aides au pilotage en place le 2026-10-29 (`docs/plans/m2-aides-pilotage.md`) : garde anti-décrochage en Pilote, instructeur en Arcade. Constat : la profondeur du modèle est très puissante (5 % de course de la croisière à 10° d'incidence), soumis à l'aérodynamicien |
| Effets des gaz | pas de remarque | — |
| Décrochage gaz réduits | 85 km/h | Seul indice historique : environ 76 km/h (47 mph), moteur coupé, sur un S.VII d'après-guerre à moteur de 180 ch (`spad7.md` § 5.3, confiance D). À soumettre à l'aérodynamicien, sans changer de coefficient sans source |
| Confort des vues | « bien, sauf la mitrailleuse qui est un gros bloc, il faut le rendre beaucoup plus réaliste » | Vickers remodelée le 2026-10-29 (manchon à fentes, culasse, viseurs circulaires) ; pare-brise éclairci |
| Réglages de la manette | « un peu trop sensible » | Courbe du manche adoucie |
| Fluidité | « bien » | — |
