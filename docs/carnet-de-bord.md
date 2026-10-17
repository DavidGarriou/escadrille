# Carnet de bord du projet

Compte rendu chronologique des échanges entre le joueur et Claude : questions, réponses, actions, décisions.
À ne pas confondre avec le *carnet de vol*, qui est une fonctionnalité du jeu.

Règle (voir `CLAUDE.md`) : chaque question, action ou décision est consignée ici, dans la même session, avant de la clore.
Format : une entrée par échange, la plus récente en bas. Les décisions structurantes renvoient à leur ADR.

Légende : **Q** question · **R** réponse · **A** action · **D** décision · **⚠** incident ou point en attente

---

## 2026-10-16 — Session 1 (poste local, reconstituée à partir du résumé de session)

### Demande initiale
- **Q** Le joueur demande la conception d'un simulateur de combat aérien 1914-1918 inspiré de *Wings*, *Red Baron* et *IL-2 Sturmovik*. Contraintes : Rust, outils 100 % open source, Bevy pour la 3D, manette 8BitDo SN30 Pro, plusieurs niveaux de réalisme (d'Arcade à As).
- **A** Rédaction du kit de démarrage pour Claude Code :
  - `CLAUDE.md` (piliers, contraintes, stack, architecture, commandes) ;
  - `docs/vision.md`, `docs/roadmap.md` (jalons M0 à M7), `docs/adr/0001` à `0003` ;
  - 14 skills, les workflows `/nouvel-avion`, `/essai-en-vol`, `/capture` et `/adr` ;
  - 3 subagents : `aerodynamicien`, `historien`, `directeur-artistique`.
- **D** Bevy comme moteur (ADR 0001), FDM indépendant du moteur (ADR 0002), réalisme par couches (ADR 0003).

### Mission « Chasse aux dirigeables »
- **Q** Le joueur demande une mission d'interception de dirigeables.
- **A** Rédaction de `docs/gdd/mission-dirigeables.md` : hydrogène simulé (fuites, inflammation sous conditions), IA d'équipage sans triche, quatre contextes historiques proposés. Mission intégrée à la vision, à la feuille de route et aux skills concernées.
- **⚠** Seul le L 49 est vérifié : posé près de Bourbonne-les-Bains après avoir été intercepté par des Nieuport de la N 152, capturé intact, entre le 19 et le 21 octobre 1917 (date exacte incertaine). Le reste est à sourcer par le subagent `historien` dans `docs/recherche/dirigeables.md`.

### Sauvegarde et environnement
- **A** Création du dépôt `DavidGarriou/escadrille`, premier commit `3a08e80` sur `main`.
- **A** Création de l'environnement cloud « Escadrille ».

---

## 2026-10-16 — Session 2 (cloud)

### Reprise
- **A** Vérification de l'état du dépôt : branche `main-tcd0yw`, au commit `3a08e80`, arbre propre, aucun code.
- **R** Claude rappelle les limites du cloud : il peut rédiger et compiler, mais ni tester la manette ni rendre d'images (pas de GPU). Il propose trois pistes : A) démarrer le M0 en mode plan, B) trancher les questions ouvertes, C) lancer l'historien sur les dirigeables.

### Question ouverte 2 : GPU de référence
- **Q** Le joueur donne sa carte : XFX Speedster MERC319 Radeon RX 7800 XT, 16 Go GDDR6, RDNA 3 (RX-78TMERCB9).
- **D** GPU de référence : **AMD Radeon RX 7800 XT 16 Go**, sous Fedora avec le pilote Mesa RADV. Cible : 60 images/s en 1440p, qualité « haut » (16,6 ms par image).
- **D** *(proposée par Claude, modifiable)* Au plus 12 Go de mémoire vidéo en qualité « haut ».
- **A** Mise à jour de `docs/vision.md` (question 2 tranchée, section « Plateformes et performances »), de la skill `rendu-visuel` (mesures sur RADV), de `CLAUDE.md` (section « Environnement ») et de `README.md` (ajout de `mesa-vulkan-drivers`).
- **A** Commit « Set AMD Radeon RX 7800 XT as reference GPU ».

### Envoi sur GitHub
- **⚠** Deux échecs d'envoi (erreur 403) : l'application GitHub Claude n'avait pas accès au dépôt. Claude indique au joueur comment reconnecter GitHub.
- **Q** Le joueur demande de réessayer.
- **A** Envoi réussi : la branche `main-tcd0yw` est créée sur GitHub. `main` n'a pas changé ; il faudra fusionner la branche, par exemple via une pull request (proposée, pas encore demandée).

### Carnet de bord
- **Q** Le joueur veut un compte rendu de chaque question, action et décision.
- **D** Création de ce carnet (`docs/carnet-de-bord.md`) et ajout de la règle correspondante dans `CLAUDE.md`, section « Façon de travailler ».
- **A** Session 1 reconstituée à partir du résumé de session : le détail des échanges de cette session n'est plus disponible.

### Contrôle automatique du carnet
- **Q** Le joueur demande que la tenue du carnet soit vérifiée automatiquement.
- **A** Deux hooks Claude Code, déclarés dans `.claude/settings.json` :
  - `.claude/hooks/logbook-turn-start.sh` (`UserPromptSubmit`) note l'heure de début de chaque échange dans `.claude/.state/`, un dossier ignoré par git ;
  - `.claude/hooks/logbook-check.sh` (`Stop`) empêche Claude de clore l'échange si le carnet n'a pas été modifié depuis ce début, s'il reste des modifications non commitées ou si son commit n'est pas poussé.
- **D** Le contrôle bloque au plus une fois par échange (`stop_hook_active`), pour éviter une boucle sans fin. Il ne s'active qu'une fois qu'un premier message a été envoyé dans une session qui le charge.
- **A** Les cinq cas ont été testés à la main : pas de marqueur, carnet inchangé, second passage, modifications non commitées, carnet à jour.
- **A** `.gitignore` complété (`.claude/.state/`, `.claude/settings.local.json`).

### En attente
- Questions ouvertes 1, 3, 4, 6, 7 et 8 de `docs/vision.md` ; la 5 (manette) sera vérifiée au M0 avec le labo manette.
- Démarrage du M0 en mode plan : version de Bevy et ADR.
- Sources de la mission dirigeables (subagent `historien`).
- Pull request de `main-tcd0yw` vers `main`, si le joueur le souhaite.

---

## 2026-10-17 — Session 3 (cloud)

### Commande /adr sans titre
- **Q** Le joueur lance `/adr` sans préciser de sujet.
- **A** Claude propose trois sujets (version de Bevy, GPU de référence, carnet de bord). Le joueur refuse le questionnaire.
- **A** Claude commence l'ADR sur la version de Bevy. Le joueur l'interrompt avant toute recherche : aucun ADR n'a été créé.

### Questions ouvertes et suggestions
- **Q** Le joueur demande de reprendre les questions ouvertes, avec quelques suggestions.
- **R** Claude présente les questions 1, 3, 4, 6, 7 et 8 de `docs/vision.md` avec une recommandation pour chacune, plus des suggestions de suite :
  - fusionner la branche `main-tcd0yw` dans `main` ;
  - ajouter un hook de démarrage pour les sessions cloud ;
  - lancer l'ADR sur la version de Bevy ;
  - faire sourcer le Raid silencieux par le subagent `historien`.
- **D** Recommandation pour l'hébergement (question 6) : rester sur GitHub, car les sessions cloud de Claude Code en dépendent, avec un miroir Codeberg facultatif. Cela revient sur la proposition initiale (Codeberg ou Forgejo).
- **⚠** En attente des réponses du joueur.

### Réponses du joueur aux questions ouvertes
- **D** Question 1 : le jeu s'appelle **Ricin**. « Escadrille » reste le nom de code technique (dépôt, crates, identifiants).
- **A** Recherche web rapide : aucun jeu nommé « Ricin » trouvé. Ce n'est pas une recherche de marque déposée.
- **⚠** En anglais, *ricin* désigne la toxine (la plante se dit *castor*). Claude aurait dû le signaler en le proposant. Il faudra un sous-titre ou un nom localisé pour la version anglaise.
- **D** Question 3 : France, SPA 3, Flandres, été 1917, SPAD VII contre Albatros D.V.
- **D** Question 4 : code sous **licence MIT** (seule, pas en double licence avec Apache-2.0). Ajout du fichier `LICENSE`, au nom de « The Ricin contributors ». La licence des assets n'a pas été précisée : CC-BY-SA 4.0 reste une proposition à confirmer.
- **D** Question 6 : hébergement sur GitHub.
- **D** Question 7 : ton sobre et réaliste.
- **D** Question 8 : le Raid silencieux d'octobre 1917.
- **A** Mise à jour de `docs/vision.md` (questions tranchées, nom du jeu) et de `CLAUDE.md` (nom du jeu, licence).
- **⚠** Restent en attente : la licence des assets, la question 5 (manette, au M0) et les suggestions (pull request vers `main`, hook de démarrage cloud, ADR sur la version de Bevy, recherches de l'historien).

### Nouveau nom : le soldat solitaire du ciel
- **Q** Le joueur veut un nom qui évoque le soldat solitaire dans le ciel, aventurier, avec du panache.
- **A** Recherches web rapides sur les jeux homonymes (pas de recherche de marques déposées) :
  - « Panache » : nom du studio Panache Digital Games (Montréal) ;
  - « Cyrano » : un jeu Steam de 2025 porte déjà ce nom ;
  - « Cavalier seul », « Haut Vol » et « Panache blanc » : aucun jeu trouvé.
- **R** Claude propose : *Cavalier seul* (recommandé), *Panache blanc*, *Haut Vol*, *Seul là-haut*, *Chevalier errant*. Il déconseille *Panache* et *Cyrano*, déjà pris.
- **⚠** Le nom *Ricin* reste en place dans les documents jusqu'au choix du joueur.

### Choix du nom : Panache blanc
- **D** Le joueur retient **Panache blanc**, qui remplace *Ricin*.
- **A** Nom mis à jour dans `CLAUDE.md`, `docs/vision.md` et `LICENSE` (« The Panache blanc contributors »). « Escadrille » reste le nom de code technique.
- **⚠** À faire avant toute publication : une recherche de marque à l'INPI et à l'EUIPO. Le titre anglais reste à décider : garder le titre français ou le traduire.
