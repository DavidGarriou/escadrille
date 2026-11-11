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

## M2 — Aides au pilotage et Vickers (2026-10-29)

Version : aides au pilotage après la revue de l'aérodynamicien (plan `docs/plans/m2-aides-pilotage.md`), Vickers remodelée.

| Point essayé | Ressenti du joueur | Suite |
|---|---|---|
| Vol avec les aides | « c'est beaucoup mieux » | Détail par préréglage à recueillir au prochain essai (liste du plan, « Ton essai ») |
| Visée de la Vickers | « les deux mires de la mitrailleuse ne sont pas alignés au regard du pilote. le réticule proche est au-dessus du réticule de bout de canon » | Corrigé le 2026-10-29 : les viseurs sont construits sur la ligne de visée qui passe par l'œil, parallèle à l'âme du canon ; l'œil est levé de 5 cm, au-dessus de la culasse ; le pare-brise est rehaussé pour que la visée passe dans le verre |

## M2 — Train et sol, vol de dix minutes (2026-11-04)

Version : plan `docs/plans/m2-train-sol.md` réalisé, `just run` au sol (commit `9e1fac3`).

| Point essayé | Ressenti du joueur | Suite |
|---|---|---|
| Vol de dix minutes à la manette, critère de fin du M2 | « Considère que le vol de 10 mn s'est bien passé » | Critère tenu sur décision du joueur ; M2 clos. Pas de retour détaillé : décollage, posé, aides au sol, centrage et préréglages restent sans avis chiffré |
| Décollage et posés | « le décollage et le posés se sont bien déroulés » | Rien à corriger. Restent sans avis : les aides au sol par préréglage, le centrage du 2026-10-31 |

## Remarques pour plus tard

- **2026-11-11 — Vibrations de la SN30 Pro** : le joueur signale que la manette peut vibrer. **Correction à faire** : le M0 avait conclu « aucune vibration ressentie » (essai du 2026-10-20, ci-dessus), et le jeu ne s'appuie pas sur la vibration (`docs/vision.md`, question 5 ; skill `manette`). À reprendre avec le chantier des retours de la manette : vérifier la chaîne (gilrs, pilote `xpad`, `fftest`), refaire le test du labo manette, puis corriger la vision, le skill et ce fichier.
- **2026-11-11 — Cockpit détaillé** : le joueur demande un poste de pilotage détaillé. À reprendre avec la finition du SPAD (incrément 5 du M3, `docs/plans/m3-beaute.md`) : instruments, bois, cuir, huile, profondeur de champ (scène `cockpit_detail` du skill `rendu-visuel`).
- **2026-11-11 — Face avant du moteur** : la face du moteur, derrière l'hélice, n'est pas réaliste pour l'instant ; à détailler (capot, radiateur et calandre de l'Hispano-Suiza du SPAD, à sourcer avec l'historien). Même incrément.
