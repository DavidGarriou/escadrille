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
