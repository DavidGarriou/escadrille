---
name: recherche-historique
description: Recherche et exactitude historique sur la guerre aérienne 1914-1918 — hiérarchie des sources, niveaux de confiance, fiches avion sourcées dans docs/recherche, chronologie (quel avion, quelle unité, quel secteur, quelle date), livrées, grades, décorations, personnages réels, droits d'auteur des sources. À charger pour toute donnée chiffrée sur un avion, un moteur ou une arme, et pour tout contenu historique (missions, escadrilles, as, textes du carnet).
---

# Recherche historique

## Règle d'or

**Aucun chiffre sans origine.** Une valeur est sourcée (`S:`), calculée (`C:`) ou `ESTIMÉ` (`E:` avec justification). Les sources se contredisent souvent : noter la fourchette, choisir, justifier.

## Hiérarchie des sources

1. **Primaires** : rapports d'essais d'époque (essais officiels britanniques, français et allemands), notices de réglage et de pilotage, rapports de combat, archives d'unités ; rapports NACA (NASA Technical Reports Server, domaine public) pour les profils et l'aérodynamique.
2. **Secondaires reconnues** : ouvrages de référence (J. M. Bruce pour les avions britanniques, Gray et Thetford pour les avions allemands, les *Windsock Datafiles*), revues spécialisées (*Cross & Cockade International*, *Over the Front*).
3. **Modernes** : essais et témoignages de pilotes de répliques volantes (The Vintage Aviator, Shuttleworth Collection, Old Rhinebeck), analyses aérodynamiques publiées.
4. **Forums et wikis** : pistes uniquement, jamais seule source d'un chiffre.

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
