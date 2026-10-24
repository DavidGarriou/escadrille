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
- Passage 2D → 3D par plan (`aero/surface.rs`) : angle induit α_i = C_L / (π e AR) résolu exactement entre les deux points de décrochage (méthode d'Illinois), au-delà, il garde sa valeur au décrochage et décroît linéairement jusqu'à zéro à 40° d'incidence géométrique (ou 15° après le décrochage, si c'est plus loin) ; traînée induite par inclinaison de la portance. **À appliquer une seule fois** : les polaires mesurées sur maquette sont ramenées à l'allongement infini au chargement (Glauert τ et δ).
- Traînée de l'aile réelle : les polaires de maquettes polies sous-estiment la traînée de profil d'une aile entoilée, surtout à forte portance. Le fichier de profil ajoute ΔC_d = k C_l² (`fabric_drag_rise`), calé sur un avion mesuré en vraie grandeur (polaire de planés, `fdm_lab polaire`) ; ce terme porte aussi, faute de mieux, ce que le modèle de biplan ne voit pas de la traînée liée à la portance. Ordre de grandeur pour le RAF 15 : k = 0,020, soit un coefficient d'Oswald de l'avion complet qui passe d'environ 0,89 à 0,73.
- Biplans et triplans (`aero/system.rs`, `LiftingSystem`) : induction mutuelle de Prandtl entre plans, α_i,j += k_ij C_L,j avec k_ij = σ S_j / (π b_i b_j) (NACA TN 182 ; σ selon l'écartement / envergure), résolue par point fixe. Le décalage horizontal modifie la répartition de charge entre les plans.
- Gouvernes : variation d'incidence effective Δα = τ δ (efficacité τ selon le rapport corde de gouverne / corde de surface), plus un supplément de traînée ; gouvernes compensées ou non.
- Empennage : déflexion de l'aile, retardée de l_t / V (voir « Assemblage de l'avion »).
- Décrochage dynamique et hystérésis : optionnels, après la base.

Traînées parasites : chaque élément (mât, câble, hauban, radiateur, train, mitrailleuse, pilote) a un C_D, une surface de référence et une position. Câbles : C_D d'un cylindre (environ 1,0 à 1,2 selon le nombre de Reynolds) × diamètre × longueur ; les câbles profilés (*RAF wires*) traînent moins.

Effet de sol : réduction de la traînée induite en fonction de h / envergure (formule de Wieselsberger ou de McCormick).

## Assemblage de l'avion

Implémentation : `crates/fdm/src/aircraft.rs` (`Aircraft::evaluate`, `Aircraft::step`).

- Ordre de calcul pour un état : données de l'air (vent, densité, pression) ; hélice (traction, couple, souffle redressé derrière les ailes) ; moteur ; surfaces portantes (`LiftingSystem`, souffle réduit par le facteur de largeur du jet, déflexion retardée sur l'empennage) ; traînées parasites et fuselage (dans tout le souffle) ; somme.
- **Fuselage en corps élancé** (`crates/fdm/src/aero/body.rs`, Allen et Perkins, NACA Report 1048, 1951) : couple de Munk ρ Vol (k₂ − k₁) u × (écoulement transversal), qui tourne le corps en travers de l'écoulement (déstabilisant en tangage et en lacet), et force d'écoulement transversal visqueuse ½ ρ η C_dc A |V_c| V_c, avec l'aire vue de dessus pour la composante verticale et l'aire vue de côté pour la latérale. La traînée axiale reste dans les traînées parasites. Sans ce terme, les stabilités de route et longitudinale sont surestimées.
- **Déflexion de l'aile sur l'empennage** : ε = `factor` × C_L des ailes (elle s'effondre avec la portance au décrochage), avec un retard l_t / V modélisé par un premier ordre (Padé d'ordre 1 du retard), dε/dt = (factor × C_L − ε) V / l_t : état auxiliaire `AUX_DOWNWASH` intégré par le RK4 avec le corps (`RigidBodyState::aux`). D'où l'amortissement classique M_α̇. La vitesse de déflexion ε V est appliquée aux bandes de l'empennage, à l'opposé de la direction de portance du plan de symétrie, et s'annule en vol queue en avant (pondération max(0, u/V)). `factor` : par exemple monoplan équivalent b²/(κ S), κ = (1 + σ)/2, puis abaques du NACA TR 648 (C:).
- Commandes du pilote dans [−1, 1] (gaz dans [0, 1]), converties en braquages par les débattements de l'avion (symétriques pour l'instant).

## Hélice

Implémentation : `crates/fdm/src/propulsion/propeller.rs`.

- J = V / (n D) ; T = C_T ρ n² D⁴ ; P = C_P ρ n³ D⁵ ; Q = P / (2π n) ; η = J C_T / C_P
- Tables C_T(J), C_P(J) sourcées ou calculées hors ligne par la théorie de l'élément de pale (BEMT, dans `fdm_lab`) pour les hélices en bois à pas fixe. Le tableau commence à J = 0 (point fixe), mesuré ou calculé.
- **Quatre quadrants** : C_T et C_P divergent quand n → 0 (hélice arrêtée, glissade sur la queue). Partout, l'hélice est décrite comme les hélices marines de la série B de Wageningen : angle d'avance β = atan2(V_a, 0,7 π n D), vitesse de référence V_r² = V_a² + (0,7 π n D)², T = C_T* ½ ρ V_r² (π/4) D², Q = C_Q* ½ ρ V_r² (π/4) D³. Dans le tableau, C_T* = 8 C_T / (π (J² + (0,7π)²)) et C_Q* = 8 C_Q / (π (J² + (0,7π)²)), avec C_Q = C_P / 2π. Au-delà, interpolation linéaire en β vers trois points d'appui estimés (E:) : hélice arrêtée, écoulement venant de l'avant (β = 90°) ou de l'arrière (β = −90°), rotation inverse à l'arrêt (β = ±180°). Leurs signes sont contrôlés au chargement.
- **Vitesse de rotation** : l'hélice voit la vitesse **absolue** du rotor (les pales tournent aussi avec la cellule) ; le moteur, la vitesse **relative** à la cellule.
- **Énergie** : le tableau et les points d'appui ne doivent jamais faire fournir à l'hélice plus de puissance qu'elle n'en reçoit, Q Ω − T V_a ≥ 0, soit C_Q* cos β / 0,35 − C_T* sin β ≥ 0 sur tout le tour (contrôlé au chargement ; dans le tableau, c'est η ≤ 1). Points d'appui d'une bipale en bois (P/D ≈ 0,8, solidité ≈ 0,08), calculés par éléments de pale par l'aérodynamicien (C:) : arrêt (−0,065 ; −0,008), écoulement inversé (+0,07 ; +0,008), rotation inverse (−0,04 ; −0,007).
- **Écoulement transversal V⊥** (incidence de l'hélice, rotation de l'avion au moyeu) :
  - **effet P** (d'après JSBSim) : la ligne de poussée se décale de `p_factor` × V⊥ V_pale / V_r², du côté où les pales avancent dans l'écoulement transversal (E:). Il s'annule au repos et hélice arrêtée, et s'inverse en rotation inverse. Hélice à droite (horaire vue du poste) cabrée : lacet à gauche ;
  - **force normale** (Ribner, NACA Report 819, 1945) : force dans le plan du disque, opposée à l'écoulement transversal, ½ ρ V_r A c_n V⊥, appliquée au moyeu (déstabilisante devant le centre de gravité). Depuis le C_Nα de Ribner (par radian, sur q∞ et le disque) : c_n = C_Nα J / √(J² + (0,7π)²). Ordre de grandeur 0,03 à 0,05 pour une bipale en bois ; le chargement refuse plus de 0,1.
- **Souffle d'hélice** (théorie de Froude), fourni comme écoulement local des surfaces, des traînées et du fuselage (plan `docs/plans/m1-souffle-ailes.md`) :
  - **vitesse axiale** : vitesse induite au disque v_i = ½ (√(V² + 2T / (ρ A)) − V), des deux côtés de T = 0 ; une hélice qui freine ralentit l'air derrière elle, v_i étant bornée à −V/2 (début du sillage turbulent, E:). v(x) = v_i (1 + x / √(x² + R²)) à la distance x derrière le disque (2 v_i loin en aval). Tube à débit constant, dont la surface triple au plus derrière une hélice qui freine (E:). Bord adouci sur ±10 % du rayon (E:) ;
  - **rotation** : r v_θ se conserve le long d'une ligne de courant ; elle vaut K = Q / ṁ juste derrière le disque et 0 devant (raccord C¹ sur ±5 % du rayon), avec un noyau en rotation solide au quart du rayon (E:). Elle tourne avec l'hélice : le souffle frappe la dérive par la gauche pour une hélice à droite, d'où la tendance à partir à gauche à pleine puissance. Derrière le bord de fuite des ailes, il n'en reste que la fraction `swirl_left` (environ ½, NACA Report 747 et TN 1146), atteinte sur une corde moyenne ;
  - **largeur du jet** : les bandes portantes ne sentent pas tout le souffle (`lifting_flow`). Ce qu'il ajoute à leur portance (pression dynamique et incidence de rotation) est multiplié par le facteur η du tableau 2 de R&M 2368 (Squire et Chester, 1945), interpolé en V/v et en 1/μ = a₀ c / 8πR ; η tend vers 1 quand le jet ralentit vers la vitesse de l'avion. Les éléments de traînée et le fuselage gardent tout le souffle. Avion de comparaison : la maquette motorisée du NACA TN 1146 (`fdm_lab --assets tools/fdm_lab/comparison souffle`), dont l'aile reprend 40 % du couple pour 39 à 50 % mesurés ;
  - **limite connue** : la théorie des bandes incline toute la portance due à la rotation de l'angle du souffle. La cellule reprend ainsi au tourbillon, en poussée, une part de son énergie que la ligne portante (R&M 2368, partie II : 28 à 37 %) ne donne pas ; le biplan d'essai dépasse même 100 % à Vmax (`fdm_lab souffle <avion>`) ;
  - rectiligne le long de l'axe de traction (E:).
- **Polaire de la cellule** (`fdm_lab polaire`) : « traînée vraie » du Report 304, mesurée hélice à traction nulle. L'outil évalue la cellule sans propulsion (`Aircraft::without_propulsion`) dans l'état de chaque plané, donc sans les forces de l'hélice ni son souffle.

## Moteurs

Implémentation : `crates/fdm/src/propulsion/engine.rs`.

- **En ligne** : deux courbes au niveau de la mer en fonction du régime ω, la puissance au frein à pleins gaz P₀(ω) et la **puissance de frottement** P_f(ω) (frottements et pompage, mesurés en entraînant le moteur ; Hispano-Suiza 8A : rapport NACA 46, 13,1 à 25 hp de 1 300 à 2 100 tr/min). Les données peuvent aussi donner P_f = k P₀ (fraction). La puissance indiquée à pleins gaz P₀ + P_f suit le **rapport de charge** c de la loi d'altitude, les frottements non : P_b = c (P₀ + P_f) − P_f. Avec c = σ et P_f = k P₀, c'est la relation de Gagg-Farrar P_b / P₀ = σ − k (1 − σ) (k = 1/7,55 dans sa forme usuelle ; origine exacte non retrouvée).
- **Loi d'altitude**, choisie dans les données d'après la fiche du moteur : `Density` (c = σ) ou `Pressure(n)` (c = (p/p₀)ⁿ en atmosphère type ; par jour non standard, multipliée par (T_type/T)^m à pression égale, T_type étant la température type à cette pression). **m est une donnée du moteur** (`temperature_exponent`, 0 par défaut), à sourcer : pour le Hispano, 0,4 (de 0,27 à 0,46 sur la puissance indiquée selon le Report 220, le L.C.E. 196 et la cohérence avec n ; R&M 462 ne trouve pas d'effet sensible en vol). La loi `Density` revient à m = 1, que ces mesures rejettent. **L'exposant porte sur la puissance indiquée**, le frottement étant retranché ensuite : une loi mesurée sur la puissance **au frein** se convertit avec la courbe de frottement du moteur. R&M 462 mesure en vol, sur un Hispano de 150 hp, une puissance au frein en (p/p₀)^1,04, soit n ≈ 0,93 sur la puissance indiquée avec le frottement du Report 46 (à ±0,6 % de 1 000 à 5 000 m ; erreur relevée par l'aérodynamicien le 2026-10-22).
- **Puissance installée** (`installed_power`, 1 par défaut) : les courbes sont des mesures au banc ; monté sur l'avion, le moteur semble fournir un peu moins en vol. La puissance indiquée est multipliée par cette fraction, comme une charge plus faible, le frottement restant entier. NACA Report 301 (VE-7, moteur Wright-Hispano) : 3 à 5 % de moins qu'au banc dans trois essais sur quatre, cause non identifiée par la source, accord « probablement dans la précision des essais en vol ».
  - **Attention au double compte** : les vols de référence (Reports 220 et 292) ramènent la puissance du banc à l'altitude par P ∝ p/√T. À 1 500–3 000 ft, cela donne environ 1 % de plus que la loi d'altitude du modèle (R&M 462), qu'il faut retrancher de la perte.
  - Pour −x % au frein, la fraction vaut 1 − x P₀ / (P₀ + P_f). SPAD : −3 % (4 % moins 1 %), soit 0,973 ; valeur fragile, plage de 0 à 4 %.
- **Gaz** : le papillon est un orifice en série avec un moteur volumétrique. Presque fermé, l'écoulement y est sonique : puissance indiquée à peu près constante, couple en 1/ω, d'où un ralenti stable au sol et le frein moteur en piqué. En l'ouvrant, le gain décroît continûment et ne s'annule qu'à pleins gaz. Avec P_pg = P₀ + P_f, s = θ P_i,max / P_pg(ω) et s₁ sa valeur à pleins gaz : P_i = c P_pg(ω) g(s) / g(s₁), g(s) = s / (1 + s²)^½ (E:) ; θ = ralenti + (1 − ralenti) × manette (E:). Pleins gaz redonne exactement les courbes. (Une première loi en min() laissait une zone morte de 22 % de la course : revue de l'aérodynamicien du 2026-10-22.)
- Hors des courbes (E:) : couple constant sous le premier point, puissance constante au-delà du dernier. Sous le régime de calage, le moteur cesse d'allumer (il repart si l'hélice le fait mouliner assez vite), avec une rampe C¹ jusqu'au double ; le frottement s'annule linéairement à l'arrêt.
- **Inertie du rotor** : hélice en bois de 2,4 à 2,5 m, 20 à 25 kg, environ m R² / 5 ≈ 6 kg·m², plus 0,3 à 0,6 kg·m² de vilebrequin (C:/E:).
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

- Trim (`tools/fdm_lab/src/trim.rs`) : huit inconnues parmi (α, β, profondeur, ailerons, direction, gaz, vitesse, pente, régime, déflexion retardée) pour un vol rectiligne stabilisé, ailes horizontales ; Levenberg-Marquardt avec plusieurs points de départ. Palier à vitesse donnée : gaz inconnus ; vitesse maximale : pleins gaz, vitesse inconnue.
- Performances (`performance.rs`) : meilleure montée par section dorée sur la vitesse, temps de montée par la méthode des trapèzes, plafond par pas puis bissection (ou extrapolation des derniers taux, comme à l'époque), décrochage par balayage de vitesse moteur au ralenti (butée de profondeur signalée).
- Jacobien numérique de l'état autour du trim (différences centrées) → valeurs propres complexes (nalgebra) et vecteurs propres → pulsation, amortissement, temps de division ou de doublement et état dominant de chaque mode. Marge statique « régime et déflexion en équilibre » par complément de Schur : K = I_yy A_qw / (m A_ww c̄), avec le régime et la déflexion retardée éliminés ; N_v et L_v pour les stabilités de route et de dièdre.
- Robustesse (`robustness.rs`) : vitesse nulle, glissade sur la queue, entrée en vrille et sortie, vol dos, piqué maximal ; la simulation doit rester finie et bornée, la vrille établie est détectée et rapportée. Contrôle de convergence en pas de temps (créneau de profondeur, 120 Hz × 2 sous-pas contre le double) : écart < 1 %.
- Essais qualitatifs (`qualitative.rs`) : boucle, décrochage moteur réduit, plané, piqué tenu puis ressource, pilotés par un pilote simple (facteur de charge visé sous une limite d'incidence, gains réduits comme la pression dynamique au-delà de la vitesse d'entrée, ailes dans le plan, dérapage nul). Ils rapportent ce que fait l'avion à côté de l'attendu des témoignages, sans bloquer la validation (sauf valeurs non finies). Leurs chiffres dépendent en partie du pilote (limite d'incidence, vitesse de décélération) : le rapport donne ses réglages à côté des résultats.
- Les dérivées de commande servent aussi à calculer automatiquement les gains de l'instructeur virtuel et de l'IA (skill `realisme`).

## Conditions d'essai non standard (`fdm_lab montee`)

Les chiffres d'époque ne donnent presque jamais les conditions de l'essai. `fdm_lab montee <id> --delta-t-k=-10,0,10,20 --airfield-m 180` encadre les temps de montée et le plafond :
- **jour non standard** : pression de l'atmosphère type, température décalée de ΔT ; les altitudes sont des **altitudes-pression**, celles d'un altimètre. Une vitesse ascensionnelle géométrique w devient w T_type/T en altitude-pression (hydrostatique) : par jour chaud, il faut monter plus haut pour lire la même altitude ;
- **départ d'un terrain** en altitude, altimètre calé sur la mer ou à zéro au sol ;
- le décollage n'est pas compté : on le chiffre à part, en attendant le modèle de sol du M2.

Le moteur suit la loi d'altitude ci-dessus, avec son exposant de température. Avec m = 0, un jour chaud donne un peu **plus** de régime : l'air léger freine moins l'hélice. C'est la conséquence de la source, pas un défaut. Le choix de m change nettement l'enveloppe : il se sourcera, et ne se réglera jamais sur le résultat cherché.

## Bibliographie de départ (à compléter dans les fiches)

- McCormick, *Aerodynamics, Aeronautics, and Flight Mechanics*
- Stevens et Lewis, *Aircraft Control and Simulation*
- Etkin, *Dynamics of Flight: Stability and Control*
- Hoerner, *Fluid-Dynamic Drag* (câbles, mâts, radiateurs)
- NASA Technical Reports Server : rapports NACA, dans le domaine public
