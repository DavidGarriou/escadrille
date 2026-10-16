---
name: adr
description: Rédige un enregistrement de décision d'architecture (ADR) dans docs/adr et met à jour l'index. À utiliser pour toute décision structurante (choix de crate, version de Bevy, convention, architecture d'un système, abandon d'une option), ou quand le joueur tape /adr <titre>.
argument-hint: "<titre de la décision>"
---

# Nouvel ADR : $ARGUMENTS

1. Lire `docs/adr/README.md` ; prendre le numéro suivant (quatre chiffres).
2. Créer `docs/adr/NNNN-titre-court.md` (minuscules, tirets, sans accents) avec le modèle ci-dessous.
3. Présenter au moins deux options réelles, dont « ne rien changer » si c'est pertinent. Pour une crate : version, licence, maintenance (dernière publication), compatibilité avec la version de Bevy épinglée, dépendance ou non à Bevy.
4. Statut « Proposé » tant que le joueur n'a pas validé ; « Accepté » ensuite. Si l'ADR en remplace un autre, mettre l'ancien en « Remplacé par NNNN ».
5. Ajouter la ligne dans l'index de `docs/adr/README.md`. Si la décision change une règle de `CLAUDE.md`, proposer la modification au joueur.

## Modèle

```markdown
# NNNN — Titre

- Statut : Proposé
- Date : AAAA-MM-JJ

## Contexte
Le problème, les contraintes (piliers, open source, Rust, performances), ce qui force à décider maintenant.

## Options
1. **Option A** : description, avantages, inconvénients, coût.
2. **Option B** : …

## Décision
L'option retenue, en une ou deux phrases, et la raison principale.

## Conséquences
Ce qui devient plus facile, plus difficile, ce qu'il faudra surveiller, les tâches qui en découlent.
```
