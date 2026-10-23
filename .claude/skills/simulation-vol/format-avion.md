# Format des fichiers avion

**Les valeurs ci-dessous sont des emplacements vides (0.0), pas des données.** Le schéma exact vit dans `crates/data` (`aircraft.rs`, `airfoil.rs`, `propeller.rs`, `essais.rs`) ; ce fichier en donne l'esprit et doit être mis à jour quand le schéma change. Un exemple complet et chargé par les tests : `crates/data/tests/fixtures/assets/` (biplan synthétique).

## Règles

- **Quantités issues d'une source** (performances, puissances, masses, dimensions principales) : unité d'origine **typée**, convertie en SI au chargement par `crates/data` (`units.rs`). On recopie ainsi la valeur telle qu'elle est écrite dans la source, sans erreur de conversion à la main. Unités disponibles :
  - longueurs : `M`, `Cm`, `Mm`, `Ft`, `In`, `FtIn(pieds, pouces)` ;
  - surfaces : `M2`, `Ft2` ;
  - masses : `Kg`, `Lb` ;
  - vitesses : `Ms`, `Kmh`, `Mph`, `Kt`, `FtMin` ;
  - puissances : `W`, `Kw`, `Hp` (cheval anglo-saxon, 745,7 W), `Ps` et `Ch` (cheval-vapeur, 735,5 W) ;
  - volumes : `L`, `ImpGal`, `UsGal` ;
  - angles : `Deg`, `Rad` ;
  - régimes : `Rpm`, `RadS` ;
  - durées : `S`, `Min`, `MinS(minutes, secondes)`.
- **Géométrie détaillée** (positions de bandes, de câbles, de roues) : mètres, repère corps (x avant, y droite, z bas), **depuis un point de référence** (`datum`) décrit dans le fichier. Le chargeur calcule le centre de gravité pour un chargement donné et y ramène toutes les positions.
- **Chaque ligne qui contient un nombre se termine par un commentaire d'origine** :
  - `// S:<réf>` source, référence à la section « Sources » de la fiche `docs/recherche/<id>.md` ;
  - `// C:<méthode>` calculé (formule, outil, à partir de quelles valeurs) ;
  - `// E:<justification>` ESTIMÉ.
  - Un commentaire `//! S:…` (ou `C:`, `E:`) sur sa propre ligne couvre tout un bloc, par exemple un tableau, jusqu'à la prochaine ligne vide.
- La validation (`crates/data`, lancée par `just test`) refuse un fichier où un nombre n'a pas d'origine, en donnant les numéros de ligne. Les nombres dans les chaînes et les identifiants (`"S1"`, `spad7`) ne comptent pas.
- **Inertie** (`inertia_empty_kg_m2`) : avion **à vide, rotor compris et bloqué**, calculée **au centre de gravité à vide** (`cg_empty_m`), en axes corps. Produit d'inertie : `ixz` = ∫ x z dm, le tenseur contenant −ixz (convention de Stevens et Lewis). Certaines sources utilisent le signe opposé : le vérifier et le noter. L'inertie axiale du rotor (`rotor_inertia_kg_m2` : hélice, vilebrequin, et cylindres d'un rotatif) est donnée à part ; le FDM la retire de la cellule (`modele-aero.md`). Pilote, carburant et charges sont des masses ponctuelles ajoutées par le théorème de Huygens.
- **Gouvernes** : leur signe n'est **pas** dans les données. Le chargeur le déduit de la géométrie (une commande positive doit donner le moment positif du pilote), ce qui supprime une source d'erreur.

## `assets/aircraft/<id>/aircraft.ron`

```ron
Aircraft(
    id: "spad7",
    name: "SPAD S.VII C.1",
    research: "docs/recherche/spad7.md",
    datum: "bord d'attaque de l'emplanture du plan inférieur, dans l'axe",

    mass: (
        empty: Kg(0.0),                                   // S:
        cg_empty_m: (0.0, 0.0, 0.0),                      // C: depuis la fiche (centrage)
        inertia_empty_kg_m2: (ixx: 0.0, iyy: 0.0, izz: 0.0, ixz: 0.0), // C: estimation par composants
        pilot: Kg(80.0),                                  // E: pilote équipé, valeur commune à tous les avions
        pilot_m: (0.0, 0.0, 0.0),                         // C:
        fuel: (capacity: L(0.0), density_kg_per_l: 0.72, position_m: (0.0, 0.0, 0.0)), // S: / E:
        payload: [(name: "vickers", mass: Kg(0.0), position_m: (0.0, 0.0, 0.0))], // S:
    ),

    surfaces: [
        (
            name: "upper_wing",
            kind: Horizontal,                             // ou Vertical (dérive : portance vers la droite)
            mirrored: true,                               // les panneaux décrivent la moitié droite
            panels: [(
                root_m: (0.0, 0.0, 0.0), tip_m: (0.0, 0.0, 0.0), // ligne du quart de corde ; S: / C:
                chord_root: M(0.0), chord_tip: M(0.0),    // S:
                incidence: Deg(0.0),                      // S: (positif : bord d'attaque vers la portance)
                twist: Some(Deg(0.0)),                    // S: vrillage saumon − emplanture (facultatif)
                strips: 8,                                // E: discrétisation
            )],
            airfoil: "airfoils/raf15.ron",                // depuis la racine des assets
            span_efficiency: 0.0,                         // C: / E:
            aspect_ratio: None,                           // par défaut envergure² / surface ; à donner pour une dérive (effet de plaque du fuselage)
            controls: [(control: Aileron, panel: 0, span_from: 0.6, span_to: 1.0, chord_ratio: 0.25)], // S:
            reference_area: Some(M2(0.0)),                // S: surface de la fiche, comparée aux bandes (±2 %)
        ),
        // lower_wing, tailplane (+ Elevator), fin (Vertical, + Rudder)
    ],
    interference: [(planes: ("upper_wing", "lower_wing"), sigma: 0.0)], // C: Prandtl, NACA TN 182, écartement / envergure
    downwash: Some((wings: ["upper_wing", "lower_wing"], tail: ["tailplane"], factor: 0.0, tail_arm: M(0.0))), // C:

    parasite_drag: [                                      // traînées axiales, dans le souffle de l'hélice
        (name: "fuselage",          cd: 0.0, reference_area: M2(1.0), position_m: (0.0, 0.0, 0.0)), // C: contrôle de traînée globale
        (name: "interplane_struts", cd: 0.0, reference_area: M2(0.0), position_m: (0.0, 0.0, 0.0)), // C: Hoerner
        (name: "radiator",          cd: 0.0, reference_area: M2(0.0), position_m: (0.0, 0.0, 0.0)), // E:
    ],

    // Fuselage en corps élancé (couple de Munk et écoulement transversal, modele-aero.md) ;
    // facultatif, mais sans lui les stabilités de route et longitudinale sont surestimées.
    fuselage: Some((
        position_m: (0.0, 0.0, 0.0),                      // C: centre des aires projetées
        volume_m3: 0.0,                                   // C: depuis les plans
        apparent_mass: 0.0,                               // C: k₂ − k₁ de Lamb selon la finesse (≈ 0,9 pour une finesse de 6)
        plan_area: M2(0.0), side_area: M2(0.0),           // C: vues de dessus et de côté
        crossflow_drag: 0.0,                              // C: η C_dc (Allen et Perkins, NACA Report 1048)
    )),

    engine: Inline((
        name: "Hispano-Suiza 8Aa",
        power_curve: [(speed: Rpm(0.0), power: Ch(0.0))], // S: puissance au frein, pleins gaz, niveau de la mer
        friction: Curve([(speed: Rpm(0.0), power: Hp(0.0))]), // S: puissance de frottement mesurée en entraînant le moteur
        // ou friction: Fraction(0.0),                    // C: le k de Gagg-Farrar, P_f = k P₀
        altitude_law: Pressure(0.93),                     // C: c = (p/p₀)ⁿ sur la puissance indiquée (R&M 462, frein en ^1,04, converti avec le frottement) ; ou Density (c = σ)
        installed_power: 1.0,                             // facultatif (1 par défaut) ; C: fraction de la puissance indiquée du banc fournie en vol (Report 301, voir modele-aero.md)
        idle_throttle: 0.0,                               // E:
        stall_speed: Rpm(0.0),                            // E:
    )),
    // plus tard : Rotary(( ..., blip_switch: true, mixture: true ))

    propeller: (
        table: "aircraft/spad7/propeller.ron",            // C_T(J), C_P(J) et points d'appui ; C: BEMT ou S:
        diameter: M(0.0),                                 // S:
        hub_m: (0.0, 0.0, 0.0),                           // S:
        rotation: Clockwise,                              // vue de derrière l'hélice, vers l'avant (du poste pour un tracteur) ; S:
        p_factor_m: 0.0,                                  // E: décalage de la ligne de poussée par unité de V⊥ V_pale / V_r²
        normal_force: 0.0,                                // C: c_n de Ribner, 0,03 à 0,05 pour une bipale en bois (refusé au-delà de 0,1)
        rotor_inertia_kg_m2: 0.0,                         // C: hélice ≈ m R² / 5, plus le vilebrequin
    ),

    travel: (aileron: Deg(0.0), elevator: Deg(0.0), rudder: Deg(0.0)), // S: / E:
)
```

Prévus plus tard, hors du schéma actuel : train d'atterrissage (M2), limites structurales (M4), armes (M4).

## `assets/airfoils/<id>.ron`

```ron
Airfoil(
    name: "RAF 15",
    source: "docs/recherche/profil-spad7.md, S2",
    convention: Absolute,            // ou Halved : coefficients britanniques k = C/2
    moment_reference: QuarterChord,  // ou LeadingEdge
    test_wing: Some((aspect_ratio: 6.0, tau: 0.0, delta: 0.0)), // S: / C: maquette de soufflerie ; None pour un profil 2D
    camber_correction: Some((zero_lift_shift: Deg(0.0), cm_shift: 0.0)), // facultatif ; C: théorie du profil mince
    fabric_drag_rise: 0.0,           // facultatif (0 par défaut) ; C: calé sur un avion en vraie grandeur
    points: [
        //! S: tableau de la source, page
        (alpha: Deg(0.0), cl: 0.0, cd: 0.0, cm: 0.0),
    ],
)
```

Le chargeur applique, dans l'ordre :
1. les coefficients britanniques doublés (`Halved`) et le moment ramené au quart de corde ;
2. le passage de la maquette d'allongement fini à l'allongement infini (corrections de Glauert τ et δ), puisque le FDM ajoute lui-même l'angle induit ;
3. `camber_correction` : un tableau mesuré sur un profil sert pour un profil voisin. Toute la courbe est décalée de la différence d'incidence de portance nulle et de C_m entre les deux lignes moyennes ; pente, traînée et décrochage restent ceux du tableau ;
4. `fabric_drag_rise` : traînée ΔC_d = k C_l² ajoutée à chaque point. Les tableaux viennent de maquettes polies ; une aile entoilée réelle traîne davantage, de plus en plus avec la portance (toile qui se creuse entre les nervures, rugosité). k se cale sur un avion **mesuré en vraie grandeur**, piloté par `fdm_lab` sur le même tableau, jamais sur les cibles de l'avion du jeu. Le RAF 15 : 0,020 ± 0,005, d'après le Sperry Messenger du NACA Report 304 (`tools/fdm_lab/comparison/`, test `the_fabric_drag_of_the_spad_wing_reproduces_the_full_scale_messenger`).

Le tableau doit dépasser le décrochage des deux côtés ; un décrochage négatif absent de la source s'ajoute explicitement, en `E:`.

## Hélice (`propeller.ron`)

```ron
Propeller(
    name: "…",
    source: "…",
    points: [
        //! S: / C:
        (j: 0.0, ct: 0.0, cp: 0.0),       // le tableau commence au point fixe J = 0
    ],
    anchors: (                            // quatre quadrants, voir modele-aero.md ; E: ou C: BEMT ; le chargeur refuse une hélice qui crée de l'énergie
        stopped: (ct: 0.0, cq: 0.0),
        reversed_flow: (ct: 0.0, cq: 0.0),
        reversed_rotation: (ct: 0.0, cq: 0.0),
    ),
)
```

## `assets/aircraft/<id>/essais.ron`

```ron
Essais(
    aircraft: "spad7",
    conditions: (fuel_fraction: 1.0),                     // S: ou E: chargement de l'essai d'époque

    quantitative: [
        MaxLevelSpeed(altitude: M(0.0), target: Kmh(0.0), tolerance_pct: 3.0, source: "S1", confidence: B), // S:
        // avec le régime relevé au même essai (contrôle hélice et moteur ensemble) :
        MaxLevelSpeed(altitude: M(0.0), target: Kmh(0.0), tolerance_pct: 3.0,
            rpm: Some((target: Rpm(0.0), tolerance_pct: 3.0)), source: "S1", confidence: A), // S:
        ClimbTime(from: M(0.0), to: M(0.0), target: MinS(0, 0), tolerance_pct: 5.0, source: "S1", confidence: B), // S:
        StallSpeed(power: Idle, target: Kmh(0.0), tolerance_pct: 5.0, source: "S2", confidence: C), // S:
        // power: Full = vitesse stabilisée minimale pleins gaz (l'avion monte) : définition
        // à confirmer par l'historien avant d'y attacher une cible (l'époque mesurait
        // plutôt une vitesse minimale en palier).
        Ceiling(climb_rate: FtMin(100.0), target: M(0.0), tolerance_pct: 5.0, source: "S1", confidence: B), // S:
    ],

    // Comportements tirés des témoignages (section 6 de la fiche), pilotés par fdm_lab
    // (pilote scripté aux mêmes commandes que le joueur) ; rapportés, jamais bloquants
    // sauf valeurs non finies.
    qualitative: [
        (name: "boucle", description: "boucle aisée, sans décrochage au sommet", source: "T1",
            manoeuvre: Some(Loop(altitude: M(2000.0), speed: Some(Kmh(170.0)), load_factor: 3.5))), // S: vitesse de croisière ; E: traction d'un pilote d'époque
        (name: "decrochage", description: "abattée nette et départ en roulis", source: "T9",
            manoeuvre: Some(PowerOffStall(altitude: M(1000.0)))),                    // E:
        (name: "plane", description: "plané plus pentu que celui d'un Nieuport", source: "T9",
            manoeuvre: Some(Glide(altitude: M(1000.0)))),                            // E:
        (name: "pique", description: "tient un piqué quasi vertical à pleine puissance", source: "T6",
            manoeuvre: Some(Dive(altitude: M(3000.0), angle: Deg(-80.0), seconds: 20.0, load_factor: 4.0))), // S: / E:
        (name: "embardee", description: "part à gauche au décollage", source: "T8"), // sans manœuvre : listé, pas simulé (M2)
    ],

    robustness: [ZeroAirspeed, TailSlide, SpinEntryRecovery(turns: 3), Inverted(seconds: 10.0), MaxDive],
)
```

Manœuvres qualitatives (`fdm_lab/src/qualitative.rs`), vérifiées au chargement (facteur de charge > 1, piqué entre −90° et 0° et d'au moins 5 s) : départ cap au nord, ailes horizontales, air calme. Le pilote vise un facteur de charge sans dépasser l'incidence de décrochage (à la manette de la manœuvre) moins 2°, anticipée sur sa vitesse de variation ; il garde les ailes dans le plan vertical aux ailerons et annule le dérapage à la direction (proportionnel et intégral).
- `Loop` (`speed` facultative, vitesse maximale en palier par défaut) : pleins gaz ; bouclée quand le **vecteur vitesse** a tourné d'un tour sans sortir du plan (inclinaison < 45°) ; sommet (hauteur, vitesse, rapport à V_s), part du temps sur la limite d'incidence, marge au décrochage.
- `PowerOffStall` : depuis le palier à 1,3 V_s, gaz réduits, vitesse diminuée d'**un nœud par seconde** (pratique des essais de certification) jusqu'au départ : abattée (le nez plonge) ou départ en roulis (inclinaison > 30°). Dès l'incidence de décrochage atteinte, plus d'ailerons ni de palonnier (trim de plané) : un pilote qui lutte contre le roulis croise les commandes et provoque lui-même le départ (revue de l'aérodynamicien). Après le départ, l'avion seul (manche tenu) jusqu'à ce que le nez cesse de plonger (3 s au plus), ou 1 s et 60° d'inclinaison au plus après un départ en roulis (au-delà, c'est une spirale ou une vrille) : abattée, inclinaison et son côté, cap ; puis le pilote rétablit. Vitesses vraie et équivalente, dérapage au décrochage, décélération mesurée.
- `Glide` : finesse maximale au ralenti.
- `Dive` : piqué tenu à l'angle donné (pente mesurée dans le plan vertical, valable jusqu'à −90°), pleins gaz ; signe du moment de tangage profondeur au neutre ; puis deux branches depuis la fin du piqué : profondeur relâchée au neutre 3 s (la pente remonte : rappelant ; elle s'accentue : engagement), et ressource jusqu'à une trajectoire horizontale. Chiffres du piqué établi (après 2 s).

Niveaux de confiance : **A** mesure primaire (rapport d'essai d'époque) ; **B** sources secondaires concordantes ; **C** déduit d'avions comparables ; **D** estimation de gameplay. Par défaut, les essais de confiance C et D sont signalés mais ne bloquent pas la validation ; le seuil se règle (`Confidence::blocks_validation`). Pour le M1, le joueur a validé la confiance C : les essais A, B et C bloquent, D reste indicatif.
