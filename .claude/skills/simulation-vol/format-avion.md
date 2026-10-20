# Format des fichiers avion

**Les valeurs ci-dessous sont des emplacements vides (0.0), pas des données.** Le schéma exact vit dans `crates/data` ; ce fichier en donne l'esprit et doit être mis à jour quand le schéma change.

## Règles

- **Quantités issues d'une source** (performances, puissances, masses, dimensions principales) : unité d'origine **typée**, convertie en SI au chargement par `crates/data`. Exemples : `Kmh(..)`, `Mph(..)`, `Ms(..)`, `M(..)`, `Ft(..)`, `Hp(..)`, `Ps(..)`, `Kw(..)`, `Kg(..)`, `Lb(..)`, `L(..)`, `ImpGal(..)`. On recopie ainsi la valeur telle qu'elle est écrite dans la source, sans erreur de conversion à la main.
- **Géométrie détaillée** (positions de bandes, de câbles, de roues) : mètres, repère corps.
- **Chaque valeur numérique est suivie d'un commentaire d'origine** :
  - `// S:<réf>` source, référence à la section « Sources » de la fiche `docs/recherche/<id>.md` ;
  - `// C:<méthode>` calculé (formule, outil, à partir de quelles valeurs) ;
  - `// E:<justification>` ESTIMÉ.
- La validation (`crates/data`, lancée par `just test`) refuse un fichier où une valeur n'a pas d'origine.
- **Inertie** (`inertia_empty_kg_m2`) : avion **à vide, rotor compris et bloqué**, calculée **au centre de gravité à vide** (`cg_empty_m`), en axes corps (x avant, y droite, z bas). Produit d'inertie : `ixz` = ∫ x z dm, le tenseur contenant −ixz (convention de Stevens et Lewis). Certaines sources utilisent le signe opposé : le vérifier et le noter. L'inertie axiale du rotor (moteur, hélice, cylindres d'un rotatif) est donnée à part, avec l'axe de rotation et le sens vu du pilote ; le FDM la retire de la cellule (`modele-aero.md`).

## `assets/aircraft/<id>/aircraft.ron`

```ron
Aircraft(
    id: "spad7",
    name: "SPAD S.VII C.1",
    research: "docs/recherche/spad7.md",
    service: (from: "1916-09", to: "1918-11"),          // S:

    mass: (
        empty: Kg(0.0),                                   // S:
        pilot: Kg(80.0),                                  // E: pilote équipé, valeur commune à tous les avions
        fuel: (capacity: L(0.0), density_kg_per_l: 0.72, position_m: (0.0, 0.0, 0.0)), // S: / E:
        cg_empty_m: (0.0, 0.0, 0.0),                      // C: depuis la fiche (centrage)
        inertia_empty_kg_m2: (ixx: 0.0, iyy: 0.0, izz: 0.0, ixz: 0.0), // C: fdm_lab inertia (estimation par composants)
    ),

    surfaces: [
        (
            name: "upper_wing",
            kind: Wing,
            panels: [(
                root_m: (0.0, 0.0, 0.0), tip_m: (0.0, 0.0, 0.0),
                chord_root_m: 0.0, chord_tip_m: 0.0,
                incidence_rad: 0.0, twist_rad: 0.0, dihedral_rad: 0.0,
                strips: 8,
            )],
            airfoil: "airfoils/spad.ron",                 // polaires 360° et origine des données
            controls: [(control: Aileron, span_from: 0.6, span_to: 1.0, chord_ratio: 0.25, travel_rad: (0.0, 0.0))],
        ),
        // lower_wing, horizontal_tail (+ elevator), vertical_tail (+ rudder)
    ],
    multiplane: Some((gap_m: 0.0, stagger_m: 0.0, decalage_rad: 0.0)), // S:

    parasite_drag: [
        (name: "interplane_struts", cd: 0.0, ref_area_m2: 0.0, position_m: (0.0, 0.0, 0.0)), // C: Hoerner
        (name: "flying_wires",      cd: 0.0, ref_area_m2: 0.0, position_m: (0.0, 0.0, 0.0)), // C:
        (name: "radiator",          cd: 0.0, ref_area_m2: 0.0, position_m: (0.0, 0.0, 0.0)), // E:
    ],

    engine: Inline((
        name: "Hispano-Suiza 8A",
        rated_power: Hp(0.0),                             // S:
        rated_rpm: 0.0,                                   // S:
        power_curve: [(rpm: 0.0, factor: 0.0)],           // S: / E:
        altitude_model: GaggFarrar,                       // C:
    )),
    // ou : Rotary(( ..., blip_switch: true, mixture: true, rotating_inertia_kg_m2: 0.0 ))

    propeller: (
        diameter: M(0.0),                                 // S:
        rotation: Clockwise,                              // vue depuis le siège du pilote ; S:
        table: "props/spad7.ron",                         // C_T(J), C_P(J) ; C: BEMT ou S:
        inertia_kg_m2: 0.0,                               // C:
    ),

    gear: [
        (name: "left_wheel", kind: Wheel, position_m: (0.0, 0.0, 0.0), stiffness_n_per_m: 0.0, damping_n_s_per_m: 0.0, friction: 0.0), // E:
        (name: "tailskid",   kind: Skid,  position_m: (0.0, 0.0, 0.0), stiffness_n_per_m: 0.0, damping_n_s_per_m: 0.0, friction: 0.0), // E:
    ],

    limits: (load_factor_pos: 0.0, load_factor_neg: 0.0, never_exceed: Kmh(0.0)), // S: ou E:

    weapons: [(gun: "vickers_303", position_m: (0.0, 0.0, 0.0), synchronized: true, rounds: 0)], // S:
)
```

## `assets/aircraft/<id>/essais.ron`

```ron
Essais(
    aircraft: "spad7",
    conditions: (fuel_fraction: 1.0, ammo: Full, atmosphere: Isa, wind: None),

    quantitative: [
        MaxLevelSpeed(altitude: M(0.0),  target: Kmh(0.0), tolerance_pct: 3.0, source: "S1", confidence: B),
        ClimbTime(from: M(0.0), to: M(0.0), target_s: 0.0, tolerance_pct: 5.0, source: "S1", confidence: B),
        StallSpeed(power: Idle,          target: Kmh(0.0), tolerance_pct: 5.0, source: "S2", confidence: C),
        ServiceCeiling(target: M(0.0),   tolerance_pct: 5.0, source: "S1", confidence: B),
    ],

    // Manœuvre scriptée + assertion sur la télémétrie. Chaque attente cite une source.
    qualitative: [
        Scenario(name: "stall_power_off", script: "scenarios/stall_power_off.ron", expect: "wing_drop_deg < 30", source: "S4"),
    ],

    robustness: [ZeroAirspeed, TailSlide, SpinEntryRecovery(turns: 3), Inverted(seconds: 10.0), MaxDive, HardLanding],
)
```

Niveaux de confiance : **A** mesure primaire (rapport d'essai d'époque) ; **B** sources secondaires concordantes ; **C** déduit d'avions comparables ; **D** estimation de gameplay. Les essais de confiance C et D sont signalés mais ne bloquent pas la validation.
