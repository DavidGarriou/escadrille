---
name: nouvel-avion
description: Workflow complet pour ajouter un avion au jeu — recherche sourcée, fichier de données, essais en vol, modèle provisoire, revues. Lancé par le joueur avec /nouvel-avion <id ou nom>.
argument-hint: "<id ou nom de l'avion>"
disable-model-invocation: true
---

# Ajouter un avion : $ARGUMENTS

Charger d'abord les skills `recherche-historique` et `simulation-vol`.

## Étapes

1. **Cadrage** : confirmer avec le joueur la sous-version exacte (moteur, armement, période) et l'identifiant (`<id>`, minuscules, sans espace).
2. **Recherche** : confier au subagent `historien` la création de `docs/recherche/<id>.md` d'après `.claude/skills/recherche-historique/fiche-avion.md`. Exiger les sources, les fourchettes et les niveaux de confiance.
3. **Profils et hélice** : retrouver les polaires du profil (rapports NACA ou d'époque) et les données d'hélice ; sinon, les calculer (`fdm_lab`, BEMT) et marquer `C:`.
4. **Données** : écrire `assets/aircraft/<id>/aircraft.ron` (skill `simulation-vol`, `format-avion.md`) ; chaque valeur avec son origine.
5. **Cibles d'essais** : écrire `assets/aircraft/<id>/essais.ron` à partir des sections « Performances » et « Qualités de vol » de la fiche.
6. **Inertie et gains** : `fdm_lab inertia <id>` et `fdm_lab gains <id>` (gains de l'instructeur virtuel et de l'IA).
7. **Essais** : `/essai-en-vol <id>`. Corriger d'abord la physique et les données, jamais en tordant un coefficient sans justification écrite.
8. **Modèle 3D** : si aucun modèle n'existe, générer un modèle provisoire aux bonnes dimensions (boîtes et plans nommés selon le contrat de la skill `pipeline-assets`) ; `asset_check` doit passer.
9. **Revues** : subagent `aerodynamicien` sur les données et les essais ; subagent `historien` sur la fiche et la chronologie.
10. **Rapport au joueur** :
    - ce qui est sourcé (A/B), calculé, estimé (C/D) ;
    - résultats d'essais et écarts ;
    - comportements caractéristiques attendus en vol, à vérifier manette en main ;
    - ce qui manque (modèle définitif, livrées, sons).

Ne pas déclarer l'avion « validé » avant l'essai du joueur.
