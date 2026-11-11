# Plan — M3, atmosphère : brume, auréole du soleil et liseré des nuages

- Statut : **validé** par le joueur le 2026-11-11 (« Oui, recommandées ») : brume modérée, pic étroit ajouté à la phase des nuages, signalement à Bevy. Le signalement est sans objet : l'inversion est déjà corrigée en amont (bevyengine/bevy#26143, fusionné le 2026-10-26, après la 0.20.1).
- Demande du joueur (2026-11-09) : « L'atmosphère », après la clôture de la passe « part de ciel » des nuages (`docs/plans/m3-beaute.md`, incrément 1).
- Le chantier regroupe deux réserves du directeur artistique, qui jouent dans les mêmes degrés autour du soleil et se règlent ensemble :
  - l'auréole du soleil, presque absente ;
  - le liseré des nuages à contre-jour, trop doux.
- **Règles** : skill `rendu-visuel` et `nuages-volumetriques.md`. Captures avant/après regardées, revue du directeur artistique. Tout chiffre physique ou historique a une source ou l'étiquette `ESTIMÉ`.

## Constat (mesures du 2026-11-09)

1. **Bevy 0.20 inverse la diffusion et l'absorption des aérosols** dans `ScatteringMedium::earth`.
   - Bevy 0.16 avait `mie_scattering` 3,996e-6 m⁻¹ et `mie_absorption` 0,444e-6 m⁻¹ (valeurs de Hillaire 2020).
   - Le milieu de Bevy 0.20 donne `absorption` 3,996e-6 et `scattering` 0,444e-6, sans commentaire qui l'explique.
   - Nos aérosols absorbent donc 90 % de la lumière qu'ils interceptent, au lieu d'en diffuser 90 %.
2. **Corriger l'inversion seule change peu.** Sur `cumulus_contre_jour`, le ciel à 1° du soleil passe de 0,495 à 0,541 (luminance linéaire de l'image). Ces valeurs décrivent un air très pur : environ 200 km de visibilité au sol (loi de Koschmieder, 3,912 / extinction).
3. **Plus d'aérosols font l'auréole, mais aussi la brume** (prototype, densité des aérosols multipliée) :
   - ×10, visibilité au sol d'environ 65 km : auréole nette, ciel plus pâle mais encore bleu, plaine légèrement voilée ;
   - ×25, environ 30 km : ciel blanchi, plaine très voilée, et un trait clair à l'horizon. La table de perspective aérienne de Bevy s'arrête à 32 km (`aerial_view_lut_max_distance`), alors que le sol s'étend à 100 km : au-delà, le sol n'est plus voilé davantage.
4. **La fonction de phase des aérosols de Bevy est trop étalée** (Henyey-Greenstein, g = 0,8).
   - Même à ×25, le ciel n'est que 1,27 fois plus clair à 1° du soleil qu'à 10°.
   - Une vraie auréole décroît bien plus vite. Le profil cible reste à sourcer (mesures publiées de l'auréole solaire).
   - La table de phase de Bevy (256 valeurs, resserrées vers l'avant) résout environ 0,6° près du soleil : elle suffit.
5. **Le liseré des nuages est doux** : sur `cumulus_lisere`, le bord ne dépasse le corps juste en dessous que de 15 % (0,84 contre 0,73).
   - Notre phase actuelle : deux lobes de Henyey-Greenstein, g = 0,8 et −0,3, poids 0,7 et 0,3.
   - **Jendersie et d'Eon** (« An Approximate Mie Scattering Function for Fog and Cloud Rendering », SIGGRAPH 2023 Talks, doi:10.1145/3587421.3595409) approchent la phase de Mie des gouttelettes par un mélange de Henyey-Greenstein (le pic) et de Draine (le corps). Ses quatre paramètres sont donnés en fonction du diamètre d des gouttelettes, pour 5 < d < 50 µm (équations 4 à 7).
   - À d = 20 µm, le pic a g ≈ 0,995 et porte la moitié de l'énergie, sur environ 0,2°, moins que le disque du soleil (0,53°).
   - Remplacer toute notre phase par ce mélange diviserait la diffusion simple vers l'arrière par 6 environ, et celle à 90° par 20 environ : les dômes de `cumulus_midi`, soleil dans le dos, s'assombriraient, et il faudrait recaler la diffusion multiple.

## Étapes

1. **Un milieu maison** (`crates/game/src/world/atmosphere.rs`) : une seule fonction, appelée par la plaine et le studio, qui construit le milieu avec les aérosols dans le bon sens.
   - Test : l'albédo de diffusion simple des aérosols vaut environ 0,9.
   - Signaler l'inversion à Bevy (issue sur GitHub) : seulement avec ton accord (décision 3).
2. **La brume de l'été 1917.**
   - Régler la densité des aérosols sur une visibilité au sol, valeur `ESTIMÉ`, à sourcer avec l'historien : récits de pilotes (brume de chaleur, fumées du front), ordres de grandeur physiques.
   - Rendre la visibilité réglable par scène (champ de la météo de la scène), avec un défaut de beau temps.
   - Étendre la perspective aérienne à la portée du sol (`aerial_view_lut_max_distance` à 100 km), pour effacer le trait de l'horizon, et en mesurer le coût.
3. **L'auréole du soleil.**
   - Donner aux aérosols une phase plus pointue : un pic étroit plus le corps actuel, construite avec `PhaseFunction::from_curve` et normalisée (test d'intégrale).
   - La caler sur un profil d'auréole publié, mesuré de 1 à 20° du soleil sur `cumulus_contre_jour` et `cumulus_lisere`.
   - Vérifier que la table du ciel de Bevy (400 × 200, environ 0,9° par texel en azimut) ne l'émousse pas.
4. **Le lobe avant des nuages**, selon la décision 2 :
   - même fonction en Rust (`escadrille_weather::scattering`) et en WGSL (`march.wesl`) ;
   - les tests d'énergie (théorie à deux flux) doivent passer ;
   - `cumulus_midi` ne doit pas perdre plus de quelques pour cent ;
   - sur `cumulus_lisere`, le bord près du soleil doit nettement dépasser le corps (le directeur artistique veut qu'il éblouisse).
5. **Revue.**
   - La brume touche toutes les scènes d'extérieur : captures avant/après de `cumulus_*`, `traversee_nuage`, `vol-essai-*`, `terrain-*` et `avion-studio`.
   - Revue du directeur artistique.
   - Mise à jour de la fiche des nuages et du skill `rendu-visuel`.

**Coût GPU attendu** : presque nul. Les tables du milieu se recalculent à sa création, et la phase des nuages ajoute un lobe par échantillon. Seule l'extension de la perspective aérienne est à mesurer.

## Décisions demandées au joueur

1. **Brume par défaut** :
   - **(A, recommandé)** une brume d'été modérée : visibilité au sol de 40 à 60 km (`ESTIMÉ`, à sourcer), entre les variantes ×10 et ×25 du prototype. Auréole nette, ciel encore bleu, réglable par scène ;
   - (B) l'air pur de Hillaire (la correction seule, environ 200 km) : presque pas d'auréole ;
   - (C) une brume forte, environ 30 km : ciel blanchi, plaine très voilée.
2. **Phase des nuages** :
   - **(A, recommandé)** ajouter un pic étroit à la phase actuelle. Il est tiré de Jendersie et d'Eon pour des gouttelettes de cumulus (diamètre à sourcer, de l'ordre de 10 à 20 µm) et élargi par le disque du soleil. Son poids se règle par balayage contre les tests d'énergie et les captures. Rapide, et le reste du nuage ne bouge presque pas ;
   - (B) le mélange complet de Henyey-Greenstein et de Draine, avec un nouveau balayage des quatre constantes de diffusion multiple. Plus juste, mais beaucoup plus long, et tout l'aspect des nuages serait à revoir.
3. **Signaler l'inversion à Bevy** :
   - **(A, recommandé)** oui : je rédige l'issue, avec les deux extraits de code, et tu la publies (ou je la publie avec ton accord) ;
   - (B) non : la correction reste chez nous.

## Risques

- **Tout le jeu change de lumière** : la brume voile le sol, éclaircit l'horizon et réchauffe le soleil bas. Chaque scène d'extérieur est à revoir, celles qui sont déjà approuvées comprises.
- **Calage de la diffusion multiple** des nuages : avec l'option A de la décision 2, il ne devrait pas bouger. Les tests d'énergie le vérifient.
- **Sources** : la visibilité de 1917 et le profil de l'auréole demandent des sources. Faute de mieux, les valeurs sont `ESTIMÉ`, avec leur justification.

## Avancement

- **2026-11-11 : étapes 1 à 5 faites, approuvées par le directeur artistique.**
  - Milieu maison aux valeurs corrigées de Bevy ; brume de 50 km par défaut, réglable par scène (`visibility`) ; perspective aérienne à 100 km ; auréole calée sur Buie (θ^−2,3) ; pic avant des gouttelettes dans la phase des nuages. Tests verts, 15 scènes capturées.
  - Verdict : la physique est juste ; le soleil se lit enfin comme un soleil ; la brume de 50 km donne l'échelle de la plaine ; pas de régression en vol ni au studio.
- **Réserves et suites** :
  - **Exposition** (réserve principale) : le contre-jour a perdu du relief, la crête et l'auréole tombant ensemble sur l'épaule du tonemapping. Essai à −1 et −2 EV : à −1 EV, le bord sous le soleil (0,93 à 0,99) se détache du ciel de l'auréole (0,62 à 0,89) et domine le corps (0,41 à 0,47), le relief des lobes revient ; à −2 EV, l'image est trop sombre. Remède : une exposition qui suit la lumière, bornée (le directeur artistique propose ±1,5 EV), avec l'incrément 2 du M3 (« Heure du jour », exposition) ; ne pas changer de tonemapper avant.
  - **Zénith** : jugé « lavande ». Mesure : la teinte n'a bougé que de 2 à 8° vers le bleu (212 → 215° à midi, 208 → 216° au contre-jour), un peu plus sombre et moins saturée à midi. L'ozone est celui de Hillaire, inchangé ; le décalage vient de la couche d'air ramenée à 8 km. Gardé tel quel.
  - **Trait à l'horizon des scènes au sol** : pas d'artefact. Le ciel s'éclaircit régulièrement jusqu'à l'horizon ; depuis 2 m, tout le sol au-delà de 3 km tient en quelques pixels, l'herbe proche restant presque intacte.
  - **Bosse grise à l'horizon de `cumulus_sous_la_base`** : le disque de l'hélice, vu de l'arrière au-dessus de l'aile, qui se détache maintenant sur l'horizon voilé (réserve connue de l'hélice un peu pâle).
  - **Brume selon la mission** (20 à 50 km), fumée du front en couche locale, albédo des aérosols (la fumée absorbe) : à arrêter sur sources, avec l'historien.
- **2026-11-11, brume de l'historien** (choix du joueur : « Suivre l'historien ») : 30 km par défaut, dans la couche de mélange (sommet à la base des cumulus, ou 1 500 m), air libre au-dessus ; atmosphère en marche de rayons par pixel, pour le trait cyan de l'horizon (0,5 ms). Revue du directeur artistique à suivre.
