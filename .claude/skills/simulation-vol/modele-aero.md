# Modèle aérodynamique : référence

## Repères et conventions de signe

| Repère | Axes | Usage |
|---|---|---|
| Monde local (NED) | x nord, y est, z bas ; origine au point de référence de la carte ; Terre plate locale | état du FDM |
| Corps | x avant (nez), y aile droite, z bas ; origine au point de référence de la fiche avion | forces, moments, inertie |
| Aérodynamique | x selon la vitesse relative | α, β, portance, traînée |
| Bevy | X droite, Y haut, -Z avant | rendu uniquement ; conversion dans `bridge.rs` |

- α (incidence) positive nez haut ; β (dérapage) positive quand le vent relatif vient de la droite.
- p, q, r : vitesses angulaires de roulis, tangage et lacet (repère corps).
- Commandes normalisées dans [-1, 1] : profondeur + = manche tiré (nez haut) ; ailerons + = roulis à droite ; direction + = palonnier droit (nez à droite) ; gaz dans [0, 1].
- Débattements des gouvernes en radians ; le signe de chaque gouverne est documenté dans la fiche avion.

## Équations du mouvement (repère corps)

- Translation : m (v̇ + ω × v) = F_aéro + F_hélice + F_sol + m g_corps
- Rotation, en traitant cellule + rotor (cylindres du rotatif et hélice) comme un seul système. **Convention retenue (revue `aerodynamicien` du 2026-10-20, implémentée dans `rigid_body.rs`)** : Ω_r est la vitesse axiale **absolue** du rotor, et l'inertie de cellule I' **exclut** l'inertie axiale du rotor : I' = I − I_r e eᵀ (I = avion complet, rotor bloqué, celle des fiches). Moment cinétique du rotor : h_r = I_r Ω_r e.
  - I' ω̇ = M_ext − ω × (I' ω + h_r) − Q_net e
  - M_ext contient les moments externes, dont le couple aérodynamique de l'hélice (−Q_hélice e).
  - Rotor : I_r Ω̇_r = Q_net = Q_moteur − Q_hélice − Q_frottements.
  - Au total, la cellule ressent −(Q_moteur − Q_frottements) e : le couple de réaction. **Ne pas ajouter un couple de réaction séparé**, et ne pas ajouter la précession gyroscopique une seconde fois : elle vient du terme ω × h_r.
  - Régime moteur (vitesse relative) : Ω_r − e·ω. Énergie : ½ ωᵀI'ω + ½ I_r Ω_r² (+ translation) ; moment cinétique : I'ω + h_r.
  - Les mêmes équations valent pour un moteur en ligne (h_r plus faible : vilebrequin et hélice).
  - La gravité est ajoutée par l'intégrateur : le modèle d'avion renvoie les seuls efforts aérodynamiques et de contact, si bien que F/m est la force spécifique (facteur de charge).
- Orientation : quaternion q (monde ← corps), q̇ = ½ q ⊗ (0, ω), notation (w ; x, y, z), soit `DQuat::from_xyzw(ω.x, ω.y, ω.z, 0)` dans glam ; renormaliser en fin de pas. Les efforts sont toujours évalués avec un quaternion unitaire.
- Masse variable : m, CG et I recalculés à partir de la masse à vide, du carburant, des munitions et du pilote.
- Intégrateur : RK4 à pas fixe (départ : 240 Hz). Les contacts au sol peuvent demander des sous-pas. Mesurer avant d'optimiser.

## Atmosphère (ISA, troposphère)

- T = 288,15 − 0,0065 h (K)
- p = 101 325 (T / 288,15)^5,2559 (Pa)
- ρ = p / (287,05 T) ; σ = ρ / 1,225
- Vitesse indiquée ≈ vitesse vraie × √σ (erreurs d'instrument plus tard, si utile).
- Vent : champ fonction de l'altitude, plus des rafales. Turbulence de Dryden (MIL-F-8785C, document public) pour le réalisme élevé.

## Surfaces portantes par bandes

Pour chaque bande s (5 à 10 par demi-aile, une ou deux par surface d'empennage) :
1. Vitesse locale : v_s = v_corps + ω × r_s − v_vent,corps + v_induite (souffle d'hélice, déflexion).
2. Projection dans le repère de la bande (corde, envergure, normale) ; composante d'envergure ignorée, ou correction de flèche.
3. α_s = atan2(v_n, v_c) + calage_s + vrillage_s ; q_s = ½ ρ |v_s,plan|²
4. Coefficients C_L(α_s, δ), C_D(α_s, δ), C_M(α_s, δ) tirés de polaires sur ±180°.
5. dL = q_s S_s C_L, perpendiculaire à v_s dans le plan de la bande ; dD = q_s S_s C_D, selon −v_s ; dM = q_s S_s c_s C_M.
6. Moment au CG : r_s × (dL + dD) + dM.

Polaires :
- Données 2D sourcées (souffleries d'époque, NACA) sur la plage mesurée ; extension à 360° par la méthode de Viterna-Corrigan ou par plaque plane (C_N ≈ 2 sin α).
- Passage 2D → 3D par plan : pente de portance corrigée par l'allongement effectif, C_Lα,3D = C_Lα,2D / (1 + C_Lα,2D / (π e AR)), et traînée induite C_Di = C_L² / (π e AR). **À appliquer une seule fois.**
- Biplans et triplans : allongement effectif et traînée induite corrigés par le facteur d'interférence de Prandtl et Munk, fonction du rapport écartement/envergure (Prandtl, *Induced Drag of Multiplanes*, publié par la NACA : vérifier la référence exacte). Le décalage horizontal modifie la répartition de charge entre les plans.
- Gouvernes : variation d'incidence effective Δα = τ δ (efficacité τ selon le rapport corde de gouverne / corde de surface), plus un supplément de traînée ; gouvernes compensées ou non.
- Empennage : déflexion ε = ε0 + (dε/dα) α_aile, avec un retard τ = l_t / V (amortissement d'incidence).
- Décrochage dynamique et hystérésis : optionnels, après la base.

Traînées parasites : chaque élément (mât, câble, hauban, radiateur, train, mitrailleuse, pilote) a un C_D, une surface de référence et une position. Câbles : C_D d'un cylindre (environ 1,0 à 1,2 selon le nombre de Reynolds) × diamètre × longueur ; les câbles profilés (*RAF wires*) traînent moins.

Effet de sol : réduction de la traînée induite en fonction de h / envergure (formule de Wieselsberger ou de McCormick).

## Hélice

Implémentation : `crates/fdm/src/propulsion/propeller.rs`.

- J = V / (n D) ; T = C_T ρ n² D⁴ ; P = C_P ρ n³ D⁵ ; Q = P / (2π n) ; η = J C_T / C_P
- Tables C_T(J), C_P(J) sourcées ou calculées hors ligne par la théorie de l'élément de pale (BEMT, dans `fdm_lab`) pour les hélices en bois à pas fixe. Le tableau commence à J = 0 (point fixe), mesuré ou calculé.
- **Quatre quadrants** : C_T et C_P divergent quand n → 0 (hélice arrêtée, glissade sur la queue). Partout, l'hélice est décrite comme les hélices marines de la série B de Wageningen : angle d'avance β = atan2(V_a, 0,7 π n D), vitesse de référence V_r² = V_a² + (0,7 π n D)², T = C_T* ½ ρ V_r² (π/4) D², Q = C_Q* ½ ρ V_r² (π/4) D³. Dans le tableau, C_T* = 8 C_T / (π (J² + (0,7π)²)) et C_Q* = 8 C_Q / (π (J² + (0,7π)²)), avec C_Q = C_P / 2π. Au-delà, interpolation linéaire en β vers trois points d'appui estimés (E:) : hélice arrêtée, écoulement venant de l'avant (β = 90°) ou de l'arrière (β = −90°), rotation inverse à l'arrêt (β = ±180°). Leurs signes sont contrôlés au chargement.
- **Vitesse de rotation** : l'hélice voit la vitesse **absolue** du rotor (les pales tournent aussi avec la cellule) ; le moteur, la vitesse **relative** à la cellule.
- **Effet P** (modèle simple de JSBSim) : la ligne de poussée se décale de `p_factor` × sin(angle d'incidence de l'hélice), du côté où les pales avancent dans l'écoulement transversal (E:). Hélice à droite (horaire vue du poste) cabrée : lacet à gauche.
- **Souffle d'hélice** (théorie de Froude), fourni comme écoulement local (`local_flow`) des surfaces et des traînées : vitesse induite au disque v_i = ½ (√(V² + 2T / (ρ A)) − V), v(x) = v_i (1 + x / √(x² + R²)) à la distance x derrière le disque (2 v_i loin en aval), tube contracté à débit constant, bord adouci sur ±10 % du rayon (E:). Pas de souffle en moulinet (T ≤ 0). Limites (E:) : souffle axial seulement (pas de rotation), rectiligne le long de l'axe de traction.
- Non modélisé pour l'instant : force normale de l'hélice (Ribner, NACA Report 819, 1945), légèrement déstabilisante en tangage et en lacet.

## Moteurs

Implémentation : `crates/fdm/src/propulsion/engine.rs`.

- **En ligne** : puissance au frein P_b = (1 + k) P₀(ω) σ θ − k P₀(ω). P₀(ω) est la courbe de puissance au frein à pleins gaz au niveau de la mer, σ le rapport de densité, θ l'ouverture effective des gaz et k la part des frottements (puissance de frottement à pleine puissance, rapportée à la puissance au frein). La puissance indiquée suit la densité de la charge et les gaz, les frottements non. À pleins gaz, on retrouve la relation de Gagg-Farrar : P_b / P₀ = σ − k (1 − σ), avec k = 1/7,55 dans sa forme usuelle ; k est sourcé dans la fiche du moteur (Hispano-Suiza 8A : 0,08 à 0,11 selon le régime d'après le rapport NACA 46, proposition de l'historien à valider, voir `docs/recherche/hispano-suiza-8a.md`). Un k constant par moteur est une approximation. L'origine exacte de la relation dite de Gagg-Farrar n'est pas encore retrouvée.
- Gaz : θ = ralenti + (1 − ralenti) × manette (E:, linéaire). Couple P₀/ω tenu constant hors de la courbe mesurée (E:). Sous le régime de calage, le moteur cesse d'allumer (il repart si l'hélice le fait mouliner assez vite) ; le frottement s'annule linéairement à l'arrêt.
- Rotatifs : I_r inclut carter, cylindres et hélice ; coupe-contact (tout ou rien, ou par groupes de cylindres) ; mélange (manette fine) : une richesse hors plage provoque ratés, noyage, arrêt ; refroidissement par la rotation.
- Températures (huile, eau), consommation, pannes : modèle simple d'abord, activé par les réglages de réalisme.

## Sol

- Roues et béquille : ressort-amortisseur non linéaire par point de contact, frottement (herbe, terre) ; la béquille qui frotte est le seul « frein ».
- Pas de freins ; au sol, on dirige par la gouverne de direction (dans le souffle d'hélice) et la béquille.
- Points de contact supplémentaires (saumons d'aile, nez, dos) pour le cheval de bois et le capotage.

## Structure

- Facteur de charge n = L / (m g) ; limites positive et négative, pression dynamique maximale par avion (fiche de recherche) ; marge avant rupture ; fatigue cumulée en option.
- Rupture : la surface se détache ou perd sa portance, d'où asymétrie et chute. Un endommagement réduit les limites.

## Aérostats

- **Flottabilité** : B = (ρ_air − ρ_gaz) g V_gaz, avec ρ_gaz = p M_gaz / (R T_gaz) (hydrogène imparfaitement pur : mélange à documenter). La température du gaz peut différer de celle de l'air (échauffement au soleil, refroidissement à l'ombre d'un nuage ou la nuit) : portance variable.
- **Sous l'altitude de pression**, la masse de gaz est conservée et les cellules se dilatent en montant : la portance statique reste à peu près constante. **Au-delà**, les cellules sont pleines, les soupapes automatiques relâchent du gaz et la perte de portance est irréversible.
- **Équilibre** : poids (structure, équipage, carburant, bombes, lest d'eau) contre flottabilité. Larguer du lest ou des bombes crée un excédent de portance et une montée rapide ; consommer du carburant allège lentement.
- **Portance dynamique** : coque inclinée en mouvement, traitée comme un corps élancé (portance et moment de Munk, déstabilisant), stabilisée par les empennages et leurs gouvernes.
- **Masse et inertie ajoutées** : l'air entraîné s'ajoute à la masse et à l'inertie apparentes, très différemment selon l'axe (coefficients de Lamb pour un ellipsoïde allongé). Indispensable.
- **Vent** : vitesse propre faible, donc dérive forte ; le vent en altitude décide souvent de la route réelle.
- **Équations** : mêmes que pour un avion, avec la flottabilité appliquée au centre de volume du gaz (qui se déplace quand une cellule se vide) et les termes de masse ajoutée.

## Linéarisation et modes propres (outils)

- Trim : trouver (profondeur, gaz, ailerons, direction, α, β) pour un vol rectiligne stabilisé à V et h donnés (Newton ou Levenberg-Marquardt).
- Jacobien numérique de l'état autour du trim (différences centrées) → valeurs propres (nalgebra) → pulsation, amortissement et constante de temps de chaque mode.
- Les dérivées de commande servent aussi à calculer automatiquement les gains de l'instructeur virtuel et de l'IA (skill `realisme`).

## Bibliographie de départ (à compléter dans les fiches)

- McCormick, *Aerodynamics, Aeronautics, and Flight Mechanics*
- Stevens et Lewis, *Aircraft Control and Simulation*
- Etkin, *Dynamics of Flight: Stability and Control*
- Hoerner, *Fluid-Dynamic Drag* (câbles, mâts, radiateurs)
- NASA Technical Reports Server : rapports NACA, dans le domaine public
