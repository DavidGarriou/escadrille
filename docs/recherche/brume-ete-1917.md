# Brume et visibilité dans le ciel des Flandres, été et automne 1917

- **Objet** : régler la brume par défaut du rendu, c'est-à-dire la densité des aérosols de l'atmosphère (`docs/plans/m3-atmosphere.md`, étape 2 et décision 1).
- **Rédaction** : subagent `historien`. Commencée le 2026-11-09, interrompue par une coupure de session, reprise et achevée le 2026-11-11.
- **Statut** : première passe complète. Aucune mesure chiffrée de la visibilité en Flandre en 1917 n'a été trouvée. La plage repose sur des témoignages qualitatifs concordants et sur des séries de mesures d'autres époques et d'autres lieux, d'où la confiance **C**.
- **En bref** :
  - par beau temps d'été, la visibilité au sol en Flandre en 1917 était probablement de **15 à 30 km** au sens de la portée optique météorologique (MOR, seuil de 5 %). Elle montait à **40-60 km** les jours très clairs et tombait sous **10 km** les jours de brume. Confiance **C** ;
  - **les 40 à 60 km du plan sont trop optimistes pour un jour ordinaire.** Ils décrivent un jour clair, pas le beau temps typique de 1917 ;
  - **défaut recommandé : 30 km dans la convention du plan (3,912/σ)**, soit σ ≈ 1,3·10⁻⁴ m⁻¹ au sol à 550 nm, ou environ 23 km en MOR. Valeur `ESTIMÉ`, réglable de 25 à 40 km ;
  - les témoignages s'accordent sur trois traits :
    - la brume est **basse**, avec un ciel bleu au-dessus des avions de chasse ;
    - le **matin est brumeux** en fin d'été et en automne, et la brume se lève vers 8 à 10 h ;
    - **la fumée de la bataille** dérive au-dessus du front les jours d'offensive.
- **Règles** : skill `recherche-historique`. Les sources sont numérotées **M1, M2…** (§ 9). Les citations font moins de 15 mots et sont données dans leur langue d'origine ; le reste est paraphrasé.
- **Marques de lecture** :
  - [LU] : texte lu en entier ou sur le passage cité ;
  - [LU-WEB] : texte lu par l'outil web, qui le résume fidèlement page par page ; les citations sont reprises de l'OCR ;
  - [RÉSUMÉ] : vu seulement dans un résumé de moteur de recherche ;
  - [PISTE] : titre ou mention seulement.

## 1. Ce que « visibilité » veut dire ici

Trois grandeurs différentes circulent. Il faut les distinguer avant de comparer les chiffres.

| Grandeur | Définition | Relation avec l'extinction σ | Source | Confiance |
|---|---|---|---|---|
| Visibilité de **Koschmieder** (convention du plan M3) | distance où le contraste d'un objet noir sur le ciel de l'horizon tombe à 2 % | V = 3,912 / σ | M9 (éq. 1, p. 2089) ; M10 § 9.1.3 (seuil de 0,02 proposé par Koschmieder en 1924) | A |
| **Portée optique météorologique** (MOR, norme OMM) | longueur de trajet qui réduit à 5 % un flux lumineux collimaté | V = −ln 0,05 / σ ≈ 3,0 / σ | M10 § 9.1.1 et 9.1.3 (seuil de 0,05 adopté pour l'aéronautique) ; C: −ln 0,05 = 2,996 | A |
| Visibilité **estimée par un observateur** (stations météorologiques humaines, et tous les témoignages de 1917) | plus grande distance où l'on reconnaît un objet sombre de taille convenable sur le ciel de l'horizon | proche de la MOR. C'est une **borne basse**, car elle dépend des repères disponibles à distance connue | M10 § 9.1.1 ; M9 § 2 (repères éloignés et précision réduite au-delà de 10 km) | B |

**Conséquence pour le plan.** Les « 40-60 km » du plan sont calculés avec 3,912/σ. Ils équivalent à **31-46 km en MOR**, la grandeur que donnent les séries d'observations (§ 4). Pour la même brume, Koschmieder donne donc un chiffre **1,3 fois plus grand** que la MOR. Le tableau du § 5.2 donne les deux.

**Visibilité au sol et vue depuis l'altitude.** Les pilotes ne décrivent presque jamais la visibilité au sol. Ils décrivent ce qu'ils voient d'en haut, souvent **au-dessus** de la couche de brume (§ 2.2, trait 2). Un repère vu à 25 km depuis 4 000 m ne prouve donc pas une visibilité au sol de 25 km.

## 2. Témoignages de pilotes et d'observateurs

### 2.1 Témoignages relevés

Chaque témoignage est situé : auteur, camp, rôle, date d'écriture, date décrite, secteur, altitude. Les citations gardent leur langue d'origine ; les paraphrases sont en français.

| N° | Auteur, camp, rôle | Écrit quand | Date et secteur décrits | Ce qu'il dit (citation courte, puis paraphrase) | Source | Biais possibles |
|---|---|---|---|---|---|---|
| T1 | **Charles J. Biddle**, Américain engagé dans l'aviation française, pilote de la SPA 73 (GC 12, le groupe de la SPA 3) | lettres écrites sur le moment, publiées en 1919 | 10 août 1917, Bergues, patrouille à l'aube vers 2 000 m | « Over it all the drifting smoke of the battle » ; les éclairs des canons se voient même en plein jour | M1 p. 49 [LU-WEB] | lettres à sa famille, sans enjeu de propagande ; pilote encore novice au front |
| T1 | idem | idem | 12 août 1917, à 500 m au-dessus des tranchées | « a beautiful clear day » : on distingue chaque soldat à son poste | M1 p. 51 | idem |
| T1 | idem | idem | 13 août 1917, vol du soir | « the smoke, mist and drifting rain squalls » : fumée, brume et grains, en même temps | M1 p. 53 | idem |
| T1 | idem | idem | 3 octobre 1917, Bergues ; protection d'un avion de réglage d'artillerie, plus près de la mer que d'habitude, vers 16 000 ft (4 900 m) | « Ostend is plainly visible » quand on vole très haut ; Ostende semble si proche qu'on y laisserait tomber une pierre | M1 p. 79 | distance à Ostende non donnée : probablement 15 à 30 km |
| T1 | idem | idem | 16 octobre 1917, vers 16 000 ft | « a rather heavy mist » à cette altitude, comme une brume basse sur les champs ; un biplace allemand en sort à 200 yards (180 m) | M1 p. 84-87 | cas isolé : une couche de brume en altitude |
| T1 | idem | idem | 24 décembre 1917, Dunkerque | il est soulagé de quitter les brouillards et l'humidité venus de la mer | M1 p. 135 | réputation du secteur (§ 2.3) |
| T2 | **James McCudden**, Britannique, 56 Squadron sur S.E.5, as | mémoires écrits avant sa mort, en juillet 1918 ; publiés en [1919] selon la notice | 13 juillet 1917, Lens–Douai, vers 12 000 ft | les balles traçantes gâchent « the lovely summer blue sky » : le ciel est bleu à l'altitude des chasseurs | M2 p. 196 [LU-WEB] | souvenirs proches des faits, d'après ses carnets ; récit d'as, centré sur le combat |
| T2 | idem | idem | 14 juillet 1917, Dunkerque–Dixmude, 11 000 ft | « The visibility was poor », avec de gros nuages cotonneux de 2 000 à 10 000 ft | M2 p. 197 [LU-WEB, première lecture seulement] | idem |
| T2 | idem | idem | vers le 19 août 1917, Polygon Wood, au crépuscule | il ne peut suivre sa victime jusqu'au sol : « it was rather hazy » | M2 p. 220 | idem |
| T2 | idem | idem | 4 septembre 1917, retour de Courtrai et Lille, vers 16 000 ft | « As the visibility was good », il garde son altitude pour guetter les biplaces | M2 p. 226 | idem |
| T2 | idem | idem | 23 septembre 1917, au soir, saillant d'Ypres ; montée vers 6 000 ft (1 800 m), plafond gris à 9 000 ft | « the visibility was poor for observation » ; plus bas, les R.E.8 de l'artillerie travaillent « in the haze » ; à l'est, des grappes de points noirs (avions allemands) | M2 p. 240 | soir du combat contre Werner Voss : souvenir très travaillé |
| T2 | idem | idem | vers le 30 septembre 1917, patrouille du petit matin | « the ground was mostly obscured by mist » | M2 p. 248 | idem |
| T2 | idem | idem | 17 octobre 1917, départ vers 10 h, Ypres à 14 000 ft | « the visibility was very good » | M2 p. 258 | idem |
| T2 | idem | idem | 21 octobre 1917, Armentières, 17 500 ft | un Rumpler, point minuscule sur un ciel « herring-bone » (en arêtes de poisson, donc des cirrocumulus ou des altocumulus) | M2 p. 263 | idem |
| T3 | **H. A. Jones**, histoire officielle britannique de la guerre aérienne | 1934, d'après les archives du RFC | 7-9 juillet 1917, Lombartzyde (côte) | visibilité mauvaise, brume au sol, nuages vers 900 ft | M3 p. 148 [LU-WEB] | source secondaire officielle ; elle signale surtout les jours où le temps gêne |
| T3 | idem | idem | 10 juillet 1917 | les écrans de fumée allemands gênent beaucoup l'observation aérienne | M3 p. 149 | idem |
| T3 | idem | idem | 16 août 1917 (Langemarck) | « A misty atmosphere and cloud patches » le matin ; le vent étale la fumée de la bataille sur l'arrière allemand | M3 p. 172-173 | idem |
| T3 | idem | idem | 20 septembre 1917 (route de Menin) | une brume humide couvre le champ de bataille à l'aube ; peu de travail aérien avant 8 h, quand elle se lève | M3 p. 183-186 | idem |
| T3 | idem | idem | 28 septembre 1917 | « a fine day » : vingt attaques de bombardement | M3 p. 195 | idem |
| T3 | idem | idem | 30 septembre-3 octobre 1917 | « The mornings … were misty » (matins des contre-attaques allemandes) | M3 p. 195 | idem |
| T3 | idem | idem | 20-22 octobre 1917 | le 20, l'attaque à très basse altitude du terrain de Rumbeke est racontée sans mention de gêne ; le 21 est « a day of fair weather » ; le 22, pluie jusqu'à l'après-midi | M3 p. 207-209 | idem |
| T3 | idem | idem | 28 octobre et début novembre 1917 | lignes dans le brouillard presque toute la journée ; puis jours secs gâchés par la brume | M3 p. 211 | idem |
| T3 | idem | idem | côte belge, 12 mai, 5 juin et septembre-octobre 1917 | à Zeebrugge, brume au sol jusqu'à 3 h 40, et visibilité depuis les navires jamais supérieure à 4 000 yards (3,7 km). Le 5 juin, les écrans de fumée allemands d'Ostende couvrent 10 à 15 milles carrés (26 à 39 km²). Les 20 et 21 octobre, ils déjouent les bombardements d'Ostende | M3 p. 84, 86, 89-90 | idem ; chiffres de la Marine |
| T4 | **Henry Bordeaux**, écrivain mobilisé, biographe de Guynemer (camp français) | 1918 ; biographie hagiographique | juillet 1917, plaine flamande | la lumière des Flandres naît des « vapeurs humides qui montent du sol ou viennent de la mer » | M4 p. 244 [LU-WEB] | ton lyrique ; écrivain, pas pilote |
| T4 | idem | idem | 11 septembre 1917, Saint-Pol-sur-Mer, au matin | « le voisinage de la mer attire ces brumes matinales », qui doivent se lever ; puis « Déjà les brumes s'élèvent » | M4 p. 265-266 | récit reconstruit après coup |
| T4 | idem | idem | fin septembre 1917, Saint-Pol, sa propre visite, par un beau soir calme | « Une brume rousse imprécise les bords de l'horizon. » | M4 p. 278 environ | **observation personnelle au sol**, la seule du corpus |
| T5 | **James Norman Hall**, Américain, escadrille N.124 (« Lafayette ») | 1918 | juin 1917, Aisne (Chemin des Dames), patrouille haute de 2 000 à 5 000 m | « I could have spanned the distance between Soissons and Laon » : les deux villes, à environ 30 km l'une de l'autre, sont visibles ensemble ; des colonnes de fumée montent d'Allemant à Craonne | M5 ch. V [LU-WEB] | hors des Flandres ; même été ; livre écrit pour le public américain |
| T5 | idem | idem | juin 1917, à l'aube | une brume épaisse s'étend sur le sol « like water » et remplit chaque creux ; seuls les clochers de Soissons et les crêtes en émergent, et les ballons flottent au-dessus | M5 ch. VI | idem |
| T6 | **lettres de pilotes du 60 Squadron** (Molesworth, Maclennan), Britanniques, publiées par A. J. L. Scott | lettres de 1917, publiées en 1920 | juin 1917, Arras, 12 000 ft | « too much ground mist » pour que la DCA les voie ; « a perfect day up top » | M6 [LU-WEB, partiel] | lettres à la famille |
| T6 | idem | idem | 9 décembre 1917, Sainte-Marie-Cappel, près de Cassel | « the visibility poor », jour couvert ; depuis la colline de Cassel, belle vue sur le front, avec ses fumées et ses ballons | M6 | hiver, hors période |
| T7 | **Manfred von Richthofen**, Allemand, chef de la Jasta 11 | 1917 (livre de propagande, relu par la censure) | mai 1917, front de Vimy et de Douai | « Nun war es an diesem Tage noch etwas dunstig » : la brume désoriente son frère Lothar après un combat | M7, chap. « Mein Bruder » [LU-WEB] | propagande ; secteur d'Artois, à 40-60 km au sud d'Ypres |
| T7 | idem | idem | 29 avril 1917, Douai | « Wunderbar klares Wetter. » | M7 | idem |
| T8 | **Ernst Udet**, Allemand, chef de la Jasta 37 | 1935 (mémoires de l'époque nazie, dix-sept ans après les faits) | fin 1917, Wynghene (Flandre-Occidentale), six semaines après sa prise de commandement (le mois n'est pas donné) | « Wenn man höher steigt, kann man hinübersehen nach Ostende und zum Meer. » Wingene est à 28 km environ d'Ostende (C: d'après les cartes) | M8, chap. 5 [LU-WEB] | souvenir tardif ; altitude non dite |
| T8 | idem | idem | 8 août 1918, Somme (hors période) | un Anglais fond sur eux « aus dem grauen Dunst » ; ailleurs, la fumée, la poussière et les mottes du barrage forment un « Vorhang » (rideau) | M8, chap. 7 | 1918, autre secteur : pour l'ambiance des jours d'offensive seulement |

**Langues et camps couverts.**
- **Anglais** : T2, T3, T6.
- **Français** : T4. Biddle et Hall, Américains de l'aviation française, écrivent en anglais.
- **Allemand** : T7, T8.

**Ce qui manque :**
- **Aucun témoignage allemand précis sur la Flandre de l'été 1917** n'a été trouvé en ligne. Pistes au § 7.
- **Italien** : recherche non faite, volontairement. Aucune unité italienne ne volait en Flandre, et les témoignages italiens sur la *foschia* décriraient le front italien, sous un autre climat.
- **Belge** : Willy Coppens, *Jours envolés* (1932), n'a pas pu être consulté (§ 7).

### 2.2 Traits retenus après croisement

Un trait n'est retenu qu'avec au moins deux témoignages indépendants.

| N° | Trait | Témoignages | Concordance | Attente vérifiable (captures de référence, § 6) | Confiance |
|---|---|---|---|---|---|
| 1 | **La visibilité change d'un jour à l'autre et se juge chaque jour** : « good », « poor », « very good » | T2 (cinq jours différents), T3 (passim), T1, T6 | forte, deux camps (aucun chiffre) | la visibilité est un paramètre de scène, pas une constante | **A** pour le fait, **C** pour les valeurs |
| 2 | **Brume basse, ciel bleu au-dessus des chasseurs.** Les avions d'artillerie travaillent dans la brume, en dessous, tandis que les chasseurs patrouillent dans l'air clair au-dessus. À l'aube, crêtes, clochers et ballons émergent de la brume | T2 (13 juillet et 23 septembre), T5 (aube ; air clair au-dessus des nuages), T6 (« perfect day up top » au-dessus de la brume au sol) | bonne, mais tous du côté allié. **Contre-exemple** : T1, brume à 4 900 m le 16 octobre | depuis 4 000 m par beau temps, le ciel au-dessus est bleu franc ; la brume est sous l'avion | **B** |
| 3 | **Matins brumeux en fin d'été et en automne**, avec une brume qui se lève dans la matinée (vers 8 h le 20 septembre) | T3 (20 septembre ; 30 septembre-3 octobre ; novembre), T2 (30 septembre), T4 (11 septembre), T5 (juin, Aisne), T6 (juin, Arras) | forte : trois sources indépendantes, deux camps (britannique et français) | brume au sol à l'aube dans les creux, sur l'eau et les zones inondées, puis dissipation | **B** |
| 4 | **La fumée de la bataille** dérive au-dessus du front et, poussée par le vent, couvre l'arrière ennemi | T1 (10 et 13 août), T3 (16 août ; écrans de fumée du 10 juillet), T5 (Aisne), T8 (1918) | forte, trois camps | voile local le long du front, déporté sous le vent | **B** |
| 5 | **Par beau temps, des repères à 15-30 km sont visibles depuis l'altitude** | T1 (Ostende depuis 4 900 m), T8 (Ostende et la mer depuis Wingene), T5 (Soissons et Laon) | bonne, deux camps | depuis 3 000-5 000 m, une ville à 20-30 km se reconnaît par beau temps | **B**, c'est une borne basse |
| 6 | **Brumes de mer** à la côte (Dunkerque, Saint-Pol, Zeebrugge, Lombartzyde) | T1 (décembre), T4 (11 septembre), T3 (12 mai, 7-9 juillet) | bonne, deux camps | sur les scènes côtières, brume plus dense le matin | **B** |
| 7 | Brume roussie à l'horizon au soir d'un beau jour | T4 seul | à confirmer | horizon chaud au couchant (cohérent avec la diffusion) | **C** |

### 2.3 Réputation du ciel des Flandres

Ce qui relève de l'impression, à distinguer des faits :
- **« Pays bas, humide, brumeux »** : Biddle compare la Somme, « not nearly so low and wet », aux Flandres, et se réjouit d'en quitter les brumes de mer (M1 p. 116 et 135). Udet note qu'à Wynghene (fin 1917, saison non précisée), les nuages traînent parfois si bas que le vol est annulé même les jours secs (M8, chap. 5).
- Cette réputation est **confirmée** pour l'automne et l'hiver (T3, octobre-novembre). Elle est **exagérée** pour l'été : les beaux jours d'août à octobre existent (12 août, 4 et 28 septembre, 17 et 21 octobre).
- Matériau utilisable au mess et dans le carnet de vol :
  - la brume qui monte de la mer ;
  - la pluie d'août 1917 ;
  - le pilote qui perd le sol de vue sous un plafond et rentre au compas (Biddle, 22 septembre, M1 p. 77) ;
  - celui que la brume désoriente après un combat (Lothar von Richthofen, M7).

## 3. Sources météorologiques et militaires d'époque

Aucune valeur chiffrée de visibilité pour la Flandre en 1917 n'a été lue. Voici ce qui existe et où chercher.

| Source | Contenu connu | Statut | Réf. |
|---|---|---|---|
| **Section « Meteor »** des Royal Engineers, à l'état-major britannique (GQG, Montreuil) | créée en France en juin 1915 sous la direction d'Ernest Gold ; prévisions pour l'artillerie, les gaz et le RFC. Elle annonce la fin du beau temps début octobre 1917. Le journal de Gold existe | **aucune observation de visibilité trouvée** | M16 [RÉSUMÉ] |
| **Daily Weather Report** du Met Office, 1917 | environ 130 stations en Europe, dont 75 dans les îles Britanniques [RÉSUMÉ]. La visibilité figure parmi les éléments publiés dans les volumes de 1921-1930 ; pour 1917, **non vérifié**. Les lettres de temps de Beaufort, dont *z* (haze) et *m* (mist), sont probablement présentes (à vérifier) | images numérisées par la National Meteorological Library | M22 [LU-WEB, page de présentation] |
| *The Seaman's Handbook of Meteorology*, Met Office, 1918 | contient probablement l'échelle de visibilité et les définitions de *haze*, *mist* et *fog* en usage en 1918 | PDF de plus de 10 Mo, refusé par l'outil | M26 [PISTE] |
| Histoire officielle britannique (Jones, vol. IV) | qualitatif, au jour près (§ 2.1, T3) | lue | M3 |
| Dépêche de Haig rapportée par la presse (*Dominion*, 24 octobre 1917) | le beau temps « has improved the visibility for the gunners » (dimanche 21 octobre ?) | réécriture de presse | M23 [RÉSUMÉ] |
| **Service météorologique militaire français**, créé en octobre ou novembre 1916 et rattaché à la Direction de l'aéronautique ; plus de 2 000 hommes en 1918 | bulletins aux armées, probablement au SHD | **cote non trouvée** | M24 [RÉSUMÉ] |
| SHD, *Archives de l'aéronautique militaire de la Première Guerre mondiale. Répertoire numérique détaillé de la série A*, 2008 | l'inventaire qui donne les cotes des comptes rendus, des JMO et des bulletins | PDF téléchargé, mais **chiffré** : seules les pages de titre ont été lues | M21 [LU, pages 1 à 11] |
| Kommandeur der Flieger der **4. Armee** (l'armée allemande de Flandre) : « Luftbilder aus der Abwehrschlacht in Flandern, 31.07.-30.09.1917 », Heft B | photos aériennes allemandes du saillant ; le voile de brume s'y verrait | BayHStA, KFT (WK) 2014 ; non numérisé | M19 [LU-WEB, notice] |
| Exemple de dossier allemand de **Feldwetterwarte** : comptes rendus quotidiens d'aviation, de ballons, de DCA et de **temps**, novembre-décembre 1917 | montre qu'un tel dossier existe ; celui-ci concerne l'Armee-Abteilung B (Alsace), **pas la Flandre** | HStA Stuttgart, M 34 Bü 116 | M20 [LU-WEB, notice] |

## 4. Ordres de grandeur physiques

### 4.1 Air pur et brume du prototype

- Selon le plan, les valeurs de Hillaire (2020), corrigées de l'inversion de Bevy 0.20, donnent environ **200 km** au sol : un air presque sans aérosols (`m3-atmosphere.md`, constat 2).
- Nuance de vocabulaire (OMM, M15) : « haze » désigne une visibilité ≤ 5 km, « mist » ≤ 2 km et « fog » ≤ 1 km. La « brume de beau temps » des pilotes est bien plus légère que ces catégories.

### 4.2 Aujourd'hui et dans les années 1950 : huit stations britanniques (M9)

Singh, Bloss et Pope (2017) publient la visibilité à midi de 1950 à 2013, observée par des humains jusqu'à la fin des années 1990. Les valeurs ci-dessous sont **lues sur leur figure 2, à ±3 km près**, donc confiance B pour l'ordre de grandeur.

| Station (type) | Moyenne annuelle, années 1950 | Moyenne d'été, années 1950 | Moyenne d'été, années 2000-2010 |
|---|---|---|---|
| Heathrow (aéroport, Londres) | ≈ 15 km | ≈ 20 km | ≈ 30 km |
| Waddington (base de la RAF, campagne) | ≈ 13-15 km | ≈ 20-25 km | ≈ 35-40 km |
| Ringway (aéroport de Manchester) | ≈ 10 km | ≈ 15 km | ≈ 28 km |
| Nottingham (ville, à partir de 1957) | ≈ 10 km | ≈ 13 km | ≈ 30 km |
| Leuchars, Tiree (campagne côtière, Écosse) | ≈ 30 km | ≈ 35-40 km | ≈ 35-40 km |

Ce que l'article dit en plus :
- **L'été a la meilleure visibilité de l'année**, et la visibilité baisse quand l'humidité relative monte, parce que les aérosols grossissent en absorbant l'eau (§ 4.1 et 4.3).
- Les **plus basses visibilités** arrivent par vent d'est, avec l'air pollué du continent ; les plus hautes par vent d'ouest ou de nord-ouest, avec de l'air passé sur la campagne (§ 4.2.2).
- La hausse depuis 1956 vient du Clean Air Act et de la fin du chauffage au charbon (§ 4.1).
- À Oxford, la visibilité baisse de 1926 à 1944, puis remonte (Gomez et Smith, cités p. 2086).
- Les observateurs humains donnent une **borne basse**, faute de repères éloignés (§ 2).

### 4.3 Début du XXᵉ siècle

- **Europe** : d'après la durée d'ensoleillement, Wandji Nyamsi et al. (2026) reconstruisent la charge d'aérosols de six villes d'Europe centrale depuis 1900 (Bâle, Zurich, Potsdam…). Ils ne donnent aucune valeur pour la Belgique, la France ou le Royaume-Uni.
  - L'atmosphère des villes européennes **s'éclaircit de 1900 à 1959**.
  - À Potsdam, l'épaisseur optique large bande culmine vers **0,3 en 1903 et 0,25 en 1912**, deux pics attribués aux éruptions de Santa María et du Novarupta. Aucun pic n'est cité pour 1917.
  - Confiance **B** pour la tendance, **C** pour la transposition à la Flandre (M11 [LU-WEB]).
  - Ohvril et al. (2009) trouvent, en Europe de l'Est, une transparence de l'atmosphère qui baisse jusqu'au début des années 1980 (M12 [RÉSUMÉ]).
- **Charbon britannique** : 183,8 millions de tonnes (*tons*) disponibles pour la consommation intérieure de la Grande-Bretagne en 1913 (M17 [RÉSUMÉ], réponse écrite au Parlement). C'est le même ordre de grandeur que dans les années 1950 : production de 228 Mt en 1952 (piste wiki), exportations bien plus faibles. Comparaison à vérifier dans la série officielle « Coal since 1853 ». Par vent d'ouest, l'air qui arrive en Flandre a traversé le sud-est de l'Angleterre.
- **Lille, Roubaix et Tourcoing occupés** : les réquisitions et le pillage des machines arrêtent une grande partie de l'industrie. La Michelin de 1919 parle d'une industrie ruinée par un pillage systématique (M18 [LU-WEB]) ; des sources secondaires vues en résumé évoquent la fermeture des filatures et 36 % de chômeurs à Lille en 1918. En 1917, les fumées locales de l'industrie étaient donc **plus faibles qu'avant-guerre**. En revanche, le front ajoutait les siennes : camps, chemins de fer, canons, incendies. Confiance **C**.
- **Sources maritimes** : sel marin et forte humidité, à moins de 30 km de la mer (T4, T6 ; M9 pour l'effet de l'humidité).

### 4.4 Synthèse physique

1. **Combien de charbon, et d'où il vient.** L'été de 1917 en Flandre se situe dans un régime de charbon comparable à l'Angleterre des années 1950 pour l'air venu de l'ouest et du sud-est. L'air local était un peu plus propre (industrie de Lille arrêtée, campagne), mais plus humide (côte).
2. **Ce que cela donne en visibilité.** Les moyennes d'été des années 1950 sont d'environ **15 à 25 km** dans l'intérieur de l'Angleterre, et de **35-40 km** à la campagne côtière la plus propre. La Flandre de 1917 se place entre les deux, avec une moyenne d'été de l'ordre de **20-30 km en MOR**.
3. **Plage par beau temps.** Les beaux jours anticycloniques de vent d'est sont plutôt brumeux, les lendemains de front par vent de nord-ouest plutôt clairs. D'où la plage du § 5.1.

## 5. Plage retenue et valeur recommandée

### 5.1 Plage de visibilité au sol, été 1917, Flandre

| Situation | Visibilité au sol (MOR, 3,0/σ) | Équivalent dans la convention du plan (3,912/σ) | σ total au sol, 550 nm | Confiance | Appui |
|---|---|---|---|---|---|
| **Beau temps d'été typique**, après-midi | **15-30 km** | 20-39 km | 1,0 à 2,0·10⁻⁴ m⁻¹ | **C** | § 4.2-4.4 ; T1, T2 (« good ») |
| Jour **très clair** (air polaire après un front, vent de nord-ouest) | 40-60 km | 52-78 km | 0,5 à 0,75·10⁻⁴ m⁻¹ | **C** | M9 (vents de NO) ; T2 (17 octobre, « very good ») ; T5 |
| Beau temps **brumeux** (anticyclone, vent d'est, chaleur) | 8-15 km | 10-20 km | 2,0 à 3,75·10⁻⁴ m⁻¹ | **C** | M9 (vents d'est) ; T2 (« rather hazy », « poor ») |
| **Matin** : brume au sol dans la couche basse | 0,5-5 km dans la couche, quelques dizaines de mètres d'épaisseur ; clair au-dessus | — | — | **B** pour le phénomène, **D** pour les chiffres | trait 3 |

Toutes les valeurs du tableau sont des déductions ; aucune n'est mesurée.

### 5.2 Valeur recommandée pour la brume par défaut du jeu

**Défaut : visibilité au sol de 30 km dans la convention du plan (3,912/σ)**, soit **σ ≈ 1,3·10⁻⁴ m⁻¹** au sol à 550 nm, Rayleigh compris, ou environ **23 km en MOR**. **`ESTIMÉ`**, réglable de **25 à 40 km** par scène.

Justification :
1. **Où se place la valeur.** 23 km en MOR tombe au milieu de la plage typique de 15 à 30 km (§ 5.1), qui repose sur les moyennes d'été de l'ère du charbon (M9) et sur le caractère maritime et rural de la Flandre (§ 4.3).
2. **Ce que disent les témoignages.** Ils montrent une visibilité souvent jugée médiocre pour l'observation du sol, mais assez bonne pour voir Ostende ou deux villes à 30 km l'une de l'autre depuis l'altitude (traits 1 et 5). Cela va avec une brume modérée et basse, pas avec un air de 60 km.
3. **Ce que la valeur demande au rendu.** Elle correspond à environ 26 fois la densité d'aérosols de Hillaire. Le calcul (C:) prend l'extinction Rayleigh à 550 nm de Hillaire (13,558·10⁻⁶ m⁻¹), à vérifier dans les sources de Bevy, et l'extinction de Mie de 4,44·10⁻⁶ m⁻¹ du plan : (1,3·10⁻⁴ − 0,136·10⁻⁴) / 4,44·10⁻⁶ ≈ 26. C'est presque la variante ×25 du prototype.
4. **Épaisseur optique implicite.** Avec une hauteur d'échelle des aérosols de 1,2 km, cette densité donne une épaisseur optique de l'ordre de **0,14** (C: 1,16·10⁻⁴ m⁻¹ × 1 200 m). Ce n'est pas une atmosphère extrême. Le chiffre est à comparer aux mesures AERONET de Lille (§ 7).
5. **Si le ciel blanchit trop à 30 km, ne pas augmenter la visibilité au sol** : resserrer plutôt le profil vertical des aérosols. Les témoignages placent la brume **sous** les chasseurs, avec un ciel bleu franc au-dessus (trait 2). C'est le profil vertical, pas la visibilité au sol, qui doit garder le ciel bleu. L'épaisseur de la couche n'est pas sourcée : 1 à 2 km l'après-midi, `ESTIMÉ`. McCudden monte à 1 800 m au-dessus des R.E.8 pris dans la brume le 23 septembre, mais il ne dit pas où finit la brume.

**Verdict franc sur les 40-60 km du plan.** Dans la convention du plan, 40 km équivalent à 31 km en MOR : c'est la limite haute d'un beau jour typique. 60 km (46 km en MOR) décrivent un **jour exceptionnellement clair** en 1917. Les sources pointent donc plus bas que la plage du plan pour le défaut, mais l'option A (brume modérée, réglable par scène) reste la bonne : il suffit de placer le curseur à 30 km et de réserver 50-60 km à un préréglage « jour clair ».

### 5.3 Ce que la visibilité implique à distance (C: Koschmieder)

Dans la convention du plan, le contraste d'un objet sombre tombe à 2 % à V, et à 0,04 % à 2V. Avec V = 30 km, le relief au-delà de 60 km est entièrement noyé dans la brume. Le prolongement de la perspective aérienne à 100 km prévu par le plan suffit donc largement.

## 6. Variations utiles au jeu

| Moment ou situation | Ce qu'on voit (sources) | Proposition de réglage (`ESTIMÉ` sauf mention) |
|---|---|---|
| **Aube jusqu'à 8-10 h**, fin d'été et automne | brume au sol dans les creux, sur les canaux, la plaine inondée de l'Yser et les cratères pleins d'eau ; clochers, crêtes et ballons émergent (T5) ; elle se lève vers 8 h le 20 septembre (T3) ; matins brumeux du 30 septembre au 3 octobre (T2, T3) ; plus fréquente près de la mer (T4, T6) | couche basse dense, de quelques dizaines de mètres, qui s'amincit puis disparaît en fin de matinée ; au-dessus, la visibilité du jour. Le brouillard de nuit existe aussi (T3, 25-26 septembre et 1er octobre) |
| **Milieu de journée et après-midi** | brume de beau temps mélangée sur une couche épaisse ; ciel bleu à l'altitude des chasseurs (T2, 13 juillet) ; avions d'artillerie bas « in the haze » (T2, 23 septembre) | défaut : 30 km (§ 5.2) ; couche d'aérosols de 1 à 2 km |
| **Soir** | brume roussie à l'horizon par temps calme (T4) ; « rather hazy » au crépuscule (T2) ; l'humidité remonte, donc la visibilité baisse (M9) | légère baisse de la visibilité (−20 % par exemple) ; la teinte vient de la diffusion |
| **Jour de grande offensive** (31 juillet, 16 août, 20 et 26 septembre, 4, 9, 12, 22 et 26 octobre) | fumée de la bataille au-dessus de tout (T1, 10 août) ; éclairs des canons visibles de jour depuis 2 000 m (T1) ; brume et fumée gênent l'observation, et le vent étale la fumée sur l'arrière ennemi (T3, 16 août). Ces jours-là, le temps est souvent bas ou pluvieux (T3, 31 juillet, 4 et 12 octobre) | une bande de fumée de quelques kilomètres le long du front, déportée sous le vent (vers l'est par vent d'ouest), plus dense en bas. Largeur, hauteur et densité : **non sourcées (D)**. Ordre de grandeur hors sujet : la fumée d'une ville en feu monte à 2 000 m environ, selon Richthofen (Russie, 1915, M7) |
| **Côte** (Nieuport, Ostende, Dunkerque, Saint-Pol) | brumes de mer matinales ; écrans de fumée allemands à Ostende, sur 26 à 39 km² le 5 juin, et encore les 20 et 21 octobre 1917 (T3) | brume plus dense près de la mer le matin ; écrans de fumée possibles en mission côtière |
| **Vent** | vent d'est : air continental, brume ; vent d'ouest ou de nord-ouest : air de mer, plus clair (M9, transposé, C) | lier la visibilité de la scène à la direction du vent dans la météo de mission |
| **Date du jeu, 20 octobre 1917** | 16 octobre : brume à 4 900 m, pluie l'après-midi (T1) ; 17 octobre : « visibility very good » (T2) ; 20 octobre : attaque à très basse altitude de Rumbeke sans gêne mentionnée (T3) ; 21 octobre : beau temps et ciel « en arêtes de poisson » à 17 500 ft (T2, T3) ; 22 octobre : pluie (T3) | journée d'automne volable entre deux périodes de pluie : brume matinale probable, puis 25-35 km dans la convention du plan l'après-midi. **Aucune donnée de visibilité pour ce jour** |

**Attentes vérifiables pour les captures de référence.** Elles sont tirées des traits retenus au § 2.2 et sont qualitatives.
1. Depuis 4 000-5 000 m par beau temps, le ciel au-dessus de l'avion est bleu franc et le sol est voilé, sans être effacé. Une ville à 20-30 km se reconnaît (traits 2 et 5).
2. Un avion à 300-1 000 m vu d'en haut est dans la brume : moins contrasté que le même avion à l'altitude de l'observateur (trait 2).
3. À l'aube (scène `aube_terrain`), la brume remplit les creux, et les arbres, les clochers et les hangars en sortent (trait 3).
4. Les jours d'offensive, une traînée de fumée se lit au-dessus de la ligne de front, pas sur tout le paysage (trait 4).

## 7. Ce qui manque, et documents à récupérer

### 7.1 Pour la session principale (téléchargements, sans demande extérieure)

1. **FranceArchives** : les pages de recherche s'affichent vides pour l'outil web. Recherches à lancer avec curl :
   - « service météorologique militaire 1917 » ;
   - « bulletin météorologique armée 1917 » ;
   - « aéronautique 1re armée Flandres 1917 » ;
   - « GC 12 journal de marche ».

   Adresse : https://francearchives.gouv.fr/fr/search?q=…
2. **Répertoire de la série A du SHD** (M21), https://www.horizon14-18.eu/wa_files/archives_20de_20l_27a_C3_A9ronautique_20militaire.pdf : PDF chiffré. Le déchiffrer (par exemple `qpdf --decrypt`), en extraire le texte, puis chercher :
   - « météo » ;
   - « Flandre » ;
   - « 1re armée » ;
   - « GC 12 » ;
   - « comptes rendus » ;
   - « JMO ».

   Relever les cotes des bulletins météorologiques et des comptes rendus quotidiens de l'aéronautique de la 1re Armée, de juillet à novembre 1917.
3. **Daily Weather Report de 1917** : archive numérique du Met Office (NMLA), https://digital.nmla.metoffice.gov.uk/archive/sdb%3Acollection|86058de1-8d55-4bc5-8305-5698d0bd7e13/ (adresse donnée par M22). Ouvrir quelques jours de juin à octobre 1917 et noter :
   - les colonnes publiées, dont la visibilité si elle existe ;
   - les lettres de temps (*z* haze, *m* mist) ;
   - les stations les plus proches de la Flandre (Douvres, Dungeness, stations françaises éventuelles).
4. ***The Seaman's Handbook of Meteorology*** (Met Office, 1918), https://www.AnalogWeather.com/uploads/7/7/7/5/77750690/meteorological_office_-_the_seamans_handbook_of_meteorology_-_1918.pdf : plus de 10 Mo, refusé par l'outil. Relever l'échelle de visibilité de l'époque et les définitions de *haze*, *mist* et *fog*.
5. **Van Beelen et van Delden** (Weather, 2012), https://dspace.library.uu.nl/bitstream/handle/1874/242388/CleanerAirBetterViewsMoreSunshine.pdf : refusé (403). Ce serait le meilleur point de comparaison moderne pour les Pays-Bas.
6. **AERONET**, station de Lille, niveau 2.0, moyennes mensuelles de juin à août : pour vérifier qu'une épaisseur optique de 0,14 à 550 nm est un ordre de grandeur actuel plausible (§ 5.2, point 4).
7. **Ohvril et al. (2009)** : texte intégral refusé (403 sur core.ac.uk) ; relever les valeurs de transparence de 1906 à 1940.

### 7.2 Demandes à des organismes (à reporter dans `demandes.md` si le joueur l'accepte ; ce fichier n'a pas été modifié)

| Proposition | Organisme | Objet |
|---|---|---|
| complément à D17 | Service historique de la Défense, département Air | dans les JMO des SPA 3 et 73 et du GC 12, les mentions de temps et de visibilité de juillet à novembre 1917 (les JMO notent souvent « temps brumeux », « visibilité nulle ») ; les bulletins du Service météorologique militaire auprès de la 1re Armée |
| nouvelle | Met Office, National Meteorological Library and Archive | archives de la section Meteor (Gold) : prévisions et observations de visibilité pour la Flandre, juillet-novembre 1917 |
| nouvelle | Bayerisches Hauptstaatsarchiv (Kriegsarchiv) ou Bundesarchiv-Militärarchiv | comptes rendus hebdomadaires du Kommandeur der Flieger de la 4. Armee et de ses Feldwetterwarten, juillet-novembre 1917 ; dossier KFT (WK) 2014 (photos aériennes) |

### 7.3 Témoignages à consulter (non lus)

- **En allemand** :
  - Karl Bodenschatz, *Jagd in Flanderns Himmel* (1935), journal de la JG I à Marcke et Courtrai, été et automne 1917 ;
  - Erwin Böhme, *Briefe eines deutschen Kampffliegers an ein junges Mädchen* (1930), Jasta Boelcke, Flandre 1917 ;
  - Reichsarchiv, *Flandern 1917* (série *Schlachten des Weltkrieges*, 1928).
- **En français et en belge** :
  - René Fonck, *Mes combats* (Flammarion, 1920 ; SPA 103, GC 12, à Bergues en 1917 ; probablement dans le domaine public en France depuis 2024, Fonck étant mort en 1953, à vérifier) ;
  - Willy Coppens, *Jours envolés* (1932).
- **En anglais** :
  - Arthur Gould Lee, *No Parachute* (1968), lettres du 46 Squadron, front d'Ypres en 1917 ;
  - Norman Macmillan, *Into the Blue* (1929), 45 Squadron.
- **Images** : photos aériennes obliques du saillant prises en 1917 (IWM, ECPAD, Musée de l'Air) : la meilleure référence visuelle du voile de brume. À consulter comme référence seulement, sans les intégrer au jeu.

## 8. Ce qui reste non trouvé

- Une **mesure chiffrée** de la visibilité en Flandre, ou à une station voisine, en juin-octobre 1917.
- L'**épaisseur de la couche de brume** l'après-midi : la hauteur de son sommet n'est donnée par aucun témoignage.
- La **hauteur et la largeur de la fumée** d'un jour d'offensive.
- Un témoignage **allemand** sur la Flandre en été 1917.

## 9. Sources

- **M1** — Charles J. Biddle, *The Way of the Eagle*, Charles Scribner's Sons, New York, 1919.
  - Archive.org `wayofeagle00biddrich`, texte OCR : https://archive.org/stream/wayofeagle00biddrich/wayofeagle00biddrich_djvu.txt (consulté le 2026-11-11) [LU-WEB].
  - Lettres du 10 août 1917 au 24 décembre 1917, p. 48-53, 77-88, 116, 135.
  - Primaire. Même texte que F3 de `flandres-1917.md`.
- **M2** — James T. B. McCudden, *Five Years in the Royal Flying Corps*, The « Aeroplane » and General Publishing Co., Londres, [1919] selon la notice.
  - Archive.org `5years-rfc`, texte OCR : https://archive.org/stream/5years-rfc/5YearsRFC_djvu.txt (consulté le 2026-11-11) [LU-WEB].
  - p. 185-298.
  - Primaire (mémoires écrits en 1918).
- **M3** — H. A. Jones, *The War in the Air*, vol. IV, Clarendon Press, Oxford, 1934.
  - Archive.org `warinairbeingsto04rale` (la notice porte à tort 1922), texte OCR : https://archive.org/stream/warinairbeingsto04rale/warinairbeingsto04rale_djvu.txt (consulté le 2026-11-11) [LU-WEB].
  - p. 83-90, 114-212.
  - Secondaire officielle, rédigée d'après les archives du RFC. Le PDF d'Electric Scotland (https://www.electricscotland.com/HISTORY/scotreg/warintheair04.pdf) dépasse la taille admise par l'outil.
- **M4** — Henry Bordeaux, *Vie héroïque de Guynemer*, Paris, Plon (éditeur et date à vérifier sur la page de titre ; la notice archive.org donne 1919).
  - Archive.org `viehroiquedegu00borduoft`, texte OCR : https://archive.org/stream/viehroiquedegu00borduoft/viehroiquedegu00borduoft_djvu.txt (consulté le 2026-11-11) [LU-WEB].
  - p. 244, 261-278.
  - Secondaire contemporaine ; une observation personnelle (p. 278 environ).
- **M5** — James Norman Hall, *High Adventure: A Narrative of Air Fighting in France*, Houghton Mifflin, Boston, 1918.
  - Project Gutenberg n° 24570 : https://gutenberg.org/cache/epub/24570/pg24570-images.html (consulté le 2026-11-11) [LU-WEB, ch. I à VII].
  - Primaire.
- **M6** — A. J. L. Scott, *Sixty Squadron R.A.F.: A History of the Squadron from its Formation*, Heinemann, Londres, 1920.
  - Project Gutenberg n° 47416 : https://gutenberg.org/cache/epub/47416/pg47416-images.html (consulté le 2026-11-11) [LU-WEB, partie centrale seulement].
  - Lettres de pilotes de 1917 : primaire.
- **M7** — Manfred von Richthofen, *Der rote Kampfflieger*, Ullstein, Berlin, 1917.
  - Project Gutenberg n° 24572 : https://www.gutenberg.org/cache/epub/24572/pg24572-images.html (consulté le 2026-11-11) [LU-WEB, jusqu'à fin mai 1917].
  - Primaire, propagande.
- **M8** — Ernst Udet, *Mein Fliegerleben*, Berlin, 1935 (éditeur à vérifier).
  - Projekt Gutenberg-DE, chapitres 5 et 7 : https://projekt-gutenberg.org/authors/ernst-udet/books/mein-fliegerleben/chapter/5 et …/chapter/7 (consulté le 2026-11-11) [LU-WEB].
  - Mémoires tardifs.
- **M9** — A. Singh, W. J. Bloss et F. D. Pope, « 60 years of UK visibility measurements: impact of meteorology and atmospheric pollutants on visibility », *Atmos. Chem. Phys.*, 17, 2085-2101, 2017, doi:10.5194/acp-17-2085-2017, licence CC BY 3.0.
  - https://acp.copernicus.org/articles/17/2085/2017/acp-17-2085-2017.pdf (consulté le 2026-11-11) [LU, p. 2085-2093].
  - Valeurs lues sur la figure 2.
- **M10** — OMM, *Guide to Instruments and Methods of Observation* (WMO-No. 8), vol. I, chapitre 9, « Measurement of visibility », édition préliminaire de 2018 (projet soumis aux membres).
  - § 9.1.1 à 9.1.3 : https://old.wmo.int/extranet/pages/prog/www/IMOP/publications/CIMO-Guide/Prelim_2018_ed/8_I_9_en_MR_tc.pdf (consulté le 2026-11-11) [LU, p. 1-4].
  - Primaire (norme). À confirmer dans l'édition définitive en vigueur.
- **M11** — W. Wandji Nyamsi et al., « Long-term trends in reconstructed atmospheric aerosol load based on large-scale sunshine duration records since 1900 », *Atmos. Chem. Phys.*, 26, 10767-10788, 2026, doi:10.5194/acp-26-10767-2026.
  - https://acp.copernicus.org/articles/26/10767/2026/acp-26-10767-2026.html (consulté le 2026-11-11) [LU-WEB], § 3.3, 4.1-4.4, 5, annexe A.
- **M12** — H. Ohvril et al., « Global dimming and brightening versus atmospheric column transparency, Europe, 1906–2007 », *J. Geophys. Res.*, 2009 (volume et identifiant à vérifier).
  - Notice PNNL : https://www.pnnl.gov/publications/global-dimming-and-brightening-versus-atmospheric-column-transparency-europe-1906-2007 [RÉSUMÉ].
- **M13** — R. G. Harrison, « Urban smoke concentrations at Kew, London, 1898–2004 », *Atmospheric Environment*, 2006 (volume et pages à vérifier).
  - Notice de l'université de Reading : https://reading-pure-test.eprints-hosting.org/5150/ [RÉSUMÉ].
  - Non utilisé pour un chiffre.
- **M14** — Gomez et Smith, articles sur la visibilité à Oxford (1926-1985), *Weather*, 1984 et 1987, cités par M9 p. 2086 [non lus].
- **M15** — R. Vautard, P. Yiou et G. J. van Oldenborgh, « Decline of fog, mist and haze in Europe over the past 30 years », *Nature Geoscience*, 2, 115-119, 2009, doi:10.1038/NGEO414.
  - Notice du KNMI : https://www.knmi.nl/research/publications/the-decline-of-fog-mist-and-haze-in-europe-during-the-last-30-years-a-warming-amplifier [RÉSUMÉ] : seuils de 1, 2 et 5 km ; 342 stations.
- **M16** — Section Meteor (Ernest Gold) :
  - Wikipédia (en), « Ernest Gold (meteorologist) » ;
  - British Artillery, https://www.britishartillery.co.uk/fc_meteor.htm ;
  - Met Office, « Met Office forecasts and World War 1 » (consultés en résumé le 2026-11-11) [RÉSUMÉ] ; la page https://www.metoffice.gov.uk/learning/library/archive-hidden-treasures/first-military-forecast renvoie une erreur 404.
  - Pistes.
- **M17** — Chambre des communes, réponse écrite du 28 juillet 1930, vol. 242, col. 58-59W, « Statistics », https://api.parliament.uk/historic-hansard/written-answers/1930/jul/28/statistics [RÉSUMÉ] : charbon disponible pour la consommation intérieure de la Grande-Bretagne en 1913, 183,8 millions de tonnes (*tons*).
- **M18** — Michelin & Cie, *Lille Before and During the War*, Clermont-Ferrand, 1919.
  - Project Gutenberg n° 49123 : https://www.gutenberg.org/cache/epub/49123/pg49123-images.html (consulté le 2026-11-11) [LU-WEB], section « The Ruined Industries of Northern France ».
- **M19** — Bayerisches Hauptstaatsarchiv, KFT (WK) 2014 (ancienne cote : Kommandeur der Flieger 4. Armee 11), « Luftbilder aus der Abwehrschlacht in Flandern, 31.07.-31.09.1917; Heft B: Artilleristische Bilderkundung ».
  - https://www.archivportal-d.de/item/OYQAP5V5U2CE64RSYHDZIOK5HNMI7AH3 (consulté le 2026-11-11) [LU-WEB, notice].
- **M20** — Landesarchiv Baden-Württemberg, Hauptstaatsarchiv Stuttgart, M 34 Bü 116, « Tägliche Meldungen und Wochenberichte betr. v. a. Flieger-, Ballon- und Flaktätigkeit, Wetter », 21 novembre-31 décembre 1917.
  - https://www-p.archivportal-d.de/item/3HOHLPXO3Y6EUQXALHCMP242LB7MYDQ7 (consulté le 2026-11-11) [LU-WEB, notice].
  - Concerne l'Armee-Abteilung B, pas la Flandre.
- **M21** — F. de Auer-Véran (sous la direction de P. Gallien, G. Rech et A. Chablat-Beylot), *Archives de l'aéronautique militaire de la Première Guerre mondiale. Répertoire numérique détaillé de la série A (1914-1919) et guide des sources*, Service historique de la Défense, Vincennes, 2008, ISBN 978-2-1109-6328-4.
  - PDF : https://www.horizon14-18.eu/wa_files/archives_20de_20l_27a_C3_A9ronautique_20militaire.pdf (consulté le 2026-11-11) [LU, pages de titre seulement ; PDF chiffré].
- **M22** — P. Brohan, « The Daily Weather Reports », https://brohan.org/UK-digitisation/sources/DWR.html (consulté le 2026-11-11) [LU-WEB] ; Met Office, notices des volumes de 1921-1930 [RÉSUMÉ].
- **M23** — *The Dominion* (Wellington), 24 octobre 1917, https://paperspast.natlib.govt.nz/newspapers/DOM19171024.2.31.3, et *Poverty Bay Herald*, 25 octobre 1917, https://paperspast.natlib.govt.nz/newspapers/PBH19171025.2.15.1 [RÉSUMÉ ; pages illisibles par l'outil]. Réécritures de presse des dépêches de Haig.
- **M24** — Association des anciens de la météorologie, bulletins *AEC*, https://anciensmeteos.info [RÉSUMÉ] : création du Service météorologique militaire en octobre ou novembre 1916, plus de 2 000 hommes en 1918. Piste.
- **M25** — A. J. van Beelen et A. J. van Delden, « Cleaner air brings better views, more sunshine and warmer summer days in the Netherlands », *Weather*, 2012 [PISTE ; 403].
- **M26** — Meteorological Office, *The Seaman's Handbook of Meteorology*, 1918, PDF sur AnalogWeather.com [PISTE ; trop volumineux pour l'outil].

## 10. Journal des décisions

- **2026-11-11 — Deux conventions de visibilité.** Le plan calcule avec Koschmieder (3,912/σ). Les séries d'observations et les témoignages relèvent de la MOR, ou de l'estimation humaine (≈ 3,0/σ). Les deux sont données partout (§ 1 et 5.1). Sans cela, les « 40-60 km » du plan semblaient plus proches des observations qu'ils ne le sont.
- **2026-11-11 — Plage d'été 1917 : 15-30 km en MOR par beau temps typique, confiance C.**
  - Aucune mesure de 1917 n'a été trouvée.
  - La plage combine les moyennes d'été des années 1950 en Angleterre (M9), dans le même régime de charbon (M17), et le caractère rural et maritime de la Flandre, avec une industrie lilloise arrêtée (M18).
  - Les témoignages (traits 1 et 5) concordent qualitativement.
  - Écart retenu : 40-60 km pour les jours très clairs, 8-15 km pour les jours de brume.
- **2026-11-11 — Défaut du jeu : 30 km dans la convention du plan, `ESTIMÉ`.**
  - Le plan proposait 40-60 km ; les sources pointent plus bas pour un jour typique.
  - 30 km (23 km en MOR) est au centre de la plage.
  - 50-60 km sont réservés à un préréglage « jour clair ».
  - Si l'image souffre, on agit sur le profil vertical, conformément au trait 2, plutôt que sur la visibilité au sol.
- **2026-11-11 — Trait « brume basse, ciel bleu au-dessus » : B, pas A.** Trois sources alliées concordent (T2, T5, T6), mais il n'y a aucune source allemande, et Biddle donne un contre-exemple à 4 900 m le 16 octobre. Le trait est retenu comme cas général, pas comme règle.
- **2026-11-11 — Témoignages hors période ou hors secteur.** Richthofen (Artois, printemps 1917), Hall (Aisne, juin 1917) et Udet (1918) sont gardés pour croiser les traits, jamais seuls. Ils sont signalés comme tels dans le tableau du § 2.1.
- **2026-11-11 — Italien non cherché.** Aucune unité italienne ne volait en Flandre ; une brume du front italien ne renseignerait pas sur celle des Flandres.
