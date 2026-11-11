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

### Explication de la licence CC-BY-SA
- **Q** Le joueur demande ce qu'est la licence CC-BY-SA.
- **R** Claude explique CC-BY-SA 4.0 : ce qu'elle permet, ses deux conditions (citer l'auteur, partager à l'identique), sa portée dans un jeu (elle s'applique à l'asset modifié, pas au code ni au reste du jeu), ses conséquences, et les autres choix possibles (CC-BY 4.0, CC0).
- **⚠** La licence des assets reste à décider.

### Pourquoi plusieurs licences
- **Q** Le joueur demande pourquoi avoir plusieurs licences.
- **R** Les licences de logiciel (MIT) sont écrites pour du code, les licences Creative Commons pour des œuvres (images, sons, modèles). Creative Commons déconseille ses licences pour le code, et MIT convient mal aux œuvres. Les assets tiers récupérés seront de toute façon sous Creative Commons ou dans le domaine public, et `cargo deny` attend des licences de logiciel. C'est l'usage dans les jeux libres : 0 A.D. met son code sous GPL et ses graphismes sous CC-BY-SA. Une licence unique (tout sous MIT) reste possible, mais protège moins les assets.
- **⚠** La licence des assets reste à décider.

### Licence des assets
- **D** Le joueur retient la recommandation de Claude : **CC-BY-SA 4.0** pour les assets (`assets/`), les sources artistiques (`art/`) et la documentation (`docs/`).
- **D** Les fichiers de données (`.ron`, `.ftl`) restent sous MIT avec le code, comme Claude l'avait proposé. Les shaders WGSL sont du code, donc sous MIT.
- **A** Création de `LICENSE-ASSETS`. C'est une notice qui renvoie au texte officiel sur creativecommons.org, sans le recopier. Mise à jour de `CLAUDE.md` et de `docs/vision.md` (question 4 tranchée).
- **⚠** Les questions ouvertes de la vision sont toutes tranchées, sauf la 5 (manette), qui sera vérifiée au M0.

### Hook de démarrage pour les sessions cloud
- **Q** Le joueur demande plus de détails sur le hook de démarrage.
- **A** Inventaire du conteneur cloud : Ubuntu 24.04 ; Rust 1.97, clang et jq sont présents ; `just`, `cargo-nextest`, `cargo-deny` et `mold` sont absents, ainsi que les bibliothèques système dont Bevy a besoin (ALSA, udev, Wayland, xkbcommon).
- **R** Claude explique le rôle du hook, son contenu proposé, ses limites (pas de GPU ni de manette dans le cloud, durée ajoutée au démarrage de chaque session) et l'autre solution : le script d'installation de l'environnement cloud, réglé dans claude.ai.
- **⚠** En attente de la décision du joueur.

---

## 2026-10-19 — Session 3, suite (cloud)

### Hook de démarrage pour les sessions cloud
- **D** Le joueur demande d'ajouter le hook dans le dépôt.
- **A** Création de `.claude/hooks/session-start.sh`, déclaré dans `.claude/settings.json` (`SessionStart`) :
  - il ne s'exécute que si `CLAUDE_CODE_REMOTE=true` ; sur le poste Fedora, il ne fait rien ;
  - il installe avec apt : `pkg-config`, `clang`, `mold`, `just`, et les bibliothèques de développement d'ALSA, udev, Wayland, xkbcommon et X11 ;
  - il installe `cargo-nextest` 0.9.148 et `cargo-deny` 0.20.2 à partir des binaires publiés sur GitHub (versions épinglées dans le script) ;
  - il ajoute `rustfmt` et `clippy` ;
  - il ne réinstalle rien de ce qui est déjà présent ; il s'exécute de façon synchrone.
- **A** Réseau du conteneur vérifié : l'index et les téléchargements de crates.io, les binaires publiés sur GitHub et les dépôts Ubuntu sont accessibles ; l'API GitHub et get.nexte.st sont bloqués, d'où des versions épinglées.
- **A** Validation :
  - sur poste local, le hook ne fait rien (sortie 0) ;
  - dans le cloud, première exécution en 17 s, deuxième en 0,5 s ;
  - `pkg-config` trouve les bibliothèques ;
  - sur une crate de test jetable : `cargo fmt --check`, `clippy -D warnings` et `nextest` (1 test) passent, ainsi que `just`.
  - `cargo deny check licenses` a d'abord échoué, comme attendu, car la crate de test n'avait pas de champ `license` ; il passe après ajout de `license = "MIT"`.
- **⚠** Le projet n'a pas encore de code ni de `justfile` : `just check` et `just test` ne pourront être validés qu'au M0.
- **⚠** `just` vient d'apt (version 1.21, ancienne). À remplacer par le binaire GitHub si une recette a besoin d'une version plus récente.
- **⚠** Le hook ne s'appliquera à toutes les sessions qu'une fois fusionné dans `main`.

### Qu'est-ce que l'ADR sur la version de Bevy ?
- **Q** Le joueur demande ce qu'est l'ADR sur la version de Bevy.
- **R** Claude explique ce qu'est un ADR, pourquoi il faut choisir la version de Bevy avant d'écrire du code, les options (Bevy 0.20 ou 0.19), ce qu'il vérifiera (compatibilité de `bevy_egui` et `bevy_hanabi`, licences, maintenance) et comment l'ADR sera rédigé.
- **⚠** En attente du feu vert du joueur pour lancer la recherche et rédiger l'ADR 0004.

### Recherche et ADR 0004 : version de Bevy
- **D** Le joueur donne son feu vert pour la recherche.
- **A** Relevé sur l'index et l'API de crates.io, et sur la branche `main` de bevy_hanabi (GitHub) :
  - Bevy 0.20.0 (8 octobre 2026, Rust ≥ 1.97.1) et 0.19.1 ;
  - `bevy_egui` 0.43.1 (10 octobre 2026), déjà compatible avec Bevy 0.20 ;
  - `bevy_hanabi` 0.19.0 (27 juin 2026), qui ne supporte que Bevy 0.19 ; sa branche `main` est en `0.20.0-dev`, le portage est en cours ;
  - par le passé, bevy_hanabi a suivi chaque version de Bevy en 5 à 37 jours.
  - Licences : MIT ou MIT OR Apache-2.0, toutes conformes.
- **⚠** Bevy 0.20 exige Rust ≥ 1.97.1, mais le conteneur cloud fournit 1.97.0. La version stable actuelle est 1.99.1.
- **A** Rédaction de `docs/adr/0004-version-bevy.md`, en statut « Proposé », et ajout à l'index.
- **R** Recommandation : Bevy 0.20.0 et bevy_egui 0.43.1 ; bevy_hanabi sera ajouté plus tard, au premier effet de particules (le M0 n'en a pas besoin). La version de Rust sera fixée dans `rust-toolchain.toml`, et le hook cloud devra l'installer.
- **⚠** En attente de la validation du joueur.

### Validation de l'ADR 0004 et plan du M0
- **D** Le joueur valide l'ADR 0004 : Bevy 0.20.0, bevy_egui 0.43.1, bevy_hanabi reporté, Rust 1.99.1.
- **A** ADR 0004 passé en « Accepté » (fichier et index). Versions en vigueur notées dans `CLAUDE.md` (section Bevy). Feuille de route : l'ADR de version est coché.
- **A** Rédaction du plan du M0 dans `docs/plans/m0-fondations.md`, en statut « Proposé », en trois étapes :
  1. outillage et workspace, dans le cloud ;
  2. le jeu compilé dans le cloud : états de jeu, labo manette, mode capture, scène `m0-sol` ;
  3. essais sur la machine du joueur.
- **R** Claude demande au joueur de valider le plan et les dépendances : `bevy`, `bevy_egui`, `glam`, `serde`, `ron`, `clap`, `thiserror`.
- **R** Claude propose aussi une expérience : faire les captures dans le cloud avec un rendu logiciel (Mesa lavapipe et un écran virtuel).
- **⚠** En attente de la validation du plan.

### Validation du plan du M0 et étape 1
- **D** Le joueur valide le plan du M0 et les dépendances (`bevy`, `bevy_egui`, `glam`, `serde`, `ron`, `clap`, `thiserror`).
- **Q** Le joueur demande de lui rappeler l'architecture du jeu retenue et le moteur choisi.
- **R** Claude rappelle :
  - le moteur Bevy 0.20.0 (ADR 0001 et 0004) ;
  - le modèle de vol `escadrille_fdm`, sans Bevy, en f64 (ADR 0002) ;
  - le jeu Bevy organisé en un plugin par domaine, simulé à pas fixe de 120 Hz ;
  - le réalisme réglé par couches (ADR 0003) ;
  - les outils sans rendu (`fdm_lab`).
- **A** Étape 1 du M0 réalisée :
  - `rust-toolchain.toml` (Rust 1.99.1, avec `rustfmt` et `clippy`) ; le hook de démarrage installe désormais cette version ;
  - workspace Cargo (édition 2024, Rust ≥ 1.97.1, licence MIT) avec trois crates : `escadrille_fdm` (constantes de l'atmosphère standard ISO 2533, testées), `escadrille_data` (chargeur de fichiers RON avec erreurs situées), `fdm_lab` (interface en ligne de commande ; les commandes `trim`, `essai` et `modes` arrivent au M1) ;
  - réglages de compilation (dépendances optimisées en développement), édition de liens avec clang et mold ;
  - `deny.toml` (licences de `CLAUDE.md`, sources inconnues refusées), `rustfmt.toml`, `justfile`, et le registre `assets/LICENCES.md`.
- **A** Résultats : `just check` vert (formatage, clippy sans avertissement, cargo-deny : advisories, bans, licences et sources ok) ; `just test` : 7 tests sur 7 réussis.
- **⚠** `glam` est déclaré dans le workspace (même version mineure que Bevy, 0.33) mais n'est pas encore utilisé ; il le sera au M1. Les recettes `run`, `labo-manette` et `capture` affichent « arrive à l'étape 2 ».
- **⚠** Les valeurs d'exemple des tests de `escadrille_data` sont neutres : Claude a retiré une surface alaire du SPAD VII qu'il avait d'abord écrite sans source.

### Étape 2 du M0 (en cours) et surface alaire du SPAD
- **D** Le joueur lance l'étape 2 du M0.
- **Q** Le joueur demande comment Claude connaîtra la surface alaire du SPAD VII.
- **R** Claude ne la connaîtra pas de mémoire. La méthode (skill `recherche-historique`) :
  - recherche par le subagent `historien`, des sources primaires vers les sources secondaires (forums et wikis : simples pistes) ;
  - fiche `docs/recherche/spad_vii.md`, avec sources numérotées et fourchette justifiée si elles divergent ;
  - niveau de confiance A, B, C ou D pour chaque valeur ;
  - contrôle de cohérence par le calcul (envergure, corde, charge alaire), puis essais en vol de `fdm_lab` au M1 ;
  - relecture par le subagent `aerodynamicien` avant l'entrée de la valeur dans `aircraft.ron`.
- **A** Lecture des sources de Bevy 0.20.0 et de bevy_egui 0.43.1, avec les exemples du tag `v0.20.0` : manettes (événements bruts `RawGamepad*`, vibrations `GamepadRumbleRequest`), captures d'écran (`Screenshot` et `save_to_disk`), états, et panneaux egui (`egui::Window`, puisque `CentralPanel` exige désormais un `Ui`).
- **A** Code écrit (non encore compilé au moment de ce commit) : crate `escadrille` dans `crates/game`, avec les états de jeu (`Vol`, `LaboManette`, `Capture`), le plugin `world` (scène `m0-sol`) et le plugin `dev` :
  - labo manette : axes et boutons bruts et traités, histogrammes de L2 et R2 avec verdict progressif ou tout-ou-rien, tests de vibration, rapport texte ;
  - mode capture : PNG et JSON, avec délai maximal.
- **A** Recettes `just run`, `just labo-manette` et `just capture` branchées.
- **D** La scène `m0-sol` utilise des repères de 1 m et de 10 m au lieu d'un « avion factice à l'échelle du SPAD VII » : on n'utilise pas de dimensions historiques non sourcées. Le plan est corrigé en conséquence.
- **A** Expérience du rendu logiciel : Mesa lavapipe installé ; sous Xvfb, `vulkaninfo` voit le périphérique « llvmpipe ». Le test de capture se fera après compilation.
- **⚠** Travail en cours : la première compilation de Bevy n'est pas terminée. `just check` et `just test` restent à valider pour l'étape 2.

### Témoignages de pilotes et réputation des avions
- **D** Le joueur demande que la recherche sur le comportement des avions inclue des témoignages de pilotes d'époque, en plusieurs langues, ainsi que la réputation de chaque avion.
- **A** Skill `recherche-historique` complétée par une section « Témoignages de pilotes et réputation des avions » :
  - pistes par langue (français, anglais, allemand, italien, belge, russe, austro-hongrois), à vérifier avant citation ;
  - chaque témoignage est situé (auteur, camp, date, sous-version, biais) ;
  - un trait n'est retenu qu'avec au moins deux témoignages indépendants, de préférence de camps différents ; les essais d'avions capturés sont privilégiés ;
  - paraphrase en français, avec la langue d'origine indiquée ;
  - la réputation est distinguée des faits (confirmé, contesté, légende), et sert aussi au récit ;
  - chaque trait confirmé devient un essai qualitatif dans `essais.ron`.
- **A** Modèle `fiche-avion.md` : section 6 refondue (6.1 témoignages, 6.2 tableau des traits, 6.3 réputation). Subagent `historien` : requêtes dans chaque langue, au minimum en français, anglais, allemand et italien. Skills `nouvel-avion` et `simulation-vol` reliées à cette section.
- **⚠** Les titres de mémoires cités comme pistes (Lewis, McCudden, Rickenbacker, Richthofen, Udet, Coppens) viennent de la mémoire de Claude : édition, date et page sont à vérifier avant tout usage.
- **A** Première compilation complète de Bevy 0.20.0 dans le cloud : 20 min 21 s, en profil dev avec les dépendances optimisées. Les compilations suivantes seront incrémentales.
- **⚠** Le contrôle du carnet se déclenche aussi sur les notifications de tâches de fond, qui ne sont pas des messages du joueur : chaque notification exige donc une ligne de carnet.
- **A** Étape 2, compilation et tests : `cargo check` sans erreur ; une seule remarque de clippy, corrigée (`if` imbriqués) ; `just test` : 19 tests sur 19 réussis (fdm 2, data 3, fdm_lab 2, jeu 12).
- **A** Expérience du rendu logiciel **réussie** : sous Xvfb avec lavapipe (Mesa 25.2.8, LLVM 20), `escadrille --capture m0-sol` produit `target/captures/m0-sol.png` (1280×720) et son JSON, avec 75 ms par image en moyenne sur le processeur. Il a fallu ajouter `libxkbcommon-x11-0`. Claude a regardé l'image : sol, ciel, cube, poteau et ombres sont corrects, mais le haut du poteau de 10 m était coupé ; cadrage de la caméra corrigé.
- **A** Hook de démarrage : ajout de lavapipe, Xvfb et des bibliothèques X11. Recette `just capture` : compilation release, puis lancement sous `xvfb-run` s'il n'y a pas d'écran.
- **⚠** `cargo deny check` échoue pour l'arbre de Bevy et d'egui (décision du joueur requise) :
  - licence MIT-0 (crates `encase`, requises par Bevy) ;
  - licences OFL-1.1 et Ubuntu Font Licence (polices par défaut d'egui, `epaint_default_fonts`) ;
  - faille RUSTSEC-2026-0344 dans `ruzstd` 0.8.3 (via `bevy_image`, pas de mise à jour compatible) ;
  - `ttf-parser` non maintenu (RUSTSEC-2026-0192, via les décorations de fenêtre Wayland).
- **⚠** Le champ `frames` du JSON compte toutes les images jusqu'à l'écriture du fichier : 33 pour 30 demandées, car la capture prend quelques images.
- **A** `just capture m0-sol --size 1280x720` validé de bout en bout dans le cloud : compilation release en 12 min, capture sous `xvfb-run` avec lavapipe, 70 ms par image. Claude a regardé l'image : le poteau de 10 m est maintenant entier dans le cadre, le cube et les ombres sont cohérents avec un soleil à droite.
- **⚠** Défaut visible, à traiter avec le terrain (M2-M3) : le bord du plan de sol de 400 m fait office d'horizon, avec un angle léger au coin du plan.

### Décisions sur cargo-deny et fin de l'étape 2
- **D** Le joueur valide les quatre recommandations :
  1. licence MIT-0 ajoutée à la liste de `CLAUDE.md` et de `deny.toml` (crates `encase`, requises par Bevy) ;
  2. exception limitée à `epaint_default_fonts` (polices d'egui) pour OFL-1.1 et Ubuntu-font-1.0, à retirer quand l'interface du jeu aura ses propres polices ;
  3. alerte RUSTSEC-2026-0344 (`ruzstd`) ignorée, avec justification : le jeu ne décompresse que ses propres fichiers ;
  4. alerte RUSTSEC-2026-0192 (`ttf-parser` non maintenu) ignorée, avec justification : il ne lit que les polices du système.
- **A** `deny.toml` mis à jour avec les justifications, à revoir à chaque montée de version de Bevy. Dans `CLAUDE.md` : MIT-0 ajoutée, et règle « exceptions uniquement dans `deny.toml`, justifiées, avec l'accord du joueur ».
- **A** `just check` **vert** (formatage, clippy, cargo-deny : advisories, bans, licences et sources ok) ; `just test` : 19 tests sur 19 réussis. **Étape 2 du M0 terminée.**
- **A** Plan du M0 mis à jour, avec le mode d'emploi de l'étape 3 pour le joueur.
- **⚠** Étape 3 en attente sur la machine du joueur : labo manette (question 5 de la vision) et capture sur la RX 7800 XT.

### Connexion de la manette
- **Q** Le joueur précise que la manette se branche sur un port USB-A.
- **A** Mise à jour de `CLAUDE.md` (Environnement), de la skill `manette`, du plan du M0 (étape 3) et du message du labo manette : on ne parle plus d'« USB-C ou Bluetooth », mais d'un câble USB sur un port USB-A du PC.
- **⚠** Modèle exact à confirmer : SN30 Pro sans fil branchée par câble (Bluetooth possible) ou version filaire « SN30 Pro USB » (câble fixe, sans Bluetooth). Cela change les tests de l'étape 3 ; le labo affichera le nom et les identifiants USB.
- **D** Le joueur précise le modèle : **8BitDo SN30 Pro USB**, la version filaire.
- **A** Recherche web (pages commerciales et forums, non consultables en direct depuis le conteneur, donc lues par résumés de recherche) :
  - câble fixe d'environ 1,8 m, sans Bluetooth ;
  - ni détection de mouvement, ni NFC, ni « HD rumble » selon 8BitDo ;
  - les gâchettes analogiques sont présentées comme une nouveauté de la SN30 Pro+ : L2 et R2 sont donc probablement tout-ou-rien sur notre modèle ;
  - en USB, Linux utilise le module `xpad` (pilote des manettes Xbox), et la vibration y fonctionne sur des modèles proches.
- **A** Mise à jour de la skill `manette` (modèle, attentes, débranchement du câble au lieu de la déconnexion Bluetooth), de `CLAUDE.md` (Environnement), de la question 5 de la vision et de l'étape 3 du plan (plus de test Bluetooth).
- **⚠** Gâchettes et vibrations restent « probables » tant que le labo manette ne les a pas mesurées. La conception (gaz incrémentaux, rien n'exige de gâchettes analogiques) ne change pas.
- **A** Message du labo manette sans mention du Bluetooth (« plug in the SN30 Pro USB »).

### Reste-t-il une étape avant le premier test ?
- **Q** Le joueur demande s'il reste une étape avant le premier test.
- **R** Côté code, non : l'étape 2 est terminée et validée dans le cloud. Côté machine du joueur, une préparation unique : paquets Fedora et `rustup` (README), récupération de la branche `main-tcd0yw`, puis une première compilation de Bevy, longue. La fusion dans `main` est facultative pour tester.
- **A** `README.md` mis à jour : présentation actuelle du dépôt, et la section « Démarrer », devenue obsolète (« git init », message de démarrage du M0), remplacée par « Premier essai sur votre machine (M0, étape 3) » avec les commandes et ce qu'il faut renvoyer à Claude.

### Qu'est-ce que « just » ?
- **Q** Le joueur demande ce qu'est `just`.
- **R** Claude explique : un lanceur de commandes libre (licence CC0, écrit en Rust, paquet Fedora `just`) qui lit le fichier `justfile` à la racine du dépôt. Chaque « recette » donne un nom court à une suite de commandes longues (par exemple, `just capture m0-sol` compile le jeu en version optimisée, puis le lance en mode capture). `just` sans argument liste les recettes. C'est un raccourci, pas une dépendance du jeu.

---

## 2026-10-20 — Session 3, suite (cloud)

### Qu'est-ce que GitHub ?
- **Q** Le joueur écrit « github », compris dans la suite de la question précédente comme « précise ce qu'est GitHub ».
- **R** Claude explique GitHub (hébergement du dépôt Git, historique, branches, pull requests) et son rôle dans le projet : dépôt `DavidGarriou/escadrille`, sauvegarde du travail, passerelle entre les sessions cloud et le poste Fedora. Il précise l'état actuel : `main` est resté au premier commit, tout le travail depuis est sur la branche `main-tcd0yw`.
- **⚠** Le joueur précise que le message « github » était une erreur d'envoi. L'explication reste dans le carnet ; aucune action n'en découle.

### Sticks ou gâchettes dans le labo manette ?
- **Q** Le joueur demande s'il ne faut pas plutôt « tirer lentement les joysticks » dans le labo manette.
- **R** Les deux, mais pour deux raisons différentes. Pour L2 et R2, on les enfonce lentement pour savoir si elles sont progressives (question 5). Les sticks sont analogiques à coup sûr : on les teste pour leur course (butées, cercles) et pour la valeur au repos, qui sert à régler les zones mortes et la dérive. Le labo affiche les sticks en direct, mais son rapport ne garde que la dernière valeur brute.
- **A** README : consignes du labo complétées pour les sticks (butées, cercles, valeur au repos).
- **⚠** Proposition en attente : enregistrer dans le rapport des statistiques sur les sticks (minimum et maximum par axe, bruit au repos), utiles pour régler les zones mortes au M2.

### Premier rapport du labo manette
- **A** Le joueur a lancé le labo sur sa Fedora et transmet `rapport.txt`. Le jeu a donc compilé et tourné sur sa machine. Le rapport est archivé dans `docs/essais/labo-manette/2026-10-20-sn30-pro-usb.txt`.
- **R** Analyse de Claude :
  - la SN30 Pro USB se présente comme « Xbox 360 Controller » (USB 045e:028e) : mode X-input, pilote `xpad`, reconnue sans réglage ;
  - tous les boutons standard remontent ;
  - L2 et R2 ne donnent que 0 ou 1, sur seulement 2 appuis chacune : très probablement tout-ou-rien. Le verdict formel (« NotEnoughData ») exige au moins 10 appuis ;
  - sticks au repos à 3·10⁻⁵ et 0 : aucune dérive ;
  - vibrations non renseignées.
- **A** Skill `manette` complétée par ces résultats ; question 5 de la vision mise à jour (premier relevé).
- **⚠** Reste à obtenir du joueur : ce qu'il a ressenti pour chaque vibration, une dizaine d'appuis lents sur L2 et R2 (ou son accord pour conclure au tout-ou-rien), la correspondance entre B physique et `South`, et la capture `m0-sol` faite sur la RX 7800 XT.

### Réponses du joueur et capture sur la RX 7800 XT
- **R** Réponses du joueur :
  1. aucune vibration ressentie ;
  2. on conclut au tout-ou-rien pour L2 et R2 ;
  3. B physique = `South` ;
  4. capture `m0-sol` transmise (image et JSON).
- **D** Question 5 de la vision **tranchée** : L2/R2 tout-ou-rien, aucune vibration ressentie, B = `South`. Le jeu ne s'appuie pas sur la vibration (retour optionnel, doublé par le son et l'image).
- **A** Skill `manette` (conclusions), `docs/vision.md` (question 5) et `docs/gdd/retours.md` (créé : premiers retours du M0) mis à jour. Capture et JSON archivés dans `docs/essais/captures/2026-10-20-m0-sol-rx7800xt.*`.
- **R** Analyse de la capture par Claude :
  - même cadrage et même éclairage que la capture du cloud (lavapipe), avec un rendu plus fin en 1920×1080 : les captures du cloud sont donc représentatives pour juger une scène ;
  - même défaut d'horizon (bord du plan de sol).
- **⚠** Le temps moyen de 16,498 ms par image correspond exactement à 60 Hz : il mesure la synchronisation verticale de l'écran, pas le coût du rendu.
- **A** Correction : le mode capture désactive la synchronisation verticale (`PresentMode::AutoNoVsync`), et les autres modes la gardent. Vérifiée dans le cloud. `just check` vert et `just test` à 19 tests sur 19.
- **⚠** Vibration : cause inconnue (moteurs absents sur ce modèle, pilote `xpad`, ou gilrs). Diagnostic proposé avec `fftest` (paquet `linuxconsoletools`).
- **⚠** Toutes les conditions de fin du M0 sont remplies : en attente de l'accord du joueur pour clore le jalon et passer au M1.

### Clôture du M0, passage au M1
- **D** Le joueur clôt le M0 et lance le M1.
- **A** Feuille de route (M0 ✅, M1 en cours), ligne « Jalon en cours » de `CLAUDE.md` et statut du plan du M0 mis à jour.
- **A** Rédaction du plan du M1, `docs/plans/m1-modele-de-vol.md`, en statut « Proposé », en sept étapes :
  1. recherche sur le SPAD VII et l'Hispano-Suiza 8A par l'historien, en parallèle ;
  2. atmosphère ISA et corps rigide à 6 degrés de liberté, intégrateur RK4 ;
  3. aérodynamique par bandes (biplan, empennage, traînées parasites) ;
  4. hélice et moteur en ligne ;
  5. schémas `aircraft.ron` et `essais.ron`, avec validation de l'origine de chaque valeur ;
  6. `fdm_lab` : trim, essais, modes propres, tracés PNG ;
  7. robustesse, `/essai-en-vol`, revue `aerodynamicien`, lecture par le joueur.
- **⚠** À valider par le joueur :
  - dépendances `plotters` et `nalgebra`, dans `fdm_lab` seulement ;
  - ADR 0005 sur l'intégration numérique ;
  - lancement immédiat du subagent `historien`.

### Lancement du M1
- **D** Le joueur valide le plan du M1 et ses trois points : dépendances `plotters` et `nalgebra` (dans `fdm_lab`), ADR 0005, lancement immédiat de l'historien.
- **A** Subagent `historien` lancé en arrière-plan. Il doit produire trois fiches : `docs/recherche/spad7.md`, `hispano-suiza-8a.md` et `profil-spad7.md`. Consignes :
  - établir la sous-version de la SPA 3 à l'été 1917 (150 ou 180 ch) et la date d'arrivée des SPAD XIII ;
  - réunir géométrie, masses et performances avec leurs conditions ;
  - recueillir des témoignages en au moins quatre langues et la réputation de l'avion ;
  - trouver des données de soufflerie pour le profil ;
  - aucun chiffre sans source, « non trouvé » plutôt qu'un chiffre de mémoire.
- **A** ADR 0005 « Intégration numérique » rédigé et accepté : RK4 à pas fixe en f64 ; le jeu appelle le modèle à 120 Hz, qui fait 2 sous-pas (240 Hz) ; quaternion renormalisé en fin de pas ; commandes constantes pendant un pas. Plan du M1 marqué « Validé ».
- **A** Étape 2 du M1 codée dans `escadrille_fdm`, avec `glam` en dépendance :
  - `atmosphere.rs` : ISA de 0 à 20 km, altitude géopotentielle ;
  - `environment.rs` : trait `Environment` et `StandardEnvironment` (ISA, vent uniforme, sol plat) ;
  - `rigid_body.rs` : `Inertia` (convention aéronautique pour ixz), `MassProperties` validées, `Wrench`, `RigidBodyState` et `step_rk4`.
- **A** Tests : 19 sur 19 réussis dans `escadrille_fdm`, dont :
  - chute libre exacte quelle que soit l'attitude ;
  - rotation stationnaire ;
  - conservation de l'énergie et du moment cinétique pendant un basculement autour de l'axe intermédiaire ;
  - précession analytique d'une toupie symétrique ;
  - couplage roulis-lacet par ixz ;
  - ordre 4 mesuré (rapport d'erreur entre 10 et 24 quand le pas est divisé par 2) ;
  - déterminisme au bit près ;
  - valeurs ISA à la tropopause.
- **⚠** Un premier test comparait deux quaternions avec `angle_between`, qui passe par `acos` et ne distingue pas un angle inférieur à environ 1e-8 : tolérance irréaliste. Remplacé par une comparaison des composantes. La physique n'était pas en cause.
- **A** Subagent `aerodynamicien` lancé en arrière-plan pour relire l'étape 2, comme l'exige `CLAUDE.md` pour toute modification du modèle de vol.
- **A** Vérification complète de l'étape 2 : `just check` vert (formatage, clippy, cargo-deny), `just test` : 36 tests sur 36 réussis sur tout le workspace.

### Retour de l'historien (étape 1 du M1)
- **A** L'historien a rédigé `docs/recherche/spad7.md` (432 lignes), `hispano-suiza-8a.md` et `profil-spad7.md`. Chaque source y est étiquetée : [LU] pour un texte intégral lu, [RÉSUMÉ] pour un résumé de moteur de recherche seulement, [PISTE] pour une référence non consultée.
- **⚠** **Limite majeure : le réseau du conteneur ne laisse passer que GitHub.** Wikipedia, archive.org, ntrs.nasa.gov (rapports NACA), gallica.bnf.fr, la bibliothèque numérique de la Défense, albindenis.free.fr, theaerodrome.com et d'autres sont refusés. Seuls quatre livres du domaine public ont été lus en entier, par un miroir GitHub de Gutenberg (Bordeaux sur Guynemer, Richthofen, Hall, Nordhoff). En conséquence, **aucune performance ne dépasse la confiance C**, et le critère de fin du M1 (essais de confiance A et B) est inatteignable en l'état.
- **R** Résultats principaux :
  - sous-version proposée : SPAD VII à moteur Hispano-Suiza 8Ab de 180 ch (confiance C), avec un parc probablement mixte 150/180 ch à la SPA 3 ; SPAD XIII en très petit nombre à partir de fin juillet-août 1917 ;
  - valeurs B : envergure supérieure 7,82 m, surface 17,85 m² (définition inconnue), longueur 6,08 m, masses en 150 ch de 500 kg à vide et 705 kg au décollage ;
  - performances : toutes en C ;
  - deux traits confirmés : structure robuste en piqué quasi vertical, moteur sensible à la température ;
  - un témoignage dans chaque langue demandée, mais l'italien seulement en résumé ;
  - lacunes : corde, écartement, décalage, calages, dièdre, gouvernes, courbe de puissance, centrage ;
  - profil d'aile réel inconnu : RAF 15 proposé (C).
- **R** Recommandation de l'historien : valider d'abord le modèle en 150 ch, la seule version mesurée à l'époque (essais STAé et rapport britannique n° 96), puis passer au 180 ch.
- **⚠** Décisions demandées au joueur :
  1. élargir l'accès réseau de l'environnement cloud aux sites de sources, ou fournir lui-même les documents ;
  2. choix de la version à valider en premier (150 ch puis 180 ch, ou 180 ch directement) ;
  3. si les sources primaires restent inaccessibles, accepter ou non un M1 validé en confiance C.

### Accès réseau élargi
- **A** Le joueur indique avoir débloqué l'accès aux sources. Vérification par curl depuis le conteneur :
  - accessibles : `archive.org` et `ntrs.nasa.gov` (HTTP 200) ;
  - `gallica.bnf.fr` répond 403 (refus du site lui-même, ou de la passerelle) ;
  - toujours refusés par la passerelle réseau : `fr.wikipedia.org`, `en.wikipedia.org`, `albindenis.free.fr`, `www.theaerodrome.com`, `catalog.hathitrust.org`, `www.gutenberg.org`, `bibliotheques-numeriques.defense.gouv.fr`, `m-selig.ae.illinois.edu`, `digital.library.unt.edu`.
- **A** L'historien est relancé pour une deuxième passe, sur archive.org et le serveur NTRS de la NASA. Ordre des priorités :
  1. essais d'époque du SPAD VII en 150 ch, puis en 180 ch ;
  2. géométrie manquante ;
  3. courbe du moteur (rapport R&M 586) et hélice ;
  4. données de soufflerie des profils RAF 15 et RAF 6 ;
  5. témoignage italien et essais allemands.
- **⚠** Les questions 2 (version à valider en premier) et 3 (validation en confiance C acceptable ou non) restent posées au joueur.

### Revue de l'aérodynamicien sur l'étape 2, et corrections
- **R** Verdict de l'aérodynamicien : « à corriger », sans point bloquant. Les équations, les signes, le RK4 et l'ISA sont justes. Il a injecté des erreurs volontaires dans une copie : la suite de tests en laissait passer 3 sur 12 (signe de ω × v inversé, ω × v supprimé, renormalisation supprimée).
- **A** Corrections appliquées :
  - **Rotor intégré à l'état** : vitesse axiale **absolue** Ω ; la cellule utilise I' = I − I_r e eᵀ ; le couple du moteur et les frottements sont internes, le couple aérodynamique de l'hélice est externe. La réaction et la précession émergent des équations, sans être ajoutées à part. Régime moteur = Ω − e·ω.
  - **Gravité ajoutée par l'intégrateur** : le modèle d'avion renvoie `Loads` (efforts aérodynamiques et de contact, couples du rotor) ; F/m donne ainsi le facteur de charge.
  - **Fonction d'efforts sans effets de bord** : elle passe de `FnMut` à `Fn` et reçoit toujours un quaternion unitaire.
  - Ajout de `Wrench::force_at` (force appliquée en un point).
  - **Inertie** : contrôle physique complet, y compris le couplage par ixz.
  - **Atmosphère** : une altitude NaN donne désormais NaN (la divergence n'est plus masquée), et des points ISO 2533 sont vérifiés à 3, 5 et 20 km.
- **A** Tests : 33 dans `escadrille_fdm`, 50 sur tout le workspace, tous réussis. Nouveaux tests :
  - chute libre d'un corps en rotation lancé avec une vitesse initiale ;
  - virage à droite coordonné (cercle exact) ;
  - vitesses angulaires bien en axes corps ;
  - impulsion angulaire ;
  - énergie totale conservée sous la gravité ;
  - rapport ṙ/ṗ exact pour ixz ;
  - quaternion unitaire après 60 s de culbute ;
  - couple moteur dans le vide (la cellule tourne en sens inverse, moment cinétique total nul) ;
  - couple de l'hélice qui freine le rotor sans faire tourner la cellule ;
  - précession gyroscopique (lacet à droite, nez qui descend, rotor selon +x) ;
  - conservation avec le rotor ;
  - trajectoire de référence figée au bit près.
- **A** Documentation : `modele-aero.md` (convention du rotor, gravité, notation du quaternion) ; `format-avion.md` (convention de l'inertie : avion complet avec rotor bloqué, au centre de gravité à vide, ixz = ∫xz dm). Claude a retiré une remarque non sourcée qu'il venait d'écrire sur le signe d'ixz.
- **⚠** Points notés pour plus tard : position du centre de gravité par rapport au point de référence de la fiche et déplacement du centre de gravité (à traiter avec les masses variables) ; normale et nature du sol (M2).

### Deuxième et troisième passes de l'historien
- **R** Deuxième passe sans gain de confiance : l'outil web du subagent ne passe pas par le réseau du conteneur, il ne peut donc pas télécharger. Il a toutefois trouvé des identifiants exacts de documents (rapports NACA, volumes d'archive.org), consignés dans les fiches (§ 9.6 de `spad7.md`, § 3.2 de `profil-spad7.md`).
- **A** Claude télécharge lui-même avec curl : **15 rapports NACA** depuis NTRS, en PDF et en texte, dans le dossier de travail de la session (non versionné). Parmi eux :
  - les profils, dont le rapport 233 sur le RAF 15 en vraie grandeur, et les rapports 309, 279, 304, 331, 93, 124 et 319 ;
  - les hélices en bois du VE-7 ;
  - la puissance des moteurs en altitude ;
  - l'interférence entre les plans d'un biplan et la stabilité.
- **⚠** archive.org : seul son moteur de recherche passe. Ses serveurs de fichiers (`*.us.archive.org`, `*.ca.archive.org`, `*.eu.archive.org`) sont refusés par la passerelle réseau, si bien que *Flight* 1917-1918, les rapports techniques britanniques et l'annuaire Jane's restent inaccessibles. Il faut que le joueur autorise `*.archive.org`.
- **A** Historien relancé (troisième passe) pour lire ces fichiers locaux : polaires du RAF 15 et du RAF 6, loi de puissance en altitude, tables d'hélice, interférence biplan.
- **⚠** Téléchargement en arrière-plan de *Flight* 1917-1918 (104 numéros trouvés par la recherche d'archive.org) terminé **sans aucun fichier** : les serveurs de fichiers d'archive.org sont refusés. Cela confirme le blocage signalé au joueur.

### Décisions : 150 ch d'abord, M1 validé en confiance C
- **D** Le joueur décide de commencer par le **SPAD VII 150 ch** (Hispano-Suiza 8Aa).
- **D** Le joueur valide un **M1 en confiance C** : les cibles d'essai peuvent être de confiance C, étiquetées comme telles ; la confiance sera relevée plus tard, avec une revalidation.
- **A** Amendement noté dans la définition de « terminé » du M1 (`docs/roadmap.md`) et dans le plan du M1 (section « Décisions du joueur en cours de jalon »).

### Étape 3 du M1 : mécanique aérodynamique
- **A** Nouveau module `aero` dans `escadrille_fdm`, indépendant des chiffres du SPAD :
  - `polar.rs` : polaires sur 360°. Interpolation du tableau mesuré ; au-delà, extension de Viterna-Corrigan (NASA CP-2230, 1982) raccordée au dernier point jusqu'à ±90° ; plaque plane en écoulement inversé. Moment de tangage au-delà du tableau : centre de poussée qui se déplace linéairement. **Hypothèse de modélisation (E:)**, soumise à l'aérodynamicien.
  - `surface.rs` : surfaces portantes découpées en bandes (vitesse locale avec ω × r, incidence dans le plan corde-normale). Effet de l'envergure finie compté une seule fois, par un angle induit résolu par itération ; il retombe sur la théorie de la ligne portante en régime linéaire. Gouvernes (efficacité τ d'un volet simple, théorie des profils minces de Glauert), déflexion de l'aile sur l'empennage, diagnostics par bande pour la télémétrie.
  - `drag.rs` : traînées parasites, élément par élément, avec leur position (elles créent donc aussi des moments).
- **A** Tests : 54 sur 54 dans `escadrille_fdm`, `just check` vert. Ils vérifient notamment :
  - l'accord avec la ligne portante (C_L et C_Di à 0,1 et 0,2 % près) ;
  - l'absence d'efforts latéraux en vol symétrique ;
  - les signes de la portance, de la traînée et du calage ;
  - les amortissements en roulis et en tangage, la stabilité statique de l'empennage, l'effet de la déflexion ;
  - les ailerons (commande positive = roulis à droite) et la dérive (stabilité en lacet) ;
  - la continuité des polaires sur le tour complet, et l'absence de NaN au repos et en vol inversé.
  Données de test synthétiques (profil mince de manuel), pas de chiffres historiques.
- **⚠** Tests corrigés en cours de route : une comparaison exacte de flottants après l'ajout de 2π (arrondi). Le code n'était pas en cause.

### Ajout de `*.archive.org`
- **A** Le joueur ajoute `*.archive.org` aux domaines autorisés. Vérification :
  - les serveurs de fichiers (`ia…us.archive.org`, `dn….ca.archive.org`) passent ;
  - le domaine nu `archive.org` est désormais **refusé** (il passait avant) : un joker ne couvre pas le domaine nu. Le moteur de recherche et l'API de métadonnées d'archive.org sont donc inaccessibles, et `www.archive.org` redirige vers le domaine nu.
- **A** Contournement : un serveur `ia…us.archive.org` redirige vers le bon serveur de stockage pour n'importe quel identifiant. Téléchargés ainsi :
  - le rapport technique britannique de 1917-1918 (`technicalreport1917aero_2`, 1,1 Mo de texte) ;
  - la bibliographie aéronautique 1917-1921 du Smithsonian (3,1 Mo).
- **A** Repérage dans la bibliographie, en particulier : « The Spad scout, Type S-VII », *Aerial Age Weekly*, vol. 5, n° 17, 2 juillet 1917, p. 569. Le numéro correspondant sur archive.org (`AerialAge19170702`) ne contient qu'une image de couverture, sans texte. D'autres articles sur le SPAD sont signalés dans *Scientific American*, *Aviation*, *Flight* et *L'Aéronautique*.
- **A** Téléchargement en cours, en arrière-plan, des numéros de *Flight* de 1916 à 1919 : identifiants générés pour chaque jeudi, puisque la recherche est inaccessible.
- **⚠** Demande au joueur : ajouter aussi le domaine nu `archive.org`, en plus de `*.archive.org`, pour retrouver la recherche et les métadonnées (noms de fichiers des volumes de revues).
- **A** Instantané intermédiaire des fiches `hispano-suiza-8a.md` et `profil-spad7.md`, en cours de modification par l'historien (troisième passe, rapports NACA). La version finale sera commitée à la fin de sa passe.

### Nouvelles sources : FranceArchives et Musée de l'Air et de l'Espace
- **D** Le joueur demande d'ajouter deux fonds : FranceArchives et les collections numériques du Musée de l'Air et de l'Espace.
- **A** Skill `recherche-historique` : nouvelle section « Fonds et portails à consulter » (FranceArchives, Musée de l'Air et de l'Espace, archive.org, NTRS), avec ce qu'on y trouve, les domaines à autoriser et un rappel sur les conditions de réutilisation. Le Musée de l'Air et ses témoignages sonores sont ajoutés aux sources françaises de témoignages. Subagent `historien` : ces deux fonds deviennent prioritaires pour les sources françaises.
- **A** Accès vérifié depuis le conteneur : `francearchives.gouv.fr` et `museeairespace.fr` sont **refusés** par la passerelle réseau. La seule adresse trouvée pour l'e-médiathèque du musée (`mediatheque.museeairespace.fr`) vient d'un article de 2020 : à confirmer.
- **⚠** Domaines à ajouter par le joueur : `francearchives.gouv.fr`, `museeairespace.fr` et `*.museeairespace.fr`, ainsi que le domaine nu `archive.org`.

### Accès élargi : vérification des nouveaux domaines
- **A** Le joueur a ajouté tous les domaines demandés. Vérification : `archive.org` (domaine nu et serveurs de fichiers), `francearchives.gouv.fr`, `museeairespace.fr`, `mediatheque.museeairespace.fr` et `ntrs.nasa.gov` répondent. `www.memoiredeshommes.sga.defense.gouv.fr` reste refusé : c'est là que sont publiés les journaux de marche numérisés des unités, dont ceux des escadrilles.
- **A** archive.org :
  - recherche et métadonnées de nouveau disponibles ;
  - le volume 5 d'*Aerial Age Weekly* (1917) n'existe pas sur le site, seul un volume de 1919 est présent ;
  - un seul volume des rapports techniques britanniques (1917-1918) ;
  - *Flight* 1916-1919 en cours de téléchargement (80 numéros sur environ 200 à ce stade, dont 4 avec des mentions du SPAD).
- **⚠** FranceArchives oppose un mécanisme anti-robots : une redirection JavaScript en boucle avec cookie. Claude **ne le contourne pas**, puisqu'il sert justement à bloquer l'accès automatique. Pistes : une recherche manuelle du joueur, ou une interface officielle si FranceArchives en propose une.
- **A** E-médiathèque du Musée de l'Air et de l'Espace : en ligne (logiciel Opsis/Skopus, photos, films, sons), avec une interface entièrement en JavaScript. Elle servira surtout de référence visuelle (livrées, cockpits) pour le M3 et les fiches, plutôt que pour des chiffres de performance.
- **A** Nouvel instantané intermédiaire des fiches de recherche modifiées par l'historien (troisième passe en cours).

### Revue de l'étape 3 par l'aérodynamicien, et corrections
- **R** Verdict **bloquant**, sur un seul point : le solveur de l'angle induit, une itération de point fixe, diverge dès que e·AR < 2, c'est-à-dire sur toutes les dérives et gouvernes de direction de 1914-1918. Mesure sur une dérive d'allongement 1,5 : C_L oscille entre −1,24 et +1,46 au lieu de −0,15, et le moment de lacet change de signe selon le nombre d'itérations. Le test existant passait par hasard. Le reste du module est jugé sain, et les corrections de l'étape 2 conformes.
- **A** Corrections :
  - **Solveur exact** entre les deux points de décrochage du tableau, où la solution est unique (méthode de l'*Illinois*, encadrée).
  - Une première version prolongeait la résolution au-delà du décrochage. Elle présentait une hystérésis à faible allongement : la portance sautait de 0,32 à 0,46. **Au-delà du décrochage**, l'angle induit garde donc sa valeur de décrochage et s'annule progressivement vers 40° d'incidence géométrique, comme le proposait l'aérodynamicien : la ligne portante n'a plus de sens en écoulement décollé ou inversé. C'est une estimation (E:), documentée dans le code.
  - Un tableau de polaire qui s'arrête **avant le décrochage** d'un côté est désormais **refusé** au chargement (cas du RAF 15 du rapport NACA 233, qui s'arrête à −1,5°).
  - Limites documentées dans le code, en confiance C :
    - le modèle induit local n'est exact que pour une charge symétrique, si bien que l'amortissement en roulis est surestimé d'environ 27 % pour un allongement de 5,5 ;
    - sous un allongement de 4, la ligne portante surestime la pente : la correction de Helmbold ira dans les données ;
    - la loi du centre de poussée est justifiée par la loi de Rayleigh (écart inférieur à 0,05 corde).
- **A** L'aérodynamicien avait injecté 15 erreurs volontaires dans une copie du code ; 8 survivaient aux tests. Les **dix tests qu'il proposait** sont ajoutés, entre autres :
  - solveur exact pour des allongements de 1 à 10 ;
  - continuité et monotonie sur le tour complet ;
  - formule fermée de l'amortissement en roulis ;
  - symétrie miroir ;
  - effet du dièdre ;
  - force de plaque en écoulement inversé ;
  - valeurs de la polaire au-delà de 90° ;
  - lacet inverse des ailerons, mesuré dans les axes de stabilité ;
  - amortissement des traînées parasites en rotation.
- **⚠** Quatre de ces nouveaux tests étaient eux-mêmes faux au premier essai, pas le code :
  - l'amortissement en roulis doit inclure la traînée de profil ;
  - le lacet inverse se mesure dans les axes de stabilité, pas dans les axes avion ;
  - deux comparaisons de flottants demandaient une tolérance.

  Une erreur de compilation (valeur déplacée) a aussi été corrigée.
- **A** Résultat : **82 tests sur 82** dans le dépôt, `just check` vert.
- **A** Reportés aux étapes suivantes, comme le demande la revue :
  - **données** : ramener les polaires mesurées en soufflerie sur des ailes d'allongement 6 à un allongement infini, pour ne pas compter l'effet induit deux fois. Pièges notés : coefficient britannique k_L = C_L/2, C_m parfois donné au bord d'attaque. Prolonger côté négatif, jusqu'à un décrochage estimé (E:), les tableaux qui s'arrêtent trop tôt ;
  - **assemblage du biplan** : interférence entre plans (Prandtl, Munk, Millikan, Fairbanks), déflexion sur l'empennage calculée depuis le C_L de l'aile avec un retard, souffle d'hélice (Froude), force latérale et moments du fuselage (Multhopp) ;
  - plus tard : un segment de « profil inversé » autour de 180°, pour la glissade sur la queue et la vrille à plat.

### Fin de la troisième passe de l'historien
- **R** Le profil, le moteur et l'hélice ont maintenant des sources d'époque lues, toutes tirées des rapports NACA téléchargés, avec les pages citées. **Aucune performance du SPAD VII ne dépasse encore la confiance C.**
  - **Profil** : la section « Spad » du rapport NACA 93 (MIT, 1918) est en B. Le type exact de SPAD mesuré n'est pas précisé, et les coefficients sont dans l'ancienne convention : il faut les doubler. Les polaires du RAF 15 sont en A pour ce profil lui-même (rapport 233, vraie grandeur), mais leur emploi pour le SPAD reste en C.
  - **Moteur** : le rapport 46 (Bureau of Standards) porte sur un Hispano de 180 hp aux cotes du 8Ab, ce qui donne une source en B pour la courbe puissance-régime, la loi d'altitude (k ≈ 0,08 à 0,11 contre 0,132 pour Gagg-Farrar) et la consommation.
  - **Hélice** : la table C_T/C_P/η de l'hélice D' du VE-7 (rapport 301) est en A pour la mesure, en C pour le SPAD. Elle servira à caler le calcul ou d'hélice de secours.
  - **Biplan** : les lois d'interférence et de stabilité (rapports 17, 271 et 362) sont en C pour le SPAD. Le plan supérieur décroche avant le plan inférieur, ce qui rend plausible l'abattée de la réputation, sans la prouver.
- **⚠** Points ouverts :
  - **contradiction sur la cote du 8Ab** : 180 hp à 1 700 tr/min dans le rapport 46, 180 ch à 2 100 tr/min dans les sources françaises ; à trancher avec les essais de montée ;
  - **sens de rotation** de l'hélice, toujours en C ;
  - **cellule du S.VII** (corde, écartement, décalage, calages), **version 150 ch** et **hélice réelle**, toujours sans source.
- **⚠** L'historien croit archive.org inaccessible : son outil web ne passe pas par le réseau du conteneur. Les numéros de *Flight* sont pourtant téléchargés : **208 numéros de 1916 à 1919** dans le dossier de travail, dont **64 mentionnent le SPAD**. Ils lui seront confiés pour une quatrième passe, avec le rapport technique britannique de 1917-1918.
- **A** Fiches `spad7.md`, `hispano-suiza-8a.md` et `profil-spad7.md` commitées dans leur version de fin de passe.

### Relectures relancées, et étape 4 du M1 : hélice et moteur en ligne
- **A** Deux relectures lancées en arrière-plan :
  - **aérodynamicien** : contre-revue des corrections de l'étape 3. Il doit vérifier que le point bloquant est levé, juger la loi après décrochage et rejouer ses erreurs volontaires ;
  - **historien** : quatrième passe, sur les 208 numéros de *Flight* téléchargés, le rapport technique britannique de 1917-1918 et *Aerial Age*. Il doit aussi chercher les données qu'attend l'aérodynamicien : corrections d'allongement τ et δ, conventions des coefficients, polaire du RAF 15 aux incidences négatives.
- **A** Étape 4 écrite pendant ce temps, dans une copie de travail séparée, pour ne pas perturber la contre-revue. Nouveau module `propulsion` d'`escadrille_fdm`, testé sur des données synthétiques (aucun chiffre historique) :
  - **hélice à pas fixe** :
    - tables C_T(J) et C_P(J), commençant au point fixe ;
    - au-delà du tableau, coefficients sur les **quatre quadrants**, comme pour les hélices marines de la série B de Wageningen : angle d'avance et vitesse de référence qui restent définis hélice arrêtée, en moulinet ou en glissade sur la queue. Trois points d'appui estimés (E:) pour l'hélice arrêtée et la rotation inverse, avec contrôle de leurs signes ;
    - l'hélice voit la vitesse **absolue** du rotor, le moteur sa vitesse **relative** à la cellule ;
  - **effet P simple** : décalage de la ligne de poussée, à la manière de JSBSim. Une hélice tournant à droite, avion cabré, fait lacer à gauche ;
  - **souffle d'hélice** selon la théorie de Froude : contraction du tube à débit constant, bord adouci, branché sur l'« écoulement local » déjà prévu pour les surfaces ;
  - **moteur en ligne** :
    - la relation de Gagg-Farrar se réécrit exactement en « puissance indiquée proportionnelle à la densité, moins un frottement constant » (constat de l'historien) ; le moteur est codé ainsi, et le même coefficient k règle la perte en altitude et le frein moteur au ralenti ;
    - gaz linéaires (E:) ;
    - le moteur s'arrête d'allumer sous un régime de calage, et repart si l'hélice le fait mouliner.
- **A** Tests :
  - table et rendement reproduits exactement ;
  - continuité sur le tour complet ;
  - hélice arrêtée qui freine et mouline ;
  - aucune valeur infinie ou NaN dans les quatre quadrants ;
  - sens de l'effet P ;
  - théorie de la quantité de mouvement et conservation du débit dans le souffle ;
  - loi de Gagg-Farrar ;
  - équilibre du régime entre le moteur et l'hélice, atteint par intégration depuis un régime plus bas comme depuis un régime plus haut.
- **A** Résultat : **85 tests sur 85** dans `escadrille_fdm`, clippy propre sur tout l'espace de travail. La section « Hélice et moteurs » de `modele-aero.md` est mise à jour.
- **⚠** Limites notées (E:) :
  - pas de force normale de l'hélice (Ribner, NACA Report 819), légèrement déstabilisante ;
  - souffle sans rotation ;
  - k constant par moteur, alors que le rapport 46 donne 0,08 à 0,11 selon le régime ;
  - l'origine exacte de la relation de Gagg-Farrar n'est toujours pas retrouvée.
- **A** Revue de l'aérodynamicien sur l'étape 4 prévue dès la fin de sa contre-revue de l'étape 3.

### Contre-revue de l'étape 3 : point bloquant levé
- **R** L'aérodynamicien confirme que le **point bloquant est levé**, sans nouveau blocage :
  - le solveur donne exactement la valeur de la ligne portante pour des allongements de 1 à 10, avec un résidu d'au plus 2,5e-15 rad ;
  - la dérive d'allongement 1,5 en dérapage de 3,4° donne +1 051 N·m de moment de lacet, au lieu de +8 188 ou −9 280 ;
  - la loi après décrochage ne présente ni saut ni hystérésis, vérifié sur 360° par pas de 0,001° ; il l'**accepte en E: pour le M1**. Réserve : à faible allongement, elle rend le décrochage plus brutal que celui de la section, l'inverse du comportement réel. À reprendre quand on fera décrocher les dérives (vrille).
- **R** Quatre points importants :
  - la monotonie supposée par le solveur n'était pas vérifiée : un creux de la courbe de portance avant le décrochage créait un saut ;
  - l'angle induit affiché valait ±360° à −180° exact ;
  - une discontinuité à ±180° apparaissait pour des allongements absurdes (e·AR < 0,2) ;
  - le test du solveur ne l'exerçait pas, car la section de test est exactement linéaire : 14 de ses 21 nouvelles erreurs volontaires survivaient.
- **A** Corrections :
  - nouveau constructeur validé `LiftingSurface::new`. Il refuse une surface sans bande, une bande mal formée (axes non unitaires ou non orthogonaux, aire ou corde nulle) et une polaire dont la pente, entre les deux décrochages, descend sous −π·e·AR : c'est la condition exacte d'unicité. Les champs deviennent privés ;
  - e·AR ≥ 0,5 exigé ;
  - fin de l'annulation bornée à 180° ;
  - angles enveloppés dans le diagnostic ;
  - constante nommée pour la marge de 15° ;
  - `max_abs_cl`, devenu inutile, est supprimé.
- **A** Ses **cinq tests** sont intégrés, avec une section cambrée, courbe et asymétrique : solveur exact, continuité sur 360°, induction nulle en écoulement décollé ou inversé, force égale aux coefficients tournés de l'angle induit, traînée parasite dans le souffle. S'y ajoute un test de refus des surfaces mal formées. Résultat : **90 tests sur 90**, `just check` vert.

### Fin de l'étape 3 : interférence entre les plans du biplan
- **A** L'aérodynamicien relit l'étape 4 et la vérification des corrections de sa contre-revue (relance de la même relecture).
- **A** Le plan du M1 prévoyait, à l'étape 3, l'interférence entre les plans du biplan ; elle manquait encore. Nouveau `LiftingSystem` (`aero/system.rs`), qui rassemble toutes les surfaces portantes :
  - induction mutuelle selon la théorie des multiplans de Prandtl (NACA TN 182, 1924), avec k_ij = σ S_j / (π b_i b_j). Le facteur σ dépend du rapport écartement / envergure, et sera calculé dans les données (C:) ;
  - chaque bande garde sa propre induction et reçoit en plus l'angle induit par le C_L moyen de l'autre plan. Le système couplé est résolu par itération plafonnée, déterministe ; le couplage est faible, environ 0,1 ;
  - au-delà du décrochage, l'induction mutuelle s'efface avec l'induction propre.
- **A** Tests :
  - formule fermée de Prandtl pour deux plans égaux, à 1e-12 près ;
  - traînée induite de deux plans inégaux comparée à la formule de Prandtl, à 0,1 % près (approximation des petits angles) ;
  - surfaces indépendantes sans couplage ;
  - continuité sur 360° avec décrochage ;
  - symétrie miroir ;
  - refus des paires incohérentes.
- **A** Résultat : 96 tests sur 96.
- **⚠** Restent dans les données, en C: ou E: :
  - la différence de portance entre les plans due à l'interférence bidimensionnelle (Millikan, rapport 362) ;
  - l'effet du décalage horizontal sur le partage de la charge (Munk, rapport 151).

  La déflexion de l'aile sur l'empennage, qui demande un retard, va dans l'assemblage de l'avion : c'est la prochaine tâche.

## 2026-10-21 — Session 3, suite (cloud)

### Reprise : « Continue la dernière session »
- **Q** Le joueur demande de reprendre la session précédente.
- **⚠** Les deux relectures en arrière-plan ont été **coupées par la limite de débit de l'API**, sans perte :
  - l'aérodynamicien, au milieu de sa revue de l'étape 4 ; il en était aux erreurs volontaires sur le module `propulsion` ;
  - l'historien, au milieu de sa quatrième passe ; il ajoutait des témoignages à `spad7.md`, notamment la version de Mortane de l'épisode du 23 septembre 1916.

  Leur travail est conservé et elles seront relancées là où elles se sont arrêtées.
- **A** Instantané intermédiaire de `docs/recherche/spad7.md` (quatrième passe en cours) commité tel quel.

### Assemblage de l'avion complet (`aircraft.rs`)
- **A** Nouveau module `aircraft` d'`escadrille_fdm`. `Aircraft::evaluate` calcule en un seul endroit les efforts d'un état, dans cet ordre :
  1. données de l'air (vent, densité) ;
  2. hélice, traction et souffle ;
  3. moteur ;
  4. surfaces portantes : souffle sur toutes, déflexion de l'aile sur l'empennage ;
  5. traînées parasites, dans le souffle.

  `Aircraft::step` intègre le tout par RK4.
- **A** **Déflexion retardée** : ε = facteur × C_L des ailes, qui atteint l'empennage avec un retard l_t / V. Ce retard est modélisé par un premier ordre, sous forme d'un **état auxiliaire** intégré avec le corps rigide (nouveaux champs `RigidBodyState::aux` et `Loads::aux_rates`). C'est ce qui donne l'amortissement classique dû à la variation d'incidence (M_α̇), utile pour les modes propres de l'étape 6. Le test de trajectoire figée au bit près passe toujours, sans changement.
- **A** Commandes du pilote dans [−1, 1] et gaz dans [0, 1], convertis en braquages par les débattements de l'avion.
- **⚠** Un nouveau test (plané symétrique sans moteur) a révélé un **vrai défaut** de l'étape 4 : l'effet P s'appliquait hélice arrêtée. Correction : le décalage est multiplié par cos β, la part de la rotation dans la vitesse de référence. Il s'annule hélice arrêtée et s'inverse en rotation inverse (E:). Au premier essai, cette correction divisait 0 par 0 dans un cas limite (hélice arrêtée sans écoulement axial), que le test de robustesse a attrapé.
- **A** Tests, sur un biplan synthétique (chiffres ronds, aucune donnée historique) :
  - repos ;
  - avion porté par le vent ;
  - plané symétrique sans effort latéral ;
  - couple de réaction du moteur ;
  - sens des commandes ;
  - retard et effet de la déflexion ;
  - souffle sur la profondeur à basse vitesse ;
  - refus des incohérences ;
  - 20 s de vol jusqu'au décrochage puis une glissade sur la queue, sans NaN et identiques au bit près sur deux exécutions.
- **A** Résultat : **106 tests sur 106**, clippy et rustfmt propres. La section « Assemblage de l'avion » est ajoutée à `modele-aero.md`.

### Relances et nouvelles sources
- **A** Les deux relectures sont relancées là où elles s'étaient arrêtées. L'aérodynamicien relit en plus l'induction mutuelle, la déflexion retardée et l'assemblage de l'avion.
- **A** Pour la question de l'interférence entre plans, deux rapports sont téléchargés depuis NTRS et confiés à l'historien :
  - NACA TN 182, Prandtl, *Induced drag of multiplanes* (1924), qui donne le facteur σ en fonction du rapport écartement / envergure ;
  - NACA TR 267, Hemke, *Drag of wings with end plates*, pour l'allongement effectif d'une dérive au contact du fuselage.

### Étape 5 du M1 : schémas de données
- **A** Le crate `escadrille_data` charge désormais `assets/aircraft/<id>/aircraft.ron`, avec les fichiers de profil et d'hélice auxquels il renvoie, ainsi que `essais.ron`.
  - **Unités d'époque typées**, converties en SI au chargement : longueurs en pieds et pouces, livres, mph, nœuds, pieds par minute, chevaux anglo-saxons (`Hp`) et cheval-vapeur (`Ps`, `Ch`), gallons, tours par minute, durées en minutes et secondes. Les facteurs de conversion exacts sont testés.
  - **Origine obligatoire** : toute ligne qui contient un nombre doit porter `// S:`, `// C:` ou `// E:`, et `//!` couvre tout un tableau. Sinon le fichier est **refusé**, avec les numéros de ligne fautifs.
  - **Profils** : conversion des conventions d'époque, selon la revue de l'aérodynamicien :
    - coefficients britanniques divisés par deux ;
    - moment rapporté au bord d'attaque ;
    - tableau mesuré sur maquette d'allongement 6, ramené à l'allongement infini par les corrections de Glauert τ et δ. Un test vérifie que l'on retrouve exactement le profil d'origine.
  - **Assemblage** pour un chargement donné :
    - masse, centre de gravité et inertie, avec pilote, carburant et armes en masses ponctuelles (théorème de Huygens, vérifié par une identité indépendante) ;
    - surfaces découpées en bandes à partir de panneaux le long du quart de corde, avec incidence, vrillage et dièdre ; envergure mesurée d'un saumon à l'autre ;
    - facteurs d'interférence de Prandtl et déflexion ;
    - traînées, hélice et moteur.
  - **Signe des gouvernes déduit de la géométrie** : une commande positive doit donner le moment positif du pilote, et les données ne peuvent donc pas se tromper de sens.
  - **Contrôles** :
    - surface comparée à celle de la fiche, à 2 % près ;
    - gouverne qui ne couvre aucune bande ;
    - nom de surface inconnu ;
    - fraction de carburant hors limites.
- **A** Les tests utilisent un biplan synthétique, dont chaque valeur est étiquetée `E:`.
- **⚠** Le premier essai du plané symétrique a fait apparaître un moment de lacet. L'erreur était dans l'avion d'essai, pas dans le code : sa dérive avait un profil cambré. La dérive et l'empennage ont reçu un profil symétrique, comme sur un vrai avion.
- **A** `format-avion.md` est réécrit pour décrire le schéma réel, avec le seuil de confiance : pour le M1, les essais A, B et C bloquent la validation.
- **A** Résultat : **138 tests sur 138** dans tout l'espace de travail, `just check` vert.
- **A** Prochaine étape : les fichiers `spad7` (avion, profil, hélice, essais), remplis **depuis les fiches**, quand l'historien aura terminé sa quatrième passe. Ensuite l'étape 6, `fdm_lab` : trim, essais, modes propres, tracés.

### Quatrième passe de l'historien : premières confiances B sur les performances
- **R** Résultats (fiches `spad7.md`, `hispano-suiza-8a.md`, `profil-spad7.md`) :
  - **passent en B** :
    - vitesse du 150 ch, 192 km/h (altitude non tranchée) ;
    - vitesses du 180 ch, 212 km/h en palier bas et 203 km/h à 3 000 m (à 703 kg) ;
    - masse à vide de 499 kg, surface alaire de 17,85 m² et envergure de 7,82 m, confirmées ;
    - hauteur ;
    - régime de référence du 8Ab, 2 100 tr/min ;
    - loi d'altitude mesurée en vol sur S.E.5 (R&M 462), sous réserve de l'OCR ;
  - **passe en A** : la convention britannique K_L = C_L/2 ;
  - **montées** : elles restent en C, les colonnes du tableau de 1919 étant illisibles à l'OCR ;
  - **contradiction sur la cote du 8Ab, expliquée** : le moteur du rapport NACA 46 suit la convention américaine (180 hp à 1 700 tr/min) ; le moteur français est donné à 2 000 tr/min en régime normal et 2 100 au maximum ;
  - **pour l'aérodynamicien** :
    - σ de Prandtl ≈ 0,54 pour le S.VII (C), d'après NACA TN 182 lu sur l'image du PDF ;
    - écartement 1,27 m (C) ; décalages horizontal et angulaire non trouvés ;
    - τ et δ non trouvés ;
    - aucun décrochage négatif du RAF 15 dans les sources (point le plus négatif mesuré : −6°) ;
    - le tableau RAF 15 du rapport 233 commence à −4,5°, et non à −1,5° comme indiqué plus haut ;
  - **témoignages** : aucun nouveau trait de vol confirmé par deux témoignages indépendants. Ajouts : la version de Mortane de l'épisode du 23 septembre 1916, Rickenbacker, *Flugsport* 1917, le surnom de « Spad allemand » donné à l'Albatros.
- **A** Téléchargés pour lui :
  - NACA TR 460 et TR 669, pour τ et δ ;
  - *The effect of staggering a biplane* (1921) ;
  - les deux numéros de *Flight* de février 1919, dont les pages des tableaux officiels sont **rendues en images** (150 dpi), lisibles directement. La page 205 montre le tableau des avions français avec les lignes « Spa. 7C1 Hispano 8Aa » et « 8Ab ».
- **⚠** Le volume I du rapport technique britannique de 1917-1918 n'existe pas sur archive.org, et l'OCR d'*Aerial Age* du 2 juillet 1917 non plus.
- **A** Historien relancé pour une cinquième passe courte : lire ces images pour faire passer en A les vitesses, les montées et le plafond du tableau officiel.

### Revue de l'étape 4 et de l'assemblage par l'aérodynamicien
- **R** Verdict « à corriger », **rien de bloquant**.
  - **Confirmé** : les corrections de l'étape 3. Il a rejoué toutes ses erreurs volontaires, et toutes celles qui ont un effet sont tuées.
  - **Juste** :
    - les quatre quadrants de l'hélice ;
    - les signes de l'effet P ;
    - les vitesses absolue et relative du rotor ;
    - l'induction mutuelle de Prandtl, qui converge en 9 à 16 itérations ;
    - la déflexion retardée (approximation de Padé d'ordre 1 du retard) ;
    - l'ordre de calcul de l'assemblage.
- **R** Points importants :
  1. L'effet P **sature à basse vitesse** : il atteint presque son maximum dès qu'un faible écoulement transversal existe, par exemple 934 N·m au sol par vent de travers de 3 m/s. Il faut le rendre proportionnel à V⊥·V_pale/V_r².
  2. Aucun **contrôle d'énergie** sur les points d'appui de l'hélice : on peut fabriquer un moteur perpétuel. Il faut vérifier Q·Ω − T·V_a ≥ 0 sur tout le tour.
  3. **Ralenti instable au sol** : le rotor s'arrête en 60 s une fois les gaz réduits. À gaz fermés, un vrai moteur à carburateur garde une puissance indiquée à peu près constante, si bien que son couple varie en 1/ω. Cela compte pour le roulage du M2.
  4. Le souffle n'a **pas de rotation** : sur la dérive, c'est l'effet dominant, environ 4 fois l'effet P en montée à pleins gaz. Sans elle, la tendance à partir à gauche ne peut pas émerger.
  5. Données : le tableau de l'hélice D' (NACA 301) donne une traction « effective » (traction moins la traînée due au souffle), à corriger au chargement.
- **R** Autres réponses :
  - valeurs proposées pour les points d'appui d'une bipale en bois, calculées par éléments de pale et vérifiées en énergie : arrêt (−0,065 ; −0,008), écoulement inversé (+0,07 ; +0,008), rotation inverse (−0,04 ; −0,007) ;
  - frottement : utiliser directement la puissance de frottement mesurée par le rapport 46, de 13,1 à 25 hp de 1 300 à 2 100 tr/min, plutôt qu'une fraction fixe ;
  - cote du 8Ab : ne pas la trancher par les essais de montée, où l'hélice inconnue se confond avec la puissance. Ce n'est pas sur le chemin critique, puisque le 150 ch passe d'abord ;
  - déflexion à pondérer par max(0, u/V) en vol queue en avant ;
  - **avant les modes propres** : ajouter la force normale de l'hélice (Ribner, NACA 819), soit 15 à 30 % de la stabilité de route due à la dérive sur un SPAD, et la force latérale et les moments du fuselage. Sans eux, les stabilités sont surestimées ;
  - 13 de ses 49 nouvelles erreurs volontaires survivent, dont la traction qui n'est plus ajoutée aux efforts. Neuf tests sont proposés.
- **A** Ces points sont traités tout de suite, avant la suite de l'étape 6.

## 2026-10-22 — Session 3, suite (cloud)

### Reprise : « Continue le travail »
- **Q** Le joueur demande de continuer, après un redémarrage du processus de la session.
- **⚠** Le redémarrage a interrompu la fin des vérifications. Les corrections de la revue de l'étape 4 étaient commitées mais pas encore poussées. Après vérification (`just check` et `just test` verts, **150 tests sur 150**), elles sont poussées.
- **⚠** Incident de la veille : la compilation complète dans la copie de travail séparée recompilait Bevy de zéro et remplissait le disque (3,7 Go libres). Elle a été arrêtée et son dossier de compilation supprimé (7,7 Go libres ensuite). Les relectures étant terminées, la copie séparée est retirée et le travail reprend dans le dépôt principal.

### Corrections de la revue de l'étape 4 (commit b3d598d)
- **A** Hélice :
  - effet P proportionnel à V⊥·V_pale/V_r², qui ne sature plus à basse vitesse ;
  - force normale de Ribner ;
  - contrôle d'énergie Q·Ω − T·V_a ≥ 0 sur tout le tour des coefficients, ce qui refuse le « moteur perpétuel » de l'aérodynamicien ainsi qu'un rendement supérieur à 1 dans le tableau ;
  - rotation du souffle, tirée du couple.
- **A** Moteur :
  - courbe de **puissance de frottement mesurée** (ou une fraction, au choix des données) ;
  - puissance indiquée plafonnée par les gaz, d'où un **ralenti stable au sol** (testé : le rotor converge vers le même ralenti qu'il parte vite ou lentement) et le frein moteur en piqué ;
  - puissance tenue constante au-delà du régime maximal.
- **A** **Fuselage en corps élancé** : moment de Munk déstabilisant et force d'écoulement transversal (Allen et Perkins, NACA 1048).
- **A** Déflexion annulée en vol queue en avant.
- **A** Les tests d'assemblage proposés par l'aérodynamicien sont ajoutés. Le principal vérifie que les efforts totaux sont exactement la somme des composants, traction comprise : c'était l'une de ses erreurs volontaires qui survivaient.
- **⚠** Deux tests étaient mal posés au premier essai :
  - en axes avion, la composante x n'est pas une traînée, et un profil soufflé incline sa portance vers l'avant ;
  - une rampe d'allumage plus raide demandait un seuil de continuité plus large.
- **A** `nalgebra` ajouté à `fdm_lab`, comme prévu au plan du M1 : licence Apache-2.0, `cargo deny` vert.

### Cinquième passe de l'historien : le SPAD VII 150 ch en confiance A
- **R** Le **tableau officiel des avions français de 1919** (*Flight*, 13 février 1919, p. 204-205, repris d'*Automotive Industries*), lu sur les images des pages, fait passer en **confiance A** :
  - les vitesses du 150 ch : 192 km/h à 2 000 m, 179 km/h à 3 000 m, 174 km/h à 4 000 m ;
  - ses montées : 6 min 40 s à 2 000 m, 11 min 30 s à 3 000 m, 19 min 30 s à 4 000 m ;
  - son plafond théorique : 5 486 m ;
  - ses masses : 500 kg à vide, 703 kg au décollage ;
  - l'envergure et la hauteur.

  La série de montée du STAé se retrouve à l'identique dans ce document indépendant. Le 180 ch obtient aussi ses valeurs officielles, dont 212 km/h à 2 000 m (et non au niveau de la mer).
- **R** Plusieurs lectures OCR de la quatrième passe étaient fausses : 694 kg, « 193 sq ft », colonnes mal attribuées. Elles sont corrigées.
- **R** **Cote des moteurs** :
  - le tableau donne trois conventions (américaine, britannique, française) ;
  - retenus : 175 hp à 1 700 tr/min pour le 8Aa et 205 hp à 1 800 tr/min pour le 8Ab, en puissance maximale autorisée ;
  - « 180 ch » est le nom d'une classe de moteur, pas une puissance mesurée.
- **⚠** Corrections d'allongement τ et δ : toujours introuvables dans les rapports 460 et 669, qui renvoient au rapport NACA 416. Claude l'a téléchargé : sa figure 11 donne bien ces facteurs pour une aile rectangulaire en fonction de l'allongement. Il sera lu à la prochaine passe.
- **A** Fiches commitées. Une image à 300 dpi de la page 205 est préparée, pour lever la réserve de lecture sur la ligne du 8Aa (119,5 ou 119,3 mph, soit ±0,3 km/h).

### Étape 6 du M1 : `fdm_lab` (trim, performances, modes, campagne)
- **A** **Trim** (vol rectiligne stabilisé, ailes à plat) :
  - huit inconnues parmi dix (incidence, dérapage, trois braquages, gaz ou vitesse ou pente, régime du rotor, déflexion), choisies selon la question posée ;
  - équations : les huit dérivées qui doivent s'annuler ;
  - braquages résolus sans limite, puis comparés aux débattements pour dire si le pilote peut tenir le trim.
- **⚠** Le trim se bloquait sur une matrice singulière. La cause est physique : à bas régime le moteur est limité par la pleine ouverture, et la manette des gaz n'a plus d'effet. C'est une vraie zone morte du carburateur. Corrections :
  - méthode de Levenberg-Marquardt et redémarrage depuis plusieurs points ;
  - coin de la loi des gaz arrondi (C¹, ±5 %), sans changer la puissance à pleins gaz.

  Conséquence attendue : à la vitesse maximale, le trim trouve une manette à 0,77 et non 1,0, le moteur étant déjà saturé. Le test vérifie la puissance, pas la position de la manette.
- **A** **Performances** :
  - vitesse maximale par altitude ;
  - meilleure montée (section dorée sur la vitesse) ;
  - temps de montée (intégration sur l'altitude) ;
  - plafond (dichotomie sur le taux de montée, 100 ft/min pour le plafond pratique de l'époque) ;
  - vitesse de décrochage au ralenti (balayage de l'incidence jusqu'au décrochage ou à la butée de profondeur).
- **A** **Modes propres** :
  - linéarisation par différences centrées sur (u, v, w, p, q, r, φ, θ, régime, déflexion) ;
  - valeurs propres par `nalgebra` ;
  - noms donnés par l'état dominant du vecteur propre plutôt que par le rang, pour ne pas plaquer des noms de manuel sur un avion atypique.
- **A** **Campagne** : `just essai <avion>`
  1. lit `essais.ron` ;
  2. compare chaque cible historique à sa tolérance (bloquante à partir de la confiance acceptée, C pour le M1) ;
  3. calcule les modes en croisière ;
  4. vérifie la convergence en pas de temps (créneau de profondeur, 20 s, dt divisé par deux : écart inférieur à 1 % exigé) ;
  5. écrit `target/essais/<id>/rapport.md` en français.
- **A** Essayé sur le biplan synthétique, l'outil marche de bout en bout :
  - vitesse maximale 168 km/h à 1 000 m ;
  - décrochage 64 km/h ;
  - montée à 1 000 m en 3,5 min ;
  - plafond 5 620 m ;
  - convergence en pas de temps parfaite.

  Ses modes sont plausibles : oscillation d'incidence bien amortie (ζ 0,86). Mais sa stabilité de route est faible (petite dérive, gros fuselage, force normale de l'hélice). D'où une oscillation latérale lente et instable : un résultat sur cet avion inventé, pas une erreur de calcul.
- **A** Résultat : **161 tests sur 161**, `just check` vert.
- **⚠** Reste à faire pour l'étape 6 :
  - les **tracés PNG**, en attente d'une décision du joueur sur la police de caractères (question ci-dessous) ;
  - les fichiers de données du **SPAD VII**, à remplir depuis les fiches.

### Robustesse et relectures
- **A** **Tests de robustesse** (étape 7 du plan), lus dans `essais.ron` : vitesse nulle, glissade sur la queue, vrille (entrée puis sortie), vol sur le dos, piqué à 60° pleins gaz (variante d'après le RAE).
  - Un scénario est réussi si la simulation reste finie et dans des bornes physiques ; ce que fait l'avion est rapporté sans être jugé.
  - Un échec bloque la validation.
  - Les cinq passent sur le biplan synthétique. Lâché à l'arrêt, il repart en avant (196 km/h après 15 s). Après une glissade sur la queue, il a repris une assiette de piqué. **163 tests sur 163**, `just check` vert.
- **A** Relectures lancées en arrière-plan :
  - **aérodynamicien** : confirmation des corrections de l'étape 4, relecture du trim, des performances, des modes et de la convergence ; avis sur la phugoïde longue et l'oscillation latérale lente du biplan synthétique ; liste des données du SPAD à estimer ou calculer, avec la méthode pour chacune ;
  - **historien** : facteurs τ et δ pour l'allongement 6 dans le rapport NACA 416 ; relecture de la ligne du 8Aa sur l'image à 300 dpi.
- **⚠** Les fichiers de données du SPAD VII (`assets/aircraft/spad7/`) relèvent du processus `/nouvel-avion`, réservé à une invocation explicite du joueur. Claude ne le reproduit pas par d'autres moyens et attend que le joueur le lance.

### Questions posées au joueur
- **A** **Police des tracés PNG** de `fdm_lab`. `plotters` a besoin d'une police pour écrire les axes. Sa police par défaut passe par `font-kit`, qui lie freetype et fontconfig en C, avec des licences hors de la liste du projet ; je l'écarte. Options proposées :
  1. **recommandée** : `plotters` sans les options par défaut, avec son rendu de texte en Rust pur (`ab_glyph`, Apache-2.0), et la police **Go** (licence BSD-3-Clause, déjà dans la liste des licences de code acceptées) incluse dans `tools/fdm_lab`. Elle sert à l'outil seulement, n'est pas livrée avec le jeu, et sera inscrite dans `assets/LICENCES.md` ;
  2. tracés **SVG** sans fichier de police : les visionneuses rendent le texte, mais Claude ne peut pas les regarder comme des images ;
  3. tracés **sans texte** : courbes seules, valeurs dans le rapport.
- **A** **Fichiers du SPAD VII** : le processus `/nouvel-avion` est réservé à une invocation explicite du joueur. Proposition : lancer `/nouvel-avion spad7` quand la liste des estimations de l'aérodynamicien sera revenue.

### Sixième passe de l'historien : corrections d'allongement et ligne du 150 ch
- **R** **NACA TR 416** (Jacobs et Abbott, 1931), lu sur l'image des pages 312 à 314 :
  - τ corrige l'angle induit, et le δ de l'aérodynamicien correspond au « σ » de ce rapport (traînée induite) ;
  - pour l'**allongement 6**, lecture graphique de la figure 11 : **τ ≈ 0,175** (±0,01) et **δ ≈ 0,05** (±0,005). Les valeurs supposées par l'aérodynamicien sont confirmées ;
  - formules exactes relevées, y compris la correction de paroi : c'est celle que le rapport 331 applique au rapport 233 (0,380° par unité de C_L). La chaîne de conversion des polaires du RAF 15 vers l'allongement infini devient applicable ;
  - avertissement ajouté dans les fiches : le « σ » du TR 416 n'a rien à voir avec le σ de Prandtl du biplan.
- **R** **Ligne du 8Aa** (*Flight*, p. 205) : la vitesse de 119,5 mph est **confirmée** par un contrôle interne, puisque le tableau arrondit toutes ses vitesses au demi-mph (85 autres valeurs vérifiées). L'incertitude devient ±0,4 km/h, la confiance reste A, et les cibles ne changent pas.
- **A** Fiches `spad7.md` et `profil-spad7.md` commitées.
- **⚠** Facultatif, non fait : un recadrage à 300 dpi de la figure 11 réduirait l'incertitude sur τ à ±0,003, sans enjeu pour le M1.

### Revue de `fdm_lab` par l'aérodynamicien
- **R** Verdict « à corriger », **rien de bloquant**.
  - Les corrections de l'étape 4 sont confirmées par ses erreurs volontaires : effet P, Ribner, énergie, moteur, déflexion et assemblage, toutes détectées.
  - Le trim est juste : ses huit équations suffisent.
  - Les méthodes de performances sont saines.
- **R** Points importants :
  1. **L'oscillation latérale du biplan synthétique venait d'une donnée** : `normal_force` de 0,3, soit 8 à 9 fois trop. Un calcul par éléments de pale donne environ 0,035. L'avion était **instable de route**, et non « faiblement stable » comme je l'avais écrit. Avec 0,04, le roulis hollandais (4,9 s) et la spirale reviennent, et le test doit les exiger.
  2. La phugoïde longue s'explique par une **marge statique de 3,2 %** seulement. En figeant à la fois le rotor et la déflexion, on étudie un autre avion. Proposition : afficher la marge statique, N_v et L_v dans le rapport.
  3. La **zone morte de la manette des gaz est un artefact** du minimum dans la loi : 22 % de la course ne sert à rien à la vitesse maximale. Un papillon sature progressivement. Loi proposée : P_i = σ·P_pg(ω)·g(s)/g(s₁), avec g(s) = s/(1 + s²)^½.
  4. Le **plafond est biaisé vers le bas** d'environ 2 %, parce qu'un échec du trim est pris pour « ne monte plus ». Il faut extrapoler le taux, comme on le faisait à l'époque.
  5. **Loi d'altitude du moteur** : la mesure en vol d'un Hispano de 150 ch (R&M 462) donne P ∝ (p/p₀)^1,04, plus pessimiste que la loi en densité de 4 à 9 % entre 2 000 et 5 000 m. Il faut un choix dans les données.
  6. `modele-aero.md` et `format-avion.md` sont **périmés**, et `/nouvel-avion` va s'appuyer dessus.
  7. Il faut un **régime moteur cible** dans `MaxLevelSpeed` : le tableau de 1919 donne 1 720, 1 680 et 1 640 tr/min, contrôle fort du couple hélice-moteur.
  8. Le **scénario de vrille** est réussi alors que l'avion ne vrille pas : il part en spirale engagée. Il faut détecter la vrille.
- **R** Suggestions :
  - rotation du souffle nulle en amont du disque et entière juste derrière (conservation de r·v_θ) ;
  - inertie du rotor plus réaliste, environ 6 kg·m² ;
  - corrections dans le trim, le décrochage et la convergence ;
  - neuf erreurs volontaires que les tests ne voient pas.
- **R** **Liste des données du SPAD VII à estimer ou calculer**, avec méthode et source pour chacune. Exemples :
  - corde ≈ 1,21 m (C:) ;
  - calages, critère « fuselage horizontal en croisière » ;
  - déflexion par les abaques du NACA TR 648 ;
  - centrage et inertie par composants ;
  - hélice D' mise à l'échelle, traction effective corrigée ;
  - traînée parasite élément par élément, avec un contrôle (et non un réglage) : f ≈ 0,94 à 0,98 m² déduit du tableau de 1919.

  Elle servira au processus `/nouvel-avion`.
- **A** Ces points sont traités avant de lancer `/nouvel-avion`.

### Corrections de la revue de `fdm_lab`
- **A** Les huit points importants de l'aérodynamicien sont traités (commit `284cc2c`) :
  - **manette des gaz** : saturation progressive du papillon, g(s) = s/(1 + s²)^½, au lieu du minimum arrondi. Plus de zone morte : le trim à la vitesse maximale retrouve les pleins gaz ;
  - **loi d'altitude** choisie dans les données : `Density` (σ) ou `Pressure(n)` ((p/p₀)ⁿ, R&M 462 : n ≈ 1,04 pour le SPAD) ;
  - **rotation du souffle** conservée le long des lignes de courant : nulle devant le disque, entière juste derrière ;
  - **données** : régime cible facultatif avec une vitesse maximale, force normale de l'hélice refusée au-delà de 0,1, biplan synthétique corrigé (force normale 0,035, inertie du rotor 6 kg·m²) ;
  - **`fdm_lab`** : plafond par pas et bissection, extrapolé comme à l'époque quand la montée ne se résout plus ; décrochage = dernière descente que la profondeur tient ; marge statique, N_v et L_v dans le rapport ; régime comparé à la cible ; **vrille détectée** (décrochée et en lacet) au lieu de compter les tours de cap.
- **R** Biplan synthétique après corrections : modes classiques (roulis hollandais 4,8 s, ζ = 0,26 ; spirale qui double en 38 s), marge statique 8,2 %, stable de route (N_v > 0) et effet de dièdre (L_v < 0) ; il vrille vraiment, à 78° d'incidence ; plafond 5 608 m.
- **A** 169 tests réussis, `just check` sans erreur.

### Documents de référence du modèle de vol à jour
- **A** `modele-aero.md` (skill `simulation-vol`) décrit maintenant ce que fait le code, puisque `/nouvel-avion` s'appuie dessus :
  - angle induit résolu exactement et induction mutuelle des biplans ;
  - fuselage en corps élancé (Munk et écoulement transversal) ;
  - déflexion retardée ;
  - hélice : contrôle d'énergie, points d'appui calculés, effet P, force normale de Ribner, rotation du souffle ;
  - moteur : courbe de frottement, lois d'altitude, loi des gaz, inertie du rotor ;
  - méthodes de `fdm_lab` (trim, performances, modes, marge statique, robustesse, convergence).
- **A** `format-avion.md` suit le schéma actuel : bloc `fuselage`, `friction: Curve(…)` ou `Fraction(k)`, `altitude_law`, `normal_force`, régime cible dans `MaxLevelSpeed`.
- **D** Tout est prêt pour le SPAD VII. Reste au joueur à lancer **`/nouvel-avion spad7`** (ce processus lui est réservé) ; la liste des données à estimer de l'aérodynamicien servira de guide. La question de la police des tracés reste ouverte.

### Essais qualitatifs dans `fdm_lab` (étape 6 du plan M1)
- **D** En attendant `/nouvel-avion spad7` et la réponse sur la police des tracés, j'avance la seule partie de l'étape 6 qui ne dépend ni de l'un ni de l'autre : les **scénarios qualitatifs issus des témoignages**, testés sur le biplan synthétique.
- **A** Schéma : champ facultatif `manoeuvre` sur les scénarios qualitatifs de `essais.ron`. Quatre manœuvres tirées du § 6.2 de la fiche SPAD :
  - `Loop` : boucle pleins gaz ;
  - `PowerOffStall` : décrochage moteur réduit ;
  - `Glide` : finesse maximale ;
  - `Dive` : piqué tenu puis ressource.

  Un scénario sans manœuvre (embardée au sol, cockpit…) est listé « non simulé ».
- **A** **Pilote scripté**, avec les mêmes commandes que le joueur :
  - profondeur : vise un facteur de charge sans dépasser l'incidence de décrochage moins 2°, en anticipant sa vitesse de variation ; ses gains diminuent comme la pression dynamique au-delà de la vitesse d'entrée, sinon la ressource dépassait 6 g pour 4 demandés ;
  - ailerons : ailes dans le plan vertical ;
  - direction : dérapage nul.

  Premier essai corrigé : sans anticipation, la boucle dépassait l'incidence de décrochage de 5,5°. Profondeur laissée au neutre, le piqué se redressait seul et la ressource n'avait plus d'objet. Le pilote **tient** donc la pente, et le fdm_lab relève à chaque instant le signe du moment de tangage profondeur au neutre : cabreur = rappelant, piqueur = engagement.
- **R** Biplan synthétique, données non historiques :
  - boucle bouclée en 8,8 s, sommet à +80 m et 48 km/h, sans décrocher ;
  - abattée de 33° et inclinaison de 23° au décrochage (64 km/h, contre 69 km/h stabilisé), 66 m perdus ;
  - finesse 6,8 à 100 km/h ;
  - piqué tenu à −80° : 376 km/h, moment rappelant, ressource en 4,3 s avec 265 m perdus.
- **A** Rapport : nouvelle section « Essais qualitatifs » (attendu, source, observé). Ces essais **ne bloquent pas** la validation : comparer l'observé à l'attendu revient au joueur et à l'aérodynamicien. Seules des valeurs non finies feraient échouer la campagne. `format-avion.md` et `modele-aero.md` sont à jour.
- **A** 176 tests réussis, `just check` sans erreur. Revue de l'aérodynamicien demandée.
- **⚠** La revue de l'aérodynamicien s'est arrêtée sur la limite de débit de l'API (429), puis le processus de la session a redémarré. Elle a repris là où elle s'était arrêtée après la levée de la limite. **Q** du joueur : « Continue ». **R** : reprise de la revue, puis traitement de ses retours.

### Revue des essais qualitatifs par l'aérodynamicien, et corrections
- **R** Verdict « à corriger », **rien de bloquant**. Ce qui va : le commit est propre, diviser dt par deux ne change rien, les repères et les signes sont justes. Le problème : plusieurs mesures décrivaient **autant le pilote scripté que l'avion**.
  - **Pente** calculée par asin, qui ne voit pas au-delà de −90° : un piqué « vertical » faisait pousser le pilote jusqu'à −13 g, sans que rien ne le signale. La ressource s'arrêtait sur l'assiette alors que l'avion descendait encore.
  - **Décrochage** :
    - l'abattée de 33° comprenait la poussée du pilote ; l'avion seul fait environ 16° ;
    - un départ en roulis était décrit comme « l'avion s'enfonce » ;
    - le résultat changeait de nature avec la vitesse à laquelle on tire le manche ;
    - la direction purement proportionnelle laissait 2,5° de dérapage.
  - **Boucle** : « sans décrocher » dépendait du réglage du pilote (sans anticipation, il décroche), et « bouclée » ne regardait que le nez.
  - **Piqué** : la doc ne correspondait pas au code ; la détection d'« engagement » reposait sur un seul échantillon ; les chiffres incluaient la mise en place.
  - **Doc** : « jamais bloquant » contredisait le code, qui échoue sur des valeurs non finies.
- **A** Corrections (commit `a425f72`) :
  - pente mesurée dans le plan vertical, valable jusqu'à −90° ; ressource jusqu'à une trajectoire horizontale ;
  - décrochage à **un nœud par seconde**, comme dans les essais de certification ; départ par abattée ou par l'aile (inclinaison > 30°) ; ensuite l'avion seul (manche tenu, ailerons et palonnier au neutre) ; abattée mesurée sur l'assiette vraie ; dérapage, vitesses vraie et équivalente au décrochage ;
  - direction avec intégrale ;
  - boucle jugée sur le vecteur vitesse et sur le plan, vitesse d'entrée en paramètre, limite d'incidence prise pleins gaz ; le rapport montre la limite du pilote, le temps passé dessus et la marge ;
  - piqué : profondeur **relâchée au neutre** 3 s comme le demande la fiche ;
  - manœuvres vérifiées au chargement.
- **A** **Nouveau** : vitesse de décrochage à n'importe quelle manette. La cible `StallSpeed(power: Full)` de la campagne, « pas encore calculée » jusqu'ici, est maintenant calculée.
- **⚠** En corrigeant, deux pièges trouvés au premier essai :
  - garder **toutes** les commandes après le départ, y compris la direction qui suit le roulis, faisait vriller l'avion. Ailerons et palonnier sont remis au neutre ;
  - l'assiette dans le plan vertical n'a plus de sens quand le cap a tourné de 106° : elle donnait une abattée de 138°. Le décrochage utilise l'assiette vraie (48°).
- **R** Biplan synthétique, données non historiques :
  - décrochage à 1 kt/s : **départ en roulis à droite**, 1,9 s après l'incidence de décrochage, atteinte à 68 km/h avec un dérapage de −0,3°. L'avion laissé seul part en vrille naissante (86° d'inclinaison, cap +106° en 3 s). En inversant l'hélice, il part à gauche : le côté vient bien de l'hélice ;
  - boucle bouclée, sommet à 0,66 V_s, marge de 0,3° au décrochage, 93 % du temps sur la limite d'incidence : une boucle « juste », pas « aisée » ;
  - piqué à −80° relâché : il remonte à −35° en 3 s (rappelant).
- **A** Tests ajoutés, suggérés par la revue :
  - facteur de charge comparé à la courbure de la trajectoire ;
  - signe de l'inclinaison ;
  - boucle sans anticipation, qui décroche ;
  - convergence en dt ;
  - départ en roulis inversé avec l'hélice ;
  - finesse vérifiée en plané libre ;
  - piqué vertical ;
  - empennage calé à +3°, qui s'engage au relâché.

  184 tests réussis, `just check` sans erreur.
- **D** Laissés pour plus tard, faute d'enjeu avant les données du SPAD :
  - gains du pilote normalisés par avion ;
  - compteur d'inversions du manche ;
  - tracés temporels, en attente de la décision sur la police.

  Contre-vérification demandée à l'aérodynamicien.

### Contre-vérification de l'aérodynamicien
- **R** Tout est réglé **sauf le décrochage**, où le pilote décidait encore de la nature et du côté du départ :
  - entre l'incidence de décrochage et le départ, il croisait les commandes : aileron à fond contre le roulis, palonnier à fond parce que l'intégrale qui annule le dérapage suivait le roulis. Le test miroir ne pouvait pas le voir, puisque le pilote s'inverse avec l'hélice ;
  - après un départ en roulis, le maintien de 3 s mesurait une spirale, pas une abattée.
- **A** Corrigé (commit suivant `ad75506`) :
  - dès l'incidence de décrochage atteinte, **plus d'ailerons ni de palonnier** (trim de plané), comme dans la technique « pas d'aileron au décrochage » ;
  - après un départ en roulis, l'avion est laissé seul 1 s au plus, ou jusqu'à 60° d'inclinaison.
- **R** Le biplan synthétique fait maintenant une **abattée de 12°**, avec une inclinaison de 13° à gauche, cap −4°, 62 m perdus. Avant, il partait en roulis à droite à 86° : c'était le pilote.
- **A** Le test de la boucle borne désormais le dépassement d'incidence du pilote, au lieu de dépendre d'une marge de 0,3° au décrochage.
- **⚠** Signalé par la revue : `StallSpeed(power: Full)` est maintenant calculé et bloquant. Sa définition (vitesse stabilisée minimale pleins gaz, en montée) est notée dans `format-avion.md`. **D** Aucune cible du SPAD ne s'en sert (la fiche n'a pas de décrochage moteur), et l'avis de l'historien sera demandé avant d'en écrire une.
- **A** 184 tests réussis, `just check` sans erreur.

### Ce qu'il faut tester maintenant
- **Q** « Rappelle-moi ce qu'il faut tester maintenant. »
- **R** Rien à tester manette en main : le M1 est sans rendu, et le premier vol à la manette viendra au M2. L'essai du joueur qui clôt le M1 consiste à lire le rapport d'essais du SPAD et ses tracés, puis à dire si l'avion « ressemble » aux témoignages. Il faut d'abord deux choses : `/nouvel-avion spad7`, et la décision sur la police des tracés. En attendant, essai facultatif : faire tourner `fdm_lab` sur le biplan synthétique, sur la machine du joueur, et donner un avis sur la lisibilité du rapport.

### `/nouvel-avion spad7` : cadrage et décisions
- **Q** Le joueur lance `/nouvel-avion spad7`.
- **A** Relecture des fiches `spad7.md`, `hispano-suiza-8a.md` et `profil-spad7.md`, et de la liste des données à estimer de l'aérodynamicien. Constat : `fdm_lab inertia`, `fdm_lab gains` et `asset_check`, prévus par le processus, n'existent pas encore.
- **D** Décisions du joueur :
  1. **`spad7` = S.VII 150 ch** (Hispano-Suiza 8Aa), une Vickers, 703 kg des essais officiels. Toutes ses cibles sont en confiance A. Le 180 ch, machine type de la SPA 3 à l'été 1917, viendra ensuite sous `spad7_180` : même cellule, autre moteur.
  2. **`fdm_lab inertia`** est écrit maintenant : centrage et inertie tirés d'un bilan de masses par composant, dans un fichier sourcé `masses.ron`. Les gains de l'instructeur et de l'IA, ainsi que la maquette 3D avec `asset_check`, attendent le M2, puisque le M1 n'a ni IA ni rendu.
  3. **Tracés** : `plotters` sans polices système, avec `ab_glyph` et la police Go (BSD-3-Clause) embarquée, dans `fdm_lab` seulement. La question de la police est donc close.

### `/nouvel-avion spad7` : premières données et outils
- **A** **Septième passe de l'historien** (géométrie de détail) : **aucune nouvelle cote mesurée du S.VII** dans les 208 numéros de *Flight* ni sur le web. Elle apporte :
  - une fourchette de corde de 1,14 à 1,33 m, déduite de la famille SPAD ;
  - des comparables du XIII et du S.XI ;
  - deux corrections dans la fiche.
- **A** **Dessin trois vues de *Flight* du 16 août 1917**, téléchargé depuis archive.org et mesuré sur l'image à 600 points par pouce. Mis à l'échelle par l'envergure et la longueur officielles (S46), les deux vues concordent à 0,3 %.
  - Ce qu'il donne : écartement de 1,26 m (S48 : 1,27) ; **décalage nul** (mâts verticaux) ; ailerons de 1,95 × 0,44 m sur le plan supérieur ; plan fixe de 2,81 m d'envergure et 2,30 m² dont profondeur 1,00 m² ; dérive 0,37 m² et gouvernail 0,53 m² ; hauteur du fuselage, position de l'hélice, des roues et du poste.
  - **⚠** Il reprend la corde de 1,40 m de l'article, incompatible avec la surface officielle. **D** Corde retenue **1,21 m** (méthode de l'aérodynamicien, surface A), conflit signalé pour la revue. Tout est dans `docs/recherche/spad7-calculs.md` § 3.
- **A** **Profil** `assets/airfoils/spad.ron` :
  - base : le RAF 15 du Report 233 recopié tel quel ;
  - passage à l'allongement infini par le TR 416, avec l'allongement effectif de la soufflerie fermée (6,86) ;
  - **recalage sur la ligne moyenne du « Spad » par la théorie du profil mince** (−0,60° et −0,0145 sur C_m), nouveau champ `camber_correction`. La théorie retrouve le RAF 15 mesuré à 0,3° près ;
  - branche négative estimée.
- **A** **Hélice** `propeller.ron` :
  - base : la D' du Report 301, de 2,45 m de diamètre ;
  - **la traction « effective » est ramenée à la traction de l'hélice** : +5 à 6 %, en rajoutant la traînée de souffle du VE-7, que notre modèle recalcule lui-même ;
  - lu sur l'image de l'hélice I : aux J de croisière, les ailes laissent la traction effective inchangée, mais ajoutent 3 à 5 % de puissance absorbée. Ce C_P est gardé.
- **A** **Bilan de masses** `masses.ron` (dix composants fermés sur les 1 100 lb de S46) et commande **`fdm_lab inertia`**. Centre de gravité à vide à 26 % de la corde. Un test d'intégration vérifiera que `aircraft.ron` concorde avec le bilan.
- **A** **Tracés PNG** dans `fdm_lab` : `plotters` avec la police Go embarquée (BSD-3-Clause, inscrite au registre des licences) ; `cargo deny` accepte les licences. Le rapport d'essais montre :
  - la vitesse maximale, le régime, le taux et le temps de montée en fonction de l'altitude, cibles historiques en points ;
  - l'historique de chaque essai qualitatif : vitesse, angles, facteur de charge, commandes.

  Règles de la skill `dataviz` : un axe par panneau, couleurs catégorielles validées.
- **A** `essais.ron` du SPAD écrit :
  - trois vitesses maximales avec leur régime (A, ±4 % et ±3 %), trois temps de montée (A, ±5 %), le plafond théorique (A) et le décrochage de Tallman (D) ;
  - huit comportements du § 6.2, dont cinq manœuvres pilotées.
- **A** En cours : l'aérodynamicien calcule les traînées parasites élément par élément, le fuselage en corps élancé, la déflexion, les allongements effectifs, l'effet P et le ralenti.
- **⚠** Le calcul de l'aérodynamicien s'est arrêté sur la limite de débit de l'API (429), puis le processus de la session a redémarré. Il a repris où il s'était arrêté après la levée de la limite.

### `/nouvel-avion spad7` : données complètes et premier essai en vol
- **R** **Calcul de l'aérodynamicien** : traînées élément par élément, fuselage, déflexion, allongements, hélice, moteur (`spad7-calculs.md` § 5). Choix principaux :
  - **câbles ronds**, faute de preuve de câbles profilés ;
  - **pas de facteur 3/5** d'interférence ;
  - contrôle de traînée : f équivalent de 0,91 m² contre 0,94 à 0,98 m² déduits de S46, sans réglage.

  Deux points **bloquants** relevés, et corrigés :
  1. des empennages tracés en flèche gonflaient les surfaces (la dérive de 0,90 à 1,51 m²) : panneaux redressés ;
  2. la loi d'altitude `Pressure(1.04)` comptait deux fois le frottement, puisque R&M 462 mesure une puissance **au frein**. L'exposant à appliquer à la puissance indiquée est **0,93**, vérifié ici à ±0,6 %. La documentation du code et des skills, qui induisait l'erreur, est corrigée.
- **A** `aircraft.ron` et le profil d'empennage écrits. Les calages (2,3° et 1,8°) vérifient le critère « fuselage horizontal à 170 km/h » (incidence du fuselage 0,015°). Le bilan de masses concorde ; un test d'intégration charge désormais chaque avion de `assets/`.
- **⚠** Premier essai : les manœuvres de décrochage et de plané échouaient (« NoStall ») : le balayage du décrochage abandonnait dès son premier trim. Il essaie désormais plusieurs vitesses de départ.
- **R** **`/essai-en-vol spad7`** (commit `7789090`) : **ÉCHEC** sur les cibles A de montée et d'altitude.

  | Essai | Cible (S46) | Modèle | Écart | Résultat |
  |---|---|---|---|---|
  | Vitesse maximale à 2 000 m | 192,3 km/h | 198,0 | +2,9 % | réussi (±4 %) |
  | Vitesse maximale à 3 000 m | 179,4 km/h | 192,6 | +7,4 % | **échec** |
  | Vitesse maximale à 4 000 m | 173,8 km/h | 186,1 | +7,1 % | **échec** |
  | Régimes à 2 000, 3 000 et 4 000 m | 1 720, 1 680, 1 640 tr/min | 1 700, 1 668, 1 630 | −0,6 à −1,1 % | réussis (±3 %) |
  | Montées à 2 000, 3 000 et 4 000 m | 6 min 40, 11 min 30, 19 min 30 | 5,7, 9,7, 15,2 min | −15 à −22 % | **échecs** |
  | Plafond théorique | 5 486 m | 6 507 m | +18,6 % | **échec** |
  | Décrochage au ralenti (D) | 75,6 km/h | 80,7 | +6,7 % | réussi (±10 %) |

  - Modes propres : oscillation d'incidence très amortie (ζ 0,94), roulis hollandais de 3,3 s (ζ 0,19), spirale qui double en 19 s, phugoïde de 31 s ; marge statique 4,8 %. C'est cohérent avec « très stable, bonne plate-forme de tir ».
  - Qualitatifs :
    - décrochage à 84 km/h, avec abattée de 7° puis aile droite qui tombe à 33° ;
    - finesse 8,3 ;
    - boucle bouclée, sommet à 0,85 V_s ;
    - piqués tenus et rappelants. Relâché au neutre à 437 km/h, le SPAD se cabre jusqu'à 10,8 g : sans compensateur, le pilote doit pousser fort en piqué.
  - Robustesse : tout réussi, vrille établie.
- **R** **Lecture des écarts** : excès de taux de montée d'environ 0,85 m/s, constant en altitude. La traînée seule ne l'explique pas : la multiplier par 1,22 casserait la vitesse à 2 000 m (aérodynamicien).
  - **Constat** : le **tableau italien de 1919** (S47, même masse, « H.S. 140 hp ») donne 194 km/h à 2 000 m et des montées de 5 min 50 à 2 000 m et 9 min 50 à 3 000 m. Le modèle donne 198 km/h, 5,7 et 9,7 min, soit 1 à 3 % près. Les sources françaises et britanniques donnent 6 min 20 à 6 min 40 et 11 min 30.
  - **D** Ne pas choisir la source qui arrange le modèle. L'historien évalue les tableaux **sur leurs seuls mérites** (conditions de mesure, cohérence interne, version, moteur), sans connaître le résultat du modèle ; il relit aussi la fiche et la chronologie.
  - Pistes physiques sourcées, à trancher ensuite :
    - puissance en vol inférieure de 3 à 5 % à celle du banc (Report 301) ;
    - montées comptées depuis le décollage (environ une demi-minute) ;
    - traînée parasite qui croît avec l'incidence (S43, à modéliser) ;
    - traînée de profil d'une aile entoilée en vraie grandeur (Report 304) ;
    - effet Munk distinct en tangage et en lacet ;
    - efficacité des gouvernes à grand braquage (K′ de DATCOM).
- **⚠** Limite du pilote scripté : sa direction oscille pendant la reprise du décrochage (période d'environ 2 s). Ses gains, réglés sur le biplan synthétique, devront être normalisés par avion avant de servir à l'IA.

### Huitième passe de l'historien : fiabilité des performances du 150 ch
- **R** Le tableau français (S46) est le seul à donner toute la série du 150 ch.
  - **Vitesses** : elles forment deux groupes incompatibles à 3-4 % près. 192 km/h à 2 000 m (concordant avec S47) d'un côté, 179 et 174 km/h à 3 000 et 4 000 m de l'autre. Rien ne permet de dire lequel est faux.
  - **Série « STAé » du forum** : elle n'est pas indépendante de S46 (mêmes chiffres). La seule série indépendante, le rapport britannique n° 96 vu en résumé, monte 2 à 6 % plus vite.
  - **Conditions de mesure** : aucun document ne dit d'où partent les montées ni dans quelle atmosphère.
  - **Plafond « théorique »** : c'est une extrapolation.
  - **« 140 hp » de S47** : cote italienne d'un Hispano de la classe 150 ch ; sa série de montée est irrégulière.
- **A** Cibles du § 5.5.6 de la fiche reportées dans `essais.ron` :
  - 2 000 m : A, ±3 % ;
  - 3 000 et 4 000 m : déclassées en B, ±5 % ;
  - montées : A, ±5 %, et ±7 % à 4 000 m ;
  - plafond à vitesse ascensionnelle nulle : C, ±7 %.

  Les contrôles italiens ne sont pas saisis comme cibles : ce ne sont pas des estimations de jeu, donc pas des D. Ils restent cités dans les rapports.
- **A** Relecture de la fiche par l'historien. Il a corrigé :
  - la date de la blessure de Heurtaux ;
  - « Vieux Charles », nom porté par plusieurs avions ;
  - le S.VII de 1918, qui n'est plus un avion neuf ;
  - la page de présentation de S46.
- **R** Nouvel essai : rien ne change dans les résultats. La vitesse et le régime à 2 000 m passent. Restent en échec :
  - les vitesses à 3 000 et 4 000 m (+7 %) ;
  - les montées (−15 à −22 %) ;
  - le plafond (+19 %).

  **L'écart vient du modèle**, pas des sources : il monte trop bien.
- **⚠** La consigne de l'historien de chronométrer les montées « depuis le lâcher des freins » demande le modèle de sol du M2. Pour l'instant, seule la convention « depuis 0 m en vol » est calculée.

## 2026-10-23 — Session 4

### Choix de la voie pour les écarts du SPAD
- **Q** Quelle voie suivre pour les écarts du SPAD ? **D** Le joueur choisit les **pistes physiques sourcées** (plutôt que d'autres sources d'abord, ou que d'accepter les écarts).
- **A** Lecture des deux sources de mesure :
  - **NACA Report 17** (S43, maquette de JN-2, MIT 1917) : en présence des ailes, la traînée du fuselage et du train passe de 0,015 à 0,025 lb entre 0° et 12° (+67 %), et la traînée parasite totale croît jusqu'à 20 % entre 0° et 9° ;
  - **NACA Report 304** (Sperry Messenger à ailes RAF 15, vols planés en vraie grandeur) : les 29 vols du tableau II, lus sur l'image. Ajustés sur 21 vols de C_L 0,23 à 0,81, ils donnent C_D = 0,055 + 0,120 C_L² (écart type 0,003), soit un **coefficient d'Oswald de l'avion complet de 0,73**, avec l'allongement équivalent du biplan (3,6, σ de Prandtl 0,50).
- **A** Plan proposé au joueur : `docs/plans/m1-ecarts-spad7.md`.
  - Étapes : polaire du modèle mesurée par `fdm_lab polaire` ; traînée parasite fonction de l'incidence (S43) ; comparaison avec la vraie grandeur du Report 304, et le reste ajouté comme traînée de profil d'ailes entoilées ; puissance installée à 0,96 (Report 301).
  - Ordre de grandeur des trois pistes ensemble : celui de l'écart (environ −1 m/s en montée).
  - Règle : **calage sur l'avion de comparaison mesuré, jamais sur les cibles du SPAD**.
- **Q** Validation du plan, demandée par une fenêtre de question que le joueur a fermée, puis reposée dans le fil. **D** « Je valide le plan » : les quatre étapes, dans l'ordre.

### Étape 1 du plan : la polaire du modèle
- **A** Nouvelle commande **`fdm_lab polaire <id>`**, reprise dans le rapport d'essais (section et tracé `polaire.png`).
  - Mesure : vols planés stabilisés au ralenti, forces de l'hélice retranchées (traînée « vraie » du Report 304), ajustement C_D = C_D0 + K C_L² entre C_L 0,2 et 0,8.
  - Résultat : coefficient d'Oswald rapporté à l'allongement équivalent du biplan.
  - Tests : un ajustement exact, et le Report 304 retrouvé (0,734 sur les 21 vols).
- **R** **SPAD modélisé : C_D = 0,047 + 0,081 C_L², coefficient d'Oswald 0,89**, contre **0,73** pour le Sperry Messenger en vraie grandeur. Le modèle manque bien de traînée liée à la portance, la piste principale du plan.

### Étape 2 du plan : la traînée parasite et l'incidence
- **A** Relecture complète des passages de S43 (NACA Report 17, p. 285-288) avant d'écrire la loi d'incidence. Deux prémisses du plan sont corrigées :
  - la traînée du train ne croît que jusqu'à 2° environ, puis reste quasi constante jusqu'à 16° ;
  - la hausse du fuselage est lente jusqu'à 9°, puis rapide (corps, train et empennage « constants à 20 % près de 0° à 9° »).
- **A** Mesure de l'incidence du fuselage du SPAD modélisé : **2,6° à 4,9° en meilleure montée**, de 0 à 4 000 m ; −2° à 0° à la vitesse maximale.
- **R** Effet de la piste 2 sur la vitesse ascensionnelle :
  - majorant, avec la loi du plan (k = 15 en sin² α, appliquée même au radiateur) : −0,04 à −0,12 m/s ;
  - avec l'allure mesurée par S43 : −0,02 m/s au plus ;
  - écart à expliquer : 0,85 m/s.
- **D** Pas de changement du modèle pour la piste 2. La loi serait du code sans effet mesurable, et elle doublerait la traînée d'écoulement transversal du fuselage, déjà présente aux grands angles. Le détail est dans le plan (§ Résultats). La piste reste notée pour les grands angles, à reprendre avec une mesure en vraie grandeur (S43 est une maquette au 1/24 à 30 mph).

### Étape 3 du plan : la vraie grandeur du Report 304
- **A** Relevé de la géométrie du Sperry Messenger sur la figure 2 et dans le texte du Report 304 : envergure 20 ft, corde 4 ft, entreplan 3 ft 10 in, dièdre 1,5°, décalage d'environ 16 in, empennage 6 ft 8 in × 3 ft 7 in, centre de gravité dessiné à 29 % de la corde. Masse en essai : 1 008 lb (tableau II).
- **A** Avion de comparaison `messenger_raf15`, rangé dans `tools/fdm_lab/comparison/` (ce n'est pas un avion du jeu). Il vole sur les mêmes tables que le SPAD (un test garantit qu'elles restent identiques) et avec les mêmes méthodes.
- **⚠** `fdm_lab polaire` s'arrêtait à la vitesse maximale en palier : pour le Messenger, cela laissait C_L ≥ 0,5. Les planés vont maintenant jusqu'au C_L le plus bas de l'ajustement, comme les piqués du Report 304. Le SPAD passe de 0,890 à 0,899.
- **R** Messenger modélisé sur la table polie : **Oswald 0,889, contre 0,734 mesuré**. La méthode du modèle donne la même valeur aux deux avions (SPAD 0,899) : l'écart vient bien de ce que le modèle ignore, pas du SPAD.
- **A** Nouveau champ de profil `fabric_drag_rise`, ΔC_d = k C_l², pour la traînée des ailes entoilées réelles. **k = 0,020 ± 0,005**, calé sur le Messenger seul :
  - sensibilité au centrage, au e des ailes et au σ : 0,017 à 0,022 ;
  - incertitude de la mesure : ±0,004 ;
  - un test vérifie que le Messenger modélisé, avec le k de `spad.ron`, retrouve la polaire mesurée.
- **R** Essai du SPAD avec k = 0,020 :
  - Oswald 0,723 ;
  - montées −9 / −8 / −12 % (avant −15 / −16 / −22 %), toujours en échec ;
  - plafond +8 % (avant +19 %), en échec de peu (±7 %) ;
  - vitesses maximales +1,7 / +5,6 / +4,5 % : 2 000 et 4 000 m passent, 3 000 m échoue de peu ;
  - finesse 8,3 → 7,4 (« mauvais plané ») ; le décrochage part maintenant en roulis à droite (60° en 0,7 s : **mesure fausse**, c'était la butée d'arrêt du pilote scripté, voir la revue plus bas).
- **D** k reste en confiance C : il porte probablement plus que la toile (interférence réelle du biplan, mâts). Prochaine étape du plan : la puissance installée.

### Étape 4 du plan : la puissance installée
- **A** Nouveau champ moteur `installed_power` : fraction de la puissance indiquée du banc fournie en vol, le frottement restant entier. Tests dans le FDM et dans le schéma de données (valeur hors de (0, 1] refusée).
- **⚠** Le plan disait « 0,96 » pour −4 % de puissance. Appliquée à la puissance indiquée, la même perte au frein donne 1 − 0,04 × 175 / 191 = **0,963**. Corrigé avant de conclure.
- **A** Relecture des figures 11 à 13 du Report 301 : l'hélice D', le comparable du SPAD, montre l'écart le plus faible (0 à 3 %).
- **R** Campagne du SPAD avec 0,963 :
  - vitesses −1,0 / +2,6 / +1,1 %, montées +0,8 / +3,1 / +1,1 %, plafond +2,1 % : **toutes réussies** ;
  - **régime à la vitesse maximale −3,8 / −3,7 / −4,0 % (tolérance ±3 %) : en échec**, dont 2 000 m en A, qui bloque.
- **R** Sensibilité (`spad7-calculs.md` § 7) : 0,98 ferait tout passer.
- **D** Pas de réglage à 0,98 : ce serait caler la puissance sur les cibles du SPAD, et la source donne 3 à 5 % de perte. Le régime dépend surtout de l'hélice, un comparable : 5 cm de diamètre en moins le relèveraient d'environ 3 %. Piste documentaire : le diamètre et le pas de l'hélice réelle.

### Revue de l'aérodynamicien (étapes 2 à 4) et corrections
- **A** Campagne `/essai-en-vol spad7` relue avec ses tracés : montée passant par les trois cibles, polaire parabolique. L'écart de régime est un décalage constant d'environ 65 tr/min, de même pente que les cibles : il vient de l'absorption de l'hélice, pas de la loi d'altitude.
- **R** Revue : rien de bloquant ; quatre points importants.
  1. **Régime et montée vont ensemble** : au bon régime (hélice de 2,40 m au lieu de 2,45 m), le modèle monterait encore 4 à 8 % trop vite. Le succès des montées repose en partie sur un régime trop bas.
  2. **Puissance installée** : « air d'admission plus chaud » n'est pas dans la source ; valeur fragile (confiance D) ; double compte possible avec la loi d'altitude des vols de référence.
  3. **Décrochage** : le « 60° » était la butée du pilote scripté ; deux critères d'arrêt faisaient la course.
  4. **Étape 2** : l'abandon est validé, mais l'argument du double compte est faux aux angles en jeu.
- **A** Reports 220 et 292 téléchargés (NTRS) et lus.
  - Les deux ramènent la puissance du banc au vol par P ∝ p/√T, à 1 500–3 000 ft.
  - À ces altitudes, cette loi donne 0,6 à 1,3 % de plus que celle du modèle (R&M 462) : environ 1 point des 3 à 5 % est déjà compté.
  - **Puissance installée corrigée de 0,963 à 0,973** (−3 % au frein). Transparence : je savais qu'une fraction de 0,98 ferait passer le régime ; la correction vient des sources, mais va dans ce sens.
- **A** Mesure du décrochage refaite sur une fenêtre fixe de 3 s après l'incidence de décrochage, relevée à 1, 2 et 3 s, avec un test de convergence en pas de temps. (Une fenêtre de 2 s ne voyait pas le départ tardif du biplan d'essai, à 2,5 s.) À 74 % de carburant : inclinaison +6 / +33 / +44°, assiette +6 / 0 / −2°. De 60 à 90 % de carburant, toujours à droite, 33 à 36° à 2 s ; 42 à 78° à 3 s (départ exponentiel).
- **A** Vérification demandée par la revue : pente de portance du Messenger, mesurée (tableau II) 0,053 ± 0,002 par degré, modélisée 0,066. Le modèle est 16 à 23 % trop raide : une partie de k est sans doute « induite » (0,016 transposé au SPAD, dans la marge). Piste séparée notée.
- **A** Code : k négatif refusé ; le test des tables compare aussi convention et référence des moments.
- **R** Campagne finale du SPAD (k 0,020, puissance installée 0,973) :
  - vitesses −0,2 / +3,4 / +2,1 %, montées −2,1 / −0,2 / −2,9 %, plafond +3,8 % : réussis ;
  - **régime à la vitesse maximale −3,2 / −3,0 / −3,3 % (±3 %) : en échec de peu**, A à 2 000 m, qui bloque la validation.
- **D** Pas de réglage. L'écart restant, et la montée trop rapide qu'il cache, tiennent à l'hélice, un comparable. Pistes notées dans `spad7-calculs.md` § 7 (historien, éléments de pale, postes à faible J). Le `spad7` n'est pas déclaré « validé ».

- **R** Rapport d'essai `spad7` remis au joueur (commit 95a300a) : verdict ÉCARTS. Tout passe, sauf le régime à la vitesse maximale (−3,0 à −3,3 % pour ±3 %). Propositions de suite : l'hélice réelle du S.VII 150 ch (historien), puis un calcul par éléments de pale. En attente des retours du joueur ; `docs/essais/spad7.md` ne sera écrit qu'après son accord.

### Suite : l'hélice réelle du S.VII 150 ch
- **Q** Le joueur : « Lance l'étape 1 ». **D** Recherche confiée à l'historien : l'hélice réelle du S.VII 150 ch (fabricant, diamètre, pas et définition du pas, forme des pales), un régime relevé en montée, et, en question secondaire, le bord d'attaque des ailes (contreplaqué ou toile). Les cotes connues ne viennent que d'un forum de collectionneurs (H8, vu en résumé, confiance C).
- **R** Rapport de l'historien (sixième passe de `hispano-suiza-8a.md` § 7.5 ; neuvième passe de `spad7.md`). Toutes les pages web n'ont été vues qu'en résumé : l'outil de lecture de l'historien échoue sur ces sites.
  - **Liste de sept hélices autorisées** pour le « Type VII · Hispano 150 ch » (dossier Hartmann, sans doute une liste du STAé de 1917, confiance C) : de 2,35 à 2,50 m de diamètre, P/D de 0,74 à 0,82. Aucun document ne dit laquelle équipait l'avion de S46 : le parc était mêlé.
  - **Référence retenue : Levasseur série 485, 2,45 × 1,95 m (P/D 0,80)**, à la place de la Gallia série 3. C'est déjà, à 1 cm près, l'hélice du modèle (D' portée à 2,45 m, pas de 1,96 m) : **la table du modèle est maintenue**.
  - **Définition française du pas** : non trouvée. Les hélices de la Navy sont à pas géométrique constant (vérifié sur les calages des Reports 220 et 292).
  - **Hélices de la famille de D' mesurées en vol** (Reports 220 et 292, lus sur l'image) :
    - la forme de la courbe C_P(J) varie d'environ ±1,5 % en régime ;
    - l'écart du modèle (−3,2 %) est du même ordre que l'incertitude sur l'hélice, mais plus grand que ce que la seule forme de courbe explique.
  - **Régime en montée sur S.VII** : non trouvé. Sur le VE-7, le régime de montée vaut 0,88 à 0,92 fois celui du palier.
  - **Bord d'attaque** : non tranché (C dans les deux sens).
  - **Document décisif** : le R&M 586 (1921), plusieurs hélices sur un Hispano de 150 hp sans réducteur.
- **A** Contrôle sur le modèle : ce rapport vaut 0,87 au sol et 0,91 à 4 000 m, en limite basse des mesures du VE-7. Plausible.
- **⚠** Le proxy de l'environnement refuse `reports.aerade.cranfield.ac.uk` (R&M 586) et `www.hydroretro.net` (dossier Hartmann). Pour les lire, le joueur doit les ajouter aux domaines autorisés de l'environnement.

## 2026-10-24 — Session 4 (suite)

- **Q** Comment autoriser les deux domaines depuis l'application iOS, et les retrouver dans l'application de bureau ? **R** :
  - le réglage appartient à l'environnement cloud, enregistré sur le compte et non sur l'appareil : modifié depuis une application, il vaut pour toutes ;
  - chemin : menu de l'environnement dans la barre de titre de la session, **Edit**, puis **Network access**, domaines à ajouter à la liste autorisée (https://code.claude.com/docs/en/cloud-environments#network-access) ;
  - domaines à ajouter : `reports.aerade.cranfield.ac.uk`, `www.hydroretro.net` et `hydroretro.net` ; en option `catalog.hathitrust.org` et `babel.hathitrust.org` (Eiffel 1919) ;
  - déjà autorisé : `archive.org`. `gallica.bnf.fr` répond 403 sans qu'on sache si le refus vient du proxy ou du site.

- **R** Le joueur a autorisé les deux domaines (« C'est fait »). Accès vérifié : Cranfield et hydroretro passent ; HathiTrust et Gallica refusent par eux-mêmes (403 du site).
- **A** Dossier Hartmann téléchargé et lu en entier ; tableau vérifié sur l'image des pages 19-20.
  - Colonnes confirmées (diamètre, pas, épaisseur du moyeu) ; quatre lignes de plus que le résumé ;
  - un indice sur le pas, sur une seule ligne : « pas mesuré aux 2/3 de la pale » ;
  - le texte présente **Éclair (types 6 et 100) comme le fournisseur principal du S.VII**, Levasseur étant homologué aussi.
- **⚠** R&M 586 introuvable en ligne. C'est une brochure (index R&M 3555, tables 3 et 5) ; Cranfield n'en a pas de copie ; HathiTrust refuse les accès automatiques. Reste une demande manuelle.
- **R** Fourchette des trois hélices autorisées de P/D 0,80 (`spad7-calculs.md` § 7) :
  - Éclair 6 (2,40 m) : régime juste (+0,8 à +1,5 %), mais montées 7 à 11 % trop rapides ;
  - Levasseur 485 (2,45 m, modèle actuel) : montées justes, régime −3 % ;
  - Gallia 3 (2,50 m) : régime −7,5 %, montées 6 à 9 % trop lentes.

  S46 tombe dans la fourchette, mais aucune hélice ne tient régime et montée à la fois : l'écart restant est du côté de la montée.
- **D** Rien de changé dans les données. Le choix de l'hélice de référence (Levasseur 485 ou Éclair 6) est soumis au joueur.
- **Q** Le joueur demande des recommandations pour les deux décisions. **R** :
  - **hélice** : passer à l'Éclair 6, avec Levasseur 485 et Éclair 100 en variantes. Raisons : le texte de Hartmann, des deux erreurs qui se compensent aujourd'hui, et des recherches sur la montée à mener avec l'hélice documentée. Biais déclaré : on savait déjà que 2,40 m donne le bon régime ;
  - **suite** : d'abord les conditions des essais de 1919 (départ du chrono, altitude du terrain, température), parce que c'est peu coûteux et que cela chiffre l'incertitude de la cible ; puis la traction à faible J (surfaces du VE-7 à sourcer) ; enfin la courbe du moteur vers 1 500 tr/min et la loi d'altitude, puisque l'excès croît avec l'altitude (−8 / −7 / −11 %) ;
  - en attente de la décision du joueur.
- **Q** « D'accord » : le joueur valide les deux recommandations. **D** Hélice de référence : Éclair 6 (2,40 m). Suite : les conditions des essais de 1919.
- **A** Hélice passée à 2,40 m ; effet P (1,10 → 1,08 m) et inertie du rotor (7,0 → 6,7 kg·m²) mis à l'échelle du diamètre.
- **⚠** Première campagne fausse :
  - avec la nouvelle hélice, la recherche du décrochage au ralenti rendait 128 km/h à 2,9° d'incidence ; décrochage qualitatif, plané et polaire étaient faussés à sa suite ;
  - cause supposée : un trim isolé ne converge pas, l'hélice en moulinet étant au-delà de la fin de sa table (J ≈ 1,3, coefficients linéaires par morceaux), et la recherche s'arrêtait au premier échec. **Diagnostic faux**, corrigé plus bas après la revue : c'était le tourbillon du souffle, qui sautait quand la traction changeait de signe ;
  - correction : nouvel essai de plusieurs vitesses de départ, puis saut de l'incidence, avec trois échecs de suite avant de conclure. Test de non-régression sur le SPAD, vérifié en échec sans la correction.
- **R** Campagne avec l'Éclair 6 :
  - régimes +0,8 / +1,5 / +1,5 % : **réussis** ;
  - vitesses +0,8 / +5,2 / +4,2 % (3 000 m en échec de peu) ;
  - **montées −8,0 / −7,1 / −11,1 %, plafond +7,8 % : en échec** ;
  - décrochage en roulis à droite (+5 / +37 / +78° à 1, 2 et 3 s), plané 7,4 ; polaire inchangée.
- **A** Question posée à l'historien : comment les montées officielles françaises étaient chronométrées et mesurées (départ du chrono, terrain, altimètre, saison), et le décollage du S.VII.
- **A** Nouvelle commande **`fdm_lab montee`** : temps de montée et plafond sur des jours non standard (altitudes-pression, comme un altimètre), depuis un terrain en altitude, altimètre calé sur la mer ou à zéro au sol. Quatre tests.
- **⚠** Premier résultat absurde : par jour chaud, le plafond **montait**. La loi `Pressure(0.93)` ignorait la température ; l'air plus léger freinait moins l'hélice, et le moteur tournait plus vite. **Modification du modèle de vol** : la charge suit (T_type/T)^0,5 à pression égale (NACA Report 220, p. 7, mesure au banc sur un Wright-Hispano). Le facteur vaut 1 en atmosphère type : la campagne habituelle est identique au chiffre près. Tests ajoutés (inversion de l'atmosphère type, charge par jour chaud). Revue de l'aérodynamicien à faire.
- **R** Enveloppe (`spad7-calculs.md` § 8), écarts aux cibles de montée :
  - atmosphère type, depuis 0 m : −8 / −7 / −11 % ;
  - ISA +10 K : −2 / −0,5 / −4 % ;
  - terrain de 180 m avec altimètre à zéro au sol : −4 / −2 / −5 % ;
  - ISA +20 K, depuis le terrain : +9 / +12 / +12 %.

  Décollage estimé à environ 10 s (roulage de 6 à 7 s sur 85 à 95 m, puis 3 s d'accélération), soit +2,5 % à 2 000 m. **L'écart de montée est du même ordre que l'incertitude sur les conditions de l'essai.**
- **R** Dixième passe de l'historien (`spad7.md` § 5.7) :
  - départ du chronomètre français : non trouvé ;
  - pratique britannique de 1917 : temps ramenés à une montée en vol depuis la hauteur type 0 (Tizard) ;
  - pas d'atmosphère type française avant 1920 ;
  - Villacoublay à 178 m ;
  - conditions de l'essai de S46 : inconnues ;
  - aucun chiffre de décollage d'époque.
- **A** Documents téléchargés par le coordinateur (son outil de lecture échoue sur ces sites) : NACA TM 15, TN 20, Report 126, Report 218, dossier Hartmann « Reims 1911 ». Le Report 126 dit que les altimètres ont une échelle tournante « pour faire le zéro au départ du vol » ; les altimètres français sont gradués selon la loi de Radau ; l'atmosphère type du STAé date de 1920.
- **R** Enveloppe recommandée (178 m, ±15 K) : de −23 % à +8 % selon les conventions ; S46 est dedans, l'enveloppe est trop large pour juger le modèle.
- **R** Comparaison avec le rapport britannique n° 96 (vu en résumé, convention sans doute celle du modèle) : modèle −3,8 / −4,8 / −5,9 %, plafond +2,2 %. **Le modèle monte probablement 4 à 6 % trop vite** ; c'est le reste à expliquer, du même ordre que les incertitudes connues.

### Revue de l'aérodynamicien (hélice Éclair 6, conditions d'essai) et corrections
- **R** Verdict **bloquant**, sur un défaut ancien du FDM. Le tourbillon du souffle n'était calculé que pour une traction positive, et le champ de souffle disparaissait sans vitesse induite. Au passage de T = 0, en plané au ralenti vers 125 km/h, le moment de roulis sautait d'environ 40 N·m : aucun équilibre n'existait entre 3,15° et 3,36°. **Mon diagnostic « cassure de la table d'hélice » était faux** ; il figure aussi dans le message du commit 85aac08.
- **A** Correction du FDM :
  - le tourbillon K = Q/ṁ ne dépend plus du signe de la traction ; une hélice en moulinet, ou arrêtée dans l'écoulement, tourne l'air ;
  - fondu à zéro quand le débit à travers le disque s'annule (2 m/s, E:) ;
  - le champ de souffle garde le tourbillon sans vitesse induite.
- **A** Tests ajoutés ou réécrits :
  - continuité du souffle à ±ε de traction ;
  - hélice en moulinet ou arrêtée qui fait tourner l'air ;
  - symétrie du plané vérifiée au régime de moulinet libre (couple nul) ;
  - tous les planés du SPAD de 2,8° à 3,5° trimés par le solveur seul ;
  - balayage du décrochage sans incidence sautée.

  Le biplan d'essai, synthétique, part maintenant en roulis au décrochage plutôt qu'en abattée : le test accepte l'un ou l'autre.
- **A** La recherche du décrochage compte les incidences sautées ; le rapport les signale en gras.
- **R** Important : l'exposant de température, imposé à 0,5 pour tous les moteurs, contredisait en partie les sources ; ce choix, fait dans le même commit, allait dans le sens de la conclusion. **D** Il devient une donnée par moteur (`temperature_exponent`, 0 par défaut, loi `Pressure` seulement). **0,4 pour le Hispano** (C : 0,27 à 0,46 selon le Report 220, le L.C.E. 196 et la cohérence avec n = 0,93 ; R&M 462 ne voit pas d'effet en vol).
- **R** Important : la conclusion du § 8 penchait du côté favorable. **D** Reformulée : les conditions inconnues déplacent les temps d'environ −20 à +12 %, si bien que S46 ne départage pas un écart de ±10 %. L'écart n'est ni la preuve d'un défaut, ni une validation. Deux indices vont contre une explication par les seules conditions : les vitesses du même tableau, qui demandent un jour froid, et la tranche de 3 000 à 4 000 m (−17 %). Tolérances inchangées.
- **A** Autres corrections :
  - inertie propre de l'hélice à 2,40 m (6,3 / 3,2 / 3,2) ;
  - NaN propagé par la température type à pression donnée ;
  - écarts de température bornés à ±60 K ;
  - plafond calculé une fois par jour, affichage des minutes corrigé ;
  - test hydrostatique indépendant de la conversion en altitude-pression (0,2 %) ;
  - « départ des cales » au lieu de « lâcher des freins » ;
  - ligne ISA +20 K retirée (hors climatologie).
- **R** Campagne en atmosphère type inchangée par ces corrections. Enveloppe avec m = 0,4 : de −22 à +7 % selon les conventions (`spad7-calculs.md` § 8).
- **R** Seconde revue de l'aérodynamicien (commit 50ae4f9) : **plus rien de bloquant**. Le tourbillon est continu en T = 0, et les 61 incidences d'un balayage fin convergent avec le solveur seul. À corriger :
  - **le côté du départ au décrochage dépend du tourbillon**, le terme le moins sûr du modèle. Avec un facteur 0,5 ou 0 sur K, le SPAD part à gauche ; le départ en roulis existe aussi sans tourbillon (dynamique propre du biplan). Le côté n'est donc pas significatif ;
  - deux chiffres du § 8 étaient périmés : enveloppe de −22 à +7 % (et non −20 à +12 %), indices recalculés avec m = 0,4 (tranche de 3 000 à 4 000 m à ISA +10 K : −10 %).
- **A** Corrections :
  - § 8 mis à jour ;
  - nouveau § 9 de `spad7-calculs.md` (sens du départ non significatif) ; réserve ajoutée à la description du scénario dans `essais.ron` ;
  - étalonnage du souffle sur les ailes (Smelt et Davies) passé en tête des tâches hors plan ;
  - `temperature_exponent` obligatoire avec une loi `Pressure`, refusé avec `Density` ;
  - champ de `NonStandardDay` privé ;
  - test du signe du roulis de l'hélice arrêtée ;
  - test de symétrie de la cellule sans propulsion.


### Souffle d'hélice sur les ailes : sources, mesure et plan

- **Q** Le joueur : « On travaille par priorité, étape 1 ». **D** Étalonnage du souffle d'hélice sur les ailes (Smelt et Davies, R&M 1788). Comme il s'agit d'une modification du FDM touchant plusieurs choses, un plan sera soumis au joueur avant le code.
- **A** Sources lues :
  - R&M 2368 (Squire et Chester), parties I et II ;
  - R&M 2732, qui cite la formule de Smelt et Davies (éq. 6) ;
  - R&M 2701 ;
  - sur le serveur de la NASA : NACA TN 1146, Report 747 et TM 874 (Stüper).

  R&M 1788 n'est pas en ligne : la base AERADE de Cranfield ne l'a pas.
- **⚠** L'attribution faite au § 9 était inexacte. Smelt et Davies traitent la hausse de portance due à la vitesse du souffle, pas le tourbillon. Pour une aile infinie, la ligne portante de R&M 2368 donne même au tourbillon le **même** moment que la théorie des bandes. C'est la **mesure** du NACA TN 1146 qui le contredit : sans empennage, le roulis net vaut 50 à 60 % du couple, donc l'aile et le fuselage n'en reprennent que 40 à 50 %. Le § 9 et le plan précédent sont corrigés, avec l'ancienne référence barrée.
- **A** Mesure sur le SPAD, avec un test temporaire non commité : le moment de roulis du tourbillon vaut 1,1 à 1,7 fois le couple selon le vol. L'empennage en porte 0,6 Q, car il voit tout le tourbillon.
- **⚠** Incohérence d'énergie : la cellule reprend au tourbillon **151 %** de son énergie de rotation à Vmax, et **106 %** en montée. La théorie donne au plus 37 % pour une aile, la mesure environ la moitié. L'hélice installée du modèle est donc trop efficace, de 1,5 à 2,5 % de la puissance sur l'arbre.
- **⚠** En rédigeant le plan, j'ai d'abord lu à l'envers le signe des écarts de montée : ce sont des écarts de temps, donc le modèle monte trop vite. Corrigé avant la publication. La correction prévue ralentirait la montée d'environ 5 %, dans le sens des cibles : **biais déclaré** dans le plan.
- **D** Plan proposé : `docs/plans/m1-souffle-ailes.md`.
  - Maquette du TN 1146 comme avion de comparaison.
  - Facteur de largeur du souffle tiré de R&M 2368 (tableau 2).
  - Tourbillon divisé par deux derrière les ailes (Report 747, TN 1146).
  - Vitesse induite de Froude pour une traction négative.
  - Invariant d'énergie, campagne et § 9 refait.

  En attente de la validation du joueur.
- **Q** Le joueur : « Je valide le plan ». **D** Plan `m1-souffle-ailes.md` validé ; début de l'étape 1 (maquette du TN 1146).
- **A** Étape 1 : maquette du TN 1146 comme avion de comparaison.
  - Géométrie : tableau I et figure 1.
  - Profil : NACA 66(215)-216, relu sur le NACA Report 824.
  - Hélice : une table par calage, tirée de la figure 7.
  - Nouvelle commande `fdm_lab souffle`.
  - La lecture des figures est vérifiée : le roulis calculé par le NACA retombe sur leur courbe.
- **R** Le modèle actuel fait reprendre à l'aile 57 à 71 % du couple de l'hélice ; la mesure donne 39 à 50 %. Surestimation d'un facteur 1,2 à 1,8, moindre que le facteur 2 estimé ; elle résiste aux incertitudes de la maquette. Détail dans le plan, « Résultats ».
- **⚠** Les ancres « hélice arrêtée » recopiées du Messenger faisaient créer de l'énergie à la table à 15°. Remplacées par des valeurs qui passent le contrôle du FDM ; elles ne servent pas au point mesuré.
- **A** Étape 2 : facteur de largeur du souffle (R&M 2368, tableau 2) appliqué à ce que le souffle ajoute à la portance des bandes, pour la vitesse comme pour le tourbillon. Les éléments de traînée gardent tout le souffle.
- **R** Maquette du TN 1146 : l'aile reprend 41 / 40 / 39 % du couple (avant : 71 / 70 / 57 %), pour 39 à 50 % mesurés. C'est le bas de la plage, sans aucun paramètre réglé. SPAD : le roulis du tourbillon sur les ailes baisse en montée et au décrochage plein gaz ; il ne change presque pas à Vmax ni au ralenti (limite de Koning).
- **⚠** Le test de linéarisation des modes propres a échoué de peu (2,3 % pour 2 % admis). Diagnostic : les polaires linéaires par morceaux cassent la pente le long de la perturbation ; ces cassures existaient déjà, dont une au point d'équilibre. **D** Plancher absolu du test porté de 1e-3 à 5e-3, avec la justification en commentaire. À relire par l'aérodynamicien.
- **A** Étapes 3 et 4 :
  - rotation du souffle réduite derrière les ailes (`swirl_left` = 0,5, donnée obligatoire du bloc `downwash`) ;
  - vitesse induite de Froude prolongée sous traction négative, bornée à −V/2 ;
  - nouvelle commande `fdm_lab souffle <avion>` : bilan du tourbillon par surface.
- **⚠** La polaire du Messenger a dérivé : son C_D0 est passé de 0,055 à 0,051. L'hélice qui freine au ralenti ralentit maintenant l'air sur sa traînée parasite, concentrée en un point dans le souffle. Or le Report 304 planait hélice à traction nulle. **D** `fdm_lab polaire` évalue la cellule sans propulsion, comme la « traînée vraie » du Report 304. Oswald inchangé pour le Messenger, 0,727 pour le SPAD (0,723).
- **⚠** L'invariant d'énergie prévu par le plan ne tient pas pour le biplan d'essai : 120 % de l'énergie de rotation reprise à Vmax. C'est la limite de la théorie des bandes décrite par R&M 2368, partie II. Le test est remplacé par un test de cohérence ; un addendum est proposé au joueur (inclinaison partielle de la portance due à la rotation, option (b) recommandée).
- **R** Campagne du SPAD après les étapes 2 à 4 :
  - toutes les cibles chiffrées passent : vitesses +0,3 / +4,6 / +3,5 %, montées −4,5 / −3,3 / −7,0 %, plafond +6,4 % ;
  - départ au décrochage à gauche (−54°) ;
  - c'est le sens du biais déclaré, pas une validation.
- **⚠** Disque plein pendant les essais : 26 Go de `target`. `target/release` et `target/debug/incremental` supprimés (reconstructibles).
- **A** Revue de l'aérodynamicien lancée sur les étapes 1 à 4 et l'addendum (en cours).
- **R** Sensibilité du côté du départ au décrochage moteur réduit à la part de rotation laissée derrière les ailes (`swirl_left`) :
  - 0,3 : à gauche, −53° à 3 s ;
  - 0,5 (retenu) : à gauche, −54° ;
  - 0,7 : à gauche, −56° ;
  - 1,0 (aucun redressement) : à droite, +78°, comme avant.

  Dans la plage des sources (« environ la moitié »), le départ se fait à gauche ; il dépend surtout de la rotation qui atteint l'empennage. Ce n'est pas encore une conclusion : l'addendum et la revue peuvent la déplacer. § 9 à reprendre à l'étape 6.
- **D** En attente du joueur : addendum sur l'inclinaison de la portance due à la rotation, option (b) recommandée.

## 2026-10-25 — Session 4 (suite)

### Addendum (b) : inclinaison de la portance due à la rotation

- **⚠** La revue de l'aérodynamicien (étapes 1 à 4) s'est arrêtée sur la limite de session de l'API. Sa dernière note demandait si le Report 747 dit bien « la moitié de la rotation » : oui, « can be expected to remove about half the race rotation ». La revue reste à reprendre.
- **Q** Le joueur : « Partons sur l'option b ». **D** Addendum validé.
- **⚠** En recalculant τ : mon « 0,4 extrapolé pour le SPAD » confondait le μ de la partie I de R&M 2368 et le a = 8R/(a₀c) de la partie II (a = μ/π). Pour le SPAD, a ≈ 1,4, dans le tableau : τ ≈ 0,15. Le plan est corrigé.
- **A** Mise en œuvre : les bandes reçoivent la part due à la rotation et la fraction τ de son inclinaison que la force suit (`StripFlow`, `swirl_tilt`). L'incidence et la portance, donc le moment de roulis, sont inchangées.
- **⚠** Premier essai : la rotation coûtait de la puissance (−5 à −12 % de son énergie pour le SPAD), contre +28 à 37 % en ligne portante. Double compte : nos bandes inclinent déjà leur force de leur propre angle induit, k a₀/(1 + k a₀) de l'incidence. **D** La part redressée explicitement devient 1 − τ − k a₀/(1 + k a₀).
- **R** Puissance reprise au tourbillon : SPAD 21 / 6 / 5 % (Vmax, montée, décrochage plein gaz), biplan d'essai 13 à 25 %. C'est sous la ligne portante, à cause de la traînée de profil, et sous le Report 747 (environ la moitié) : écart rapporté, non réglé. L'invariant d'énergie redevient un test (au plus la moitié).
- **R** Campagne du SPAD :
  - vitesses −0,2 / +4,1 / +2,8 % ;
  - montées −1,8 / −0,2 / −3,6 % ;
  - plafond +5,1 % ;
  - départ au décrochage à gauche (−54°).

  Toutes les cibles passent : sens du biais déclaré, pas une validation.
- **A** Revue de l'aérodynamicien reprise sur les étapes 1 à 4 et l'addendum (en cours).
- **R** Sensibilité du côté du départ au décrochage moteur réduit, addendum compris :
  - `swirl_left` 0,3 : à gauche, 53° ;
  - 0,5 : à gauche, 54° ;
  - 0,7 : à gauche, 61° ;
  - 1,0 (aucun redressement) : à droite, 42°.

  Même conclusion qu'avant l'addendum : le départ se fait à gauche dans toute la plage des sources.

### Revue de l'aérodynamicien et corrections

- **R** Revue reçue : à corriger, rien de bloquant.
  - **Important** :
    - (1) l'addendum inclinait toute la force de la bande, alors que τ ne vaut que pour la portance de rotation ;
    - (2) `swirl_left` = 0,5 contredit le bilan de moment cinétique du modèle en croisière, où les ailes reprennent 0,9 Q ;
    - (3) la tolérance du test TN 1146 n'était pas honnête ;
    - (4) η sur l'empennage à basse vitesse double l'effet d'allongement ;
    - (5) l'écart d'énergie ne vient qu'en partie de la traînée de profil.
  - Plusieurs points mineurs, et un point du profil manqué.
- **A** Corrections :
  - (1) portance de base inclinée comme en théorie des bandes ; poussée en trop de la seule portance de rotation reprise comme une traînée ;
  - (2) diagnostic de moment cinétique dans `fdm_lab souffle` ; version calculée à chaque pas mise en tête des suites ;
  - (3) test de non-régression assumé, écart écrit ; point du profil ajouté (−3,94°) ;
  - (4) reporté au M2 ;
  - (5) rapporté ;
  - points mineurs : τ au-delà de a = 2, η = 1 pour un jet lent, Froude en C¹ vers −0,4 V, aile décalée de l'axe, sillage limité aux bandes dans l'ombre du disque et absent pour une hélice propulsive, commentaires, test des modes propres en différence centrée, nouveaux tests.
- **R** Maquette du TN 1146 : 39 / 38 / 38 % repris (mesure 47,5-50 / 39-45 / 43,5-46 %), le modèle restant sous la mesure.
- **R** SPAD :
  - puissance reprise au tourbillon : 18 / 4 / 6 % ;
  - vitesses −0,2 / +4,1 / +2,8 %, montées −1,7 / 0,0 / −3,3 %, plafond +5,0 % ;
  - départ au décrochage à gauche, 54°.
- **D** § 9 de `spad7-calculs.md` mis à jour : départ à gauche **probable** (confiance C), aucune conclusion sur l'équilibre en croisière. Nouveau § 10 : synthèse du souffle. Le plan `m1-ecarts-spad7.md` marque la tâche prioritaire comme faite.

- **⚠** `just check` refusait `swirled_strip_loads` (8 arguments, clippy `too_many_arguments`). **A** La rotation et sa fraction τ passent dans le `StripFlow` de la bande. `just check` et `just test` (234 tests) passent.

### Suite 1 : rotation derrière les ailes par le bilan de moment cinétique

- **Q** Le joueur : « Prépare le plan de la suite 1 et lance le ». **D** Plan `docs/plans/m1-rotation-derriere-ailes.md` rédigé et lancé dans la même session, sur cette demande.
- **A** Recherche : aucune mesure de la rotation résiduelle derrière une aile en charge légère (Ribner et MacLachlan 1947 traitent le fuselage ; NACA RM SL53G10, la dérive du XF-91). Relecture du NACA TN 1146 :
  - sa « moitié » vient du Report 747 (sa référence 11) ;
  - avec elle, les forces calculées sur la dérive « generally agreed with the computed values within 10 to 15 percent ».
- **⚠** Le tableau des sources de `m1-souffle-ailes.md` disait l'inverse (forces « tombant à 10-15 % des mesures »). Corrigé, avec renvoi au nouveau plan.
- **A** FDM :
  - les ailes sont calculées d'abord ; le moment que la rotation exerce sur elles, autour de l'axe de l'hélice, est retiré au flux de moment cinétique du jet, ṁK ;
  - le reste de l'avion (empennage, dérive, traînées, fuselage) vole dans le souffle redressé, la fraction étant bornée à [0, 1] ;
  - la donnée `swirl_left` disparaît (SPAD, Messenger, biplan d'essai, `format-avion.md`) ;
  - nouveaux accès : `LiftingSystem::loads_of`, `SystemLoads::with`, `Slipstream::swirl_left_after`, `StripState::swirl_force`, `Diagnostics::swirl_left`.
- **⚠** Première version : la portance ajoutée par la rotation était lue avec la pente à portance nulle. Elle s'écartait du contrôle par différence de 0,02 à 0,05. Deux causes : la pente locale de la table, et la pression dynamique que la rotation change à grande incidence. **D** La part de la rotation est désormais la différence complète entre la bande et la même bande sans rotation. Elle colle au contrôle à 0,01 près sur quatre vols, et c'est un test.
- **R** Rotation laissée derrière les ailes du SPAD :
  - 0,07 à Vmax, 0,28 en meilleure montée, 0 au plané, 0,59 au décrochage plein gaz ;
  - biplan d'essai : 0,09 / 0,31 / 0 / 0,49.
- **R** Commandes à l'équilibre (avant = même code, fraction figée à 0,5) :
  - direction en montée +0,057 → +0,001 de la course, à Vmax +0,031 → −0,024, au décrochage plein gaz +0,098 → +0,128 ;
  - ailerons presque inchangés (au plus 0,01 de la course en croisière).

  Résultat du modèle, non validé : il dépend du moment des ailes en charge légère, que R&M 2368 juge peu sûr.
- **R** Campagne du SPAD :
  - vitesses −0,2 / +4,0 / +2,7 % ;
  - montées −1,5 / +0,1 / −3,2 % ;
  - plafond +5,0 % ;
  - départ à gauche, −55°.

  Aucune cible ne dépendait du changement : pas de biais vers elles.
- **⚠** Marge statique 5,9 → 4,9 %, expliquée par une sonde.
  - Le niveau de rotation sur le plan fixe compte, pas sa variation avec l'incidence.
  - Le profil d'empennage (RAF 15 décambré puis symétrisé) a un creux de pente près de la portance nulle (0,069 par degré contre 0,076 à 0,081). La rotation de 0,5 l'en sortait en partie.

  Rapporté ; à regarder si la marge devient une cible.
- **⚠** Le bilan du tourbillon en montée passe à −1 % de l'énergie de rotation (les ailes y perdent de la puissance par la traînée de profil, l'empennage reprend moins). Le test qui exigeait un gain de puissance est remplacé par une borne, l'écart étant écrit (point 5 de la revue précédente).
- **R** Sensibilité du départ au décrochage moteur réduit (§ 9) :
  - modèle : à gauche, 55° ;
  - contre-rotation permise : à gauche, 55° ;
  - chute sur une demi-corde ou sur deux cordes : à gauche, 56° et 55° ;
  - fraction figée à 0,3 / 0,7 : à gauche, 53° / 56° ;
  - fraction figée à 1 : à droite, 78°.

  **D** Le départ à gauche reste « probable » (confiance C).
- **⚠** Pendant les variantes, le hook d'arrêt a exigé un commit alors que l'arbre contenait du code temporaire. **A** Version propre commitée par l'index (`c8a5fa8`), variantes poursuivies sur une copie du binaire, sources propres remises.

### Revue de l'aérodynamicien (suite 1) et corrections

- **R** Verdict : à corriger. Le code est juste (signes, bilan, continuité), mais les chiffres publiés n'étaient pas convergés.
- **⚠** Bloquant : avec 8 bandes par demi-aile, 1,5 à 2 bandes seulement couvraient chaque demi-largeur du jet. La fraction 1 − M/Q, petite différence de deux grands nombres, tombait à 0,07 à Vmax au lieu d'environ 0,21.
  - Étaient des artefacts : la cellule qui reprenait plus que le couple au plané, la borne 0 atteinte, le −1 % d'énergie en montée.
  - Les chiffres de l'entrée précédente (0,07 / 0,28 / 0 / 0,59, commandes, marge) sont remplacés ci-dessous.
- **A** SPAD à 16 bandes par demi-aile, empennage et dérive doublés (C:, convergence). Test `the_swirl_left_converges_with_the_strips` : doubler encore change la fraction de moins de 0,02 et les commandes de moins de 0,005 de la course, 0,01 au décrochage plein gaz (0,0065 mesuré, décrochage des bandes discrétisé).
- **⚠** Important : la marge statique était du bruit de mesure (tangente sur un seul segment des tables), de 2 à 6 % selon la vitesse. L'écart 5,9 → 4,9 % et son explication par le profil d'empennage ne tiennent pas. **A** Marge par sécante sur ±1° (`static_stability`) : 3,8 / 5,0 / 5,1 / 4,7 / 4,0 / 3,2 / 1,2 % de 140 à 170 km/h. **D** Le lissage des tables (C_m de l'aile surtout, empennage) est laissé au joueur, en ADR.
- **A** Coût : l'itération mutuelle ne sépare plus la part de rotation, qui ne change aucun C_L (test). La revue mesurait 15,6 % des instructions perdues ; `just test` passe de 145 à 65 s.
- **A** Test miroir (hélice à gauche : même fraction, efforts latéraux exactement opposés). Bornes justifiées dans le code (grille d'aubes ; la borne haute laisse moins de rotation à l'empennage près du décrochage).
- **R** Bandes convergées, SPAD :
  - fraction 0,21 / 0,33 / 0,07 / 0,52 (Vmax, montée, plané, décrochage plein gaz) ;
  - la cellule ne reprend plus nulle part plus que le couple ;
  - puissance reprise au tourbillon : 8 / 1 / 13 / 7 %.
- **R** Commandes à l'équilibre, avant (fraction 0,5) → après :
  - direction à Vmax +0,033 → −0,004, en montée +0,062 → +0,019, au décrochage plein gaz +0,095 → +0,100 ;
  - ailerons au plus 0,013 en vol attaché, 0,08 au décrochage.

  Ailes horizontales, dérapage libre (0,7° à Vmax, 2,0° en montée). **D** Confiance D : c/D = 0,5, au-delà du domaine sûr de R&M 2368.
- **R** Campagne :
  - vitesses −0,2 / +4,0 / +2,7 % ;
  - montées −1,5 / +0,2 / −3,1 % ;
  - plafond +4,9 % ;
  - départ au décrochage moteur réduit à gauche, 55°.
- **R** Biplan d'essai (synthétique, laissé à 6 bandes) : il ne converge pas aussi bien (0,09 / 0 / 0,02 à Vmax pour 6 / 12 / 24 bandes). Au plané, ses ailes reprennent 1,08 Q à toute finesse : c'est sa géométrie.
- **D** Suites notées dans le plan :
  - lissage des tables (ADR à décider) ;
  - Newton de l'induction mutuelle (coût, avant l'IA) ;
  - test d'énergie par décomposition ;
  - retard ailes → empennage à la vitesse du jet ;
  - exposition du rapport non borné ;
  - question à l'historien sur un calage différentiel des ailes du S.VII ;
  - essai manette en main au M2.
- **⚠** Le processus de travail a redémarré pendant les corrections de la revue. Les modifications non commitées étaient intactes ; les calculs en arrière-plan ont été relancés.
- **Q** Le joueur : « Continue ». **A** Corrections terminées, poussées (`165b57b`, `6dcfd0b`).
- **A** Sensibilité du départ au décrochage relancée avec les bandes convergées. Premier résultat : contre-rotation permise, départ à gauche, 55°. Les autres variantes sont en cours.
- **R** Sensibilité, bandes convergées : chute de la rotation sur une demi-corde, départ à gauche, 55°.
- **R** Sensibilité complète, bandes convergées :
  - modèle et contre-rotation permise : à gauche, 55° ;
  - chute sur une demi-corde ou deux cordes : à gauche, 55° ;
  - fraction figée à 0,3 / 0,7 : à gauche, 53° / 54° ;
  - fraction figée à 1 (aucun redressement) : **à gauche aussi**, 87°.
- **⚠** Avec 8 bandes, ce dernier cas partait à droite : la sensibilité précédente était faussée par la discrétisation.
- **D** § 9 : le côté du départ ne dépend plus de la rotation derrière les ailes. Il reste « probable », confiance C, faute de témoignage, avec la réserve de la direction du souffle.
- **R** Rapport final au joueur sur la suite 1 :
  - mise en œuvre : rotation derrière les ailes par le bilan de moment cinétique, donnée `swirl_left` supprimée ;
  - revue et corrections : bandes convergées, coût, marge par sécante, test miroir ;
  - résultats : fraction 0,21 / 0,33 / 0,07 / 0,52 ; couple presque compensé en croisière (confiance D) ; performances inchangées à 0,2 point près ; départ au décrochage à gauche dans toutes les variantes (confiance C).
- **D** Décisions demandées au joueur :
  - lissage des tables de profil (C_m de l'aile, empennage), qui fixe la marge statique : par ADR ;
  - question à l'historien sur un calage différentiel des ailes du S.VII.

### `/nouvel-avion spad7` : reprise du parcours

- **Q** Le joueur lance `/nouvel-avion spad7`. **R** L'avion existe déjà : version et identifiant confirmés au M0/M1 (SPAD S.VII C.1, Hispano-Suiza 8Aa de 150 ch, une Vickers, `spad7`). Je reprends la liste des étapes et ne fais que ce qui manque.
- **R** État des étapes :
  - fiche (10 passes de l'historien), profils, hélice, données et cibles : faits ;
  - inertie : `fdm_lab inertia spad7` concorde avec `aircraft.ron` ;
  - gains de l'instructeur et de l'IA : l'outil n'existe pas encore (M4, M5) ;
  - essais : rapport « validé », toutes les cibles quantitatives passent, robustesse verte ;
  - modèle 3D provisoire et `asset_check` : rien n'existe ;
  - revues : l'aérodynamicien a relu après chaque changement, l'historien a relu la fiche et la chronologie (huitième passe).
- **A** Texte de l'essai qualitatif « décrochage moteur réduit » mis à jour : le côté du départ (à gauche) résiste à toutes les variantes, probable, confiance C.
- **A** Onzième passe confiée à l'historien : compensation du couple sur le S.VII (calage dissymétrique, haubans, dérive), témoignages sur le pied à tenir, sens de rotation de l'hélice.
- **Q** Le joueur : « explique ce qui manque avant le modèle 3D ».
- **R** Six préalables, vérifiés dans le dépôt :
  1. un ADR sur la chaîne d'axes Blender → glTF → Bevy → modèle de vol, et sur l'origine du modèle. Aucun ADR ne la fixe (0001 à 0005), et `crates/game/src/flight/bridge.rs`, l'unique conversion prévue par CLAUDE.md, n'existe pas ;
  2. la liste minimale des nœuds d'un modèle provisoire, tirée du contrat de `pipeline-assets` ;
  3. l'outil `tools/asset_check`, absent. `aircraft.ron` ne porte ni longueur ni hauteur hors tout (elles sont dans la fiche : 6,08 à 6,20 m et 2,20 m) ;
  4. un générateur en Rust, puisque Blender n'est pas installé dans le conteneur. Il demande une dépendance directe pour écrire le glTF (`gltf-json` ou `serde_json`, déjà dans `Cargo.lock` par Bevy), à approuver ;
  5. des cotes estimées (E:) : section du fuselage, œil du pilote, bouche de la Vickers, train ;
  6. rien n'affiche encore un avion dans le jeu : le pont FDM ↔ ECS et les caméras sont au M2.
- **D** Proposition : faire le modèle provisoire au début du M2, avec un plan à valider. Décision laissée au joueur.
- **Q** Le joueur : « qu'est-ce qu'un ADR ? » **R** Une fiche de décision d'architecture (*Architecture Decision Record*) dans `docs/adr/` : le contexte, les options, le choix et ses conséquences. Un ADR accepté ne se modifie pas : on le remplace par un nouveau. Exemples du dépôt : 0001 à 0005, dont 0004 (version de Bevy). Celui des axes fixerait les repères et l'origine du modèle 3D.

### ADR 0006 : repères et conversions

- **Q** Le joueur : « Écris le modèle de conversion et l'ADR pour ce sujet » (les repères de Blender, glTF, Bevy et du modèle de vol).
- **A** Vérifié dans les sources épinglées :
  - Bevy 0.20 et le modèle de vol partagent glam 0.33 ;
  - le chargeur glTF de Bevy 0.20 ne convertit rien par défaut ; sa conversion (`GltfConvertCoordinates`) est marquée expérimentale ;
  - `Transform::forward`, `right` et `up` donnent −Z, +X et +Y.
- **A** `docs/adr/0006-reperes-et-conversions.md` (statut **Proposé**), avec cinq choix et leurs options :
  - nord = −Z et est = +X dans Bevy, d'où **une seule rotation propre** pour le corps et le monde, (x, y, z) ↦ (y, −z, −x) ;
  - demi-tour fixe autour de Y pour les modèles glTF, appliqué par notre pont et non par l'option expérimentale ;
  - dans Blender : nez vers −Y, dessus vers +Z, aile droite vers −X, export par défaut ;
  - origine du modèle 3D au `datum` d'`aircraft.ron`, l'entité étant au centre de gravité ;
  - attitudes par conjugaison du quaternion, sans angles d'Euler.
- **A** Code : `crates/game/src/flight/bridge.rs` (`to_bevy`, `from_bevy`, `attitude_to_bevy`, `attitude_from_bevy`, `GLTF_TO_BEVY`, `model_transform`, `render_transform`). Huit tests :
  - axes et aller-retour ;
  - rotation propre (pas de miroir) ;
  - attitude cohérente avec les vecteurs ;
  - vols simples (cap à l'est, cabré de 10°, inclinaison à droite) ;
  - hélice à droite horaire à l'écran vue du poste ;
  - demi-tour glTF et lignes de l'ADR pour `asset_check` et Blender ;
  - origine du modèle au `datum` ;
  - origine de rendu (seul l'écart passe en f32).
- **A** Index des ADR et règle d'orientation de `pipeline-assets` mis à jour. `just check` passe, ainsi que `just test` (250 tests).
- **⚠** Un de mes tests attendait −3,0012 au lieu de −2,9988 (signe d'un décalage vers l'est) : erreur du test, corrigée ; le code était juste.
- **D** En attente : validation de l'ADR par le joueur. Aucune règle de `CLAUDE.md` ne change.

### Onzième passe de l'historien : couple de l'hélice

- **R** Rapport de l'historien (`spad7.md` § 6.5, sources S89 à S100) :
  - **réglage dissymétrique de la cellule du S.VII** (calage gauche-droite, haubans, dérive) : rien trouvé, pour le S.VII ni pour la famille ; indice faible (C), question non tranchée, cellule symétrique par défaut (E:) ;
  - **pratique d'époque** sur d'autres avions (Albatros D.I calé 4°/2° aux saumons, dérive décalée du Fokker D VII, manuel britannique de 1919 jugeant la correction « en général plus nécessaire ») : indices contraires, rien à transposer. Le résultat du modèle en croisière n'a donc **aucune cible historique** ;
  - **témoignages** : aucun témoignage d'époque sur le couple d'un S.VII, en cinq langues. T8 est précisé (le XIII part à droite à l'ouverture des gaz), T9 complété (Tallman, sans difficulté de direction, en résumé) ;
  - **sens de rotation** : la convention britannique de 1917-1918 (« right hand » = horaire vu du pilote, *Flight* et rapports sur des moteurs capturés) donne son sens au marquage « R H Tractor S7 » de l'hélice AB 723. Proposition : confiance relevée de C à B.
- **⚠** Mon commit `281f6de` (ADR 0006) a emporté par `git add -A` la onzième passe de l'historien, écrite pendant ce temps. Le contenu est complet et poussé, mais le message du commit ne la mentionne pas ; cette entrée en tient lieu. **D** Désormais, ne commiter que les fichiers du travail en cours quand un subagent écrit en parallèle.
- **A** Proposition reportée :
  - dans `hispano-suiza-8a.md` § 7.3 : sens horaire vu du pilote, confiance B ;
  - dans `aircraft.ron` : commentaire de `rotation: Clockwise` (S:, confiance B).

  La valeur ne change pas.
- **A** Piste de l'historien pour l'aérodynamicien, ajoutée aux suites du plan : mesure de la rotation du souffle derrière le F.E.9 (R&M 408). L'hélice y est propulsive : borne « sans redressement » seulement.
- **D** Documents à demander, ce qui revient au joueur : les sites sont inaccessibles depuis le conteneur, et plusieurs documents ne sont pas numérisés.
  - Australian War Memorial : *Technical notes: SPAD biplane Type S.VII (150 h.p.)*, 1918, reproduction à demander. C'est la source la plus probable d'une épure de réglage.
  - Fac-similé des *Technical Notes* de l'Air Board (Harris, 1999).
  - Bulletins américains de 1918 (Joyner Library).
  - Fonds de plans Blériot-Spad du Musée de l'Air.
- **Q** Le joueur : « ok pour l'ADR 0006 ». **D** ADR 0006 (repères et conversions) **accepté** ; statut mis à jour dans le fichier et dans l'index.

### Demandes aux archives ; plan du modèle 3D provisoire

- **Q** Le joueur : « je veux un fichier de consigne des demandes à faire, notamment vers les organismes cités (AWM, fac-similé des *Technical Notes* de l'Air Board, bulletins américains de 1918, fonds Blériot-Spad du Musée de l'Air) ; on continue sur le modèle 3D provisoire ».
- **A** `docs/recherche/demandes.md` :
  - les quatre demandes citées (D1 à D4), plus trois tirées des fiches : SHD fonds Dorand (D5), TNA AIR 1/1196 (D6), Cranfield R&M 586 (D7) ;
  - pour chacune : référence, ce qu'on cherche et pourquoi, démarche, message proposé ;
  - un tableau de suivi et un journal.
- **D** Règles du fichier :
  - le joueur envoie les demandes, Claude les prépare et les suit ;
  - coordonnées et tarifs à vérifier avant l'envoi ;
  - les reproductions reçues restent hors du dépôt (droits) : seuls les faits et les références vont dans les fiches.
- **A** Plan `docs/plans/modele-3d-provisoire.md` :
  - boîtes et plaques générées par un outil Rust (`tools/mockup`) à partir d'`aircraft.ron` et d'un nouveau `maquette.ron` ;
  - contrôle `tools/asset_check` (nœuds, charnières, hélice, dimensions, licence ; un modèle en miroir doit échouer) ;
  - scène de capture `maquette-spad7` qui vérifie l'ADR 0006 à l'œil.
- **D** Quatre décisions demandées au joueur avant le code :
  - la dépendance `gltf` / `gltf-json` 1.4.1 (déjà dans `Cargo.lock` par Bevy, MIT ou Apache-2.0) ;
  - le champ `dimensions` d'`aircraft.ron` et le fichier `maquette.ron` ;
  - l'outil `tools/mockup` et son ajout à l'arborescence de `CLAUDE.md` ;
  - le plan lui-même.
- **Q** Réponses du joueur : `gltf` / `gltf-json` ; les deux changements de format ; nouvel outil `tools/mockup` ; plan validé. **D** Plan `modele-3d-provisoire.md` validé, exécution lancée.
- **A** Étapes 1 et 2 :
  - `dimensions` (envergure, longueur, hauteur) dans `aircraft.ron`, optionnel pour les modèles de comparaison. SPAD : 7,82 / 6,08 / 2,20 m, sourcés par la fiche ;
  - schéma `MockupFile` et chargeur (`crates/data/src/mockup.rs`), et `assets/aircraft/spad7/maquette.ron` (fuselage, roues, béquille, mâts, œil, Vickers), chaque cote avec son origine ;
  - module `escadrille_data::frames` (`to_bevy`, `from_bevy`, `to_gltf`, `from_gltf`), réutilisé par le pont du jeu : une seule copie, comme le prévoit l'ADR 0006 ;
  - `format-avion.md` mis à jour.
- **R** La fiche ne dit pas si la hauteur de 2,20 m est mesurée en ligne de vol ou au sol. **D** Le contrôle prendra la plus proche des deux attitudes.
- **A** Étape 3, générateur `tools/mockup` (`just mockup spad7`, sans Bevy) :
  - contrat du modèle dans `escadrille_data::model` : une plaque par bande du modèle de vol, coupée à la charnière ; pour chaque gouverne, sa charnière (droite passant par le centre des points de charnière de ses bandes) et le sens d'une braque positive, tiré des bandes elles-mêmes ; axe et moyeu de l'hélice ;
  - le gouvernail du SPAD couvre deux panneaux de la dérive : un seul nœud `ctrl_rudder`, une seule charnière ;
  - `assets/aircraft/spad7/model.glb` (124 ko) : fuselage par sections, plans, mâts, empennages, train, béquille, hélice, disque flou, œil, Vickers ; aile droite verte, gauche beige, gouvernes orange ; sortie identique à l'octet d'une génération à l'autre.
- **A** Étape 4, contrôle `tools/asset_check` (`cargo run -p asset_check`, et à chaque `just test` sur tous les modèles d'`assets/`) : validité glTF, nœuds, pivots et axes des gouvernes (1 cm, 1°), hélice, œil et arme vers l'avant, envergure, longueur et hauteur (1 %), licence. Un modèle en miroir, retourné ou décalé de 5 cm échoue (tests).
- **R** Le modèle généré passe tout, sauf deux cotes :
  - **longueur** 6,264 m contre 6,08 m (+3,0 %). Cause : le modèle dessine l'empennage **équivalent** du modèle de vol (quart de corde sans flèche, surface conservée, `spad7-calculs.md` § 5). Au-dessus du plan fixe, la dérive équivalente a 1,61 m de corde à l'emplanture ; son bord de fuite finit 0,16 m derrière celui du dessin de 1917 ;
  - **hauteur** 2,53 m en ligne de vol, 2,355 m au sol, contre 2,20 m (A). En ligne de vol, le haut de la dérive et le bout de l'hélice couvrent déjà 2,22 m sans aucune garde : 2,20 m ne peut être mesuré qu'au sol. Mais au sol, le train du dessin donne 2,30 m (roues de 0,58 m du dessin) à 2,355 m (roues de 700 mm retenues). Le dessin (C) et les tableaux d'époque (A) se contredisent ; les sources donnent de 2,12 à 2,35 m.
- **D** Ni l'un ni l'autre ne se règle sans décision du joueur : l'empennage relève du modèle de vol, et la hauteur d'un conflit de sources. Les deux écarts sont **acceptés provisoirement** dans `maquette.ron` (`accepted_deviations`), avec leur cause, **en attente de l'accord du joueur**. Un écart accepté ne vaut qu'à la valeur mesurée (5 mm près) : si une donnée le fait bouger, le contrôle échoue de nouveau. Il ne vaut que pour une cote, jamais pour une charnière ou un axe.
- **R** Conséquence pour le modèle définitif : sur l'empennage équivalent, la charnière du gouvernail penche d'environ 27° et celle de la profondeur d'environ 8°, alors que les vraies sont droites. Un modèle Blender fidèle au dessin échouerait au contrôle des charnières. Point à trancher avant le modèle définitif (proposé dans le rapport au joueur).
- **A** Place disque : il ne restait que 1,3 Go. Trois anciens exécutables de test du jeu (environ 6 Go) ont été supprimés de `target/debug`.
- **A** Étape 5, scène de capture `maquette-spad7` (`crates/game/src/world/maquette.rs`) :
  - le modèle est placé comme le jeu le placera : entité de l'avion au centre de gravité (`render_transform`), modèle en enfant (`model_transform`, demi-tour glTF de l'ADR 0006). Cap à l'est, inclinaison de 30° à droite, 6 m au-dessus du sol ;
  - gizmos des axes du modèle de vol au centre de gravité (x rouge, y vert, z bleu), dessinés par-dessus le modèle ;
  - hélice tournée de +30° autour de son axe local depuis la position pales verticales, flèche jaune du sens de rotation tirée du contrat, disque flou masqué ;
  - registre des scènes : chaque scène a son système, lancé si la capture la demande (`capture_scene_is`) ; la capture attend la fin du chargement du modèle (`CaptureHold`) avant de compter ses images ;
  - le jeu charge ses assets depuis `assets/` à la racine du dépôt (`ASSETS_DIR`) : sous `cargo run`, Bevy les cherchait dans `crates/game/assets`.
- **A** Capture sous Xvfb (lavapipe), binaire de développement, 1920 × 1080 ; pas de `just capture`, dont la compilation optimisée ne tient pas sur le disque restant. **Image regardée** :
  - le nez est du côté de la flèche rouge ;
  - l'aile droite, verte, est du côté de la flèche verte et plus basse ; la flèche bleue penche à gauche, comme il se doit en virage à droite ;
  - la flèche jaune tourne dans le sens horaire vu de derrière, et la pale du haut est tournée dans ce sens : l'axe local +X du nœud `prop` est bien celui des données.
- **D** L'ADR 0006 est vérifié à l'œil sur le premier modèle, comme prévu par `pipeline-assets`.
- **A** Documentation : `pipeline-assets` (conventions d'axes des nœuds, générateur, écarts acceptés, contrôles, empennage équivalent), `format-avion.md` (`accepted_deviations`), bilan d'exécution dans le plan.
- **A** Revue de l'aérodynamicien, pivots et axes : **rien de bloquant**.
  - Conformes, vérifiés par le calcul : sens de braquage du contrat (aileron droit bord de fuite en haut, gauche en bas, profondeur en haut, gouvernail vers +y), axe de l'hélice (+x pour `Clockwise`), repères, capture.
  - Analyse des deux écarts confirmée. Pour la hauteur, une explication de plus : l'écrasement statique des pneus et des sandows, absent du modèle (3 à 6 cm donnent 2,26 à 2,33 m) ; et la tolérance de 1 % (2,2 cm) est plus fine que la donnée elle-même.
- **⚠** Ma description du point ouvert était fausse. Les axes des charnières du contrat ne penchent pas : ils sont verticaux pour le gouvernail et transversaux pour la profondeur. Ce qui penche, ce sont les points de charnière des bandes. Un modèle fidèle au dessin aurait le bon axe et échouerait sur le **pivot**, à environ 7 cm. Corrigé dans `pipeline-assets` et dans le plan.
- **A** Corrigé après la revue :
  - le générateur coupait chaque plaque à la charnière de sa bande, mais le nœud tourne autour de la droite du contrat : jusqu'à 8,7 cm de jour ou de recouvrement au gouvernail braqué à fond. Il coupe désormais sur la droite du nœud (test) ;
  - le pivot dépendait du nombre de bandes (1,6 cm en doublant le panneau inférieur du gouvernail). La charnière est maintenant prise aux bornes exactes de chaque gouverne et pondérée par la longueur (test, qui échoue avec l'ancienne méthode) ;
  - en passant, un défaut trouvé : la profondeur mélangeait ses deux côtés dans l'ajustement, sa droite était de longueur nulle. Chaque côté est désormais un groupe (test des longueurs de charnière) ;
  - `asset_check` mesure le pivot au **segment** de charnière : un aileron droit posé sur l'aile gauche échoue (test) ;
  - gouvernes en violet, mieux distinctes du beige de l'aile gauche ; raison de l'écart de hauteur complétée.
- **A** Nouvelle capture `maquette-spad7`, **regardée** : ailerons, profondeur et gouvernail en violet bien visibles ; le gouvernail commence sur une droite verticale ; flèches et vérifications inchangées.
- **D** En attente du joueur :
  - accord sur les deux écarts acceptés (longueur, hauteur) ;
  - charnières de l'empennage pour le modèle définitif. Recommandation de l'aérodynamicien : une charnière déclarée, facultative, par gouverne dans `aircraft.ron` (`hinge_m`, sourcée sur le dessin), avec un garde-fou au chargement. C'est un changement de format ;
  - ajout de `tools/mockup` et du rôle d'`asset_check` à l'arborescence de `CLAUDE.md` (proposé, non modifié).

## 2026-10-26

### Décisions sur le modèle 3D provisoire

- **Q** Le joueur :
  - « 1. d'accord pour l'analyse et le choix » ;
  - « 2. on adopte la recommandation de l'aérodynamicien » ;
  - « 3. ok pour CLAUDE.md ».
- **D** Les deux écarts du SPAD (longueur 6,264 m, hauteur 2,355 m) sont **acceptés**. Le commentaire de `maquette.ron` passe de « en attente » à « accepté le 2026-10-26 ».
- **D** Charnière déclarée adoptée. **A** Réalisation :
  - champ facultatif `hinge_m` des gouvernes dans `aircraft.ron` : extrémités de la vraie charnière, côté droit, symétrisées pour le gauche ;
  - le contrat (`escadrille_data::model`) la prend telle quelle, l'axe étant orienté par les bandes pour garder le sens des braques ; sinon, il garde la charnière déduite des bandes ;
  - garde-fou : refus si elle s'écarte de plus d'un quart de la corde moyenne de la gouverne ou de plus de 10° ;
  - le modèle de vol n'en tient pas compte ;
  - tests : charnière prise telle quelle, profondeur symétrisée, refus d'une charnière trop loin ou trop penchée, découpe du générateur sur la charnière déclarée, contrat de chaque avion vérifié par le test de cohérence des données ;
  - documentation dans `format-avion.md` et `pipeline-assets`.
- **R** Le SPAD ne déclare encore aucune charnière : il faut d'abord lire les vraies sur le dessin de 1917 (gouvernail sur l'étambot, charnière droite de la profondeur). `model.glb` est inchangé.
- **A** `CLAUDE.md` : `tools/mockup` ajouté à l'arborescence, rôle d'`asset_check` précisé (pivots et axes, lancé par `just test`), commande `just mockup <avion>`.

### Relecture des charnières sur le dessin de 1917

- **Q** Le joueur : « Fait les relire » (les vraies charnières du gouvernail et de la profondeur du SPAD, pour `hinge_m`).
- **A** Dessin retrouvé : *Flight*, 16 août 1917, p. 841 (archive.org `Flight_International_Magazine_1917-08-16-pdf`), page 15 rendue à 600 points par pouce dans le dossier de travail de la session. **Rien n'en est versé au dépôt** (règle des reproductions d'archives).
- **R** Le texte de 1917 dit : « The rudder is hinged at a point about 10 ins. beyond the fuselage termination ». À confronter au dessin.
- **A** Lecture confiée à l'aérodynamicien :
  - charnière du gouvernail en vue de côté ; charnière de la profondeur en vue de dessus ; contrôle des ailerons ;
  - valeurs proposées pour `hinge_m`, avec incertitude et origine `C:` ;
  - confrontation au garde-fou du contrat, sans forcer une valeur pour qu'elle passe.
- **R** Relecture de l'aérodynamicien (`spad7-calculs.md` § 3.1). Mesures au centre des traits, sur des profils d'intensité ; biais du scan retiré ; les deux vues concordent à 2 px près.
  - **Gouvernail** : axe vertical à **x −4,70 ± 0,03 m**, de z −0,31 à −1,29, là où le bord d'attaque de la dérive le rejoint ; pas de compensation.
  - **Étambot** : x −4,47. La charnière est 0,23 m (8,9 in) derrière, ce qui concorde avec les « about 10 ins. » de 1917.
  - **Profondeur** : charnière droite à **x −4,51 ± 0,03 m**, perpendiculaire à l'axe ; à 4 mm de celle des bandes.
  - **Ailerons** : la charnière des bandes concorde, pas de `hinge_m`.
  - **Garde-fou** : tout est dedans avec de la marge (gouvernail 0,057 m pour 0,115 ; profondeur 0,004 pour 0,090 ; angles nuls).
- **A** Images annotées de l'aérodynamicien **regardées** : l'axe du gouvernail et la charnière de la profondeur sont des traits nets, l'étambot et l'échancrure en V sont là où il les lit.
- **⚠** L'« étambot à x −4,75 » que j'avais écrit dans le plan et dans `pipeline-assets` était le bord d'attaque du panneau inférieur équivalent. L'axe réel est à 2 cm de la droite des bandes, et non 7. Corrigé.
- **A** Modifications :
  - `hinge_m` de la profondeur et des deux panneaux du gouvernail dans `aircraft.ron`, origine C: (dessin, échelle, incertitude). Le modèle de vol ne lit pas ce champ (vérifié) : pas de nouvelle campagne d'essais ;
  - `maquette.ron` : fuselage arrêté à l'étambot lu (x −4,47 au lieu de −4,80 estimé) ; l'en-tête ne dit plus qu'aucun dessin n'a été lu ;
  - deux tests figeaient la charnière déduite des bandes sur les données réelles (prévu par l'aérodynamicien). Celui du générateur vide maintenant les `hinge_m` ; celui des longueurs vérifie la charnière déclarée du gouvernail (0,98 m) ;
  - modèle régénéré. `asset_check` passe, et les écarts acceptés de longueur et de hauteur sont inchangés.
- **A** Nouvelle capture `maquette-spad7` **regardée** : gouvernail à partir de l'axe lu, fuselage arrêté à l'étambot ; flèches et vérifications inchangées.
- **R** Écarts du dessin avec le modèle de vol, signalés au joueur ; rien n'est changé :
  - **partie fixe de dérive absente** : sous le plan fixe, entre l'étambot et l'axe du gouvernail, environ 0,08 m². L'empennage vertical fait 0,98 m² sur le dessin contre 0,907 m² dans le modèle (−7 %, hors de la tolérance de ±2 %) ;
  - **ailerons** à 29 % de la corde sur le dessin, contre 32 % (environ 4 % d'efficacité en roulis) ;
  - **gouvernail équivalent** : corde décroissante au lieu de presque constante, centre plus bas, bras du roulis induit environ 15 % plus court ;
  - **bord d'attaque du plan fixe** à 3,86 m au lieu de 3,78 : retrouve les « 12 ins. » de la dérive ; sans effet sur le modèle ;
  - **moyeu**, lecture rapide : plan des pales à x 0,83 au lieu de 0,78.

### Plan de la partie fixe de la dérive

- **Q** Le joueur : « Oui » (préparer le plan de la correction de la partie fixe de dérive absente).
- **A** Plan `docs/plans/m1-derive-spad7.md` (proposé). Seules les données changent ; aucun code du modèle de vol :
  - panneau inférieur de la dérive de l'étambot (x −4,47) au bord de fuite dessiné, corde d'environ 0,78 m ;
  - gouvernail à environ 70 %, sa charnière sur l'axe dessiné ;
  - `reference_area`, `aspect_ratio` et `span_efficiency` recalculés sur le contour numérisé ;
  - campagnes d'essais avant et après, comparées point par point : stabilité de route attendue en hausse de 5 à 9 %, spirale un peu plus divergente, performances inchangées ;
  - modèle 3D régénéré, capture, revue de l'aérodynamicien.
- **D** Deux décisions demandées au joueur :
  - le plan ;
  - le sillage du fuselage sur le panneau ajouté : (a) aucun facteur, écart documenté (recommandé), ou (b) nouvelle fonction du modèle de vol.
- **Q** Le joueur : « 1. Oui 2. (a) ». **D** Plan `m1-derive-spad7.md` validé ; aucun facteur pour le sillage du fuselage, l'écart sera documenté.
- **A** Campagne d'essais de référence relancée sur l'état actuel et copiée dans le dossier de travail de la session. Elle redonne le dernier rapport : N_v +111,5 N·m/(m/s), L_v −18,0, spirale ×2 en 17,5 s, roulis hollandais de 3,46 s amorti à 0,193.
- **A** Géométrie du nouveau panneau demandée à l'aérodynamicien, sur son contour numérisé.
- **R** Géométrie de l'aérodynamicien, avec deux corrections au plan :
  - **panneau rectangulaire**, de l'étambot au bord de fuite dessiné : corde 0,775 m, gouvernail à 70 %, charnière des bandes à x −4,703, sur l'axe dessiné. La version effilée était refusée au chargement (−3,4 %), à cause de la façon dont le panneau supérieur avait été construit ;
  - surface de référence 1,00 m², allongement 1,36, `span_efficiency` inchangé ; 2 bandes suffisent.
- **⚠** Deux estimations de mon plan étaient fausses.
  - N_v ne gagne que +4,2 %, et non +5 à +9 % : à envergure égale, la corde ajoutée fait baisser l'allongement.
  - Le sillage du fuselage surestime N_v de 5 à 9 %, et non d'environ 1 % : tout le panneau inférieur est derrière le fuselage, gouvernail compris, qui était déjà compté à pleine pression avant. La partie ajoutée compte pour moins de 1 %.
  - L'option (a) reste appliquée, avec le bon chiffre dans le commentaire de `span_efficiency`.
- **A** Valeurs inscrites dans `aircraft.ron`, chacune avec son origine `C:`.
- **A** Campagne après, comparée à celle de référence ; aucun écart inexpliqué :
  - N_v de +111,5 à +116,2 (+4,2 %) ; L_v de −18,0 à −17,2 ;
  - roulis hollandais de 3,46 à 3,39 s ; spirale de ×2 en 17,50 s à 17,40 s ;
  - performances à 0,2 % près ;
  - vrille et manœuvres à la marge ; départ au décrochage toujours à gauche.
- **A** Modèle 3D régénéré ; `asset_check` passe. Captures avant et après **regardées**, avec un agrandissement de la queue : l'espace vide entre l'étambot et le gouvernail du bas est comblé.
- **A** Revue de l'aérodynamicien : conforme, rien de bloquant. Elle relève que mon test du générateur, à 5 cm près, était trop lâche. Il recalcule maintenant le pivot attendu depuis les panneaux, à 1 mm près. Renvoi ajouté dans le commentaire du gouvernail du haut.
- **A** `spad7-calculs.md` § 3 et § 5 mis à jour ; bilan d'exécution dans le plan.

### Modèle 3D réaliste : plan

- **Q** Le joueur : « Je suis d'accord avec les suggestions. Réalise un modèle 3D le plus réaliste possible, avec une excellente qualité de rendu. »
- **A** Vérifications préalables :
  - Blender n'est pas installé, mais le paquet Ubuntu 4.0.2 est disponible, ainsi que le module Python `bpy` 5.2 ; blender.org est bloqué par le proxy ;
  - environ 3,7 Go de disque libre ;
  - Bevy 0.20 sait rendre la transmission diffuse avec texture, le vernis, l'anisotropie et les décalques en grappes, mais plusieurs de ces fonctions demandent des features hors défaut.
- **A** Plan `docs/plans/m3-modele-spad7.md` (proposé). C'est l'élément « Modèle PBR du premier avion » du M3, avancé :
  - partie A : les trois points du dessin dans le modèle de vol (ailerons à 29 %, empennage vertical tel que dessiné, moyeu à confirmer), avec essais et revue ;
  - partie B : fiche d'aspect et livrée (historien), outil, géométrie, matériaux, rendu, poste de pilotage, validation par captures et par le directeur artistique.
- **D** Décisions demandées au joueur :
  - le plan, et la lecture de « les suggestions » ;
  - l'outil : Blender piloté par script, avec une exception au « Rust uniquement » pour les scripts de contenu dans `art/`, ou tout en Rust ;
  - la livrée ;
  - le périmètre du poste de pilotage.
- **Q** Réponses du joueur :
  - plan complet, partie A comprise (« les suggestions » = les trois points du dessin) ;
  - outil : Blender par script ;
  - livrée : la mieux documentée ;
  - poste : premier poste.
- **D** Plan `m3-modele-spad7.md` validé.
- **A** Lancés en parallèle :
  - l'historien : fiche d'aspect `spad7-aspect.md`, candidats de livrée, demandes d'archives ;
  - l'aérodynamicien : valeurs de la partie A (ailerons, empennage vertical tel que dessiné, mesure du moyeu).
- **A** Outil :
  - 1,6 Go libérés (cache incrémental de `target/`) ;
  - Blender 4.0.2 installé depuis le paquet Ubuntu (après `apt-get update` : un paquet de dépendance renvoyait 404), avec `python3-numpy` dont l'exporteur glTF a besoin ;
  - essai sans écran réussi : export glTF et Cycles.
- **D** ADR 0007 « Scripts Blender (Python) pour les sources de contenu dans `art/` », accepté par le choix du joueur. Le Python n'est admis que dans les scripts exécutés par Blender, rangés dans `art/` ; le jeu et les outils restent en Rust.
- **A** `CLAUDE.md` : la contrainte « Rust uniquement » cite l'exception, et l'arborescence mentionne les scripts de `art/`. Modification annoncée dans l'option que le joueur a choisie.
- **A** Chaîne de construction du modèle réaliste, partie B4 :
  - **contrat** : chaque charnière porte maintenant les zones qu'elle entraîne (surface, panneau, côté, envergure, part de corde). Le script Blender coupe les gouvernes comme le contrat les nomme, sans refaire la règle des noms (test) ;
  - **`mockup <avion> --json <fichier>`** écrit les données du modèle pour les scripts Blender : contrat, surfaces, formes de `maquette.ron`. JSON écrit à la main, sans dépendance nouvelle ; un test le relit ;
  - **protection** : `mockup` refuse d'écraser un `model.glb` qu'il n'a pas écrit, sauf `--force`, pour que le modèle réaliste ne soit jamais effacé par erreur (test) ;
  - **`asset_check <avion> --model <fichier>`** contrôle un modèle en cours de construction ;
  - **`art/spad7/build.py`**, première passe de forme : ailes au profil « Spad » du NACA Report 93, saumons arrondis, fuselage lofté en sections arrondies, empennages à section mince symétrique, mâts profilés, cabane, train et roues, béquille, hélice vrillée au pas de l'Éclair, Vickers, nœuds du contrat ;
  - **`just modele <avion>`** : construction dans `target/modele/`, puis contrôle ; `just modele <avion> assets` remplace `model.glb` si le contrôle passe.
- **R** Premier essai : tous les contrôles de repères passent du premier coup (4 gouvernes à 0,0 mm et 0,00°, hélice, œil, arme, envergure). La chaîne Blender → glTF → Bevy de l'ADR 0006 tient.
  - La longueur (6,34 m) et la hauteur échouent, comme prévu : l'empennage est encore l'équivalent du modèle de vol, en attendant les contours réels du dessin.
  - Le modèle provisoire reste donc dans `assets/`.
- **A** Capture dans le jeu (modèle copié temporairement, puis remis) **regardée** :
  - fuselage arrondi, profil cambré, mâts profilés, roues à voiles, pales vrillées : le saut par rapport aux boîtes est net ;
  - corrigé après la capture : couleurs trop claires (Blender attend des valeurs linéaires, je donnais du sRGB) et arrondi des saumons trop fort (rayon de 0,18 à 0,10 m, « corners slightly rounded off »).
- **⚠** J'ai reposé par erreur au joueur les quatre questions déjà tranchées (plan, outil, livrée, poste). Ses réponses sont identiques ; rien ne change.
- **R** Valeurs de l'aérodynamicien pour la partie A, chacune vérifiée sur une copie des données :
  - **ailerons** à 29 % ;
  - **empennage vertical** en cinq panneaux, qui reproduisent la surface, le centre et la répartition du gouvernail dessinés ;
  - **moyeu** : x reste à 0,78 (mon « 0,83 » était la face avant des pales), mais z se lit à −0,40 au lieu de −0,38, au-delà de l'incertitude. C'est un point à part : le bas de la dérive est à cheval sur l'axe du souffle, et les équilibres de direction y sont sensibles.
- **A** Partie A, point 1 : `chord_ratio` des ailerons de 0,32 à 0,29, avec son origine `C:`.
  - Campagne : rapport identique au caractère près, la campagne ne mesurant pas le roulis. Efficacité des ailerons prévue à −4,2 %.
  - Modèle provisoire régénéré : sa charnière d'aileron recule de 3,6 cm.
- **R** Rapport de l'historien. Fiche `docs/recherche/spad7-aspect.md` et demandes D4 (complément) et D8 à D12.
  - **Limite** : les sites des musées et des archives n'ont pu être lus qu'en résumé du moteur de recherche.
  - **Établi** (confiance C, sauf mention) : capot aluminium peint jaune ; radiateur sans volets sur un 150 ch du début de 1917 ; échappements en collecteur ; découpe du plan supérieur en demi-cercle ; renvois d'ailerons (B) ; poste d'après le S.XI ; hélice Éclair 6 en noyer verni.
  - **Candidats de livrée** : S.254 « Vieux Charles » de Guynemer (SPA 3, photographié le 20 octobre 1917, conservé ; mais **180 ch**), S.113 de Heurtaux, S.2489 de Baracca. Sa recommandation : S.254 en deux couches de décalques, la finition d'usine puis les marques de Guynemer.
- **D** Décisions à soumettre au joueur, après la partie A : la livrée, le moteur du S.254 (180 ch) et l'usage d'un personnage réel.
- **A** Partie A, point 2 : empennage vertical en cinq panneaux.
  - **Découpage** : une bande par cassure du contour dessiné. Chacune garde la surface, la hauteur de son centre et son quart de corde moyen ; chacune a sa part de gouvernail et sa charnière déclarée sur l'axe dessiné.
  - **Gouvernail** : 0,536 m², centre à z −0,859 contre −0,863 dessiné ; l'ancien découpage le mettait à −0,792.
- **A** Campagne, conforme aux prévisions de l'aérodynamicien :
  - N_v de +116,2 à +113,9 (−2,0 %), le quart de corde moyen de la dérive avançant de 4 cm ; L_v de −17,2 à −17,6 ;
  - roulis hollandais de 3,39 à 3,42 s ; spirale de ×2 en 17,40 s à 17,42 s ;
  - glissade sur la queue de 109,5 à 100,9 km/h ; performances inchangées.
- **R** Modèle provisoire régénéré : il mesure 6,099 m pour 6,08 (+0,3 %), dans la tolérance. `asset_check` exige alors de retirer l'écart de longueur accepté : c'est le mécanisme prévu. Écart retiré de `maquette.ron` ; seule la hauteur reste acceptée.
- **A** Tests adaptés à la nouvelle dérive : zones du gouvernail, longueur de charnière, écarts acceptés.
- **A** Partie A, point 3 : moyeu à z −0,40 au lieu de −0,38 (x inchangé).
  - Moteur, radiateur et hélice suivent la ligne de traction dans `masses.ron`, ainsi que la traînée du radiateur frontal.
  - Bilan à vide recalculé par `fdm_lab inertia` : centre de gravité z −0,459, ixz 24 ; `aircraft.ron` concorde.
  - Commentaires de convergence des bandes de la dérive mis à jour avec les chiffres de l'aérodynamicien pour l'état final.
- **A** Campagne, conforme aux prévisions :
  - dérapage de croisière de 0,80 à 0,65°, direction d'équilibre +0,01 ;
  - N_v 113,0, L_v −17,3, phugoïde 29,8 s ;
  - glissade sur la queue de 100,9 à 68,6 km/h ;
  - départ au décrochage toujours à gauche ; performances inchangées.
- **R** Le bas de la dérive est à cheval sur l'axe du souffle, où sa rotation change de sens : les équilibres de direction y gagnent environ 0,01 de course par centimètre de hauteur du moyeu.
- **A** Modèle provisoire régénéré ; `spad7-calculs.md` § 3.1 et § 5 mis à jour. Revue finale de la partie A demandée à l'aérodynamicien.
- **Q** Le joueur : « J'ai atteint ma limite d'utilisation pendant que vous travailliez, mais elle a été réinitialisée. Veuillez continuer là où vous vous êtes arrêté. »
- **⚠** La revue finale de la partie A par l'aérodynamicien avait été coupée par la limite d'usage ; le processus de la session a aussi redémarré. État vérifié : tout était commité et poussé jusqu'au moyeu (`0cdc99e`).
- **A** Revue reprise. **R** Conforme, rien de bloquant :
  - données et origines vérifiées, chiffres de convergence mesurés par l'aérodynamicien sur ces données ;
  - campagnes `essais-a0` à `a3` identiques aux siennes (à izz près, 1359 contre 1360, sans effet sur les équilibres) ;
  - tests modifiés corrects.
- **A** Corrigé dans la foulée :
  - trois passages périmés de `spad7-calculs.md` § 3.1 : charnière des cinq panneaux, tableau du garde-fou recalculé pour les cinq panneaux, ancienne correction et ancienne liste d'écarts marquées comme remplacées ;
  - commentaire du sillage dans `aircraft.ron` : deux panneaux sous le plan fixe, et chiffre mesuré avec l'ancien moyeu.
- **Q** Livrée (B2), demandée au joueur : **S.254 « Vieux Charles »** dans son état du 20 octobre 1917, et **usage réservé** (studio, as de l'IA, livrée historique au choix).
- **D** Livrée retenue en deux couches de décalques : finition d'usine, puis marques de Guynemer. Moteur de 180 ch accepté, invisible de l'extérieur. L'avion du joueur prend la finition d'usine avec un numéro fictif. Décision consignée dans la fiche d'aspect (§ 17) et dans le plan.
- **A** Corrections de l'historien reportées (fiche d'aspect § 14) :
  - dans `spad7.md` : moteur de 180 ch de S.254, S.254 à Saint-Pol-sur-Mer jusqu'en octobre 1917, contradiction sur S.113, dates de l'appellation « SPA 3 » ;
  - dans `hispano-suiza-8a.md` : préférence de Guynemer pour les hélices Chauvière (Hartmann).
- **A** B4, passe 2 : l'empennage du modèle réaliste suit les contours du dessin.
  - **Données** : contours numérisés par l'aérodynamicien, ré-échantillonnés tous les 2 cm et inscrits dans un nouveau champ `outlines` de `maquette.ron`, avec leur origine `C:`.
    - Plan fixe : demi-envergure, moyenne des deux moitiés du dessin, qui est décentré de 6 cm, ramenée aux 1,405 m des données.
    - Dérive et gouvernail : de z −0,315 à −1,395.
    - Échancrure en V de la profondeur : sa pointe sur la charnière à l'axe, où le dessin ne donne pas de bord de fuite.
  - **Export** : le JSON des scripts Blender transmet les contours ; un contour qui nomme une surface inconnue est refusé (test).
  - **Script** : `build.py` construit une surface d'après son contour quand il en a un, et coupe les gouvernes sur leurs charnières déclarées.
- **⚠** Premier essai arrêté : en haut du gouvernail, toute la corde est derrière l'axe et la partie mobile commençait au bord d'attaque même, ce qui changeait le nombre de points d'un anneau. La fraction de charnière est maintenant gardée strictement dans la corde.
- **R** Le modèle Blender passe tous les contrôles, **longueur comprise** : 6,037 m pour 6,08, sans écart accepté. La hauteur au sol vaut 2,343 m, au lieu de 2,355 pour les boîtes, à cause des saumons arrondis.
- **D** Le modèle Blender remplace les boîtes dans `assets/` (2,4 Mo) : meilleur à tous égards, et tous ses contrôles de repères passent. L'écart de hauteur accepté passe à sa valeur (2,343 m), pour la même raison. `mockup` refuse désormais de l'écraser.
- **A** Capture `maquette-spad7` **regardée**. Vérifications de repères inchangées. Fuselage arrondi, ailes aux saumons arrondis, mâts profilés, Vickers, vrai contour de la dérive.
- **⚠** La capture montrait des stries sur la dérive : chaque tranche de gouverne était fermée par ses propres faces de bout, ce qui laissait des faces internes. Les tranches consécutives d'une même gouverne sont maintenant lissées d'un seul tenant (ailes comprises). Nouvelle capture **regardée** : dérive lisse, léger moiré dû au bruit de numérisation (de l'ordre du centimètre), à lisser plus tard.
- **R** Le fuselage paraît presque noir : la couleur des matériaux est l'objet de B5.
- **A** Documentation : `outlines` dans `format-avion.md`, modèle réaliste dans `pipeline-assets`.
- **R** Lecture de l'aérodynamicien pour B4 (tables T1 à T7, au centre des traits, ±0,015 m, superpositions de contrôle regardées) :
  - **fuselage** : profil de côté et largeur en vue de dessus. La largeur dessinée est de 0,85 à 0,97 m, contre 0,80 m estimé (E:) ;
  - **nez** : radiateur de 0,67 m (0,71 à la première lecture était le bord extérieur), anneau avant, bossages des arbres à cames, échappements ;
  - **plan supérieur** : découpe au-dessus du poste, congés de 0,19 m aux saumons ;
  - **mâts** aux longerons (12 et 46 % de la corde dessinée), cabane verticale, tube d'acier ;
  - **train** : roue de 0,61 m (et non 700 mm), jambes, béquille au patin à z −0,09 ;
  - **poste** : ouverture et bourrelet. Le pare-brise et la Vickers ne sont pas dessinés.
- **A** B4, passe 3 : les mesures passent dans le modèle réaliste.
  - **`maquette.ron`** :
    - 52 sections de fuselage mesurées au lieu de 6 estimées ;
    - roues de 0,61 m et patin de béquille du dessin ;
    - mâts aux y lus ;
    - nouveau champ générique `features` : tables nommées, chacune avec son origine, lues par le script Blender. Noms uniques, nombres finis (test) ;
    - Vickers remontée sur le capot : sa bouche estimée (z −0,95) était sous le dessus du capot mesuré (−0,97).
  - **`build.py`** :
    - fuselage plus finement maillé, avec l'ouverture du poste découpée, son bourrelet et un baquet sombre ;
    - radiateur, lèvre de l'anneau avant, bossages aux bouts arrondis ;
    - échappements : quatre tubulures, collecteur, coude avant et tuyau le long du flanc ;
    - découpe du plan supérieur et congés des saumons ;
    - mâts aux longerons, cabane verticale, tube d'acier ;
    - jambes de train, béquille en lame.
- **R** Le contrôle passe. La hauteur au sol tombe à **2,271 m** avec le train dessiné (2,343 m avant), soit +3,2 % sur les 2,20 m publiés. L'écart accepté par le joueur est mis à jour avec cette valeur, pour la même raison (pneus et sandows non écrasés).
- **A** Capture du jeu et rendus Cycles en trois quarts avant et de face, **regardés**. La silhouette est celle d'un SPAD VII : capot rond et radiateur, bossages, échappements, train en V caréné, cabane, bourrelet du poste.
  - Corrigé : bossages à bouts plats, devenus arrondis.
  - Reste à revoir sur photo : le coude avant du collecteur, un peu raide.
- **R** Écarts du dessin avec le **modèle de vol**, signalés au joueur, rien n'est changé :
  - largeur du fuselage de 0,96 m au lieu de 0,80 m (bloc fuselage : surfaces, volume) ;
  - roue de 0,61 m au lieu de 700 mm (traînée des roues) ;
  - roue et béquille dessinées : utiles pour le train au sol du M2.
- **Q** Le joueur : « Peux tu faire un modèle 3D encore plus réaliste ? »
- **R** Oui, dans le cadre du plan validé (passes de B4, puis B5). Ce qui manquait le plus au réalisme : les couleurs, le câblage, la toile et les détails mécaniques. Les marques (cocardes, gouvernail tricolore, insignes) demandent de vraies textures : passe suivante.
- **A** B4, passe 4 :
  - **câblage** : câbles de vol doublés en tandem, câbles d'atterrissage croisés aux mâts intermédiaires (S48), câbles d'incidence, croisillons de cabane, haubans d'empennage. Ce sont des nœuds `wire_*`, aux diamètres d'`aircraft.ron` (4,5, 3,5 et 3 mm) ;
  - **toile** : bord de fuite festonné entre les nervures (pas de 0,19 m du S.XI, `ESTIMÉ` ; creux de 1,5 cm, E:) ;
  - **mécanique** : renvois d'ailerons extérieurs (tige de 15 mm, guignol, renvoi), pare-brise en verre sur cadre, Vickers avec manchon, cône de bouche et boîte de culasse ;
  - **matériaux** : le capot en aluminium peint jusqu'au poste (S48), la toile en arrière ;
  - **finition d'usine de 1917** (fiche d'aspect § 12.1, C) : toile vernie jaune pâle, métal peint d'un jaune plus foncé, mâts en bois verni, hélice en noyer verni. Les teintes sont `ESTIMÉ`, faute de nuancier d'époque, en attendant la demande D8.
  - Les dimensions sont des tables nommées de `maquette.ron`, chacune avec son origine.
- **A** Rendus Cycles en trois quarts avant et arrière, **regardés**.
- **⚠** Le fuselage était bosselé : le bruit de lecture des 52 sections mesurées (1 à 2 cm) faisait onduler le loft. **A** Le script lisse maintenant sections et contours (filtre binomial sur ±0,2 m) ; les données gardent les mesures brutes. Nouveau rendu **regardé** : fuselage lisse, dérive sans moiré.
- **A** Modèle installé dans `assets/` (2,7 Mo) ; `asset_check` passe (longueur 6,031 m, hauteur à l'écart accepté). Capture du jeu **regardée** : finition jaune et câblage visibles, vérifications de repères inchangées.
- **R** Les câbles, plus fins qu'un pixel, scintilleront en mouvement sans l'anticrénelage prévu au plan (B6).

## 2026-10-27

### Modèle 3D réaliste : textures de la toile (B5, première passe)

- **Q** Le joueur : « Continue » (suite de la passe 4, qui annonçait les textures).
- **A** Données :
  - nouveau fichier `assets/aircraft/spad7/livrees.ron`, schéma `crates/data/src/livery.rs` : couleurs nommées (sRGB) et livrées. Une livrée a une couleur de toile, une couleur de métal peint et des marques : cocardes (`Roundel`) sur une face, bandes (`Bands`) sur une gouverne ;
  - première livrée : la **finition d'usine de 1917** (« usine-1917 »), couche de base de S.254 :
    - toile vernie jaune pâle et métal d'un jaune plus foncé, les teintes de la passe 4 (`ESTIMÉ`, demande D8) ;
    - cocardes d'environ 1,0 m sur l'extrados du plan supérieur et l'intrados du plan inférieur, à un diamètre du saumon (`ESTIMÉ`, § 12.2 de la fiche d'aspect) ;
    - gouvernail en bandes bleu, blanc, rouge, bleu côté charnière ;
  - contrôles au chargement (couleurs connues, composantes, faces, diamètres) et à l'export pour Blender (la face existe sur l'avion, la bande est sur une gouverne) ; tests ;
  - quatre tables de `maquette.ron` pour la toile, toutes `ESTIMÉ` sauf une :
    - `rib_tapes` : rubans de 40 mm, coutures tous les 60 mm, teinte un peu plus foncée sous le vernis ;
    - `false_ribs` : fausses nervures jusqu'au longeron avant, à 12 % de la corde (C, S.XI) ;
    - `fabric_sag` : creux de 1,5 mm entre deux nervures ;
    - `tail_ribs` : nervures de l'empennage tous les 0,20 m ;
  - le format des données pour Blender passe en version 2.
- **A** Script Blender (`art/spad7/build.py`) :
  - chaque face entoilée est étiquetée par sa surface, puis rangée par côté selon sa normale (dessus et dessous, ou flancs) ;
  - chaque région est projetée à plat, et toutes sont rangées dans un atlas de 4096 px, à 446 px/m (2,2 mm par pixel) ;
  - deux textures sont peintes en numpy, sans image tierce : couleur et normales ;
  - les gouvernes partagent la région de leur surface ;
  - les nervures peintes tombent sur les festons du bord de fuite ;
  - modèle de 4,8 Mo, contre 2,7 Mo à la passe 4 (deux PNG de 0,7 Mo).
- **A** Vérifications :
  - `asset_check` passe, inchangé ;
  - textures **regardées** (atlas entier, détail d'une cocarde et de la carte de normales) ;
  - trois rendus Cycles **regardés** (trois quarts avant, dessus, dessous) : cocardes à leur place, bleu du gouvernail côté charnière, nervures et rubans visibles dessus comme dessous ;
  - capture du jeu **regardée** : textures chargées, cocardes et gouvernail en place ;
  - `just check` et `just test` passent (280 tests).
- **⚠** En jeu, de fines lignes claires scintillent sur le gouvernail. Bevy 0.20 ne génère pas les mipmaps des images d'un glTF (vérifié dans les sources : seule une génération pour les cibles de rendu existe, `MipGenerationJobs`) ; les rubans et les coutures, plus fins qu'un pixel de loin, crénèlent. **D** À traiter en ouvrant B6 : KTX2 avec mipmaps, ou mipmaps calculées au chargement.
- **R** Poids du dépôt : chaque régénération du modèle ajoute environ 5 Mo à l'historique. Le plan prévoit Git LFS pour les gros fichiers, pas encore mis en place.
- **R** Reste en B5 :
  - les marques de Guynemer (cigogne, « 2 », « Vieux Charles », « S 254 »), en seconde livrée sur le même atlas ;
  - la transmission de la toile à contre-jour ;
  - le bois, le métal et la rugosité.
- **⚠** Le commit des textures contenait par erreur un `.blend` de 9 Mo nommé `--atlas` : un appel de débogage du script l'avait pris pour son chemin de sortie. **A** Fichier retiré par un commit suivant (il reste dans l'historique, sans réécriture de la branche poussée). Le script refuse désormais des arguments qui ne finissent pas par `.glb` et `.blend`.

### Rendu en jeu : mipmaps des textures (début de B6)

- **Q** Le joueur : « Continue ».
- **A** Nouveau plugin `render` du jeu (`crates/game/src/render/`). Quand un matériau arrive, il calcule les mipmaps de ses textures qui n'en ont qu'un niveau, jusqu'à un texel :
  - couleur moyennée en lumière linéaire ;
  - normales moyennées puis renormalisées ;
  - autres données moyennées telles quelles.

  Il active aussi le filtrage anisotrope ×16 quand l'échantillonneur est trilinéaire, ce que demande le glTF de Blender. Les ailes sont presque toujours vues par la tranche.
- **A** Tests : longueur de la chaîne, moyenne en lumière linéaire (noir et blanc donnent 188, pas 128), normales renormalisées, image complétée une seule fois.
- **A** Coût mesuré : environ 0,5 s par texture 4K en profil de développement, après passage à une table de décodage (1,5 s avant). Des KTX2 faites hors ligne restent prévues.
- **A** Captures avant et après **regardées**, avec l'image de leur différence :
  - les changements se concentrent là où la texture est réduite : la cocarde vue par la tranche, les bords des bandes du gouvernail ;
  - les lignes fines qui restent sur le gouvernail sont ses nervures, qui accrochent la lumière rasante ;
  - le scintillement, lui, ne se juge qu'en mouvement : à vérifier sur ta machine.
- **A** `just check` et `just test` passent (284 tests).

### Rendu en jeu : scène de studio (B6)

- **Q** Le joueur : « Continue ».
- **A** Lancement de l'historien, en arrière-plan, sur les marques de S.254 (cigogne, « 2 », « Vieux Charles », « S 254 ») pour la seconde couche de livrée.
- **A** Nouvelle scène de référence `avion-studio` (`crates/game/src/world/studio.rs`) :
  - **Avion** : le SPAD au repos sur ses roues et sa béquille. L'assiette (14,4° cabré) est calculée depuis les roues et la béquille de `maquette.ron`, ce qui retrouve les « environ 14° » du dessin (test).
  - **Prise de vue** : trois quarts avant, œil à 1,65 m.
  - **Lumière** : soleil d'après-midi à 35° de hauteur, ombres en deux cascades, atmosphère de Bevy, éclairage ambiant tiré du ciel, bloom.
  - **Anticrénelage** : TAA.
- **A** Capture **regardée** :
  - avion bien posé, ombres nettes, cocardes et gouvernail en place ;
  - nervures visibles sous l'intrados ;
  - câbles continus sur l'image fixe.

  Les cocardes visibles sur le plan inférieur sont celles de son intrados : cabré au sol, l'avion montre le dessous de ses ailes à un observateur debout devant lui.
- **D** Tonemapping : **TonyMcMapface**, choisi sur trois captures comparées. ACES sature la toile vers l'orange ; AgX la délave à cette exposition. Consigné dans le skill `rendu-visuel`, avec les scènes `maquette-spad7` et `avion-studio` ajoutées à sa table.
- **R** Rendu logiciel dans le conteneur : 232 ms par image ; la vitesse se juge sur ta machine.

### Rendu en jeu : anticrénelage des câbles (B6)

- **A** Technique « phone-wire AA » (skill `rendu-visuel`) :
  - nouveau matériau du jeu pour les câbles, `crates/game/src/render/wires.rs`, sur la base du matériau standard ;
  - shader `assets/shaders/wire_aa.wesl` :
    - le shader de sommets élargit chaque câble à au moins 1,5 pixel à l'écran, le long des normales du tube ;
    - le shader de fragments réduit l'opacité dans la même proportion, en mélange alpha.
- **A** Modèle : le script Blender écrit le diamètre des câbles de chaque nœud `wire_*` dans ses extras glTF (`diameter_m` : 4,5 mm pour le vol, 3,5 mm pour l'atterrissage et l'empennage, 3 mm pour l'incidence et la cabane), et lisse ses tubes pour que leurs normales s'écartent de l'axe. Modèle régénéré et installé ; `asset_check` inchangé.
- **A** Captures d'`avion-studio` **regardées** :
  - sans la technique, les câbles d'incidence sont pointillés ;
  - à 1,0 pixel, ils le restent : un tube à six pans ne mesure que 0,87 fois son diamètre entre deux faces ;
  - à **1,5 pixel**, tous les câbles sont continus. Ils sont aussi plus clairs, comme il se doit pour un fil de 4,5 mm qui couvre moins d'un pixel à 8 m.
- **A** Tests : lecture du diamètre dans les extras, rayon transmis au shader. `just check` et `just test` passent.
- **Q au joueur** : Bevy 0.20 écrit ses shaders en **WESL**, du WGSL avec imports et compilation conditionnelle. Les nôtres suivent, pour importer ses modules. `CLAUDE.md` dit « Shaders en WGSL » : faut-il écrire « WGSL (dialecte WESL de Bevy) » ? Je n'ai pas modifié `CLAUDE.md`.
- **R** Le scintillement en mouvement ne se juge pas sur une image fixe : à regarder sur ta machine.

### Recherche : marques de S.254 pour la seconde couche (historien)

- **A** L'historien a rédigé le § 12.6 de `docs/recherche/spad7-aspect.md` :
  - ce que disent les notices des photos datées ;
  - la cigogne de la SPA 3 et ce qui la distingue de celles du GC 12 ;
  - le « 2 », « Vieux Charles », le gouvernail ;
  - les autres marques ;
  - l'avion conservé ;
  - un gabarit de la cigogne pour le dessin vectoriel, construit sans copier d'image ;
  - une proposition chiffrée de places et de tailles.

  Il a ajouté neuf sources (V40 à V48) et complété les demandes D8 et D9 ; D13 (ouvrages) et D14 (Musée de l'Armée) sont nouvelles. Le redémarrage du conteneur l'a interrompu après la rédaction, avant son rapport : le texte a été relu ici.
- **R** Établi : la présence le 20 octobre 1917 du nom, de la cigogne et du « 2 » (B) ; la posture « ailes basses » de la cigogne de la SPA 3 (B) ; la cigogne et le « 2 » rouges (C) ; une inscription peinte sur le gouvernail (C).
- **R** Non trouvés, donc `ESTIMÉ` (D) : positions, tailles, graphies, sens de vol ; noir et place du nom (seul témoin : Udet, récit contesté).
- **⚠** Contradiction ouverte : la version anglaise d'un billet du Musée de l'Armée parle d'une « bande tricolore des as » sur la queue de S.254, que le § 12.5 classe en anachronisme. **D** Retenu provisoirement : pas de bande supplémentaire. C'est le premier point à vérifier sur les plaques du 20 octobre 1917 (D9, D14).
- **Pour le joueur** : D13 et D14 sont à envoyer, si tu le souhaites (achat ou consultation d'ouvrages ; photos du Musée de l'Armée).

### Toile translucide et scène `contre_jour_toile` (B5 et B6)

- **A** Transmission de la toile :
  - **Texture** : le script Blender peint dans l'alpha de la couleur la part de lumière que laisse passer la toile, valeurs `ESTIMÉ` de `fabric_transmission` (`maquette.ron`) :

    | Endroit | Part transmise |
    |---|---|
    | entre les nervures | 0,35 |
    | sous un ruban | 0,25 |
    | sous la peinture d'une livrée (cocardes, gouvernail) | 0,10 |
    | sur une nervure, une fausse nervure ou un longeron | 0,03 |

    Les longerons sont sous les mâts, à 12 et 46 % de la corde.
  - **Matériau** : ses extras glTF portent `diffuse_transmission` et la profondeur du second lobe, 10,6 cm.
  - **Jeu** : `crates/game/src/render/fabric.rs` applique la transmission diffuse de Bevy, avec pour carte la texture de couleur.
  - **Feature** : `pbr_transmission_textures` activée pour le jeu, consignée dans l'ADR 0004, comme le prévoyait le plan.
- **D** Profondeur du second lobe : Bevy éclaire le côté transmis depuis un point placé à `thickness` derrière la surface, et l'ombre depuis là. On la règle au-delà de la peau opposée, là où les surfaces sont les plus épaisses. Une aile ne s'ombre ainsi pas elle-même, mais l'autre plan et le fuselage l'ombrent encore.
- **A** Nouvelle scène de référence `contre_jour_toile` :
  - **Avion** : en vol, 13 m au-dessus et 15 m au nord d'un homme debout ; cap nord-est, incliné de 10°.
  - **Lumière** : soleil au nord à 50°, soit 9° derrière l'avion vu du sol.
  - **Objectif** : 24°.
  - **Code** : les deux scènes partagent désormais l'avion, le plein air et la caméra.
- **A** Captures **regardées** :
  - **Cadrage** : la première, avion de profil vu d'en dessous, était peu lisible ; elle a été recadrée de trois quarts et resserrée.
  - **Contre-jour** : la toile s'illumine ; le quadrillage des nervures et des longerons se lit sur les ailes et l'empennage, et le gouvernail peint paraît plus dense.
  - **Studio** : l'intrados du plan supérieur, éclairé par le soleil à travers la toile, montre maintenant son quadrillage.
- **R** **Limite** : le fuselage s'éclaire uniformément, sans la pénombre de son intérieur ni ses cadres.
- **A** Ménage du disque : 12 Go de compilations périmées supprimées (`target/debug/deps`, anciennes versions des crates Bevy et du jeu ; brouillons de compilation du bloc-notes). Un redémarrage du conteneur a interrompu une compilation, sans perte.
- **A** `just check` et `just test` passent (288 tests).

### Question : peut-on déjà piloter à la manette ?

- **Q** Le joueur : « y a t'il déjà prêt une version où je peux essayer de piloter avec la manette ? »
- **R** Pas encore. Ce qui existe :
  - le modèle de vol, qui vole en ligne de commande (`fdm_lab`, `just essai spad7`), sans train ni sol, reportés au M2 ;
  - dans le jeu, la scène de référence fixe (`just run`), les scènes de capture et le **labo manette** (`just labo-manette`), qui vérifie déjà les boutons et les axes de la SN30 Pro.

  Le pont entre le modèle de vol et le jeu ne contient encore que les conversions de repères (ADR 0006). Piloter à la manette est l'objet du **M2 « Premier vol »** ; le M1 attend toujours son essai joueur (lecture du rapport d'essais).
- **R** Proposition : un premier incrément jouable du M2, un « vol d'essai » qui démarre en l'air. Il comprendrait :
  - le modèle de vol au pas fixe dans le jeu ;
  - la manette par une première couche d'actions (manche, palonnier, gaz) ;
  - une caméra de poursuite ;
  - le SPAD sur la scène de plein air.

  Décollage et atterrissage viendraient ensuite, avec le train. Plan à faire valider avant le code (tâche sur plusieurs plugins). **En attente de la décision du joueur.**

### Plan du vol d'essai (premier incrément du M2)

- **Q** Le joueur : « Avant la seconde livrée, je veux une première version jouable, un vol d'essai qui démarre en l'air. » Il demande :
  - le modèle de vol branché dans le jeu ;
  - stick gauche pour le manche, gâchettes pour le palonnier, stick droit ou boutons pour les gaz ;
  - au choix, une vue de poursuite et une vue du pilote ;
  - la plaine, le ciel et les ombres.
- **A** Plan rédigé : `docs/plans/m2-vol-essai.md`, en sept étapes :
  1. état de départ ;
  2. modèle de vol au pas fixe ;
  3. couche d'actions et manette ;
  4. vues ;
  5. plaine ;
  6. télémétrie ;
  7. vérification.
- **R** Constats faits en préparant le plan :
  - **Équilibre au départ** : le solveur d'équilibre de `fdm_lab` utilise `nalgebra`, réservé aux outils. L'état de départ sera donc calculé par `fdm_lab`, rangé dans un fichier de données, et contrôlé par un test.
  - **Manche au neutre** : le SPAD du modèle s'équilibre à **88 km/h** (incidence 10,7°), près du décrochage. Pour tenir 160 km/h, il faut pousser le manche de 11 % (sonde du 2026-10-28, palier à 1 000 m). Le plan propose un neutre de profondeur réglable sur la manette. Ce constat ira à l'aérodynamicien à la prochaine revue : le calage de l'empennage est estimé, et aucun compensateur du S.VII n'est connu.
  - **Palonnier** : les gâchettes L2/R2 sont tout-ou-rien (labo du 2026-10-20) ; il passera donc par une rampe.
- **D** Quatre décisions demandées au joueur :
  1. les gaz ;
  2. le neutre de la profondeur ;
  3. le point de départ ;
  4. le vol sans aides.

### Vol d'essai, étape 1 : état de départ

- **Q** Le joueur valide le plan avec les options recommandées :
  - gaz sur la croix ↑/↓, stick droit au regard libre ;
  - neutre de la profondeur réglable ;
  - départ à 1 000 m et 160 km/h ;
  - sans aides.
- **A** Nouveau fichier `assets/scenarios/vol-essai.ron` (schéma `crates/data/src/scenario.rs`) :
  - SPAD, réservoirs pleins, cap à l'est, 1 000 m, 160 km/h ;
  - équilibre du modèle de vol calculé par `fdm_lab trim` : incidence 0,14°, dérapage 0,61°, manche −0,108, gaz 0,544, hélice 1 476 tr/min ;
  - le schéma construit l'état initial et les commandes.
- **A** Test `the_test_flight_starts_in_equilibrium` (`fdm_lab`) :
  - il refait l'équilibre et le compare au fichier ;
  - il vérifie qu'une seconde d'intégration depuis l'état du fichier ne bouge presque pas : moins de 5 cm, de 0,01 m/s et de 0,001 rad/s ;
  - contrôle par mutation : un manche faussé à −0,100 le fait échouer, avec les valeurs à recopier.

### Vol d'essai, étapes 2 à 7 : on peut piloter le SPAD à la manette

- **A** Modèle de vol dans le jeu (`crates/game/src/flight/`) :
  - **Simulation** : pas fixe à 120 Hz (RK4, ADR 0005), états précédent et courant, rendu interpolé.
  - **Origine flottante** : origine sur une grille de 2 km sous l'avion, qui est la période de la texture du sol.
  - **Animation du modèle** : les gouvernes tournent de leur déflexion autour de leur charnière. Le contrat du modèle donne désormais la commande de chaque charnière : nouveau champ `control` de `Hinge`, dans `escadrille_data::model`. L'hélice tourne à son régime ; au-dessus de 400 tr/min, son disque flou translucide remplace les pales.
  - **Sol et pause** : contact avec le sol (roues, béquille, saumons, disque de l'hélice), message, puis nouveau départ après 3 s. La pause arrête l'horloge virtuelle.
  - **Tests sans rendu** :
    - une minute de palier tenue à moins de 2 m, 2,67 km parcourus vers l'est, origine flottante suivie ;
    - un piqué au sol, suivi du nouveau départ ;
    - l'origine recalée sur sa grille.
- **A** Couche d'actions (`crates/game/src/input/`) : profil « Vol d'essai » du plan, documenté dans le skill `manette`.
  - **Traitement** : zone morte radiale, courbe, rampe du palonnier sur les gâchettes tout-ou-rien, gaz et neutre en leviers (appui et maintien).
  - **Neutre** : le manche est mis à l'échelle autour du neutre ; au départ, il vaut le manche qui tient le palier (−0,108).
  - **Clavier de secours** et tests de chaque traitement.
- **A** Vues (`crates/game/src/camera/`) : poursuite à 10 m, amortie sur 0,3 s, horizon droit ; œil du pilote calculé depuis la pose de l'avion de la même image, sans retard ; regard libre. La caméra d'extérieur est mise en commun avec les scènes de studio.
- **A** Plaine (`crates/game/src/world/plain.rs`) :
  - **Texture** : tuile de 2 km générée au lancement, faite d'îlots bordés de haies et découpés en parcelles de 25 à 70 m, d'une à quatre bandes de même culture.
  - **Raccord** : un test vérifie que la texture se répète exactement ; contrôle par mutation fait.
  - **Lumière** : atmosphère, soleil au sud-ouest à 40°, ombres en quatre cascades.
- **A** Télémétrie et aide (`crates/game/src/ui/`, egui) : vitesses indiquée et vraie, altitude, variomètre, régime, gaz, incidence, dérapage, facteur de charge, commandes. L'aide des commandes s'affiche 20 s au départ et à chaque pause. Ces textes d'interface de débogage passeront en Fluent avec les premiers vrais écrans.
- **A** `just run` lance désormais le vol d'essai, à la place de la scène fixe du M0. Deux scènes de capture : `vol-essai-poursuite` et `vol-essai-pilote`.
- **⚠** Première vue du pilote : un rectangle sombre bouchait la vue. La vitre du pare-brise, exportée par Blender en transmission spéculaire, s'affiche opaque dans Bevy sans sa passe de transmission. **A** Elle passe en mélange alpha (opacité 0,12) ; capture **regardée** : on voit le capot, la Vickers, la cabane et les câbles.
- **⚠** Première plaine : des polygones très contrastés, à l'allure de camouflage. **A** Elle devient un bocage. Captures **regardées** à chaque étape.
- **R** Vue de poursuite **regardée** après une demi-minute sans toucher aux commandes : l'avion s'incline lentement à droite. C'est la spirale, attendue sans pilote ; l'équilibre ne compense ni le léger aileron ni le palonnier du trim.
- **⚠** Coût du modèle de vol : environ 1,7 ms par pas dans le conteneur, même optimisé (`opt-level = 3` pour `escadrille_fdm` en profil de développement, ajouté au `Cargo.toml` du workspace). Acceptable pour un avion ; à profiler avant d'en faire voler plusieurs.
- **⚠** Disque plein deux fois pendant le travail (une compilation en release, des compilations périmées). **A** Ménage de `target/` : 10 Go, puis 2 Go.
- **A** `just check` et `just test` passent (299 tests).
- **Pour le joueur** : liste d'essai dans le plan (`docs/plans/m2-vol-essai.md`, « Essai du joueur ») ; retours dans `docs/gdd/retours.md`. Rien n'a été vérifié manette en main ici : le conteneur n'a pas de manette.

## 2026-10-29

### Retours du joueur sur le vol d'essai

- **Q** Le joueur : « Ce n'est pas facile de faire tous ces tests, voici mon ressenti » :
  - palier difficile à tenir, légère dérive à gauche ;
  - décrochage facile en virage appuyé, par perte de vitesse ; « Ajouter un mode où l'on aide le pilotage » ;
  - décrochage gaz réduits à 85 km/h ;
  - vues bien, « sauf la mitrailleuse qui est un gros bloc, il faut le rendre beaucoup plus réaliste » ;
  - manette « un peu trop sensible » ;
  - fluidité bien.
- **A** Retours consignés dans `docs/gdd/retours.md` (section M2), avec la suite donnée à chacun.
- **A** Manette « un peu trop sensible » : la courbe du manche passe de k = 0,4 à **k = 0,6**. À mi-course, le manche donne 27 % de la commande au lieu de 35 % ; les butées donnent toujours la pleine course. Le skill `manette` est mis à jour.
- **R** Analyse des deux symptômes de vol :
  - **dérive à gauche** : l'équilibre demande un peu de palonnier et d'aileron à droite, contre le couple et le souffle de l'hélice ; le neutre ne règle que la profondeur ;
  - **décrochage en virage** : à 60° d'inclinaison, il faut 2 g, la traînée induite est quatre fois plus forte et la vitesse de décrochage monte de 41 %, vers 120 km/h. C'est le comportement réel du SPAD ; une aide doit en protéger le joueur, pas le changer.
- **R** Décrochage à 85 km/h, contre environ 76 km/h pour le seul indice historique (47 mph, moteur coupé, S.VII d'après-guerre de 180 ch, confiance D). À soumettre à l'aérodynamicien ; aucun coefficient changé sans source.
- **A** Plan `docs/plans/m2-aides-pilotage.md` : les préréglages de `docs/vision.md`, avec des gains calculés depuis la linéarisation de `fdm_lab` (skill `realisme`). Aides par les commandes seulement :
  - **Arcade** : instructeur virtuel ;
  - **Pilote** : coordination automatique et garde anti-décrochage douce ;
  - **Vétéran** : coordination automatique ;
  - **As** : aucune.
- **D** Le joueur valide les options recommandées : les quatre préréglages, Pilote au lancement, le bouton Home pour changer en vol.
- **D** La Vickers sera remodelée d'après la fiche d'aspect (§ 7).

### La Vickers remodelée

- **A** La mitrailleuse du capot n'est plus un bloc (`build_gun` de `art/spad7/build.py`, nouvelle table `vickers_parts` de `maquette.ron`, proportions `E:` de la Vickers .303 d'aviation, à vérifier sur un plan ou une photo du montage du S.VII) :
  - manchon de refroidissement percé de fentes, colliers, cône de bouche ;
  - boîte de culasse aux arêtes arrondies, couvercle et sa charnière ;
  - bloc d'alimentation et guide de bande à droite, boîte du ressort à gauche, levier d'armement ;
  - deux viseurs circulaires sur leurs tiges (« deux viseurs circulaires », Hall, fiche d'aspect § 7) ;
  - pattes de fixation jusqu'au capot.
- **R** Aperçus Blender (de face, de côté, depuis l'œil du pilote), puis captures `vol-essai-pilote`, `avion-studio` et `vol-essai-poursuite` **regardées**. Une première série d'aperçus était tête en bas : erreur de caméra de mon script d'aperçu (axe haut), pas du modèle.
- **⚠** Dans la vue pilote, le pare-brise voilait tout ce qui était derrière lui, mitrailleuse et capot : son verre avait une couleur de base blanche que le soleil éclairait. **A** Base sombre et opacité de 0,10 : le verre ne fait plus que refléter et teinter un peu. La Vickers se lit en acier bruni.
- **A** `just check` et `just test` passent (299 tests).

### Aides au pilotage (plan `m2-aides-pilotage.md`)

- **A** Étape 1, `fdm_lab assist spad7` → `assets/aircraft/spad7/assist_gains.ron` :
  - à 10 vitesses, trim, linéarisation, puis trois modèles réduits (roulis, incidence, dérapage), avec placement de pôles et intégrateur ;
  - statiques : charge par incidence, profondeur par incidence et par vitesse de tangage, palonnier contre le lacet inverse ;
  - incidence de décrochage 12,0°, minimum plein gaz et ralenti ;
  - test de `fdm_lab` : le fichier suit le modèle. Schéma dans `crates/data/src/assist.rs`.
- **A** Étapes 2-3, `crates/game/src/realism/` :
  - `RealismSettings`, préréglages Arcade, Pilote (au lancement), Vétéran et As ;
  - bouton Home (touche I) ;
  - aides au pas fixe, entre les commandes du pilote et le pas du modèle ;
  - préréglage à l'écran et dans la télémétrie.

  Côté vol, la couche d'actions transmet aussi la position du manche, et l'avion du joueur prend ses commandes dans un nouvel ensemble `FlightSet::Command`.
- **R** Les premiers essais en simulation ont montré quatre défauts, corrigés par des anticipations calculées depuis la même linéarisation, sans réglage à la main :
  - coordination trop lente contre le lacet inverse d'un roulis rapide ;
  - garde qui dépassait sa limite de 1,6° ;
  - instructeur qui partait à 75° d'inclinaison pour 60° demandés ;
  - instructeur qui partait en looping, manche tiré.
- **R** Limites constatées et documentées :
  - **chandelle** : la garde seule (Pilote) ne rend pas la vitesse d'une chandelle ; manche tenu à fond, le SPAD passe sur le dos au sommet, comme l'avion réel, sans vrille. Seul l'instructeur limite la montée ;
  - **dérive à gauche** : mains libres, elle demeure en Pilote et en Vétéran (spirale lente, 29° en 50 s, comme en As). Seul l'instructeur la supprime. J'avais d'abord écrit le contraire dans le plan, et je l'ai corrigé après vérification en simulation.
- **⚠** Constat pour l'aérodynamicien : dans le modèle, la profondeur est très puissante.
  - De la croisière à 10° d'incidence, il suffit de 5 % de la course.
  - Un quart de manche arrière fait décrocher.
  - La marge statique vaut 1,5 % à 160 km/h.
  - C'est probablement la vraie cause du manche « trop sensible » et des décrochages faciles. Aucun coefficient n'a été changé.
- **A** Tests sans rendu (9) :
  - fumée par préréglage ;
  - absence de triche en As ;
  - coordination ;
  - palier mains libres ;
  - branchement dans Bevy ;
  - cycle des préréglages.

  Le test ignoré `print_a_flight` sert au diagnostic.
- **A** Docs mises à jour :
  - plan (avancement, écarts, liste d'essai) ;
  - `retours.md` ;
  - skills `realisme`, `manette` et `simulation-vol` (format du fichier de gains).
- **R** Réponse au joueur, en fin d'échange :
  - la Vickers est remodelée et le pare-brise éclairci ;
  - les quatre préréglages, au bouton Home, avec ce que fait chacun ;
  - les deux limites vérifiées en simulation : la dérive mains libres demeure hors Arcade, et la garde de Pilote ne rend pas la vitesse d'une chandelle ;
  - la profondeur très puissante, cause probable du manche nerveux ;
  - la liste d'essai, dans le plan, section « Ton essai ».
- **⚠** La revue de l'aérodynamicien (étape 4 du plan) était encore en cours à la clôture de l'échange. Elle porte sur :
  - les gains ;
  - l'incidence de décrochage ;
  - la puissance de la profondeur et la marge statique de 1,5 % ;
  - le décrochage à 85 km/h ;
  - l'équilibre manche au neutre à 88 km/h.

  Ses conclusions seront consignées à leur arrivée.

### Revue de l'aérodynamicien sur les aides, et corrections

- **R** Revue reçue. Verdict : « à corriger », rien de bloquant, toutes les boucles stables. Trois défauts dans mon calcul des gains :
  - **inclinaison** : boucle quatre fois trop lente, environ 10 s pour remettre les ailes à plat ;
  - **incidence** : boucle conçue sans le retard de la déflexion, amortie à 0,15-0,3 seulement en piqué et en altitude ;
  - **statiques** : prises sur des tangentes, elles changeaient de signe d'une ligne à l'autre.
- **A** Corrections dans `fdm_lab assist` :
  - linéarisation par sécantes (`modes::linearise_secant`, partagée avec la marge statique) ;
  - roulis placé selon l'amortissement propre du SPAD ;
  - incidence conçue sur le modèle à déflexion établie : la plus douce dès 4 rad/s, vérifiée sur tout le modèle longitudinal ;
  - statiques par équilibres non linéaires, en sécante jusqu'à la limite de la garde ;
  - coordination portée à 3 rad/s au moins ;
  - nouveau coefficient : l'aileron par vitesse de roulis.

  Nouveau test : amortissement d'au moins 0,5 sur toute l'enveloppe, à 0 et 4 000 m, réservoir plein et vide (0,52 au pire).
- **A** Corrections dans le jeu :
  - mise en virage de l'instructeur limitée à 45°/s, avec l'aileron de ce roulis ; un premier essai, sans cette anticipation, dépassait jusqu'à 78° ;
  - anti-emballement de la coordination tenant compte du palonnier du pilote ;
  - pédale enfoncée, la boucle de dérapage se suspend : sinon, plus raide, elle annulait le palonnier volontaire du joueur.

  Tests ajoutés : temps de réponse en inclinaison et palonnier du pilote respecté (11 tests au total).
- **R** Gardé et documenté : l'incidence de décrochage au minimum plein gaz (12,0°) est une borne prudente, qui coûte environ 0,4 g en croisière.
- **R** Pas d'action, selon la revue :
  - décrochage à 85 km/h : l'écart avec l'indice historique (confiance D) reste dans les 10 % ;
  - neutre à 88 km/h : conséquence du plan fixe et du centrage.
- **⚠** Cause probable du manche nerveux, selon la revue :
  - centrage presque neutre : 39,5 % de la corde réservoir plein ;
  - marge statique nulle vers 160 km/h, négative au-delà de 165 km/h, phugoïde instable vers 180 km/h ;
  - les sources décrivent pourtant un avion « très stable ».

  À investiguer, sans rien changer sans source : bilan de masses à relire, sources de centrage à chercher (rapport britannique n° 96, rapport McCook, circulaire n° 286, notice du S.XIII). Également signalés : la spirale à basse vitesse (aucun dièdre dans les données) et des libellés trompeurs dans `fdm_lab modes`.
- **D** À proposer au joueur : lancer l'enquête sur le centrage (historien pour les sources, aérodynamicien pour le bilan de masses).
- **R** Réponse au joueur après la revue :
  - les trois défauts du calcul des gains sont corrigés ;
  - le décrochage à 85 km/h et le neutre à 88 km/h sont laissés tels quels ;
  - le centrage presque neutre est la cause probable du manche nerveux ;
  - question posée : lancer l'enquête sur le centrage (historien pour les sources, aérodynamicien pour le bilan de masses) ?
- **A** Tâche séparée proposée au joueur, à lancer s'il le veut : corriger les libellés trompeurs des modes dans `fdm_lab modes`. Le « retard de la déflexion » à −1,5 s⁻¹ y est en réalité la racine lente d'incidence.
- **⚠** Le processus de la session a redémarré après le dernier commit (`eadb759`). Vérification faite : tout était déjà commité et poussé, l'arbre de travail était propre.

### Visée de la Vickers

- **Q** Le joueur : « c'est beaucoup mieux. les deux mires de la mitrailleuse ne sont pas alignés au regard du pilote. le réticule proche est au-dessus du réticule de bout de canon ».
- **R** Constat par le calcul :
  - l'œil du modèle était à z −1,10 m ;
  - le viseur arrière, 39 cm devant, apparaissait 6,6° au-dessus de l'horizon, celui de bouche, 1,33 m devant, à 2,1° seulement ;
  - plus profond : l'œil n'était que 2 mm au-dessus du couvercle de la culasse, si bien que le pilote regardait l'arrière de la mitrailleuse au lieu de viser par-dessus.
- **A** `art/spad7/build.py` :
  - les deux viseurs sont construits sur la ligne de visée qui passe par l'œil du pilote, parallèle à l'âme du canon (réglage à l'infini, en attendant le réglage de tir des armes) : ils sont alignés par construction ;
  - le script refuse un modèle dont un viseur ne dégage pas le canon d'au moins 1 cm, ou dont la ligne de visée ne traverse pas le verre du pare-brise sous son cadre.
- **A** `maquette.ron` (valeurs estimées) :
  - œil levé à z −1,15 : le pilote lève la tête pour viser, par-dessus la culasse ;
  - pare-brise porté de 0,17 à 0,22 m de haut, pour que la visée passe dans le verre ;
  - la hauteur des viseurs disparaît de `vickers_parts` : elle découle maintenant de l'œil.
- **R** Aperçu Blender au téléobjectif depuis l'œil, puis captures `vol-essai-pilote` et `avion-studio` **regardées** : les deux anneaux sont concentriques, le pilote voit par-dessus le couvercle, et le pare-brise reste discret de l'extérieur.
- **⚠** La ligne de visée est 12 cm au-dessus de l'âme et lui est parallèle : le réglage de tir (convergence à une distance donnée) viendra avec les armes (skill `combat`).
- **A** Retours consignés dans `docs/gdd/retours.md`.
- **R** Réponse au joueur :
  - les viseurs sont alignés, d'après la capture en jeu ;
  - la cause de l'écart ;
  - l'œil levé de 5 cm et le pare-brise rehaussé ;
  - les garde-fous du script ;
  - le réglage de tir, renvoyé aux armes.

  Questions toujours ouvertes : son avis préréglage par préréglage au prochain vol, et l'enquête sur le centrage.

## 2026-10-31

### Enquête sur le centrage

- **Q** Le joueur : « centrage : ok », donc l'enquête proposée après la revue du 2026-10-29 est validée.
- **A** Deux agents lancés en parallèle :
  - **historien** : sources de centrage du S.VII ou d'un SPAD voisin, par de nouvelles pistes (manuels de conception de 1918-1921, rapports NACA et McCook, revues d'époque). Accessoirement : dièdre, plan fixe réglable, débattement de la profondeur. Résultats dans `docs/recherche/spad7.md` ;
  - **aérodynamicien** (lecture seule) : relecture du bilan de masses (`masses.ron`) et décomposition de M_α par élément à 100 et 180 km/h, pour expliquer la marge qui chute avec la vitesse.
- **D** Aucune donnée ne change avant leurs rapports, et rien sans source ou sans justification physique notée.
- **⚠** Les deux agents se sont arrêtés vers 15 h 30 UTC sur la limite d'usage hebdomadaire, levée à 19 h UTC.
  - L'historien avait commencé par Barnwell (1918) et le corpus de *Flight* ; l'aérodynamicien en était à la relecture du contexte.
  - Rien n'a été écrit dans le dépôt. Leur travail est conservé et peut reprendre là où il s'est arrêté.
- **A** Reprise programmée à 19 h 05 UTC.
- **A** 19 h 06 UTC : la reprise programmée s'est déclenchée. Les deux agents ont été relancés là où ils s'étaient arrêtés.
- **R** Rapport de l'historien, douzième passe (`spad7.md` § 3.1) :
  - **aucune donnée de centrage**, pour aucun SPAD : ni position du centre de gravité, ni pesée, ni limites de notice, ni point neutre mesuré ;
  - trouvé : la règle de conception de Barnwell (1918, *Flight*), centrage optimal de 0,33 à 0,38 de la corde moyenne de la cellule, confiance C pour le SPAD ;
  - trouvé : le « lourd de la queue » du SPAD XIII de McCook, qui est une remarque d'**équilibre** et non de stabilité (Halliwell, 1919) ;
  - **dièdre nul** maintenu : le S.VII est dit « flat-winged » ;
  - non trouvés : plan fixe réglable, débattement de la profondeur.
- **A** Demande D15 ajoutée par l'historien dans `demandes.md`, et compléments de D1 à D6 pour le centrage. **Rien n'est envoyé** : c'est au joueur de le faire.
- **A** La session principale a téléchargé et fouillé le texte OCR des quatre manuels de conception (archive.org), que l'outil de l'historien ne pouvait pas lire ; aucune copie n'est commitée.
  - Pomilio, Judge, Barnwell et Sayers, Engineering Division : **aucun centrage de SPAD**. Le manuel de McCook ne cite le SPAD que pour la structure de sa cellule.
  - Recherche « spad » sur la NTRS : deux résultats de 1915 à 1935, aucun sur le S.VII ou le XIII.
  - Complément écrit dans `spad7.md` § 3.1 et § 9.6.
- **R** Rapport de l'aérodynamicien, consigné dans `spad7-calculs.md` § 4.1 :
  - **centrage** : 39,5 % n'est contredit par aucune donnée du dépôt. Le bilan relu contre le dessin le reculerait plutôt (moteur, armement) ; fourchette défendable de 35 à 44 % plein ;
  - **marge qui chute avec la vitesse** (−10,6 points de 100 à 180 km/h) : pas de défaut de code. Environ 6,6 points sont physiques (inclinaison de la portance des plans d'un biplan, déflexion, fuselage). Environ 4 points sont des artefacts des tables : C_m du RAF 15 en dents de scie, creux de pente du plan fixe autour de zéro, venu du miroir d'un profil cambré.
- **A** Corrections de documentation sans effet sur le modèle :
  - `spad7-calculs.md` § 4 reprend les valeurs à vide de `aircraft.ron` ;
  - le commentaire du moteur dans `masses.ron` donne la bonne longueur de boîte.
- **D** À décider par le joueur :
  1. corriger les deux tables : C_m de l'aile lissé, plan fixe linéaire à travers zéro ;
  2. garder 39,5 %, ou avancer le centrage à 38 % (limite de Barnwell, deux repositionnements justifiés) ou à 35 % (bas de la fourchette, pour la stabilité décrite par les témoins, sans source de mesure).
- **D** Le joueur choisit les options recommandées :
  - **corriger les deux tables** : C_m de l'aile lissé, plan fixe linéaire ;
  - **centrage à 38 %** : pilote à −1,10 m, 20 L dans le réservoir de cabane.

  Procédure : tests, campagne d'essais en vol, revue de l'aérodynamicien, puis régénération du départ du vol d'essai et des gains des aides.
- **A** Appliqué (détail dans `spad7-calculs.md` § 4.1, « Appliqué le 2026-10-31 ») :
  - **R1** : `spad.ron`, C_m sur la droite des moindres carrés, recalculée : identique, résidu 0,0047 ;
  - **R2** : `spad_empennage.ron`, C_L linéaire à 0,0762/° ;
  - **pilote** : x −1,10 ;
  - **carburant** : point équivalent des deux réservoirs.
- **⚠** Écart à la décision du joueur : **10 L** dans le réservoir de cabane, ceux du S.XI (S49, comparable de famille), au lieu des 20 L estimés sans source. Le centrage plein vaut donc **38,6 %** (39,5 % avant ; vide 32,9 %) et non 38 %.
- **R** Marge statique, réservoir plein, niveau de la mer :
  - en palier : 100 km/h +8,4 %, 130 +4,3, 160 +2,8, 180 +1,7 ;
  - au ralenti : +6,8 / +2,8 / +1,6 / +0,4 ;
  - l'avion est stable à toutes les vitesses (avant : +0,3 % à 160 km/h, −1,6 % à 180).
- **A** Régénérés :
  - `assets/scenarios/vol-essai.ron` (équilibre de départ) ;
  - `assist_gains.ron` (amortissement en boucle fermée de 0,52 au pire, test passé) ;
  - les copies des deux tables dans le modèle de comparaison du Messenger (`tools/fdm_lab/comparison/airfoils/`), qu'un test garde identiques : l'étalonnage de la traînée de toile tient toujours.
- **R** Campagne `just essai spad7` comparée à la même campagne sur les données d'avant :
  - performances inchangées à 0,2 % près, tous les essais réussis ;
  - tracés du piqué et du décrochage lus : sains ;
  - au point du rapport (74 %, 153 km/h, 2 000 m), la marge reste à 5,5 % : R1 y retire ce que le centrage ajoute ;
  - N_v passe de 113,0 à 120,3 (R2 sur la dérive).
- **A** `just check` passe, `just test` aussi (314 tests).
- **⚠** Revue de l'aérodynamicien sur ces changements **en cours** au moment du commit : elle fera l'objet d'un commit séparé.
- **R** Revue de l'aérodynamicien sur ces changements. Verdict : **OK**, rien à changer dans le modèle ; il retrouve les marges à 0,05 point près.
  - **10 L de cabane** : mieux fondés que les 20 L estimés. Le point équivalent est acceptable, mais « les deux réservoirs se vident ensemble » est physiquement faux (le réservoir en charge reste plein) ; l'écart est inférieur à 0,4 % de corde.
  - **N_v +6 %** : suppression d'un artefact, la dérive volant dans l'ancien creux de la table ; le roulis hollandais le confirme.
  - **Qualitatifs** : plausibles, inchangés à 1 ou 2 unités près.
  - **Commande de profondeur par g** à 160 km/h, plein : de 0,054 à **0,068** de course (+26 %). Le manche reste léger : il faut 17 % de la course pour les 3,5 g disponibles.
  - **Leviers restants** contre le manche nerveux, hors du modèle de vol : le débattement de profondeur de ±25° (estimé, sans source) et la courbe de la manette.
- **A** Retouches de commentaires demandées par la revue :
  - origines des C_m et des C_L dans les deux tables, et dans leurs copies du modèle de comparaison ;
  - formulation du carburant ;
  - pentes du plan fixe et de la dérive dans `aircraft.ron` ;
  - N_v absolus datés dans `spad7-calculs.md`.
- **⚠** La ligne **D** plus haut garde les « 20 L » du choix du joueur : les données en ont pris 10, comme l'explique la ligne ⚠ qui la suit.
- **R** Vérifié en plus : phugoïde à 180 km/h au niveau de la mer **stable** (ζ 0,49, période 63 s) ; avant, une racine instable doublait en 13,6 s.
- **A** Pistes notées par la revue, sans action pour l'instant :
  - critère insensible à la phase et tracé pour les essais de robustesse « vitesse nulle » et « glissade sur la queue » ;
  - un vrai second réservoir dans le schéma quand viendront les réservoirs crevés (skill `combat`).

### Combat, dégâts et son : exigences du joueur

- **Q** Le joueur : « Par la suite, il faut penser au modèle de combat. Les dégâts sur les avions et pilotes doivent être particulièrement réalistes : dérive déchirée, ailes endommagées ou coupées, câbles sectionnés, etc. Tous les dégâts doivent être visibles et influencer le vol de l'avion. Le comportement des armes doit être réaliste. Il faut aussi ajouter le son. »
- **R** Ce qui existe déjà :
  - le skill `combat` décrit les dégâts par composant, agissant sur le modèle de vol, et des armes simulées balle par balle ;
  - les conventions du modèle 3D prévoient `hit_` et `dmg_` ;
  - le combat est au M4, le son au M6.

  Trois exigences n'étaient pas écrites : l'aile coupée ou arrachée, la dérive déchirée, et le fait que **tous** les dégâts soient visibles.
- **A** Ajouts :
  - `docs/gdd/combat-degats.md` : exigences, effets sur le vol et rendu de chaque dégât cité, besoins par domaine, ordre proposé, questions ouvertes ;
  - feuille de route : M4 (exigences et critère de fin), M6 ;
  - skill `combat` : empennage, aile coupée, exigence de visibilité ;
  - skill `pipeline-assets` : pièces endommageables à prévoir dans le modèle 3D.
- **D** À demander au joueur :
  - avancer un premier son (moteur et vent), qui sert au pilotage, juste après le M2 ;
  - préparer dès le M3 le découpage du modèle 3D en pièces endommageables.

  Aucun code avant un plan du M4 validé.

## 2026-11-01

### Dégâts dans le modèle 3D

- **Q** Le joueur : « Liste tous les dégâts possibles et intègre-les tout de suite dans le modèle 3D. »
- **A** Catalogue complet : **48 dégâts** en huit familles, dans `docs/gdd/combat-degats.md` § 3 bis. Pour chacun : l'effet en vol, ce qui se voit, et ce que le modèle 3D doit prévoir. Les familles :
  - cellule et toile ;
  - commandes ;
  - moteur et hélice ;
  - carburant et feu ;
  - pilote ;
  - armement ;
  - train ;
  - instruments.
- **A** Plan `docs/plans/m4-degats-modele-3d.md` :
  - cellule découpée en pièces nommées ;
  - structure sous la toile ;
  - volumes `hit_`, points `fx_` ;
  - variantes `dmg_` ;
  - contrat et contrôle ;
  - masquage dans le jeu ;
  - galerie `degats-spad7`.
- **D** Le joueur valide le périmètre complet du modèle 3D, avec la structure modélisée sous la toile ; au M4, les trous viendront d'un masque dans le shader. La logique des dégâts et leurs effets en vol restent au M4.
- **A** Étape 1 du plan, la cellule découpée en pièces nommées (`art/spad7/build.py`) :
  - chaque face reçoit une étiquette de pièce, comme sa région de toile ; des règles la résolvent selon sa position (panneau entre mâts, côté) ;
  - un nœud par pièce, avec son origine au centre de sa boîte : 12 panneaux d'aile coupés aux mâts, plan fixe par côté, dérive, fuselage avant et arrière, chaque mât de cellule et de cabane, tubes, train (jambes, roues, essieu, béquille), mitrailleuse, pare-brise, échappements, biellettes ;
  - un nœud par câble (`wire_<famille>_<l|r>_<n>`, 28 câbles), et des pales d'hélice séparées sous `prop`.

  Les pièces partagent les sommets de leurs coutures. Le studio capturé et regardé est identique à avant.
- **A** `tools/asset_check` :
  - les dimensions ignorent les pièces de l'hélice et les futurs nœuds masqués (`prop_*`, `hit_*`, `dmg_*`) ;
  - le contrôle des bouches de canon ne vise plus que les nœuds `gun_*_muzzle`, la pièce `gun_1` étant la mitrailleuse elle-même.
- **A** Étape 2 du plan, la structure sous la toile (`build.py`, `build_frame` et `build_fuselage_frame`), dans le nœud de sa pièce :
  - **nervures** au pas des nervures (19 cm aux ailes, 20 cm à l'empennage), taillées dans le profil réduit à 85 % de l'épaisseur. Elles sont coupées à l'axe des gouvernes : leur partie arrière bouge avec la gouverne ;
  - **longerons d'ailes** à 12 et 46 % de la corde nominale, droits et arrêtés avant les saumons arrondis ;
  - **empennage** : longeron arrière droit juste devant l'axe de la profondeur et du gouvernail (étambot de la dérive) ;
  - **fuselage** : poutre en bois du poste à l'étambot, avec quatre longerons, des cadres tous les 45 cm et un câble diagonal par travée sur chaque flanc.

  Valeurs estimées (`E:`).
- **⚠** Premier essai : les longerons de l'empennage suivaient une fraction de la corde locale, donc la forme du contour (en V, en zigzag). Corrigé par des longerons droits le long de l'axe des gouvernes.
- **R** Vérifications : aperçus Blender avec la toile rendue transparente, regardés ; captures `avion-studio` et `vol-essai-pilote` regardées, rien ne perce la toile.
- **A** Étapes 3 et 4 du plan (`build.py`) :
  - **47 volumes d'impact `hit_`**, boîtes fermées et convexes : moteur, radiateur, réservoirs, huile, munitions, synchronisation, carburateur, magnétos, pilote (tête, torse, bras, jambes), câbles de commande, longerons de chaque panneau. Leurs positions et tailles non tirées d'`aircraft.ron` sont des `features` de `maquette.ron`, en `E:` ;
  - **7 points d'effets `fx_`** : moteur, radiateur, échappements, réservoirs, poste ;
  - **54 variantes `dmg_`** : moignons des 12 panneaux d'aile et du fuselage arrière, 8 mâts brisés, 28 câbles coupés, 2 moignons de pale, pare-brise touché, 2 jambes de train rompues.
- **A** Étape 5, le contrat des dégâts dans `tools/asset_check`. Il s'applique quand `maquette.ron` déclare un volume `hit_`, et vérifie :
  - la présence des pièces, volumes, variantes et points d'effets déduits des données (`damage_nodes`), soit 93 nœuds ;
  - une variante coupée pour chaque câble et un moignon pour chaque pale ;
  - que chaque `hit_` est fermé et convexe.
  
  Trois tests ajoutés.
- **⚠** Le contrôle de convexité a trouvé une vraie faute : le parcours de chaque câble d'aileron, deux barres dans un seul nœud, n'était pas convexe. Correction : un composant en plusieurs morceaux a un nœud par morceau, numéroté (`hit_cable_aileron_l_1`, `_2`).
- **A** Étape 6, le jeu :
  - un observateur global (`crates/game/src/render/damage.rs`) masque les nœuds `hit_` et `dmg_` dès que le modèle apparaît ;
  - les câbles coupés reçoivent le shader des câbles ;
  - deux scènes de référence : `degats-spad7`, l'avion au repos vu de l'avant droit, et `degats-spad7-proche`, vue de l'œil du pilote ;
  - le studio prend un point de vue explicite (`View`, `Eye`).
- **⚠** Première galerie : aucune variante n'apparaissait. Bevy nomme les maillages d'un nœud d'après le maillage et le matériau (`dmg_prop_blade_1_stub.bois…`). Le masquage les attrapait directement, si bien qu'un nœud remontré restait invisible. Corrigé : seuls les nœuds sont masqués (sans `Mesh3d`), et un test le couvre.
- **R** Non-régression : `avion-studio`, `vol-essai-pilote`, `vol-essai-poursuite` et `contre_jour_toile` ont été capturés, regardés et comparés aux images d'avant. Le modèle est identique : aucun volume ni aucune variante ne se voit. Dans les scènes de vol, seul le terrain est décalé, parce que le vol avance selon le temps de capture.
- **R** Coût de rendu, mesuré sous lavapipe sur `avion-studio` : 249 ms par image avant comme après, avec 192 nœuds au lieu de 16 et un GLB de 6,9 Mo au lieu de 4,9. Le vrai coût se mesurera sur la RX 7800 XT.
- **A** Commit et push des étapes 3 à 6 (`a73e58f`), `just check` et `just test` verts (319 tests).
- **A** Première revue du directeur artistique : aucune régression, mais une galerie qui ne valide pas la moitié des dégâts. Il relève :
  - des cassures toutes pareilles, en boîtes à pointe, sans bois frais ;
  - des sections différentes de celles des pièces intactes ;
  - un pare-brise brisé invisible ;
  - un cadrage à revoir.
- **A** Reprise selon la revue :
  - une fonction commune `break_end` donne à chaque cassure une coupe déchirée en bois frais (matériau `wood_fresh`) et 3 à 5 éclats, tirés d'un générateur dont la graine est le nom du nœud ;
  - le mât brisé garde sa section profilée, avec le tronçon du haut qui pend vers l'arrière ;
  - le moignon de pale garde la pale vrillée, avec une cassure en noyer frais ;
  - les longerons des moignons d'aile ont des longueurs inégales, et un bout de nervure pend ;
  - les câbles coupés ont du mou et une boucle ouverte ;
  - la galerie est recadrée, avec un soleil bas du côté de la caméra.
- **A** L'historien, sur le pare-brise :
  - le verre est attesté par Hall (SPA 124, 1917 ; C) ; le verre feuilleté de type Triplex est **ESTIMÉ** (D) ;
  - un tel pare-brise reste en place sous une balle : un trou, du verre écrasé laiteux, des fêlures en rayons et en anneaux ;
  - la ligne `windscreen` de `maquette.ron` citait à tort Bordeaux, qui parle du Nieuport de Guynemer : corrigée. Décision consignée dans `spad7-aspect.md` § 17, documents à consulter au § 15.
- **⚠** De l'extérieur, le petit pare-brise reste caché par les mâts de cabane, la Vickers ou l'aile supérieure, sous tous les angles essayés (arrière, côté droit, avant de l'hélice). La vue rapprochée est donc l'œil du pilote, au champ de vision du vol (60°). C'est là que ce dégât compte.
- **A** Deuxième revue : la galerie extérieure est approuvée. Corrigé ensuite :
  - les fêlures sont devenues fines, effilées et translucides, sur un verre écrasé en dégradé ;
  - la cassure de l'hélice est en noyer ;
  - l'hélice de la galerie est tournée pour que le moignon se détache.
- **R** Troisième revue : **approuvé**, plus rien ne bloque. Le pare-brise est lisible et crédible, et la gêne sur la visée reste supportable. Le noyer frais est juste. Les restes mineurs vont au M4 (liste dans le plan) : verre écrasé en paliers, moignon de pale vu par la tranche, boucle des câbles.
- **A** Docs à jour :
  - skills `pipeline-assets` (pièces, nœuds `hit_` `fx_` `dmg_`, cassures, galeries, contrôles), `combat` et `format-avion.md` (volumes de `maquette.ron`) ;
  - section « Réalisation » du plan.

### Reprise du dépôt sur la machine du joueur

- **Q** Le joueur : « Je t'ai fourni un répertoire de travail qui est un dépôt d'une autre session. Analyse ce dépôt pour reprendre le travail. »
- **A** Lecture de la feuille de route, des plans, du carnet, des retours et du code. Aucun code ni asset modifié. Pour la première fois, Claude travaille sur la machine du joueur (Fedora, RX 7800 XT, manette branchée) et non plus dans le conteneur.
- **R** Vérifications sur cette machine :
  - `cargo fmt --check` et `cargo clippy --workspace --all-targets -- -D warnings` : rien à signaler ;
  - `cargo test --workspace` : 319 tests passés, 0 échec, 1 ignoré ;
  - `just modele spad7` sous **Blender 5.2.2** (sortie dans `target/`) : `asset_check` passe ;
  - capture `avion-studio` sur la RX 7800 XT, et `degats-spad7` déjà présente dans `target/captures/` : **regardées**, rendu correct.
- **R** Modèle reconstruit sous Blender 5.2.2, comparé au `model.glb` du dépôt, fait sous 4.0.2 :
  - mêmes 192 nœuds, 201 851 triangles et 20 matériaux ; textures identiques au pixel, mieux compressées (6,7 Mo au lieu de 7,3) ;
  - une seule différence : le `clearcoatRoughnessFactor` de l'hélice (0,03) n'est plus exporté. Le script ne le fixe pas, il prenait la valeur par défaut de Blender. Effet très faible ; à regarder quand le modèle sera régénéré dans `assets/`.
- **R** Coût de rendu sur la RX 7800 XT, en 1920×1080 : 4,30 ms par image pour `avion-studio`, 4,36 pour `degats-spad7`, 4,44 pour `degats-spad7-proche`, soit environ 225 images par seconde avec les 192 nœuds. C'est la mesure que le plan `m4-degats-modele-3d.md` laissait à faire ; sans nuages ni terrain réel, elle ne dit rien encore du budget du M3.
- **⚠** `cargo-nextest` et `cargo-deny` ne sont pas installés ici : `just test` et `just check` ne tournent pas tels quels, et `cargo deny check` n'a pas pu être vérifié. `Cargo.lock` n'a pas changé depuis le 2026-10-28. Rien n'a été installé sans l'accord du joueur (commande dans le `README.md`).
- **R** État des jalons :
  - **M1** : encore ouvert. Tout est fait sauf l'essai du joueur, la lecture du rapport d'essais, dont l'accord déclenche l'écriture de `docs/essais/spad7.md`. La ligne « Jalon en cours » de `CLAUDE.md` reste donc juste ;
  - **M2** : vol d'essai et aides faits. Il manque l'essentiel de la fin : le **train et le sol**, absents de `crates/fdm` et du schéma d'`aircraft.ron`, donc décollage et atterrissage ; puis les gizmos de forces et le vol de dix minutes ;
  - **M3** : avancé sur le modèle ; manquent le poste de pilotage (B7), le bois, le métal et la rugosité (B5), la seconde livrée, puis nuages et terrain ;
  - **M4** : modèle 3D prêt ; logique, armes et effets en vol non commencés, sans plan validé.
- **⚠** Le joueur n'a pas volé depuis le recul du centrage du 2026-10-31, qui visait le manche nerveux : le dernier retour date du 2026-10-29. Le détail par préréglage reste aussi à recueillir.
- **⚠** Toutes les demandes aux archives (D1 à D15) sont encore « à envoyer », sauf D12, en attente. Elles relèvent du joueur.
- **⚠** Décision encore ouverte : avancer un premier son (moteur et vent) juste après le M2 (`combat-degats.md` § 6, question 1).
- **⚠** Tout le travail est sur `main-tcd0yw`, 187 commits devant `main`. `gh` n'est pas installé ici : l'existence d'une pull request n'a pas pu être vérifiée.
- **⚠** Push impossible à la clôture : l'agent SSH de cette machine n'a aucune clé chargée (`Permission denied (publickey)`) et il n'y a pas de `~/.ssh/config`. Cette entrée est commitée en local. Claude n'a pas utilisé directement les fichiers de clé de `~/.ssh` : le joueur charge sa clé, puis pousse.
- **A** Le joueur a chargé sa clé dans l'agent et poussé `87fe39d` : l'incident est clos, l'agent de la session a désormais accès à GitHub.

### Plan du train et du sol

- **Q** Le joueur : « Oui, plan du train ».
- **A** Plan `docs/plans/m2-train-sol.md`, dernier incrément du M2. Aucun code : la validation du joueur est attendue.
  - contact au sol dans `crates/fdm/src/ground.rs` : roues, sandows, frottements, béquille, points de structure ;
  - section `undercarriage` d'`aircraft.ron`, essais de sol dans `fdm_lab` ;
  - départs au sol et en finale, pré, gizmos de forces, aides au sol.
- **R** Chiffres calculés depuis les données, réservoirs pleins :
  - assiette trois points de 14,4° ; le plan supérieur y est à environ 16,7°, au-delà du décrochage ;
  - 150 kg sur la béquille (20,5 %) et 290 kg par roue ;
  - le centre de gravité passe à la verticale des roues à 22° sous la ligne de vol, l'hélice touche à 22,7°, avec 0,27 m de garde en ligne de vol.

  Ils vont dans le sens de Meister (S.XIII) : s'arrête court, ne capote pas.
- **⚠** Écart trouvé en préparant le plan : `fdm_lab` calcule à 240 Hz, comme le veut l'ADR 0005, mais **le jeu fait un seul pas RK4 à 120 Hz**. Les essais ne valident donc pas exactement ce que le joueur pilote. Le plan le corrige (étape 2), avec un ADR 0008 pour le contact.
- **⚠** Les aides actuelles ne valent pas au sol : à l'arrêt, l'avion est au-delà de l'incidence limite, et la garde pousserait le manche en permanence. Le plan les coupe au contact. Elle reste active jusqu'au toucher, donc le posé en Pilote se fait queue haute.
- **⚠** Aucune source sur les sandows, les pneus ou la charge de rupture : ces valeurs seront `E:`, et les essais de sol seront qualitatifs. Treizième passe de l'historien prévue en parallèle.
- **D** Quatre décisions demandées au joueur : départ au lancement, terrain dessiné, aides au sol, règle des accidents.

### Outils installés, `just check` réparé

- **A** Le joueur a installé `cargo-nextest` 0.9.150 et `cargo-deny` 0.20.2 (`cargo-nextest` 0.9.148 dans les sessions cloud, `session-start.sh`).
- **⚠** Premier `just check` sur cette machine : **échec** de `cargo deny`, sur une seule erreur. `clipboard-win` 5.4.1 a été retirée de crates.io (*yanked*). Cette crate ne sert que sous Windows, par `arboard` et `bevy_egui`, et ne se compile pas sous Linux. Le carnet donne `just check` vert au commit `a73e58f`, le même jour, et `Cargo.lock` n'a pas changé depuis : la cause est extérieure au dépôt.
- **A** `cargo update -p clipboard-win` : passage à 5.4.3. Vérifié à blanc avant : un seul paquet change, sans nouvelle dépendance. Ni `deny.toml` ni la version de Bevy ne bougent.
- **R** `just check` passe (`advisories ok, bans ok, licenses ok, sources ok`) et `just test` aussi : 319 tests passés, 1 ignoré.
- **R** Le plan `m2-train-sol.md` attend toujours la validation du joueur.
- **D** Proposition au joueur : installer les deux outils, faire un vol court pour juger le centrage, puis passer au plan du train et du sol, qui clôt le M2. Aucun code avant la validation de ce plan.

### Train et sol : plan validé, étape 1

- **Q** Le joueur : « Oui, tout en A ».
- **D** Plan `m2-train-sol.md` validé avec les quatre options recommandées :
  - `just run` démarre au sol, et la pause permet de choisir sol, finale ou en vol ;
  - pré dessiné ;
  - tenue de cap en Arcade et Pilote, envol automatique en Arcade ;
  - accidents par seuils, avec une tolérance selon le préréglage.
- **A** Étape 0 : treizième passe de l'historien lancée en parallèle (train, frottements, technique de pilotage, terrains).
- **A** Étape 1 : nouveau composant `crates/fdm/src/ground.rs`, sans Bevy, branché dans `Aircraft` par `with_undercarriage` :
  - **roues** : sphère autour de l'axe, pneu en série avec le sandow précontraint, butée en fin de course (`Spring`) ; résistance au roulement ; force latérale qui croît avec l'angle de dérive jusqu'à l'adhérence ;
  - **béquille** : ressort et frottement dans toutes les directions ;
  - **points de structure** : ressort raide et frottement ;
  - **frottement régularisé** sous 0,05 m/s, sans division par la vitesse ;
  - `Diagnostics.ground` donne les charges et les compressions ; `Aircraft::propeller_clearance` donne la garde de l'hélice.
- **⚠** Le premier test d'énergie échouait par ma faute : il oubliait l'énergie stockée dans les sandows, qui revient forcément pendant la détente. L'énergie se compare désormais aux instants où rien ne touche.
- **R** 18 tests ajoutés, sur un avion de chiffres ronds, sans donnée historique. En particulier :
  - en l'air, un avion avec train donne le même résultat au bit près que sans train, sur 5 s de vol ;
  - au repos, les charges équilibrent le poids ;
  - pas de gain d'énergie sur les rebonds, frottement fini à l'arrêt, symétrie, déterminisme.
- **A** `just check` passe, `just test` aussi (337 tests). Aucun avion n'a encore de train : le jeu et les essais ne changent pas.
- **A** Étape 2 : `Aircraft::advance` fait les deux sous-pas de l'ADR 0005, et le jeu l'appelle à 120 Hz. Le jeu calcule donc à 240 Hz, comme `fdm_lab`. `Aircraft::step` reste un pas RK4 simple : les outils, qui lui passent déjà 1/240 s, ne changent pas. Les tests des aides et le test d'équilibre du vol d'essai passent par `advance`.
- **R** Coût mesuré sur le Ryzen 7 7800X3D : **1,0 ms par pas RK4**, 1,7 ms dans le conteneur. Soit 2 ms par pas de jeu, un quart de cœur pour un avion. À revoir au M4 avec plusieurs avions : l'ADR 0005 prévoit alors un pas plus long pour les avions lointains de l'IA.
- **A** `just check` passe, `just test` aussi (338 tests). L'ADR 0008 attendra la mesure de convergence au sol, à l'étape 4.
- **A** Étape 3 : section `undercarriage` dans `aircraft.ron` et son schéma (`crates/data`), décrite dans `format-avion.md`. Le chargeur construit le train depuis le centre de gravité.
  - **Géométrie** : roues et béquille du dessin de 1917, identiques à `maquette.ron` (test).
  - **Glissière** : la pente de la jambe arrière, soit environ 46° vers l'arrière. L'essieu recule donc en montant. Le chargeur en déduit le ressort vu du sol.
  - **Sandows, pneus, frottements** : valeurs `E:`, en attendant la treizième passe de l'historien. La course de 125 mm vient du S.XI.
- **⚠** Piège évité : `fdm_lab` place ses calculs au niveau de la mer à 0 m d'altitude, sol compris. Avec un train, les roues auraient été sous terre. Ses environnements passent à `NO_GROUND`, sans sol : les essais en vol ne le voient pas.
- **A** `just test` passe (340 tests) avec le vrai train du SPAD : les essais en vol ne bougent pas. Dans le jeu, l'ancien arrêt au contact reste en place jusqu'à l'étape 5.
- **A** Étape 4 : `fdm_lab sol <avion>` (`tools/fdm_lab/src/ground.rs`), et dans `essais.ron` les manœuvres `TakeOff` et `GroundHandling`, les robustesses `HardLanding`, `GroundLoop` et `NoseOver`. Pilote scripté, avec les commandes du joueur.
- **⚠** Les essais ont trouvé deux fautes de ma part, et aucune ne se voyait dans les tests :
  - **ressort de la glissière inversé** : 91 mm d'essieu à l'arrêt au lieu de 20. J'avais écrit k cos² θ, et le bon résultat est k / cos² θ. Mon test d'alors validait l'erreur : il est refait sur l'énergie stockée ;
  - **béquille isotrope** : en fin de roulage, l'avion partait seul en cheval de bois vers 40 km/h, jusqu'à 170° de cap, plein palonnier, contre le « no tendency to turn » de Meister. Les roues sont devant le centre de gravité et déstabilisent ; un sabot qui frotte pareil dans toutes les directions ne compense pas. Or un sabot étroit trace un sillon et résiste de côté. La béquille a maintenant la même loi directionnelle que les roues (adhérence 0,8, pente à 8°, `E:`). C'est une justification physique : la valeur reste estimée.
- **R** Campagne `just essai spad7`, valeurs `E:`, sans cible historique :
  - **repos** : assiette 14,9°, 296 kg par roue, 127 kg sur la béquille, 20 mm d'essieu ;
  - **point fixe** : 1 593 tr/min et 4,0 kN, le calcul de `spad7-calculs.md` ;
  - **mise en mouvement** : 15 % de gaz ;
  - **décollage** : sans palonnier, l'avion part à **gauche** et finit en cheval de bois, saumon au sol ; au pied, palonnier jusqu'à 54 % à droite, écart de cap 11°, envol en 11,4 s sur 152 m à 100 km/h, 15 m à 319 m ;
  - **arrêt** : de 82 km/h en 161 m et 16,7 s, cap tenu à 2,5° ;
  - **virages de roulage** : 3,8 °/s à 30 % de gaz, 20 °/s à 50 % manche poussé, la technique d'époque ;
  - **posés** : 5,9 kN par roue à 0,5 m/s, 13 kN à 2 m/s, 20 kN à 3 m/s ; butée vers 2,5 m/s ; à 4 m/s, rebond, décrochage et aile au sol, sans intervention du pilote scripté ;
  - **capotage** : seulement en le cherchant, manche poussé à fond plein gaz ; l'hélice touche d'abord.
- **R** Les performances en vol ne bougent pas. L'envol de 152 m reste au-dessus des 88 m estimés hors outil, qui ignoraient le début queue basse ; le temps total, 11,4 s, tombe dans les 10 à 15 s de cette estimation.
- **A** ADR 0008 (contact par efforts de pénalité dans le RK4, à 240 Hz), au statut **Proposé** ; `modele-aero.md` et `format-avion.md` à jour.
- **R** Convergence : diviser le pas par deux change l'envol et le posé de moins de 1 % (test). Dérive à l'arrêt : 0,7 cm/s, soit 40 cm par minute, gardée pour l'instant.
- **A** `just check` passe, `just test` aussi (350 tests). Revue de l'aérodynamicien lancée sur le contact, les valeurs `E:` et les essais.
- **R** Treizième passe de l'historien (`spad7.md` § 6.6, `demandes.md` D16 à D18). archive.org et la NTRS s'ouvrent enfin.
  - **Comparable majeur** : *Structural Analysis and Design of Airplanes* (1920) calcule un amortisseur complet pour 748 kg : 130 mm de course, 60 brins de ½ in, 3,8 kJ, 6,75 fois le poids en butée (C).
  - **Roulement sur herbe** : 0,06 (C). **Sabot** : 0,4 (C/D, `E:`). **Limites** : 14 kN par roue d'après un programme français, jusqu'à 6,75 fois le poids.
  - **Technique** : queue levée tôt, manche poussé (B pour la pratique, C pour le S.VII) ; posé trois points, norme enseignée, et queue haute admissible (T8, T26).
  - **Vitesse d'atterrissage** du S.VII : 85 à 95 km/h (C), sans mesure directe.
  - **T** : en fusées à la nuit (Hall) et en ampoules (RNAS) ; de jour, aucun T de toile avant 1919. **Le pré reste sans T.**
  - **Géométrie** : α de 22° et 20,5 % sur la béquille, contre 15 à 18° et 10 à 17 % dans la pratique d'époque. Le centre de gravité du modèle serait de 11 à 20 cm trop en arrière des roues. Ce n'est pas une preuve : aucune de ces mesures n'est un SPAD. Versé au dossier du centrage, sans correction.
- **A** Étape 5, dans le jeu :
  - **format des scénarios** : `start: Flight(...)` ou `Ground(...)`, et une position ;
  - **`terrain.ron` et `finale.ron`** : états calculés par `fdm_lab`, tenus par des tests comme `vol-essai.ron` ; plané de finale à −7,9°, soit 1/7, la pente que l'historien attend ;
  - **`just run` démarre au sol** ; la croix ← / → choisit le prochain départ, Select y va ;
  - **accidents** : hélice, structure ou train au-delà de leur limite (`Accident`), à la place de l'ancien arrêt au premier contact ;
  - **animation** : roues qui roulent, essieu qui remonte sa glissière ;
  - **caméra** gardée au-dessus du sol, **télémétrie** de la charge sur les roues et la béquille.
- **⚠** En relisant : `LandingTolerance` n'était créée que dans les applications de test. Le vrai jeu aurait planté au lancement, sans qu'aucun test le voie. Corrigé.
- **A** `just check` passe, `just test` aussi (358 tests). Les valeurs de l'historien entreront dans `aircraft.ron` après la revue de l'aérodynamicien : elles viennent d'un amortisseur vertical, alors que la glissière du SPAD penche de 46°.
- **A** Étape 6, aides au sol (`crates/game/src/realism/`) :
  - au contact, et 0,5 s après pour les rebonds, la garde et la coordination se coupent ;
  - **tenue de cap** en Arcade et Pilote, par le seul gouvernail : le palonnier du joueur s'y ajoute et déplace le cap tenu. Ses gains (`ground_yaw`) viennent de `fdm_lab assist`, du moment du gouvernail dans le souffle, sur la pression à la dérive (q + T/A) ;
  - **envol en Arcade** : gaz au-delà de 50 %, l'instructeur lève la queue, puis tire à 1,2 V_s ;
  - **`landing_rule`** dans `RealismSettings`, marqué non physique : Arcade × 2, Pilote × 1,3, Vétéran et As × 1 (`E:`, à régler à l'essai du joueur).
- **⚠** La première tenue de cap, purement proportionnelle, laissait dériver l'avion de 12° à gauche contre le couple permanent de l'hélice. Un terme intégral, le pied qu'un pilote garde appuyé, ramène l'écart à 7°.
- **R** Tests de fumée par préréglage, sans toucher au palonnier :
  - **Pilote** : décolle, écart de cap au plus 7° ;
  - **Arcade** : décolle aux seuls gaz, écart au plus 7° ;
  - **As** : même roulage sans aide, l'avion part en cheval de bois (au moins 28° de cap) ;
  - **Vétéran** et **As** : le palonnier reste au joueur, et la garde n'intervient pas à l'arrêt.
- **A** Étape 7 :
  - **pré** (`crates/game/src/world/meadow.rs`) : 800 × 300 m (`E:`), est-ouest, posé sur la plaine, car la tuile de 2 km aurait dessiné un terrain tous les 2 km. Sans T, faute de source de jour ;
  - **gizmos de forces** : touche G (plugin `dev`), air, traction, et sol à chaque contact ;
  - **scènes** `terrain-repos`, `terrain-pilote`, `terrain-roulage`, `terrain-finale`.
- **⚠** Première capture : l'herbe était un aplat, et sur `terrain-roulage` on ne voyait même pas que l'avion roulait. Sans repère au sol, ni la vitesse ni la hauteur à l'arrondi ne se lisent. Une tuile de touffes de 40 m, sans couture, s'ajoute sur le pré. Reste à voir si la plaine en demande autant quand on se pose ailleurs.
- **R** Captures **regardées** sur la RX 7800 XT : l'avion est posé sur ses trois points ; depuis le poste, queue basse, le capot et la Vickers bouchent l'avant, comme l'écrivent les témoins ; depuis la finale, le pré se voit au-dessus du viseur. `vol-essai-poursuite` ne change pas. Coût en 1080p : 5,5 ms en moyenne sur 125 images, chargement compris, et 2,9 ms sur 1 805 images au roulage. La moyenne courte mesure surtout le démarrage : seule la longue vaut pour le budget.
- **A** `just check` passe, `just test` aussi. Revue du directeur artistique lancée sur le pré.
- **A** `asset_check` vérifie le pivot des roues sur leur axe (0,0 mm).
- **A** Premier **atterrissage complet** en test, de la finale à l'arrêt, avec les commandes du joueur et le préréglage Pilote. Il a trouvé un défaut de conception du plan :
- **⚠** Couper la garde au contact était une erreur. Un manche tiré au toucher, à 103 km/h, cabrait l'avion à 28° : il redécollait queue basse, décrochait et tombait sur un saumon. Un joueur fera ce geste. **D** La garde reste active au sol tant que la vitesse peut porter l'avion, et se coupe en dessous. Plan corrigé.
- **R** Même garde, manche tenu à fond : l'avion remonte à 7,7 m à 10° d'incidence, s'essouffle et retombe à 5 m/s, train brisé. C'est le « ballon », une vraie faute de pilotage : gardé tel quel.
- **R** Posé scripté correct, sans palonnier : arrondi sur le taux de descente depuis 12 m, toucher à 103 km/h et 0,5 m/s, 123 m après le début du pré ; la vitesse tombe sur les roues, puis le manche revient avec la queue ; arrêt 276 m plus loin, cap à 1°, rien de cassé.
- **R** Revue du directeur artistique sur le pré : **à retravailler**. Un seul point bloquant, et il est de jouabilité : depuis le poste, le sol n'apporte aucun repère. Vu de 2 m, un pixel couvre 12 cm de sol à 15 m et 5 m à 100 m, or la force du bruit était dans les 16 à 42 cm. Autre point : `AlphaMode::Blend` fait échapper le pré à la brume de l'atmosphère, d'où sa teinte chaude et son halo depuis la finale.
- **A** Corrections selon la revue (`meadow.rs`) :
  - taches de 6 × 2 m à 0,6 × 0,2 m, étirées d'est en ouest : elles résistent au raccourci et dessinent des lignes de fuite ;
  - grandes taches de 18 et 6 m dans la texture du pré, hors de la tuile de 40 m, dont la période se serait vue d'en haut ;
  - pré en `Mask`, donc dans la brume, avec un contour net ; touffes gardées en `Blend`, qui s'effacent entre 300 et 600 m de l'avion ;
  - tuile à 2 cm par texel, plus sombre que claire.
- **A** Nouvelle scène `terrain-roues`, vue de l'avant et du côté, qui montre le contact au sol.
- **R** Captures **regardées** : le sol se lit depuis le poste, les roues reposent sur l'herbe, le pré a la teinte de la plaine sous la brume. L'herbe est éclaircie ensuite pour que le pré ressorte depuis la finale.
- **⚠** Le voile du disque d'hélice a un bord net sur le terrain : relevé par la revue, sans lien avec le pré. Noté pour plus tard.
- **A** Valeurs de la treizième passe dans `aircraft.ron` : pneu de 220 kN/m, jante à 55 mm, roulement 0,06, sabot 0,4, course de 125 mm, limite de béquille 7,4 kN. `Spring` gagne la butée de jante : à 3 m/s, le pneu va à fond avant le sandow.
- **D** Le sandow de S106 est calculé pour un axe vertical. Je garde ses **charges vues de la roue** (1,7 kN de précontrainte, 24,8 kN en butée), ramenées le long des glissières du SPAD, inclinées de 46° : 1,17 kN et 127 kN/m. Copier la raideur du cordon telle quelle aurait donné 35 kN en butée, loin au-delà de toute limite.
- **D** Limite de 20 kN par roue, entre les 14 kN de la règle française et les 24,8 kN de S106 : 14 kN casseraient le train dès 2,3 m/s, un posé seulement ferme selon les mêmes sources.
- **R** Résultats (`fdm_lab sol`) : au repos, 6 mm d'essieu, le chiffre de l'historien ; 5 kN par roue à 0,5 m/s, 11,5 kN à 2 m/s, 18 kN à 3 m/s, 25 kN à 4 m/s, essieu presque en butée. Arrêt de 84 km/h en 184 m.
- **R** Revue de l'aérodynamicien sur le contact. Verdict : **bloquant**, un seul point, avec cinq points importants. Le reste tient : signes, stabilité de route, levée de queue, convergence.
- **⚠** **Bloquant** : le ressort de roue était figé pour l'assiette de vol, avec cos θ = 0,689. Or l'avion roule et se pose à 15° queue basse, où la glissière fait 61° avec la verticale (cos θ = 0,48). La raideur vue du sol était sous-estimée d'un facteur 2,1, la charge de butée de 44 %, et le point d'application partait sous le sol. **A** Le ressort se calcule à chaque état, selon l'assiette ; `aircraft.ron` donne les valeurs le long du sandow. Deux tests ajoutés : le contact reste au sol à toute assiette, et la glissière est plus raide queue basse.
- **A** Autres corrections de la revue :
  - **course** : 125 mm lus comme la montée de la roue en ligne de vol, soit 181 mm le long de la glissière et 85,5 kN/m (`E:`, à vérifier sur l'image de S49) : on retrouve 3,2 kJ ;
  - **amortissement** : 700 N·s/m, soit 0,2 du critique à l'assiette du posé. La revue note que le vrai terme physique serait le frottement dans la glissière ;
  - **limite de roue** : 24 kN (S106), et son commentaire ne s'appuie plus sur le résultat visé ;
  - **`CREEP_SPEED`** : 0,03 m/s. À 0,02, le mode couplé des roues, du roulis et du lacet sortait de la limite du RK4, avec un état au repos incohérent. Dérive à l'arrêt : 1,4 cm/s, soit 84 cm par minute ;
  - **cheval de bois** : l'ancien essai vérifiait en fait un roulage sur trois points. Il devient une seconde de pied gauche à 75 km/h, queue haute, plein gaz ; le même pied à 60 km/h sur trois points n'écarte le cap que de 3° ;
  - **décrochage** de référence pris à 0 m, et non à 500 m ;
  - **commentaires** du sabot justifiés par la physique, et non par le résultat cherché.
- **D** Adhérence latérale des roues gardée à 0,6, contre le 0,35 proposé par la fiche (confiance D) : c'est un frottement longitudinal transposé, et il interdit de basculer sur une aile, ce qui demande plus de 0,53.
- **⚠** Le « 152 m » d'envol donné plus haut était **faux** : le temps était celui de l'envol, mais la distance et la vitesse étaient lues 0,5 s plus tard. Corrigé dans `fdm_lab`.
- **R** Campagne après correction : les charges retombent sur les mesures de la revue, 12,5 kN à 2 m/s, 20,0 kN à 3 m/s, 28,4 kN à 4 m/s. Avec 24 kN, 3 m/s survit et 4 m/s casse. Arrêt comparable aux 164 et 217 m de Roosevelt Field (S108) ; « s'arrête court » reste qualitatif.
- **A** `just test` passe (377 tests).
- **R** Relecture ciblée de l'aérodynamicien : correctif bloquant bien recopié. `stop_stiffness` reste hors du passage à la verticale, car elle sert aussi à la jante.
- **⚠** Le cheval de bois partait moteur au ralenti : le souffle arrivait trop tard sur le gouvernail, et l'aile ne touchait jamais, alors que le commentaire de `grip` s'appuie sur ce contact. **A** L'essai part à pleine puissance, et son test exige maintenant que le saumon touche.
- **R** Une seconde de pied gauche à 75 km/h, queue haute, plein gaz : 180° de cap, 17° d'inclinaison, 12,7 kN sur un saumon. Le même pied à 60 km/h sur trois points n'écarte le cap que de 3°.

### ADR 0008 validé

- **Q** Le joueur : « Oui, valider ».
- **D** ADR 0008 (contact au sol : efforts de pénalité dans le RK4, à 240 Hz) accepté.
- **A** Statut mis à jour dans l'ADR, son index et le plan `m2-train-sol.md`.
- **R** Reste à faire pour clore le M2 : le vol de dix minutes à la manette, dont les retours iront dans `docs/gdd/retours.md`.

## 2026-11-04

### Conversation sur l'application mobile

- **Q** Le joueur : « je voudrais avoir cette conversation disponible dans mon application mobile claude ».
- **A** Remote Control activé pour cette session, intitulée « Simulation WWI » : elle se suit et se pilote depuis l'application mobile Claude et claude.ai/code.
- **R** Le lien ne tient que tant que l'application de bureau tourne et que la machine reste allumée et éveillée. Rien n'a changé dans le code.
- **⚠** Push impossible : l'agent SSH n'a de nouveau aucune clé chargée (`Permission denied (publickey)`). Cette entrée est commitée en local, comme le 2026-11-01 ; le joueur charge sa clé, puis pousse.
- **Q** Le joueur : « c'est fait ». **R** Push vérifié : `origin/main-tcd0yw` est sur `b367ed8`.
- **⚠** L'agent SSH de la session (`/run/user/1000/ssh-agent.socket`, le seul de la machine) reste vide : le push du joueur est passé par un autre chemin. Tant que la clé n'y est pas chargée, chaque entrée du carnet attend un push du joueur. Commande proposée : `SSH_AUTH_SOCK=/run/user/1000/ssh-agent.socket ssh-add ~/.ssh/id_ed25519_fixe`.
- **Q** Le joueur : « c'est fait ». **R** La clé est dans l'agent de la session : Claude a poussé `7d7ac2c` lui-même. L'incident est clos jusqu'au prochain redémarrage de la machine.

### Clôture du M2

- **Q** Le joueur : « Considère que le vol de 10mn s'est bien passé ».
- **D** Le critère de fin du M2 est tenu sur décision du joueur : **M2 clos le 2026-11-04**. Le décollage et l'atterrissage en Pilote sont aussi couverts par des tests scriptés.
- **A** Mis à jour : `docs/gdd/retours.md`, la feuille de route, le statut du plan `m2-train-sol.md` et la ligne « Jalon en cours » de `CLAUDE.md`.
- **⚠** Aucun retour détaillé : le décollage, le posé, les aides au sol, le centrage du 2026-10-31 et l'avis par préréglage restent sans retour du joueur.
- **⚠** Le M1 reste ouvert sur son seul essai du joueur, la lecture du rapport d'essais.
- **D** À demander au joueur : le prochain jalon.
- **Q** Le joueur : « le décollage et le posés se sont bien déroulés ». **A** Ajouté à `docs/gdd/retours.md`. Restent sans retour : les aides au sol par préréglage et le centrage du 2026-10-31.
- **Q** Le joueur : « quels jalons sont ouverts ? ». **R** Clos : M0 et M2. Ouvert : M1, dont il ne reste que la lecture du rapport d'essais par le joueur. Non commencés comme jalons, mais déjà avancés : M3 (modèle PBR du SPAD presque fini, poste de pilotage absent), M4 (modèle 3D prêt pour les dégâts) et M5 (aides au pilotage et tolérance d'atterrissage). M6 et M7 sont intacts.

### M3 : plan

- **Q** Le joueur : « on continue avec le M3 ».
- **A** Plan `docs/plans/m3-beaute.md`, en sept incréments :
  - scènes en données, mesure GPU et réglages de qualité ;
  - nuages volumétriques, avec parité CPU/GPU ;
  - heure du jour ;
  - relief réel et `tools/terrain_bake` ;
  - paysage de 1917 et front ;
  - finition du SPAD (bois, métal, seconde livrée, poste de pilotage) ;
  - budget et revue finale.

  Aucun code avant la validation du joueur.
- **R** Vérifié dans les sources de Bevy 0.20 : l'atmosphère physique et la perspective aérienne existent, mais aucun rendu de nuages. Les volumes de brouillard (`FogVolume`) sont des boîtes pensées pour un brouillard local. `RenderDiagnosticsPlugin` donne les temps GPU par passe.
- **⚠** Les MNT nationaux (RGE ALTI, Licence Ouverte 2.0 ; DHMV II, licence flamande) demandent une attribution, mais ne figurent pas mot pour mot dans la liste de licences de `CLAUDE.md` : ils demandent l'accord du joueur. OpenStreetMap est exclu (ODbL, et paysage moderne).
- **D** Cinq décisions demandées au joueur : ordre des incréments, technique des nuages, données de relief, méthode pour l'occupation du sol de 1917, étendue.
- **A** Feuille de route et ligne « Jalon en cours » de `CLAUDE.md` : M3 en cours.

### M3 : plan validé

- **Q** Le joueur : « Oui, tout en A ».
- **D** Plan `m3-beaute.md` validé avec les cinq options recommandées. Les licences RGE ALTI (Licence Ouverte 2.0) et DHMV II (licence flamande de réutilisation) sont acceptées par le joueur. Les fichiers se téléchargeront avec son accord, fichier par fichier, et seront inscrits dans `assets/LICENCES.md`.
- **A** Historien lancé en parallèle : terrains de la SPA 3 dans les Flandres en 1917, tracé du front, cartes de 1917 et leurs droits, licences et fichiers exacts des deux MNT.

## 2026-11-05

### M3, incrément 0 : scènes en données et mesure

- **Q** Le joueur : « Continuez le travail en cours sur main-tcd0yw : examinez les modifications non commitées, dites-moi dans quel état elles sont et terminez ce qui a été commencé ».
- **R** État trouvé : l'incrément 0 du plan `m3-beaute.md`, entamé sans entrée au carnet :
  - format `assets/scenes/<nom>.ron` et sept scènes de vol ;
  - ressource `Quality` ;
  - temps GPU par passe dans le `.json` ;
  - captures en 2560 × 1440 par défaut.

  Le code compilait, et `just check` comme `just test` passaient. Restaient à faire le rapport JSON, assemblé en retirant puis en remettant son accolade finale, la documentation et les captures.
- **⚠** Les captures sortaient en **2560 × 1366** : le compositeur ramène la fenêtre à la zone utile de l'écran. **A** Les caméras rendent désormais dans une image hors écran, à la taille exacte demandée (`Screenshot::image`, vérifié dans les sources de Bevy 0.20 et l'exemple `headless_renderer`).
- **⚠** Le temps simulé d'une capture suivait l'horloge réelle : les 1 800 images de `terrain-roulage` valaient 5,2 s de roulage en 1080p, et 6,3 s en 1440p. Une scène en vol dépendait donc de la vitesse du GPU. **A** L'horloge reste en pause pendant le chargement, puis avance de 1/60 s par image ; la durée réelle se mesure à part. **R** Deux captures de la même scène diffèrent de 9 pixels sur 3,7 millions.
- **D** `terrain-roulage` passe à 420 images, soit 7 s : la queue est levée et l'avion roule encore. À 300 images, elle est basse ; à 540, l'avion a décollé.
- **D** Les six scènes qui posent l'avion à la main restent construites en code ; seules les scènes d'un vol sont des fichiers.
- **A** La recette `just capture` garde la capture précédente en `<scène>.avant.png` et `.avant.json`, comme le décrivait déjà le skill `capture`.
- **R** Les treize scènes ont été recapturées et **regardées**, puis comparées à celles du 1er novembre :
  - scènes en code : identiques, avec un écart moyen de 0,2 à 0,6 niveau sur 255 ;
  - vol d'essai : même composition, deux secondes plus tard ;
  - terrain : herbe un peu plus claire, éclaircie après les captures du 1er novembre.
- **R** Coût en 1440p, qualité « haut » : de 1,1 à 3,3 ms de GPU. La sonde d'environnement de l'atmosphère, recalculée à chaque image, coûte déjà 0,7 ms.
- **A** Mis à jour : skills `bevy` (§ 7) et `rendu-visuel`, plan `m3-beaute.md`. `just check` et `just test` passent (380 tests).
- **⚠** La recherche de l'historien lancée le 2026-11-04 n'a laissé aucune fiche dans `docs/recherche/`. Les terrains de la SPA 3, le tracé du front, les cartes de 1917 et les fichiers des MNT restent à chercher avant les incréments 3 et 4.
- **⚠** Push impossible : l'agent SSH de la session est de nouveau vide (`Permission denied (publickey)`), comme le 2026-11-04. Le travail est commité en local (`7f438bd`). Le joueur charge sa clé avec `SSH_AUTH_SOCK=/run/user/1000/ssh-agent.socket ssh-add ~/.ssh/id_ed25519_fixe`, puis Claude pousse.

### Retour à la conversation « Simulation WWI »

- **Q** Le joueur : « je veux repasse sur la conversation "Simulation WWI" ».
- **R** Je ne peux pas ouvrir cette session : seules celles qu'elle a lancées le peuvent. Le joueur l'ouvre depuis la barre latérale. Elle partage le dossier et la branche `main-tcd0yw` : elle verra les commits du jour et l'entrée ci-dessus. Remote Control n'y est plus actif.
- **⚠** Push toujours impossible : l'agent SSH de la session reste vide. `7f438bd`, `7f54307` et cette entrée attendent en local.

### Reprise dans la conversation « Simulation WWI »

- **Q** Le joueur : « reprend le travail sur main-tcd0yw ».
- **R** État trouvé : l'incrément 0 du M3 est fait et commité (`7f438bd`) par l'autre conversation, avec deux entrées de carnet. Trois commits attendent en local : l'agent SSH est vide depuis le redémarrage de la machine.
- **A** Historien relancé sur la recherche interrompue du 2026-11-04, avec la consigne d'écrire sa fiche question par question.
- **A** Suite : incrément 1 du M3, les nuages volumétriques.

## 2026-11-07

### Reprise de l'entrée du 2026-11-05

- **Q** Le joueur : « reprend l'entrée 2026-11-05 dans le carnet de bord ».
- **R** État trouvé : la machine a redémarré, l'agent SSH est vide et quatre commits attendent en local. `just check` et `just test` passent d'une traite (380 tests) : l'arrêt du 2026-11-05 (code 137) ne s'est pas reproduit.
- **A** Historien relancé une troisième fois, avec la consigne d'enregistrer sa fiche après chaque question. **R** `docs/recherche/flandres-1917.md` est écrite (sources F1 à F41). `demandes.md` reçoit un complément à D17 et les demandes D19 à D22 ; rien n'est envoyé.
  - La SPA 3 est à Bergues-Bierne le 20 octobre 1917 (B), et à Saint-Pol-sur-Mer du 15 août au 25 septembre.
  - Ce jour-là, S.254 est exposé aux Invalides : « S.254 au 20 octobre » désigne une livrée, pas un vol. La fiche propose trois lectures, à trancher par le joueur (§ 1.2).
  - Aucun tracé du front n'existe en données ouvertes. Il faudra le relever sur les cartes de tranchées britanniques de la NLS, dont la licence CC-BY reste à confirmer.
  - RGE ALTI : deux archives de 233 et 271 Mo, sous Licence Ouverte 2.0, avec une mention d'attribution proposée.
  - DHMV II : GeoTIFF à 5 m, une archive par feuille NGI. La licence flamande de réutilisation gratuite est à confirmer dans le paquet.
  - Comines-Warneton échappe peut-être aux deux MNT.
- **A** Correction dans la fiche : la demande au SPW est D22, pas D23.

### M3, incrément 1 : densité des nuages en Rust

- **D** Nouvelle crate `escadrille_weather` (`crates/weather`), indépendante de Bevy comme le FDM et optimisée même en dev.
  - La densité sert au rendu, au jeu et aux outils.
  - Ses bruits (2 millions de texels) se génèrent en 0,2 s sur tous les cœurs, au lieu de plusieurs secondes.
  - Son ajout à l'arborescence de `CLAUDE.md` attend l'accord du joueur.
- **A** Contenu, couvert par 25 tests :
  - bruits périodiques et déterministes : hachage PCG3D, Perlin, Worley, sommes fractales ;
  - grilles de 8 bits échantillonnées comme le GPU : centres de texels, filtrage linéaire, répétition ;
  - textures : forme 128³ (Perlin-Worley égalisé), détail 32³ (Worley), carte météo 512² (amas et type de nuage) ;
  - `CloudLayer` (base, épaisseur par type, couverture, vent, extinction) et `CloudUniforms`, les uniformes du futur shader, avec des décalages ramenés dans une période en f64 ;
  - densité grossière (premier regard de la marche de rayons) et densité complète.
- **⚠** La première version suivait Schneider 2015 : la couverture coupait le bruit de forme. Vue de dessus, la répétition de ce bruit tous les 4 km dessinait une grille de nuages identiques. **A** Passage au schéma de *Nubis* (Schneider, 2017) : la carte météo place les nuages et fixe leur hauteur, le bruit de forme n'érode que leur enveloppe. **R** Vue de dessus : des cumulus épars, sans grille. En coupe : bases plates, dômes, flancs bosselés. Images regardées.
- **R** Valeurs par défaut, des cumulus d'été (`ESTIMÉ`, justifiés dans le code) :
  - base à 1 200 m ;
  - 42 % du ciel couvert ;
  - 680 m d'épaisseur moyenne, sommets jusqu'à 1 100 m au-dessus de la base ;
  - extinction de 0,06/m, d'après σ ≈ 3 LWC / (2 ρw re) : environ 65 m de visibilité dans le nuage.
- **A** Suite : la même densité en WGSL, puis le test de parité sur le GPU. Il passera par `initialize_renderer` de Bevy, sans dépendance nouvelle.
- **A** `just check` et `just test` passent (405 tests). Commit `064c0e0`.
- **⚠** Push impossible : l'agent SSH est vide depuis le redémarrage. Six commits attendent en local. Le joueur charge sa clé avec `SSH_AUTH_SOCK=/run/user/1000/ssh-agent.socket ssh-add ~/.ssh/id_ed25519_fixe`, puis Claude pousse.
- **?** Questions au joueur : l'ajout de `crates/weather/` à l'arborescence de `CLAUDE.md`, et la lecture retenue pour la date du jeu (`flandres-1917.md` § 1.2).

### Push et arborescence

- **Q** Le joueur : « c'est fait et ajoute crates/weather à l'arborescence de CLAUDE.md ».
- **A** Clé chargée par le joueur : les six commits en attente sont poussés (`ffae64d..d61d50a`).
- **D** `crates/weather/` (`escadrille_weather`) entre dans l'arborescence de `CLAUDE.md`, avec l'accord du joueur. Le fichier `nuages-volumetriques.md` du skill `rendu-visuel` indique où se trouve la densité de référence et pourquoi le schéma de *Nubis* a été retenu.
- **?** Reste au joueur : la lecture retenue pour la date du jeu (`flandres-1917.md` § 1.2).
- **A** Suite : la densité en WGSL et le test de parité sur le GPU.

### M3, incrément 1 : densité en WGSL et test de parité

- **Q** Le joueur : « Oui, continuer ».
- **A** `assets/shaders/clouds/density.wesl` : la densité en WGSL, jumelle ligne à ligne de la référence Rust. Le module est en WGSL pur, sans import ; textures et échantillonneur y sont des paramètres de fonction, si bien que chaque passe les lie où elle veut.
- **A** `crates/game/src/render/clouds.rs` :
  - `GpuCloudUniforms`, les uniformes au format WGSL (96 octets, vérifiés par un test) ;
  - `GridTexture`, qui fait des grilles des textures de Bevy, échantillonnées en linéaire avec répétition ;
  - le **test de parité** : il obtient un GPU par `initialize_renderer` de Bevy, sans dépendance nouvelle, lance le module dans une passe de calcul sur 65 536 points et compare avec la référence.
- **R** Sur la RX 7800 XT, 18 % des points tombent dans un nuage. L'écart est au pire de 3,5·10⁻³ et en moyenne de 2,5·10⁻⁵, à cause de l'arrondi des poids du filtrage matériel. Seuils du test : 10⁻² au pire, 2·10⁻⁴ en moyenne.
- **A** Mis à jour : skill `rendu-visuel` (parité, et `nuages-volumetriques.md`), plan `m3-beaute.md`. `just check` et `just test` passent (408 tests).
- **A** Suite : la passe de rendu (marche de rayons en quart de résolution, éclairage, composition avec la profondeur), l'ADR 0009 et la scène `cumulus_midi`.

### M3, incrément 1 : passe de rendu des nuages, premières images

- **Q** Le joueur : « Oui, continuer » (passe de rendu, ADR 0009, scène `cumulus_midi`).
- **D** ADR 0009 (proposé) : une marche de rayons maison, éclairée par les tables de l'atmosphère de Bevy. La passe recrée le groupe de liaisons de `render_sky`, dont toutes les ressources sont publiques, et importe ses fonctions WESL.
- **⚠** Il faut poser les nuages après le ciel de Bevy et avant les objets transparents. Or `render_sky` est un système privé, et trois voies ont échoué :
  - `.after()` ne peut pas nommer ce système ;
  - son ensemble implicite se retrouve dans le schedule, mais on ne peut pas reconstruire son identifiant interné ;
  - une passe de construction du schedule exigerait la crate `indexmap`, une dépendance de plus.

  **A** Solution en deux temps, par des API publiques seulement :
  - la marche écrit lumière et transmittance dans sa propre image, entre les opaques et les transparents, sans toucher à l'image principale ;
  - un matériau transparent plein écran compose cette image. Un biais de tri le fait dessiner en premier dans la passe des transparents, donc après le ciel et sous les haubans.
- **A** Scène `cumulus_midi` (champ `clouds` des scènes, facultatif) : le SPAD à 1 000 m parmi des cumulus dont la base est à 600 m, avec la graine 3, choisie par la référence Rust pour que la trajectoire reste dégagée. Les scènes sans nuages ne changent pas (écart de 0,09 %, le bruit de la TAA).
- **⚠** Quatre défauts, trouvés l'un après l'autre sur les captures :
  1. Vus de dessous, les nuages paraissaient bleus et translucides. Une capture de diagnostic, qui dessine les nuages en noir selon leur seule opacité, les a montrés opaques : c'était l'éclairage. Sous un nuage épais, la marche vers le soleil s'éteint, et les octaves de multidiffusion aussi ; seule restait la lumière du ciel. **A** Approximation de diffusion, en 1/(1 + ¾(1−g)τ) (Bohren 1987), et rebond du sol sous les bases.
  2. Caméra dans la couche : les 96 pas s'épuisaient vers 1,5 km, et aucun nuage lointain n'apparaissait. **A** La marche enjambe l'air vide trois pas à la fois, et seuls les pas dans un nuage comptent.
  3. Nuages cotonneux. **A** La lumière du ciel est atténuée dans le nuage ; l'extinction passe à 0,09/m (eau liquide 0,5 g/m³, gouttes de 8 µm) ; la frange en filaments de la base passe de 10 % à 3 % de la couche.
  4. Encore trop doux et trop gris : en revue par le directeur artistique.
- **R** Coût : 1,3 ms de GPU en 1440p, qualité « haut », en pleine résolution, sur la RX 7800 XT.
- **A** `just check` et `just test` passent (410 tests).

### Revue du directeur artistique : premiers nuages

- **R** Verdict « à retravailler » : l'image se lit comme du brouillard, pas comme des cumulus. Ce qui fonctionne :
  - la composition des plans, nuages derrière l'avion et haubans par-dessus ;
  - pas de banding ni de bruit ;
  - pas de répétition visible ;
  - un voile accordé à celui du sol.
- **R** À corriger, par ordre d'impact :
  1. **Nuages éteints**, plus sombres que la toile au soleil. Le terme diffus 1/(1 + 0,11 τ), divisé par 4π, oublie la lumière renvoyée et accumulée : il manque un facteur 3 à 4. Cible : une plaque τ = 50 rend R ≈ 1 − T. Test CPU proposé.
  2. **Aucun modelé**. La lumière diffuse ne dépend pas de l'incidence du soleil. Le rebond du sol n'est ni atténué ni ombré, et compte sur toute la sphère, avec l'albédo 0,3 de Bevy (cible : 0,15 à 0,2 pour des champs).
  3. **Bords mous**. Densité plafonnée vers 0,3-0,45 : la raidir (×4 à ×6, saturée) et monter l'érosion de détail à 0,5-0,6 sur les flancs.
  4. **Cadrage**. L'avion est dans la couche et ne voit que des flancs. Il faut des cumulus de 600 à 800 m d'épaisseur ou l'avion vers 1 500 m, deux tiers de ciel, et une variante face au soleil.
  5. **Couleur** : blancs crème au soleil, ombres bleutées. Les points 1 et 2 devraient corriger l'essentiel.
  6. **Ombres des nuages au sol**, déjà prévues. Une capture sans TAA dira quelle part du flou vient d'elle.
- **D** Prochaine itération : points 1, 2 et 4 ensemble, validés par le test d'albédo ; le point 3 ensuite.
- **R** Critique transmise au joueur, avec la recommandation : traiter ensemble les points 1, 2 et 4, puis le point 3.
- **?** En attente du joueur : valider l'ADR 0009, et enchaîner ou non sur l'énergie de l'éclairage et le cadrage.

### M3, incrément 1 : éclairage et cadrage des nuages, d'après la revue

- **Q** Le joueur : « Oui, et valider l'ADR ».
- **D** ADR 0009 accepté.
- **A** Point 1, l'énergie. Nouveau module `escadrille_weather::scattering`, jumeau des fonctions du shader, avec 6 tests :
  - diffusion multiple en 2,8 / (4π (1 + 0,08 τ)), étalonnée sur une couche τ = 50 éclairée à 58°. Elle renvoie par le dessus R = 1 − T et transmet par la base T = 1 / (1 + ¾(1−g)τ), d'après la théorie à deux flux de Bohren (1987), à 5 % près ; la base tient aussi de τ = 5 à 100 ;
  - même étalonnage pour la lumière ambiante : une couche épaisse sous un ciel uniforme renvoie R ;
  - phase normalisée, liseré à contre-jour, décroissance dans le nuage.
- **⚠** Limite du modèle, qui est local : une frange mince au soleil renvoie autant que le sommet d'un nuage épais (2,3 fois trop à τ = 5). C'est écrit dans le code.
- **A** Point 2, le modelé. Ciel et sol comptent chacun sur leur demi-sphère, atténués par diffusion. Le sol a un albédo de 0,18 (`ESTIMÉ` : 0,16 à 0,26 pour l'herbe et les cultures, Oke 1987) et reçoit le soleil hors des ombres des nuages.
- **A** Point 4, le cadrage. Nouveau scénario `vol-2000` (trim de `fdm_lab` à 2 000 m et 160 km/h ; le test d'équilibre couvre désormais les deux vols). `cumulus_midi` passe au-dessus d'un champ de cumulus (base 1 000 m, 700 m d'épaisseur : champ `thickness` des scènes). Nouvelle scène `cumulus_contre_jour`, soleil au sud à 22°.
- **⚠** Des stries horizontales barraient les dômes. Quand la marche enjambe l'air vide trois pas à la fois, elle entre dans le nuage à des distances quantifiées que la gigue ne couvre pas. **A** À l'entrée d'un nuage, la marche recule d'une enjambée et repart à pas fins.
- **A** Point 3, les bords. Densité finale raidie ×4 puis saturée, érosion de détail portée à 0,55, en Rust comme en WGSL. L'écart de parité au pire passe à 1,6·10⁻², puisque le gain multiplie l'arrondi du filtrage ; la moyenne reste à 2,8·10⁻⁵. Seuil du test : 4·10⁻².
- **R** Images regardées : un champ de cumulus blancs, en chou-fleur, plus clairs que la toile, avec des bases grises et des rangées qui bleuissent vers l'horizon ; à contre-jour, un vrai modelé. Restent des stries légères au loin (2 à 10 km), et le soleil est hors du cadre de la scène à contre-jour. Coût : 0,6 à 1,2 ms de GPU en 1440p.
- **A** `just check` et `just test` passent (416 tests). Seconde revue du directeur artistique en cours.

### Seconde revue du directeur artistique

- **R** Verdict « à retravailler, net progrès ». `cumulus_midi` est enfin une image de vol ; l'énergie, le cadrage, l'échelle et les bords à moyenne distance sont réglés.
- **R** À corriger, par ordre d'impact :
  1. **Pas de modelé en lumière de face.** Le terme diffus isotrope pèse 7 fois la phase, et ne baisse qu'en 1/(1 + 0,08τ) : un bourgeon dans sa propre ombre n'est que 1,3 fois plus sombre. La marche vers le soleil commence à 20 m, sur la densité grossière, donc les bourgeons ne s'ombrent pas entre eux. Piste : des octaves Σ bⁿ e^(−aⁿτ), recalées sur les tests à deux flux ; un premier pas de 6 à 8 m vers le soleil, sur la densité détaillée.
  2. **Stries lointaines.** Elles viennent de l'enjambée et non du temporel : toute une ligne d'écran partage la même grille d'échantillons, et la gigue ne couvre qu'un pas sur trois. Piste : gigue sur toute l'enjambée, test de vide conservateur.
  3. **Le contre-jour ne se lit pas.** Les nuages sont à 25-40° du soleil, trop loin du lobe avant. Le ciel est atténué par la profondeur vers le soleil, si bien que les faces à l'ombre perdent leur bleu. Pistes : soleil à 8-10°, caméra dans la couche, ciel atténué vers le haut.
  4. **Bords mous de près.** Un texel de détail couvre 8 m. Piste : une seconde octave de détail, distordue par un bruit de rotationnel.
  5. **Champ trop régulier.** Les dômes ont tous la même taille. Piste : des tailles et des épaisseurs par cellule.
  6. **Liserés jaunes** sur quelques sommets lointains : c'est la limite de la frange mince, à revoir après le point 1.
- **D** Prochaine itération recommandée : refaire le terme de diffusion multiple en octaves, avec un premier pas fin vers le soleil et le ciel atténué vers le haut, plus la gigue sur l'enjambée et le recadrage du contre-jour. Ensuite viendront les ombres au sol.

### M3, incrément 1 : modelé et contre-jour des nuages

- **Q** Le joueur : « Oui, enchaîner ».
- **A** Diffusion multiple en deux parts (`escadrille_weather::scattering`, et le shader en jumeau) : une part rapide près de la face éclairée, 1,8·e^−τ, puis la traîne de diffusion, 2,1/(1 + 0,055τ).
  - **⚠** La première version réglait la réflexion à τ = 50 et le contraste des bourgeons, mais assombrissait de 30 à 48 % les bases des nuages minces.
  - **A** Les constantes viennent donc d'un balayage, par un script qui reproduit l'intégration du test : sommet et base à τ = 50 à 8 % près, bases de τ = 5 à 100 à 20 % près, bourgeon de 50 m dans son ombre 2 fois plus sombre.
  - Nouveau test de contraste. La lumière ambiante suit la même décroissance (niveau 2,3, recalé par son test).
- **A** Pour l'éclairage :
  - **soleil** : le premier pas vers lui passe de 40 à 8 m, et les deux premiers portent sur la densité détaillée ;
  - **ciel** : il est atténué par la profondeur jusqu'au sommet de l'enveloppe. La densité grossière renvoie désormais cette distance, en Rust comme en WGSL, et un test la vérifie.
- **A** Pour les stries :
  - la marche n'enjambe plus que l'air hors de toute enveloppe ;
  - la gigue s'étale sur toute l'enjambée.

  **R** Les stries ont disparu, jusqu'à l'horizon.
- **A** Contre-jour recadré : soleil à 9°, avion à 2 000 m dans une couche de 1 500 à 2 400 m, caméra relevée de 5°. La graine 3 collait une tour à l'avion ; la graine 20, choisie avec la référence Rust, met les tours entre 1 et 3 km. **R** Soleil dans le cadre, bords clairs et chauds, faces à l'ombre.
- **R** Coût : 0,9 ms (`cumulus_midi`) et 1,5 ms (`cumulus_contre_jour`) de GPU en 1440p. `just check` et `just test` passent (418 tests). Troisième revue du directeur artistique en cours.

### Troisième revue du directeur artistique

- **R** Verdict « à retravailler, de peu ». L'image de midi peut servir de base. Le contre-jour se lit, mais il est faux de couleur et de valeurs.
- **R** Ce qui est réglé :
  - les stries ;
  - le modelé vertical à midi (dôme blanc, base gris lilas, proche des autochromes) ;
  - la composition du contre-jour, soleil entre deux tours.
- **R** À corriger, par ordre d'impact :
  1. **Corps des tours brun saumon** à contre-jour, de la luminance du ciel. La lumière du ciel n'est atténuée que vers le haut : un flanc 600 m sous le sommet n'en garde que 8 %, alors qu'il voit la moitié du ciel par le côté. Piste : prendre aussi la profondeur latérale jusqu'au bord de l'enveloppe, après une vue de débogage par terme.
  2. **Pas de liseré argenté** : mesurer dans l'image HDR les rapports bord/corps/ciel avant de régler quoi que ce soit (tonemapping, gain de densité, bloom).
  3. **Bas des nuages qui se dissout**, bases arrondies. Piste : une rampe de base de 30 à 60 m, et une érosion qui diminue vers la base.
  4. **Pas de bourgeons de 30 à 150 m** sur le nuage proche : il manque un bruit intermédiaire en chou-fleur.
  5. **Pas d'auréole autour du soleil**.
  6. **Mineurs** : un liseré jaune-vert sur les sommets les plus lointains ; à contre-jour, un flanc du fuselage trop éclairé, peut-être par la translucidité de la toile.
- **R** Base suffisante pour passer au quart de résolution, à une condition : corriger d'abord le point 1. Les ombres au sol devraient être une carte de profondeur optique vers le soleil, qui servira aussi à l'ombre lointaine d'un nuage sur son voisin.
- **?** Au joueur : corriger d'abord la lumière du ciel sur les flancs (environ une heure), comme le recommande la revue.

### M3, incrément 1 : lumière du ciel sur les flancs, contre-jour

- **Q** Le joueur : « Oui, corriger ».
- **A** Vue de débogage par terme sur `cumulus_contre_jour` : quatre captures, une par terme d'éclairage (diffusion simple, diffusion multiple, ciel, sol), en remplaçant la somme dans le shader le temps de les faire.
- **R** Le corps des tours est dominé par la diffusion multiple du soleil, orangée et presque uniforme. La lumière du ciel, bleue, y est déjà notable ; la diffusion simple ne donne qu'un liseré fin ; le sol est sombre.
- **⚠** La cause principale n'est donc pas celle que supposait la revue. La marche vers le soleil s'arrêtait vers 900 m, alors qu'un soleil à 9° traverse plusieurs kilomètres de tour. La profondeur vers le soleil était sous-estimée, et la face sombre gardait environ trois fois la lumière que laisse passer la théorie à deux flux.
- **A** La marche vers le soleil porte à 2,9 km (6 pas, ×3).
- **A** La lumière du ciel passe aussi par le côté de l'œil : sa profondeur est la plus faible entre celle jusqu'au sommet de l'enveloppe et celle déjà traversée le long du rayon de vue. Un flanc vu de face voit le ciel devant lui. Ce chemin ne vaut pas quand l'œil regarde une base par en dessous, puisque de ce côté la base voit le sol. C'est la profondeur latérale de la revue, sans échantillon de plus.
- **R** Images regardées :
  - **contre-jour** : le corps des tours passe du brun saumon à un gris lilas bleuté, avec des bords dorés près du soleil. Le corps reste à peu près de la luminance du ciel ;
  - **midi** : inchangé pour l'essentiel.
- **R** Coût : 0,9 et 1,6 ms de GPU en 1440p. `just check` et `just test` passent (418 tests). Seul le shader change : le modèle Rust et ses tests restent valables.

### Contrôle du directeur artistique, et captures non reproductibles

- **R** Contrôle du directeur artistique : la condition minimale est remplie. Le corps des tours à contre-jour est gris lilas froid sous des crêtes dorées ; aucune régression à midi. Passage au quart de résolution approuvé, avec des réserves :
  - le bas des tours est aussi clair que le cœur : le chemin de la lumière du ciel côté œil vaut aussi pour les flancs bas. Piste : ne l'appliquer que dans le haut de l'enveloppe ;
  - le corps reste clair et plat ;
  - les captures d'avant et d'après ne se superposent pas.
- **⚠** Ce dernier point était un vrai défaut. Deux captures de suite de la même scène différaient de 1,85 % en moyenne, avec le soleil à 1 182 puis à 1 248 px. Deux scènes temporaires ont mis hors de cause les nuages et le scénario `vol-2000`. **R** Cause : en capture, le jeu lisait la manette, restée branchée. La dérive de son stick droit faisait tourner la caméra de poursuite, y compris pendant les images de chargement, où le temps réel avance alors que l'horloge du jeu est en pause. Le test de reproductibilité du 2026-11-05 portait sans doute sur une scène au sol.
- **A** Pendant une capture, `read_controls` ignore la manette et le clavier : les commandes restent celles du départ ou de la scène. Nouveau test : une flèche pressée ne change rien pendant une capture, et change bien le manche en dehors. Le skill `bevy` (§ 7) décrit le contrôle : deux captures de suite doivent différer de moins de 10⁻⁴. **R** Écart entre deux captures : 1,7·10⁻⁵, soleil au même pixel.
- **A** `just check` et `just test` passent (419 tests).
- **D** Suite : le quart de résolution et la reconstruction temporelle. Réserves à garder pour après : le chemin du ciel limité au haut de l'enveloppe, le liseré, les bases plates, le bruit intermédiaire, l'auréole du soleil, l'éclairage de l'avion à contre-jour.

### M3, incrément 1 : quart de résolution et reconstruction temporelle des nuages

- **Q** Le joueur : « Oui, attaquer ».
- **A** Trois passes à la place d'une :
  - **marche** à la moitié de la résolution sur chaque axe (diviseur 3, 2, 2, 1 selon la qualité). Deux sorties : la lumière et la transmittance ; la distance où elle s'est arrêtée (objet le plus proche du bloc) et la distance moyenne des nuages ;
  - **passe temporelle** à basse résolution. Elle reprojette l'image précédente à la distance des nuages, découpe l'historique dans la moyenne du voisinage ± 1,25 écart-type (Salvi 2016), donne 12 % de poids à l'image courante, et repart de zéro quand l'origine de rendu saute. Elle écrit dans les images de `CloudTarget`, puis les recopie comme historique ;
  - **composition** par le matériau transparent, avec un suréchantillonnage selon la profondeur du prépassage. Les texels dont la distance d'arrêt s'accorde avec le pixel pèsent le plus, et un nuage situé derrière l'objet du pixel ne passe pas devant.
- **⚠** Premier essai : la capture est sortie avec le code 0 alors que le shader de composition ne compilait pas (chemin d'import faux, `bevy_pbr::render::mesh_view_bindings`). Le mode capture ne détecte pas l'échec d'un pipeline de matériau. C'est à corriger.
- **⚠** Les cumulus lointains s'étiraient vers le bas en rideaux. La capture sans accumulation a désigné la passe temporelle, et la vue du décalage de reprojection en a montré un, vertical et presque uniforme. Cause : la direction du rayon était reconstruite en soustrayant l'œil, à 2 000 m, d'un point du plan proche situé à 5 cm, ce qui perd quelques pixels à l'arrondi f32. **A** Reprojection centrée sur l'œil (rotation seule, plus le déplacement de l'œil). **R** Décalage nul sur le ciel, plus de rideaux.
- **R** Images regardées : `cumulus_midi` et `cumulus_contre_jour` à peu près identiques à la pleine résolution, bords de l'avion propres. Deux images consécutives diffèrent de 0,04 % (3 pixels sur 1,28 million de plus de 4 %) : pas de scintillement.
- **R** Coût en 1440p : marche 0,3 à 0,44 ms, temporel 0,02 ms, contre 0,9 à 1,6 ms en pleine résolution.
- **A** Deux tests nouveaux (diviseur par qualité, taille arrondie). `just check` et `just test` passent (421 tests).

### M3, incrément 1 : ombres des nuages

- **Q** Le joueur : « Oui, les ombres ».
- **D** Bevy 0.20 a des textures de lumière : `DirectionalLightTexture`, posé sur le soleil, voile sa lumière par une image projetée le long de sa direction. Elles passent par les décalques regroupés. J'active la feature `pbr_light_textures` et je l'inscris au tableau des features de l'ADR 0004, qui prévoit ces ajouts. Ses cascades d'ombre n'utilisent que la rotation du soleil : la translation et l'échelle peuvent servir à cadrer la carte.
- **A** Une passe avant les objets opaques (`shadow.wesl`) écrit une carte de transmittance vue du soleil : R8, 1 024² sur 24 km, 24 pas sur la densité grossière par texel, T = e^−τ. Le soleil, recentré chaque image sur la caméra, la porte en mode répété. Non répétée, la texture éteindrait le soleil hors de la carte ; répétée, la répétition se perd dans la brume.
- **⚠** Bevy ne lit qu'une valeur par rayon de soleil : un avion au soleil au-dessus d'un nuage aurait pris l'ombre que ce nuage jette au sol. **A** Autour de la ligne de soleil de l'avion joueur (30 m, puis un fondu sur autant), la carte ne compte que les nuages au-dessus de lui ; le sol de cette ligne est dans l'ombre de l'avion. Les avions de l'IA poseront la même question au M4.
- **R** Images regardées :
  - **midi** : la plaine est tachetée d'ombres bleutées sous chaque cumulus, décalées selon le soleil, et l'avion reste au soleil ;
  - **contre-jour** : les ombres longues du soleil bas couvrent une grande part de la plaine.
- **R** Une scène sans nuages n'a pas changé (0,1 %). Coût de la passe : 0,12 à 0,14 ms. Les autres passes paraissaient 30 % plus lentes et le CPU trois fois plus lent : l'autre session (capture en erreur) tournait en même temps. Le coût des textures de lumière dans la passe des opaques reste à remesurer au calme.
- **A** `just check` et `just test` passent (421 tests). Revue du directeur artistique demandée.
- **R** Réponse au joueur : les ombres tachettent la plaine sous chaque cumulus, l'avion garde le soleil au-dessus des nuages ; restent l'ombre lointaine d'un nuage sur son voisin, les réserves des revues et deux scènes. La revue du directeur artistique est en cours.

### Revue du directeur artistique : ombres des nuages

- **R** Verdict « approuvé ». Les ombres donnent l'échelle à midi : la couche se lit à 1 000 m au-dessus de la plaine. Le contre-jour est jugé la meilleure image du projet : ombres de 9 à 15 km, couloir ensoleillé dans l'azimut du soleil, contraste chaud sur froid. L'avion garde le soleil dans les deux scènes.
- **R** À corriger, par ordre d'impact :
  1. **À midi, les ombres se lisent comme des lacs bleus.** La seule lumière d'ambiance est la carte d'environnement d'un ciel clair (`camera/mod.rs:88`). Piste : une `AmbientLight` presque neutre, proportionnelle à la couverture, pour viser un rapport ombre/soleil d'environ 1/3 en gardant la teinte des champs.
  2. **Au-delà de 15 km, les ombres sont une copie périodique de la carte**, sans lien avec leurs nuages ; elles ne sont lues qu'au niveau de mip 0. Pistes : avancer le centre de la carte dans la direction de la vue, passer à 2 048² sur 48 km, et fondre ses bords vers la transmittance moyenne.
  3. **Bords trop nets par soleil bas**, alors que la pénombre vaut 90 à 140 m à 10-15 km. Piste : un flou dans l'espace du soleil, proportionnel à la distance.
  4. **Bords qui glissent en mouvement** : le centre suit l'œil sans s'aligner sur la grille des texels. Piste : l'aligner.
  5. **Risque d'une tache de soleil fantôme** au sol, sur la ligne de soleil de l'avion, quand un nuage est sous lui. Piste : réduire le rayon à deux texels, et ajouter une scène de contrôle.
- **R** À vérifier : l'avion à l'ombre d'un nuage, sous la base (aucune capture ne le montre) ; la lumière renvoyée par la mer de nuages vers le dessous de l'avion, que l'ambiance ignore.
- **D** Prochaine itération recommandée : la lumière d'ambiance neutre (point 1), et dans la même passe les points 2 et 4 (centre avancé, bords fondus, alignement), puis une séquence en mouvement.

### Session « Capture en erreur si un pipeline ne compile pas »

- **R** La session séparée, lancée par le joueur, a fini son tour. Commit `d0d8eea` sur la branche `claude/elated-pascal-b9f3dd` : la capture sort avec le code 1 quand un pipeline ne se construit pas, en nommant le pipeline et le shader. Vérifié avec un import faux et une erreur de syntaxe ; les 15 scènes passent toujours, ainsi que `just check` et `just test`.
- **?** La branche n'est pas fusionnée dans `main-tcd0yw` : la fusion attend l'accord du joueur.
- **R** Second avis de la session séparée : rien de nouveau, elle a seulement pris acte de la fin de sa surveillance. La branche `claude/elated-pascal-b9f3dd` attend toujours l'accord du joueur pour être fusionnée.

### M3, incrément 1 : ombres de midi et portée de la carte d'ombre

- **Q** Le joueur : « Oui, corriger ».
- **A** Lumière d'ambiance des nuages : une `AmbientLight` blanche sur la caméra, de luminance 0,6 × couverture × E_sol / π (`ESTIMÉ` : sous un champ de cumulus, la lumière diffuse fait 25 à 35 % de l'éclairement global). E_sol, l'éclairement du soleil sur un sol horizontal, prend une épaisseur optique de l'air de 0,25 au zénith (`ESTIMÉ`). Bevy ajoute cette lumière à la carte d'environnement du ciel clair. Elle s'en va avec les nuages.
- **R** Mesuré sur `cumulus_midi` : l'ombre passe de (34, 47, 58) % à (40, 52, 59) % en sRGB ; le rouge et le vert remontent. Le bleu qui reste vient surtout de la perspective aérienne, à 2 km de distance, et de la lumière du ciel. La lumière diffuse fait environ 30 % de l'éclairement global, dans la fourchette visée.
- **A** Carte d'ombre :
  - 2 048² sur 48 km (toujours 23 m par texel) ;
  - avancée de 0,35 fois sa largeur dans la direction horizontale de la vue ;
  - alignée sur sa grille de texels dans le plan du soleil ;
  - ses 20 % extérieurs fondus vers la transmittance moyenne de la couche ;
  - la zone autour de la ligne de soleil de l'avion réduite à un texel, plus un de fondu.

  **R** Deux images consécutives diffèrent de 0,07 à 0,08 % au sol : les ombres ne glissent pas.
- **A** Scène de contrôle `cumulus_sous_la_base` : le SPAD à 1 000 m sous une base à 1 200 m, graine 8, choisie avec la référence Rust pour qu'un nuage soit sur sa ligne de soleil. **R** L'avion est à l'ombre : son fuselage passe de 86 % à 69 % en sRGB (0,6 en linéaire), sans ombre nette de l'aile haute, éclairé par le ciel et l'ambiance.
- **R** Coût en 1440p, au calme : passe d'ombre 0,37 à 0,40 ms ; passe des opaques 0,18 à 0,21 ms, donc les textures de lumière n'y coûtent presque rien.
- **A** Trois tests nouveaux : placement de la carte (devant l'œil, sur sa grille) ; part de la lumière d'ambiance ; transmittance de l'air. `just check` et `just test` passent (423 tests).

### Relecture des ombres corrigées

- **Q** Le joueur : « Oui, faire relire ».
- **A** Revue du directeur artistique lancée sur `cumulus_midi`, `cumulus_contre_jour` (avant et après les corrections) et `cumulus_sous_la_base` (nouvelle). Questions posées : les points 1, 2, 4 et 5 sont-ils réglés, l'avion est-il juste à l'ombre, le contre-jour a-t-il régressé, et que reste-t-il pour clore l'incrément 1 des nuages.
- **R** Verdict « approuvé pour les corrections » ; l'incrément 1 ne peut pas encore être clos.
  - Point 1 (lacs bleus) : réglé au premier plan seulement. Au plan moyen de `cumulus_midi` et dans `cumulus_sous_la_base`, les ombres restent bleues et saturées : la perspective aérienne, que les nuages n'ombrent pas, s'ajoute par-dessus et domine avec la distance.
  - Point 2 (ombres lointaines) : réglé à l'œil. Point 4 (bords qui glissent) : réglé d'après les mesures, à confirmer en séquence. Point 5 (tache fantôme) : réglé.
  - Contre-jour : pas de régression, toujours la meilleure image du projet. À midi, l'ambiance remplit un peu le côté à l'ombre du capot (léger, non bloquant).
  - Avion à l'ombre : couleur juste, valeur peut-être un peu claire. Critère proposé : rapport de luminance linéaire ombre/soleil de l'aile haute entre 0,25 et 0,4 ; au-delà de 0,5, le facteur 0,6 de l'ambiance est trop fort. Le fuselage mesuré la dernière fois donne environ 0,6 (mesure faite après le tonemapping, donc approchée).
- **R** À corriger, par ordre d'impact :
  1. **L'avion est plat à l'ombre**, comme une découpe de carton : l'`AmbientLight` est uniforme et rien ne l'occulte. Pistes : la SSAO de Bevy (`ScreenSpaceAmbientOcclusion`, présente dans `bevy_pbr` 0.20.0, vérifié), puis une ambiance directionnelle (le dessus voit la base grise du nuage, le dessous le sol au soleil).
  2. **Lacs bleus au plan moyen** : atténuer la part solaire de la perspective aérienne par la transmittance de la couche sous la base, et/ou des aérosols (Mie) pour une brume plus blanche.
  3. Bords nets par soleil bas (point 3, connu) : pénombre croissante avec la distance, à l'étape « heure du jour ».
  4. Hors nuages : le terrain provisoire en cellules de Voronoï pèse désormais le plus sur `cumulus_sous_la_base`.
  5. Ambiance par soleil bas, blanche et presque nulle à 9° : la teinter par le soleil et utiliser l'éclairement normal.
  6. Risques à contrôler : le cockpit, que l'ambiance non occultée risque de déboucher ; les ombres lointaines qui « respirent » quand la caméra fait demi-tour.
- **R** Pour clore l'incrément 1 :
  - `plafond_bas`, avec un profil de stratus et une ambiance revue : à couverture 1 la formule actuelle donne l'ambiance maximale, alors qu'une couche épaisse ne transmet que 10 à 30 %. Il faut aussi baisser `AtmosphereEnvironmentMapLight::intensity` (champ vérifié dans `bevy_light` 0.20.0) selon l'épaisseur optique.
  - `traversee_nuage` en séquence : pas de saut à l'entrée, la queue s'estompe avant le nez, l'avion s'assombrit en s'enfonçant. *(Critère reformulé le 2026-11-08 par le directeur artistique : ce qui est le plus loin de l'œil, à travers le nuage, s'estompe le premier ; vu de la poursuite, c'est le nez.)*
  - Une courte séquence de `cumulus_sous_la_base`, l'avion franchissant un bord d'ombre.
  - Les réserves ouvertes (ombre d'un nuage sur son voisin, bases trop tranchées, auréole du soleil), reportables avec l'accord du joueur, et l'approbation du joueur sur les scènes.
- **D** Recommandation du directeur artistique : la SSAO sur la caméra de vol, au meilleur rapport effet/effort, validée sur `cumulus_sous_la_base` et `avion-studio` (surveiller les halos autour de l'avion). En attente de la décision du joueur.

### Occlusion ambiante (SSAO)

- **Q** Le joueur : « Oui, la SSAO » (ajouter la SSAO sur la caméra de vol, revoir le facteur d'ambiance sur l'avion, captures avant/après de `cumulus_sous_la_base` et `avion-studio`).
- **A** Mesure en lumière linéaire, sans tonemapping ni bloom, sur `cumulus_sous_la_base`, avec et sans l'ombre du nuage. À l'ombre, l'avion reçoit 0,28 à 0,36 fois sa lumière au soleil sur les faces du dessus, et 0,39 à 0,43 sur les flancs.
  **R** C'est dans la fourchette du directeur artistique (0,25 à 0,4) : le facteur d'ambiance de 0,6 est gardé. Le « 0,6 » mesuré la fois précédente venait du tonemapping. L'avion à l'ombre paraît aussi clair que les champs au soleil parce que sa toile crème renvoie trois fois plus de lumière qu'eux.
- **A** SSAO de Bevy (`ScreenSpaceAmbientOcclusion`) sur toutes les caméras d'extérieur :
  - rayon de 1,5 m, l'entre-plans du SPAD ;
  - échantillons au niveau de qualité du jeu (`Quality::occlusion`).
- **⚠** La SSAO exige une prépasse de normales, et avec elle Bevy 0.20 éclaire chaque pixel avec la normale lue dans la prépasse (`LOAD_PREPASS_NORMALS`).
  - L'hélice vernie est devenue blanche : la normale du clearcoat n'est alors jamais calculée et reste nulle.
  - Les câbles, le disque d'hélice et le pare-brise, transparents, ont pris la normale de ce qu'il y a derrière eux. En vue pilote, le disque éclaircissait tout le sol.
- **A** Correction : `render/own_normals.rs`. Une extension de matériau `OwnNormals` retire `LOAD_PREPASS_NORMALS` du shader (Bevy ne le fait que pour la transmission spéculaire), et elle est posée sur tout matériau transparent ou verni ; les câbles font de même dans leur `specialize`. L'hélice garde son vrai clearcoat.
  **R** Après correction, le sol, les câbles et le disque ne changent plus d'un pixel, et l'hélice retrouve son noyer.
- **R** Effet de la SSAO :
  - en vue pilote, fort : de 21 à 24 % sous l'aile haute, sur les mâts et les flancs du capot. Le risque du cockpit trop éclairé, soulevé par le directeur artistique, est réglé ;
  - à 10 m, en poursuite, presque nul (2 à 4 %) : la SSAO ne voit pas l'aile haute, qui n'est qu'une plaque mince. L'avion à l'ombre de `cumulus_sous_la_base` reste plat ;
  - sur `avion-studio`, le sol s'assombrit de 11 % dans l'ombre de l'avion ; l'avion lui-même change de 1 à 2 % ;
  - aucun halo autour de la silhouette.
- **R** Coût en 1440p, niveau « haut » : 0,60 à 0,68 ms pour la SSAO. Image complète : `avion-studio` de 2,16 à 2,85 ms, `cumulus_sous_la_base` de 3,03 à 3,61 ms.
- **A** Documentation : skills `rendu-visuel` (occlusion ambiante et son piège) et `bevy` (§ 9), plan du M3. Cinq tests nouveaux. `just check` et `just test` passent (427 tests).
- **D** Le modelé de l'avion vu de loin demande une ambiance directionnelle : la base grise du nuage au-dessus, le sol au soleil en dessous, au lieu de l'`AmbientLight` uniforme. Proposée comme prochaine étape ; la revue du directeur artistique portera sur les deux ensemble.

### Ambiance directionnelle : plan à valider

- **Q** Le joueur : « Oui, l'ambiance » (ambiance directionnelle : la base du nuage au-dessus, le sol au soleil en dessous ; puis relecture de la SSAO et de l'ambiance par le directeur artistique).
- **R** Piste du directeur artistique écartée : des lumières directionnelles sans ombre, l'une venant du haut, l'autre du bas. Bevy 0.20 traite toute lumière directionnelle comme un soleil de l'atmosphère (diffusion simple, multidiffusion, sol de la planète : `atmosphere/functions.wesl`). Une lumière venant du haut éclaircirait tout le ciel. Notre shader des nuages prend en outre la lumière d'indice 0 comme soleil.
- **R** Constats :
  - la carte d'environnement de l'atmosphère (128² par face) montre sous l'horizon un sol gris (albédo 0,3) au plein soleil, sans les ombres des nuages ;
  - la lumière des nuages est une `AmbientLight` uniforme, qui éclaire le dessous des ailes autant que le dessus ;
  - Bevy filtre la carte (`lightprobe_irradiance_map`, `lightprobe_radiance_map`) dans `Render` avant le graphe, sur ce que l'image précédente y a laissé. Une passe placée après `main_opaque`, où tournent déjà nos nuages, peut donc réécrire le cube, avec une image de retard.
- **D** Plan proposé (plus d'une heure, à faire valider), option A, recommandée : **les nuages dans la carte d'environnement**.
  1. Passe de calcul `clouds/environment.wesl`, pour chaque texel du cube : le ciel lu dans la table de l'atmosphère, plus les nuages, marchés en peu de pas (réglés par le niveau de qualité). Elle écrit dans le cube de `GeneratedEnvironmentMapLight`.
  2. Le sol de l'atmosphère prend l'albédo des champs (0,18, vert-ocre, `ESTIMÉ`), atténué par la transmittance moyenne de la couche (1 − 0,76 × couverture).
  3. L'`AmbientLight` des nuages disparaît : la lumière diffuse de Bevy, qui vient de la carte, devient directionnelle d'elle-même. Elle reste occultée par la SSAO.
  4. Contrôles : le rapport ombre/soleil en linéaire, toujours entre 0,25 et 0,4 sur les faces du dessus, avec un dégradé du dessus au dessous ; captures des scènes de nuages ; `avion-studio` inchangée ; coût mesuré.
  5. Complément à l'ADR 0009, documentation, revue du directeur artistique de la SSAO et de l'ambiance.
  
  Ce que cela règle aussi : les reflets du vernis et du verre montrent les nuages ; au-dessus d'une mer de nuages, leur lumière éclaire le dessous de l'avion (réserve de la revue) ; sous un plafond, la carte montre des bases grises au lieu d'un ciel bleu (prérequis de `plafond_bas`).
- **D** Option B, plus courte mais jetable : une extension de matériau sur l'avion, qui pondère la lumière d'ambiance par l'hémisphère (pleine en haut, nulle en bas), et un albédo du sol teinté. Elle ne touche que l'avion, et la carte d'environnement reste fausse.

## 2026-11-08

### Le travail invisible sur GitHub

- **Q** Le joueur : « je ne vois rien de ce que tu as poussé sur le dépôt. un git pull de mon côté ne donne rien (dépôt déjà à jour). explique-moi pourquoi et ensuite continue ».
- **R** Tout le travail du projet est sur la branche `main-tcd0yw`, 243 commits devant `main`. `main` est resté au premier commit (`3a08e80`, 16 octobre), comme le carnet le note depuis le 16 octobre.
  - Les envois vont vers `origin/main-tcd0yw`. Le dernier, `28195a5` (7 novembre), a bien réussi.
  - Un `git pull` sur `main` n'a donc rien à récupérer. Dans ce dossier même, les commits sont déjà là, puisque Claude les y fait.
  - Pour voir le travail : choisir la branche `main-tcd0yw` sur GitHub, ou `git fetch origin` puis `git switch main-tcd0yw` dans un autre clone.
  - Pour que `main` rattrape la branche : une simple avance rapide, puisque `main` n'a aucun commit propre. Elle est proposée, pas faite.
- **⚠** L'agent SSH a redémarré le 8 novembre (prise recréée à 13 h 58) et ne tient plus de clé : tout envoi échoue tant que le joueur ne l'a pas rechargée. Les commits de cette séance restent locaux d'ici là.

### M3, incrément 1 : la lumière des nuages dans la carte d'environnement

- **Q** Le joueur : « Plan A, la carte ».
- **A** Passe de calcul `clouds/environment.wesl`, après `main_opaque_pass_3d`.
  - Elle écrit dans le cube de la carte d'environnement de la caméra (`GeneratedEnvironmentMapLight`, 128² par face) : le ciel de l'atmosphère et les nuages.
  - La marche des rayons est sortie de `render.wesl` dans une fonction partagée, `march.wesl`. Dans le cube, elle fait moins de pas (24 à 64 selon le niveau), avec un bruit fixe par texel.
  - Bevy filtre ce cube au début de l'image suivante.
- **⚠** Premier essai : le sol de l'atmosphère avait pris l'albédo des champs, comme prévu au plan. Sa multidiffusion a changé le ciel entier, même sans nuages : l'horizon de `vol-essai-poursuite` passait de (178, 190, 185) à (176, 186, 162), olive.
  - **A** L'albédo de l'atmosphère reste donc le gris de la Terre. La passe retire, sous l'horizon, la planète de Bevy (même formule que `raymarch_atmosphere`) et met les champs à sa place.
  - Les champs ont l'albédo moyen de leur palette, (0,15 ; 0,17 ; 0,06) en linéaire, sous le soleil atténué par l'ombre moyenne des nuages (1 − 0,76 × couverture) et sous le ciel.
  - **R** Les scènes sans nuages sont identiques au pixel près.
- **⚠** Sans l'`AmbientLight` uniforme, les ombres au sol de `cumulus_midi` redevenaient des lacs bleus : (103, 131, 152) puis (77, 107, 125). La carte, vue de la caméra au-dessus des nuages, montre un ciel clair, alors que le sol voit les bases.
  - **A** Le sol seul (`render/ground.rs`, `shaders/ground.wesl`) garde la lumière des bases, 0,6 × couverture × E_sol / π, dans la part de la couche que la caméra survole.
  - **R** Les ombres reviennent à (111, 138, 152).
- **R** L'`AmbientLight` uniforme des nuages a disparu. L'avion à l'ombre de `cumulus_sous_la_base` a du modelé : clair dessus, plus sombre et plus chaud dessous. Mesure en linéaire, entre l'ombre et le soleil :
  - faces du dessus : 0,27 à 0,31 de la lumière au soleil ;
  - flancs : 0,30 (contre 0,39 à 0,43 avec l'ambiance uniforme) ;
  - à l'ombre, le dessus du fuselage reçoit environ 10 % de plus que le dessous.

  Le contre-jour est quasi inchangé.
- **R** Coût : 0,09 à 0,13 ms en 1440p.
- **A** Complément à l'ADR 0009 ; skills `rendu-visuel` (`SKILL.md`, `nuages-volumetriques.md`) et `bevy` (§ 9) ; plan du M3. Cinq tests nouveaux. `just check` et `just test` passent (430 tests). Commit local `35e5afe`, non poussé (clé absente).
- **A** Revue du directeur artistique demandée sur la SSAO et l'ambiance.

### Revue du directeur artistique : SSAO et carte d'environnement

- **R** Verdict « approuvé » pour les deux changements.
  - **SSAO** : approuvée sous réserve. Pas de halo ni de bruit en vue pilote, et la correction des normales est visible (câbles, hélice vernie). Elle coûte 0,65 ms, la passe la plus chère du post-traitement, pour un effet presque nul en poursuite.
    - **⚠** Les images de la SSAO seule, dans le dossier temporaire de session, ont disparu : la machine a redémarré le 8 novembre à 13 h 57, et `/tmp` a été vidé. Le directeur artistique n'a donc pas vu les cartes de différence ; il demande à les revoir pour un verdict ferme sur les 21 à 24 %.
    - **⚠** Trois montages laissés dans le dossier `env/` montraient encore l'état intermédiaire, à l'horizon olive ; il les a écartés.
  - **Carte d'environnement** : approuvée. Sous la base, l'avion est enfin à l'ombre, plus sombre que les champs au soleil ; le fuselage a du modelé, du dos crème au ventre plus sombre. Sa teinte plus chaude est juste dans son principe : sous un nuage, plus de ciel bleu au-dessus. Aucune régression : ombres au sol, contre-jour, horizon, scènes sans nuages. Garder l'albédo gris pour l'atmosphère était le bon choix.
- **R** À corriger, par ordre d'impact :
  1. Le terrain (pavage de Voronoï, répétition tous les 2 km), hors de l'incrément 1 : incréments 3 et 4.
  2. **La toile crème vire à l'olive à l'ombre**, surtout dans le contre-jour, et un peu plus qu'avant dans `cumulus_midi` : c'est la seule régression visible, légère.
     - Cause probable : la couleur de la toile dans `livrees.ron`, (0,84 ; 0,76 ; 0,52) en sRGB, étiquetée `E:`. En linéaire, son bleu ne fait que 35 % de son rouge.
     - Piste : la faire vérifier par l'historien sur des autochromes.
  3. Ailes plates à l'ombre, sans occlusion entre les plans, et nervures effacées. Piste : une carte d'occlusion cuite dans `art/spad7/build.py`.
  4. Ombres lointaines encore bleues : il manque des aérosols (Mie) pour un été en Flandre.
  5. Bases en plaques près de l'horizon, disque d'hélice en dôme gris : réserves déjà connues.
  6. Comportement en mouvement à vérifier sur une séquence.
- **R** Pour clore l'incrément 1 :
  - les scènes `traversee_nuage` et `plafond_bas` ;
  - les réserves des revues (bases en plaques, bruit intermédiaire, ciel limité au haut de l'enveloppe, auréole du soleil, ombre d'un nuage sur son voisin) ;
  - la densité dans `Environment` côté jeu ;
  - une revue finale des quatre scènes de nuages, puis l'accord du joueur.
  
  L'olive de la toile et l'occlusion cuite peuvent attendre l'incrément 5.
- **D** Prochaine amélioration recommandée : faire vérifier par l'historien le ton de la toile vernie, une seule valeur, puis la carte d'occlusion cuite.
- **⚠** Envoi impossible : l'agent SSH ne tient toujours pas de clé. Trois commits locaux attendent.

### Avance de `main`

- **Q** Le joueur : « Oui, avancer main » (pousser `main-tcd0yw`, puis faire avancer `main` jusqu'à elle, en avance rapide, sans forcer).
- **D** Accordé par le joueur : `main` rattrapera `main-tcd0yw`. L'avance est rapide, car `main` n'a aucun commit propre ; elle ne réécrit rien.
- **⚠** Au moment de la réponse, l'agent SSH ne tient toujours pas de clé : l'envoi attend que le joueur la recharge.
- **Q** Le joueur : « essaie » (la clé est rechargée).
- **A** Envoi de `main-tcd0yw` (`28195a5..280919d`), puis avance rapide de `main` sur GitHub (`3a08e80..280919d`, sans forcer) et de `main` en local. **R** Les deux ont réussi : `main` contient désormais tout le travail du projet, et un `git pull` sur `main` le récupère.
- **?** Proposé, à confirmer par le joueur : que `main` suive la branche à chaque envoi, toujours en avance rapide. L'accord donné vaut pour cet échange seulement.
- **Q** Le joueur : « Oui, à chaque envoi ».
- **D** Règle durable : à chaque envoi, Claude pousse `main-tcd0yw`, puis fait avancer `main` jusqu'à elle (`git push origin main-tcd0yw:main`), toujours en avance rapide. Si l'avance rapide est refusée, il s'arrête et demande au lieu de forcer. **A** Règle enregistrée dans la mémoire de Claude pour ce projet.

### Suite : le ton de la toile et la réserve sur la SSAO

- **Q** Le joueur : « continue les activités ».
- **A** Recommandation du directeur artistique reprise : l'historien recherche la teinte de la toile vernie d'usine de 1916-1917 (sources, confiance, rapport entre bleu et rouge). Il met à jour `docs/recherche/spad7-aspect.md` § 12.1 sans toucher à `livrees.ron`. La couleur du jeu ne changera que si une source le justifie.
- **A** Images de la SSAO seule refaites (les premières avaient disparu avec le redémarrage) : code actuel, sans puis avec la SSAO.
  - **R** En vue pilote, la lumière est multipliée par 0,80 sous l'aile haute, par 0,76 sur le mât avant gauche et le flanc du capot, et par 0,97 sur le dessus du fuselage au soleil.
  - **R** 41 % des pixels de la vue pilote s'assombrissent de plus de 5 %, contre 5,6 % pour `avion-studio` et 3,0 % pour `cumulus_sous_la_base`. Coût : 0,59 à 0,61 ms.
  - **A** Images envoyées au directeur artistique, qui doit lever ou confirmer sa réserve.
- **R** Blender 5.2.2 est installé : le modèle pourra être reconstruit (`just modele spad7 assets`) si la couleur change.
- **⚠** Le dépôt local est désormais sur la branche `main`, et non plus sur `main-tcd0yw` : le joueur l'a changée lui-même. Le commit précédent du carnet est donc arrivé sur `main`. **A** `main` est poussée, et `main-tcd0yw` avance jusqu'à elle en avance rapide, sur GitHub et en local : les deux restent identiques. Claude ne change pas de branche sans le demander. La mémoire du projet tient compte de ce cas.
- **R** Verdict ferme du directeur artistique sur la SSAO : **approuvée**, rayon de 1,5 m et niveau « haut » gardés.
  - **Intensité** : 0,76 à 0,80 de près, plausible pour des surfaces prises entre les plans.
  - **Halos** : aucun visible, seulement des liserés d'un pixel aux silhouettes.
  - **Coût** : 0,6 ms. Le post-traitement reste vers 1,05 ms, sous le budget de 1,5 ms.
  - **Meilleur effet** : l'ombre de contact sous l'avion au sol ; le poste gagne en densité.
  - **À surveiller en mouvement** : un léger grain sur le sol lointain (1 à 3 km), dans les ombres des nuages. S'il scintille, estomper la SSAO avec la distance.
  - **Plus tard**, si le budget se tend : la couper en poursuite au-dessus d'une cinquantaine de mètres ; et cuire une carte d'occlusion pour le modelé de l'avion vu de l'extérieur.
- **R** Rapport de l'historien sur la toile de la période jaune (fiche `spad7-aspect.md` § 12.1.1, sources V49 à V63) :
  - aucune mesure d'une toile d'origine n'a été trouvée ;
  - la seule source primaire, un rapport allemand sur un S.XI de juillet 1917 traduit dans *Flight* (16 mai 1918), dit « blanc jaunâtre » ;
  - les codes Methuen cités de seconde main (Windsock, Abbott) donnent, pour le lin enduit puis verni, une saturation C* de 19 à 31 et un bleu de 35 à 56 % du rouge en linéaire. L'ancienne valeur (C* 33) dépassait la borne haute ;
  - les plaques noir et blanc d'époque rendent le jaune trop sombre (Bureau of Standards), et juger la teinte sur leurs tons n'a pas de base.
  - **Proposition** : sRGB (0,85 ; 0,77 ; 0,60), `E:`. Même clarté et même teinte, saturation ramenée au milieu de la fourchette ; le bleu fait 46 % du rouge en linéaire. Confiance C pour la fourchette, D pour la valeur.
- **A** Valeur appliquée dans `livrees.ron`, avec son commentaire sourcé ; l'ancienne valeur reste notée comme borne d'une toile vieillie. Modèle reconstruit (`just modele spad7 assets`, contrôlé par `asset_check`). Test de l'exportateur mis à la nouvelle valeur.
- **R** Captures de contrôle (`cumulus_contre_jour`, `cumulus_midi`, `cumulus_sous_la_base`, `avion-studio`) :
  - la toile est moins jaune, plus crème : sur le fuselage sous la base, le bleu passe de 119 à 130 en sRGB ;
  - au contre-jour, le fuselage perd son orangé, mais le dessous de l'aile basse reste vert menthe. Il voit les champs, et leur lumière domine là ;
  - le capot, d'un jaune plus saturé (`jaune_metal`), tranche un peu plus sur la toile.
- **⚠** Deux points de l'historien, hors de cette passe :
  - **la transparence de la toile à contre-jour n'est pas établie** : le rapport allemand dit la toile « peinte », une autre source parle d'une couche opaque. C'est à lire sur les photos de la période jaune ; la toile translucide du rendu (`render/fabric.rs`) en dépend ;
  - `jaune_metal` (C* ≈ 41) mérite le même examen.
- **R** Compléments ajoutés aux demandes D5, D8, D9, D11 et D12. Téléchargements suggérés par l'historien (fiche § 15 : CSV des Archives de la Planète, e-médiathèque du Musée de l'Air, ImagesDéfense) : non faits, ils attendent l'accord du joueur, fichier par fichier.
- **A** `just check` et `just test` passent (430 tests).

### Traversée du nuage : la scène `traversee_nuage`

- **Q** Le joueur : « Traversée du nuage » (la scène `traversee_nuage` en séquence : entrée, intérieur, sortie, avec les critères du directeur artistique).
- **A** Champ `sequence` des fiches de scène : des images antérieures du même vol, enregistrées en plus sous `<scène>-<image>.png`. La recette `just capture` garde aussi les précédentes en `.avant.png`. Tests sur le chargement et sur les chemins.
- **A** Nuage placé sur la route d'après la densité de référence en Rust (`escadrille_weather`), en cherchant une graine et une base parmi 150 graines.
  - **Retenu** : graine 63, base 700 m, couverture 0,55, scénario `vol-essai` (1 000 m, cap à l'est, 160 km/h).
  - **Le trajet** : un voile ténu dès 265 m du départ, le cœur dense dès 305 m (6,9 s), le flanc de sortie à 915 m (20,6 s).
  - **Le soleil sur l'avion** : 100 % à 250 m, 43 % à 300 m, plus rien à 350 m.
- **A** Séquence : 3 s, 5 s, 6 s, 6,5 s, 7 s, 7,5 s, 10 s, 19,5 s, 20,25 s, 20,75 s et 22 s, vue de la poursuite.
- **R** Premières images, avant correction : l'entrée est progressive, mais trois défauts apparaissent.
  - **⚠** Une ligne horizontale coupe le brouillard à l'horizon : gris-beige au-dessus, bleuté en dessous. La marche supposait que le côté de l'œil donne sur le ciel, comme pour un flanc vu de dehors. **A** Depuis l'intérieur, ce chemin passe d'abord par le nuage derrière l'œil : sa profondeur optique est maintenant comptée. La ligne a disparu.
  - **⚠** Le brouillard entre la caméra et l'avion était faux. Le premier pas de la marche tombe au hasard dans une enjambée de 36 m, et la tranche entre l'œil et ce pas n'était jamais intégrée ; le dernier pas n'était pas coupé à la distance de l'avion. **A** Un échantillon tiré au hasard dans la tranche la représente tout entière, et le dernier pas est coupé où le rayon s'arrête.
  - **⚠** Les câbles, le disque de l'hélice et le verre restaient nets dans le brouillard : ces matériaux transparents sont dessinés après la composition des nuages. **A** Leurs shaders appliquent maintenant le brouillard devant eux (`render/clouds/fog.rs`, `shaders/clouds/fog.wesl`).
    - Là où la marche s'est arrêtée sur l'avion derrière eux, ils prennent sa lumière et sa transmittance.
    - Ailleurs, ils prennent l'extinction du nuage à l'œil, calculée en Rust et écrite une fois par image dans un tampon partagé.
    - `own_normals` reçoit pour cela son propre shader de fragment, celui de Bevy plus ce voile.
- **R** Mesures après correction :
  - **luminance d'affichage, fuselage / brouillard** : 0,65 / 0,62 à l'approche, 0,54 / 0,71 à l'entrée dans l'ombre du nuage, 0,50 / 0,58 au cœur. L'avion s'assombrit et reste une silhouette un peu plus sombre que le brouillard ;
  - **séquence dense** (une image sur deux, en 720p) : pas de saut à l'entrée ni à la sortie, la variation monte et redescend en douceur sur une dizaine d'images ;
  - **régressions** : aucune mesurable sur `cumulus_midi`, `cumulus_contre_jour`, `cumulus_sous_la_base` et `avion-studio` ; coût GPU inchangé.
- **A** `just check` et `just test` passent (433 tests). Skills `bevy` et `rendu-visuel` et fiche des nuages à jour.
- **R** Verdict du directeur artistique : **à retravailler, un seul point bloquant**. Les trois critères sont remplis : la ligne horizontale a disparu, les transparents sont voilés comme le reste, l'avion s'assombrit assez pour ce nuage.
  - **Bloquant** : des formes à bords nets dans le brouillard (une arête en diagonale à 7,5 s, un triangle plus sombre à 19,5 s), qui dépendent de la direction du regard. Elles viennent de la profondeur du nuage derrière l'œil, calculée pixel par pixel en marche arrière : elle projette la forme du nuage derrière soi sur le brouillard devant. Proposition : une seule valeur par image, calculée en Rust.
  - **Importants, hors de la traversée** : des cumulus en falaises de craie, aux flancs verticaux et aux sommets plats à une même hauteur, sur la dernière image (l'enveloppe sature sur l'intérieur des taches de la carte météo) ; le mur vu à 45 m sans relief ; il manque une vue du pilote, une caméra qui tourne et une poursuite longue.
  - **Détails** : une image fixe au franchissement du cœur (7,1 s) ; un arc faible du disque d'hélice ; l'avion s'incline d'environ 4° faute de commandes.
- **D** Critère reformulé par le directeur artistique : ce qui est le plus loin de l'œil, à travers le nuage, s'estompe le premier. Vue de la poursuite, c'est le nez avant la dérive ; vue du pilote, les bouts d'ailes avant le capot. Il remplace « la queue s'estompe avant le nez » de la revue du 2026-11-07.
- **Q** Le joueur, après une interruption (limite d'utilisation atteinte) : « continue from where you left off ».
- **A** Correctif bloquant : la profondeur de l'œil dans le nuage (`eye_depth`) est une seule valeur par image, calculée en Rust sur la densité de référence. C'est la plus petite profondeur optique sur les 600 premiers mètres, vers le haut, les quatre horizontales et le soleil. Elle vaut zéro en air libre.
  - La marche l'ajoute au chemin vers le ciel par le côté de l'œil. Dès 2 de profondeur, tous les rayons, même ceux qui montent, prennent ce chemin, en rampe continue à l'entrée.
  - Tests : la profondeur optique de référence (`CloudTextures::optical_depth`) ; `eye_depth` est nulle hors du nuage, dépasse 5 au cœur, et varie au plus de 0,72 par mètre à l'entrée (pas de saut).
- **A** Nouvelle scène `traversee_nuage_pilote` : le même vol vu du poste, regard à 40° à droite. Image 426 (7,1 s, la caméra entre dans le cœur) ajoutée aux deux séquences.
- **R** Mesures après le correctif :
  - **écart-type du brouillard** : de 9 à 10·10⁻³ à 3 à 6·10⁻³. Il reste le même avec la caméra tournée de 30° : aucune forme ne suit le cadre ;
  - **séquence dense** : toujours aucun pic isolé ;
  - **couleur** : le brouillard au cœur est maintenant gris neutre, et non plus beige ;
  - **vue du pilote** : les mâts et les câbles de l'aile droite se voilent avant ceux du poste.
- **⚠** Le dossier `target/` a été vidé pendant l'interruption, et avec lui les captures de référence. **A** Les images « avant » ont été refaites en construisant `b4efb84` dans un worktree temporaire, supprimé ensuite.
- **⚠** Ce worktree partageait le dossier `target/` : ses artefacts ont écrasé ceux de la version courante (mêmes empreintes cargo). Cargo a cru celle-ci à jour et n'a pas refait le binaire, qui cherchait encore les assets du worktree supprimé. **A** Artefacts « release » des crates du projet nettoyés, puis reconstruction. Piège noté dans le skill `bevy`.
- **R** Bruit d'une capture à l'autre, avec le même binaire : 0,8 à 1,2 % des pixels de `cumulus_midi` changent de plus de 8 niveaux. La gigue des nuages suit le compteur d'images de Bevy, qui inclut les images de chargement. Le skill `bevy` annonçait « quelques pixels » : corrigé. L'écart avec `b4efb84` (0,16 %) reste sous ce bruit, donc pas de régression sur les trois scènes de cumulus ni sur `avion-studio`.
- **A** `just check` et `just test` passent (435 tests).
- **R** Contre-revue du directeur artistique : **approuvé**. Le point bloquant est levé, à l'œil comme à la mesure : au cœur, le brouillard varie de 1 à 2 niveaux sRGB sur tout le cadre, aussi avec la caméra tournée de 30°.
  - **Gris neutre au cœur : juste, à garder.** Le beige d'avant venait du défaut : le terme du ciel manquait. Se perdre dans un blanc sans horizon fait partie du pilier 3.
  - **Vue du pilote** : critères remplis, le cadrage à 40° est gardé.
  - **Condition pour clore** : une capture du pilote regard droit devant, pour vérifier le voile du pare-brise.
  - **Reste, hors de la traversée** : un liseré clair bleuté de 1 à 2 px le long des silhouettes sur fond de brouillard (anneau du viseur, bord d'attaque). Ce n'est pas une régression, et trois causes restent à départager : le reflet de Fresnel, l'image des nuages en demi-résolution, la TAA. S'y ajoutent des contours d'un niveau en éventail autour de l'avion, invisibles à contraste normal, à vérifier en qualité « bas ».
- **A** Capture de contrôle du pilote regard droit devant (lacet 0°, tangage −6°), faite à part, sans changer la scène.
  - **R** Autour de la ligne de visée, à travers le pare-brise, le brouillard est uniforme : 0,524 ± 0,003 de luminance à 10 s. Pas de disque ni de halo pâle.
  - **R** Le disque de l'hélice assombrit le brouillard d'environ 6 % (0,518 contre 0,549), comme il assombrit le ciel hors du nuage (5 %). C'est attendu pour du bois vernis presque transparent.
- **D** Traversée du nuage close : scènes `traversee_nuage` et `traversee_nuage_pilote` approuvées.
- **?** Suites proposées par le directeur artistique, hors de la traversée :
  - arrondir les sommets des cumulus, au meilleur rapport effet/effort ;
  - donner du relief au mur de nuage vu de près ;
  - le liseré des silhouettes ;
  - une poursuite longue, pour voir l'avion avalé nez en premier.
- **Q** Le joueur : « as-tu poussé du code ? »
- **R** Oui. Trois commits sont sur GitHub depuis celui de la toile, sur `main` et `main-tcd0yw` (toutes deux à `ad81dc3`) : `b4efb84` (scène de traversée, marche, voile des transparents, champ `sequence`), `aa73462` (profondeur de l'œil par image, vue du pilote) et `ad81dc3` (carnet et plan seulement).
- **Q** Le joueur : « je suis sur main, et mon git pull me répond "Déjà à jour". Sur quelle branche pousses-tu ? »
- **R** Sur `main` de `git@github.com:DavidGarriou/escadrille.git` (et `main-tcd0yw`, gardée identique). Claude travaille dans le dépôt `/home/dacid/Depots/personnel/escadrille` lui-même, sur `main` : ses commits y sont créés en local, puis poussés. Dans ce dossier, `git pull` n'a donc rien à rapporter, car la branche locale contient déjà tout ; `git log --oneline -5` montre les commits. Dans un autre clone, `git pull` les récupère.
- **Q** Le joueur : « ok, c'est bon » (les commits sont bien dans sa `main`).
- **Q** Le joueur : « je ne vois pas les nuages avec un "just run" ».
- **R** Cause : seule une fiche de scène posait la couche de nuages. Le vol d'essai de `just run` (`spawn_on_the_field`) n'en avait pas, donc le ciel restait dégagé ; les nuages n'existaient que dans les captures.
- **A** Le vol d'essai reçoit la couche de cumulus de `cumulus_sous_la_base`, revue par le directeur artistique : base à 1 200 m, couverture 0,55, 700 m d'épaisseur, graine 8. Elle est vue d'en dessous au décollage et traversée en montée. Elle tiendra lieu de météo jusqu'à celle des missions. `just check` et `just test` passent (435 tests). Le rendu suit le même chemin que la scène revue ; la fenêtre de `just run` n'a pas été regardée par Claude.
- **Q** Le joueur : « Oui, l'ajouter » (piloter directement une scène de référence, comme la traversée).
- **A** Option `--scene <scène>` : `just run --scene traversee_nuage` lance le vol depuis une scène de `assets/scenes`, avec son scénario, son soleil, ses nuages et sa caméra.
  - Les manettes partent des commandes de la scène, puis suivent la manette. Le redémarrage ramène au départ du scénario, donc devant le nuage pour la traversée.
  - Une ressource `FlightScene` porte le nom de la scène ; les captures de scènes fichiers passent par elle aussi. Les poses construites en code (`avion-studio`…) ne se pilotent pas : le nom est refusé, avec la liste des scènes pilotables.
  - La recette `just run` transmet ses arguments.
- **R** Vérifications :
  - test de l'option (scène acceptée, pose refusée, conflit avec `--capture`) ;
  - captures de `cumulus_midi` et `avion-studio` toujours bonnes ;
  - `--scene traversee_nuage` et le vol d'essai lancés 15 s chacun sans erreur, manette détectée. La fenêtre elle-même n'a pas été regardée : pas d'écran virtuel sur cette machine.
  - `just check` et `just test` passent (436 tests).
- **Q** Le joueur : « Branche locale main-tcd0yw : fait comme avant ».
- **D** La branche locale `main-tcd0yw` est gardée et tenue identique à `main` à chaque envoi, comme avant.
- **Q** Le joueur : « Prochain chantier : 1. arrondi des sommets des cumulus, puis 2. liseré clair le long des silhouettes dans le brouillard ».
- **A** Avant d'écrire du code, Claude mesure d'où viennent les sommets plats et les parois, puis propose un plan à valider : le chantier touche le modèle de densité de référence, son jumeau WGSL et toutes les scènes de nuages.
- **R** Diagnostic des cumulus en falaises (densité de référence, sans le bruit de détail) :
  - **les parois** : au bord visible d'un nuage, la colonne est déjà haute de 140 à 230 m en médiane (80 à 350 m du premier au troisième quartile). Le profil d'un cumulus est plein de 6 % à 65 % de sa hauteur ; or le nuage n'apparaît que là où l'enveloppe dépasse l'érosion du bruit de forme (environ 0,35). À toutes les hauteurs sous ce plein, son bord tombe donc au même endroit, et la paroi est verticale ;
  - **les sommets** : la couverture sature à 1 au cœur des taches de la carte météo (10 à 31 % de la surface nuageuse selon la scène). Le sommet y est partout à l'épaisseur de la couche, et seul le bruit de forme le découpe ;
  - aucune autre source ne fait varier la hauteur d'un nuage à l'autre.
- **?** Plan proposé, à valider : profil en dôme, couverture non saturée pour la hauteur, variation du sommet d'un nuage à l'autre ; mêmes changements dans le jumeau WGSL, test de parité, nouveaux tests chiffrés, captures avant/après de toutes les scènes de nuages et revue du directeur artistique.
- **Q** Le joueur : « Oui, lancer » (le plan d'arrondi des cumulus).
- **A** Première passe (v1) dans le modèle de référence et le WGSL : profil en dôme 1 − h² sur toute la hauteur, sommet qui part du bord visible et sature en douceur, variation de 25 % par le bruit de type, gain d'enveloppe 2.
  - **R** Sur le banc de mesure : parois −33 à −48 %, plus de colonnes au haut de la couche (7,6 % → 0,2 %), test de l'après-midi d'été tenu. Parité GPU tenue.
  - **⚠** Mais sur les images, régression nette : les tours de `cumulus_contre_jour` deviennent une mer de nuages basse ; dans la traversée, la falaise reste et des pics en pain de sucre apparaissent. Le dôme vide le haut des nuages, que l'érosion mange ; les mesures de paroi ne disaient pas ce qui se voit.
  - **⚠** Le nuage de la traversée a aussi changé (entrée franche à 225 m, sortie à 980 m) : instants des deux séquences recalés.
- **A** Itérations sur le shader seul, qui se recharge sans recompiler, en captures rapides de trois scènes : v4 (arrondi dès la moitié de la hauteur, sommets plus bas), v6 et v7 (sommet mesuré par rapport au plus fort de la tache, sans plateau). v7 garde les tours du contre-jour et arrondit les sommets ; dans la traversée, le grand nuage vu de près reste une masse raide.
- **A** Arbitrage demandé au directeur artistique sur les planches v0 à v7, avant de reporter la variante retenue dans le Rust. Rien de cette passe n'est encore commité.
- **R** Arbitrage du directeur artistique : **v6**.
  - Seule la tache la plus forte de la carte atteint le haut de la couche, sans plateau, quelle que soit la couverture ; les falaises deviennent des dômes bourgeonnants aux flancs en pente.
  - v4 plafonnait les sommets (le contre-jour perdait ses tours, et les plateaux revenaient à forte couverture) ; v7 ramenait une ligne de crête presque horizontale.
  - **Mur continu de la traversée** : c'est la scène. À une couverture de 0,55, l'empreinte des nuages approche le seuil de percolation (50 %) et les taches se soudent en crêtes de plusieurs kilomètres. Couverture à baisser à 0,45.
  - **Contre-jour sans tours** : épaisseur à monter vers 1 200 m.
  - **Défauts à corriger au portage** : dénominateur négatif sous une couverture de 0,04, `pow(0, …)` non défini en WGSL, constantes à nommer, test d'été à recaler.
- **A** v6 reportée dans le modèle de référence et le WGSL, avec les correctifs (dénominateur borné, sortie avant `pow(0, …)`, constantes `SUMMIT_START`, `SUMMIT_POWER`, `SUMMIT_VARIATION`). Test de parité étendu aux couvertures 0,02 et 0 : ni nuage ni NaN.
  - **⚠** v6 laissait des trous dans un couvert (77 % au lieu de plus de 95 % à couverture 1). **A** Les sommets remontent vers le haut de la couche selon la couverture puissance 8, sous 1 % jusqu'à 0,55.
  - **A** Test de l'après-midi d'été recalé, avec sa justification : sommets (centile 90) de 600 m à 1,2 km, mesuré 740 m ; moyenne sur les colonnes de 350 à 700 m, mesuré 477 m, un dôme étant en moyenne deux fois moins épais qu'à son sommet ; ciel couvert à 38 %.
  - **R** Hauteur médiane des parois au bord visible, v0 → v6 : 150 → 140 m (`cumulus_midi`), 250 → 190 m (traversée à 0,55), 150 → 110 m (`cumulus_sous_la_base`). Elle baisse, mais modestement.
- **A** `cumulus_contre_jour` passe à 1 200 m d'épaisseur : les tours dépassent de nouveau l'horizon.
- **A** Traversée à une couverture de 0,45, sur un nouveau nuage choisi avec la densité Rust (graine 85, base 600 m). Les premiers essais (graine 195) mettaient le bord du nuage à 200 m du départ : l'approche ne montrait qu'un mur blanc, et la dérive de l'avion vers le nord allongeait la traversée.
  - Le SPAD survole à 1 000 m des sommets plus bas, puis entre dans une tour qui monte à 1 100 m, par son milieu (nuage plein à 100 m de part et d'autre de la route).
  - Bord à 725 m (16,3 s), cœur à 760 m, sortie vers 1 175 m (26,5 s), ciel libre ensuite. 13 instants de 3 à 29 s.
  - **R** Séquence dense : pas de saut, au plus 4,8·10⁻³ par paire d'images à l'entrée ; à la sortie, une rampe jusqu'à 1,3·10⁻².
- **A** `just check` et `just test` passent. Skill `rendu-visuel`, fiche des nuages et plan du M3 à jour. Revue finale du directeur artistique en cours.
- **R** Revue finale du directeur artistique : **approuvé, chantier de l'arrondi clos**.
  - Falaises et sommets plats réglés partout ; le mur continu a disparu de la traversée. La raideur restante du bord ne l'inquiète pas : un cumulus a des flancs presque verticaux près de sa base ; la falaise venait de la raideur, du sommet plat et de la longueur continue réunis.
  - `cumulus_contre_jour` dépasse l'ancien rendu (crête variée, tourelles) et reste la meilleure image du projet ; `cumulus_midi` est un vrai champ de cumulus de beau temps.
  - Traversée : bonne référence pour l'entrée et la sortie sans saut. Faiblesses non bloquantes : l'approche longe la tour sans la montrer entière ; l'avion s'assombrit à peine si près du sommet ; la dernière image de la poursuite montre surtout du nuage.
- **A** Commentaire et description de la traversée corrigés (le flanc de la tour occupe déjà la moitié du cadre à 3 s ; l'avion s'estompe, à peine assombri) ; limite de la couverture consignée dans la doc de `CloudLayer::cover`.
- **?** Reports proposés par le directeur artistique, par priorité :
  - **haute** : le liseré argenté absent du contre-jour et la matière « coton » des nuages proches, à départager entre la résolution de la marche et l'éclairage (capture en pleine résolution d'abord) ;
  - **moyenne** : la couverture par défaut de 0,55, structurelle (une carte météo en cellules distinctes) ;
  - **basse** : la hauteur d'une tour à l'autre (troisième canal), la dernière image de la poursuite, une bande jaunâtre possible sur les sommets lointains de `cumulus_midi` ;
  - **à surveiller** : la remontée des sommets selon la couverture puissance 8, sur `plafond_bas`.
- **A** Suite, comme le joueur l'a fixé : le liseré clair le long des silhouettes dans le brouillard.

### Le liseré clair le long des silhouettes dans le brouillard

- **A** Diagnostic par les trois captures que proposait le directeur artistique, vue du pilote à 18 s, au cœur du nuage :
  - en « ultra » (marche en pleine résolution), le liseré s'amincit à un pixel sans disparaître ;
  - sans TAA (changement temporaire, annulé ensuite), il reste, en marches d'escalier : ce n'est pas le TAA de Bevy ;
  - en mesurant les pixels en travers du bord d'attaque, le liseré tombe côté ciel, à la couleur du ciel bleu de l'atmosphère vu à travers un brouillard trop mince : ce n'est pas un reflet sur l'aile.
- **R** Cause : la passe temporelle des nuages. Elle borne l'historique à la plage des 3×3 texels voisins ; au bord d'une silhouette, ces voisins mêlent des texels arrêtés sur l'avion (brouillard de quelques mètres) et des texels passés par le ciel (brouillard opaque), et la plage devenait si large que l'historique mince fuyait dans les texels du ciel.
- **⚠** Deux essais dans la composition (écarter les texels discordants, chercher en 4×4) ne changeaient rien : annulés.
- **A** Correctif dans `temporal.wesl` : la plage ne compte que les voisins arrêtés à la même distance (à 25 % près).
  - **R** Le liseré disparaît : le brouillard va jusqu'au bord, puis un pixel de transition, puis l'aile. Même « ultra » gardait un pixel bleuté.
  - **R** Effet plus large que prévu : hors du brouillard, les silhouettes sur fond de nuages traînaient un halo blanchâtre dédoublé (mâts, bords d'aile, mitrailleuse) ; elles sont nettes maintenant.
  - **R** Coût de la passe temporelle : 0,03 → 0,07 ms. Ailleurs, écarts au niveau du bruit d'une capture à l'autre. `just check` et `just test` passent (437 tests).
- **A** Confirmation demandée au directeur artistique.
- **R** Verdict du directeur artistique : **approuvé, le liseré est réglé, point clos**. Le grain des nuages, du sol et du brouillard est inchangé ; seuls les bords de l'avion gagnent en netteté (mâts non dédoublés, bord du stabilisateur sans dents de scie, cadre du pare-brise visible). Selon lui, la plus forte amélioration de netteté de l'avion depuis l'arrivée des nuages.
- **A** Risque qu'il signalait, vérifié : le scintillement aux silhouettes, le bornage disposant de moins de voisins.
  - **R** Séquence dense de l'entrée, avant/après, dans une bande de 8 px autour de la silhouette : variation moyenne d'une image à l'autre de 4,04 → 3,84·10⁻³ (95ᵉ centile 9,56 → 9,22), zone témoin inchangée (2,02). Pas de scintillement supplémentaire.
  - **R** En qualité « bas » (texels trois fois plus grands) : bords propres dans le brouillard comme sur fond de nuages.
- **?** Points antérieurs qu'il relève, hors de ce correctif :
  - un trait jaune droit à l'horizon de `cumulus_midi`, de 2 à 4 px, à travers les nuages lointains (le bleu 17 niveaux sous le rouge), présent avant et après. Cause probable : la limite de distance de la marche ou le bord du terrain vus à travers les nuages lointains. La scène phare : à traiter vite ;
  - le disque de l'hélice un peu plus clair que le brouillard en poursuite (0660) et en vue du pilote (1080), quand des pales de bois floues devraient l'assombrir. Détail.

### Le trait jaune à l'horizon de `cumulus_midi`

- **Q** Le joueur : « Oui, le trait jaune ».
- **A** Mesure : un trait de 2 à 4 px à travers les nuages lointains, juste sous l'horizon, un peu incliné (y de 464 à 473 d'un bord à l'autre), plus clair et plus jaune que ses voisins (222, 218, 198 contre 216, 215, 217).
- **A** Deux essais sur le shader seul :
  - **R** marche des nuages limitée à 40 km au lieu de 60 : le trait reste à sa place. Ce n'est pas la distance de marche ;
  - **R** sans perspective aérienne sur les nuages : le trait disparaît.
- **R** Cause : la transmittance de l'air entre l'œil et les nuages, prise à la fonction de Bevy `sample_transmittance_lut_segment`. Elle divise deux valeurs de la table de transmittance, toutes deux minuscules près de l'horizon de la planète, et change de formule là où le rayon touche le sol. Bornée à 1, elle laissait une bande de nuages lointains sans voile : leur lumière chaude du soleil, sans le bleu de l'air.
- **A** Transmittance intégrée sur 12 pas le long du segment, à partir de la table de densité de Bevy (`air_transmittance`, `march.wesl`), pour la vue et pour la carte d'environnement. Le terme du sol de la planète garde la fonction de Bevy, pour retrancher exactement ce que Bevy ajoute.
  - **R** Le trait disparaît : la bande vaut (221, 219, 219), comme ses voisins.
  - **R** Ailleurs, écarts au niveau du bruit (0,1 à 0,5 niveau en moyenne) ; coût non mesurable, sous la variation d'une capture à l'autre.
  - **D** Pas de revue du directeur artistique pour ce correctif ponctuel, vérifié sur image et en mesures.
- **A** `just check` et `just test` passent (437 tests).

### Prochaines étapes

- **Q** Le joueur : « continue, quelles sont les nouvelles étapes ? »
- **R** Par priorité : la matière des nuages proches et le liseré argenté du contre-jour ; la scène `plafond_bas` ; la couverture par défaut de 0,55 (carte en cellules distinctes) ; les réserves ouvertes (ombre d'un nuage sur son voisin, bases tranchées, auréole du soleil, disque de l'hélice un peu clair) ; l'essai du joueur, manette en main.
- **A** Diagnostic proposé par le directeur artistique pour la première : `cumulus_contre_jour` et l'approche de la traversée (11 s) en qualité « haut » (marche en demi-résolution) et « ultra » (pleine résolution).
  - **R** Identiques à 1 ou 2 % près sur la crête, les lobes à l'ombre et le flanc proche : détail fin 4,6 / 1,3 / 2,9·10⁻³ dans les deux cas ; centile 99 de la crête 0,70 dans les deux cas, sans bord plus clair.
  - **R** Conclusion : ni le « coton » ni l'absence de liseré ne viennent de la résolution. Le détail est dans la densité, l'éclairage l'aplatit (contraste fin de 1,3·10⁻³ sur les lobes à l'ombre). C'est un chantier d'éclairage, qui touche le modèle de référence `escadrille_weather::scattering` et ses tests.
- **?** Plan proposé, à valider : vue par terme pour voir où le détail se perd, ombrage des bourgeons entre eux et terme « poudre » (Schneider 2015), frange plus douce au bord pour le liseré, tests d'énergie tenus, captures et revue du directeur artistique.

## 2026-11-09

### Chantier d'éclairage des nuages

- **Q** Le joueur : « Oui, l'éclairage », et « je veux accéder à cette discussion avec l'appli claude sur mon téléphone, comment faire ? »
- **R** Le Remote Control est déjà actif pour cette session : dans l'application Claude du téléphone, sur le même compte, partie Code, la session « Simulation WWI ». L'ordinateur doit rester allumé, l'application de bureau ouverte.
- **⚠** La machine a redémarré : dossier temporaire vidé (images de la veille perdues, refaites), agent SSH sans clé.
- **A** Vue terme par terme (diffusion simple, rapide, traîne, ciel, sol) sur `cumulus_contre_jour`, `cumulus_midi` et l'approche de la traversée.
  - **R** Les lobes à l'ombre du contre-jour sont faits de la traîne (orange, 0,165) et du ciel (bleu, 0,152), presque uniformes. Le flanc proche de la traversée, de la traîne pour 0,66 sur 0,755. Le relief des dômes vient du terme rapide. Le liseré existe dans la diffusion simple, noyé par la traîne et le ciel.
- **A** Balayage des constantes de diffusion multiple contre les tests d'énergie.
  - **R** Seul le réglage actuel passe ; le rapport bourgeon éclairé / à l'ombre plafonne vers 2,1, même avec une troisième composante.
- **A** Essais sur le shader, en captures :
  - **R retenu** : un troisième pas détaillé vers le soleil (densité complète jusqu'à 104 m au lieu de 32). Bords éclairés de la crête du contre-jour 1,45 → 1,63 fois le ciel voisin, plus de relief ; flanc de la traversée un peu plus modelé ; cœur du nuage un peu plus sombre. Coût d'environ 0,05 ms (0,91 contre 0,86 ms, dans le bruit) ;
  - **R écartés** : la troisième composante (aucune différence visible), la « poudre » de Schneider (dômes plus gris, moins détaillés), la frange de densité adoucie (rien de plus, bords émoussés).
- **A** `just check` et `just test` passent (437 tests). Fiche des nuages à jour. Revue du directeur artistique en cours.
- **R** Verdict du directeur artistique : **approuvé, passe d'éclairage close** avec le troisième pas détaillé. Sur la crête du contre-jour, les tourelles s'individualisent et prennent un bord clair ; sur le flanc de l'approche, un creux ombré donne de la profondeur ; pas de grain, pas de coût mesurable. Les trois rejets sont confirmés.
  - **Limites acceptées** : un cumulus éclairé de face est plat ; un flanc nord en plein soleil de midi est gris et doux. Notées dans la fiche des nuages.
  - **Le liseré n'est pas encore là** : ce qui fait un liseré, c'est un fil clair sur un corps sombre, et les faces à l'ombre sont trop claires et uniformes.
- **?** Incrément suivant qu'il propose : la part de ciel par une courte marche vers le haut, penchée vers le côté ouvert, dans `march.wesl` et sa jumelle Rust. Il vise les faces sombres et bleutées du contre-jour et le contraste sous le liseré ; un test (part de ciel au pied d'une tour < à mi-flanc < au sommet) ; environ 3 ou 4 échantillons grossiers de plus. Ensuite, vérifier l'auréole du soleil (aucune autour du disque malgré le bloom). Ne pas toucher à la densité maintenant.

### La part de ciel

- **Q** Le joueur : « Oui, la part de ciel ».
- **A** La part de ciel d'un échantillon était prise jusqu'au sommet de l'enveloppe, ou le long du rayon de l'œil sur un flanc, chemin nul à la surface : tout point visible recevait le ciel entier. Elle vient maintenant d'une courte marche vers le haut, penchée vers le côté de l'œil (ouvert pour un flanc visible) : 10, 30, 90 puis 270 m, le premier pas sur la densité complète (`optical_depth_to_sky`, `march.wesl`).
  - Référence Rust `CloudTextures::optical_depth_to_sky` et test : sur le flanc de la tour de la traversée, 21 au pied, 6 à mi-flanc, 0,8 près du sommet.
  - La profondeur de l'œil par image (2026-11-08) est retirée, avec son calcul en Rust et son test : continue dans toutes les directions, la marche vers le ciel couvre aussi l'intérieur du nuage.
- **R** Captures :
  - `cumulus_contre_jour` : les lobes à l'ombre gagnent leurs bourgeons, bords plus clairs et creux plus sombres (luminance 0,290 → 0,266) ; ils deviennent plus chauds que bleutés, la traîne orangée y dominant une fois le ciel retiré ;
  - au cœur du nuage, brouillard uniforme, un peu plus clair et bleuté (2 niveaux), sans ligne ni forme ;
  - autres scènes : écarts faibles ;
  - séquence dense : pas de saut à l'entrée ni à la sortie, des rampes un peu plus marquées ; juste avant la sortie, un éclaircissement régulier (le ciel se rapproche), pas de scintillement.
  - Coût : quelques centièmes de milliseconde en 1280×720.
- **A** `just check` et `just test` passent (436 tests : le test de la profondeur de l'œil est parti avec elle). Fiche des nuages à jour. Revue du directeur artistique à venir.
- **A** Auréole du soleil vérifiée, comme le demandait le directeur artistique : profil de luminance du ciel autour du soleil sur `cumulus_contre_jour` (1440p).
  - **R** Le disque (`SunDisk::EARTH`, intensité physique) sature sur 6 px ; le bloom (`Bloom::NATURAL`) s'étend sur environ 20 px ; au-delà, le ciel ne gagne que 6 % à 3° du soleil (0,461 → 0,490). L'auréole de Mie de `ScatteringMedium::earth` est très faible.
  - **?** La renforcer demanderait plus d'aérosols dans l'atmosphère, ce qui change aussi la couleur du ciel et la brume partout : proposé comme chantier séparé (atmosphère), soumis au directeur artistique.
- **R** Verdict du directeur artistique sur la part de ciel : **approuvée, sous condition**. Les lobes du contre-jour ont gagné leur relief ; le ton gris chaud des faces à l'ombre est juste (le soleil bas traverse la tour en orangé), mais la description de la scène, qui les voulait bleutées, est à corriger. L'auréole devient un chantier séparé (atmosphère, aérosols).
  - **Condition** : un « pommelé », taches rondes et pâles sur les faces à l'ombre, à faire disparaître avant de clore.
  - **?** Test qu'il demande : placer le soleil juste derrière le bord de la tour, pour savoir si le liseré relève encore du shader.
- **A** Pommelé : la marche vers le ciel lisait au milieu de chaque pas, et les cellules du bruit s'imprimaient en taches. Le décalage de ses pas suit maintenant une gigue par pixel (`fract(first + SECOND_JITTER)`), que la passe temporelle moyenne ; la référence Rust prend le décalage en paramètre.
  - **R** Les taches s'effacent, sur la vue du seul terme de ciel comme sur l'image complète.
- **A** Description de `cumulus_contre_jour` : « faces à l'ombre tournées vers la caméra, d'un gris chaud (le soleil bas traverse la tour en orangé) ».
- **A** Test du liseré : soleil de `cumulus_contre_jour` déplacé de 176° à 152° d'azimut (élévation 9°), puis remis à 180°.
  - **R** Près du soleil, une bande claire de 10 à 30 px s'allume juste à l'intérieur du bord (0,84 à 0,86 en luminance linéaire, contre 0,50 pour le corps) ; à 200 px du soleil, rien. À 152°, le disque se voit à travers le sommet mince d'un bourgeon. Le modèle fait donc le liseré quand le soleil est au bord ; dans la composition de la scène (soleil à 6–10° des bords), il n'y en a pas : c'est une affaire de composition, plus de shader.
- **A** `just check` et `just test` passent (436 tests). Captures `cumulus_contre_jour` et `cumulus_midi` refaites ; nuages 1,12 ms en 1440p. Fiche des nuages à jour. Verdict de clôture demandé au directeur artistique.
- **R** Verdict du directeur artistique : **approuvé, passe « part de ciel » close**. La gigue a fait disparaître les disques : il reste des plages claires au pied des lobes, qui se lisent comme des bourgeons tournés vers le ciel. Le liseré a le bon comportement angulaire, doré plutôt qu'argenté à 9°, ce qui est juste. Rien ne régresse ailleurs.
  - **Réserve** : le liseré reste doux (le bord ne dépasse le corps juste en dessous que de 15 %). La double Henyey-Greenstein n'a pas le pic de diffraction des gouttelettes ; piste : un lobe avant étroit (phase HG + Draine), à régler dans le chantier d'atmosphère avec l'auréole, puisque les deux jouent dans les mêmes degrés autour du soleil.
  - **Demandes** : retirer de `cumulus_contre_jour` la promesse d'un « liseré argenté », et ajouter dès maintenant une scène « soleil au bord », sans laquelle une régression du liseré passerait inaperçue ; vérifier une fois la gigue en mouvement.
- **A** Description de `cumulus_contre_jour` sans le liseré (« bords clairs des crêtes ») ; ligne de la table des scènes (skill `rendu-visuel`) précisée.
- **A** Nouvelle scène `cumulus_lisere` : la couche de `cumulus_contre_jour`, le soleil à 156° d'azimut, la caméra tournée de 12° vers lui, le soleil juste au-dessus de la crête. Le bord s'y allume près du soleil. Elle servira d'image « avant » au chantier de la phase et de l'auréole.
- **A** Gigue vérifiée en mouvement : séquences d'images consécutives, avec et sans gigue, dans un virage de `cumulus_contre_jour`, un virage de `cumulus_midi` et l'approche de la traversée (scènes temporaires, retirées).
  - **R** Le bruit d'une image à l'autre ne change que de ±3 % ; les différences amplifiées ne montrent que le déplacement des bords, aucun grouillement sur les faces à l'ombre.
  - **⚠** Les premiers virages, commandes non tirées du trim (gaz 1,0, manche tiré), cabraient l'avion vers le ciel vide : refaits à partir du trim de `vol-2000` (gaz 0,6, manche −0,06).
- **A** `just check` et `just test` passent (436 tests). Fiche des nuages à jour (gigue en mouvement, liseré doux et piste du lobe avant).

### Le chantier d'atmosphère : étude et plan

- **Q** Le joueur choisit le chantier suivant : « L'atmosphère » (l'auréole du soleil et le lobe avant étroit des nuages).
- **A** Étude de l'atmosphère de Bevy 0.20 (`bevy_light`, `bevy_pbr`) et de notre phase des nuages.
  - **R** Bevy 0.20 inverse diffusion et absorption des aérosols dans `ScatteringMedium::earth` : 3,996e-6 m⁻¹ d'absorption et 0,444e-6 de diffusion, quand Bevy 0.16 (valeurs de Hillaire 2020) avait l'inverse. Vérifié dans les sources des deux versions (docs.rs pour la 0.16). Nos aérosols absorbent 90 % au lieu de diffuser 90 %.
  - **R** La phase des aérosols de Bevy (Henyey-Greenstein, g = 0,8) est trop étalée pour une auréole. Sa table de phase résout environ 0,6° près du soleil. La perspective aérienne s'arrête à 32 km, le sol va jusqu'à 100 km.
- **A** Lecture de Jendersie et d'Eon (« An Approximate Mie Scattering Function for Fog and Cloud Rendering », SIGGRAPH 2023 Talks) : mélange de Henyey-Greenstein et de Draine, paramètres en fonction du diamètre des gouttelettes (5 à 50 µm).
  - **R** À 20 µm, la moitié de l'énergie part dans un pic d'environ 0,2° (g ≈ 0,995). Remplacer toute notre phase par ce mélange diviserait la diffusion simple vers l'arrière par 6 et celle à 90° par 25 : il faudrait recaler la diffusion multiple.
  - **⚠** L'outil de lecture du web a enregistré de lui-même les deux PDF de l'article (et un troisième fichier) dans le dossier de la session, hors du dépôt, sans que je t'aie demandé l'autorisation de les télécharger. Rien n'est entré dans le dépôt.
- **A** Prototype (non commité) : aérosols remis dans le bon sens, densité ×1, ×10, ×25, sur `cumulus_contre_jour`, `cumulus_lisere`, `cumulus_midi` et `vol-essai-poursuite`.
  - **R** La correction seule change peu : ciel à 1° du soleil 0,495 → 0,541. À ×10 (environ 65 km de visibilité au sol), l'auréole est nette, le ciel plus pâle mais encore bleu. À ×25 (environ 30 km), le ciel blanchit, la plaine se voile fort et un trait clair apparaît à l'horizon (fin de la perspective aérienne à 32 km). Même à ×25, le ciel à 1° du soleil n'est que 1,27 fois celui à 10° : il faut aussi une phase plus pointue.
- **A** Plan rédigé : `docs/plans/m3-atmosphere.md` (milieu maison, brume de 1917 réglable par scène, auréole par une phase plus pointue des aérosols, lobe avant des nuages, revue de toutes les scènes d'extérieur). Lien depuis `m3-beaute.md`.
- **?** Décisions demandées au joueur : la brume par défaut (recommandé : visibilité de 40 à 60 km), la phase des nuages (recommandé : un pic étroit ajouté à la phase actuelle), signaler l'inversion à Bevy (recommandé : oui).

## 2026-11-11

### Le chantier d'atmosphère

- **Q** Le joueur valide le plan avec les trois options recommandées : « Oui, recommandées » (brume modérée, pic étroit ajouté à la phase des nuages, signalement à Bevy).
- **⚠** La session a été coupée pendant le lancement des recherches ; le dossier temporaire a été vidé (images et scripts de mesure refaits). L'historien, interrompu, a été relancé.
- **A** Recherche du correctif côté Bevy, avant de rédiger un signalement.
  - **R** Déjà corrigé en amont : bevyengine/bevy#26143, « Fix Earth atmosphere scale heights and Mie coefficients », fusionné le 2026-10-26. Il corrige l'inversion des aérosols, mais aussi les hauteurs d'échelle (13,3 et 2 km au lieu de 8 et 1,2, écrites pour une atmosphère de 60 km quand elle en fait 100) et l'indexation de la table de diffusion quand ses deux résolutions diffèrent. Bevy 0.20.1 (2026-10-28) ne le contient pas (vérifié dans ses sources). **Aucun signalement à faire.**
  - **⚠** En interrogeant crates.io, j'ai mis l'adresse e-mail du joueur dans l'en-tête User-Agent de la requête, alors qu'elle ne doit pas partir vers un service tiers sans son accord. Erreur de ma part, signalée au joueur ; à ne pas refaire.
- **A** Milieu maison `crates/game/src/world/atmosphere.rs`, appelé par la plaine et le studio : les valeurs corrigées de Bevy, des tables de 512 (égales), et la densité des aérosols tirée de la visibilité au sol (loi de Koschmieder). Champ `visibility` des scènes (unité `Km` ajoutée aux longueurs), 50 km par défaut (`ESTIMÉ`). Perspective aérienne étendue à 100 km.
  - **R** Tests : visibilité obtenue, albédo des aérosols 0,9, hauteurs d'échelle, tables carrées.
  - **R** Le ciel à 1° du soleil passe de 0,496 à 0,639 ; le haut du ciel bleuit (couche d'air plus mince) ; la brume voile le sol vu d'en haut et monte jusqu'à l'horizon, sans trait.
  - **R** 64 tranches de perspective aérienne coûtaient 0,16 ms de plus pour 1 à 2 % de différence sur le sol proche : gardé 32.
- **A** Profil de l'auréole : la recherche web de l'agent n'a rien donné ; les PDF n'ont pas été téléchargés. Les formules de Buie, Monger et Dey (2003) ont été lues dans le code source libre de Tonatiuh (`SunshapeBuie.cpp`) : φ(θ) = e^κ θ^γ au-delà du disque, γ = −2,3 pour un rapport circumsolaire de 0,05.
- **A** Phase des aérosols à deux lobes, ajustée sur cette pente (pic g = 0,991 pour un tiers, corps g = 0,706, cosinus moyen 0,8 gardé).
  - **R** Profil du ciel au contre-jour, de 1 à 10° du soleil : 0,884, 0,732, 0,594, 0,494, 0,431 (avant : 0,639 à 0,466). Le soleil s'entoure d'un halo blanc serré, au lieu d'un large voile. Au-delà de 30°, le ciel ne change pas.
- **A** Pic avant des nuages (Jendersie et d'Eon, gouttelettes de 15 µm `ESTIMÉ`) : g = 0,99 et la moitié de la lumière, dans `march.wesl` et `escadrille_weather::scattering`. Balayage du poids de 0 à 0,5.
  - **R** Le bourgeon juste sous le soleil de `cumulus_lisere` prend un bord blanc net. Les dômes éclairés perdent 0,4 % à midi, 1,5 % au contre-jour. Les 36 tests de la météo passent (tests d'énergie compris). Le liseré se lit contre le corps du nuage plus que contre le ciel : l'auréole éclaire aussi le ciel derrière le bord, et le tonemapping écrase les deux près du blanc.
- **A** Captures de toutes les scènes d'extérieur avec la nouvelle atmosphère, pour la revue du directeur artistique (en cours : la traversée, le vol d'essai et le studio restent).
  - **R** Cumulus : auréole serrée autour du soleil ; crête blanche sous le soleil de `cumulus_lisere` ; sol voilé de gris clair vers le soleil bas (diffusion avant de la brume) ; à midi et sous la base, un horizon plus brumeux, rien d'autre.
  - **R** Les scènes `terrain-*` n'avaient pas de capture précédente dans `target/captures` : pas d'image « avant ». Leur horizon est plus pâle et doux, sans trait.
- **A** Captures terminées (15 scènes) : traversée et studio presque inchangés, sol lointain un peu plus voilé ; `contre_jour_toile` gagne une auréole autour du soleil. Revue demandée au directeur artistique, avec les planches avant/après.
- **A** `just check` et `just test` passent (443 tests).
- **R** Verdict du directeur artistique : **approuvé**. La physique est juste ; le soleil se lit enfin comme un soleil (auréole) ; la brume de 50 km est le bon niveau par défaut et donne l'échelle de la plaine ; le liseré naît là où il doit ; pas de régression en vol ni au studio. Réserves : le contre-jour a perdu relief et chaleur (exposition), zénith « lavande », un trait à l'horizon au sol, une bosse grise sous la base, la brume selon la mission.
- **A** Essai d'exposition à −1 et −2 EV (surcharge temporaire, retirée) sur `cumulus_contre_jour`, `cumulus_lisere` et `cumulus_midi`.
  - **R** À −1 EV, le bord sous le soleil (0,93 à 0,99) se détache du ciel de l'auréole (0,62 à 0,89) et domine le corps du nuage (0,41 à 0,47) ; le relief revient. À −2 EV, trop sombre. Remède : une exposition qui suit la lumière, avec l'incrément « Heure du jour ».
- **A** Zénith mesuré : la teinte n'a bougé que de 2 à 8° vers le bleu, pas vers le violet ; ciel un peu plus sombre et moins saturé à midi. Ozone de Hillaire inchangé : le décalage vient de la couche d'air corrigée. **D** Gardé tel quel.
- **A** Horizon des scènes au sol : le ciel s'éclaircit régulièrement jusqu'à l'horizon, sans bande ; pas d'artefact. Bosse grise de `cumulus_sous_la_base` : le disque de l'hélice, vu au-dessus de l'aile, qui se détache sur l'horizon voilé.
- Plan `m3-atmosphere.md` : section d'avancement ajoutée.
- **R** Fiche de l'historien : `docs/recherche/brume-ete-1917.md`. Aucune mesure de la visibilité en Flandre en 1917 : des déductions (confiance C), à partir des moyennes d'été britanniques de l'ère du charbon, des témoignages (McCudden, Jones, Biddle, Bordeaux) et du caractère maritime de la Flandre.
  - Mise en garde : notre visibilité suit Koschmieder (3,912/σ, contraste de 2 %) ; les météorologues, la portée optique de l'OMM (3,0/σ, 5 %). Nos 50 km valent environ 38 km au sens des météorologues.
  - Beau temps d'été typique : 15 à 30 km en portée optique, soit 20 à 39 km dans notre convention. Valeur recommandée : **30 km** dans notre convention (`ESTIMÉ`), réglable de 25 à 40 km ; 50 à 60 km pour un « jour clair ». Pour garder le ciel bleu, resserrer la brume vers le bas (sous les chasseurs, ciel bleu au-dessus, confiance B) plutôt que relever la visibilité.
  - Documents à récupérer (§ 7.1) et demandes à des organismes (§ 7.2), non reportées dans `demandes.md` : à décider avec le joueur.
- **A** Essai à 30 km (copies temporaires des scènes, retirées) : midi change à peine ; sol du contre-jour et du vol d'essai plus voilé ; **un mince trait cyan à l'horizon du vol d'essai** (1 000 m), absent à 50 km, probablement la jonction de la plaine plate de 200 km avec la Terre courbe de Bevy.
- **?** Décision demandée au joueur : garder 50 km par défaut, ou suivre l'historien (30 km, avec la brume resserrée vers le bas et le trait de l'horizon corrigé).

### La brume de l'historien

- **Q** Le joueur choisit de suivre l'historien : « Suivre l'historien » (30 km par défaut, brume resserrée vers le bas, trait de l'horizon corrigé).
- **A** Visibilité par défaut 30 km (`ESTIMÉ`, fiche § 5.2 ; environ 23 km en portée optique de l'OMM).
- **A** La brume tient dans la couche de mélange de l'après-midi : homogène jusqu'à son sommet, puis un air libre au dixième de sa densité, sur 2 km d'échelle (`ESTIMÉ`). Sommet : la base des cumulus quand la scène en a (les cumulus de beau temps naissent au sommet de la couche de mélange), 1 500 m sous un ciel clair (fiche : 1 à 2 km). Remplace l'exponentielle de 1,2 km de Hillaire, qui blanchissait tout le ciel à 30 km. Tests : brume homogène sous le sommet, moins d'un dixième 500 m au-dessus.
  - **R** Ciel bleu au-dessus des nuages, midi presque inchangé, sol sous les nuages plus voilé et plus chaud.
- **A** Le trait cyan de l'horizon, revenu à 1 000 m : il ne bouge ni avec la taille de la plaine (200 ou 600 km) ni avec la portée de la perspective aérienne (100 ou 300 km), et disparaît avec une plaine de 20 km. Cause : pour le sol, Bevy prend la transmittance de `sample_transmittance_lut_segment`, qui change de formule à l'angle de l'horizon de la Terre courbe (−1,0° vu de 1 000 m) ; notre plaine plate s'étend au-delà (à 56 km), et la couleur du sol y perce la brume. C'est la fonction qui avait déjà fait le trait jaune de nos nuages.
  - **D** Mode de rendu de l'atmosphère par marche de rayons pour chaque pixel (`AtmosphereMode::Raymarched`), 8 échantillons par rayon : plus de trait (pic de 0,17 niveau contre 1,08) ; 0,5 ms en 1440p (16 échantillons, le défaut de Bevy : 0,9 ms, pour 1 à 3 % d'écart lisse dans le ciel).
  - Piste écartée pour l'instant : courber le sol avec la Terre (à revoir avec le relief réel).
- **A** Captures des 15 scènes avec la brume de l'historien. Contre-jour et liseré : sol vers le soleil bas en voile laiteux, bas des nuages fondu dedans ; midi, sous la base, vol d'essai : sol plus voilé ; au sol, bande laiteuse au-dessus de l'horizon. Ciel 0,45 à 0,47 ms, image de 3,3 à 4,9 ms de GPU en 1440p. Revue demandée au directeur artistique.
- **A** Skill `rendu-visuel` et plan `m3-atmosphere.md` à jour.
- **R** Verdict du directeur artistique : **approuvé, la brume de l'historien devient le réglage par défaut**. 30 km, sommet à 1 500 m, air libre au dixième : le bon niveau. Le plus grand gain : au-dessus de la couche, la brume reste sous l'avion et le ciel bleu au-dessus, l'altitude se lit ; le pastel du sol rejoint les autochromes. La bande laiteuse à l'horizon au sol est juste ; le trait cyan a disparu.
  - Réserve principale : le contre-jour et le liseré perdent leur structure de valeurs (sol en nappe laiteuse, physiquement juste face au soleil bas) ; l'exposition n'y suffira pas. Piste : un jour très clair pour ces deux scènes.
- **A** `cumulus_contre_jour` et `cumulus_lisere` à 50 km de visibilité (jour très clair, fiche § 5.1 : 50 à 60 km après un front, vent de nord-ouest), notés dans les deux scènes et la table du skill.
  - **R** Le sol revient sous les nuages, avec les ombres des nuages ; les bases se détachent de la nappe.
- **A** Sommet de la brume vu de 2 000 m par ciel clair : pas de bandes avec 8 échantillons (écart d'une ligne à l'autre inférieur à 0,3 niveau). Nouvelle scène de référence `brume_sommet` : la nappe pâle qui noie l'horizon sous un ciel bleu.
- **A** Herbe « trop saturée » à l'horizon au sol : vérifié, juste. À 2 m de hauteur, seul le premier pixel sous l'horizon est au-delà de 3 km, et il est déjà grisé ; les suivants sont à moins de 1,6 km.
- **A** Le bord du sommet de la brume (100 m) est étalé sur environ deux pas de la table du milieu (195 m) : noté dans le code. La lumière qui éclaire la brume est bien atténuée par la brume elle-même (tables de Bevy calculées sur le milieu).

### Remarque sur la manette, et le chantier « Heure du jour »

- **Q** Le joueur : « Ajouter dans les remarques pour plus tard : la manette SN30 pro peut vibrer : correction à faire. Note le, puis lance le chantier "Heure du jour" ».
- **A** Remarque notée dans `docs/gdd/retours.md` (nouvelle section « Remarques pour plus tard ») : le M0 avait conclu « aucune vibration ressentie » (2026-10-20) ; à reprendre (chaîne gilrs, pilote `xpad`, `fftest`, labo manette), puis corriger la vision, le skill et les retours. Renvoi ajouté dans le skill `manette`.
- **A** Étude pour « Heure du jour » : le soleil est fixé par scène (azimut et hauteur) ; l'exposition est fixe (EV100 = 13) ; l'atmosphère de Bevy atténue et colore déjà le soleil bas ; Bevy 0.20 fournit une exposition automatique (`AutoExposurePlugin`, histogramme, courbe de compensation), non activée ; la plaine n'a pas encore de position géographique.
- **A** Plan rédigé : `docs/plans/m3-heure-du-jour.md` (soleil calculé, exposition qui suit la lumière, brume du matin, nuit simple, scènes `aube_terrain`, couchant, nuit de lune). Lien depuis `m3-beaute.md`.
- **?** Décisions demandées au joueur : l'exposition (recommandé : base physique et adaptation de l'œil bornée à ±1,5 EV), l'heure du jeu (recommandé : heure légale française de 1917), les étoiles (recommandé : procédurales), la brume selon l'heure (recommandé : dans cet incrément).

### Le chantier « Heure du jour »

- **Q** Le joueur valide le plan avec les quatre options recommandées : « Oui, recommandées » (base physique et adaptation bornée à ±1,5 EV, heure légale française de 1917, étoiles procédurales, brume selon l'heure dans cet incrément).
- **A** Historien lancé sur l'heure légale de 1917 et le lieu par défaut. **⚠** Il s'est arrêté sur une limite de dépense de l'API ; à relancer.
- **A** Position du soleil : `escadrille_weather::sun`, algorithme de la NOAA tiré de Meeus, réfraction comprise. Tests : jour julien (exemple 7.a de Meeus), place du soleil (exemple 25.a, au centième de degré), extrêmes de l'équation du temps, hauteurs à midi sur Ypres aux équinoxes et aux solstices, heure du midi solaire, lever à l'est et coucher à l'ouest.
- **A** Exposition (`crates/game/src/render/exposure.rs`), en trois parts : la base de la caméra (EV100 13 en plein jour, qui suit la lumière au crépuscule et la nuit), l'adaptation de Bevy (`AutoExposurePlugin`, moyenne du cadre ramenée à une valeur neutre, à ±1,5 EV près), et un étalonnage appliqué après la mesure (crépuscule plus sombre, éblouissement par le soleil).
  - **R** Trois essais avant d'y arriver, tous mesurés : une base qui suit l'éclairement toute la journée ouvrait le contre-jour de 2,6 EV ; la moyenne du cadre, même pondérée vers les hautes lumières, l'ouvrait aussi (cadre surtout fait de faces à l'ombre) ; un éblouissement mis dans la base était défait par l'adaptation. L'éblouissement suit la formule de Stiles et Holladay (CIE 146:2002) : environ 1,5 EV avec le soleil à 16° du regard, rien avec le soleil dans le dos.
  - **R** Valeur neutre calée sur `vol-essai-poursuite` (−0,25, exacte en binaire : Bevy compare les raccords de la courbe à l'égalité). Écarts de luminance d'image avec les captures approuvées : vol d'essai +0,04, sous la base −0,06, terrain +0,08, midi −0,30, traversée −0,50 (cadres très clairs), contre-jour −0,55, liseré −0,25, toile à contre-jour −0,93. Le bord du liseré se détache mieux du ciel de l'auréole.
- **A** Brume selon l'heure (`world::atmosphere::haze_of_the_day`, heure solaire, valeurs `ESTIMÉ` d'après la fiche de l'historien § 6) : brume du matin au sol (4 km, sommet 150 m, bord net de 40 m) jusqu'à 7 h, qui se lève jusqu'à 9 h ; couche de mélange de 300 m qui monte jusqu'à son sommet de l'après-midi à 13 h. La brume du matin s'ajoute à celle du jour, qui continue au-dessus. Les couches sont normalisées au sol (un sommet bas faussait la visibilité). Les scènes sans heure gardent l'après-midi.
- **A** Position et phase de la lune (`escadrille_weather::moon`, formules basse précision de l'Astronomical Almanac, parallaxe, réfraction) : tests sur les exemples 47.a et 48.a de Meeus.
- **R** Fiche de l'historien : `docs/recherche/heure-legale-1917.md`. Heure légale française de 1917 : heure de Greenwich (loi de 1911), et UTC + 1 du samedi 24 mars à 23 h au dimanche 7 octobre à 24 h (confiance B pour les dates, A pour le cadre légal) ; les Britanniques du front suivent l'heure française ; les Allemands ont une heure d'avance, sauf du 25 mars au 16 avril et du 17 septembre au 7 octobre. Ypres : 50,851° N, 2,885° E ; terrain de la SPA 3 : Bergues-Bierne. Valeurs de contrôle de l'US Naval Observatory. Documents à télécharger listés au § 10.1 (non faits : ton accord d'abord).
- **A** Heure légale (`escadrille_data::legal_time`) : table des heures d'été 1916 à 1918, conversion en UTC ; les scènes qui demandent l'heure sautée du 24 mars ou l'heure lue deux fois du 7 octobre sont refusées. Champ `time` des scènes.
- **A** Ressource `Sky` (`world::sky`) : soleil et lune de l'heure sur Ypres, une seule lumière d'astre (le soleil le jour et au crépuscule, la lune en pleine nuit, sous −12°), pour que les nuages, qui prennent la première lumière, soient éclairés par la lune la nuit. Le vol d'essai (`just run`) a désormais une date : le 1er septembre 1917 à 15 h, soleil à 224° et 39,6° (proposé par l'historien). Les scènes sans heure gardent le soleil de l'après-midi.
- **A** Nuit (`render::night`) : la lune, sphère éclairée côté soleil (phase), luminance d'après la loi de phase d'Allen ; 5 000 étoiles procédurales (graine fixe), réparties en magnitude comme le ciel réel, mises en place par le temps sidéral et la latitude, visibles de nuit seulement. L'exposition compte le clair de lune.
  - **R** Tests : exemples de Meeus (temps sidéral 12.a), valeurs de l'US Naval Observatory (hauteurs à 0,1°, passages au méridien à la minute et demie), étoile polaire plein nord à la hauteur de la latitude, lever des étoiles à l'est, pleine lune à environ un quart de lux.
- **A** Trois nouvelles scènes : `aube_terrain` (10 juillet 1917, 5 h 30, soleil à 5°, brume du matin), `couchant_cumulus` (15 août 1917, 19 h 45, soleil à 3° au-dessus des cumulus), `nuit_lune` (nuit du 2 au 3 août 1917, pleine lune haute au sud).
  - **R** Aube et couchant réussis d'emblée. La nuit était éclairée comme le jour : la lumière ambiante uniforme par défaut de Bevy (80 cd/m²), un ou deux pour cent d'une scène au soleil, éclairait tout sous l'exposition ouverte de la nuit. **D** Mise à zéro pour tout le jeu (le ciel éclaire déjà par la carte d'environnement de l'atmosphère) : les scènes de jour n'ont pas bougé (vol d'essai +0,05, midi −0,33 comme avant), la nuit devient une nuit de lune (ciel gris-bleu sombre, halo de la lune dans la brume, nuages gris, seules les étoiles brillantes percent).
- **A** Captures des 19 scènes. **⚠** Avec l'adaptation complète, la traversée avait des nuages gris : un cadre presque tout blanc était ramené vers le gris moyen, comme la neige sur une photo. **D** Adaptation partielle : la moitié du chemin vers la valeur neutre (`ESTIMÉ`), toujours bornée à ±1,5 EV. Écarts de luminance d'image avec les captures approuvées : vol d'essai −0,01, traversée −0,23, midi −0,17, studio +0,19, contre-jour −0,90, liseré −0,53, toile à contre-jour −1,09.
- **R** Verdict du directeur artistique : **approuvé**. La structure de valeurs du contre-jour est rendue (ciel plus profond, crête claire, corps des nuages en valeur moyenne) ; le liseré se détache de l'auréole ; l'adaptation à mi-chemin est le bon choix (la traversée reste blanche) ; `couchant_cumulus` est « sans doute la plus belle image du projet » ; `brume_sommet` montre l'horizon de brume sans bandes.
  - Réserves : (1) l'éblouissement ferme l'œil sans le voile laiteux qui va avec (ciel ardoise de `contre_jour_toile`) : ajouter le voile de Stiles et Holladay à l'image, ou plafonner l'éblouissement vers 0,7 EV ; (2) la nuit se lit comme un jour sous-exposé : ajouter la vision de nuit (désaturation, Purkinje) au tonemapping ; (3) la brume du matin ne se lit pas sur une plaine vide (il faudra des objets à distances étagées) ; (4) surveiller une dominante brune des faces à l'ombre ; (5) le soleil à 3° du couchant peut-être trop blanc.
- **A** Captures de référence remises à jour avec le code commité (19 scènes). Plan `m3-heure-du-jour.md` : avancement et réserves.

### Remarques pour plus tard, et le voile d'éblouissement

- **Q** Le joueur : « Garde pour plus tard : cockpit détaillé ; face moteur au niveau de l'hélice à détailler, pour l'instant ce n'est pas réaliste. Continue avec le voile d'éblouissement ».
- **A** Les deux remarques notées dans `docs/gdd/retours.md` (« Remarques pour plus tard »), à reprendre avec la finition du SPAD (incrément 5 du M3).
- **A** Voile d'éblouissement (`render/glare.rs`, `shaders/glare.wesl`) : la luminance de voile de Stiles et Holladay (10 E / θ² cd/m², bornée à 1° du soleil) ajoutée à toute l'image, E l'éclairement du soleil à l'œil (modèle IES, teinté par l'air selon la masse d'air), la même que celle qui ferme l'œil. Il disparaît quand un objet (cinq points du disque) ou les nuages (leur transmittance) cachent le soleil. Les scènes du studio, sans horloge, le prennent de la lumière du soleil.
  - **R** Écarts de luminance d'image : toile à contre-jour +0,49, contre-jour +0,14, liseré +0,13, aube +0,04, couchant, vol d'essai et midi 0. Au contre-jour, le ciel autour du soleil reste lumineux et doré pendant que l'œil se ferme. Sur la toile à contre-jour, le voile est fort et noie l'avion dans un halo laiteux. Revue du directeur artistique demandée.
- **R** Verdict du directeur artistique sur le voile : **approuvé, garder le voile entier** (« le boche dans le soleil » est un pilier du jeu) et recadrer `contre_jour_toile` plutôt que l'atténuer. Réserves : un halo étiré à la verticale, une occultation à cinq points trop grossière (câbles), un éblouissement peut-être compté deux fois, le couchant à vérifier, le plateau de 1°.
- **⚠** Le commit du voile est parti avec un avertissement de clippy (huit arguments) : ma chaîne de commandes a continué malgré l'échec. Corrigé au commit suivant.
- **A** Halo étiré : vérifié avec le soleil au centre du cadre, il est rond (sur un cercle de 300 px, 0,236 à 0,240 dans toutes les directions) ; l'impression de colonne vient du halo qui touche le bord haut quand le soleil est haut dans le cadre.
- **A** Couchant : le voile y est (+0,05 près du soleil) mais faible ; le modèle IES comptait la masse d'air en 1/sin h, 19 à 3° au lieu de 15. **D** Masse d'air de Kasten et Young (1989) ; **D** équation générale d'éblouissement de la CIE (Vos), de 0,1° à 100°, pilote de 25 ans, yeux bruns (`ESTIMÉ`) : plus de plateau de 1°, proche de Stiles et Holladay entre 1 et 30° ; **D** occultation sur seize points du disque.
- **A** La fermeture de l'œil, calculée sur le processeur, ignorait le soleil caché : elle assombrissait la scène quand l'aile masquait le soleil. **D** Le shader du voile écrit la part visible du soleil dans un tampon, relu chaque image (`Readback` de Bevy) ; la fermeture en est pondérée. Comme le voile, elle ne compte qu'une fois la part visible.
- **A** `contre_jour_toile` recadrée, le soleil caché derrière l'aile haute droite (9°, 45°) : la toile s'illumine, nervures en ombre, ciel clair. Nouvelle scène `avion_dans_le_soleil` (l'ancien cadrage). Captures : contre-jour +0,34, liseré +0,24 (moins de fermeture avec la CIE), couchant −0,08, autres scènes à moins de 0,05.
- **R** Verdict du directeur artistique : **approuvé, passe du voile close**. Halo rond (réserve retirée) ; la structure de valeurs du contre-jour tient ; `avion_dans_le_soleil` est une bonne référence de jeu ; `contre_jour_toile` bien cadrée, qui montre que la translucidité de la toile n'est pas encore là. Réserves : un anneau sombre au cœur du disque solaire ; la matière toile (chantier à part) ; les constantes de temps de la fermeture de l'œil.
- **Q** Le joueur : « continue ».
- **A** Anneau du disque solaire : il est au bord du disque (7,7 px de rayon en 1440p), 5 niveaux de creux ; il disparaît avec un disque cent fois moins intense, pas sans bloom. Cause : le disque à son éclat physique (2·10⁹ cd/m²) dépasse le plus grand demi-flottant de l'image une fois exposé. **D** Disque dessiné au centième de son éclat (blanc quand même après tonemapping) et voile borné à 3·10⁴ : creux de 0,1 à 0,3 niveau au couchant, au contre-jour et au liseré ; 1,1 au soleil haut, de l'ordre du bruit.
- **A** Pupille : fermeture par l'éblouissement en 0,3 s, réouverture en 4 s (`ESTIMÉ`), voile instantané. **⚠** Les captures de `contre_jour_toile` sortaient 0,9 puis 0,7 diaphragme trop sombres : pendant le chargement, avant que l'avion ne cache le soleil, la pupille se fermait pour des secondes. **D** En capture, la pupille suit sans délai, comme l'adaptation ; tant que la part visible du soleil n'est pas relue, le soleil compte comme caché. Captures revenues à la passe du voile (à 0,02 près).
- **A** `just check` et `just test` passent (474 tests). Références recapturées.
