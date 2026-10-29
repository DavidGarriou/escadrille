---
name: realisme
description: Niveaux de réalisme d'Escadrille, du fun (Wings) au très réaliste (IL-2) — ressource RealismSettings, préréglages, assistances de pilotage (instructeur virtuel, coordination automatique, garde anti-décrochage), réglages non physiques signalés, tests de fumée par préréglage. À charger pour toute tâche dans crates/game/src/realism, pour ajouter une aide ou un réglage de difficulté, ou quand une fonctionnalité doit se comporter différemment selon le niveau de réalisme.
---

# Niveaux de réalisme

Décision : `docs/adr/0003-realisme-par-couches.md`. Préréglages : tableau de `docs/vision.md` (**source de vérité** ; ce skill décrit l'implémentation).

## Quatre points d'intervention, pas un de plus

1. **Entrées → commandes** (assistances de pilotage) : instructeur virtuel, coordination automatique, garde anti-décrochage, trim automatique, gestion moteur automatique. Elles produisent des positions de commandes, comme un pilote.
2. **Paramètres du modèle** : facteurs d'échelle documentés (gyroscopique, couple), structure renforcée ou indestructible, pannes désactivées. Ils sont transmis au FDM par sa configuration, jamais codés en dur dans le FDM.
3. **Règles de jeu** : munitions et carburant illimités, multiplicateur de dégâts reçus, mort permanente, tolérance à l'atterrissage.
4. **Présentation** : HUD, marqueurs, aide de visée, vues autorisées, effets du facteur de charge à l'écran.

## Règles

- **Pas de `if arcade`.** Chaque réglage est un champ de `RealismSettings`, lu par **un seul** système (ou une seule fonction de configuration).
- Chaque champ a un commentaire qui dit ce qu'il change et s'il est **non physique**. Les réglages non physiques portent une icône dans l'interface.
- Une assistance n'agit que par les commandes. Une force non physique (par exemple une stabilisation « magique ») est interdite, sauf réglage explicitement non physique.
- L'interface décrit chaque réglage en une phrase et, si possible, sa réalité historique (« Les pilotes du Camel compensaient la précession au palonnier dans chaque virage »).

## Esquisse

```rust
/// Realism settings. Every field is read by exactly one system.
/// Fields marked NON-PHYSICAL change the simulation away from reality.
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct RealismSettings {
    pub flight_assist: FlightAssist,           // Instructor | Coordinated { stall_guard: bool } | Off
    pub gyro_scale: f64,                       // NON-PHYSICAL below 1.0
    pub torque_scale: f64,                     // NON-PHYSICAL below 1.0
    pub engine_management: EngineManagement,   // Auto | ThrottleOnly | ThrottleAndBlip | Full
    pub failures: Failures,                    // Off | Rare | Realistic
    pub structure: Structure,                  // Indestructible (NON-PHYSICAL) | Reinforced(f64) (NON-PHYSICAL) | Real
    pub infinite_ammo: bool,                   // NON-PHYSICAL
    pub infinite_fuel: bool,                   // NON-PHYSICAL
    pub weather: WeatherLevel,                 // Light | Real
    pub special_ammo: SpecialAmmo,             // AutoFastIgnition (NON-PHYSICAL) | Auto | PlayerChoice | PlayerChoiceStrictDates
    pub gunnery_aid: bool,
    pub hud: HudLevel,                         // Full | ShortRange | None
    pub views: ViewRule,                       // All | CockpitAndLimitedExternal | CockpitOnly
    pub g_effects: GEffects,                   // Off | Reduced | Real
    pub altitude_effects: AltitudeEffects,     // Off | Reduced | Real (cold, lack of oxygen)
    pub landing: LandingRule,                  // Forgiving | Normal | Real
    pub pilot_fate: PilotFate,                 // Immortal | Wounded | Permadeath
    pub damage_taken_scale: f64,               // NON-PHYSICAL if not 1.0
}

pub enum Preset { Arcade, Pilot, Veteran, Ace, Custom }
```

## Instructeur virtuel

- **Commande d'attitude** (défaut en Arcade) : stick X = inclinaison voulue (stick relâché → ailes à plat) ; stick Y = taux de tangage ou facteur de charge voulu. Coordination automatique (β → 0) ; limitation de l'incidence sous α_décrochage moins une marge ; gestion des gaz automatique.
- **Vol dirigé par le regard** (option) : l'avion suit la direction regardée (stick droit), à la manière d'un « instructeur » de jeu d'action.
- **Contrôleurs** : PD ou PID par axe, avec gains planifiés selon la pression dynamique. Les gains sont **calculés** à partir des dérivées de commande issues de la linéarisation du FDM (`fdm_lab` produit `assets/aircraft/<id>/assist_gains.ron`) : aucun réglage à la main par avion.
- **Mêmes contrôleurs pour l'IA** (poursuite, mise en virage, ressource) : skill `combat`.
- L'instructeur ne peut pas faire mieux que l'avion : s'il n'y a plus d'énergie, il décroche aussi (gentiment).

## Implémentation en place (M2, plan `m2-aides-pilotage.md`)

- **Code** :
  - `crates/game/src/realism/mod.rs` : `RealismSettings` (un seul champ pour l'instant, `flight_assist`), `Preset`, cycle par le bouton Home ;
  - `realism/assist.rs` : les contrôleurs, en code pur testable sans Bevy.
- **Place** : au pas fixe, entre `FlightSet::Command` (l'avion du joueur prend les commandes du pilote) et `FlightSet::Step`.
- **Gains** : `fdm_lab assist <id>` (`tools/fdm_lab/src/assist.rs`). Trois modèles réduits, avec placement de pôles et intégrateur :
  - roulis (p, φ) ;
  - incidence (α, q), avec Ω et ε résidualisés pour les statiques ;
  - dérapage (β, r).

  Le fichier écrit est décrit dans `simulation-vol/format-avion.md`.
- **Lois** :
  - **coordination** : PI sur β, amortissement de r au-delà de celui du virage, et anticipation du lacet inverse par la vitesse de roulis et l'aileron ;
  - **garde** : min(profondeur du pilote, profondeur qui tient α_décrochage − 2°), avec la profondeur statique de l'incidence et de la vitesse de tangage ;
  - **instructeur** : inclinaison jusqu'à 60°, facteur de charge de 0 à 3,5 g autour du palier, retour au palier en 4 s, montée limitée à 30° tant que la vitesse le permet.
- **Limite connue** : la garde seule (Pilote) ne rend pas la vitesse d'une chandelle. Trop cabré, l'avion passe sur le dos au sommet comme il le ferait réellement ; seul l'instructeur limite la montée.
- **Diagnostic** : le test ignoré `print_a_flight` (`realism/assist.rs`) imprime un vol, réglé par variables d'environnement : `ASSIST`, `STICK`, `THROTTLE`, `SECONDS`, `ROLL_FOR`, `CUT_AT`.

## Tests

- **Tests de fumée par préréglage** : vols scriptés « débutant » (manche à fond en arrière, roulis à fond, gaz coupés en montée). Attendu : en Arcade, ni vrille ni écrasement ; en As, le comportement réel (vrille possible).
- **Test d'absence de triche** : en As, aucune assistance active, aucun facteur d'échelle ≠ 1, aucune règle non physique.
- **Test de cohérence** : à réglages physiques identiques (facteurs d'échelle à 1, assistance de pilotage coupée), une même séquence de commandes produit exactement la même trajectoire quel que soit le préréglage d'origine : seules les couches de présentation et de règles diffèrent.
