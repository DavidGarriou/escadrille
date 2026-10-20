---
name: recherche-historique
description: Recherche et exactitude historique sur la guerre aérienne 1914-1918 — hiérarchie des sources, niveaux de confiance, fiches avion sourcées dans docs/recherche, témoignages de pilotes d'époque en plusieurs langues et réputation des avions, chronologie (quel avion, quelle unité, quel secteur, quelle date), livrées, grades, décorations, personnages réels, droits d'auteur des sources. À charger pour toute donnée chiffrée sur un avion, un moteur ou une arme, et pour tout contenu historique (missions, escadrilles, as, textes du carnet).
---

# Recherche historique

## Règle d'or

**Aucun chiffre sans origine.** Une valeur est sourcée (`S:`), calculée (`C:`) ou `ESTIMÉ` (`E:` avec justification). Les sources se contredisent souvent : noter la fourchette, choisir, justifier.

## Hiérarchie des sources

1. **Primaires** : rapports d'essais d'époque (essais officiels britanniques, français et allemands), notices de réglage et de pilotage, rapports de combat, archives d'unités ; rapports NACA (NASA Technical Reports Server, domaine public) pour les profils et l'aérodynamique.
2. **Secondaires reconnues** : ouvrages de référence (J. M. Bruce pour les avions britanniques, Gray et Thetford pour les avions allemands, les *Windsock Datafiles*), revues spécialisées (*Cross & Cockade International*, *Over the Front*).
3. **Modernes** : essais et témoignages de pilotes de répliques volantes (The Vintage Aviator, Shuttleworth Collection, Old Rhinebeck), analyses aérodynamiques publiées.
4. **Forums et wikis** : pistes uniquement, jamais seule source d'un chiffre.

## Témoignages de pilotes et réputation des avions

Le comportement d'un avion ne se résume pas à ses chiffres : il faut savoir comment il se pilotait et comment on le jugeait. Pour chaque avion, chercher **systématiquement** des témoignages de pilotes d'époque, **dans plusieurs langues** :

| Langue | Où chercher (pistes, à vérifier) |
|---|---|
| Français | carnets de vol et lettres de pilotes, rapports d'escadrille et comptes rendus de combat (Service historique de la Défense, via FranceArchives), mémoires, presse aéronautique d'époque (*La Guerre aérienne illustrée*, *L'Aérophile*), témoignages sonores du Musée de l'Air et de l'Espace |
| Anglais | combat reports et rapports d'essais du RFC/RNAS/RAF (National Archives), mémoires (Cecil Lewis, *Sagittarius Rising* ; James McCudden, *Five Years in the Royal Flying Corps*), pilotes américains (Eddie Rickenbacker, *Fighting the Flying Circus*) |
| Allemand | rapports de Jasta et évaluations de l'Idflieg, mémoires (Manfred von Richthofen, *Der rote Kampfflieger* ; Ernst Udet, *Mein Fliegerleben*), rapports d'essais d'avions alliés capturés |
| Italien | lettres et mémoires des pilotes de la Squadriglia 91 et d'autres unités équipées de SPAD, Nieuport, Hanriot |
| Autres | belges (Willy Coppens, *Jours envolés*), russes (Service aérien impérial, Nieuport et SPAD), austro-hongrois |

Ces titres sont des **pistes** : vérifier l'édition, la date et la page avant de les citer.

Méthode :
- **Situer chaque témoignage** : qui (novice ou as, quel camp), quand (sur le moment ou dans des mémoires écrits des années plus tard), sur quelle sous-version exacte, en comparaison de quel avion précédent. Noter les biais possibles : propagande, nostalgie, vantardise, traduction.
- **Croiser** : un trait de comportement n'est retenu que s'il apparaît dans au moins deux témoignages indépendants, idéalement de camps différents. Les **essais d'avions capturés** par l'adversaire sont précieux : regard technique, sans affection pour la machine.
- **Paraphraser en français**, en indiquant la langue d'origine. Une citation originale de quelques mots est permise quand la nuance compte, avec sa référence complète.
- **Distinguer la réputation des faits** : ce que les pilotes disaient d'un avion (surnoms, rumeurs, peurs, comme une aile réputée fragile) est noté séparément de ce qui est confirmé. La réputation est un matériau précieux même quand elle est fausse : dialogues au mess, carnet de vol, choix du joueur.
- **Traduire chaque trait confirmé en attente vérifiable** dans `essais.ron` (scénario qualitatif, skill `simulation-vol`). Exemple : « tend à engager la vrille en virage serré à droite » devient un essai scénarisé.

## Fonds et portails à consulter

| Fonds | Ce qu'on y trouve | Domaine à autoriser dans l'environnement cloud |
|---|---|---|
| **FranceArchives** (portail national des archives de France) | inventaires et documents numérisés des services d'archives français, dont ceux du Service historique de la Défense (journaux de marche et opérations des escadrilles, dossiers de personnel, rapports techniques) | `francearchives.gouv.fr` |
| **Musée de l'Air et de l'Espace** (Le Bourget), collections numériques | photographies, plaques de verre et tirages anciens (numérisés avec l'ECPAD), films, témoignages sonores ; e-médiathèque signalée en 2020 sous `mediatheque.museeairespace.fr` (adresse à confirmer) | `museeairespace.fr`, `*.museeairespace.fr` |
| **archive.org** | revues d'époque (*Flight*, *Aerial Age*, *The Aeroplane*), rapports techniques britanniques (ACA), bibliographies | `archive.org` **et** `*.archive.org` (le joker ne couvre pas le domaine nu, qui porte la recherche et les métadonnées) |
| **NASA Technical Reports Server** | rapports NACA (profils, hélices, moteurs), domaine public | `ntrs.nasa.gov` |

Rappels : vérifier les **conditions de réutilisation** de chaque document avant de l'intégrer au jeu (photos et films du Musée de l'Air et de l'ECPAD notamment) ; une consultation pour référence est toujours permise. L'outil web du subagent `historien` ne passe pas par le réseau du conteneur : c'est la session principale qui télécharge les documents avec curl (dossier de travail de la session) et les lui confie.

## Niveaux de confiance

**A** mesure primaire · **B** sources secondaires concordantes · **C** déduit d'avions comparables · **D** estimation de gameplay. Repris dans `essais.ron` (skill `simulation-vol`).

## Fiches

- Une fiche par avion, moteur, arme : `docs/recherche/<id>.md`, d'après le modèle `fiche-avion.md` de ce dossier.
- Sources numérotées (S1, S2…) avec référence complète (auteur, titre, année, page ou URL consultée et date).
- Journal des décisions en fin de fiche : quelle valeur a été retenue parmi les sources, et pourquoi.

## Chronologie et anachronismes

Vérifier pour chaque contenu : dates d'entrée et de sortie de service, **par secteur et par unité** ; armement et moteur selon la sous-version ; grades, décorations (Légion d'honneur, Croix de guerre, Military Cross, Pour le Mérite…) et règles d'attribution ; uniformes ; insignes et cocardes selon la date ; aérodromes occupés à la date de la mission.

## Personnages réels

- Les as historiques sont des personnes réelles : les représenter avec respect, d'après des faits documentés, sans inventer de traits dégradants ni de propos.
- Pertes, blessures et mort sont traitées avec sobriété (ton de *Wings* : le coût humain est réel).
- Distinguer clairement les personnages historiques des personnages fictifs de l'escadrille du joueur.

## Droits d'auteur

- Les **faits** (dimensions, performances, dates) sont libres ; les **textes, plans, profils couleur et photos** ne le sont pas forcément.
- Paraphraser, ne jamais recopier ; redessiner les livrées à partir de l'analyse de plusieurs sources.
- Une photo consultée comme **référence** n'a pas besoin d'être libre ; une image **incluse dans le jeu** (écran de chargement, carnet) doit avoir une licence compatible vérifiée et être inscrite dans `assets/LICENCES.md`. Attention : beaucoup de numérisations d'archives ont des conditions non commerciales, même pour des photos anciennes.
- Cartes de tranchées numérisées : vérifier la licence de chaque collection avant de les intégrer.

## Outils

Recherche web autorisée pour trouver et vérifier les sources ; citer l'URL et la date de consultation. Le subagent `historien` peut faire la recherche et relire un contenu.
