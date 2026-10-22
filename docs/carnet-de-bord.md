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
