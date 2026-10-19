---
name: simulation-vol
description: Modèle de vol des avions de 1914-1918 et des aérostats (crate escadrille_fdm) — corps rigide 6DDL, aérodynamique par bandes pour monoplans, biplans et triplans, hélice, moteurs rotatifs et en ligne, atmosphère, train d'atterrissage, structure, dirigeables et ballons (flottabilité, lest, altitude de pression), validation par essais. À charger pour toute tâche qui touche crates/fdm, tools/fdm_lab, un aircraft.ron, un airship.ron ou un essais.ron, ou pour toute question sur le comportement en vol d'un avion ou d'un dirigeable.
---

# Simulation de vol

Le FDM est le cœur du jeu : il sert au joueur, à l'IA, aux assistances et aux essais. Il doit être **physique, sourcé, déterministe et testé**.

Références de ce dossier, à lire selon le besoin :
- `modele-aero.md` : repères, équations, formules. À lire avant de coder une force.
- `format-avion.md` : schéma des fichiers `aircraft.ron` et `essais.ron`.

## Principes

1. **Pas de Bevy dans `crates/fdm`.** API pure, par exemple `Aircraft::step(&mut self, controls: &Controls, env: &dyn Environment, dt: f64)`. L'environnement (hauteur du sol, vent, turbulence, densité de nuage) est fourni par un trait, implémenté par le jeu et par les outils.
2. **f64 et SI partout**, angles en radians. Les conversions d'unités d'époque se font au chargement des données (`crates/data`).
3. **Déterminisme** : pas d'horloge, pas d'aléatoire sans graine, pas d'itération sur une `HashMap`.
4. **Modèle par composants** : chaque surface, l'hélice, le moteur, le train et les traînées parasites renvoient une force et un moment dans le repère corps ; le corps rigide les somme. Un composant endommagé modifie ses propres coefficients.
5. **Les comportements émergent** (couple, effet P, précession gyroscopique, vrille, décrochage asymétrique) ; on ne les scripte pas. Seules exceptions : les facteurs d'échelle des préréglages de réalisme, documentés comme non physiques (skill `realisme`).
6. **Chaque coefficient a une origine** dans le fichier de données : source (`S:`), calcul (`C:`) ou estimation (`E:`, avec justification).

## Ordre de construction recommandé

1. Atmosphère ISA et vent constant.
2. Corps rigide 6DDL. Tests : chute libre, conservation de l'énergie sans aéro, précession d'une toupie.
3. Surfaces portantes par bandes avec polaires sur 360° ; empennage avec déflexion ; traînées parasites.
4. Hélice (tables C_T/C_P en fonction de J) et moteur en ligne simple.
5. Solveur de trim, `fdm_lab`, premiers essais.
6. Train d'atterrissage, sol, roulage, décollage, atterrissage.
7. Structure (facteur de charge, pression dynamique) et dégâts.
8. Moteur rotatif (masse tournante, précession, coupe-contact, mélange).
9. Turbulence, rafales, effet de sol, sillage des autres avions.

## Particularités des avions de 14-18

- **Traînée parasite énorme** : mâts, haubans, câbles, radiateurs, train fixe, tête du pilote. Les modéliser un par un, avec leur position (ils créent aussi des moments).
- **Biplans et triplans** : interférence entre plans (écartement, décalage horizontal, décalage angulaire), déflexion d'un plan sur l'autre, ailerons parfois sur un seul plan.
- **Profils minces très cambrés** (série RAF) ou épais (Göttingen sur les Fokker) : décrochages souvent brutaux. Chercher des données de soufflerie (NACA, rapports britanniques) plutôt qu'inventer.
- **Gauchissement** des ailes (Fokker Eindecker) à la place d'ailerons.
- **Moteurs rotatifs** (Gnome, Le Rhône, Clerget, Oberursel) : carter et cylindres tournent avec l'hélice, d'où un grand moment cinétique et une précession marquée en virage (Camel, Dr.I). Pas toujours de vraie manette des gaz : coupe-contact (*blip switch*) et réglage du mélange.
- **Puissance en altitude** : moteurs atmosphériques. Par défaut, relation de Gagg-Farrar ; les moteurs « surcomprimés » (BMW IIIa) se traitent à part.
- **Centrage** : certains avions sont volontairement peu stables (Camel), d'autres très stables (S.E.5a). La stabilité doit émerger de la géométrie et du centrage, pas d'un réglage.
- **Au sol** : béquille de queue, pas de freins, herbe, cheval de bois, capotage.
- **Structures fragiles** : ruptures connues (plan inférieur des sesquiplans Nieuport et Albatros D.III, par exemple). Facteurs de charge limites dans la fiche de recherche.

## Aérostats (dirigeables, ballons d'observation)

- Module `airship` de `escadrille_fdm`, mêmes principes (sans Bevy, f64, déterministe, même trait `Environment`). Équations : section « Aérostats » de `modele-aero.md`.
- Ce qui fait le comportement : flottabilité de l'hydrogène, **altitude de pression** (au-delà, les soupapes perdent du gaz), lest et bombes largables, portance dynamique de la coque inclinée, **masse ajoutée** énorme (sans elle, le dirigeable serait absurdement agile), dérive au vent.
- La masse de gaz est réduite par les fuites que calcule le système hydrogène (skill `combat`) : le FDM ne connaît que la masse et la température du gaz par cellule.
- Données dans `assets/airships/<id>/airship.ron`, essais dans `assets/airships/<id>/essais.ron` (montée après largage de lest, plafond selon la charge, vitesse maximale, altitude de pression), même format d'origine des valeurs que les avions.
- Ballon captif : même modèle, avec un câble (ressort non linéaire) vers le treuil.

## Validation : ce que « juste » veut dire

Un avion est validé quand :
- les **essais quantitatifs** de `essais.ron` passent dans leurs tolérances (Vmax à l'altitude, temps de montée, plafond, vitesse de décrochage, virage et roulis s'ils sont sourcés) ;
- les **modes propres** (oscillation d'incidence, phugoïde, roulis hollandais, spirale, roulis), calculés par linéarisation autour du trim, sont cohérents avec les qualités de vol décrites dans la fiche ;
- les **essais qualitatifs** scénarisés passent (par exemple, le comportement en virage à droite et à gauche d'un Camel, tel que décrit par les sources) ; ils viennent des traits confirmés par les témoignages de pilotes (section 6 de la fiche de recherche) ;
- les **tests de robustesse** passent sans NaN ni divergence : vitesse nulle, glissade sur la queue, vrille, vol sur le dos, piqué à vitesse maximale, posé brutal ;
- la **convergence en pas de temps** est vérifiée : diviser `dt` par deux change les résultats d'essai de moins de 1 %.

## Pièges fréquents

- Degrés et radians ; mph et km/h ; cheval-vapeur métrique (1 PS = 735,5 W) et impérial (1 hp = 745,7 W).
- Portance perpendiculaire à la vitesse **locale** de la bande, pas à l'axe de l'avion.
- Oubli de ω × r dans la vitesse locale : plus d'amortissement en roulis ni en tangage.
- Traînée induite comptée deux fois (polaire 3D, puis traînée induite ajoutée en plus).
- Quaternion non renormalisé ; orientation intégrée dans le mauvais repère.
- Division par zéro à vitesse nulle (au sol, glissade sur la queue) : formulations robustes et seuils.
- Masse, centre de gravité et inertie non mis à jour avec la consommation de carburant et de munitions.
- Coefficient ajusté « pour que ça passe » : interdit sans justification écrite.

## Avant de déclarer fini

- [ ] Tests unitaires ajoutés, `just test` vert
- [ ] `/essai-en-vol` sur les avions touchés, sans régression inexpliquée
- [ ] Revue du subagent `aerodynamicien`
- [ ] Origine des nouveaux coefficients renseignée
