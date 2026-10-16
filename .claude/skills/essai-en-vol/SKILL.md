---
name: essai-en-vol
description: Lance la campagne d'essais en vol sans rendu (fdm_lab) d'un avion ou de tous, compare aux cibles historiques et au dernier rapport validé, lit les tracés et rédige un rapport. À utiliser après toute modification de crates/fdm, d'un aircraft.ron ou d'un essais.ron, ou quand le joueur tape /essai-en-vol.
argument-hint: "[id-avion | all]"
allowed-tools: Bash(just essai:*), Bash(cargo run -p fdm_lab:*), Read, Glob, Grep
---

# Essais en vol : $ARGUMENTS

Si aucun avion n'est indiqué : essayer tous les avions dont `aircraft.ron`, `essais.ron` ou le code de `crates/fdm` ont changé depuis leur dernier rapport validé.

## Déroulé

1. `just essai <id>` pour chaque avion. Sorties attendues dans `target/essais/<id>/` : `rapport.json`, tracés PNG (polaire des vitesses, montée, trim en fonction de la vitesse, modes propres, scénarios).
2. **Lire les tracés** (fichiers PNG) : une courbe qui « passe » mais dont la forme est absurde est un échec.
3. Comparer :
   - aux cibles de `essais.ron` (tolérances et niveaux de confiance) ;
   - au dernier rapport validé `docs/essais/<id>.md` (régressions).
4. Vérifier la convergence en pas de temps si le code du FDM a changé (`--dt-check`).

## Rapport (dans la réponse, et dans `docs/essais/<id>.md` une fois accepté par le joueur)

```
## <id> — <date> — commit <hash>
Verdict : VALIDÉ / ÉCARTS / ÉCHEC

| Essai | Cible | Mesuré | Écart | Tolérance | Confiance | État |
|---|---|---|---|---|---|---|

Modes propres : période, amortissement, comparaison qualitative avec la fiche
Scénarios qualitatifs : réussis / échoués (avec explication)
Robustesse : réussis / échoués
Régressions depuis le dernier rapport : …
Hypothèses sur la cause des écarts, et pistes (physique ou données, avec sources)
```

Ne jamais proposer d'ajuster un coefficient sans source ni raisonnement physique. Si un écart est dû à une source douteuse, le dire et proposer une vérification par le subagent `historien`.
