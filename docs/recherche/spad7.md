# SPAD S.VII C.1 — fiche de recherche

- Identifiant : `spad7` (même nom que `assets/aircraft/spad7/`)
- Dernière mise à jour : 2026-10-24
- État : **en cours** (dixième passe). Sont désormais sourcés par des documents d'époque lus :
  - le profil « Spad » (B) ;
  - les polaires du RAF 15 (A pour ce profil) ;
  - la courbe de puissance et la loi d'altitude du Hispano de 180 hp (B pour un 8Ab) ;
  - une hélice en bois comparable (A pour la mesure, C pour l'usage) ;
  - des lois d'interférence biplan (C pour le SPAD) ;
  - **quatrième passe** : des **vitesses en palier, des masses, une surface et des cotes du S.VII** lues dans *Flight* (1917 et 1919), dont deux tableaux officiels américains d'avions français et italiens ;
  - **cinquième passe (2026-10-21)** : ces tableaux ont été **lus sur l'image des pages**. Les vitesses en palier à 2 000, 3 000, 4 000 et 5 000 m, les temps de montée, les plafonds théoriques et les régimes moteur des deux versions passent en **A** (§ 5). Plusieurs lectures OCR de la quatrième passe étaient fausses ; elles sont corrigées et signalées au journal.
  - **sixième passe (2026-10-22)** : la réserve de lecture sur la ligne du 8Aa (119,5 et 111,5 mph) est **levée** par un contrôle interne du tableau (§ 0) ; les facteurs τ et δ de l'aile rectangulaire sont trouvés dans le NACA TR 416 (`profil-spad7.md` § 4.2).
  - **septième passe (2026-10-22), géométrie de détail** (§ 2.5) : **aucune cote chiffrée nouvelle du S.VII**. Apports : des descriptions qualitatives du S.VII (forme en plan, haubanage, fuselage, capot), des comparables chiffrés du SPAD XIII (empennage, ailerons, dérive ; tableau français de 1919 lu sur l'image) et du biplace S.XI, et une fourchette calculée de la corde (1,14 à 1,33 m, C). Empennage, gouvernes, débattements, décalage, calages, fuselage, roues et centrage du S.VII restent **non trouvés**.
  - **huitième passe (2026-10-22), fiabilité des performances du 150 ch** (§ 5.5) : les vitesses de S46 forment deux groupes incompatibles à 3-4 % près (192 km/h à 2 000 m d'un côté, 179 et 174 km/h à 3 000 et 4 000 m de l'autre) ; la série de montée du STAé citée par S11 n'est pas indépendante de S46 ; le plafond « théorique » est une extrapolation. Nouvelles cibles avec tolérances au § 5.5.6, qui remplace le § 5.4 pour le 150 ch.
  - **neuvième passe (2026-10-23), hélice, régimes et bord d'attaque** :
    - aucun régime relevé en montée sur un S.VII (§ 5.6) ; régimes mesurés en vol sur un avion comparable, le VE-7 (`hispano-suiza-8a.md` § 7.5) ;
    - hélice de l'avion de S46 non trouvée ; liste des hélices autorisées pour le S.VII à Hispano de 150 ch vue en résumé (`hispano-suiza-8a.md` § 7.5) ;
    - bord d'attaque des ailes du S.VII : aucune source directe ; indices de famille contradictoires (§ 2.6).
  - **dixième passe (2026-10-24), conditions réelles des essais de montée** (§ 5.7) :
    - départ du chronomètre des essais français : **non trouvé** ;
    - pratique britannique de 1917 lue (Tizard, S75) : temps réduits par intégration depuis la hauteur type 0, donc en vol, comme le modèle ;
    - pas d'atmosphère type officielle en France avant le 15 avril 1920 (S79, en résumé) ;
    - terrain d'essai probable : Villacoublay, environ 178 m (C) ; date et saison de l'essai de S46 : non trouvées ;
    - ordres de grandeur calculés pour l'enveloppe (§ 5.7.5) et documents à télécharger (§ 5.7.7).
- Fiches liées : [`hispano-suiza-8a.md`](hispano-suiza-8a.md) (moteur et hélice), [`profil-spad7.md`](profil-spad7.md) (profil d'aile)
- Contexte de jeu visé : escadrille SPA 3 (« Cigognes », Groupe de Combat 12), Flandres, été 1917

## 0. Méthode et limites de cette passe (à lire avant d'utiliser un chiffre)

**Accès réseau.** Depuis le conteneur, l'outil de lecture de pages n'atteint que `github.com` et `raw.githubusercontent.com`. Tous les autres sites testés sont inaccessibles (liste au § 9.5). Le moteur de recherche fonctionne, mais il renvoie un **résumé généré** à partir des pages, pas les pages elles-mêmes.

**Statut de chaque source**, indiqué entre crochets :
- **[LU]** : texte intégral lu. Ce sont des ouvrages du domaine public (Project Gutenberg) récupérés par le miroir GITenberg sur GitHub, puis recherchés et lus ligne à ligne.
- **[RÉSUMÉ]** : vu seulement dans un résumé du moteur de recherche. La page n'a pas été ouverte, et la formulation comme les chiffres peuvent avoir été déformés. **Aucun chiffre [RÉSUMÉ] ne doit être tenu pour vérifié.**
- **[PISTE]** : référence bibliographique repérée, non consultée.

**Conséquences.**
- Les performances sont plafonnées à la **confiance C**. Les deux séries de mesures d'époque (STAé et rapport britannique n° 96) ne sont connues que par un forum vu en résumé. *(Dépassé : depuis la cinquième passe, les performances sont lues dans les tableaux S46 et S47 ; leur fiabilité est discutée au § 5.5.)*
- Une géométrie ou une masse n'obtient **B** que si plusieurs sources secondaires concordent, même vues en résumé.
- **Exclusion** : les chiffres des jeux et simulateurs (IL-2 *Flying Circus*, *Rise of Flight*, *Wings of Glory*) ne sont pas retenus comme sources (S39). Ils sont dérivés et alimentent les forums en boucle.
- Aucune lacune n'est comblée de mémoire. Quand une valeur manque, c'est écrit « non trouvé ».

**Deuxième passe (2026-10-20).** Le coordinateur signale qu'archive.org et ntrs.nasa.gov répondent désormais au `curl` du conteneur. Mais l'outil de lecture de l'historien ne passe pas par le réseau du conteneur et n'a pas de terminal.
- Échec de résolution DNS, testé le 2026-10-20 : archive.org, www.archive.org, scholar.archive.org, ia800300.us.archive.org, ntrs.nasa.gov.
- Refus explicite de l'outil : web.archive.org.
- **Aucun document d'archive.org ni de NTRS n'a donc pu être lu, et aucune confiance n'a été relevée.**
- Seul gain : des **identifiants exacts** de documents à télécharger (§ 9.6), trouvés par le moteur de recherche restreint à ces domaines. Pour les exploiter, un agent ou une personne disposant d'un terminal doit les télécharger dans un dossier local (texte OCR `_djvu.txt` d'archive.org, PDF de NTRS) ; l'historien les lira ensuite.

**Troisième passe (2026-10-20).** Le coordinateur a téléchargé **15 rapports NACA** depuis NTRS (PDF et texte extrait) dans un dossier de travail local. L'historien les a lus comme des données, sans rien exécuter, et a vérifié sur l'image des pages les chiffres douteux de l'OCR. Ils sont marqués **[LU]** avec leurs pages.
- Ils alimentent `profil-spad7.md` (profil et polaires), `hispano-suiza-8a.md` (moteur, consommation, hélice) et le § 2.4 ci-dessous (interférence biplan, stabilité).
- **Les fichiers d'archive.org restent inaccessibles.** *Flight*, *The Aeroplane*, Jane's et les rapports d'essais du STAé et britanniques ne sont donc toujours pas lus, et les **performances du S.VII restent en confiance C**.

**Quatrième passe (2026-10-20).** Le coordinateur a téléchargé le texte OCR de documents d'archive.org. Lus comme des données, sans rien exécuter, et marqués **[LU-OCR]** :
- **208 numéros de *Flight*** (1916-1919). Tous les passages contenant « Spad » (67 numéros) ont été lus en contexte, ainsi que les passages pertinents sur « Hispano », « Guynemer » et « Storks » ;
- le **volume II** du *Technical Report of the Advisory Committee for Aeronautics* 1917-18 (hélices et essais en vraie grandeur, publié en 1921) ;
- la **bibliographie aéronautique 1917-1921** du Smithsonian (P. Brockett), comme outil de repérage ;
- trois fichiers d'*Aerial Age Weekly*. Ceux du 2 juillet et du 19 novembre 1917 sont des **pages HTML d'archive.org**, pas le texte OCR : inexploitables (§ 9.6). Le volume de 1919 ne contient rien d'utile sur le S.VII ;
- le 2026-10-21, deux rapports NACA supplémentaires (TN 182 de Prandtl et Report 267 de Hemke), en PDF : les tableaux ont été lus **sur l'image** des pages ([LU]).

**Limite de lecture (quatrième passe).** Les deux tableaux officiels de 1919 (S46, S47) sont imprimés en colonnes serrées et l'OCR les restitue dans le désordre. En quatrième passe, seuls les chiffres validés par un contrôle interne avaient été retenus, avec un plafond en B.

**Cinquième passe (2026-10-21) : lecture sur l'image [LU-IMG].** Le coordinateur a fourni les pages 199 à 207 du numéro du 13 février 1919 et 237-238 du numéro du 20 février 1919, en images PNG à 150 dpi, tirées des PDF d'archive.org. Chaque chiffre cité comme [LU-IMG] a été lu sur l'image.
- **Règle de lecture du tableau S46**, établie sur l'image : le régime moteur de chaque ligne est imprimé **sous** la ligne, en face de la puissance et de chaque vitesse. Contrôle : sur toutes les lignes voisines (Nieuport, Breguet, Voisin, SPAD XIII), le nombre de régimes imprimés sous une ligne égale exactement le nombre de vitesses de cette ligne. En quatrième passe, j'avais attribué à tort au 180 ch les régimes de la ligne du 150 ch.
- **Limite** : la ligne « Spa. 7C1 / Hispano 8Aa » est la **première de la page 205**, en haut de la feuille, et elle est pâle. Elle reste lisible, mais les chiffres 3 et 5 y sont les plus incertains. L'OCR d'archive.org y lisait systématiquement des 3 là où l'image montre des 5 (431 pour 451, 1 530 pour 1 550, 119,3 pour 119,5). La résolution fournie ne permet pas mieux.

**Sixième passe (2026-10-22) : réserve levée [LU-IMG + contrôle C].** Le coordinateur a fourni le haut de la p. 205 à 300 dpi (4 013 × 2 700 px, `f19190213-p205-300dpi-15.png`).
- **Limite de l'outil.** L'image m'est affichée réduite de moitié (2 000 px de large), soit la **même résolution effective** qu'en cinquième passe, autour de 150 dpi. Ma lecture directe est inchangée : « 119·5 », « 111·5 », « 451 », « 1550 ». Elle ne suffirait pas seule à lever le doute.
- **Contrôle interne décisif pour les vitesses** (C, fait ici sur les p. 204-205) :
  - le tableau « in service » arrondit **toutes** ses vitesses au demi-mph. J'ai relevé 85 autres vitesses dans ce tableau (68 p. 204, 17 en haut de la p. 205, dont la ligne du 8Ab : 132, 126, 124·5, 116) : toutes sont des entiers ou des demis ;
  - à l'inverse, le tableau « under test », plus bas sur la p. 205, donne des dixièmes (110·6, 118·7, 126·4, 113·2, 133·7…) ;
  - une valeur « 119·3 » ou « 111·3 » serait donc la **seule exception** du tableau. **119,5 et 111,5 mph sont confirmés.**
- **Masses** : 451 = 176 + 176 + 88 + 11 et 1 100 + 451 = 1 551 ≈ 1 550, contrôles déjà notés en cinquième passe (§ 3).
- **Conséquence sur la précision** : l'arrondi au demi-mph de l'imprimé ajoute une incertitude de ±0,25 mph, soit **±0,4 km/h**, sur chaque vitesse du tableau « in service ». Elle remplace la réserve de lecture de ±0,3 km/h.

**Septième passe (2026-10-22) : géométrie de détail.** Objet : les cotes de détail qui manquent pour écrire `aircraft.ron` (S.VII 150 ch, 8Aa, 703 kg). Méthode et bilan au § 2.5.
- **Sources locales** relues comme des données, sans rien exécuter : tous les passages « Spad » des 208 numéros de *Flight* ; S48 relu en entier dans l'OCR ; S49 relu en entier, légendes du dessin coté comprises ; les lignes SPAD XIII et SPAD XI des tableaux S46 et S47 relues **sur l'image** (p. 204-205 et 237) ; la bibliographie de Brockett (S56) ; le volume II du rapport ACA 1917-18 ; les rapports NACA du dossier NTRS. En dehors du profil « Spad » du Report 93, aucun rapport NACA du dossier ne parle d'un SPAD.
- **Web** : environ 35 requêtes en anglais, français, allemand, italien, tchèque et russe. Les pages visées restent hors d'atteinte de l'outil de lecture (échec DNS le 2026-10-22 pour thisdayinaviation.com, bibliotheques-numeriques.defense.gouv.fr et sites.ualberta.ca) : tout ce qui vient du web est **[RÉSUMÉ]**.
- **Exclusion maintenue** : aucun chiffre de jeu (S39) ni de maquette du commerce n'est retenu.

**Huitième passe (2026-10-22) : fiabilité des performances.** Objet : juger les performances d'époque du 150 ch sur leurs seuls mérites de sources et fixer les cibles d'essai (§ 5.5).
- **Relu sur l'image** [LU-IMG] : p. 201 (présentation des tableaux), 204, 205 (150 et 300 dpi) et 206 du 13 février 1919 ; p. 237 et 238 du 20 février 1919. Le tableau britannique de la p. 206 (S66) et le tableau américain de la p. 238 (S67) servent de comparaison.
- **Relu dans l'OCR** [LU-OCR] : R&M 462, § 18 à 21 et annexe II (réduction des performances à une base commune), dans le volume II du rapport ACA 1917-18, qui ne contient aucun essai de SPAD ; tous les passages « Spad » et « Heurt » des 208 numéros de *Flight*. Aucune autre mesure du S.VII que S46, S47 et S48.
- **Web** : 9 requêtes en anglais, français, allemand et italien, toutes en résumé. Échec DNS le 2026-10-22 pour francearchives.gouv.fr, le portail du SHD, fr.geneawiki.com, beta.nationalarchives.gov.uk et enginehistory.org. Deux cotes d'archives nouvelles, vues en résumé seulement (S69, S70 ; § 5.5.7).
- Les calculs de cohérence du § 5.5.3 sont des **ordres de grandeur** (C) sur une polaire supposée ; le calcul exact revient à l'aérodynamicien.

**Neuvième passe (2026-10-23) : hélice, régimes d'essai, bord d'attaque.** Objet :
- l'hélice réelle du S.VII 150 ch, détaillée dans `hispano-suiza-8a.md` § 7.5 ;
- un régime moteur relevé hors vitesse maximale (§ 5.6) ;
- la construction du bord d'attaque des ailes (§ 2.6).

Méthode et accès :
- **Outil de lecture** : échec DNS le 2026-10-23 pour tous les sites visés (woodenpropeller.com, theaerodrome.com, archive.org, gallica.bnf.fr, francearchives.gouv.fr, catalog.hathitrust.org, reports.aerade.cranfield.ac.uk). Tout ce qui vient du web est donc **[RÉSUMÉ]**.
- **Relus localement** : les NACA Reports 220 et 292, lus sur l'image [LU-IMG] ; les passages « Spad », « propeller », « screw », « leading edge » et « plywood » de *Flight* (208 numéros), dont S48 et S49, et du volume II du rapport ACA 1917-18 [LU-OCR].
- **Rien n'a été téléchargé.**

**Dixième passe (2026-10-24) : conditions réelles des essais de montée.** Objet : savoir comment les montées d'époque étaient chronométrées et mesurées, pour chiffrer ce que les conditions d'essai ajoutent au temps (§ 5.7).
- **Relus localement, comme des données** [LU-OCR] :
  - les 208 numéros de *Flight* (1916-1919), sur les termes de méthode : « leaving the ground », « start of the climb », « barograph », « aneroid », « standard atmosphere », « Villacoublay », « French method », « Section technique », « official trials ». Trois textes utiles : la conférence de Tizard de mars 1917 (S75), le règlement des essais certifiés du Royal Aero Club (S76) et l'article de Dines (S77) ;
  - le volume II du rapport ACA 1917-18 : R&M 462 relu ; aucun autre texte sur la méthode des essais de montée ;
  - le dossier Hartmann sur les héliciers [LU].
- **Web** : environ 30 requêtes en français, anglais et italien, toutes vues en **résumé**. L'outil de lecture échoue toujours (DNS) sur www.hydroretro.net, archive.org, ntrs.nasa.gov, www.academie-technologies.fr et resair420.free.fr (testés le 2026-10-24).
- **Rien n'a été téléchargé.** Les documents qui répondraient sont listés au § 5.7.7.

## 1. Identification et contexte

### 1.1 Constructeur et genèse
- Constructeur : SPAD (Société Pour L'Aviation et ses Dérivés), Paris ; concepteur Louis Béchereau (S1 [LU], qui nomme « M. Béchereau of the Spad works » ; S15 [RÉSUMÉ]).
- Prototype SPAD V : premier vol en **avril 1916**, piloté par Bécquet (S5 [RÉSUMÉ]).
- Premier marché le **10 mai 1916**, pour **268** appareils ; le dernier de cette série n'est livré qu'en février 1917 (S5 [RÉSUMÉ]). Au 25 février 1917, **268** S.VII livrés à l'Aviation militaire, plus **39** au Royal Flying Corps (S8 [RÉSUMÉ]). La lenteur des premières livraisons est attribuée à la fabrication des radiateurs (S5 [RÉSUMÉ]).
- Construction sous licence en France par Grémont, Janoir, Kellner et Fils, de Marçay, la Société d'Études Aéronautiques, Régy et Sommer (S5 [RÉSUMÉ]). En Grande-Bretagne par Mann, Egerton & Co et Blériot & SPAD : premier exemplaire britannique essayé en avril 1917 (S5, S35 [RÉSUMÉ]).

### 1.2 Sous-versions (ce qui compte pour le modèle de vol)

| Sous-version | Moteur | Introduction | Sources |
|---|---|---|---|
| S.VII « 150 ch » | Hispano-Suiza 8Aa, 150 ch | premiers appareils au front début septembre 1916 | S1 [LU] ; S5 [RÉSUMÉ] |
| S.VII « 180 ch » | Hispano-Suiza 8Ab, 180 ch (taux de compression porté de 4,7 à 5,3) | « début 1917 » (S5) ; « à partir du printemps 1917 » (S8) ; toutes les machines neuves équipées « en avril 1917 » (S16) | S5, S8, S16 [RÉSUMÉ] |
| (S.XII à canon) | Hispano-Suiza 8C à réducteur (`hispano-suiza-8a.md`, H1 [RÉSUMÉ]) | été 1917 (avion de Guynemer) | S1 [LU] (« avion magique », type non nommé) ; identification comme SPAD XII à vérifier |
| (S.XIII) | Hispano-Suiza 8B à réducteur, deux Vickers | prototype le 4 avril 1917 ; premières livraisons début mai 1917 ; **131** livrés en décembre 1917 seulement | S21, S16 [RÉSUMÉ] |

Le n° **S.254** (« Vieux Charles » de Guynemer, conservé au Musée de l'Air) est présenté par une revue de maquette comme « le premier appareil produit avec ce moteur [180 ch] » (S17 [RÉSUMÉ]). Une autre source le dit remis à Guynemer en décembre 1916. **Non vérifié** : aucune source fiable ne précise la motorisation de S.254.

### 1.3 Service (repères)
- **France** : la N 3 reçoit les deux premiers SPAD au front ; Guynemer et le caporal Sauvage les pilotent sur la Somme au début de septembre 1916 (S1 [LU]). Une source en résumé donne le **2 septembre 1916** (S5 [RÉSUMÉ]). En août 1917, près de **500** S.VII servent dans plus de **50** escadrilles françaises, dont la SPA 3 (S5 [RÉSUMÉ]).
- **Royaume-Uni** : n° 19 et 23 Squadrons du RFC sur le front Ouest (S9 [RÉSUMÉ]). **185** SPAD en service au RFC puis à la RAF (S35 [RÉSUMÉ]).
- **Italie** : la 77a Squadriglia reçoit des S.VII en mars 1917. La 91a Squadriglia est créée le **1er mai 1917** avec **4** S.VII et **3** Nieuport 17 (S24 [RÉSUMÉ]). Baracca essaie le S.VII le **20 mars 1917** (S25 [RÉSUMÉ]).
- **Escadrille Lafayette** : la N 124 devient SPA 124 en 1917, quand les S.VII remplacent ses Nieuport (S32 [RÉSUMÉ]). Hall décrit les SPAD de l'escadrille, marqués d'une tête d'Indien (S3 [LU]).
- **Russie** : pistes seulement (S36 [RÉSUMÉ]) : environ 100 appareils construits par Dux, un S.VII sur skis attribué à Kozakov en décembre 1917. Source : description de maquette, **à vérifier**.

### 1.4 La SPA 3 à l'été 1917 : quels avions ? (point obligatoire)

**Chronologie établie** (avec le statut de chaque source) :

| Date | Fait | Source |
|---|---|---|
| début sept. 1916 | Les deux premiers SPAD au front, à la N 3 (Guynemer, Sauvage) | S1 [LU] |
| 8 sept. 1916 | Lettre de Guynemer à Béchereau sur le « baptême du feu » du SPAD | S1 [LU] |
| 28 déc. 1916 | Guynemer à Béchereau : le « Spad 150 H.P. » n'a guère d'avance sur le Halberstadt, qui n'est pas plus rapide mais monte bien mieux ; il réclame plus de vitesse et une meilleure hélice | S1 [LU] |
| janv. 1917 (?) | Désignation « Spa 3 » (janvier 1917 selon S18 ; S20 la place après une citation de mars 1917, sans mois) | S18, S20 [RÉSUMÉ] — **contradiction** |
| printemps 1917 | Le 8Ab de 180 ch devient le moteur des S.VII neufs | S5, S8, S16 [RÉSUMÉ] |
| 4 avr. / mai 1917 | Premier vol du SPAD XIII, puis premières livraisons | S21, S16 [RÉSUMÉ] |
| 5 juil. 1917 | Premier vol de l'« avion magique » de Guynemer, à canon | S1 [LU] |
| 12 juil. 1917 | Le GC 12 part en Flandre, en appui de la 1re Armée | S20 [RÉSUMÉ] |
| juil. 1917 | Les Cigognes arrivent en Flandre : camp de toile près de Dunkerque, au bord de la mer ; les Cigognes rattachées à un groupement sous le commandant Brocard | S1 [LU] |
| 23 juil. 1917 | Guynemer ramène de Fismes son « avion magique » | S1 [LU] |
| fin juil. 1917 | Guynemer commence à voler sur le SPAD XIII **S.504** | S18 [RÉSUMÉ] |
| 15-20 août 1917 | Guynemer revient en Flandre le 15 août. Le 20, « volant cette fois sur son vieux *Vieux-Charles* », il abat un DFW au-dessus de Poperinghe (S1 [LU]). Selon S18, c'est sa 53e victoire, sur S.504 [RÉSUMÉ]. Bordeaux écrit qu'Auger est alors déjà mort | S1, S18 |
| 4 sept. 1917 | Guynemer au terrain de **Saint-Pol-sur-Mer**, près de Dunkerque ; le terrain est partagé avec les Britanniques | S1 [LU] |
| 5 et 10 sept. 1917 | Guynemer vole sur une « machine à deux mitrailleuses » (*two-gun machine*), donc un SPAD XIII ; il emprunte aussi les avions de Deullin et de Lagache, de type non précisé | S1 [LU] |
| 11 sept. 1917 | Départ à 8 h 25 avec Bozon-Verduraz sur le *Vieux-Charles* (S1 [LU], journal d'escadrille cité) ; SPAD XIII S.504 « n° 2 » selon S19 [RÉSUMÉ] | S1, S19 |
| sept. 1917, **date exacte non donnée** | Le capitaine Heurtaux (« Heurteaux » dans la presse britannique), des Cigognes, blessé en combat ; la nouvelle est publiée à Paris le 30 septembre (S55). *Flight* du 11 octobre précise qu'il essayait une machine neuve vers 21 900 ft près d'Ypres ; sa mitrailleuse enraye, il rompt en piqué, est touché au côté gauche et se pose sur un terrain britannique (S68). Les deux échos divergent (deux balles, puis une) ; l'altitude est un chiffre de presse. Huitième passe : la date « fin septembre » des passes précédentes était celle de la publication, pas celle de la blessure | S55, S68 [LU-OCR] |
| 5 déc. 1917 | Biddle (SPA 73, même GC 12) remporte sa première victoire sur SPAD XIII près de Langemarck | S22 [RÉSUMÉ] |

**Contradictions de personnel** (à vérifier avant tout texte de jeu sur ces hommes) :
- S20 [RÉSUMÉ] donne le capitaine Auger tué le **28 juin 1917**. Or S1 [LU] le cite parmi les Cigognes présentes en Flandre fin juillet 1917, en précisant « bientôt tué », et le dit mort au 17 août 1917 au plus tard. Sa mort se situerait donc fin juillet ou début août 1917.
- S20 donne Guynemer commandant de l'escadrille du **29 juin au 5 août 1917**. S1 écrit que Heurtaux, remis de sa blessure d'avril, « commandait » en juillet 1917.
- S1 cite Fonck parmi les « nouveaux venus » aux côtés des Cigognes en Flandre. S40 [RÉSUMÉ] le dit affecté à la SPA 103, du même GC 12, en avril 1917. Pour le jeu, Fonck appartient à la SPA 103, pas à la SPA 3, sauf preuve contraire.

**Ce qui n'a pas été trouvé** : aucun état des appareils de la SPA 3 (numéros, moteurs) pour juillet-septembre 1917. Ni l'historique d'Albin Denis (`albindenis.free.fr`, inaccessible) ni les journaux de marche (SHD) n'ont été consultés.

**Évaluation (fourchette)** :
1. **Type** : à l'été 1917, la SPA 3 vole surtout sur **S.VII**. Le SPAD XIII n'y apparaît qu'en très petit nombre à partir de fin juillet-août 1917 (Guynemer). Deux indices : seuls **131** SPAD XIII avaient été livrés en décembre 1917 (S21), et S5 mentionne la SPA 3 parmi les unités sur S.VII en août 1917. Confiance **C**.
2. **Moteur** : le parc de la SPA 3 mêle très probablement des 150 ch, livrés de septembre 1916 au début de 1917, et des 180 ch, livrés au printemps-été 1917. Aucune source ne donne la proportion. L'hypothèse selon laquelle une unité d'élite comme le GC 12 aurait été servie en priorité **n'est pas sourcée**.
3. **Choix pour le jeu** : **S.VII à Hispano-Suiza 8Ab de 180 ch**, comme machine « type » d'une recrue arrivant à la SPA 3 en juillet-septembre 1917. Confiance **C**. La version de 150 ch reste documentée ici (§ 3 et § 5), car c'est la mieux mesurée (voir journal des décisions).

### 1.5 Anachronismes à éviter (liste de contrôle pour les missions)
- Pas de SPAD XIII pour le joueur au M1. À l'été 1917, un XIII à la SPA 3 reste une machine de chef (Guynemer), rare jusqu'à la fin de 1917 (S21, S18, S1).
- Terrain de l'été 1917 : près de Dunkerque, Saint-Pol-sur-Mer nommément en septembre 1917 (S1). Les autres terrains de la Flandre (Bierne, Coudekerque…) ne sont **pas confirmés**.
- Insigne : cigogne de la SPA 3 (voir § 8). La désignation « SPA 3 » est en usage en 1917 (date exacte de l'adoption contestée, § 1.4).
- Le Rumpler ou le DFW de Poelkapelle et les circonstances de la mort de Guynemer : s'en tenir aux faits documentés (S1, S19), ton sobre.
- « Vieux Charles » est un **nom porté par plusieurs avions** de Guynemer (§ 8), pas un numéro. Pour août-septembre 1917, ne pas l'assimiler au S.VII S.254 conservé au Musée de l'Air : S18 et S19 [RÉSUMÉ] placent Guynemer sur le SPAD XIII S.504 le 20 août et le 11 septembre, et S1 ne donne pas de numéro (huitième passe).
- **Fin de carrière du S.VII** (huitième passe) : dans la compilation américaine de 1918 (S46, note de la p. 205), les deux lignes du S.VII n'ont pas l'astérisque des types encore livrés aux escadrilles ; elles font partie des types qui « figurent encore dans certaines escadrilles » mais ne sont plus fournis. Pour une mission de 1918 : S.VII possible, mais pas comme avion neuf (C ; date exacte de la compilation inconnue).

## 2. Géométrie

Quatrième passe : une description d'époque du S.VII (*Aerial Age*, reprise par *Flight* le 16 août 1917, S48) donne pour la première fois une **corde**, un **écartement**, une voie et des cotes d'empennage. L'article les dit **approximatives**, et sa corde contredit toutes les surfaces publiées (voir la ligne « Corde »). Décalage, calages, dièdre et gouvernes restent **non trouvés**. Septième passe : le détail de ce qui a été cherché, trouvé et non trouvé est au § 2.5. Les valeurs du SPAD XIII (§ 2.2) ne valent que comme **comparables** (confiance C au mieux) : Béchereau a dessiné le XIII comme une version agrandie du VII (S15, S21 [RÉSUMÉ]).

### 2.1 SPAD S.VII

| Grandeur | Valeur retenue | Unité d'origine | Source | Confiance | Remarque |
|---|---|---|---|---|---|
| Envergure du plan supérieur | 7,82 m | m | S7, S8 [RÉSUMÉ] ; 7,81 m (S6) ; 7,80 m (S9, S10) ; 25 ft 8 in (S14). **Époque** : 25,7 ft = 7,83 m sur les deux lignes « Spa. 7C1 » de S46, p. 205 [LU-IMG] ; 25,8 ft = 7,86 m (S47, p. 237) [LU-IMG] ; 25 ft 6 in = 7,77 m, « approximatif » (S48, 1917) [LU-OCR] | **A** (relevée de B) | Deux tableaux officiels d'époque à 7,83 et 7,86 m ; 7,82 m reste dans l'écart (conversions C). Les tableaux ne donnent qu'une envergure |
| Envergure du plan inférieur | **non tranchée** ; retenu provisoirement **égale au plan supérieur** | m | Égale : S48 (1917, 25 ft 6 in pour les deux plans) [LU-OCR] et S16 [RÉSUMÉ]. Plus courte : 7,61 m (S9 [RÉSUMÉ]) | C | Deux sources contre une, dont une d'époque mais « approximative ». À vérifier sur plan (P1-P3) ou sur appareil (P17) (journal) |
| Surface alaire | 17,85 m² | m² | S6, S8, S10, S33 [RÉSUMÉ] ; 18 m² (S7). **Époque** : 192 sq ft = 17,84 m² (S47, p. 237) ; 195 sq ft = 18,12 m² sur les **deux** lignes de S46, libellé et colonne (p. 205) [LU-IMG]. Les « 193 sq ft » de la quatrième passe étaient une erreur d'OCR | **A** pour 17,85 m² ; **définition inconnue** | Contrôle (C) : la charge alaire imprimée dans **les deux** tableaux, 8,08 et 8,1 lb/sq ft, correspond à 1 550 lb sur 192 sq ft (8,07), et non sur 195 sq ft (7,95). Les deux tableaux ont donc calculé avec 192 sq ft ; le 195 de S46 est soit une coquille, soit une autre définition de la surface. Ce que la surface inclut n'est précisé nulle part |
| Longueur | 6,08 m | m | S6, S7, S8 ; 6,10 m (S5) ; 6,15 m (S10) ; 19 ft 11 in (S14). **Époque** : 20 ft = 6,10 m (S46, S47) [LU-IMG] ; 20 ft 0 in (S48) | B | Les tableaux arrondissent au pied ou au dixième de pied ; écart de 2 cm, sans effet |
| Hauteur | 2,20 m | m | S6 ; 2,35 m (S8) ; 2,12 m (S10) ; 7 ft 2 in (S14). **Époque** : 7,20 ft = 2,19 m (S46, deux lignes) et 7,2 ft (S47) [LU-IMG] | **A** (relevée de B) | Position de mesure (ligne de vol ou au sol) toujours non précisée |
| Corde | **non tranchée** : 1,40 m selon S48, incompatible avec la surface ; **fourchette calculée 1,14 à 1,33 m** (septième passe) | ft-in | S48 (1917) : 4 ft 7 in = 1,397 m pour les deux plans, « approximatif » [LU-OCR] | D pour 1,40 m ; **C** pour la fourchette | **Contradiction** (C, calcul) : 2 × 7,77 × 1,397 ≈ 21,7 m² bruts, contre 17,8 à 18,1 m² dans tous les documents. **Septième passe** (§ 2.5.2) : dans la famille SPAD, la surface publiée vaut de 0,89 à 0,94 fois le rectangle brut des deux plans (XIII, S46 et S15 ; S.XI, S49). Transposé au S.VII (17,85 m², envergures égales de 7,82 m), cela donne une corde de **1,22 à 1,33 m** ; 1,14 m si la surface publiée était le rectangle brut. Une corde de 1,40 m obligerait à exclure 3,9 à 4,0 m² de la surface, plus que sur le biplace S.XI, pourtant plus grand (2,95 m²). La corde de S48 coïncide avec celle du XIII (1,400 m) : coïncidence ou confusion, non tranché. **Choix de la valeur de travail : à l'aérodynamicien** (`E:` ou `C:` avec renvoi au § 2.5.2) |
| Écartement des plans | 1,27 m | ft-in | S48 (1917) : 4 ft 2 in, « approximatif » [LU-OCR] | C | Source unique, approximative. Rapport écartement/corde ≈ 0,91 avec 1,40 m, ≈ 1,06 avec 1,2 m, ≈ 1,0 avec 1,25 m (C ; fourchette de corde au § 2.5.2). Septième passe : aucune autre source trouvée |
| Décalage horizontal | **non trouvé** (septième passe : toujours rien pour le S.VII) | — | Comparables : S.XI, 0,40 m, soit environ 26 % de sa corde supérieure (S49 [LU-OCR] ; pourcentage C) ; XIII, plan inférieur « en arrière de 1¼° » (S15 [RÉSUMÉ]) | — | Exprimé en angle, le décalage du XIII ne se convertit pas sans connaître la convention. Si c'était l'inclinaison des mâts sur la verticale, il ne ferait que 1,181 × tan 1,25° ≈ 2,6 cm (C) : valeur qui paraît trop faible, d'où un doute sur la transcription. Le S.XI est un biplace d'une autre cellule. **Rien à transposer** |
| Décalage angulaire, calages | **non trouvé** pour le VII | — | Comparables : biplace S.XI, plan supérieur 2,8° au centre et 2,5° au saumon, plan inférieur 1,5° (rapport allemand, S49) [LU-OCR] ; XIII, supérieur 1½°, inférieur 1° (S15 [RÉSUMÉ]) | C (famille) | Les deux comparables donnent un plan supérieur calé de 0,5 à 1,3° de plus que le plan inférieur (C). À ne pas transposer sans plan : autres cellules |
| Forme en plan, flèche | deux plans « presque rectangulaires », extrémités droites et non inclinées, coins légèrement arrondis ; **aucune flèche mentionnée** | — | S48 (1917) [LU-OCR] ; XIII : ni flèche ni dièdre (S15 [RÉSUMÉ]) | C | Le biplace S.XI a une flèche de 3° par côté (« 174° », S49) : propre à ce type |
| Découpe du plan supérieur | « profonde », au-dessus du siège du pilote | — | S48 [LU-OCR] | C (qualitatif) | Dimensions **non trouvées**. Elle entre sans doute dans l'écart entre surface publiée et rectangle brut (§ 2.5.2, C) |
| Dièdre | non trouvé ; **indice** de dièdre nul | — | S47 (tableau italien 1919, p. 237) qualifie le SPAD monoplace de « flat-winged » [LU-IMG] ; XIII : 0° (S15) ; S.XI : pas de dièdre (S49) | C | « Flat-winged » s'entend sans doute « ailes sans dièdre » : interprétation, à confirmer |
| Structure de voilure | une seule travée, avec des mâts intermédiaires au croisement des câbles de vol et d'atterrissage, reliés par une entretoise en tube d'acier | — | S48 (1917) : les câbles partent du fuselage vers les mâts extérieurs et se croisent aux mâts intermédiaires, où un tube d'acier relie les mâts intermédiaires avant et arrière ; S49 (rapport allemand, 1918) : les monoplaces ont des « mâts intérieurs divisés » ; S50 (1919) : schéma « familier des SPAD » [LU-OCR] ; S16, S42 [RÉSUMÉ] | **B** (qualitatif, trois descriptions d'époque) | Brevet britannique n° 100,632 de la Société Spad, « Strutting and staying of aerial machines », publié en décembre 1916 (S51) : piste pour le détail |
| Ailerons | sur le plan supérieur seulement ; étroits, articulés sur un longeron auxiliaire à environ 9 in (0,23 m) en arrière du longeron arrière ; commande par bielles logées dans le plan inférieur et renvois au pied des mâts extérieurs | — | S48 (1917, déduction de l'auteur) ; S49 (même schéma décrit sur le S.XI, non compensés) [LU-OCR] ; S41 [RÉSUMÉ] | B (qualitatif) | **Septième passe** : le texte de S48 dit *about 9 ins. back of the main beam*. Lire « longeron arrière » est une interprétation du contexte (la phrase précédente parle de la surface située en arrière des longerons arrière), et l'auteur déduit cette disposition du dessin (*must be carried*) : ce n'est pas une mesure. Envergure, corde, surface et débattements du S.VII : **non trouvés**. Comparable XIII : 2,222 × 0,495 m par aileron (§ 2.2, C) |
| Empennage, gouvernes | gouvernail articulé à environ 10 in (0,25 m) au-delà de l'extrémité du fuselage ; dérive avançant de 12 in (0,30 m) devant le bord d'attaque du plan fixe ; plan fixe et profondeur « habituels » | in | S48 (1917) [LU-OCR] | C | Sur le S.XI de la même famille, le plan fixe est cambré sur ses deux faces, calé à 0° par rapport au longeron supérieur, profondeur en deux parties et guignols à l'intérieur du fuselage et de la dérive ; le rapport dit ces surfaces « du type SPAD habituel » (S49). **Septième passe** : envergure et corde du plan fixe, corde de la profondeur, hauteur et cordes de la dérive et du gouvernail, surfaces, calage, bras de levier et débattements du S.VII : **non trouvés**. Comparables chiffrés du XIII au § 2.2 (C au mieux) |
| Débattements des gouvernes | **non trouvés** | — | aucun SPAD (VII, XIII ou XI) dans les sources lues ou résumées | — | À estimer (`E:`) |
| Fuselage | « exceptionnellement profond » ; fond arrondi sous les longerons inférieurs, comme les flancs et le dessus : section arrondie sur les quatre faces | — | S48 (1917) [LU-OCR] ; S49 (S.XI : quatre longerons, dessus et dessous arrondis par des couples et des lisses) [LU-OCR] | C (qualitatif) | Largeur et hauteur maximales, longueur du fuselage seul : **non trouvées** |
| Capot et radiateur | capot avant en aluminium ; ouverture **circulaire** du radiateur, rappelant le capot d'un moteur rotatif ; bossages latéraux des carters d'arbres à cames ; perforations d'aération | — | S48 [LU-OCR] ; S49 (S.XI : radiateur à volets formant le nez) | C (qualitatif) | Diamètre et surface frontale du radiateur : **non trouvés** |
| Positions du pilote, du moteur, du réservoir, de la mitrailleuse | moteur à l'avant derrière le radiateur ; mitrailleuse au-dessus du moteur ; réservoir principal sous le siège | — | S48 ; § 7 ; S13 [RÉSUMÉ] pour le réservoir ; S.XI : réservoir sous pression formant le siège (S49) | C (qualitatif) | **Aucune cote de position trouvée** |
| Voie du train | 1,52 m | ft | S48 (1917) : 5 ft [LU-OCR] | C | XIII : 1,492 m (S15). Essieu coulissant dans des glissières, amortisseurs en sandow (S48) |
| Profil | section nommée **« Spad »** dans le NACA Report 93 (MIT, 1918) : épaisseur ≈ 5,4 %, cambrure ≈ 3,8 %, intrados creux | % de corde | `profil-spad7.md` § 2 [LU] | **B** | Type de SPAD mesuré non précisé. Polaires : calcul calé sur le RAF 15 de vraie grandeur (A pour ce profil, C pour le SPAD) |

### 2.2 Comparable : SPAD S.XIII C.1 (cotes publiées d'après un exemplaire américain)

Source : S15 [RÉSUMÉ], *This Day in Aviation*, d'après l'exemplaire 17956 essayé à McCook Field (projet P-154, 1921). Le rapport original (Air Service Information Circular vol. III n° 286, 1er octobre 1921) n'a pas été consulté (P12). Les numéros de paragraphe cités dans les résumés (1-10, 1-11, 4-3…) laissent penser que le site reprend la structure de la circulaire. Septième passe : ajout des cotes de S15 vues en résumé le 2026-10-22, et des deux lignes « Spa. 13C1 » du tableau français de 1919 (S46, p. 205), **lues sur l'image**. **Pour le S.VII, toutes ces valeurs sont des comparables : confiance C au mieux.**

| Grandeur (XIII) | Valeur | Unité d'origine | Remarque |
|---|---|---|---|
| Envergure (deux plans, égale) | 26 ft 3¾ in (8,020 m) | ft-in | Envergure et corde égales sur les deux plans |
| Corde | 4 ft 7-1/8 in (1,400 m) | ft-in | |
| Écartement | 3 ft 10½ in (1,181 m) | ft-in | |
| Décalage | plan inférieur en arrière de 1¼° | degrés | Décalage exprimé en angle ; convention à vérifier |
| Calage | supérieur 1½°, inférieur 1° | degrés | soit un décalage angulaire de +0,5° (C) |
| Dièdre | 0° | degrés | |
| Longueur hors tout | 20 ft 4 in (6,198 m) | ft-in | Libellée « longueur du fuselage » dans les passes précédentes. S46 imprime 20,35 ft dans sa colonne « Length » (longueur hors tout) : c'est donc la **longueur totale**, pas celle du fuselage seul (C). Longueur du fuselage seul : non trouvée |
| Envergure, longueur, hauteur, surface (tableau français de 1919) | 26,3 ft (8,02 m) ; 20,35 ft (6,20 m) ; 7,55 ft (2,30 m) ; 215 sq ft (19,97 m²) | ft, sq ft | S46, p. 205, sur les **deux** lignes « Spa. 13C1 » (8Ba de 220 ch et 8BEc de 235 ch) [LU-IMG]. Envergure et longueur concordent avec S15 : deux sources indépendantes. Charge alaire imprimée 8,42 lb/sq ft, cohérente avec 1 815 lb sur 215 sq ft (8,44, C) |
| Surface (McCook) | 227 sq ft (21,089 m²) | sq ft | S15 [RÉSUMÉ]. 5,6 % de plus que S46 : les deux documents officiels ne définissent pas la surface de la même façon (C). Voir § 2.5.2 |
| Plan fixe horizontal | envergure 10 ft 2 in (3,099 m) ; corde maximale 1 ft 8¾ in (0,527 m) | ft-in | S15 [RÉSUMÉ]. Le résumé ne dit pas si cette corde comprend la profondeur. Rectangle englobant 3,099 × 0,527 = 1,63 m², borne haute (C). Profondeur : **aucune cote** dans le résumé |
| Ailerons | plan supérieur seulement ; envergure 7 ft 3½ in (2,222 m) ; corde 1 ft 7½ in (0,495 m) | ft-in | S15 [RÉSUMÉ]. Envergure **par aileron** (lecture probable du texte). Rectangle englobant 1,10 m² par aileron, 2,20 m² pour les deux, soit environ 10 % de 21,09 m² (C) |
| Gouvernail de direction | hauteur 3 ft 10-5/8 in (1,184 m), corde maxi 2 ft 2 in (0,660 m) | ft-in | Rectangle englobant 0,78 m², borne haute (C) |
| Dérive | hauteur « 2 feet, 7/8-inch (0.876 m) » ; longueur maximale 3 ft 11¼ in (1,200 m) | ft-in | **Incohérence de hauteur** maintenue : 2 ft 7/8 in = 0,632 m, alors que 0,876 m = 2 ft 10½ in. La longueur de 1,200 m est cohérente (C). À vérifier sur l'original (P12) |
| Voie du train | 4 ft 10¾ in (1,492 m) | ft-in | Roues : pneumatiques, sandows, béquille fixe (S15 [RÉSUMÉ]) ; dimensions des roues **non trouvées** |
| Masses (XIII) | 1 464 lb à vide (664 kg), 2 036 lb en charge (924 kg) | lb | Exemplaire américain de 1921. Le moteur de cet essai serait un « Wright » de 220 hp : un article d'*Aerial Age* du 19 décembre 1921 s'intitule « Performance test of Spad 13 equipped with 220-horsepower Wright engine » (S56 [LU-OCR], titre seulement ; P25) |
| Masses (tableau français de 1919) | 1 255 lb (569 kg) à vide ; 1 815 lb (823 kg) en charge | lb | S46 [LU-IMG]. Les composantes de la charge (242 + 176 + 132 + 11 = 561 lb) dépassent de 5 lb la charge totale imprimée (556 lb) ; 1 255 + 561 = 1 816 ≈ 1 815 (C) |
| Envergure et hauteur (exemplaire « Smith IV », NASM) | 8,2 m (26 ft 11 in) ; 2,4 m | m | S65 [RÉSUMÉ]. **Diverge** de S15 et S46 (8,02 m) : mode de mesure inconnu ; non retenu |
| Centrage, inertie, débattements | **non trouvés** | — | Seul indice : Meister le dit « lourd de la queue » (T8, qualitatif) |
| Rapports S.VII / XIII dans le même tableau (S46) | envergure 0,977 ; longueur 0,983 ; hauteur 0,954 ; surface 0,893 (192/215) | — | C, calcul. La surface diminue plus que l'envergure : à définition égale, la corde du S.VII serait d'environ 0,893/0,977 ≈ 0,91 fois celle du XIII, soit environ 1,28 m (même hypothèse qu'au § 2.5.2) |

### 2.3 Traînées parasites (inventaire pour le modèle de vol)
Éléments à modéliser, d'après les descriptions : mâts interplans **et mâts intermédiaires** (S16), câbles de vol et d'atterrissage, tringlerie d'ailerons extérieure à renvois (attestée sur le XIII), radiateur frontal du V8 à refroidissement liquide, train fixe, mitrailleuse au-dessus du moteur (§ 7), tête du pilote. **Aucune dimension ni aucun coefficient n'a été trouvé.** Tout devra être calculé (Hoerner) et marqué `C:`, ou estimé (`E:`) et justifié par l'aérodynamicien.

**Septième passe : inventaire précisé** (toujours **sans aucune dimension** pour le S.VII) :

| Élément | Ce qui est établi | Source, statut | Confiance | Non trouvé |
|---|---|---|---|---|
| Mâts | par côté, deux mâts extérieurs (avant et arrière) et deux mâts intermédiaires reliés par une entretoise en tube d'acier ; soit **4 mâts interplans et 4 mâts intermédiaires** pour l'avion | S48 [LU-OCR] ; S49 (« mâts intérieurs divisés » des monoplaces) ; décompte déduit (C) | B (disposition), C (décompte) | cordes, épaisseurs et sections des mâts ; mâts de cabane (nombre et forme : non décrits) |
| Câbles | câbles partant des flancs du fuselage vers les mâts extérieurs, croisés aux mâts intermédiaires | S48 [LU-OCR] | B (disposition) | nombre exact, diamètres, **type** (câbles profilés de type « RAF wires » ou ronds) : non trouvé pour le S.VII. Comparable S.XI : câbles porteurs **doublés**, l'intervalle rempli de lattes de bois pour réduire la traînée ; câbles de retour simples (S49) |
| Tringlerie d'ailerons | bielles logées dans le plan inférieur, renvois au pied des mâts extérieurs, biellettes verticales jusqu'aux ailerons | S48 ; S49 (S.XI) [LU-OCR] | B (qualitatif) | dimensions des biellettes extérieures |
| Train | jambes fixes ; essieu coulissant dans des glissières orientées vers le haut et l'arrière, dans l'axe des jambes arrière ; sandows ; voie 5 ft | S48 [LU-OCR] | C | dimensions des roues et des jambes. Comparable S.XI : jambes en V en bois lamellé entoilé, demi-essieux entre deux tubes transversaux entoilés, débattement 125 mm (S49). La légende du dessin coté du S.XI porte une dimension de roues illisible dans l'OCR (« 700 × 7… ») : **à lire sur l'image** (§ 9.6) |
| Radiateur et capot | ouverture circulaire dans un capot en aluminium ; bossages des arbres à cames ; perforations | S48 [LU-OCR] | C (qualitatif) | diamètre, surface frontale, volets éventuels sur le S.VII (le S.XI a des volets, S49) |
| Fuselage | section arrondie sur les quatre faces, « exceptionnellement profond » | S48 [LU-OCR] | C (qualitatif) | largeur, hauteur, longueur seule, ouverture du poste |
| Mitrailleuse, pare-brise | Vickers au-dessus du moteur (§ 7) | S5, S6 [RÉSUMÉ] | C | dimensions du pare-brise et du carénage |

### 2.4 Interférence biplan et stabilité : ce qui servira au modèle (troisième passe) [LU]

**Portée.** Aucune de ces mesures ne porte sur un SPAD. Ce sont des lois de cellules biplans mesurées ou calculées sur d'autres avions. **Confiance A pour l'objet essayé, C pour le SPAD.** Les cotes de cellule du S.VII restent inconnues (§ 2.1). À titre d'ordre de grandeur, le XIII a un rapport écartement/corde de 1,181/1,400 ≈ 0,84 (calcul C, § 2.2).

**Portance de la cellule**
- **Facteurs de cellule mesurés** (S43 : maquette de Curtiss JN-2, profil Eiffel 36, écartement/corde ≈ 1, MIT, été 1917). Le facteur est la portance du biplan rapportée à deux fois celle d'un plan seul.
  - Aux incidences de vol, il vaut de **0,82 à 0,94**, avec un minimum vers 8° (p. 290). Le résumé donne 0,82 à 0,86 aux faibles incidences (p. 339).
  - Pour le **C_L max**, les auteurs recommandent **0,95** pour un écartement/corde voisin de 1 et un décalage de 10 à 25 % (p. 290, 339).
  - D'après des essais de Hunsaker et du NPL cités p. 290, passer d'un écartement/corde de 1,2 à 1 fait perdre environ 5 % ; un décalage de 20 % fait gagner environ 2 %.
- **Pentes de cellule calculées et recoupées par l'expérience** (S45, p. 659). Plans égaux, allongement 6, écartement/corde 1, sans décalage angulaire :
  - RAF 15 : **3,57** sans décalage, **3,65** à 29° de décalage ;
  - Clark Y : 3,30 et 3,59 ;
  - Göttingen 387 : 3,27 et 3,40.

  L'unité n'est pas écrite dans le tableau ; les valeurs sont cohérentes avec une pente par radian. Selon l'auteur, la théorie et l'expérience diffèrent de moins de 1 %.
- **Recoupement** (C, calcul fait ici). En monoplan d'allongement 6, corrigé de l'effet de paroi, le RAF 15 a une pente d'environ 0,0747/° ≈ 4,28/rad (`profil-spad7.md` § 4.2). Le rapport 3,57/4,28 ≈ **0,83** concorde avec les 0,82 à 0,86 de S43 aux faibles incidences : deux sources indépendantes, deux profils, même ordre de grandeur.
- **Lois utiles de S45** (p. 658-661) :
  - la cambrure ne change pas sensiblement la pente de la cellule ;
  - pour un profil peu cambré comme le RAF 15, la cellule garde l'**incidence de portance nulle du profil**. Elle devient plus négative quand la cambrure augmente : Clark Y −5,8° seul, environ −6,4° en biplan. **À retenir pour le « Spad », plus cambré que le RAF 15** ;
  - un décalage de −15° à +30° est sans effet sensible sur l'incidence de portance nulle et a un petit effet sur la pente (pente minimale pour un décalage nul ou faible) ;
  - un décalage angulaire de −2° à +4° (essais sur USA 27) ne change pas la pente. La **méthode simple de Glauert** (ARC R&M 901, 1923) suffit donc pour des plans égaux, avec ou sans décalage angulaire, sauf en cas de décalage négatif ;
  - un décalage positif augmente la part de portance du plan supérieur. Un écartement plus faible accentue toutes les interférences ;
  - la théorie ne vaut qu'au-dessus d'un écartement/corde d'environ 0,75. **Le SPAD est près de cette limite** s'il ressemble au XIII ;
  - pour chaque plan, on peut garder la relation centre de poussée / C_L du monoplan ;
  - pour la traînée induite, S45 renvoie à la formule de Prandtl des multiplans. Supposer les portances des deux plans proportionnelles à leurs surfaces ne coûte qu'environ 3 % d'erreur dans le pire cas étudié (p. 661).

**Traînée induite de la cellule : le facteur σ de Prandtl (S61, quatrième passe) [LU, tableaux vérifiés sur l'image]**

Source : L. Prandtl, NACA TN 182 (1924), traduction d'un article des *Technische Berichte* de Göttingen (vol. III, n° 7, p. 309-315).

**Attention à la notation** (sixième passe) : le NACA TR 416 emploie aussi la lettre σ, mais pour un tout autre facteur, celui de la traînée induite d'une aile **rectangulaire**. Dans nos fiches, ce dernier est noté **δ** (`profil-spad7.md` § 4.2). Ici, σ désigne seulement le facteur biplan de Prandtl.
- **Hypothèses** (p. 1, 5-6) : portance de chaque plan répartie en demi-ellipse le long de l'envergure ; plans sans décalage horizontal ; G = écartement vertical ; b₁, b₂ = envergures ; r = b₂/b₁ ; q = ρV²/2 (pression dynamique, définie p. 12 par l'exemple numérique).
- **Formules** (p. 6 et 9) : traînée propre d'un plan D₁₁ = L₁²/(π q b₁²) ; traînée mutuelle D₁₂ = D₂₁ = σ L₁ L₂ /(π q b₁ b₂) ; traînée induite totale du biplan D = [L₁²/b₁² + 2σ L₁L₂/(b₁b₂) + L₂²/b₂²]/(π q) (équation 7).
- **Décalage horizontal** (p. 9) : il ne change pas la traînée induite **totale**. Avec un décalage positif, la traînée du plan supérieur diminue et celle du plan inférieur augmente d'autant.
- **Tableau I (p. 6-7)**, σ en fonction de G rapporté à l'envergure moyenne (b₁ + b₂)/2 :

| G / b moyenne | 0 | 0,05 | 0,10 | 0,15 | 0,20 | 0,25 | 0,30 | 0,35 | 0,40 | 0,45 | 0,50 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| r = 1,0 | 1,000 | 0,780 | 0,655 | 0,561 | 0,485 | 0,420 | 0,370 | 0,327 | 0,290 | 0,255 | 0,230 |
| r = 0,8 | 0,800 | 0,690 | 0,600 | 0,523 | 0,459 | 0,401 | 0,355 | 0,315 | 0,282 | 0,252 | 0,225 |
| r = 0,6 | 0,600 | 0,540 | 0,485 | 0,437 | 0,394 | 0,351 | 0,315 | 0,285 | 0,255 | 0,231 | 0,210 |

  Lecture à l'image ; l'OCR NTRS est inutilisable pour ce tableau. Contrôles (C) : la formule (5) ci-dessous redonne 0,291 à G/b = 0,40 (tableau : 0,290) ; le tableau II redonne κ = (1 + σ)/2 pour r = 1 à 0,002 près sur toutes les colonnes vérifiées.
- **Formules approchées pour des envergures égales** (p. 7) : σ ≈ 1/(1 + 5,3 G/b), valable de G/b = 1/15 à 1/4 ; plus exacte, σ ≈ (1 − 0,66 G/b)/(1,05 + 3,7 G/b), valable de 1/15 à 1/2. Pour des envergures inégales, la formule (6), p. 7, corrige σ à partir de la valeur à envergure moyenne ; exemple chiffré p. 8.
- **Répartition optimale et rendement** (p. 10, 13) : la part de portance du plan inférieur qui minimise la traînée est x = (r − σ)/(r + 1/r − 2σ) ; pour r = 1, x = 1/2. Le rapport κ = D/D_monoplan (monoplan de même portance et d'envergure b₁) est tabulé (tableau II, p. 13) ; pour des envergures égales, κ = (1 + σ)/2 (équation 9a).

**Application au S.VII** (C, calcul fait ici, sur des cotes elles-mêmes incertaines) :
- avec l'écartement approximatif de S48 (1,27 m) et des envergures égales de 7,82 m : G/b ≈ 0,162, d'où **σ ≈ 0,54** (tableau interpolé : 0,543 ; formule exacte : 0,541) et **κ ≈ 0,77** ;
- avec un plan inférieur de 7,61 m (S9) : r ≈ 0,97 et G/b moyenne ≈ 0,165, d'où un σ presque identique ;
- comparable du XIII (1,181 m sur 8,02 m, § 2.2) : G/b ≈ 0,147, σ ≈ 0,57.

L'écartement du S.VII n'étant connu qu'à « environ » près, **σ reste en confiance C**. Une erreur de 10 cm sur l'écartement change σ d'environ 0,02 (C).

**Effet du décalage horizontal sur la portance : NACA TN 70 (S63, cinquième passe) [LU, tableau vérifié sur l'image]**

F. H. Norton publie en septembre 1921 des essais faits au MIT en **1918** pour l'US Air Service. Le dispositif : un plan supérieur USA 15 et un plan inférieur RAF 15, maquettes de 3 × 18 in, écartement égal à la corde, 13,4 m/s (très bas Reynolds), décalage de −100 % à +100 % (p. 2). Les coefficients sont modernes (C_L = L/qS, q = ½ρV², note de la p. 2). Le tableau 1 (p. 3) donne les facteurs par lesquels multiplier la portance du **monoplan** (moyenne des deux profils seuls) pour obtenir celle du biplan :

| Incidence | +50 % | +25 % | 0 | −25 % | −50 % |
|---|---|---|---|---|---|
| 0° | 0,95 | 0,91 | 0,86 | 0,81 | 0,79 |
| 4° | 0,89 | 0,87 | 0,87 | 0,86 | 0,86 |
| 8° | 0,89 | 0,87 | 0,85 | 0,84 | 0,85 |
| 12° | 0,89 | 0,87 | 0,85 | 0,84 | 0,82 |
| 14° | 0,96 | 0,92 | 0,88 | 0,85 | 0,82 |
| 16° | 1,09 | 1,05 | 1,01 | 0,97 | 0,93 |

Le tableau va de −2° à 16° et donne aussi des facteurs de finesse. À −2°, près de la portance nulle, les facteurs n'ont pas de sens physique simple (valeurs de −1,35 à 1,75).

Conclusions de l'auteur (p. 3) : un décalage positif augmente le rendement et la portance maximale, et réduit fortement le déplacement du centre de poussée. À décalage nul, le facteur de **0,85 à 0,88** concorde avec les 0,82 à 0,86 de S43 (C).

**Pour le S.VII**, le décalage horizontal reste **inconnu** : le tableau ne donne donc qu'une fourchette (C). D'un décalage nul à +25 %, le facteur varie de 0 à 6 % selon l'incidence entre 0° et 14° (C, lu dans le tableau).

**Dérive et empennage : allongement effectif (S62, quatrième passe) [LU, p. 257 vérifiée sur l'image]**. P. E. Hemke (NACA Report 267) traite des plaques d'extrémité sur une aile. Pour une plaque de hauteur totale 2h à chaque extrémité d'une aile d'envergure b, la traînée induite est multipliée par R ≈ 1/(1 + 1,66 × 2h/b), soit un allongement effectif accru du facteur 1/R (approximation tirée de Nagel, p. 257). Appliquer ce résultat à une dérive dont le pied touche le fuselage ou le plan fixe est une **analogie** que le rapport lu ne fait pas lui-même ; elle relève de l'aérodynamicien (C).

**Décrochage de la cellule (S44)**

Essais de pression sur maquettes des ailes du chasseur PW-9 : profil Göttingen 436, environ 30 m/s, de −6° à +24°, plans essayés seuls puis en cellule (p. 3 et 11 à 13).
- L'interférence touche presque uniquement l'intrados du plan supérieur et l'extrados du plan inférieur.
- Le plan supérieur décroche **à la même incidence** que seul. Le plan inférieur décroche **plus tard**, avec un maximum un peu plus élevé et **sans chute brutale**.
- La pente et le maximum de la cellule sont inférieurs à la somme des deux plans seuls.
- Aux grandes incidences, les centres de poussée **avancent** sur les deux plans.

Conséquence possible (C, raisonnement, à faire émerger et non à scripter) : si le modèle traite les deux plans séparément, le décrochage de la cellule commence par le plan supérieur. Si ce plan est en avant du centre de gravité, la perte de portance peut donner une **abattée**, ce qui rejoint la réputation (§ 6.2). L'intensité dépend du décalage et des calages, inconnus pour le S.VII.

**Traînée et stabilité (S43, p. 339-340)**
- En présence des ailes, la traînée du fuselage et du train vaut environ les **trois cinquièmes** de celle de ces éléments essayés seuls.
- L'empennage fournit **82 %** de l'amortissement en tangage du JN-2. La théorie usuelle n'en prédit qu'environ les deux tiers. La règle des auteurs, pour une estimation préliminaire : calculer l'amortissement de l'empennage et le compter pour **55 %** de celui de l'avion complet, soit un total d'environ 1,8 fois la valeur calculée (C, conversion faite ici). Cela sert de **contrôle de vraisemblance** de la dérivée M_q, pas de valeur SPAD.
- Le centrage du JN-2 (centre de gravité de 4 à 18 in en avant du centre de poussée moyen des ailes, p. 339) lui est propre et **n'est pas transposable**.

**Hélice et ailes** : sur le VE-7, la présence des ailes réduit d'environ 5 % le rendement propulsif maximal (`hispano-suiza-8a.md` § 7.4).

**Ce qui manque** : le décalage, les calages et une corde fiable du S.VII ; l'écartement n'est connu que de façon approximative (1,27 m, S48, C). Sans eux, ces lois ne donnent aucun chiffre propre au SPAD. Le volume I du rapport ACA 1917-18 (non téléchargé, § 9.6) contient un essai d'« effet biplan sur le profil RAF 15 » et un essai sur l'effet d'une **découpe dans le plan supérieur** d'un biplan, cas du SPAD (titres lus dans la table des matières du volume II). Pour la cellule RAF 15, S45 cite des données expérimentales dans les ARC R&M 774, 816, 857 et 997 et le NACA TR 256 ; ce sont des pistes, non lues.

### 2.5 Septième passe : géométrie de détail (2026-10-22)

**Objet.** Réunir les cotes de détail qui manquent pour écrire `assets/aircraft/spad7/aircraft.ron` (S.VII 150 ch, Hispano-Suiza 8Aa, 703 kg) : empennage, gouvernes, cellule, fuselage, traînées, centrage. **Tout ce qui est marqué « non trouvé » ici est à estimer (`E:`) par l'aérodynamicien.** Méthode et sources fouillées : § 0.

**Bilan.** Aucune cote chiffrée nouvelle du S.VII n'a été trouvée, ni dans les documents locaux ni sur le web. Cette passe apporte :
- des **descriptions qualitatives** du S.VII tirées de S48 (relu en entier) : forme en plan, découpe, position relative des longerons, haubanage, section du fuselage, capot et radiateur, train ; elles sont reportées au § 2.1 et au § 2.3 ;
- des **comparables chiffrés du SPAD XIII** : empennage horizontal, ailerons, dérive et surface de McCook (S15, en résumé) ; envergure, longueur, hauteur, surface et masses du tableau français de 1919 (S46, **lu sur l'image**). Ils sont au § 2.2 ;
- des **comparables de famille** tirés du rapport allemand sur le biplace S.XI (S49), recoupé par S46 (§ 2.5.1) ;
- un **contrôle par le calcul** qui encadre la corde du S.VII (§ 2.5.2).

#### 2.5.1 Comparable de famille : biplace SPAD S.XI (S49 [LU-OCR], recoupé par S46 [LU-IMG])

Avion examiné : n° B 6006, construit sous licence par Bernard en juillet 1917, Hispano de 200 hp à réducteur. **Autre cellule, autre masse** : ces valeurs ne servent qu'à connaître les habitudes de construction de Béchereau, jamais à remplir une cote du S.VII.

| Grandeur (S.XI) | Valeur | Remarque |
|---|---|---|
| Envergures | supérieure 11,22 m ; inférieure 10,90 m | S46 (p. 204) : 36,8 ft = 11,22 m, concordant |
| Cordes | supérieure 1,53 m ; inférieure 1,43 m | cordes **inégales** sur ce type |
| Écartement | 1,335 m | |
| Décalage horizontal | 0,40 m | environ 26 % de la corde supérieure (C) |
| Calages | supérieur 2,8° au centre, 2,5° au saumon ; inférieur 1,5° | déjà au § 2.1 |
| Flèche, dièdre | flèche des deux plans, « 174° » (soit 3° par côté, C) ; pas de dièdre | |
| Surface totale | 29,8 m² (texte du rapport) | S46 : 323 sq ft = 30,0 m², écart 0,7 % (C) |
| Construction des plans | longerons du plan supérieur plus rapprochés que ceux du plan inférieur ; mâts convergeant vers le haut ; plan inférieur échancré près du fuselage, plan supérieur découpé au centre ; nervures espacées de 190 mm (supérieur) et 175 mm (inférieur) | |
| Empennage | surfaces « du type SPAD habituel » ; plan fixe traversant le fuselage, cambré sur ses deux faces, calé à 0° par rapport au longeron supérieur ; profondeur en deux parties articulée sur un tube d'acier ; guignols à l'intérieur du fuselage et de la dérive | **Indice de famille** le plus direct pour le S.VII, dont S48 dit l'empennage « habituel » |
| Ailerons | non compensés, sur faux longerons du plan supérieur | |
| Câbles et mâts | câbles porteurs doublés, carénés par des lattes de bois ; câbles de retour simples ; mâts en bois profilé, ferrures sur un tube d'acier intérieur | |
| Train | jambes en V en bois lamellé entoilé ; demi-essieux entre deux tubes entoilés ; débattement 125 mm | |
| Masses par élément (avion vide, eau comprise : 765 kg) | moteur 225,0 kg ; eau 33,0 kg ; ailes 167,9 kg ; profondeur et gouvernail 19,9 kg ; fuselage et reste 319,2 kg | La valeur est mal lue par l'OCR (« rg.9 ») ; elle est restituée par différence, 765 − 225,0 − 33,0 − 167,9 − 319,2 = 19,9, ce qui concorde avec le « .9 » lisible (C). Proportions de la masse à vide (C) : moteur 29 %, eau 4 %, ailes 22 %, empennage mobile 2,6 %, fuselage et reste 42 % |
| Légendes du dessin coté | illisibles dans l'OCR : on devine une cote « Tail plane, 250 m/m … line of thrust », des surfaces par plan dont la somme ne retombe pas sur 29,8 m² à la lecture OCR, et une dimension de roues « 700 × 7… » | **Rien n'en est retenu** ; image de la page à demander (§ 9.6) |

#### 2.5.2 La corde du S.VII : encadrement par les surfaces de la famille (C, calcul)

**Principe.** Une surface publiée n'est presque jamais le rectangle brut « envergure × corde » des deux plans : on en retire la partie couverte par le fuselage, les échancrures et la découpe, et les conventions varient. Trois cas de la famille permettent de mesurer cet écart (calculs faits ici) :

| Avion et document | Rectangle brut des deux plans | Surface publiée | Rapport | Partie exclue |
|---|---|---|---|---|
| XIII, tableau français (S46) | 2 × 8,020 × 1,400 = 22,46 m² (cotes S15) | 215 sq ft = 19,97 m² | 0,89 | 2,48 m² |
| XIII, McCook (S15) | 22,46 m² | 227 sq ft = 21,09 m² | 0,94 | 1,37 m² |
| S.XI, rapport allemand (S49) | 11,22 × 1,53 + 10,90 × 1,43 = 32,75 m² | 29,8 m² | 0,91 | 2,95 m² |

**Application au S.VII** (17,85 m², envergures égales de 7,82 m) :
- si la surface publiée était le rectangle brut : corde de **1,14 m** ;
- en transposant la **proportion** exclue (rapport 0,94, 0,91 ou 0,89) : **1,22, 1,25 ou 1,28 m** ;
- en transposant la **valeur absolue** exclue (1,37 à 2,95 m²) : **1,23 à 1,33 m** ;
- avec l'envergure de S48 (7,77 m), toutes ces cordes augmentent de 0,6 % ;
- recoupement par une autre voie, sous la même hypothèse de définition : dans le même tableau S46, la surface du S.VII vaut 0,893 fois celle du XIII, pour une envergure égale à 0,977 fois celle du XIII ; à définition égale, sa corde serait d'environ 0,91 × 1,40 ≈ **1,28 m** (§ 2.2).

**Ce que cela dit de 1,40 m (S48).** Il faudrait exclure 3,9 à 4,0 m² de la surface (rapport 0,82), soit plus que sur le biplace S.XI, pourtant plus grand (2,95 m²). La valeur reste donc en **D**. Elle n'est pas impossible pour autant : la découpe du S.VII est dite « profonde » et la définition de sa surface est inconnue.

**Conclusion.** Corde **plausible de 1,14 à 1,33 m** ; vers 1,25 à 1,28 m si la surface de 17,85 m² suit la convention des documents français de la famille. **Confiance C**. Le rapport écartement/corde serait alors voisin de 1,0 (1,27/1,25, C). Le facteur σ de Prandtl ne dépend pas de la corde (§ 2.4). L'écart de 5,6 % entre deux surfaces officielles du même XIII montre qu'on ne resserrera pas cette fourchette sans un plan coté (P1 à P3, P19) ou une mesure sur un appareil conservé (P17). **Le choix de la valeur de travail revient à l'aérodynamicien.**

#### 2.5.3 Ce qui reste non trouvé pour le S.VII

| Rubrique | Non trouvé | Meilleur comparable disponible |
|---|---|---|
| 1. Empennage | envergure et corde du plan fixe ; surfaces du plan fixe et de la profondeur ; corde de la profondeur ; hauteur, cordes et surfaces de la dérive et du gouvernail ; position par rapport aux ailes (bras de levier) ; calage du plan fixe | XIII (§ 2.2, C) ; calage de 0° sur le S.XI (famille, C) |
| 2. Ailerons et gouvernes | envergure, corde et surface des ailerons ; **débattements de toutes les gouvernes** (pour aucun SPAD) | ailerons du XIII (§ 2.2, C) |
| 3. Cellule | décalage horizontal ; calages ; corde réelle (fourchette C au § 2.5.2) ; envergure du plan inférieur (toujours non tranchée, § 2.1) ; dimensions de la découpe | calages du XIII et du S.XI (§ 2.1, C) |
| 4. Fuselage | largeur et hauteur maximales ; longueur du fuselage seul ; dimensions du radiateur ; positions du pilote, du moteur, du réservoir et de la mitrailleuse | aucun (formes qualitatives seulement, § 2.1) |
| 5. Traînées | dimensions des mâts ; nombre exact, diamètre et type des câbles ; roues ; jambes de train | S.XI (§ 2.5.1), qualitatif |
| 6. Centrage et inertie | position du centre de gravité ; toute mesure d'inertie. Ni le rapport britannique n° 96 (P10), ni le rapport de McCook (P12), ni une notice n'ont pu être lus | répartition des masses du S.XI (§ 2.5.1), pour une estimation par composants (C) |

Documents qui lèveraient ces manques : § 9.6 (lignes de la septième passe).

### 2.6 Neuvième passe : le bord d'attaque des ailes (2026-10-23)

**Question de l'aérodynamicien.** Le bord d'attaque du S.VII est-il coffré en contreplaqué jusqu'au longeron avant, ou entoilé directement ? Un terme de traînée du modèle est calé sur un avion américain de 1922 à bord d'attaque en contreplaqué.

**Réponse : non trouvé pour le S.VII.** Aucun document d'époque lu ne décrit le bord d'attaque d'un S.VII. S48 (1917), relu en entier, n'en parle pas. Indices, du plus fort au plus faible :

| Indice | Ce qu'il dit | Avion | Source, statut | Portée pour le S.VII |
|---|---|---|---|---|
| Rapport allemand sur un biplace capturé (1918) | nervures espacées de 190 mm (plan supérieur) et 175 mm (plan inférieur) ; **entre les nervures, de fausses nervures sur l'extrados, du bord d'attaque au longeron avant** ; toile clouée sur des nervures renforcées, cousue aux nervures et au fil de bord de fuite. **Aucune mention de contreplaqué** sur la voilure, alors que le rapport en signale dans le fuselage | S.XI (B 6006, juillet 1917) | S49, *Flight*, 16 mai 1918, p. 528 [LU-OCR] | famille SPAD, autre cellule. Des fausses nervures limitées à l'extrados vont avec un bord d'attaque entoilé, soutenu entre les nervures ; elles n'excluent pas un coffrage mince, que le rapport aurait pu passer sous silence (C) |
| Réplique d'aujourd'hui | « les bords d'attaque de toutes les ailes sont recouverts de contreplaqué » | SPAD XIII | Memorial Flight, S71 [RÉSUMÉ] | autre type ; constructeurs de répliques volantes (rang 3) |
| Revues de maquettes | coffrage en contreplaqué au bord d'attaque, sur l'**extrados seulement** | SPAD XIII | Hyperscale, revues de maquettes Eduard, S72 [RÉSUMÉ] | maquettes du commerce : exclues comme source de chiffre ; simple indice |
| Forum de constructeurs | la plupart des constructeurs (« Albatros, SPAD, Nieuport ») coffraient l'**extrados**, du bord d'attaque au longeron avant, en contreplaqué de bouleau trois plis de 1,5 à 2,0 mm | non précisé | The Aerodrome, S73 [RÉSUMÉ] | forum : piste |
| Forum de constructeurs (William Retzlaff) | bord d'attaque du XIII : baguette triangulaire en épicéa avec deux lattes de contreplaqué de bouleau de 1,5 mm ; un participant écrit que le S-7 « n'est pas construit ainsi », sans que le résumé dise sur quel point ; il ne subsisterait qu'environ 25 % des plans d'usine du S-7 | XIII, S-7 | The Aerodrome, S74 [RÉSUMÉ] | ambigu |

**Bilan.**
- Coffrage en contreplaqué de l'**extrados** seul, du bord d'attaque au longeron avant : **plausible, C**. Il est attesté pour le XIII par deux sources modernes faibles, et attribué aux SPAD en général par un forum.
- Bord d'attaque **entoilé** sur fausses nervures : **plausible aussi, C**. C'est la seule description d'époque pour la famille (S.XI).
- **Non tranché.** Dans tous les cas, l'intrados est entoilé : aucune source ne parle de contreplaqué sous l'aile.
- **Pour le modèle** : rien n'autorise à dire que le S.VII avait le même bord d'attaque que l'avion américain de 1922 ; rien ne l'exclut non plus. Le terme de traînée calé sur cet avion reste un comparable (C), à signaler comme tel dans les données.

**Pour trancher** (§ 9.6) :
- les plans d'usine du S.VII : jeux de calques attribués à Roger Freeman, copies chez Leo Opdycke, selon S74 [RÉSUMÉ] ;
- le Windsock Datafile n° 8 (P1) ;
- une photographie d'époque d'un S.VII entoilé ou en cours de réparation ;
- un relevé sur un appareil conservé (P17), sous réserve des restaurations.

## 3. Masses et centrage

| Grandeur | Valeur | Conditions | Source | Confiance | Remarque |
|---|---|---|---|---|---|
| Masse à vide | 500 kg | S.VII 150 ch | S9 [RÉSUMÉ] ; S5 [RÉSUMÉ], qui la tire de Davilla et Soltan (P6). **Époque** : 1 100 lb = 499 kg sur les **deux** lignes « Spa. 7C1 » de S46 (p. 205) et sur la ligne SPAD de S47 (p. 237) [LU-IMG] | **A** (relevée de B) | Deux tableaux officiels et une source secondaire concordent |
| Masse au décollage | **703 kg** (1 550 lb) ; 705 kg (704,7 kg) dans les ouvrages | S.VII 150 ch | **Époque** : 1 550 lb = 703 kg sur la ligne 8Aa de S46 (p. 205), 1 551 lb sur la ligne SPAD de S47 (p. 237) [LU-IMG]. Ouvrages : S5, S9 [RÉSUMÉ] | **A** pour 703 kg (deux tableaux d'époque) ; 705 kg : B | 705 kg = 1 554 lb, soit 0,3 % d'écart. Les tableaux en livres arrondissent peut-être une valeur française de 705 kg (C), mais rien ne le prouve : la valeur imprimée, 703 kg, est retenue. Les 694 kg de la quatrième passe venaient d'une erreur d'OCR (1 530 lu pour 1 550) : **retirés** |
| Masse au décollage | 703 à 705 kg (1 550 lb) | S.VII **180 ch** (ligne 8Ab de S46) | S46, p. 205 [LU-IMG] | **B** | Les deux lignes de S46 portent **exactement les mêmes masses**, au détail près. Le compilateur a donc pu reprendre pour le 180 ch les masses du 150 ch. Or le même numéro donne un Hispano de 180 hp plus lourd de 55 lb (25 kg) à sec que le 150 hp britannique (`hispano-suiza-8a.md` § 6). D'où B, et la variante de 740 kg ci-dessous |
| Masse à vide | 510 kg | moteur **non précisé** | S6, S33 [RÉSUMÉ] | C | |
| Masse au décollage | 740 kg (1 632 lb) | moteur non précisé (« avec armement », S6) | S6, S33 [RÉSUMÉ] | C | Gardée en **variante** pour le 180 ch ; 37 kg de plus que S46, origine inconnue |
| Masse au décollage | 692 kg (1 525 lb), charge utile 213 kg (470 lb) | « S.VII » de 1917, moteur annoncé de « 160 hp » | S48 [LU-OCR] | C | Valeurs « approximatives » d'un article de presse américain |
| Masse au décollage | 705 kg « avec un 8-Aa de 180 ch » | — | S14 [RÉSUMÉ] | D | Fiche de musée qui confond 8Aa et 180 ch |
| Répartition de la charge (S46, p. 204-205) | charge totale 451 lb (205 kg) = essence **et** huile 176 lb (80 kg) + équipage 176 lb (80 kg) + armement 88 lb (40 kg) + divers 11 lb (5 kg) ; identique sur les deux lignes | lb | S46 [LU-IMG] (intitulés des colonnes lus p. 204) | A pour les chiffres imprimés | **Correction** : en quatrième passe, j'avais pris la colonne « essence et huile » pour l'essence seule et les 11 lb « divers » pour l'huile. Somme exacte (C) |
| Répartition de la charge (S47, p. 237) | charge totale 451 lb = essence et huile 231 lb (105 kg) + équipage 165 lb (75 kg) + armement et munitions 55 lb (25 kg) | lb | S47 [LU-IMG] | A pour les chiffres imprimés | Même total que S46, autre répartition : pilote plus léger, plus d'essence. Somme exacte (C) |
| Carburant | 135 L au total : réservoir principal dans le fuselage sous le siège, réservoir d'appoint dans la partie centrale (cabane) | — | S13 [RÉSUMÉ] | C | 105 L dans une source de jeu (exclue, S39). Essence et huile : 80 kg (S46) ou 105 kg (S47) dans la charge des tableaux ; à 0,72 kg/L, 105 kg d'essence seule feraient environ 146 L (C). Contenances non lues |
| Huile | non trouvé séparément | — | — | — | Comprise dans « essence et huile » (S46, S47) |
| Munitions | non trouvé (nombre de cartouches) | — | — | — | Armement : 40 kg (88 lb) dans S46 ; armement et munitions : 25 kg (55 lb) dans S47 |
| Centrage, et son évolution | **non trouvé** (septième passe) | — | Ni le rapport britannique n° 96 (P10), ni le rapport de McCook sur le XIII (P12, S15 en résumé), ni une notice SPAD n'ont pu être lus ; aucun résumé n'en cite un centrage. Seul indice, qualitatif et pour le XIII : « lourd de la queue » (T8) | — | À estimer (`E:`). Le centrage du JN-2 (S43) n'est pas transposable (§ 2.4) |
| Inertie | **non trouvé** | — | Comparable de famille pour une estimation par composants : répartition de la masse à vide du biplace S.XI (moteur 29 %, eau 4 %, ailes 22 %, empennage mobile 2,6 %, fuselage et reste 42 %), S49 [LU-OCR], § 2.5.1 | C (famille) pour les proportions | À estimer par composants (`fdm_lab inertia`). Le S.XI a un moteur à réducteur de 225 kg et une cellule plus grande : proportions indicatives seulement. Positions du moteur, du pilote, du réservoir et de la mitrailleuse : non trouvées (§ 2.5.3) |

## 4. Moteur et hélice (résumé ; détail dans [`hispano-suiza-8a.md`](hispano-suiza-8a.md))
- 150 ch : Hispano-Suiza 8Aa ; 180 ch : Hispano-Suiza 8Ab. V8 à refroidissement liquide, entraînement **direct** de l'hélice.
- **Puissance** (troisième passe) : la forme de la courbe puissance/régime, la loi d'altitude et la consommation d'un Hispano de 180 hp (même alésage, course et compression que le 8Ab) ont été mesurées par le Bureau of Standards (NACA Report 46, lu). Confiance **B pour un 8Ab**. Détail et journal dans `hispano-suiza-8a.md` § 3 à 5.
- **Cote du 8Ab** (quatrième et cinquième passes) : la contradiction 1 700 / 2 100 tr/min s'explique par **plusieurs conventions de cote**. Le tableau officiel des moteurs (*Flight*, 13 février 1919, p. 201) cote le Hispano de 180 hp **américain** à 180 hp « normaux » à 1 700 tr/min et 205 hp au maximum à 1 800 tr/min ; celui de la **section britannique** à 220 hp à 2 000 tr/min et 225 hp à 2 100 tr/min ; le **français** à 208 hp à 2 000 tr/min et 225 hp à 2 100 tr/min [LU-IMG]. Le moteur du Report 46, « coté 180 hp à 1 700 tr/min », suit la convention américaine. Détail : `hispano-suiza-8a.md` § 3.2.
- **Puissance maximale autorisée sur le S.VII** (S46, p. 205, colonne « Maximum Power Permitted ») [LU-IMG] : **175 hp à 1 700 tr/min** pour le 8Aa, **205 hp à 1 800 tr/min** pour le 8Ab. C'est la puissance que les compilateurs associent aux performances de chaque ligne (A pour la valeur imprimée).
- **Régime en palier à pleins gaz** (S46, p. 205) [LU-IMG] : 8Aa, **1 720, 1 680 et 1 640 tr/min** à 2 000, 3 000 et 4 000 m ; 8Ab, **1 870, 1 830, 1 780 et 1 710 tr/min** à 2 000, 3 000, 4 000 et 5 000 m. A. **Correction** : en quatrième passe, j'avais attribué au 180 ch les régimes du 150 ch. Ces régimes sont précieux pour caler le pas de l'hélice (`hispano-suiza-8a.md` § 7.4).
- **Loi d'altitude du 150 ch** (quatrième passe) : un essai britannique **en vol** (R&M 462, mars 1918) sur un Hispano de 150 hp sans réducteur, monté sur S.E.5, donne une puissance proportionnelle à la pression à la puissance 1,04 environ, indépendante de la température (exposant lu dans l'OCR, à confirmer). Le tableau américain de 1919 **estime** au contraire une perte voisine de la densité (`hispano-suiza-8a.md` § 4.3 et 4.4).
- Hélice bipale en bois à pas fixe. Selon des forums [RÉSUMÉ], diamètre de 2,35 à 2,45 m et pas de 1,75 à 2,10 m pour le 150 ch. Rien de fiable pour le 180 ch. **Table C_T/C_P mesurée** sur une hélice en bois comparable (Navy D' du VE-7 à moteur Wright de 180 hp, 2,39 m, P/D 0,8) : A pour la mesure, **C pour le SPAD** (`hispano-suiza-8a.md` § 7.4).
- **Sens de rotation** : **horaire vu du siège du pilote** (confiance C), déduit de sources concordantes qui ne sont pas primaires. Troisième passe : les hélices du VE-7 portent « Rotation: Right hand » (NACA Report 301, p. 5). L'indice concorde, mais il ne relève pas la confiance, faute de définition de la convention et d'identification certaine du moteur. Conséquence attendue (C) : à pleine puissance et à basse vitesse, l'avion tend à partir à gauche, ce qui demande du pied à droite. C'est l'inverse du SPAD XIII à réducteur, pour lequel McCook note qu'il faut beaucoup de pied à gauche (S15).
- **Neuvième passe (2026-10-23), hélice** (détail : `hispano-suiza-8a.md` § 7.5) :
  - **Liste française** : sept hélices autorisées pour le « Type VII · Hispano 150 ch », vues en résumé dans le dossier de Hartmann : Levasseur 485 et 561, Gallia 3, Éclair 100 et 6, Chauvière 2066, Régy 353 (autorisation annulée). Elles vont de 2,35 à 2,50 m de diamètre et de 1,75 à 2,05 m de pas (P/D de 0,74 à 0,82).
  - **Référence retenue** : Levasseur 485, 2,45 × 1,95 m, C. La table de D' en soufflerie reste la base aérodynamique ; deux tables mesurées en vol sur VE-7 sont ajoutées comme variantes.
  - **Non trouvés** : l'hélice de l'avion de S46, la définition française du pas, et un régime relevé en montée sur S.VII (§ 5.6).

## 5. Performances

**Cinquième passe : le tableau officiel lu sur l'image (S46, *Flight*, 13 février 1919, p. 204-205) [LU-IMG].**
- **Origine** : tableau « French Aeroplanes in Service at the Front », compilé par le service aéronautique américain, publié par *Automotive Industries* et repris par *Flight*. *Flight* écrit seulement que ces tableaux « paraissent » établis officiellement et qu'il en serait « apparemment » redevable au ministère américain ; il nomme la revue *Automobile Industries* (présentation en tête de la **p. 201**, et non p. 199 comme écrit en cinquième passe ; correction de la huitième passe). Les altitudes rondes en pieds (6 500, 9 800, 13 100 et 16 400 ft) sont la conversion de 2 000, 3 000, 4 000 et 5 000 m : les données d'origine sont métriques, donc françaises (C).
- **Colonnes** (en-têtes p. 204) : puissance maximale autorisée ; vitesse en palier en mph à 6 500, 9 800, 13 100 et 16 400 ft ; durée de montée jusqu'aux mêmes altitudes ; plafond **théorique** ; heures de vol à pleine puissance au niveau du sol ; armement ; répartition de la charge ; masses ; dimensions. Le régime est imprimé sous chaque ligne (§ 0).
- **Lignes** (p. 205, en haut) : « Spad (195 sq. ft.), Spa. 7C1, Hispano 8Aa » et « Spad (195 sq. ft.), Spa. 7C1, Hispano 8Ab ». La seconde n'était pas à « 193 sq ft » : erreur d'OCR.
- **Les deux séries de montée du STAé citées par le forum S11 se retrouvent à l'identique dans S46** pour le 150 ch : 6 min 40 s, 11 min 30 s, 19 min 30 s et un plafond théorique de 18 000 ft, soit les 5 500 m de S11 convertis. La transmission par forum est donc fidèle. **Correction (huitième passe)** : l'identité des quatre chiffres montre une **origine commune** ; ce n'est pas une confirmation indépendante de la mesure (§ 5.5.1).

**Tableau italien (S47, *Flight*, 20 février 1919, p. 237) [LU-IMG]** : « Spad One Seater, flat-winged », moteur « H.S. » de **140 hp** « normaux ». Ses colonnes sont explicites : vitesse au sol et à 6 500 ft (2 000 m), montées à 1 000, 2 000, 3 000 et 4 000 m. Toutes les sommes et tous les quotients de la ligne se vérifient (C). La puissance de 140 hp, inhabituelle, n'est pas expliquée ; la ligne est gardée comme **variante italienne**, sans l'attribuer au 8Aa ou au 8Ab.

### 5.1 S.VII 150 ch (Hispano-Suiza 8Aa)

| Essai | Valeur | Conditions (altitude, masse) | Source | Confiance |
|---|---|---|---|---|
| Vitesse en palier à **2 000 m** | **192 km/h (119,5 mph)** à 1 720 tr/min | 703 kg (1 550 lb) ; puissance maximale autorisée 175 hp à 1 700 tr/min | S46, p. 205, ligne 8Aa [LU-IMG] ; 192 km/h aussi dans S5 [RÉSUMÉ] | **A** (relevée de B). Lecture « 5 » confirmée en sixième passe par l'arrondi au demi-mph de tout le tableau (§ 0) ; arrondi d'impression ±0,4 km/h |
| Vitesse en palier à **3 000 m** | **179 km/h (111,5 mph)** à 1 680 tr/min | idem | S46 [LU-IMG] | **A** ; lecture confirmée de même (§ 0), ±0,4 km/h |
| Vitesse en palier à **4 000 m** | **174 km/h (108 mph)** à 1 640 tr/min | idem | S46 [LU-IMG] | **A** |
| Vitesse à 5 000 m | non donnée (« .. ») | — | S46 | — |
| Vitesse max (valeur mal attribuée) | 212 km/h (132 mph) à 2 000 m | S9 l'attribue au 150 ch | S9 [RÉSUMÉ] | **Réattribuée au 180 ch** (§ 5.2) |
| Montée à 2 000 m | **6 min 40 s** | 703 kg | S46 [LU-IMG] ; STAé via S11 [RÉSUMÉ] | **A** (relevée de C) |
| Montée à 3 000 m | **11 min 30 s** | idem | S46 [LU-IMG] ; 11 min 20 s ou 11 min 30 s selon S11 et S12 | **A** ; la variante 11 min 20 s est écartée |
| Montée à 4 000 m | **19 min 30 s** | idem | S46 [LU-IMG] ; S11 | **A** |
| Plafond théorique | **18 000 ft (5 486 m)**, soit 5 500 m arrondis | idem | S46 [LU-IMG] ; S11 (5 500 m) | **A** |
| Heures de vol à pleine puissance au niveau du sol | **2 h 10** | idem | S46 [LU-IMG] | A (définition du tableau) |
| Variante italienne (S47) | vitesse au sol **199 km/h (123,5 mph)**, à 2 000 m **194 km/h (120,5 mph)** ; montée à 1 000 m en 2 min, à 2 000 m en 5 min 50 s, à 3 000 m en 9 min 50 s ; 2 h d'endurance à régime maximal au sol | 703 kg (1 551 lb) ; « H.S. » de 140 hp | S47, p. 237 [LU-IMG] | A pour les chiffres imprimés ; **sous-version incertaine** |
| Montée à 6 500 ft (1 981 m) | 6,3 min | rapport britannique n° 96, avril 1917 | S11 [RÉSUMÉ] | C |
| Montée à 10 000 ft (3 048 m) | 11,5 min | idem | S11 [RÉSUMÉ] | C |
| Montée à 15 000 ft (4 572 m) | 24,5 min | idem | S11 [RÉSUMÉ] | C |
| Plafond estimé | 19 000 ft (5 791 m) | idem ; c'est une **estimation** du rapport, jugée « probablement un peu faible » | S11 [RÉSUMÉ] | C |
| Montée à 2 000 m (autre valeur) | 4,5 min | **prototype SPAD V** | S5 [RÉSUMÉ] | D pour le S.VII de série |
| Plafond pratique | 5 335 m (17 500 ft) | — | S5, S33 [RÉSUMÉ] | C |
| Endurance | 1,5 à 2 h | — | S13 [RÉSUMÉ] | C |
| Montée en 10 min | 9 300 ft (2 835 m) | « S.VII » de S48, moteur annoncé de « 160 hp », 692 kg | S48 [LU-OCR] | C : sous-version incertaine ; tombe entre les séries officielles du 150 ch et du 180 ch |
| Vitesse de décrochage ou d'atterrissage | non trouvé | — | — | — |

Taux de montée moyens déduits (C : différences de temps, valeurs de S46) : 5,0 m/s de 0 à 2 000 m ; 3,4 m/s de 2 000 à 3 000 m ; 2,1 m/s de 3 000 à 4 000 m. Les valeurs britanniques (S11, C) donnent 5,2, 3,4 et 2,0 m/s sur des tranches voisines.

### 5.2 S.VII 180 ch (Hispano-Suiza 8Ab)

Toutes les valeurs A de ce tableau viennent de la ligne « Spa. 7C1 / Hispano 8Ab » de S46, p. 205 [LU-IMG]. La masse de la ligne est de 703 kg (1 550 lb, confiance B, § 3) et la puissance maximale autorisée de 205 hp à 1 800 tr/min.

| Essai | Valeur | Conditions | Source | Confiance |
|---|---|---|---|---|
| Vitesse en palier à **2 000 m** | **212 km/h (132 mph)** à 1 870 tr/min | voir ci-dessus | S46 [LU-IMG] ; S9 [RÉSUMÉ] (« 212 km/h à 2 000 m ») | **A** (relevée de B). **L'altitude est tranchée : 2 000 m.** Les « 132 mph au niveau de la mer » de S48 étaient faux |
| Vitesse en palier à **3 000 m** | **203 km/h (126 mph)** à 1 830 tr/min | idem | S46 [LU-IMG] ; S48 | **A** |
| Vitesse en palier à **4 000 m** | **200 km/h (124,5 mph)** à 1 780 tr/min | idem | S46 [LU-IMG] | **A** |
| Vitesse en palier à **5 000 m** | **187 km/h (116 mph)** à 1 710 tr/min | idem | S46 [LU-IMG] | **A** |
| Vitesse max (autres valeurs) | 208 km/h (129 mph), altitude non précisée (S5) ; 188 km/h (source russe citée par S12) | — | [RÉSUMÉ] | C (208) ; D (188) |
| Montée à 2 000 m | **4 min 40 s** | idem | S46 [LU-IMG] ; un participant du forum S11 [RÉSUMÉ] | **A** (relevée de D : je l'avais écartée à tort) |
| Montée à 3 000 m | **8 min 10 s** | idem | S46 [LU-IMG] | **A**. Les 9 min 50 s de S12 sont ceux de la ligne **italienne** de S47, pas du 180 ch français |
| Montée à 4 000 m | **12 min 40 s** | idem | S46 [LU-IMG] | **A** |
| Montée à 5 000 m | non donnée (« .. ») | — | S46 | — |
| Plafond théorique | **21 400 ft (6 523 m)** | idem | S46 [LU-IMG] | **A**. S7 donne 6 200 m, sans dire s'il s'agit du plafond pratique (C) ; les 6 553 m des jeux (S39) sont proches du chiffre officiel, origine inconnue (D) |
| Heures de vol à pleine puissance au niveau du sol | **1 h 30** | idem | S46 [LU-IMG] | A (définition du tableau) |
| Endurance réelle en croisière | non trouvé | — | — | — |
| Décrochage | non trouvé | — | — | — |

Taux de montée moyens déduits (C) : 7,1 m/s de 0 à 2 000 m ; 4,8 m/s de 2 000 à 3 000 m ; 3,7 m/s de 3 000 à 4 000 m.

### 5.3 Vitesse de décrochage : seul indice
Selon un fil de forum qui reprend un récit attribué à Frank Tallman, le décrochage moteur réduit d'un S.VII à moteur Wright-Hispano E de 180 ch se produit « vers 47 mph » (75,6 km/h, conversion C). Masse inconnue, fin des années 1950 ou années 1960 (S31 [RÉSUMÉ]). Confiance **D** : attribution incertaine, avion et moteur postérieurs.

### 5.4 Cibles proposées pour `essais.ron` (à valider par l'aérodynamicien)
> **Huitième passe : pour le 150 ch, ce tableau est remplacé par le § 5.5.6** (confiance déclassée à 3 000 et 4 000 m, tolérances justifiées, plafond à vitesse ascensionnelle nulle). Pour le 180 ch, il reste valable avec les tolérances du § 5.5.6.

Cinquième passe : **cibles de confiance A**, toutes tirées de S46, p. 205, lu sur l'image.

| Version | Masse | `MaxLevelSpeed` | Montée | Plafond théorique | Contrôle secondaire (régime à pleins gaz) |
|---|---|---|---|---|---|
| **150 ch (8Aa)** | 703 kg (A ; 705 kg des ouvrages, écart 0,3 %) | 192 km/h à 2 000 m ; 179 km/h à 3 000 m ; 174 km/h à 4 000 m | 2 000 m en 6 min 40 s ; 3 000 m en 11 min 30 s ; 4 000 m en 19 min 30 s | 5 486 m | 1 720 / 1 680 / 1 640 tr/min |
| **180 ch (8Ab)** | 703 kg (**B** : masse peut-être reprise du 150 ch) | 212 km/h à 2 000 m ; 203 km/h à 3 000 m ; 200 km/h à 4 000 m ; 187 km/h à 5 000 m | 2 000 m en 4 min 40 s ; 3 000 m en 8 min 10 s ; 4 000 m en 12 min 40 s | 6 523 m | 1 870 / 1 830 / 1 780 / 1 710 tr/min |

Précautions :
- Le tableau associe ces performances à une **puissance maximale autorisée** de 175 hp à 1 700 tr/min (8Aa) et 205 hp à 1 800 tr/min (8Ab). Les régimes en palier dépassent légèrement ces régimes. Le modèle doit retrouver vitesse **et** régime avec la même hélice : c'est un contrôle fort du couple puissance-hélice (`hispano-suiza-8a.md` § 7.4).
- Conditions atmosphériques des essais : **non données**. Les altitudes sont sans doute des altitudes d'essai françaises, en mètres (C). Faute de mieux, les comparer à l'atmosphère type et le signaler `E:`.
- Pour le 180 ch, les montées dépendent directement de la masse, qui n'est qu'en B. Prévoir une variante à 740 kg (§ 3) pour voir la sensibilité.
- Variante italienne (S47, « H.S. » de 140 hp, 703 kg) : 194 km/h à 2 000 m et une montée à 3 000 m en 9 min 50 s. Essai non bloquant, sous-version incertaine.
- **Conséquence pour le M1** : la définition de « terminé » (cibles A et B dans les tolérances) est désormais **atteignable** pour les deux versions.

### 5.5 Huitième passe : fiabilité des performances du S.VII 150 ch (2026-10-22)

**Objet.** Juger les performances d'époque du 150 ch sur leurs seuls mérites de sources, puis fixer les cibles d'essai. **Pour le 150 ch, cette section remplace le tableau du § 5.4.** Documents relus et limites : § 0 (huitième passe).

#### 5.5.1 D'où viennent les chiffres

| Série | Transmission | Indépendance | Pour | Contre |
|---|---|---|---|---|
| **S46**, ligne « Spa. 7C1 / Hispano 8Aa » | essais français (mètres, km/h), compilation du service aéronautique américain, revue new-yorkaise (*Automobile Industries* selon *Flight*, sans doute *Automotive Industries*, C), puis *Flight* du 13 février 1919, p. 204-205 | série de référence | document d'époque ; colonnes définies ; régimes imprimés | troisième main ; *Flight* dit seulement que les tableaux « paraissent » officiels (p. 201) ; ni date, ni lieu, ni conditions d'essai. Les lignes du S.VII n'ont pas l'astérisque des types encore livrés (note p. 205) : la compilation date de 1918, les essais du 150 ch sans doute de 1916-1917 (C) |
| « Résumé du STAé » cité par S11 | forum, vu en résumé | **aucune** à l'égard de S46 : mêmes quatre chiffres (6 min 40 s, 11 min 30 s, 19 min 30 s, 5 500 m), donc même origine (C) | prouve que le forum transmet fidèlement | ne confirme pas la mesure. Corrige le § 5, qui la disait confirmée par un document indépendant |
| **Rapport britannique n° 96** (avril 1917) | forum S11, vu en résumé | **indépendant** : essai britannique, en pieds, plafond « estimé » par le rapport | seule série indépendante de S46 | chiffres non vérifiés [RÉSUMÉ] ; rapport non lu (P10) |
| **S47**, ligne « Spad One Seater, flat-winged » | données italiennes, même compilation américaine, *Flight* du 20 février 1919, p. 237 | **partielle** : même masse totale que S46, à 1 lb près (1 551 et 1 550 lb), mais autre répartition de la charge et autres montées | vitesses au sol et à 2 000 m | sous-version incertaine (§ 5.5.4) ; série de montée irrégulière (§ 5.5.3) ; pas de plafond (colonne « Ceiling, Calc. » vide) |
| **S48** (*Aerial Age*, juillet 1917, repris par *Flight*) | presse américaine | indépendante, valeurs dites approximatives | — | vitesses identiques à celles de la ligne du **180 ch** de S46 ; montée proche du 150 ch ; moteur annoncé de « 160 hp » : mélange de versions (C) |
| S5 (192 km/h) | wiki | dérivée | — | S5 rattache 192 km/h au prototype SPAD V [RÉSUMÉ] (§ 5.5.5) |

**Indices tirés du tableau S46 lui-même** (C, calculs faits ici) :
- **Unités d'origine.** Les vitesses imprimées se retrouvent en convertissant des km/h entiers (192, 179 et 174 pour le 8Aa ; 212, 203, 200 et 187 pour le 8Ab), avec 1 mph ≈ 1,609 km/h et un arrondi au demi-mph ; 179 et 212 tombent à la limite de l'arrondi. Les originaux étaient donc sans doute des km/h entiers : l'origine française et les valeurs du § 5.1 sont confortées.
- **Masse conventionnelle.** L'équipage (176 lb, 80 kg) et l'armement (88 lb, 40 kg) sont les mêmes pour tous les monoplaces du tableau. La charge du SPAD fait exactement 205 kg (80 + 80 + 40 + 5). S47 arrive aussi à 205 kg, réparti autrement (105 + 75 + 25). La masse de 703 kg est donc une **charge de référence**, pas la pesée de l'avion essayé.
- **Colonnes dérivées recopiées.** La « charge par hp » vaut 9,04 lb/hp sur **les deux** lignes du S.VII. Elle ne correspond ni à 175 hp (8,86) ni à 205 hp (7,56), mais à environ 171 hp. Sur les deux lignes du XIII, la même colonne est calculée avec 220 hp et recopiée. C'est une raison de plus de garder les masses du 180 ch en B (§ 3).

#### 5.5.2 Conditions de mesure

| Condition | Ce que disent les documents | Conséquence |
|---|---|---|
| Début du chronométrage de la montée (lâcher des freins, décollage, passage à 0 m en vol) | **Rien**, ni dans S46 et S47, ni dans les tableaux britannique et américain voisins (S66, S67) | Inconnu. Calculer les deux conventions (§ 5.5.6) |
| Atmosphère et définition de l'altitude | Rien. Altitudes françaises en mètres ronds, sans doute lues au barographe (C). L'atmosphère type n'existait pas. Côté britannique, l'usage de 1917 était de comparer les essais **à densité égale** ; R&M 462 (mars 1918 ; `hispano-suiza-8a.md`, H21) montre que cette réduction pénalise les essais faits par temps froid et propose de réduire en pression (§ 18 à 21 et annexe II) [LU-OCR] | Les altitudes de S46 et du rapport n° 96 ne sont peut-être pas définies de la même façon. Écart possible de quelques pour cent, **non chiffrable** sans les documents |
| Masse | Charge de référence de 205 kg, total 703 kg (§ 5.5.1) | Masse réelle de l'avion essayé inconnue |
| Moteur | 8Aa ; puissance maximale autorisée 175 hp à 1 700 tr/min ; régimes en palier (S46). S47 : « H.S. » de 140 hp « normaux » | Puissance réelle de l'exemplaire essayé inconnue |
| Hélice | Non donnée | À estimer (`hispano-suiza-8a.md` § 7) |
| Mesure de la vitesse | Méthode non donnée | — |
| Version | S46 : 8Aa explicite ; S47 : incertaine | S47 en variante seulement |

> **Dixième passe** : chronométrage, zéro des altitudes, atmosphère de référence et terrain sont repris au § 5.7, qui ajoute la pratique britannique lue (S75) et les conventions à calculer.

#### 5.5.3 Cohérence interne et entre tableaux

**a) Vitesses du 150 ch dans S46** (192, 179 et 174 km/h à 2 000, 3 000 et 4 000 m).
- **Question de l'aérodynamicien.** Si toute la traînée était de forme (puissance nécessaire ∝ σV³), passer de 192 km/h à 2 000 m à 179 km/h à 3 000 m demanderait **27 %** de puissance utile en moins (0,903 × 0,932³ ≈ 0,73 ; calcul refait). La mesure en vol R&M 462 ne donne que **12 %** de perte à régime égal ((p/p₀)^1,04 : 0,682/0,777 = 0,878), **14 %** avec la baisse de régime imprimée (1 720 puis 1 680 tr/min, à couple constant).
- **Nuance.** La traînée induite croît avec l'altitude. Avec une traînée induite d'environ 12 % de la traînée totale à 2 000 m, la perte nécessaire tombe vers **21 %**. Hypothèses de cette estimation : κ ≈ 0,77 (§ 2.4), coefficient d'Oswald de 0,8 et rendement d'hélice de 0,70 à 0,75, tous deux supposés (E). La perte disponible n'est que de **14 à 17 %**, en comptant 1 à 3 % de rendement d'hélice perdu (E). L'incohérence demeure.
- **En vitesse.** Avec la loi R&M 462 et ces hypothèses, 192 km/h à 2 000 m appellent **184 à 186 km/h à 3 000 m**. Inversement, 179 km/h à 3 000 m appellent environ **185 km/h à 2 000 m**. L'écart entre les valeurs imprimées est donc de **3 à 4 %**, soit 6 à 7 km/h.
- Les valeurs à 3 000 et 4 000 m sont **cohérentes entre elles** : depuis 179 km/h, on attend 170 à 176 km/h à 4 000 m selon la part de traînée induite ; le tableau donne 174.
- **Dans le même tableau**, entre 2 000 et 3 000 m, le 8Aa perd 6,7 % de vitesse et le 8Ab 4,5 %. Tous les autres monoplaces de S46 perdent de 0,7 à 2,3 % : XIII, 1,5 et 0,7 % ; Nieuport 24, 1,4 % ; Nieuport 28, 1,2 % ; Morane AI, 2,3 % ; monoplaces « à l'essai », de 1,0 à 1,9 %. Dans le tableau américain S67, le S.E.5 britannique à moteur Hispano perd 2,5 % entre 6 500 et 10 000 ft, avec pourtant une baisse de régime plus forte que celle du SPAD (2 080 puis 2 000 tr/min). **La ligne du 8Aa est la plus atypique des chasseurs du tableau.** Dans S67, seul le VE-7, avion d'entraînement plus lent, perd autant (5,8 %).
- **Recoupement avec S47.** Depuis 192 km/h à 2 000 m, le même calcul attend environ **200 km/h au sol**, avec un régime au sol supposé de 1 760 tr/min (E). S47 donne 199 km/h au sol et 194 km/h à 2 000 m. **Trois valeurs forment donc un groupe cohérent (192 et 194 km/h à 2 000 m, 199 km/h au sol), et 179 et 174 km/h en forment un second, plus lent de 3 à 4 %.**
- Ces calculs reposent sur une polaire et un rendement d'hélice supposés : ce sont des **ordres de grandeur** (C). Le calcul exact revient à l'aérodynamicien.

**b) Vitesses du 180 ch** (pour mémoire). Par la même méthode, 212 km/h à 2 000 m appellent environ 205 km/h à 3 000 m, pour 203 imprimés : 1 % d'écart. La lecture de S48 (212 km/h « au niveau de la mer ») cadre moins bien : on attendrait alors environ 198 km/h à 3 000 m. La lecture de S46 (212 km/h à 2 000 m) est donc maintenue (C).

**c) Montées du 150 ch.**
- **S46.** Une vitesse ascensionnelle qui décroît linéairement avec l'altitude et s'annule entre 5 300 et 5 500 m reproduit les trois temps à ±2 % près (C). La série est régulière.
- **Rapport n° 96** (S11 [RÉSUMÉ]) : 6 500 ft en 6,3 min, 10 000 ft en 11,5 min, 15 000 ft en 24,5 min. Ramené aux altitudes de S46 (C, interpolation), il est plus rapide de **4,5 % à 2 000 m, 2 % à 3 000 m et 2 à 6 % à 4 000 m**. Si les chiffres du forum sont exacts, deux essais indépendants, de deux pays, concordent donc à mieux de 6 % près.
- **S47** : 1 000 m en 2 min, 2 000 m en 5 min 50 s, 3 000 m en 9 min 50 s. Cette série est **irrégulière**. La moyenne est de 8,3 m/s jusqu'à 1 000 m, puis de 4,3 et 4,2 m/s : une vitesse ascensionnelle presque constante de 1 000 à 3 000 m n'est pas vraisemblable pour un moteur atmosphérique (C). Le temps à 1 000 m est anormalement court sur d'autres lignes du même tableau (S.V.A. : 10 m/s, puis 5,9 ; Hanriot-Dupont : 7,7 m/s, puis 5,6) : c'est un trait du tableau, pas du SPAD. S47 est plus rapide que S46 de **12,5 % à 2 000 m et de 14,5 % à 3 000 m**.
- **S48** (1917, « 160 hp ») : 9 300 ft en 10 min, soit 5 à 6 % mieux que S46 et le rapport n° 96 à la même altitude (C).

**d) Plafond « théorique ».**
- S46 ne le définit pas. Le tableau britannique du même numéro définit à part un **plafond pratique** (« service ceiling »), où la vitesse ascensionnelle tombe à 100 ft/min (S66). S47 intitule sa colonne de plafond « Ceiling, Calc. ». Le plafond « théorique » de S46 est donc, selon toute vraisemblance, l'altitude où la vitesse ascensionnelle s'annule, **obtenue par extrapolation ou par calcul**, et non atteinte en vol (interprétation C).
- Extrapolations linéaires des montées de S46 (C) : 5 100 m à partir des tranches 2 000-4 000 m, 5 800 m à partir des tranches 0-3 000 m, 5 300 à 5 500 m sur l'ensemble. Le rapport n° 96 estimait 19 000 ft (5 791 m), « probablement un peu faible » (S11 [RÉSUMÉ]).
- **Fourchette : 5 100 à 5 800 m.**

**e) Régimes.** Ils sont réguliers (−40 tr/min par 1 000 m), sans anomalie visible. Ils accompagnent les vitesses de la même ligne et partagent donc leur incertitude à 3 000 et 4 000 m.

**f) Endurance « à pleine puissance au niveau du sol »** (2 h 10). Cette grandeur est sans doute **calculée** à partir de l'essence emportée et de la consommation, et non mesurée en vol (C). Elle n'est pas une cible.

#### 5.5.4 Ce que représente le « 140 hp » de S47

- Le même moteur « H.S. » de 140 hp équipe dans S47 un hydravion F.B.A. C'est donc la **cote italienne** d'un type de Hispano, pas une puissance mesurée sur ce SPAD. La « charge par hp » de la ligne (11,0 = 1 551/140) est calculée avec elle.
- La colonne « Normal h.p. » de S47 ne reprend pas les cotes françaises usuelles : elle donne par exemple 88 pour des Le Rhône et 143 pour des Salmson. Sa définition n'est pas imprimée.
- 140 hp est la cote du premier Hispano 8A de 1915 (`hispano-suiza-8a.md` § 1, H1 [RÉSUMÉ]). Une cote « normale » à régime réduit est aussi plausible : le tableau américain des moteurs cote le Hispano de 150 hp à 150 hp « normaux » à 1 450 tr/min (p. 201).
- Les premiers S.VII italiens (77a et 91a Squadriglie, mars à mai 1917, S24 [RÉSUMÉ]) sont antérieurs à la généralisation du 8Ab.
- **Conclusion (C).** S47 décrit très probablement un S.VII de la **classe 150 ch** (8A ou 8Aa), pas un 180 ch. Mais il monte 12 à 15 % plus vite que la ligne française, et rien dans les documents lus ne l'explique : ni moteur exact, ni masse pesée, ni méthode italienne de mesure. S47 reste une **variante non bloquante**.

#### 5.5.5 Valeurs calculées, extrapolées ou composites

- **Extrapolés** : les plafonds « théorique » (S46) et « estimé » (rapport n° 96). **Calculée** : l'endurance au sol.
- **Mesurées, selon toute apparence** : les vitesses et les montées. Leurs écarts irréguliers d'une altitude à l'autre plaident contre un calcul, qui donnerait une courbe lisse (C).
- **Ligne composite possible** (piste, non vérifiée) : S5 rattache 192 km/h au prototype SPAD V [RÉSUMÉ]. Si la vitesse à 2 000 m de S46 venait d'un autre essai que les vitesses à 3 000 et 4 000 m et les montées, l'incohérence du § 5.5.3 a) s'expliquerait. Seuls les procès-verbaux d'essais (§ 5.5.7) peuvent trancher.

#### 5.5.6 Cibles recommandées pour `essais.ron` (S.VII 150 ch, 8Aa, 703 kg)

| Essai | Cible | Source | Confiance | Tolérance | Bloquant au M1 | Ce qui justifie la tolérance |
|---|---|---|---|---|---|---|
| `MaxLevelSpeed` à 2 000 m, avec régime | **192 km/h** ; 1 720 tr/min | S46 ; S47 concordant (194 km/h) | **A** | ±3 % ; régime ±3 % | oui | S46 et S47 diffèrent de 1 % ; arrondi négligeable ; vitesse au sol de S47 cohérente |
| `MaxLevelSpeed` à 3 000 m, avec régime | **179 km/h** ; 1 680 tr/min | S46 seul | **B** (déclassée de A) | **±5 %** ; régime ±3 % | oui | source unique, incohérente de 3 à 4 % avec la valeur à 2 000 m (§ 5.5.3 a) |
| `MaxLevelSpeed` à 4 000 m, avec régime | **174 km/h** ; 1 640 tr/min | S46 seul | **B** (déclassée de A) | **±5 %** ; régime ±3 % | oui | idem |
| `ClimbTime` de 0 à 2 000 m | **6 min 40 s** | S46 ; rapport n° 96 plus rapide de 4,5 % (C) | **A** | ±5 % | oui | écart avec la seule série indépendante ; chronométrage et masse réelle inconnus |
| `ClimbTime` de 0 à 3 000 m | **11 min 30 s** | S46 ; rapport n° 96 plus rapide de 2 % (C) | **A** | ±5 % | oui | idem |
| `ClimbTime` de 0 à 4 000 m | **19 min 30 s** | S46 ; rapport n° 96 plus rapide de 2 à 6 % (C, interpolé) | **A** | ±7 % | oui | idem, plus la sensibilité croissante près du plafond |
| `Ceiling` à vitesse ascensionnelle **nulle** | **5 486 m** (18 000 ft) | S46 (« théorique ») ; rapport n° 96 : 5 791 m estimés | **C** | ±7 % | oui (règle du M1 : C bloque) | extrapolation ; la tolérance couvre la fourchette de 5 100 à 5 800 m (§ 5.5.3 d) |
| Contrôle : vitesse au sol | 199 km/h | S47 (« H.S. 140 hp ») | C (sous-version) | ±4 % | **non** : à reporter dans le rapport d'essai | sous-version incertaine ; cohérente avec 192 km/h à 2 000 m (C) |
| Contrôle : montées italiennes | 2 000 m en 5 min 50 s ; 3 000 m en 9 min 50 s | S47 | C | — | **non** | série irrégulière ; à lire comme une borne optimiste |

Au M1, seuls les essais de confiance D sont non bloquants (skill `simulation-vol`, `format-avion.md`). Pour les deux contrôles, la façon de les saisir revient à l'aérodynamicien (D avec un commentaire « valeur d'époque, sous-version incertaine », ou simple mention dans le rapport).

**Consignes** (à valider par l'aérodynamicien) :
1. **Ordre de calage.** Caler d'abord sur les montées et la vitesse à 2 000 m, puis contrôler 3 000 et 4 000 m. **Ne pas ajuster de coefficient** pour atteindre 179 km/h à 3 000 m au détriment des 192 km/h à 2 000 m ou des montées.
2. **Diagnostic.** Si le modèle, calé sur les montées avec la loi R&M 462, donne de lui-même environ 185 km/h à 2 000 m, c'est le groupe 179-174 km/h qui l'emporte. Le signaler à l'historien plutôt que de forcer.
3. **Chronométrage.** Calculer chaque montée de deux façons : depuis le lâcher des freins, décollage compris, et depuis 0 m en vol à la vitesse de meilleure montée. Déclarer l'essai réussi si la cible tombe dans l'intervalle formé par ces deux temps, élargi de la tolérance. *(Dixième passe : le S.VII n'a pas de freins, l'avion part de l'arrêt ; le zéro des altitudes et le terrain ajoutent deux autres variantes, § 5.7.6.)*
4. **Plafond.** Utiliser `Ceiling` à vitesse ascensionnelle **nulle**, et non à 100 ft/min, valeur par défaut du format qui correspond au plafond pratique britannique. Si possible, extrapoler aussi comme à l'époque : prolonger jusqu'à zéro la droite des vitesses ascensionnelles du modèle entre 2 000 et 4 000 m, et rapporter les deux valeurs.
5. **Masse.** 703 kg ; les 705 kg des ouvrages (0,3 % d'écart) sont sans effet. Rapporter la sensibilité des montées à ±10 kg.
6. **Atmosphère.** Atmosphère type, marquée `E:` : les conditions des essais ne sont pas connues.
7. **À ne pas utiliser comme cibles** : le temps à 1 000 m de S47, les valeurs de S48, l'endurance de 2 h 10.

**180 ch.** Les cibles des § 5.2 et 5.4 restent valables, avec les mêmes tolérances : ±3 % en vitesse, ±5 % en montée jusqu'à 3 000 m, ±7 % à 4 000 m, et un plafond en C (±7 %, à vitesse ascensionnelle nulle). Sa série de montée est moins régulière que celle du 150 ch : selon les tranches, les extrapolations linéaires vont de 5 450 à 7 000 m (C), contre 6 523 m imprimés.

#### 5.5.7 Points non vérifiables et documents à demander

| Point | Ce qu'il faudrait | Où (adresse, cote) |
|---|---|---|
| Conditions des essais du STAé (chronométrage, atmosphère, masse pesée, hélice) ; ligne composite ou non | procès-verbaux d'essais du S.VII, 1916-1917 | **SHD, département Air, fonds Émile Dorand, DE 2012 ZL 245** : producteur indiqué, la Section technique de l'aéronautique militaire (1915-1918) ; procès-verbaux de la Commission d'examen des avions et moteurs nouveaux et notes du Service des essais selon la notice (S69 [RÉSUMÉ]). Télécharger l'**inventaire PDF de 2012** signalé sur la page geneawiki « SHD / Air - Série (DE)ZL » et y chercher « Spad », « 150 HP », « essais » ; puis rechercher sur francearchives.gouv.fr « Commission d'examen des avions » et « Spad » |
| Rapport britannique n° 96, avril 1917 (P10) | le rapport lui-même | The National Archives (Kew), série AIR 1. Piste : **AIR 1/1196/204/5/2600**, rapports hebdomadaires sur les avions en essai à l'Aeroplane Experimental Station de Martlesham Heath, n° 1 à 119, 1916-1919 (S70 [RÉSUMÉ]) ; contenu non confirmé. Rechercher aussi « Spad » dans AIR 1 sur discovery.nationalarchives.gov.uk |
| Original américain des tableaux S46 et S47, notes éventuelles sur les essais | numéro d'*Automotive Industries* de janvier ou février 1919 | déjà au § 9.6 (priorité 1) |
| Signes de renvoi de l'en-tête de S46 (« Speed on the Level », « Theoretical Ceiling ») | lecture à pleine résolution | le PDF local `flight_1919-02-13.pdf`, p. 204, rendu à 300 dpi et recadré sur la ligne d'en-tête. À 150 dpi, je crois voir des signes de renvoi devant ces intitulés sans trouver leur note sur les p. 204-205 : **lecture incertaine** |
| Méthode italienne (S47) | notes du tableau original ou fiches italiennes | original d'*Automotive Industries* ; G. Alegi et P. Varriale, *Italian SPADs at War* (P5) |

#### 5.5.8 Relecture de la fiche (huitième passe)

Corrections faites dans cette passe : page de la présentation de S46 (p. 201, et non p. 199) ; série du STAé non indépendante de S46 (§ 5) ; ligne Heurtaux du § 1.4 (date de publication prise pour celle de la blessure) ; deux points ajoutés au § 1.5 (nom « Vieux Charles » et fin de carrière du S.VII) ; mention « dépassé » au § 0 et sur deux entrées du journal du 2026-10-20 ; statut [LU-IMG] de la masse d'armement au § 7. Le détail est au journal des décisions.

### 5.6 Neuvième passe : régimes hors vitesse maximale (2026-10-23)

**Question.** Un seul régime fiable relevé sur un S.VII à Hispano de 150 ch (ou, à défaut, de 180 ch), en montée, au point fixe ou à une vitesse autre que la vitesse maximale, permettrait de dire si l'hélice du modèle absorbe trop ou si le moteur donne trop peu.

**Réponse : non trouvé.**
- **S46** ne donne que des régimes en palier ; **S47** n'en donne aucun.
- **S48** (1917) annonce un moteur de « 160 hp at about 1,500 r.p.m. » : c'est une cote, pas une mesure.
- ***Flight* 1916-1919** : aucun régime associé à un SPAD [LU-OCR].
- **Volume II du rapport ACA 1917-18** : le R&M 462 a mesuré des régimes sur un S.E.5 à Hispano de 150 hp pour la loi d'altitude, mais ils ne figurent que sur des courbes illisibles dans l'OCR. L'hélice était la T.28116, d'après la légende de la figure 6, lue dans l'OCR.
- **Web [RÉSUMÉ]** :
  - selon un résumé du fil S11, un « Air Board summary » coterait le Hispano de 150 hp à 1 600 tr/min (1 800 tr/min au maximum) et donnerait « 108 mph à 1 640 tr/min à 13 100 ft ». L'auteur du message rattache ce document au rapport de Martlesham M.96 d'avril 1917 et suppose qu'il s'agit de N6210, le premier SPAD de construction britannique ;
  - le couple 108 mph et 1 640 tr/min à 13 100 ft est **exactement** la ligne 8Aa de S46 à 4 000 m (174 km/h à 1 640 tr/min). C'est donc un régime en palier déjà connu, pas une nouvelle mesure ;
  - un autre résumé du même fil range ces chiffres sous un « 180 Hispano » sans numéro de modèle ;
  - ces résumés se contredisent : **piste seulement**. Si l'« Air Board summary » existe, il montrerait que les chiffres de la ligne 8Aa de S46 circulaient aussi dans un document britannique : indice sur l'origine de S46, à vérifier ;
  - un autre résumé cite un relevé de McCook à « 2 300 tr/min » à la vitesse maximale au niveau de la mer, sans dire sur quel SPAD ni avec quel moteur : **écarté (D)**.

**Comparable** (A pour la mesure, C pour le SPAD) : régimes à pleins gaz mesurés en vol sur le Vought VE-7 à moteur Wright à prise directe, avec deux hélices en bois de P/D 0,80 et 0,81 (`hispano-suiza-8a.md` § 7.5.6, H24 et H25) :
- en montée, vers 103 à 128 km/h : **0,88 à 0,92** fois le régime de palier ;
- au point fixe : **0,87 à 0,92** fois le régime de palier.

**Usage proposé.** Contrôle de vraisemblance du modèle, et non cible d'`essais.ron`. Si le rapport des régimes montée/palier du modèle sort nettement de 0,88-0,92, le signaler. La transposition au SPAD n'est pas faite ici : les J de montée, le moteur et la cellule diffèrent.

**Documents qui répondraient** :
- le R&M 586, qui essaie plusieurs hélices sur un Hispano de 150 hp sans réducteur (`hispano-suiza-8a.md` § 7.5.9) ;
- le rapport de Martlesham n° 96 (P10 ; AIR 1/1196/204/5/2600, § 9.6) ;
- les procès-verbaux d'essais du STAé (S69).

### 5.7 Dixième passe : conditions réelles des essais de montée (2026-10-24)

**Pourquoi.** Le modèle du 150 ch (8Aa, 703 kg) monte plus vite que S46 : −8 % à 2 000 m, −7 % à 3 000 m, −11 % à 4 000 m, et son plafond est trop haut de 7,8 %. Il chronomètre depuis 0 m, en vol, déjà à la vitesse de meilleure montée, en atmosphère type. Avant de chercher la cause dans le modèle, il faut savoir ce que les conditions réelles des essais ajoutent au temps. Documents relus et limites : § 0 (dixième passe).

**Réponses en bref.**

| Question | Réponse | Statut et confiance |
|---|---|---|
| 1. Départ du chronomètre, essais officiels français | **Non trouvé.** Pratique britannique de 1917 : temps réduits, intégrés depuis la hauteur type 0, donc en vol (S75). Essais civils britanniques de 1919 : départ à 100 ft au-dessus du sol (S76). Italie : non trouvé | France : — ; Royaume-Uni : [LU-OCR], A pour la méthode décrite |
| 2. Instrument, zéro, corrections, atmosphère type | France : non trouvé pour S46. **Atmosphère type officielle adoptée seulement le 15 avril 1920** (S79). Royaume-Uni : anéroïde bloqué sur 760 mm, correction de température, réduction à une hauteur type définie par la densité moyenne (S75) | France : [RÉSUMÉ], C ; Royaume-Uni : [LU-OCR], A |
| 2 bis. Terrain | Villacoublay, centre d'essais du STAé (S78, S81) ; environ 178 m (S80). Le lieu de l'essai de S46 n'est pas documenté | B pour l'altitude ; C pour l'attribution |
| 3. Date, saison et charge de l'essai de S46 | **Non trouvé.** Fenêtre déduite : de mi-1916 à mi-1917 (C). Charge de référence de 703 kg (§ 5.5.1) | C |
| 4. Décollage du S.VII | **Non trouvé** pour 1916-1918. Seul chiffre : environ 150 ft (46 m) de roulage, sur un S.VII à Wright-Hispano de 180 ch des années 1960 (S31) | [RÉSUMÉ], D |

#### 5.7.1 Départ du chronomètre

**France (STAé, Villacoublay, 1916-1918) : non trouvé.** Aucun règlement, aucune instruction et aucun procès-verbal lu ne dit si le temps de montée part du début du roulage, du décollage ou du passage d'une hauteur.
- **Pistes essayées** :
  - les 208 numéros de *Flight* (termes au § 0), le volume II du rapport ACA 1917-18 et le dossier Hartmann ;
  - des requêtes en français (« temps de montée », « compté à partir du décollage », « barographe », « procès-verbal d'essais », « réception »), en anglais et en italien (« tempo di salita », « collaudo »).
  - *L'Aérophile*, *La Guerre aérienne illustrée* et *L'Aéronautique* sont sur Gallica, qui refuse les accès automatiques.
- **Ce qui est établi sur le cadre, sans la méthode** :
  - une section des essais en vol, rattachée au Service technique, est créée à Villacoublay en 1915. Elle contrôle notamment le temps de montée, les vitesses en palier à plusieurs altitudes et le plafond (S81 [RÉSUMÉ]) ;
  - Hartmann situe à Villacoublay le centre d'essais du Service technique de l'aéronautique, où l'hélice Éclair est présentée en 1915, après des essais à Buc. Il mentionne aussi la réception des essais en vol des Farman à Buc (S78, p. 12 [LU]) ;
  - des essais officiels du Morane-Saulnier AI ont lieu à Villacoublay en août 1917, le troisième et dernier jour étant le 9 août (S84 [RÉSUMÉ]). Le Morane AI figure dans le même tableau S46.
- **Indice britannique sur les altitudes françaises.** Tizard écrit en mars 1917 que les hauteurs types britanniques de 6 500, 10 000, 13 000 et 16 500 ft ont été choisies pour correspondre aux altitudes d'essai françaises de 2 000 et 3 000 m, puis aux kilomètres suivants (S75, note du 8 mars et texte du 22 mars 1917) [LU-OCR].
  - L'OCR de la note porte « a T ooo metres » ; je rétablis « 2,000 metres » par le contexte (C).
  - Ce passage confirme l'usage français des kilomètres ronds. Il ne dit rien du chronométrage.
- **Avant-guerre (piste).** Au concours militaire de Reims de 1911, une épreuve préliminaire aurait demandé d'atteindre 500 m **au-dessus du point de départ** (S83 [RÉSUMÉ]). Un autre résumé parle plutôt de tenir 500 m pendant quinze minutes (S83). Le règlement reste à lire (S82). Le mode de chronométrage n'est pas connu.

**Royaume-Uni : Testing Squadron du RFC, 1916-1917 (S75, Tizard) [LU-OCR].**
- **Statut du texte.** Il décrit les méthodes « en usage » au Testing Squadron. Celui-ci s'installe à Martlesham Heath en janvier 1917 (S85 [RÉSUMÉ]) ; le rapport n° 96 y aurait été établi en avril 1917 (S11 [RÉSUMÉ]).
- **Relevés pendant la montée** : hauteur à l'anéroïde tous les 1 000 ft, temps écoulé « depuis le début de la montée » (*from the start of the climb*), température, vitesse indiquée, régime. Le « début de la montée » n'est pas défini plus précisément (15 mars 1917).
- **Répétition.** Chaque nouvel avion fait au moins deux montées jusqu'à 16 000 ft ou plus à l'anéroïde, trois si le temps le permet. Le résultat publié est la moyenne, ramenée à un « jour type ».
- **Réduction** :
  - les temps observés sont tracés en fonction de la hauteur à l'anéroïde ;
  - la vitesse ascensionnelle est la pente de cette courbe tous les 1 000 ft, corrigée de la température ;
  - elle est reportée en fonction de la hauteur type, puis le temps de montée est obtenu **par intégration graphique** ;
  - dans l'exemple publié (tableau IV, essai du 27 décembre 1916, 36 °F à 0 ft d'anéroïde), la colonne réduite part de 0 à la hauteur type 0. Elle donne 1,20 min à 1 000 ft, 2,56 min à 2 000 ft et 21,61 min à 10 000 ft.
- **Conséquence (C).** Un temps britannique réduit ne dépend pas du départ réel du chronomètre. C'est le temps d'une montée **en vol, depuis la hauteur type 0**, à la vitesse ascensionnelle mesurée et prolongée vers le bas : la convention du modèle.
  - Le rapport n° 96 a très probablement été réduit ainsi (C). Il vient de la même unité, un mois après la conférence. Tizard ajoute que les essais officiels de l'armée et de la marine sont désormais tous réduits à la même base.
- **Plafond.** Tizard appelle plafond le point où la courbe des vitesses ascensionnelles, prolongée, coupe l'axe des hauteurs (15 mars 1917). Le § 5.5.3 d) supposait cette définition pour le plafond « théorique » de S46. Elle est désormais lue dans un texte d'époque, britannique ; pour S46, elle reste une interprétation (C).

**Royaume-Uni : essais civils certifiés du Royal Aero Club, 1919 (S76) [LU-OCR].**
- L'essai de vitesse ascensionnelle commence quand l'avion s'est élevé de 100 ft au barographe.
- Le certificat donne le temps de chaque tranche de 1 000 ft **au-dessus de ce niveau de départ**.
- Pour les essais de hauteur, un observateur note à part l'heure où l'avion quitte le sol.
- C'est un règlement civil de 1919 : il montre une pratique britannique, pas celle du STAé.

**Italie (S47) : non trouvé.**
- Les temps à 1 000 m de S47 sont anormalement courts sur plusieurs lignes du tableau (§ 5.5.3 c).
- Un chronomètre déclenché en vol, ou des hauteurs comptées au-dessus d'un terrain, raccourciraient surtout cette première tranche. C'est une **hypothèse**, appuyée sur aucun document.

**Déduction sur un essai au barographe (C).**
- Un barographe trace la pression en fonction du temps. Sur le tracé, l'origine naturelle est le moment où la pression commence à baisser, c'est-à-dire le **décollage**. Le roulage n'y laisse pas de trace. Le seuil de 100 ft du Royal Aero Club suggère que même le décollage était difficile à repérer sur le tracé (interprétation).
- Si les temps français ont été lus sur un barogramme, ce que je n'ai pas pu vérifier, leur origine est donc plutôt le décollage que le début du roulage.
- Un chronométreur au sol pouvait au contraire partir du début du roulage. Aucun document ne le dit.

**Pas de freins.** Aucun document lu ne décrit la mise en route d'un essai de montée : avion retenu par des cales ou par les mécaniciens, moteur au régime, puis lâché. Je ne comble pas ce point.

#### 5.7.2 Altitudes : instrument, zéro, corrections, atmosphère type

**Royaume-Uni (S75) [LU-OCR].**
- **Contexte.** Selon Tizard, pendant les premiers mois de la guerre, une montée à 3 000 ou 5 000 ft à l'anéroïde et un passage sur une base de vitesse suffisaient comme essai d'un avion neuf (8 mars 1917).
- **Trois sens possibles.** « 10 000 ft » peut désigner la lecture de l'anéroïde, la hauteur au-dessus du point de départ, ou la hauteur au-dessus du niveau de la mer. Tizard en conclut qu'il faut comparer les essais à densité égale (8 mars 1917). Au début de 1917, l'usage n'était donc pas unifié.
- **Zéro.** Pour les essais, l'anéroïde est **bloqué** : son zéro est fixé sur la pression type de 760 mm, sans réglage au sol. Il sert de manomètre (15 mars 1917).
- **Correction de température.** L'anéroïde est gradué pour une colonne d'air uniforme à 50 °F (10 °C). Il lit donc trop haut dès que l'air est plus froid : en hiver, l'erreur peut atteindre 2 000 ft à 16 000 ft. Les hauteurs sont corrigées par un facteur de température (8 mars 1917).
- **Atmosphère de réduction.** C'est une atmosphère moyenne, tirée des observations de J. S. Dines (tableau I : 1 014 mb et 282 K au niveau de la mer). La hauteur type est la hauteur moyenne qui correspond à la densité mesurée.
- **Comparaison avec l'atmosphère type internationale (C, calcul fait ici).**
  - Densités du tableau I, de 0 à 5 km : 1,253, 1,128, 1,014, 0,909, 0,818 et 0,735 kg/m³. Atmosphère type internationale : 1,225, 1,112, 1,007, 0,909, 0,819 et 0,736.
  - Écarts : +2,3 % au niveau de la mer, +1,5 % à 1 km, +0,7 % à 2 km, moins de 0,3 % de 3 à 5 km.
  - Sous 2 000 m, l'échelle britannique est donc un peu plus dense. Un temps réduit britannique y est un peu plus court qu'en atmosphère type internationale, de l'ordre de 1 à 3 % (estimation grossière, C). Au-dessus de 3 000 m, les deux échelles se confondent.
  - L'OCR du tableau I porte 568 mb à 5 km. La densité (0,735) et la température (255 K) imprimées impliquent environ 540 mb : erreur d'OCR (C).
- **Biais saisonnier de la réduction.** R&M 462 (mars 1918) montre que la réduction à densité égale pénalise les essais faits par temps froid (§ 18 à 21 ; `hispano-suiza-8a.md`, H21) [LU-OCR, déjà cité au § 5.5.2] :
  - S.E.5a à moteur Viper : deux montées à 15 000 ft, le 28 février et le 6 mars 1918, diffèrent d'environ six minutes avec l'ancienne méthode ; elles sont pratiquement identiques avec la réduction en pression ;
  - N.E.1 à Hispano de 200 hp : deux montées à 10 000 ft, le 13 septembre et le 15 octobre 1917, diffèrent d'environ deux minutes avec l'ancienne méthode ;
  - même réduits, des temps de 1917 gardent donc un biais saisonnier de plusieurs pour cent (C).
- **États-Unis (piste).** Selon un document du Bureau of Standards vu en résumé, les altimètres américains sont étalonnés depuis 1917 pour une atmosphère isotherme à +10 °C (S86 [RÉSUMÉ], attribution à vérifier).

**France.**
- **Instrument** : non documenté pour S46. Des altimètres enregistreurs Jules Richard, gradués jusqu'à 5 000 m, existaient en 1917 (S88 [RÉSUMÉ]). Rien ne dit qu'ils servaient aux essais du STAé.
- **Zéro** : non trouvé. Seule piste : des hauteurs comptées au-dessus du point de départ en 1911 (S83 [RÉSUMÉ]).
- **Graduation et correction de température** des instruments français : non trouvées. Le rapport du NACA sur les instruments d'altitude, qui décrit aussi les instruments français, devrait répondre (§ 5.7.7).
- **Atmosphère type.**
  - Le 15 avril 1920, le sous-secrétaire d'État à l'Aéronautique adopte comme atmosphère type **pour les essais officiels d'avions en France** la loi dite de la S.T.Aé. : température de 15 − 0,0065 z (°C, z en mètres) jusqu'à 11 000 m, pression de 760 mm au niveau de la mer (S79 [RÉSUMÉ]).
  - A. Toussaint avait publié les premières études de cette loi dans *L'Aéronautique*, n° 5, octobre 1919 (S79).
  - La S.T.Aé. l'aurait d'abord diffusée en mars 1920 comme projet d'accord interallié, et appliquée à partir du 1er mai 1920 (S87 [RÉSUMÉ]).
  - **Réponse à la question (C) : aucune atmosphère type officielle française n'a été trouvée avant 1920.** Une méthode de réduction interne au STAé pendant la guerre n'est pas exclue, mais aucun document ne la décrit.
  - Le même article de Grimault illustre l'effet des conditions de référence : la vitesse d'un Breguet XIV à 3 000 m varie d'environ 183 à 186 km/h selon elles (S79 [RÉSUMÉ]).
- **Chiffres bruts ou réduits ? Non tranché.**
  - Pour des chiffres bruts : la compilation date de 1918, avant toute atmosphère type française, et les écarts irréguliers de la ligne 8Aa plaident contre un lissage (§ 5.5.5).
  - Pour une analyse de la courbe de montée : le plafond « théorique » est une extrapolation.
  - Conclusion : chiffres bruts, ou réduits par une méthode du STAé inconnue (C).

**Terrain.**
- Villacoublay est le centre d'essais du STAé (S78 [LU], S81 [RÉSUMÉ]), et des essais officiels y sont attestés en 1917 (S84 [RÉSUMÉ]).
- Altitude de référence actuelle de l'aérodrome : **585 ft, soit environ 178 m** (AIP France, S80 [RÉSUMÉ]). Le plateau n'a pas changé d'altitude : environ 175 à 180 m en 1916-1918 (B pour l'ordre de grandeur).
- Le lieu de l'essai du S.VII de S46 **n'est pas documenté** ; Villacoublay est le plus probable (C). Buc servait à des essais de réception (Farman, S78), mais aucun document lu ne le rattache aux essais officiels du SPAD.

#### 5.7.3 Date et conditions de l'essai du S.VII 150 ch de S46

**Non trouvé** : ni date, ni saison, ni température, ni pression, ni masse pesée.

**Ce qui encadre l'essai (C)** :
- il est postérieur au premier marché de série, du 10 mai 1916 (§ 1.1) ;
- il est antérieur à la compilation américaine de 1918, où le S.VII n'est plus marqué comme livré (§ 5.5.1) ;
- le 8Ab de 180 ch équipe les avions neufs à partir du printemps 1917 (§ 1.2) ;
- la fenêtre la plus probable va donc de **mi-1916 à mi-1917**. Elle contient un hiver et un été : **la saison reste inconnue**.

**Autres éléments** :
- la charge de 703 kg est une charge de référence, pas une pesée (§ 5.5.1) ;
- le chiffre du prototype, 4,5 min à 2 000 m (S5 [RÉSUMÉ]), vient d'un autre essai, sur un autre avion ;
- flyingmachines.ru donne, à côté de 6 min 40 s, une seconde valeur de 6 min 55 s à 2 000 m pour le 150 ch (S12 [RÉSUMÉ]). Au moins deux chiffres français ont donc pu circuler ; piste à vérifier sur les procès-verbaux (S69).

#### 5.7.4 Décollage du S.VII

**Non trouvé** pour 1916-1918, ni pour le 150 ch ni pour le 180 ch : ni distance, ni durée de roulage, ni vitesse de décollage. S46, S47 et S48 n'ont pas de colonne de décollage.

**Seul chiffre vu** : un récit attribué à Frank Tallman, sur un S.VII à moteur Wright-Hispano E de 180 ch, dans les années 1960 (S31 [RÉSUMÉ]) :
- la queue se lève presque aussitôt ;
- l'avion est en l'air et monte après environ **150 ft (46 m)** ;
- la meilleure montée se fait vers **60 à 65 mph (97 à 105 km/h)** ;
- le décrochage moteur réduit a lieu vers 47 mph (§ 5.3).

**Confiance D** : avion et moteur postérieurs ; masse, vent et terrain inconnus ; reprise informelle d'un livre. Ce chiffre ne doit pas servir de cible ; tout au plus de contrôle grossier de la vitesse de meilleure montée.

#### 5.7.5 Ordres de grandeur pour l'enveloppe (C, à refaire avec le modèle)

**Loi utilisée.** Une vitesse ascensionnelle qui décroît linéairement avec l'altitude, de r₀ ≈ 6,2 m/s au niveau de la mer à zéro vers H ≈ 5 400 m, reproduit les temps de S46. Elle donne 400, 701 et 1 167 s à 2 000, 3 000 et 4 000 m, contre 400, 690 et 1 170 s imprimés (§ 5.5.3 c). Les écarts ci-dessous sont des ordres de grandeur ; l'aérodynamicien les refera avec le modèle.

**Cas 1 : terrain à 178 m, altitudes comptées au-dessus du terrain, chronomètre lancé en vol au niveau du terrain** (décollage non compris).

| Altitude lue | Temps selon la loi, depuis 0 m | Temps selon la loi, de 178 m à 178 m + altitude lue | Écart |
|---|---|---|---|
| 2 000 m | 400 s | 418 s | **+17 s (+4,4 %)** |
| 3 000 m | 701 s | 739 s | **+38 s (+5,4 %)** |
| 4 000 m | 1 167 s | 1 256 s | **+89 s (+7,6 %)** |

Dans ce cas, le plafond théorique de 5 486 m serait lu au-dessus du terrain, soit environ 5 664 m au-dessus de la mer. L'excès de 7,8 % du modèle tomberait vers **+4,4 %** (C).

**Cas 2 : altimètre bloqué sur 760 mm, à la britannique, chronomètre lancé au sol à 178 m.** L'avion ne monte que de 178 m jusqu'à l'altitude lue. Le temps diminue d'environ **29 s** à chaque altitude, soit **−7 % à 2 000 m** et −2,5 % à 4 000 m. **Le sens de l'effet dépend donc entièrement de la convention du zéro.**

**Température (non chiffrée ici).**
- Dines donne au sol, dans le sud de l'Angleterre, des températures de 20 à 80 °F (−7 à +27 °C). La température seule fait varier la densité d'environ ±6 % ; avec la pression, des écarts de 8 % ne sont pas impossibles. Au-dessus de 5 000 ft, les variations sont bien plus faibles (S77 [LU-OCR]). Ce sont des données britanniques, transposées à la région parisienne (C).
- Un instrument gradué pour une colonne isotherme et non corrigé se trompe d'environ 0,35 % de la hauteur par kelvin d'écart entre la température moyenne de la colonne et sa température de graduation (C, formule du nivellement barométrique).
- Un été chaud allonge le temps nécessaire pour atteindre une altitude lue ; un hiver le raccourcit. La température de graduation des instruments français est inconnue.

**Décollage : non chiffré**, faute de données (§ 5.7.4). C'est au modèle de le calculer : départ arrêté, plein gaz, roulage, décollage, puis accélération jusqu'à la vitesse de meilleure montée.

**Comparaison avec le rapport n° 96 (C).**
- Supposons que ses chiffres, vus en résumé, soient exacts, et qu'il ait été réduit selon la méthode de Tizard. Il serait alors presque dans la convention du modèle : en vol depuis la hauteur type 0, avec une densité proche de l'atmosphère type au-dessus de 2 000 m.
- Or le modèle est aussi plus rapide que lui : d'environ 4 % à 2 000 m, 5 % à 3 000 m et 5 à 9 % à 4 000 m (calcul fait ici à partir du § 5.5.3 c). L'échelle britannique, un peu plus dense sous 2 000 m, rend même ce rapport légèrement optimiste.
- **Les conditions d'essai françaises ne suffiraient donc peut-être pas à expliquer tout l'écart.** Cette conclusion reste fragile tant que le rapport n° 96 n'est pas lu (P10).

#### 5.7.6 Ce que je recommande de retenir pour l'enveloppe des temps de montée

1. **Départ du chronomètre** : non tranché pour le STAé. Calculer trois bornes :
   - (a) en vol, au niveau du terrain, à la vitesse de meilleure montée. Avec un terrain au niveau de la mer, c'est la convention du modèle et, par construction, celle des temps réduits britanniques ;
   - (b) au décollage, roues quittant le sol : origine naturelle d'un barogramme (C) ;
   - (c) au début du roulage, avion arrêté, plein gaz, puisqu'il n'y a pas de freins.
2. **Altitude du terrain** : Villacoublay, **178 m** (B pour l'altitude, C pour l'attribution de l'essai).
3. **Zéro des altitudes** : calculer les deux cas.
   - (i) zéro au sol du terrain : hauteurs au-dessus du point de départ (piste de 1911) ;
   - (ii) zéro sur la pression de 760 mm : altitude-pression, comme l'anéroïde bloqué britannique ;
   - **ne pas retenir la convention qui arrange le modèle** : aucune source ne permet de choisir.
4. **Température** : atmosphère type au terrain (13,8 °C à 178 m), et ±15 K autour (`E:`), d'après la fourchette de Dines (S77). Pour un instrument isotherme non corrigé, prendre 10 °C en hypothèse (`E:`) : c'est la convention britannique et américaine, non prouvée pour la France.
5. **Masse** : 703 kg, inchangée.
6. **Cibles** : celles du § 5.5.6 et leurs tolérances **ne changent pas**. Je propose de comparer chaque cible à l'enveloppe formée par ces variantes plutôt qu'à un seul temps. Décision à l'aérodynamicien.
7. **Diagnostic** : si le modèle reste plus rapide que le rapport n° 96 une fois celui-ci lu, la cause est probablement dans le modèle, pas dans les conditions d'essai.

#### 5.7.7 Documents à télécharger (dixième passe)

| Priorité | Document | Adresse | Ce qu'on y cherche |
|---|---|---|---|
| 1 | P. Grimault, « On the definition of the standard atmosphere », NACA TM 15, avril 1921, traduit du français (S79) | NTRS 19930080725 : `https://ntrs.nasa.gov/api/citations/19930080725/downloads/19930080725.pdf` | pratique française avant avril 1920 ; réduction des essais officiels ; exemple du Breguet XIV |
| 1 | Bureau of Standards, « Aeronautic instruments. Section II: altitude instruments », rapport du NACA (n° 126 ou 127, vers 1921 ; **numéro à vérifier**) | UNT : `https://digital.library.unt.edu/ark:/67531/metadc65776/`. Sur NTRS, sans doute 19930091189, entre la section I (19930091188) et la section III (19930091190) : **identifiant non vérifié** | graduation (température, formule) et réglage du zéro des altimètres et barographes français, britanniques et italiens |
| 1 | Images à 300 dpi du tableau IV de Tizard (*Flight*, 15 mars 1917) et de sa note sur la « French test height » (*Flight*, 8 mars 1917, p. 233) | items archive.org `Flight_International_Magazine_1917-03-15-pdf` et `Flight_International_Magazine_1917-03-08-pdf` | origine réelle du temps dans la colonne brute (0,0 min à 0 ft d'anéroïde ?) ; lecture « 2,000 metres » de la note |
| 2 | W. Margoulis, « Notes on specifications for French airplane competitions », NACA TN 20, 1er octobre 1920 | NTRS 19930080812 : `https://ntrs.nasa.gov/api/citations/19930080812/downloads/19930080812.pdf` | règles françaises de 1920 (Aéro-Club de France, Villacoublay) : départ du chronomètre, mesure des hauteurs |
| 2 | Rapport du NACA « Standard atmosphere – tables and data » (S87 probable) | NTRS 19930091285 : `https://ntrs.nasa.gov/api/citations/19930091285/downloads/19930091285.pdf` | historique : projet interallié de la S.T.Aé. de mars 1920, pratiques antérieures |
| 2 | G. Hartmann, « Le grand concours d'aviation militaire, Reims 1911 » (S82) | `https://www.hydroretro.net/etudegh/grand_concours_d_aviation_militaire__reims_1911.pdf` | règlement de l'épreuve de hauteur : zéro au départ, chronométrage, barographe |
| 2 | G. Hartmann, « Les Nieuport de la guerre » | `https://www.hydroretro.net/etudegh/les_nieuports_de_la_guerre.pdf` | transcriptions d'essais du STAé avec leurs conditions (comparable de méthode) |
| 2 | Comité pour l'histoire de l'aéronautique, *Un demi-siècle d'aéronautique en France : centres et moyens d'essais*, t. 1, coord. J.-P. Marec, 2013 (S81) | `https://www.academie-technologies.fr/wp-content/uploads/2013/08/UN-DEMI-SIECLE-DAERONAUTIQUE-EN-FRANCE-CENTRES-ET-MOYENS-DESSAIS-T1.-COORD.-PAR-J.P.-MAREC-COMITE-POUR-LHISTOIRE-DE-LAERONAUTIQUE-2013.pdf` | historique de la section des essais de Villacoublay (1915-1918) et de ses méthodes |
| 2 | « Les essais en vol à Villacoublay » (PDF, auteur non relevé) | `http://resair420.free.fr/popote%20des%20ailes/les_essais_en_vol_a_villacoublay.pdf` | idem |
| 3 | A. Toussaint, article sur la loi de température, *L'Aéronautique*, n° 5, octobre 1919 | Gallica (accès automatique refusé) ; à chercher aussi sur archive.org | atmosphère et réduction des essais français avant 1920 |
| rappel | SHD, fonds Dorand (DE 2012 ZL 245) ; The National Archives, AIR 1/1196/204/5/2600 | § 5.5.7 et § 9.6 | procès-verbaux du STAé (date, température, chronométrage) ; rapport n° 96 et sa méthode de réduction |

## 6. Qualités de vol, témoignages et réputation

### 6.1 Témoignages

Format : auteur · camp et rôle · date du témoignage · sous-version · langue · statut. Puis la paraphrase en français et les biais.

**Français**

- **T1 — Georges Guynemer, lettre à Béchereau, 8 septembre 1916** · France, pilote de chasse (N 3), conseiller de Béchereau · sur le moment · un des deux premiers SPAD au front (150 ch, C) · original français, lu dans la traduction anglaise de 1918 · S1 [LU].
  Premier combat du SPAD : il fond sur un groupe de biplaces, qui le prennent « pour un météore », et sa mitrailleuse n'enraye pas une fois. Sur l'avion : il « boucle merveilleusement » ; sa **vrille** est « un peu paresseuse et irrégulière, mais délicieusement douce » (*a bit lazy and irregular, but deliciously soft* dans la traduction). Il ajoute de petites améliorations à apporter. Le chiffre « 1800 on the speedometer » (vitesse d'approche) est ambigu : sans doute le compte-tours, **interprétation non vérifiée**.
  Biais : lettre au constructeur dont il est proche ; enthousiasme du premier vol ; double traduction (français, anglais, puis paraphrase française).

- **T2 — Guynemer, lettre à son père, 23 septembre 1916** · idem · sur le moment · S.VII 150 ch (C) · S1 [LU].
  Après trois victoires, un obus de 75 français crève son réservoir d'eau et arrache l'entoilage du plan supérieur gauche. L'avion part en vrille ; il parvient à la transformer en plané et touche le sol vers 160 à 180 km/h. Tout est brisé « comme des allumettes » **sauf le fuselage, intact** : selon lui, le SPAD est solide, et sur une autre machine il serait « plus mince que cette feuille de papier ». La chute est décrite comme partant de 3 000 m.
  Biais : lettre familiale (minimiser le danger) ; avion endommagé, donc pas une mesure de comportement normal.

- **T2 bis — Le même épisode raconté par Jacques Mortane** · France, journaliste aéronautique, directeur de *La Guerre aérienne illustrée* · série d'articles sur Guynemer, 1918, citée et traduite en anglais par *Flight* le 18 juillet 1918, p. 798-799 · S.VII 150 ch (C) · original français non consulté · S52 [LU-OCR].
  Récit à la première personne prêté à Guynemer. Un obus de 75 français déchire l'aile gauche ; l'entoilage claque et se déchire davantage pendant la chute. Une **vrille** commence vers 3 000 m et se poursuit jusque vers 1 600 m. Les commandes ne répondent plus, dans aucun sens. L'avion touche le sol à environ 180 km/h ; il n'en reste rien, et la ceinture sauve le pilote.
  Biais : *Flight* souligne que Mortane « parle à travers son héros » ; la forme est celle du journaliste. Ce n'est **pas un témoignage indépendant** de T2 : même épisode, même source première (Guynemer). Il confirme les chiffres de T2 (3 000 m, environ 180 km/h) et ajoute l'altitude de 1 600 m et l'inefficacité des commandes. Avion gravement endommagé : à ne pas transposer au comportement normal.

- **T3 — Guynemer, lettre à Béchereau, 28 décembre 1916** · idem · sur le moment · « Spad 150 H.P. » · S1 [LU].
  Le SPAD de 150 ch n'a guère d'avance sur le Halberstadt : celui-ci n'est pas plus rapide, mais il **monte tellement mieux** que cela revient au même. Il note que « notre dernier modèle » les surclasse tous (lequel ? non précisé). Il demande plus de vitesse et une **meilleure hélice**. Le 18 novembre 1916, il réclamait déjà un moteur renforcé (S1).
  Biais : pilote exigeant qui plaide pour un moteur plus puissant ; comparaison avec un seul type adverse.

- **T4 — Henry Bordeaux, biographe, témoin au camp de l'Aisne au printemps 1917** · France, écrivain (pas pilote) · 1918 · SPAD (150 ou 180 ch, non précisé) · S1 [LU].
  (a) Le « terrible Spad » est un instrument d'attaque extraordinaire, dont la défense ne tient qu'à la vitesse de déplacement et d'évolution. Son arrière est mal défendu : champ de vision très limité sur les côtés, vue seulement vers le haut et vers le bas, avec l'angle mort du moteur et du poste. Le pilote perd facilement de vue les autres avions.
  (b) Au printemps 1917, le SPAD serait supérieur au meilleur Albatros en solidité, en vitesse et en facilité de conduite.
  (c) Témoin oculaire : des SPAD décollent et montent « presque verticalement » (formule de journaliste), puis, au retour, enchaînent spirales, boucles et piqués.
  Biais : hagiographie publiée en pleine guerre, propagande. Ce n'est pas un pilote, mais il rapporte sans doute les propos des pilotes.

- **T5 — Incidents mécaniques à la SPA 3 en Flandre, 5-10 septembre 1917** · S1 [LU], d'après les témoins de l'escadrille.
  Enrayage des deux mitrailleuses (gâchette défaillante) ; commande de pompe à eau inopérante sur la machine à deux mitrailleuses (atterrissage chez les Belges) ; pompe à air touchée ; essence qui jaillit sur le carburateur de l'avion de Lagache. Sous-versions : un SPAD XIII, plus des avions de type non précisé.
  Biais : récit dramatisé des derniers jours de Guynemer.

**Anglais (Américains au service de la France, Britanniques, essais américains)**

- **T6 — James Norman Hall, *High Adventure*** · volontaire américain, caporal pilote à l'Escadrille Lafayette (SPA 124) · lettres de l'été 1917 (les ordres sont datés « de juin »), publiées en 1918 · S.VII d'occasion au moteur neuf ou reconstruit, sans doute 150 ch (C) · anglais · S3 [LU].
  (a) Son vieux SPAD décolle avec vigueur, mais « n'est pas un grimpeur rapide » : les camarades doivent l'attendre.
  (b) Seul vers 4 000 m, sa « vieille caisse » au moteur reconstruit **chauffe**. Il doit réduire les gaz et piquer plusieurs fois pour la refroidir.
  (c) Touché, il tombe moteur à plein régime, presque à la verticale, dans une sorte de **demi-vrille** : « aucune machine qu'un Spad n'aurait supporté l'effort ». Il reprend le contrôle près du sol ; l'avion est détruit dans une tranchée.
  (d) Un camarade blessé : « son Spad a tenu ».
  (e) Perception de l'époque : le Nieuport est « surpassé et presque entièrement supplanté » par des SPAD de « 140, 180, 200 et 230 » ch. Ces puissances sont inexactes ou mélangées.
  (f) Ailleurs : un camarade, hélice calée, atterrit en planant « à plat ». Six fusées montées sur les mâts (trois de chaque côté) servent à l'attaque des ballons. Hall ne donne pas le type de ces fusées ; l'identification « Le Prieur » n'est pas sourcée.
  Biais : textes écrits pour l'*Atlantic Monthly*, avec pseudonymes, censure et enthousiasme de recrue.

- **T7 — Charles Nordhoff, *The Fledgling*** · volontaire américain dans l'aviation française · lettres de 1917-1918, publiées en 1919 · SPAD de 1918 (type non précisé) · anglais · S4 [LU].
  Un récit composite et en partie imaginaire (« Wilkins ») montre une recrue ravie d'être affectée sur SPAD plutôt que sur Nieuport. Une patrouille de SPAD « monte à cent milles à l'heure » (formule narrative, pas une mesure).
  Biais : récit romancé. Il est utile pour la **réputation** (avion convoité), pas pour les chiffres.

- **T8 — Louis G. Meister, pilote d'essai en chef de McCook Field, rapport du 27 avril 1921** · États-Unis, pilote d'essai (regard technique, sans attachement) · 1921 · **SPAD XIII** (moteur à réducteur, rotation inverse du VII) · anglais · S15 [RÉSUMÉ].
  En vol : très stable, mais il faut beaucoup de pied à gauche, car le couple moteur est « très prononcé ». Avion lourd de la queue (la phrase est coupée dans le résumé). Le moteur est très sensible à la température en plané et refroidit vite : le pilote doit manœuvrer sans cesse les volets du radiateur. Au sol, deux versions coexistent dans le résumé : il roule facilement même par vent fort, sans tendance à tourner, **ou** il tend à embarder au décollage. Il se pose facilement, s'arrête court grâce à sa queue lourde et ne capote pas, même posé queue haute ou sur terrain bosselé.
  Biais : avion d'après-guerre, autre sous-type.

- **T9 — Frank Tallman, *Flying the Old Planes*, Doubleday, 1973** · États-Unis, pilote de cinéma et collectionneur · années 1950-1970 · **S.VII original à moteur Wright-Hispano E de 180 ch** (immatriculé N4727V, S31) · anglais · S31 [RÉSUMÉ].
  D'après un fil de forum qui semble reproduire son chapitre (attribution non certaine) : efforts aux commandes **légers** ; décollage court ; montée rapide ; décrochage moteur réduit vers 47 mph ; **plané médiocre**.
  Biais : avion restauré, moteur américain ; vol de loisir, pas d'essai réglé.

- **T10 — Histoire de l'Escadrille Lafayette par l'US Air Force (*Like a Thunderbolt*)** · secondaire institutionnelle · S32 [RÉSUMÉ].
  Moins vif que le Nieuport, mais plus rapide en ligne droite, meilleur en piqué et plus solide en l'air. Plus difficile à entretenir : surchauffe, fuites d'huile, réducteurs fragiles des 220 ch. Des avions neufs perdus sur des défauts de fabrication.

- **T17 — Edward Rickenbacker, *Fighting the Flying Circus* (1919), résumé par *Flight* le 29 mai 1919, p. 706-707** · États-Unis, as du 94th Aero Squadron · mémoires publiés juste après la guerre · SPAD de 1918 (XIII, C) face au Nieuport · anglais · S53 [LU-OCR], **paraphrase de *Flight***, livre non consulté (P7 bis).
  Selon *Flight*, Rickenbacker écrit que les Français avaient abandonné le Nieuport pour le « Spad, plus solide », ce qui laissa aux Américains des Nieuport démodés au début de 1918.
  Biais : plaidoyer contre l'administration américaine ; comparaison avec le Nieuport ; troisième main (livre résumé par une revue). Utile pour la **réputation** de solidité, pas pour un chiffre.

- **Presse britannique d'époque sur le SPAD** (rumeurs et réputation, pas des témoignages de pilote) · S54 [LU-OCR] :
  - *Flight*, 25 mai 1916, p. 450-451 : d'après le correspondant parisien du *Daily Express*, le nouvel avion rapide « Spad » (attribué à tort à Blériot) monterait « rapidement et en douceur » et dépasserait 125 mph (201 km/h). C'est l'époque du prototype ;
  - *Flight*, 28 décembre 1916, vers p. 1146 : un nouveau monoplace Albatros à moteur fixe refroidi par eau est surnommé le « Spad allemand ». *The Aeroplane* du 10 janvier 1917 titre de même « The Albatros-Spad » (référence relevée dans S56, article non lu) ;
  - *Flight*, 31 janvier 1918, p. 111-112 (éditorial) : selon les informations reçues par la revue, les pilotes français ont une « entière confiance » dans les Breguet et les SPAD ;
  - *Flight*, 22 novembre 1917, p. 1217 : citant un correspondant de presse, la revue écrit que l'état-major allemand voulait un chasseur capable d'affronter « à égalité » les Nieuport et les SPAD français.
  Biais : presse alliée de guerre, propagande, informations de seconde main.

**Allemand**

- **T11 — Manfred von Richthofen, *Der rote Kampfflieger*** · Allemagne, as et chef de la Jasta 11 · écrit en 1917, publié en 1917 · SPAD britanniques du printemps 1917 (S.VII, sans doute 150 ch, C) · allemand · S2 [LU].
  (a) Le 25 avril 1917 (en-tête du chapitre « Das Anti-Richthofen-Geschwader »), trois SPAD monoplaces anglais se croient très supérieurs « grâce à leurs bonnes machines ». L'attaque tourne pourtant à leur défense. L'un d'eux, abattu, se disloque sous le feu.
  (b) Le 29 avril 1917 : « les célèbres triplans et Spads, machines toutes neuves », mais « ce n'est pas la caisse qui compte, c'est celui qui est dedans ».
  Biais : livre de propagande publié sous censure ; mépris affiché pour l'adversaire. Il atteste pourtant la **réputation** du SPAD côté allemand : bonne machine, récente, célèbre.

- **T12 — Ernst Udet, *Mein Fliegerleben*** · Allemagne, as · mémoires de 1935 (éd. Projekt-Gutenberg datée 1942) · SPAD de Guynemer marqué « Vieux », juin 1917 · allemand · S29 [RÉSUMÉ].
  Duel seul contre un SPAD : Udet ne parvient pas à prendre l'avantage, sa mitrailleuse s'enraye, et l'adversaire rompt le combat d'un signe de la main.
  Biais : témoignage **unique**, tardif ; Guynemer n'en a jamais parlé, et la date est discutée (S29). Classé **légende** (§ 6.3).

- **T13 — Gerhard Fieseler (front de Macédoine)** · Allemagne, pilote de chasse · mémoires tardifs, transmis par une page biographique · SPAD (type non précisé) · allemand · S30 [RÉSUMÉ].
  Le pilote français Fronval s'échappe en piqué très raide, ce que son SPAD « stable » lui permettait.
  Biais : paraphrase de seconde main.

- **T18 — *Flugsport* (revue technique allemande), article sur les chasseurs français repris par *Flight* le 30 août 1917, p. 885 environ** · Allemagne, presse technique · 1917 · « le SPAD moderne » (type non précisé) · allemand, lu en traduction anglaise · S57 [LU-OCR].
  Avant l'arrivée du SPAD moderne, le Nieuport était, de tous les chasseurs français, celui que les pilotes recherchaient le plus.
  Biais : regard adverse, sans affection pour la machine ; jugement indirect (ce que les pilotes français préféraient). Concorde avec T7 et T17 sur un point : le SPAD a pris la place du Nieuport comme avion convoité.

- **T19 — Rapport allemand sur un SPAD biplace capturé, publié par *Flugsport* le 10 avril 1918 et traduit par *Flight* le 16 mai 1918, p. 527-529** · Allemagne, rapport « apparemment officiel » selon *Flight* · 1918 · **SPAD S.XI** n° B 6006, construit sous licence par Bernard en juillet 1917, moteur Hispano de 200 hp à réducteur · S49 [LU-OCR].
  **C'est le premier examen allemand d'un SPAD capturé trouvé dans nos sources**, mais il ne porte **pas sur le S.VII**. Regard purement technique : construction de la voilure et des ailerons, calages, masses détaillées (765 kg à vide, eau comprise), réservoirs (140 L sous pression formant le siège, 10 L en charge dans le plan supérieur ; 15 L d'huile ; environ 2 h d'autonomie). Il note au passage que les **monoplaces** SPAD ont des « mâts intérieurs divisés », absents du biplace. Critiques : instruments « assez mal disposés », le compte-tours étant posé à plat, si bien que le pilote lit les chiffres à l'envers ; pas de commande d'avance à l'allumage ; siège très étroit « comme sur tous les SPAD ».
  Biais : autre type d'avion. Utile pour la **famille** SPAD (construction, poste de pilotage), pas pour les qualités de vol, qu'il ne décrit pas.

- **Essais en vol allemands d'un S.VII capturé** : **toujours non trouvés**. Les communiqués allemands reproduits par *Flight* en 1917 (rubrique « In the hands of the enemy ») recensent des SPAD monoplaces alliés tombés de leur côté, britanniques et français : un « Spad » et un « Hispano-Spad » dans le numéro du 10 mai 1917, puis de quatre à treize « Spad » monoplaces par liste d'août à novembre 1917 (S58 [LU-OCR]). Ces listes sont peu sûres : le même numéro A6633 y apparaît en mai puis en septembre 1917 avec deux pilotes différents. Elles montrent seulement que les Allemands disposaient d'exemplaires. Piste : *Flugsport* de 1917-1918 (§ 9.6). Des S.VII capturés ont aussi été utilisés par les Allemands et les Austro-Hongrois (S5 [RÉSUMÉ]).

**Italien**

- **T14 — Francesco Baracca** · Italie, as (70a, puis 91a Squadriglia) · 1917 · S.VII, puis XIII · italien · S25, S26 [RÉSUMÉ].
  Il essaie le S.VII le 20 mars 1917 (S25). Il serait revenu du XIII au VII en octobre 1917, et S26 lui prête cette phrase (en anglais) : peu importe que le VII n'ait qu'une mitrailleuse, une seule suffit à un bon chasseur.
  **Citation non vérifiée sur l'original.** Ses lettres à sa mère forment le corpus principal (S25), conservé au Museo del Risorgimento de Milan et à la Biblioteca Trisi de Lugo (P13). Non consultées.

- **T15 — Sources italiennes secondaires** [RÉSUMÉ] :
  - S23 (Treccani) : le SPAD est supérieur aux Nieuport ; c'est sur lui que la 91a remporte ses victoires.
  - S24 (notice Baracchini) : moteur plus puissant que le Nieuport 11, « ottimi tempi di salita e caratteristiche di picchiata ».
  - S27 (musée de Vigna di Valle, S.153) : avion puissant et difficile à piloter, excellent en montée et en piqué ; apprécié pour sa vitesse en piqué et sa résistance aux dégâts malgré les problèmes de refroidissement et l'armement léger.
  Biais : sources commémoratives. La comparaison se fait **avec le Nieuport 11**, ce qui explique sans doute l'éloge de la montée, contraire à T3.

- **T16 — Accident de Luigi Olivari, 13 (ou 15) octobre 1917** · S28 [RÉSUMÉ] : son S.VII décroche au décollage à Santa Caterina, près d'Udine, accident mortel. C'est un **fait**, pas un témoignage.
  Quatrième passe : *Flight* du 18 octobre 1917 (p. 1087-1088, S59 [LU-OCR]) reprend le correspondant du *Daily Mail* : Olivari venait de décoller et n'était monté qu'à environ 260 ft (80 m) quand son avion est « tombé droit au sol ». **L'accident à basse hauteur juste après le décollage est donc confirmé par une source d'époque.** Le type d'avion et la cause (décrochage) ne sont donnés que par S28. Le récit de presse comporte une mise en scène (pressentiment, paroles sur la mort de Guynemer) à ne pas reprendre comme un fait.

**Pilotes de répliques modernes** : aucun témoignage publié trouvé sur une réplique de S.VII. Celle d'Old Rhinebeck vole avec un Lycoming de 150 ch (S34 [RÉSUMÉ]) et n'est donc pas représentative. Seul T9 (Tallman, avion original) a été repéré.

### 6.2 Traits de comportement

| Trait | Témoignages (sources, langues, camps) | Concordance | Attente vérifiable pour `essais.ron` | Confiance |
|---|---|---|---|---|
| **Structure robuste, tenue en piqué quasi vertical à pleine puissance** | T2 Guynemer (FR, 1916) [LU], et T2 bis, même épisode, non indépendant ; T6 Hall (EN, 1917) [LU] ; T13 Fieseler (DE) [RÉSUMÉ] ; T17 Rickenbacker (EN, 1919, réputation) [LU-OCR] ; T10, T15 (secondaires) | **Confirmé** : deux témoignages directs indépendants, plus des secondaires des deux camps. **Nuance d'époque** : le RAE estimait en avril 1917 que les piqués « verticaux » décrits par les pilotes sont plutôt de 45 à 60°, l'observateur surestimant toujours la pente (R&M 492, sur le S.E.5, S60 [LU-OCR]) | **M1, robustesse** : depuis le trim à 3 000 m, piqué de −80° gaz pleins pendant 20 s (situation extrême de T6). Attendu : pas de NaN ni de divergence ; le moment de tangage reste rappelant (pas d'engagement en piqué divergent, profondeur au neutre) ; la ressource au manche ramène l'avion en palier. Vitesse et facteur de charge atteints enregistrés. **Variante réaliste** : même essai à −60°, pente plus probable des piqués de combat selon S60. **M4** : aucune limite structurelle chiffrée n'est sourcée pour le SPAD ; ne pas en inventer | B (trait) ; aucun chiffre |
| **Montée du 150 ch jugée insuffisante face aux chasseurs allemands** | T3 Guynemer (FR, 1916) [LU] ; T6 Hall (EN, 1917, moteur reconstruit) [LU] ; contre : T15 (IT, face au Nieuport 11) | **Partiellement confirmé** : jugement relatif au comparant | Pas d'essai qualitatif. Cible **quantitative** : temps de montée STAé (§ 5.1). Contrôle : au même rapport puissance/masse, la montée du 150 ch ne doit pas dépasser les cibles | C |
| **Moteur sensible à la température** (surchauffe en montée prolongée en altitude, refroidissement rapide en plané) | T6 Hall (S.VII, 1917) [LU] ; T8 Meister (XIII, 1921) [RÉSUMÉ] ; T10, T15 ; S5 | **Confirmé pour la famille SPAD/Hispano** | **Hors M1** (pas de modèle thermique). Pour plus tard : en montée à pleine puissance au-delà de 3 500-4 000 m, volets de radiateur fermés, la température d'eau monte jusqu'à imposer une réduction ou un piqué ; en plané, elle chute vite | B− (qualitatif ; T8 vu en résumé seulement) |
| Boucle aisée | T1 Guynemer [LU] ; T4 (c), témoin de boucles après combat [LU] | **À confirmer** : un seul jugement de pilote | Provisoire, non bloquant : boucle depuis la vitesse de croisière à pleins gaz, bouclée sans décrochage au sommet | C |
| Vrille : entrée « paresseuse et irrégulière, mais très douce » | T1 Guynemer [LU] | **À confirmer** : un seul témoignage | Provisoire : `SpinEntryRecovery(turns: 3)`. Rotation établie lentement et irrégulière, sortie franche aux commandes croisées. Pas de seuil chiffré | C |
| Moins maniable que le Nieuport ; commandes lourdes | Secondaires : S5, S32/T10, S33, S34 [RÉSUMÉ] ; **contre** : T9 Tallman (commandes légères) [RÉSUMÉ] et T4 (b), Bordeaux (« facilité de conduite » supérieure à celle de l'Albatros) [LU] | **Contesté** ; le comparant change (Nieuport ou Albatros) | Aucune. Ne pas régler le modèle pour l'obtenir : le rayon de virage doit émerger de la charge alaire et de la puissance | — |
| Décrochage brutal à basse vitesse, mauvais plané, atterrissage au moteur | T9 Tallman [RÉSUMÉ] ; secondaires (`profil-spad7.md` A12 : forum, page Memorial Flight ou NASM) ; fait T16 (Olivari : chute à environ 80 m juste après le décollage, confirmée par la presse de 1917, cause non précisée par elle) ; S21 (XIII) | **À confirmer** : aucun témoignage direct lu | Provisoire, non bloquant : décrochage moteur réduit avec abattée nette et départ en roulis, plané plus pentu que celui d'un Nieuport. **Aucun seuil chiffré** tant qu'il n'est pas sourcé. Mécanismes physiques maintenant documentés, sans valeur SPAD : chute de portance nette après le maximum du RAF 15 à Reynolds élevé (`profil-spad7.md` § 3.1) ; décrochage du plan supérieur avant le plan inférieur (§ 2.4, S44). Ils doivent **produire** le trait, pas le remplacer | C |
| Très stable en vol, bonne plate-forme de tir | T8 Meister (XIII) [RÉSUMÉ] ; S5 [RÉSUMÉ] | **À confirmer** pour le VII | Provisoire : modes propres. Oscillation d'incidence bien amortie, spirale peu divergente | C |
| Couple marqué, embardée au décollage | T8 (XIII, rotation inverse) | **À confirmer** ; sens déduit pour le VII | M2 (sol) : à pleine puissance, l'avion part à gauche, contré au pied droit (déduit du sens de rotation, C) | C |
| Visibilité latérale et vers l'arrière limitée, angle mort sous le moteur | T4 (a) Bordeaux [LU] ; S9 [RÉSUMÉ] (avant et vers le bas médiocres) | **À confirmer** | Pour la caméra cockpit (M2+), pas pour le modèle de vol | C |
| Poste étroit, instruments mal disposés | T19 (rapport allemand, S.XI, 1918) [LU-OCR] : siège très étroit « comme sur tous les SPAD », compte-tours posé à plat et lu à l'envers | **À confirmer** pour le S.VII : un seul regard, sur un autre type | Pour le cockpit (M2+) : ne pas reproduire sans photographie d'un poste de S.VII | C |
| Au sol : roule bien, capote peu, s'arrête court | T8 (XIII) [RÉSUMÉ] | **À confirmer** | M2 (sol) | C |

### 6.3 Réputation

| Élément | Statut | Sources | Usage au récit |
|---|---|---|---|
| « Le terrible Spad », l'avion des as, arme d'attaque par excellence | **Confirmé** comme réputation (française et allemande) | T4, T11 [LU] ; T15 | Mess : fierté d'escadrille ; le joueur reçoit « un Spad » |
| « Le Spad tient » : la solidité qui sauve | **Confirmé** | T2, T6 [LU] ; T17 (« le Spad, plus solide » que le Nieuport) [LU-OCR] | Carnet de vol après un atterrissage brutal ; pilote qui « doit la vie à sa machine » |
| Le SPAD, étalon du chasseur rapide : les Britanniques surnomment un monoplace Albatros le « Spad allemand » (fin 1916) | **Confirmé** comme surnom d'époque | S54 (*Flight*, 28 déc. 1916) [LU-OCR] ; *The Aeroplane*, 10 janv. 1917 (titre relevé, non lu) | Dialogue d'escadrille : « leur Spad à eux » |
| L'avion que les pilotes veulent, après le Nieuport | **Confirmé** comme réputation, dans les deux camps | T18 (*Flugsport*, 1917) [LU-OCR] ; T7 [LU] ; T17 [LU-OCR] | Affectation du joueur ; jalousie des pilotes restés sur Nieuport |
| « Plus de 125 mph et une montée rapide » (mai 1916) | **Rumeur de presse** d'avant la mise en service | S54 (*Flight*, 25 mai 1916) [LU-OCR] | Journal lu au mess : la légende précède l'avion |
| Rapide, et surtout en piqué : on rompt le combat en piquant | **Confirmé** comme réputation ; chiffre de vitesse en piqué non sourcé | T4, T10, T15 ; T13 | Conseil d'ancien à la recrue : « ne tourne pas, pique » (formule de jeu, pas une citation) |
| « Plongée sûre jusqu'à 400 km/h » | **Légende probable** : chez Bordeaux, les 400 km/h désignent la **vitesse de rapprochement** de deux avions, pas une vitesse de piqué (S1 [LU]). Le chiffre circule ailleurs comme vitesse de piqué (résumé de recherche). Contexte d'époque : en avril 1917, le RAE juge que des vitesses de l'ordre de 200 mph (320 km/h) sont « rarement atteintes en pratique » par les chasseurs, même si le calcul les permet en piqué vertical (S60, sur le S.E.5) | S1 ; S60 [LU-OCR] ; résumé non attribué | À ne pas reprendre comme fait ; possible rumeur de mess |
| Le 150 ch « ne monte pas » face au Halberstadt | **Confirmé** (Guynemer) | T3 [LU] | Récit : pourquoi l'arrivée des 180 ch compte en 1917 |
| Hispano capricieux : surchauffe, fuites, mécaniciens indispensables | **Confirmé** | T5, T6 [LU] ; T8, T10 | Scènes de hangar, pannes en mission (à doser) |
| Une seule mitrailleuse | **Fait** ; jugement partagé (Baracca : elle suffit, S26, à vérifier) | S5, S6 ; T14 | Débat au mess |
| Moins maniable qu'un Nieuport | **Contesté** | § 6.2 | Préjugé de pilote venu du Nieuport |
| « Tombe comme une pierre » à basse vitesse, se pose au moteur | **À confirmer** (rumeur probable) | T9 ; forums | Avertissement de moniteur ; peur de la recrue |
| Guynemer réclame plus de puissance et un canon, d'où les SPAD XII et XIII | **Partiellement confirmé** : les lettres existent (T3, et S1 sur l'« avion magique") ; le lien de cause à effet est affirmé par des secondaires (S8, S18) | S1 [LU] ; S8, S18 | Contexte historique |
| Udet épargné par Guynemer (mitrailleuse enrayée) | **Légende / contesté** : source unique et tardive | T12 | Récit de mess raconté comme on-dit, jamais comme un fait |
| Le *Vieux-Charles* introuvable | **Rapporté** par Bordeaux : le numéro n'apparaît sur aucune liste allemande d'appareils abattus, et la tombe n'a pas été retrouvée | S1 [LU] | Uniquement avec sobriété ; personnages réels |
| L'avion convoité par les recrues de 1918 | **Témoignage romancé** | T7 [LU] | Affectation du joueur |

### 6.4 Témoignages repérés mais non consultés (pistes)
- **Français** : René Fonck, *Mes combats* (Flammarion, 1920 ; P8), SPA 103, même GC 12 ; Albert Deullin, « La chasse en monoplace » (1917 ; P9) ; journaux de marche des SPA 3 et 73 (SHD) ; original français de Bordeaux (P18).
- **Anglais** : Charles J. Biddle, *The Way of the Eagle* (Scribner, 1919 ; P7). Lettres de la SPA 73 (GC 12), en Flandre fin 1917 : la source la plus proche du contexte de jeu. Hall et Nordhoff, *The Lafayette Flying Corps* (1920 ; P15). E. Rickenbacker, *Fighting the Flying Circus* (1919 ; P7 bis), à lire en entier plutôt que dans le résumé de *Flight*. *Scientific American*, « Spad pursuit biplane », vol. 118, 2 février 1918, p. 107 (relevé dans S56).
- **Allemand** : rapports de Jasta et notes de l'Idflieg sur les appareils capturés ; texte complet d'Udet (projekt-gutenberg.org, inaccessible). *Flugsport* 1917-1918 : la revue publiait des examens d'appareils capturés (T19) ; chercher un S.VII. *Deutsche Luftfahrer-Zeitschrift*, 22 mai 1918, p. 20-21 : « Der Spad-Doppeldecker mit geteiltem Rumpf » (relevé dans S56 ; type à identifier).
- **Italien** : lettres de Baracca (P13) ; G. Alegi et P. Varriale, *Italian SPADs at War* (Windsock Datafile 155 ; P5).

## 7. Armement
- **Une** mitrailleuse Vickers de calibre .303 (7,7 mm), au-dessus du moteur, synchronisée pour tirer à travers le champ de l'hélice (S5, S6 [RÉSUMÉ]). Synchronisation **Birkigt** d'après une source spécialisée non identifiée dans le résumé : à vérifier.
- Une deuxième mitrailleuse a été envisagée puis abandonnée, car elle dégradait les performances (S6 [RÉSUMÉ]). Un appareil britannique, doté d'une Lewis supplémentaire sur le plan supérieur, a été essayé à Martlesham Heath en mai 1917 : perte de performances jugée trop forte (S5 [RÉSUMÉ]).
- Fusées pour l'attaque des ballons : six, sur des supports fixés aux mâts (trois de chaque côté), avec une plaque de tôle protégeant l'entoilage du plan inférieur sous les supports ; mise à feu électrique par un bouton dans le poste (S3 [LU], Escadrille Lafayette, 1917). Hall ne nomme pas le type de fusée : « Le Prieur » est à vérifier.
- Nombre de cartouches : **non trouvé**. Masse de l'armement dans la charge d'essai : 40 kg (88 lb), une Vickers frontale (S46 [LU-IMG]) ; valeur conventionnelle, commune à tous les monoplaces à une Vickers du tableau (§ 5.5.1).
- Synchronisation, indice de famille : sur le biplace S.XI examiné par les Allemands, la Vickers du pilote est commandée par une tige poussée depuis l'arbre à cames droit, et la détente est sur le manche (S49 [LU-OCR]). Cela concorde avec une synchronisation mécanique de type Birkigt, **sans preuve pour le S.VII** (C).
- *Aerial Age* (S48, 1917) écrit que les S.VII de l'Escadrille Lafayette portent « une et parfois deux » mitrailleuses Lewis ou Vickers. **Non retenu** : affirmation de presse isolée, contraire aux autres sources sur l'armement du S.VII.
- Pour mémoire : Guynemer constate à l'automne 1916 que les enrayages de sa mitrailleuse viennent du froid, et trouve un remède (S1 [LU]).

## 8. Livrées (bref ; le M1 ne les utilise pas)
- **Premières séries** : toile vernie incolore ou teinte « écru » / beige pâle, capots métalliques un peu plus foncés (S37 [RÉSUMÉ], descriptions divergentes). Indice d'époque pour la famille : le biplace S.XI construit en juillet 1917 et examiné par les Allemands avait voilure et fuselage peints « blanc jaunâtre » (S49 [LU-OCR]). S48 (1917) décrit un capot avant en aluminium, ouvert sur un radiateur circulaire, avec des bossages latéraux là où dépassent les carters d'arbres à cames, et des perforations d'aération autour.
- **Plus tard** : camouflage français à cinq couleurs. Les meilleures études citées sont les articles « Project Butterfly » de Hardesty, Hastings et Toelle, *Cross & Cockade* (USA) vol. 9 (1968) et vol. 13 (1972) (P14).
- **Insigne de la SPA 3** : cigogne en vol, ailes **basses** (fin du battement). Deux sources concordantes : S36 [RÉSUMÉ], lettre de lecteur à *Motor Sport* (1976), et S20 [RÉSUMÉ]. Les autres escadrilles du GC 12 ont des cigognes différentes (S36). À vérifier sur photographies datées avant tout dessin.
- **Marques individuelles** : nom « Vieux Charles » sur les SPAD de Guynemer (S1 ; S17, S29) ; numéro « 2 » sur S.504 (S19).
- **Règle** : redessiner à partir de plusieurs photographies d'époque ; ne recopier aucun profil couleur publié.

## 9. Sources

### 9.1 Lues [LU] (texte intégral, consulté le 2026-10-20)
- **S1** — Henry Bordeaux, *Georges Guynemer: Knight of the Air*, trad. Louise Morgan Sill, introd. Theodore Roosevelt, Yale University Press, New Haven, 1918. Project Gutenberg n° 18117, texte ISO-8859-1 via GITenberg : https://raw.githubusercontent.com/GITenberg/Georges-Guynemer--Knight-of-the-Air_18117/master/18117-8.txt (consulté le 2026-10-20). Type : secondaire (biographie de guerre), qui contient des lettres et carnets de Guynemer (primaires, traduits). L'original français n'a pas été consulté (P18).
- **S2** — Manfred Freiherr von Richthofen, *Der rote Kampfflieger*, Ullstein & Co, Berlin-Wien, 1917 (151.–200. Tausend). Project Gutenberg n° 24572 via GITenberg : https://raw.githubusercontent.com/GITenberg/Der-rote-Kampfflieger_24572/master/24572-8.txt (consulté le 2026-10-20). Chapitres « Das Anti-Richthofen-Geschwader (25. April 1917) » et « Der "alte Herr" kommt uns besuchen ». Type : primaire (mémoires de guerre, propagande).
- **S3** — James Norman Hall, *High Adventure: A Narrative of Air Fighting in France*, Houghton Mifflin, Boston et New York, juin 1918. Project Gutenberg n° 24570 via GITenberg : https://raw.githubusercontent.com/GITenberg/High-Adventure--A-Narrative-of-Air-Fighting-in-France_24570/master/24570-8.txt (consulté le 2026-10-20). Type : primaire (lettres et récit).
- **S4** — Charles Bernard Nordhoff, *The Fledgling*, Houghton Mifflin, Boston et New York, 1919. Project Gutenberg n° 51559 via GITenberg : https://raw.githubusercontent.com/GITenberg/The-Fledgling_51559/master/51559-8.txt (consulté le 2026-10-20). Type : primaire, en partie romancé.

**Lues en troisième passe** : rapports NACA téléchargés depuis NASA NTRS par le coordinateur le 2026-10-20 (PDF et texte extrait). Lus dans le dossier de travail ; chiffres douteux vérifiés sur l'image des pages ; pagination imprimée.
- **S43** — Alexander Klemin, Edward P. Warner et George M. Denkinger (Massachusetts Institute of Technology), « An Investigation of the Elements Which Contribute to Statical and Dynamical Stability, and of the Effects of Variation in Those Elements », NACA Report No. 17, rapport annuel du NACA, p. 273-340. Essais de l'été 1917 ; 1918 d'après l'index NTRS. NTRS 19930091044, https://ntrs.nasa.gov/citations/19930091044. Pages utilisées : 289-290 (interférence biplan) et 339-340 (résumés). Primaire.
- **S44** — A. J. Fairbanks (Langley Memorial Aeronautical Laboratory), « Pressure Distribution Tests on PW-9 Wing Models Showing Effects of Biplane Interference », NACA Report No. 271. Texte daté du 7 avril 1927 ; 1928 d'après l'index NTRS. NTRS 19930091339, https://ntrs.nasa.gov/citations/19930091339. Pages 3 et 11 à 13. Primaire.
- **S45** — Clark B. Millikan (California Institute of Technology), « An Extended Theory of Thin Airfoils and Its Application to the Biplane Problem », NACA Report No. 362. Texte daté du 15 mars 1930 ; 1931 d'après l'index NTRS. NTRS 19930091433, https://ntrs.nasa.gov/citations/19930091433. Pages 658 à 662 (vérifiées sur l'image pour 659 à 661). Primaire (théorie confrontée à l'expérience).
- Rapports NACA sur le profil et le moteur : voir `profil-spad7.md` § 6.1 (A1, A4, A15 à A18, A21) et `hispano-suiza-8a.md` § 9 (H14 à H17).

**Lues en quatrième passe [LU-OCR]** : texte OCR d'archive.org téléchargé par le coordinateur le 2026-10-20 et lu comme des données. Pour *Flight*, l'identifiant archive.org est `Flight_International_Magazine_AAAA-MM-JJ-pdf`, à l'adresse `https://archive.org/details/<identifiant>`. Les pages sont la pagination imprimée, repérée dans l'OCR (à ±1 page près quand un article chevauche deux pages).
- **S46** — « French Aeroplanes in Service at the Front » et « French Aeroplanes under Test », tableaux compilés par le service aéronautique américain, publiés par *Automotive Industries* (New York ; numéro non identifié, début 1919), reproduits dans *Flight*, vol. 11, 13 février 1919, p. 201-207 (présentation en tête de la p. 201, « Some interesting statistics », vérifiée sur l'image en huitième passe ; *Flight* y nomme la revue *Automobile Industries*). Même numéro : « Complete technical specifications of important American and foreign aeroplane engines » et tableaux de performances des moteurs. Identifiant `Flight_International_Magazine_1919-02-13-pdf`. Primaire (compilation officielle). OCR désordonné ; **lu sur l'image en cinquième passe** (complément ci-dessous). Le tableau des avions français occupe les p. 204-205.
- **S47** — « Comparative Chart of Aeroplanes in the Italian Service », même origine américaine, *Flight*, 20 février 1919, **p. 237** (présentation p. 238, « More statistics » ; la p. 238 porte le tableau des avions américains). `Flight_International_Magazine_1919-02-20-pdf`. Primaire (compilation). **[LU-IMG]** en cinquième passe : ligne « Spad One Seater, flat-winged ».
- **Complément à S46 (cinquième passe)** : pages lues sur l'image [LU-IMG], images PNG à 150 dpi des pages 199 à 207 tirées du PDF de l'item `Flight_International_Magazine_1919-02-13-pdf`, fournies par le coordinateur le 2026-10-21. Les en-têtes du tableau des avions sont p. 204 ; les deux lignes « Spa. 7C1 » sont en tête de la p. 205 ; les tableaux des moteurs sont p. 201 (caractéristiques), 202 (accessoires) et 203 (consommations, masses, puissances estimées en altitude). **Sixième passe** : haut de la p. 205 en image à 300 dpi (`f19190213-p205-300dpi-15.png`, 4 013 × 2 700 px), fourni par le coordinateur le 2026-10-22, tiré du même PDF.
- **S63** — F. H. Norton (Langley), « The Effect of Staggering a Biplane », NACA Technical Note n° 70, septembre 1921 (essais du MIT de 1918). NTRS 19930080864, https://ntrs.nasa.gov/citations/19930080864, téléchargé par le coordinateur le 2026-10-21. **[LU]** : p. 1-3 et figure 1 sur l'image. Primaire.
- **S48** — « The Spad Scout », d'après *Aerial Age Weekly* (New York, vol. 5, n° 17, 2 juillet 1917, p. 569, selon S56), reproduit dans *Flight*, rubrique « From other lands », 16 août 1917, p. 840-841. `Flight_International_Magazine_1917-08-16-pdf`. Secondaire d'époque (presse) ; cotes dites « approximatives ».
- **S49** — « The Spad Two-Seater, 200 h.p. Hispano-Suiza engine », traduction d'un rapport allemand « apparemment officiel » publié par *Flugsport* le 10 avril 1918, *Flight*, 16 mai 1918, p. 527-529. `Flight_International_Magazine_1918-05-16-pdf`. Primaire (examen d'un appareil capturé), sur le **S.XI**.
- **S50** — Article sur la coupe Schneider 1919, *Flight*, 4 septembre 1919, p. 1184-1185 : schéma de haubanage « familier des SPAD ». `Flight_International_Magazine_1919-09-04-pdf`. Secondaire d'époque.
- **S51** — Liste des brevets publiés, *Flight*, 14 décembre 1916, fin du numéro (après la p. 1102) : brevet n° 100,632 de la Société anonyme Spad, « Strutting and staying of aerial machines ». `Flight_International_Magazine_1916-12-14-pdf`. Piste (brevet non lu).
- **S52** — « A Modern Bayard », compte rendu de la série de Jacques Mortane sur Guynemer dans *La Guerre aérienne*, *Flight*, 18 juillet 1918, p. 798-799. `Flight_International_Magazine_1918-07-18-pdf`. Secondaire d'époque (citation traduite).
- **S53** — Résumé du livre de Rickenbacker, *Flight*, 29 mai 1919, p. 706-707. `Flight_International_Magazine_1919-05-29-pdf`.
- **S54** — Échos de presse : *Flight*, 25 mai 1916, p. 450-451 ; 28 décembre 1916, vers p. 1146 ; 22 novembre 1917, p. 1217 ; 31 janvier 1918, p. 111-112. Identifiants sur le même modèle.
- **S55** — « Captain Heurteaux wounded », *Flight*, 4 octobre 1917, p. 1028. `Flight_International_Magazine_1917-10-04-pdf`.
- **S56** — Paul Brockett, *Bibliography of Aeronautics 1917-1919* (Smithsonian Institution, publié en 1921), archive.org `bibliographyofa191721broc`. Outil de repérage : entrées « Spad », « Hispano-Suiza », « Wright-Martin ».
- **S57** — Article de *Flugsport* sur les chasseurs français (Nieuport), traduit dans *Flight*, 30 août 1917, p. 885 environ. `Flight_International_Magazine_1917-08-30-pdf`.
- **S58** — Listes allemandes d'appareils alliés tombés dans leurs lignes, rubrique « In the hands of the enemy » : *Flight*, 10 mai, 2 août, 9 août, 6 septembre, 1er novembre et 29 novembre 1917.
- **S59** — Mort de Luigi Olivari d'après le *Daily Mail*, *Flight*, 18 octobre 1917, p. 1087-1088. `Flight_International_Magazine_1917-10-18-pdf`.
- **S60** — Royal Aircraft Factory, « The Terminal Velocity of S.E.5 », ARC R&M n° 492, avril 1917, dans *Technical Report of the Advisory Committee for Aeronautics for the Year 1917-18*, vol. II, HMSO, Londres, 1921, p. 712-713. archive.org `technicalreport1917aero_2`. Primaire (calcul de l'établissement d'essais).
- **S61** — L. Prandtl, « Induced Drag of Multiplanes », NACA Technical Note n° 182, 1924 (traduction de *Technische Berichte*, vol. III, n° 7, p. 309-315). NTRS 19930080964, https://ntrs.nasa.gov/citations/19930080964, téléchargé par le coordinateur le 2026-10-21. **[LU]** : p. 1-2 et 5-13 sur l'image du PDF. Primaire (théorie).
- **S62** — P. E. Hemke (Langley), « Drag of Wings with End Plates », NACA Report n° 267, p. 253 et suivantes. NTRS 19930091335, https://ntrs.nasa.gov/citations/19930091335, téléchargé le 2026-10-21. **[LU]** : résumé et p. 255-257 sur l'image. Primaire.

**Lues en huitième passe (2026-10-22)**
- **S66** — « Record of Performances of British Aeroplanes (From information supplied by the U.S. Government) », *Flight*, 13 février 1919, p. 206. Même item que S46. **[LU-IMG]** (PNG à 150 dpi). Utilisé pour la définition du plafond pratique (« service ceiling » à 100 ft/min, note du tableau) et comme comparaison. Primaire (compilation).
- **S67** — « Record of Performance of American Planes », *Flight*, 20 février 1919, p. 238 (présentation « More statistics », même page). Même item que S47. **[LU-IMG]**. Utilisé pour la perte de vitesse en altitude de deux S.E.5 à moteur Hispano et du VE-7, avec leurs régimes. Primaire (compilation).
- **S68** — « Wounded on Trial Trip » (Heurtaux), *Flight*, 11 octobre 1917, p. 1063. `Flight_International_Magazine_1917-10-11-pdf`. **[LU-OCR]**. Presse d'époque.

**Lues en dixième passe (2026-10-24)**. Pagination lue dans l'OCR, à une page près.
- **S75** — Capt. H. T. Tizard (RFC), « Methods of Measuring Aircraft Performances ». Conférence lue devant l'Aeronautical Society of Great Britain le 7 mars 1917, reproduite par *Flight* en trois parties : 8 mars 1917, p. 232-233 ; 15 mars 1917, p. 252-254 ; 22 mars 1917, p. 278-280, discussion comprise. Items `Flight_International_Magazine_1917-03-08-pdf`, `…-1917-03-15-pdf` et `…-1917-03-22-pdf`. **[LU-OCR]** ; tableaux en partie désordonnés dans l'OCR. Primaire : méthodes du Testing Squadron du RFC. Passages utilisés : note sur les altitudes d'essai françaises ; anéroïde bloqué et correction de température ; relevés de montée et réduction par intégration ; tableau IV (essai du 27 décembre 1916) ; définition du plafond ; choix des hauteurs types. Publiée aussi dans *The Aeronautical Journal* en 1917 (notice Cambridge, https://resolve.cambridge.org/core/journals/aeronautical-journal/article/abs/methods-of-measuring-aircraft-performances/F201C6804CFF2FB0E852A74B2E4E78E2, vue en résumé le 2026-10-24).
- **S76** — Royal Aero Club, « Certified Trials » : règlements des essais de vitesse ascensionnelle et de hauteur, *Flight*, 3 juillet 1919, p. 872-873 environ (avis officiels du club). `Flight_International_Magazine_1919-07-03-pdf`. **[LU-OCR]**. Primaire (règlement civil).
- **S77** — W. H. Dines, « Meteorology in Relation to Aeronautics », *Flight*, 20 décembre 1917, p. 1344-1346 environ. `Flight_International_Magazine_1917-12-20-pdf`. **[LU-OCR]**. Primaire, écrite par un météorologue ; ne pas confondre avec J. S. Dines, dont Tizard cite les observations. Utilisée pour les variations de température et de densité au sol, formule du nivellement barométrique.
- **S78** — G. Hartmann, « Les héliciers français », PDF de 23 pages, hydroretro.net, 2008, p. 12 : centre d'essais du Service technique de l'aéronautique à Villacoublay ; réception des Farman à Buc. Même document que H9 de `hispano-suiza-8a.md`, téléchargé par le coordinateur (`sources/hartmann/heliciersfrancais.txt`). **[LU]**. Secondaire.

### 9.2 Vues seulement en résumé de recherche [RÉSUMÉ] (pages non ouvertes ; consultées le 2026-10-20)
- **S5** — Wikipedia (en), « SPAD S.VII », https://en.wikipedia.org/wiki/SPAD_S.VII (wiki : piste).
- **S6** — Wikipédia (fr), « SPAD S.VII », https://fr.wikipedia.org/wiki/SPAD_VII (wiki : piste).
- **S7** — Musée de l'Air et de l'Espace, « Spad VII », https://www.museeairespace.fr/aller-plus-haut/collections/spad-vii/ (secondaire institutionnelle).
- **S8** — avionslegendaires.net, « SPAD S.VII », https://www.avionslegendaires.net/avion-militaire/spad-s-vii/ (vulgarisation).
- **S9** — Aeropedia (David C. Eyre), « Spad », https://aeropedia.com.au/content/spad/ (vulgarisation).
- **S10** — Encyclopédie des armes, « SPAD VII », https://www.encyclopedie-des-armes.com/index.php/aviation/avions-de-combat/1173-spad-vii (vulgarisation).
- **S11** — The Aerodrome (forum), fil « Climbing performance of the SPAD VII », https://theaerodrome.com/forum/threads/climbing-performance-of-the-spad-vii.30985/ (forum : piste). Il cite un résumé du STAé pour le S.VII 150 ch et le rapport d'essai britannique n° 96 d'avril 1917.
- **S12** — flyingmachines.ru, « SPAD S.VII (Spa 7C-1) », https://flyingmachines.ru/Site2/Crafts/Craft25466.htm (compilation d'ouvrages : piste).
- **S13** — polot.net, « SPAD VII, SPAD XIII, SPAD 51, SPAD 61. 1916 », https://polot.net/en/spad-vii-spad-xiii-spad-51-spad-61-1916-912 (vulgarisation).
- **S14** — San Diego Air & Space Museum, « Artifact Spotlight: The SPAD VII », https://sandiegoairandspace.org/newsletters/article/artifact-spotlight-the-spad-vii (institutionnelle, exemplaire B9916).
- **S15** — This Day in Aviation, articles sur le SPAD S.XIII C.1 et le rapport de McCook Field du 27 avril 1921 : https://www.thisdayinaviation.com/tag/spad-s-xiii-c-1/ et https://thisdayinaviation.com/?p=49509. Ils renvoient à l'Air Service Information Circular vol. III n° 286 (1er octobre 1921).
- **S16** — migflug.com, « SPAD S.VII » (https://migflug.com/aircraft/spad-s-vii/) et « SPAD S.XIII » (https://migflug.com/aircraft/spad-xiii/) (vulgarisation).
- **S17** — Modeling Madness, revue de la maquette Airfix 1/72 SPAD VII, https://modelingmadness.com/review/w1/wentzelspad7.htm (piste).
- **S18** — Tank Poelkapelle, « A word of history about Guynemer… », https://www.tankpoelcapelle.be/en/node/24 (secondaire locale).
- **S19** — Western Front Association, « 11 September 1917: Capt Georges Guynemer », https://www.westernfrontassociation.com/on-this-day/11-september-1917-capt-georges-guynemer/ ; horizon14-18.eu, https://horizon14-18.eu/georges-guynemer.html (qui cite Willy Coppens pour l'heure de départ).
- **S20** — Wikipedia (en), « Groupe de Combat 12 », https://en.wikipedia.org/wiki/Groupe_de_Combat_12, et « Escadrille 3 », https://en.wikipedia.org/wiki/Escadrille_3 ; miroir alchetron, https://alchetron.com/Escadrille-3 (wikis : pistes).
- **S21** — Wikipedia (en), « SPAD S.XIII », https://en.wikipedia.org/wiki/SPAD_S.XIII ; Wings of Linen, « SPAD 13 », https://linen.miraheze.org/wiki/SPAD_13 (wikis : pistes).
- **S22** — The Aerodrome, fiche « Charles John Biddle », https://theaerodrome.com/aces/usa/biddle.php.
- **S23** — Treccani, *Dizionario Biografico degli Italiani*, « BARACCA, Francesco », https://www.treccani.it/enciclopedia/francesco-baracca_(Dizionario-Biografico)/.
- **S24** — Wikipedia (it), « Flavio Torello Baracchini », https://it.wikipedia.org/wiki/Flavio_Torello_Baracchini ; « 91ª Squadriglia aeroplani da caccia », https://it.wikipedia.org/wiki/91%C2%AA_Squadriglia_aeroplani_da_caccia ; Wikipedia (en), « 77a Squadriglia » (wikis : pistes).
- **S25** — *1914-1918-online. International Encyclopedia of the First World War*, « Baracca, Francesco », https://encyclopedia.1914-1918-online.net/article/baracca_francesco.
- **S26** — Wikipedia (en), « Francesco Baracca », https://en.wikipedia.org/wiki/Francesco_Baracca (wiki : piste ; citation non vérifiée).
- **S27** — Descriptions du SPAD VII S.153 du musée de Vigna di Valle : https://www.targeta.co.uk/vigna_di_valle3.htm ; flugzeuginfo.net (secondaires).
- **S28** — Wikipedia (en), « Luigi Olivari », https://en.wikipedia.org/wiki/Luigi_Olivari (wiki : piste).
- **S29** — Ernst Udet, *Mein Fliegerleben* (Deutscher Verlag, 1935 ; édition Projekt-Gutenberg de 1942, https://projekt-gutenberg.org/authors/ernst-udet/books/mein-fliegerleben/, inaccessible). Vu par les résumés de https://www.sim-outhouse.org/sohforums/resources/alb_dv-udet-zip.4106/updates et https://combatace.com/forums/topic/87451-check-this-out-guys (forums).
- **S30** — Page biographique de Gerhard Fieseler, https://www.fliegerabteilung.com/fieseler-lebenslauf.htm (secondaire).
- **S31** — Frank Tallman, *Flying the Old Planes*, Doubleday, 1973. Vu par le fil CombatACE « Flying The Old Planes - Part 2: SPAD VII », https://combatace.com/forums/topic/43914-flying-the-old-planes-part-2-spad-vii/ (attribution incertaine), et par Aero Vintage Books, « Tallmantz SPAD N4727V », https://www.aerovintage.com/tallmantz-aviation/tallmantz_spad_n4727v/ (moteur Wright-Hispano E de 180 ch).
- **S32** — US Air Force History and Museums Program, *Like a Thunderbolt: The Lafayette Escadrille and the Advent of American Pursuit in World War I*, PDF : https://www.afhistoryandmuseums.af.mil/Portals/56/Documents/WWI/LafayettePama.pdf (secondaire institutionnelle ; auteur non relevé).
- **S33** — Hangar47, « Spad VII C 1 preview » (maquette Special Hobby), https://hangar47.com/spad-vii-c-1-preview/ (fiche de maquette).
- **S34** — Old Rhinebeck Aerodrome, « SPAD VII », https://oldrhinebeck.org/spad-vii/ (réplique à moteur Lycoming) ; CombatACE, « It's time to play: what's my plane », https://combatace.com/forums/topic/46006-its-time-to-playwhats-my-plane/page/4/ (forum).
- **S35** — SilverHawkAuthor, « Canadian Warplanes 1: S.P.A.D. S.VII Scout », https://silverhawkauthor.com/?p=50402, et « Aircraft preserved in the Canada Aviation & Space Museum: SPAD S.VII » (B9913), https://silverhawkauthor.com/?p=50235.
- **S36** — *Motor Sport*, « That Stork », mai 1976, https://www.motorsportmagazine.com/archive/article/may-1976/37/that-stork/ (lettre de lecteur). Pour la Russie : IPMS USA, revue de la maquette « SPAD VII c.1 (w/Skis) », https://reviews.ipmsusa.org/review/spad-vii-c1wskis (piste).
- **S37** — Aeroscale, revue https://aeroscale.kitmaker.net/review/4494, et forum https://archive.aeroscale.net/forums/90429-page=2.htm (finitions d'usine ; pistes).
- **S38** — Société Spad, *Notice de réglage et d'entretien du biplan monoplace type S XIII, moteur Hispano-Suiza de 200 HP*, Paris, [1917 ?], 11 p. Notice du catalogue des Bibliothèques numériques de la Défense, https://bibliotheques-numeriques.defense.gouv.fr/index.php/document/fc97b662-8b01-43a8-9698-6e0be5953006 (primaire, non consultée : site inaccessible).
- **S39** — **Exclues** (données de jeux ou dérivées) : https://aergistal.github.io/il2/planes/spad7early.html et spad7late.html (IL-2) ; https://www.checksix-forums.com/viewtopic.php?t=171094 (tableau de vitesses indiquées repris des jeux) ; https://riseofflight.com/store/aircraft/spad-7c1/ ; https://www.monsieurminiatures.com/avions-de-wings-of-glory-ww1-spad/.
- **S40** — Aircrew Remembered, « Fonck René », https://aircrewremembered.com/fonck-rene.html, et notices voisines (ww2gravestone.com, techno-science.net). Elles placent Fonck à la SPA 103 en avril 1917 et le font voler sur SPAD VII, XIII et XII à canon.
- **S41** — The Aerodrome (forum), fil « Ailerons », https://theaerodrome.com/forum/threads/ailerons.69210/ ; Modeling Madness, présentation de la maquette du SPAD XIII, https://modelingmadness.com/review/w1/michaelsspad13preview.htm (pistes ; l'attribution exacte de chaque affirmation entre ces deux pages n'est pas certaine).
- **S42** — CGTrader, « SPAD VII » (description d'un modèle 3D), https://www.cgtrader.com/3d-models/aircraft/military-aircraft/spad-vii (piste).

**Septième passe (consultées en résumé le 2026-10-22)**
- **S15 (complément)** — *This Day in Aviation*, mêmes pages que S15 (https://www.thisdayinaviation.com/tag/spad-s-xiii-c-1, https://thisdayinaviation.com/?p=49509, https://www.thisdayinaviation.com/tag/louis-bechereau/), revues en résumé le 2026-10-22 : plan fixe, ailerons, dérive, surface de 227 sq ft, train. Page toujours inaccessible à l'outil de lecture (échec DNS).
- **S64** — Bibliothèques numériques de la Défense, notice bibliographique de J. M. Bruce, *Spad 13.C1*, éd. R. L. Rimell, Windsock Datafile n° 32, Albatros Productions, Berkhamsted, réimpression 2000, 36 p., ISBN 0-948414-39-1 (don de G. Magnier), https://bibliotheques-numeriques.defense.gouv.fr/document/b6ed97b4-3667-4988-83ce-ebdd94fbf370. Notice seulement ; l'ouvrage est un exemplaire papier (P24).
- **S65** — Smithsonian, National Air and Space Museum, notice de l'objet « Spad XIII "Smith IV" », https://avpreservation.si.edu/object/nasm_A19200001000 (institutionnelle ; envergure 8,2 m et hauteur 2,4 m selon le résumé, mode de mesure inconnu).
- **S37 (complément)** — La revue d'Aeroscale (https://aeroscale.kitmaker.net/review/4494) contient, selon le résumé du 2026-10-22, une bibliographie d'articles sur le SPAD 7 (P19 à P21).

**Huitième passe (consultées en résumé le 2026-10-22)**
- **S69** — Service historique de la Défense, notice du « Fonds Émile Dorand - DE 2012 ZL », producteur : Section technique de l'aéronautique militaire (1915-1918) ; la notice cite des procès-verbaux de la Commission d'examen des avions et moteurs nouveaux et des notes du Service des essais (sans nommer le SPAD dans l'extrait vu). Pages : https://servicehistorique-admin.cnmosis.dirisi.defense.gouv.fr/en/node/707 ; geneawiki, « SHD / Air - Série (DE)ZL », https://fr.geneawiki.com/wiki/SHD_/_Air_-_S%C3%A9rie_(DE)ZL, qui donne l'article 2012 ZL 245 (1895-1922) et un inventaire PDF de 2012. Échec DNS de l'outil de lecture sur les deux pages.
- **S70** — enginehistory.org, « UK National Archives at Kew » (https://enginehistory.org/References/UKNA/Eng-AIR.shtml), d'après un résumé : pièce **AIR 1/1196/204/5/2600**, rapports hebdomadaires sur la situation des avions en essai à l'Aeroplane Experimental Station de Martlesham Heath (n° 1 à 119, 1916-1919). Présence d'un SPAD non confirmée. Échec DNS de l'outil de lecture.

**Neuvième passe (consultées en résumé le 2026-10-23 ; échec DNS de l'outil de lecture sur toutes ces pages)**
- **S71** — Memorial Flight, « SPAD XIII », https://www.memorial-flight.com/spad-xiii?lang=en : bords d'attaque de toutes les ailes recouverts de contreplaqué (XIII). Association de constructeurs et restaurateurs de répliques (rang 3).
- **S72** — Hyperscale, revues de maquettes Eduard au 1/48 du SPAD XIII par Rob Baumgartner : https://www.hyperscale.com/2008/reviews/kits/eduard8196reviewrb_1.htm et https://www.hyperscale.com/2009/reviews/kits/eduard1142reviewrb_1.htm. Coffrage du bord d'attaque sur l'extrados seulement. Revues de maquettes : piste, jamais source de chiffre.
- **S73** — The Aerodrome (forum), « Plywood used as wing surface on WW1 aircraft? », https://theaerodrome.com/forum/threads/plywood-used-as-wing-surface-on-ww1-aircraft.12945/ : contreplaqué de bouleau de 1,5 à 2,0 mm sur l'extrados, du bord d'attaque au longeron avant, chez « Albatros, SPAD, Nieuport ». Forum : piste.
- **S74** — The Aerodrome (forum), « My Spad Projects » (William Retzlaff), https://theaerodrome.com/forum/threads/my-spad-projects.27757/ : construction du bord d'attaque du XIII ; remarque sur le S-7 ; plans d'usine survivants (environ 60 % pour le S-13, 25 % pour le S-7 ; calques attribués à Roger Freeman, copies chez Leo Opdycke). Forum : piste.
- **S11 (complément)** — même fil que S11, résumé du 2026-10-23 : « Air Board summary », Hispano de 150 hp coté à 1 600 tr/min (1 800 tr/min au maximum), renvoi au rapport M.96 et à N6210 « supposé » ; résumés contradictoires (§ 5.6).
- Hélices : sources H8 à H10 et H24 à H36 de `hispano-suiza-8a.md` § 9.

**Dixième passe (consultées en résumé le 2026-10-24 ; échec DNS de l'outil de lecture sur toutes ces pages)**
- **S79** — P. Grimault, « On the definition of the standard atmosphere », NACA Technical Memorandum n° 15, 1er avril 1921, traduction du bureau de Paris du NACA. NTRS 19930080725, https://ntrs.nasa.gov/citations/19930080725. Contenu vu en résumé : adoption, le 15 avril 1920, de la « loi de la S.T.Aé. » comme atmosphère type des essais officiels français ; renvoi à A. Toussaint (*L'Aéronautique*, n° 5, octobre 1919) ; exemple d'un Breguet XIV à 3 000 m.
- **S80** — Service de l'information aéronautique (SIA), AIP France, carte et fiche de l'aérodrome LFPV (Vélizy-Villacoublay), par exemple https://www.sia.aviation-civile.gouv.fr/media/dvd/eAIP_09_JUL_2026/Atlas-VAC/PDF_AIPparSSection/VACH/AD/AD-3.LFPV.pdf : altitude de référence de 585 ft. Officielle, mais moderne.
- **S81** — Comité pour l'histoire de l'aéronautique (Académie des technologies), *Un demi-siècle d'aéronautique en France : centres et moyens d'essais*, t. 1, coord. J.-P. Marec, 2013, PDF (adresse au § 5.7.7). Vu en résumé : section des essais en vol créée à Villacoublay en 1915, rattachée au Service technique ; contrôle du temps de montée, des vitesses en palier et du plafond. Secondaire institutionnelle.
- **S82** — G. Hartmann, « Le grand concours d'aviation militaire, Reims 1911 », hydroretro.net, PDF (adresse au § 5.7.7). Vu en résumé : règlement publié en octobre 1910. Secondaire.
- **S83** — Pistes sur les épreuves de 1911 :
  - document en allemand de la Dolnośląska Biblioteka Cyfrowa, https://dbc.wroc.pl//Content/18456/PDF/20621.pdf (titre non relevé) : 500 m « au-dessus du point de départ » ;
  - page d'histoire de la base aérienne 112 de Reims (franco.wiki) : tenir 500 m pendant quinze minutes.
  - Les deux résumés divergent. Pistes seulement.
- **S84** — historyofwar.org, « 9 August 1917 », https://www.historyofwar.org/firstworldwar/daybyday/1917_08_09.html : troisième et dernier jour des essais officiels du Morane-Saulnier AI à Villacoublay. Vulgarisation.
- **S85** — Martlesham Heath Aviation Society, « History », https://www.mhas.org.uk/history/ : Testing Squadron installé à Martlesham Heath en janvier 1917 ; station ouverte le 16 janvier 1917. Association locale.
- **S86** — Bureau of Standards et NACA, documents sur les instruments d'altitude vus en résumé : « Aeronautic instruments. Section II: altitude instruments », https://digital.library.unt.edu/ark:/67531/metadc65776/, et lettre circulaire 190 du NBS, https://nvlpubs.nist.gov/nistpubs/Legacy/LC/nbslettercircular190.pdf. L'un d'eux indique un étalonnage des altimètres américains à +10 °C isotherme depuis 1917 ; **lequel, je ne sais pas**. Primaires, non lues.
- **S87** — Rapport du NACA sur l'atmosphère type, vu en résumé (sans doute « Standard atmosphere – tables and data », NTRS 19930091285 ; attribution à vérifier) : loi diffusée par la S.T.Aé. en mars 1920 comme projet d'accord interallié, appliquée à partir du 1er mai 1920, puis adoptée par l'Angleterre, la Belgique et l'Espagne, et par le NACA en 1921.
- **S88** — PATSTEC (patrimoine scientifique et technique contemporain), fiches d'altimètres enregistreurs Jules Richard, https://www.patstec.fr/pdfgen.php?id=25010 et https://www.patstec.fr/pdfgen.php?id=16263 : instrument gradué jusqu'à 5 000 m, numéro de série de 1917 environ. Fiche muséale.
- **S31 (complément)** — même fil CombatACE que S31, résumé du 2026-10-24 : avion en l'air après environ 150 ft, meilleure montée vers 60 à 65 mph.
- **S12 (complément)** — flyingmachines.ru, résumé du 2026-10-24 : seconde valeur de 6 min 55 s à 2 000 m pour le 150 ch.

### 9.3 Pistes bibliographiques [PISTE] (non consultées ; éditions à vérifier)
- **P1** — J. M. Bruce, *Spad 7.C1*, Windsock Datafile n° 8, Albatros Productions, 1988 (fiche de catalogue de libraire).
- **P2** — T. Gronczewski et S. M. Fleischer, *Spad VII C.1*, Aviatik Vintage Aircraft n° 1, 2004 (14 pages de plans selon une revue).
- **P3** — M. Velek, M. Ovčáčík et K. Susa, *SPAD S.VII C1*, 4+ Publications n° 19 (4+019), 2004 (plans au 1/48).
- **P4** — J. Guttman, *SPAD VII Aces of World War I*, Osprey, Aircraft of the Aces 39, 2001.
- **P5** — G. Alegi et P. Varriale, *Italian SPADs at War*, Windsock Datafile n° 155.
- **P6** — J. J. Davilla et A. M. Soltan, *French Aircraft of the First World War*, Flying Machines Press, 1997 (source des masses selon S5).
- **P7** — C. J. Biddle, *The Way of the Eagle*, Charles Scribner's Sons, New York, 1919.
- **P7 bis** — E. V. Rickenbacker, *Fighting the Flying Circus*, 1919 (éditeur à vérifier) ; seulement lu en résumé dans *Flight* (S53).
- **P8** — R. Fonck, *Mes combats*, Flammarion, 1920 (préface du maréchal Foch).
- **P9** — A. Deullin, « La chasse en monoplace », 1917 (à localiser au SHD).
- **P10** — Rapport d'essai britannique n° 96, avril 1917 (S.VII 150 ch), cité par S11. Probablement aux National Archives (série AIR 1).
- **P11** — Résumé d'essais du STAé pour le SPAD 7 C1 150 ch, cité par S11 (SHD, département Air).
- **P12** — *Air Service Information Circular*, vol. III, n° 286, 1er octobre 1921 (essai du SPAD XIII à McCook Field).
- **P13** — Lettres de F. Baracca (Museo del Risorgimento, Milan ; Biblioteca Fabrizio Trisi, Lugo) ; biographie de Guerrini et Pluviano citée par S25.
- **P14** — Hardesty, Hastings et Toelle, « Project Butterfly », *Cross & Cockade* (USA), vol. 9 (1968) et vol. 13 (1972).
- **P15** — J. N. Hall et C. B. Nordhoff (dir.), *The Lafayette Flying Corps*, 2 vol., Houghton Mifflin, 1920.
- **P16** — *WW1 Aero*, n° 47 (« twin repro Spad 7s ») ; index : https://ww1aeroinc.org/ww1-aero-index/.
- **P17** — Appareils originaux de S.VII à mesurer : Musée de l'Air (S.254), San Diego Air & Space Museum (B9916), Canada Aviation and Space Museum (B9913), Museo Baracca de Lugo (S.2489), musée de Vigna di Valle (S.153).
- **P18** — Original français du livre de Henry Bordeaux sur Guynemer (Plon, 1918 ; titre exact à vérifier).

Ajoutées en septième passe (géométrie de détail) :
- **P19** — *Cross & Cockade* (US), vol. 2, n° 1, 1961 : « Spad 7: 3 View Drawings » (d'après la bibliographie d'Aeroscale, S37 [RÉSUMÉ]). Numéros anciens vendus en téléchargement par la League of WWI Aviation Historians (overthefront.com).
- **P20** — *Cross & Cockade* (US), vol. 15, n° 4, 1974 : dessins d'ensemble d'origine de Blériot Aéronautique (même source ; **type d'avion non confirmé**, peut-être le SPAD XIII).
- **P21** — B. S. Kelsey, « The Real Spad », *Over the Front*, vol. 8, n° 2, p. 114-122 (même source ; une seconde recherche ne l'a pas retrouvé : référence à vérifier).
- **P22** — « "Smith IV" Restored », *Over the Front*, vol. 2, n° 2, 1987 (SPAD XIII du NASM ; une restauration peut avoir donné lieu à des relevés).
- **P23** — « SPAD S VII, S.1777: Whose Aircraft? », *Over the Front*, vol. 15, n° 3, 2000.
- **P24** — J. M. Bruce, *Spad 13.C1*, Windsock Datafile n° 32 (S64). Avec P1 (Datafile n° 8, *Spad 7.C1*, plans d'Ian Stair au 1/48 et au 1/72 selon un résumé), source secondaire la plus probable des cotes d'empennage et de cellule.
- **P25** — « Performance test of Spad 13 equipped with 220-horsepower Wright engine », *Aerial Age*, vol. 14, n° 15, 19 décembre 1921, p. 344-345 (S56 [LU-OCR], titre). Sans doute la publication de l'essai de McCook (P12).
- **P26** — J. Fromentin, « Les avions Spad », *La Vie aérienne*, 1920, p. 1054-1055, 1080-1081 et 1097-1098 (pagination lue dans l'OCR de S56, à vérifier).
- **P27** — Descriptions du biplace SPAD avec schémas : *Automotive Industries*, vol. 38, n° 25, 20 juin 1918, p. 1212-1214 ; *Aviation*, vol. 4, n° 10, 15 juin 1918, p. 674-675 (S56). Comparables de famille seulement.

### 9.4 Historique des escadrilles
- Albin Denis, historiques des escadrilles (`http://albindenis.free.fr/Site_escadrille/escadrille003.htm`) : **inaccessible**, à consulter en priorité pour la SPA 3 (terrains, appareils, dates).

### 9.5 Sites inaccessibles depuis l'outil de lecture de l'historien (testés le 2026-10-20 ; à consulter plus tard)
albindenis.free.fr ; en.wikipedia.org ; fr.wikipedia.org ; it.wikisource.org ; catalog.hathitrust.org ; gallica.bnf.fr (403 selon le coordinateur) ; www.gutenberg.org (direct) ; projekt-gutenberg.org ; www.theaerodrome.com ; flyingmachines.ru ; www.avionslegendaires.net ; historynet.com ; www.flightglobal.com (archives de *Flight*) ; www.thisdayinaviation.com ; www.hydroretro.net ; digital.library.unt.edu ; bibliotheques-numeriques.defense.gouv.fr ; m-selig.ae.illinois.edu ; aergistal.github.io ; web.archive.org ; www.google.com. Seuls github.com, raw.githubusercontent.com et, par intermittence, api.github.com répondent.

**Cas particulier** : archive.org et ntrs.nasa.gov répondent au `curl` du conteneur (coordinateur, 2026-10-20), mais **pas** à l'outil de lecture de l'historien (DNS en échec). Ils ne sont exploitables que par téléchargement préalable (§ 9.6).

**Dixième passe (2026-10-24)** : échec DNS de l'outil de lecture pour www.hydroretro.net, archive.org, ntrs.nasa.gov, www.academie-technologies.fr et resair420.free.fr.

**Neuvième passe (2026-10-23)** : échec DNS de l'outil de lecture pour woodenpropeller.com, www.woodenpropeller.com, archive.org, gallica.bnf.fr, francearchives.gouv.fr, www.theaerodrome.com, catalog.hathitrust.org et reports.aerade.cranfield.ac.uk.

**Septième passe (2026-10-22)** : échec DNS de l'outil de lecture pour www.thisdayinaviation.com, bibliotheques-numeriques.defense.gouv.fr et sites.ualberta.ca (fichier `~khorne/archives/Issue4/hs-spad.pdf`, contenu inconnu, sans doute un article de modélisme).

### 9.6 Documents à télécharger en priorité (identifiants vérifiés par le moteur de recherche, contenu non lu)
Pour chaque document d'archive.org, récupérer le texte OCR à l'adresse `https://archive.org/download/<identifiant>/<identifiant>_djvu.txt` et, si possible, le PDF. Pour NTRS, l'adresse est `https://ntrs.nasa.gov/api/citations/<id>/downloads/<id>.pdf`.

| Priorité | Document | Identifiant | Ce qu'on y cherche |
|---|---|---|---|
| — | *Flight* 1916-1919 (208 numéros) | `Flight_International_Magazine_AAAA-MM-JJ-pdf` | **Fait (quatrième passe)** : texte OCR lu (S46 à S59) |
| — | *Bibliography of Aeronautics* 1917-1921 (P. Brockett) | `bibliographyofa191721broc` | **Fait (quatrième passe)** : S56 |
| — | *Technical Report of the ACA* 1917-18, vol. II | `technicalreport1917aero_2` | **Fait (quatrième passe)** : volume II (hélices, vraie grandeur) ; rien sur le SPAD lui-même, mais R&M 462, 324 et 492 (S60 ; `hispano-suiza-8a.md`, `profil-spad7.md`) |
| — | Images des pages de *Flight* du 13 février 1919 (p. 199-207) et du 20 février 1919 (p. 237-238) | `Flight_International_Magazine_1919-02-13-pdf` ; `…-02-20-pdf` | **Fait (cinquième passe)** |
| — | Image à 300 dpi du **haut de la p. 205** de *Flight* du 13 février 1919 | même item | **Fait (sixième passe)** : 119,5 et 111,5 mph confirmés par contrôle interne (§ 0). Facultatif : un **recadrage** de cette image (environ x = 900 à 3 000 et y = 0 à 200 en pixels de l'original), qui serait affiché sans réduction, pour une lecture directe à pleine résolution |
| 1 | *Automotive Industries*, numéro de janvier ou février 1919 contenant les tableaux repris par *Flight* (S46, S47) | à chercher (collection *Automotive Industries* sur archive.org ou HathiTrust) | Original américain des tableaux, sans doute mieux imprimé ; peut-être des notes sur les conditions d'essai |
| 2 | *Aerial Age Weekly*, vol. 5, n° 17, 2 juillet 1917, p. 569 : texte OCR et dessin | **absent d'archive.org** selon le coordinateur | Original de S48 ; moins urgent depuis la lecture de S46 |
| 2 | *Technical Report of the ACA* 1917-18, **vol. I** (maquettes) | **absent d'archive.org** selon le coordinateur ; piste : Bibliothèques numériques de la Défense (`bibliotheques-numeriques.defense.gouv.fr`), qui signalent les volumes II et III de la série, ou les archives ARC de Cranfield | RAF 14 aux incidences négatives ; effet biplan sur le RAF 15 ; découpe dans le plan supérieur |
| — | NACA TR 416, Jacobs et Abbott, *The N.A.C.A. Variable-Density Wind Tunnel* (1932) | NTRS 19930091490 (confirmé) | **Fait (sixième passe)** : τ ≈ 0,175 et δ ≈ 0,05 à l'allongement 6 (`profil-spad7.md` § 4.2, A26). Facultatif : figure 11 (PDF p. 10) rendue à 300 dpi et recadrée, pour affiner la lecture (commande au § 5 de `profil-spad7.md`) |
| 2 | *Flugsport* 1917-1918 | à chercher (archive.org, ou bibliothèques numériques allemandes) | Examen d'un **S.VII capturé** ; original du rapport sur le S.XI (S49) |
| 2 | *Scientific American*, vol. 118, 2 février 1918, p. 107, « Spad pursuit biplane » | à chercher | Description d'époque avec diagramme |
| 2 | *The Aeroplane*, 10 janvier 1917, p. 152 (« The Albatros-Spad ») et 2 mai 1917, p. 1129-1130 (usine Blériot-Spad d'Addlestone) | identifiants non relevés | Réputation ; production britannique |
| 2 | Autres volumes de la même série (1916-17, 1918-19, 1919-20, 1920-21, 1921-22) | identifiants non relevés (modèle supposé : `technicalreport19xxaero…`) | ARC R&M 586 (juillet 1921) : courbe de puissance de l'Hispano |
| 3 | *All the World's Aircraft* 1924 (le plus ancien volume trouvé sur archive.org) | `allworldsaircraft1924` | Probablement sans fiche du S.VII ; les volumes 1917-1919 ne sont pas trouvés sur archive.org |
| 3 | NACA Reports n° 233, 309, 279, 304, 331, 93 et 124 | voir `profil-spad7.md` § 3 et 6.1 | **Fait (troisième passe)** : téléchargés par le coordinateur et lus |
| 1 | ARC R&M 586 (juillet 1921), figure 10 | ni archive.org ni NTRS ; archives ARC de Cranfield (AERADE) | Courbe puissance/régime du Hispano de **150 ch** (aucune mesure lue pour le 8Aa) |
| 2 | NACA Reports 220 et 292 | NTRS, identifiants non relevés | Essais en vol des hélices du VE-7 (`hispano-suiza-8a.md` § 9) |
| 2 | ARC R&M 774, 816, 857, 901 et 997 ; NACA TR 256 | archives ARC (Cranfield) ; NTRS | Cellules biplans RAF 15 et méthode de Glauert (§ 2.4) |
| 1 (7e passe) | Image à 300 dpi des **p. 840-841** de *Flight* du 16 août 1917 : dessin du S.VII repris d'*Aerial Age* (S48) ; l'OCR n'en restitue aucune légende | `Flight_International_Magazine_1917-08-16-pdf` | Forme en plan, découpe, décalage, proportions et position de l'empennage, mesurables en proportion de l'envergure connue (C : dessin de presse). Recadrage pleine résolution si le dessin est petit |
| 1 (7e passe) | Image à 300 dpi des **p. 527-529** de *Flight* du 16 mai 1918 : dessin coté du biplace S.XI (S49) | `Flight_International_Magazine_1918-05-16-pdf` | Légendes illisibles dans l'OCR : position du plan fixe par rapport à l'axe de traction, surfaces par plan, dimension des roues (« 700 × … ») ; comparable de famille |
| 1 (7e passe) | Air Service Information Circular vol. III n° 286 (1er octobre 1921), ou sa reprise dans *Aerial Age*, vol. 14, n° 15, 19 décembre 1921, p. 344-345 (P12, P25) | archive.org ou HathiTrust (identifiants à chercher) | Cotes complètes du XIII (profondeur, dérive, décalage dont la convention « 1¼° »), centrage éventuel |
| 2 (7e passe) | *Scientific American*, vol. 118, 2 février 1918, p. 107, « Spad pursuit biplane » | archive.org (identifiant à chercher) | Schéma d'époque du SPAD |
| 2 (7e passe) | P19 à P23 (*Cross & Cockade* US, *Over the Front*) | téléchargements payants (overthefront.com) | Plans à trois vues, dessins d'usine, relevés de restauration |
| 2 (7e passe) | P1 et P24 (Windsock Datafiles 8 et 32) | ouvrages papier ; P24 est au fonds des Bibliothèques de la Défense (S64) | Cotes de cellule et d'empennage (secondaire) |
| 1 (8e passe) | SHD, département Air : **inventaire PDF de 2012 du fonds Émile Dorand (DE 2012 ZL 245)**, puis les articles qui concernent les essais du SPAD 7 en 1916-1917 (S69) | lien de l'inventaire sur https://fr.geneawiki.com/wiki/SHD_/_Air_-_S%C3%A9rie_(DE)ZL ; notice SHD https://servicehistorique-admin.cnmosis.dirisi.defense.gouv.fr/en/node/707 | Procès-verbaux d'essais du STAé : début du chronométrage, atmosphère, masse pesée, hélice, date de chaque essai ; dire si la ligne 8Aa de S46 est composite (§ 5.5.5) |
| 1 (8e passe) | The National Archives, **AIR 1/1196/204/5/2600** (Martlesham Heath, rapports n° 1 à 119, 1916-1919), et recherche « Spad » dans AIR 1 sur discovery.nationalarchives.gov.uk (S70) | cote vue en résumé seulement | Rapport d'essai n° 96 d'avril 1917 (P10) ou ses chiffres : montée, vitesses, plafond estimé, méthode de réduction |
| 2 (8e passe) | Recadrage à 300 dpi de la **ligne d'en-tête de la p. 204** de *Flight* du 13 février 1919 | PDF local `flight_1919-02-13.pdf` | Signes de renvoi devant « Speed on the Level » et « Theoretical Ceiling » et leur note éventuelle (lecture incertaine à 150 dpi) |
| 2 (7e passe) | FranceArchives et SHD : notice de réglage du S.VII, fiches d'essais et de pesée du STAé | **cote non trouvée** ; recherche à faire sur francearchives.gouv.fr (mots-clés « SPAD », « STAé », « notice de réglage », « biplan monoplace ») ; seule notice SPAD repérée : celle du XIII (S38, Bibliothèques numériques de la Défense, `fc97b662-8b01-43a8-9698-6e0be5953006`) | Ce qu'on y cherche : calages, décalage, débattements, centrage |
| 1 (9e passe) | ARC R&M 586 (1921), « Report on Various Airscrews Designed for S.E.5, with 150 H.P. Ungeared Hispano-Suiza Engine » | archives AERADE de Cranfield (collection des R&M de l'ARC, `https://reports.aerade.cranfield.ac.uk/handle/1826.2/11`) ; détail dans `hispano-suiza-8a.md` § 7.5.9 | hélices et régimes en montée et en palier sur un Hispano de 150 hp sans réducteur : séparer l'hélice du moteur (§ 5.6) |
| 1 (9e passe) | G. Hartmann, « Les héliciers français » (PDF) | `https://www.hydroretro.net/etudegh/heliciersfrancais.pdf` | liste complète des hélices autorisées pour le « Type VII », avec colonnes, date et origine |
| 1 (9e passe) | G. Eiffel, *Études sur l'hélice aérienne faites au laboratoire d'Auteuil* (1921, 2 vol.) | BnF (CCFr `ark:/16871/00111293469`), Gallica à chercher ; Smithsonian Libraries TL705 .E5X 1921 | dessin et essais d'une hélice de SPAD à Hispano de 150 hp (piste H28 de la fiche moteur) |
| 2 (9e passe) | Plans d'usine du S.VII (nervures, bord d'attaque), et P1 | calques attribués à Roger Freeman, copies chez Leo Opdycke (S74 [RÉSUMÉ]) ; Windsock Datafile n° 8 | bord d'attaque coffré ou entoilé (§ 2.6), nervures, fausses nervures |
| 2 (9e passe) | Fil S11, page entière | `https://theaerodrome.com/forum/threads/climbing-performance-of-the-spad-vii.30985/` | texte exact de l'« Air Board summary » et du rapport M.96, avec leurs références (§ 5.6) |
| 1 et 2 (10e passe) | Dix documents sur les conditions des essais de montée : NACA TM 15 (Grimault), rapport du NACA sur les instruments d'altitude, images de Tizard dans *Flight* (mars 1917), NACA TN 20, dossiers Hartmann (Reims 1911, Nieuport), histoire des essais de Villacoublay, Toussaint 1919 | adresses exactes au § 5.7.7 | départ du chronomètre, zéro et graduation des altimètres français, atmosphère de réduction avant 1920 |

## 10. Journal des décisions
- **2026-10-20 — Sous-version pour la SPA 3, été 1917.** S5, S8 et S16 font du 8Ab de 180 ch le moteur des S.VII neufs à partir du printemps 1917 (« avril 1917 » pour S16). Guynemer juge le 150 ch insuffisant dès décembre 1916 (S1). Aucun état des appareils de la SPA 3 n'a été trouvé. **Retenu : S.VII 180 ch (8Ab), confiance C**, en sachant que la SPA 3 alignait sans doute encore des 150 ch et quelques SPAD XIII à partir de fin juillet 1917 (S18, S1). Ce choix sera à revoir dès que l'historique d'Albin Denis ou les journaux de marche seront accessibles.
- **2026-10-20 — Version à utiliser pour valider le modèle de vol.** Les seules mesures d'époque trouvées (STAé et rapport britannique n° 96, via S11) concernent le **150 ch**. **Recommandé** : valider d'abord le modèle sur la configuration 150 ch (masse de 705 kg, S5/S9), puis passer au 180 ch en changeant le moteur et la masse. La cellule est la même ; les cibles du 180 ch sont plus faibles (C ou D). Décision finale au joueur et à l'aérodynamicien.
- **2026-10-20 — Envergure supérieure.** 7,80 à 7,82 m selon les sources : **retenu 7,82 m** (S7, institutionnelle, concordante avec S8 et avec les 25 ft 8 in de S14), confiance B.
- **2026-10-20 — Envergure inférieure.** S9 donne 7,61 m ; S16 décrit des envergures égales. **Non tranché.** Retenu provisoirement 7,61 m (C), à vérifier sur un plan (P1 à P3) ou sur un appareil (P17).
- **2026-10-20 — Surface alaire.** 17,85 m² (S6, S8, S10, S33) contre 18 m² (S7, sans doute arrondi). **Retenu 17,85 m²** (B pour le chiffre). Ce qu'elle inclut est inconnu ; la corde moyenne apparente (environ 1,16 m, C) paraît faible et devra être recoupée.
- **2026-10-20 — Masses.** 500 kg à vide et 705 kg au décollage pour le 150 ch (S9, et S5 qui cite Davilla et Soltan) : **retenu, B**. 510 et 740 kg (S6, S33) : motorisation non précisée, **notées sans attribution** ; leur emploi pour le 180 ch est une hypothèse (C).
- **2026-10-20 — Vitesse maximale.** 192 km/h (150 ch) et 208 km/h (180 ch) (S5), sans altitude ; S5 rattache le 192 km/h au prototype SPAD V. Les 212 km/h à 2 000 m de S9 sont isolés : écartés (D). **Aucune valeur exploitable en confiance B.**
- **2026-10-20 — Montée du 180 ch.** S12 : 3 000 m en 9 min 50 s ; S11 : 2 000 m en 4 min 40 s. Ensemble, ces deux chiffres impliqueraient une tranche de 2 000 à 3 000 m plus lente que celle du 150 ch : incohérent. **Retenu 9 min 50 s à 3 000 m (C)** ; les 4 min 40 s sont écartés (D). *(Dépassé : cinquième passe, 4 min 40 s et 8 min 10 s lus dans S46 ; les 9 min 50 s sont ceux de la ligne italienne S47.)*
- **2026-10-20 — Plafond du 180 ch.** 6 200 m (S7, musée) contre 6 553 m (sites de jeux, S39). **Retenu 6 200 m (C)**, la source institutionnelle étant préférée. *(Dépassé : cinquième passe, plafond théorique de 6 523 m lu dans S46 ; huitième passe, c'est une extrapolation, § 5.5.3 d.)*
- **2026-10-20 — Données de jeux.** Les chiffres d'IL-2, de *Rise of Flight* et de *Wings of Glory* (S39) sont exclus comme sources ; la confusion qu'ils entretiennent est signalée.
- **2026-10-20 — Sens de rotation de l'hélice.** Horaire vu du pilote (C). Détail et justification dans `hispano-suiza-8a.md`.
- **2026-10-20 — Deuxième passe : aucune confiance relevée.** archive.org et NTRS ne sont pas lisibles par l'outil de l'historien (§ 0, § 9.5) ; ni Jane's 1917-1919, ni *Flight*, ni *The Aeroplane*, ni *L'Aérophile*, ni le rapport n° 96, ni les essais du STAé n'ont pu être consultés. Toutes les valeurs restent au niveau de la première passe. Les identifiants des documents à télécharger sont au § 9.6.
- **2026-10-20 — Troisième passe : profil.** La « section Spad » du NACA Report 93 remplace « inconnu » au § 2.1. **Confiance B** : document d'époque, mais type de SPAD non précisé. Polaires et convention dans `profil-spad7.md`.
- **2026-10-20 — Troisième passe : interférence biplan.** Pour le modèle, retenu en **C** :
  - un facteur de cellule de **0,82 à 0,86** sur la portance aux faibles incidences (S43) ;
  - un facteur de **0,95 sur le C_L max** (S43) ;
  - la pente de cellule du RAF 15 à 3,57 par radian pour un écartement/corde de 1, sans décalage (S45). Ces trois valeurs concordent entre elles.

  **Méthode préférée** : la méthode de Glauert (R&M 901, non lu), ou un modèle à deux plans séparés. Le facteur global n'est qu'une **valeur de contrôle**. Le choix revient à l'aérodynamicien. Aucune cote de cellule du S.VII n'étant connue, rien n'est chiffré pour le SPAD lui-même.
- **2026-10-20 — Troisième passe : décrochage.** Le décrochage du plan supérieur en premier (S44) et la chute nette du RAF 15 après son maximum (Report 233) rendent **plausible** l'abattée de la réputation. Le trait reste « à confirmer » (§ 6.2), faute de témoignage direct lu sur le S.VII.
- **2026-10-20 — Troisième passe : amortissement en tangage.** Règle de S43 retenue comme **contrôle** : amortissement total ≈ amortissement calculé de l'empennage / 0,55. Elle vient du JN-2 de 1917 : confiance C pour le SPAD.
- **2026-10-20 — Troisième passe : performances.** Aucune source nouvelle sur les performances du S.VII : les **valeurs du § 5 restent en C**. Le moteur et l'hélice ont progressé (`hispano-suiza-8a.md`) ; il faudra refaire les cibles de puissance quand l'aérodynamicien aura tranché la cote du 8Ab (1 700 ou 2 100 tr/min).
- **2026-10-20 — Traits retenus.** Seuls deux traits sont **confirmés** par au moins deux témoignages directs indépendants : la robustesse et la tenue en piqué (Guynemer, Hall), et la sensibilité thermique du moteur (Hall pour le S.VII, Meister pour le XIII). Le second sort du M1. Les autres traits restent « à confirmer » ou « contestés » et ne doivent servir que d'essais non bloquants.

**Quatrième passe (2026-10-20 et 2026-10-21 ; *Flight*, ACA 1917-18 vol. II, TN 182)**
- **2026-10-21 — Méthode de lecture des tableaux de 1919 (S46, S47).** L'OCR mélange les colonnes. Une valeur n'est retenue que si un contrôle interne la valide : somme de la charge utile (essence + pilote + armement + huile), masse à vide + charge = total, cohérence avec la surface du libellé. Les vitesses sont retenues parce que les **mêmes couples de chiffres** (132 et 126 mph) se retrouvent dans un document indépendant de 1917 (S48), et 119,3 mph dans S5. Les altitudes des colonnes et les temps de montée ne sont **pas** retenus : à lire sur l'image (§ 9.6).
- **2026-10-21 — Vitesse du 180 ch (remplace l'entrée « Vitesse maximale » du 2026-10-20 ; précisée par la cinquième passe).** 212 km/h et 203 km/h : S46 (ligne 8Ab) et S48 (vitesses « au niveau de la mer » et « à 3 000 m »). **Retenu : 203 km/h à 3 000 m, B** ; 212 km/h en palier bas, **B pour la valeur**, altitude non tranchée entre le sol (S48) et 2 000 m (S9). Les 212 km/h de S9, que j'avais écartés comme incohérents (D) parce que S9 les attribue au 150 ch, sont en fait la vitesse du **180 ch** : l'erreur était l'attribution de S9, pas le chiffre. Les 208 km/h de S5 restent une variante (C).
- **2026-10-21 — Vitesse du 150 ch.** 192 km/h : S5 et S46 (ligne 8Aa). **Relevé de C à B pour la valeur** ; altitude non tranchée.
- **2026-10-21 — Masses.** Masse à vide 499 kg (1 100 lb) sur trois lignes de deux tableaux d'époque : **500 kg confirmé, B**. Masse au décollage : S46 donne 694 kg (8Aa) et 703 kg (8Ab) ; S5 et S9 donnent 705 kg pour le 150 ch. **Pour le 150 ch, 705 kg est conservé** en attendant l'image : l'écart n'est que de 11 kg, et l'attribution des lignes de masses aux lignes de moteurs repose sur l'ordre de l'OCR. **Pour le 180 ch, retenu 703 kg (B provisoire)**, qui forme un ensemble cohérent avec les vitesses de la même ligne ; 740 kg (S6, S33) devient une variante (C).
- **2026-10-21 — Envergure inférieure (remplace l'entrée du 2026-10-20).** S48 (1917) donne deux plans de 25 ft 6 in ; S16 décrit des envergures égales ; seul S9 donne 7,61 m. **Retenu provisoirement : envergures égales, C.** Effet sur le modèle : faible (σ change d'environ 0,01, § 2.4) ; à vérifier sur plan.
- **2026-10-21 — Corde.** S48 donne 4 ft 7 in (1,40 m), incompatible avec toutes les surfaces publiées (17,8 à 18,1 m²) : il faudrait environ 21,7 m². **Non retenu (D)** ; la corde reste à mesurer. La surface de 17,85 m² est maintenant confirmée par deux tableaux d'époque : c'est elle qu'il faut tenir pour sûre, et la corde qu'il faut chercher.
- **2026-10-21 — Écartement, décalage, calages.** Écartement : 1,27 m (S48, « approximatif »), **C**, seule valeur trouvée. Décalage horizontal et décalage angulaire du S.VII : **non trouvés** dans les documents lus. Calages du S.XI (S49) notés comme comparables de famille seulement.
- **2026-10-21 — Facteur σ de Prandtl (question de l'aérodynamicien).** Tableau I de la TN 182 relu sur l'image et contrôlé (§ 2.4). Pour le S.VII, σ ≈ 0,54 et κ ≈ 0,77 avec l'écartement de S48 : **C**, à cause de l'écartement approximatif. La formule de Prandtl (traînée induite totale indépendante du décalage horizontal) complète la méthode de Glauert retenue en troisième passe pour la portance.
- **2026-10-21 — Hauteur.** 2,19 m (7,2 ft) dans deux tableaux d'époque, concordant avec 2,20 m (S6) : **relevée de C à B**.
- **2026-10-21 — Traits et réputation.** Aucun nouveau trait de vol n'atteint le seuil de deux témoignages directs indépendants. Ajouts : réputation de solidité (Rickenbacker), d'avion convoité (*Flugsport*, Rickenbacker), surnom « Spad allemand » donné à l'Albatros ; chute d'Olivari confirmée par la presse de 1917 ; nuance du RAE sur la pente réelle des piqués « verticaux » (S60), d'où une variante d'essai à −60°.
- **2026-10-21 — Version de validation du M1 (complète l'entrée du 2026-10-20).** Le 180 ch dispose désormais de cibles B de vitesse avec une masse cohérente (703 kg) ; le 150 ch garde la meilleure série de montée (C) et une vitesse B sans altitude. **Suggestion** : valider la traînée et l'hélice sur la vitesse du 180 ch à 3 000 m, et la puissance en montée sur la série STAé du 150 ch. Décision à l'aérodynamicien et au joueur. *(Dépassé par la cinquième passe, ci-dessous.)*

**Cinquième passe (2026-10-21 ; images des pages de *Flight*, TN 70)**
- **Règle de lecture de S46.** Le régime est imprimé sous chaque ligne : contrôle par le nombre de régimes égal au nombre de vitesses sur toutes les lignes voisines (§ 0). Elle remplace mon hypothèse de la quatrième passe, qui plaçait le régime au-dessus.
- **Corrections d'erreurs de la quatrième passe**, toutes dues à l'OCR ou à une mauvaise lecture des colonnes :
  - 694 kg (1 530 lb) pour le 150 ch : faux, l'image porte 1 550 lb (703 kg) ;
  - « 193 sq ft » sur la ligne 8Ab : faux, les deux lignes portent 195 sq ft ;
  - « essence 176 lb, huile 11 lb » : faux, la colonne de 176 lb est « essence et huile », celle de 11 lb « divers » ;
  - régimes de 1 720 et 1 680 tr/min attribués au 180 ch : ils sont ceux du 150 ch ;
  - « 132 mph au niveau de la mer » (S48) : faux, S46 donne 132 mph à 6 500 ft, soit 2 000 m.
- **Altitude des vitesses : tranchée.** Les colonnes de S46 sont à 6 500, 9 800, 13 100 et 16 400 ft, soit 2 000 à 5 000 m. **Retenu** : 212 km/h à 2 000 m pour le 180 ch, 192 km/h à 2 000 m pour le 150 ch, **A**.
- **Performances du 150 ch : A.** La série STAé de S11 (6 min 40 s, 11 min 30 s, 19 min 30 s ; 5 500 m) se retrouve à l'identique dans S46. La variante 11 min 20 s est écartée. Ajouts : vitesses à 3 000 et 4 000 m, régimes, 2 h 10 à pleine puissance au sol.
- **Performances du 180 ch : A** pour vitesses (2 000 à 5 000 m), montées (2 000 à 4 000 m), plafond théorique (6 523 m) et régimes. Les 4 min 40 s à 2 000 m d'un participant de S11, écartés en D, sont confirmés. Les 9 min 50 s à 3 000 m de S12 correspondent à la ligne **italienne** de S47 (140 hp) et non au 180 ch français (8 min 10 s) : erreur d'attribution de S12.
- **Masses.** Pour le 150 ch, à vide 500 kg (1 100 lb) et au décollage **703 kg** (1 550 lb, S46 ; 1 551 lb, S47) : **A**. Les 705 kg des ouvrages (S5, S9) restent à 0,3 % ; **retenu** : la valeur imprimée, faute de preuve d'un original français à 705 kg. Pour le 180 ch, S46 répète exactement les masses du 150 ch : **703 à 705 kg, B**, avec 740 kg en variante (C).
- **Surface.** S46 imprime 195 sq ft, mais sa charge alaire (8,08 lb/sq ft) a été calculée avec 192 sq ft, comme celle de S47 : **17,85 m² retenu, A** pour le chiffre ; définition toujours inconnue.
- **Envergure 7,82 m et hauteur 2,20 m : A.**
- **Décalage horizontal (question de l'aérodynamicien).** Toujours non sourcé pour le S.VII. Facteurs de portance en fonction du décalage d'après la TN 70 (MIT, 1918) ajoutés au § 2.4, en C pour le SPAD.

**Sixième passe (2026-10-22 ; *Flight* p. 205 à 300 dpi, NACA TR 416)**
- **Vitesses du 150 ch à 2 000 et 3 000 m : 119,5 et 111,5 mph confirmés**, soit 192 et 179 km/h. L'image à 300 dpi m'est affichée à la même résolution effective qu'en cinquième passe, donc ma lecture seule ne tranchait pas. Le contrôle décisif est interne : les 85 autres vitesses du tableau « in service » sont toutes arrondies au demi-mph, contre des dixièmes dans le tableau « under test ». La réserve de lecture (±0,3 km/h) est remplacée par l'arrondi d'impression (±0,4 km/h). **Confiance A maintenue** ; les cibles du § 5.4 ne changent pas.
- **Facteurs τ et δ de l'aile rectangulaire** (question de l'aérodynamicien) : trouvés dans le TR 416, p. 312-314. À l'allongement 6, **τ ≈ 0,175 et δ ≈ 0,05**, lecture graphique à préciser ; ils confirment les valeurs de l'aérodynamicien. Détail et décision : `profil-spad7.md` § 4.2 et § 7. **Notation** : le TR 416 appelle σ ce facteur de traînée. Ce n'est pas le σ de Prandtl du § 2.4 ; dans nos fiches, il reste noté δ.

**Septième passe (2026-10-22 ; géométrie de détail pour `aircraft.ron`)**
- **Résultat d'ensemble.** Aucune cote chiffrée nouvelle du S.VII. Empennage, ailerons, débattements, décalage, calages, dimensions du fuselage et du radiateur, mâts, câbles, roues, centrage et inertie restent **non trouvés** (§ 2.5.3). Ils seront estimés (`E:`) par l'aérodynamicien. **Je n'ai comblé aucun de ces manques** par une valeur de mémoire ou de jeu.
- **Corde : fourchette, pas de choix.** Les surfaces publiées de la famille valent de 0,89 à 0,94 fois le rectangle brut des deux plans (XIII : S46 et S15 ; S.XI : S49). Transposé au S.VII, cela donne une corde de 1,14 à 1,33 m, vers 1,25 à 1,28 m avec la convention française (§ 2.5.2), **en C**. Les 1,40 m de S48 restent en **D** : il faudrait exclure 3,9 à 4,0 m² de la surface, plus que sur le S.XI. Je ne retiens **pas** de valeur unique : la convention de surface du S.VII est inconnue, et deux documents officiels diffèrent déjà de 5,6 % sur le XIII. Choix de la valeur de travail : à l'aérodynamicien, avec renvoi au § 2.5.2.
- **Comparables du XIII ajoutés au § 2.2** : plan fixe 3,099 × 0,527 m, ailerons 2,222 × 0,495 m, dérive longue de 1,200 m, surface de 227 sq ft (S15, **résumé**, donc C au mieux pour le S.VII) ; envergure 8,02 m, longueur 6,20 m, hauteur 2,30 m et surface de 215 sq ft (S46, **lu sur l'image**). L'envergure et la longueur du XIII sont désormais confirmées par deux sources indépendantes. L'incohérence de hauteur de la dérive (0,632 ou 0,876 m) reste ouverte. Les 8,2 m du NASM (S65) sont notés, non retenus.
- **Correction d'un libellé** : les 20 ft 4 in du XIII (S15) sont la **longueur hors tout**, comme le montre la colonne « Length » de S46, et non la longueur du fuselage seul.
- **Précision sur S48 (ailerons)** : le texte dit « à environ 9 in derrière le longeron principal », et l'auteur le déduit du dessin. « Longeron arrière » est une lecture du contexte. La valeur ne donne pas la corde des ailerons.
- **S.XI comme comparable de famille** (§ 2.5.1) : retenu pour les habitudes de construction (empennage « du type SPAD habituel », plan fixe calé à 0°, câbles porteurs doublés et carénés de bois, train) et pour la répartition des masses par élément, utile à une estimation d'inertie par composants. **Jamais** pour une cote du S.VII : autre cellule, cordes inégales, flèche de 3°, moteur à réducteur.
- **Haubanage** : 4 mâts interplans et 4 mâts intermédiaires (décompte C déduit de S48 et S49). Le type des câbles (profilés ou ronds) du S.VII reste inconnu ; le S.XI montre des câbles doublés carénés de bois, ce qui n'est pas la même chose que des câbles profilés.
- **Documents demandés** (§ 9.6, lignes « 7e passe ») : en priorité, les images à 300 dpi du dessin du S.VII (*Flight*, 16 août 1917, p. 840-841) et du dessin coté du S.XI (*Flight*, 16 mai 1918, p. 527-529), déjà disponibles dans les PDF d'archive.org ; puis la circulaire de McCook ou sa reprise dans *Aerial Age* (décembre 1921).

**Huitième passe (2026-10-22 ; fiabilité des performances du 150 ch)**
- **2026-10-22 — Cibles du 150 ch (remplacent, pour cette version, celles du § 5.4).** Détail et calculs au § 5.5.
  - **Vitesse à 2 000 m.** Fourchette : 185 à 194 km/h. 192 km/h (S46) et 194 km/h (S47) sont imprimés ; environ 185 km/h se déduit des valeurs de S46 à 3 000 et 4 000 m avec la loi R&M 462 (C). **Retenu : 192 km/h, A, ±3 %.** Raisons : deux tableaux concordent à 1 % près, et la vitesse au sol de S47 (199 km/h) se raccorde à 192 km/h par le calcul (C).
  - **Vitesses à 3 000 et 4 000 m.** 179 et 174 km/h sont **maintenus, mais déclassés en B, avec ±5 %**. Elles viennent d'une seule source, et elles sont incohérentes de 3 à 4 % avec la valeur à 2 000 m. Entre 2 000 et 3 000 m, la ligne du 8Aa perd 6,7 % de vitesse, contre 0,7 à 2,3 % pour tous les autres monoplaces du tableau. J'ignore laquelle des deux familles de valeurs est fausse ; la tolérance couvre l'écart. Le modèle ne doit pas être réglé pour atteindre 179 km/h au détriment de 192 km/h.
  - **Montées.** 6 min 40 s, 11 min 30 s et 19 min 30 s : **A, ±5 %, ±5 % et ±7 %**. Fourchette : le rapport britannique n° 96, indépendant mais vu en résumé, est plus rapide de 2 à 6 % ; S47 est plus rapide de 12 à 15 %, mais sa série est irrégulière (C, non bloquante).
  - **Plafond.** 5 486 m est un plafond **théorique**, c'est-à-dire extrapolé. **Retenu en C, ±7 %**, comme altitude où la vitesse ascensionnelle s'annule, et non à 100 ft/min. Fourchette : 5 100 à 5 800 m (extrapolations C ; 5 791 m estimés par le rapport n° 96).
  - **S47** est classé « classe 150 ch, sous-version incertaine » (C) ; ses chiffres servent de contrôles non bloquants. Son « 140 hp » est la cote italienne d'un type de Hispano : le même moteur équipe un F.B.A. dans le même tableau.
- **2026-10-22 — Correction : la série du STAé (S11) n'est pas indépendante de S46.** Ses quatre chiffres sont identiques à ceux de S46, donc d'origine commune. Le § 5 disait le contraire ; corrigé. La seule série indépendante reste le rapport britannique n° 96, connu par un forum.
- **2026-10-22 — Conditions de mesure.** Le début du chronométrage, l'atmosphère de référence, la masse pesée, l'hélice et la méthode de mesure de la vitesse ne sont donnés par **aucun** document lu. La masse de 703 kg est une charge de référence (équipage et armement conventionnels, charge de 205 kg exactement), pas une pesée. Les tolérances ci-dessus en tiennent compte en partie. Consignes de calcul au § 5.5.6 : deux conventions de chronométrage, plafond à vitesse ascensionnelle nulle, sensibilité à la masse. **Aucune condition n'a été comblée de mémoire.**
- **2026-10-22 — 180 ch.** La lecture de S46 (212 km/h à 2 000 m) cadre mieux par le calcul que celle de S48 (212 km/h au sol) : **maintenue** (C). Les colonnes dérivées des deux lignes du S.VII sont recopiées l'une de l'autre, ce qui confirme les masses du 180 ch en B. Mêmes tolérances que pour le 150 ch.
- **2026-10-22 — Relecture et corrections dans la fiche.**
  - Présentation de S46 : p. 201, et non p. 199. *Flight* dit que les tableaux « paraissent » officiels, ce que la fiche résumait trop fermement.
  - § 1.4 : la blessure de Heurtaux était datée de « fin septembre 1917 », date de publication de la nouvelle. La date réelle n'est pas donnée ; S68 ajoute les circonstances.
  - § 1.5 : deux points ajoutés, sur le nom « Vieux Charles », porté par plusieurs avions, et sur la fin des livraisons du S.VII dans la compilation de 1918.
  - § 0, première passe : la phrase « performances plafonnées à C » est marquée dépassée.
  - Journal du 2026-10-20 : entrées « Montée du 180 ch » et « Plafond du 180 ch » marquées dépassées.
  - § 7 : masse d'armement en [LU-IMG], et signalée comme conventionnelle.
- **2026-10-22 — Documents demandés** (§ 5.5.7 et § 9.6, lignes « 8e passe ») : en priorité, l'inventaire du fonds Émile Dorand au SHD (DE 2012 ZL 245) et la pièce AIR 1/1196/204/5/2600 des National Archives. Ces deux cotes n'ont été vues qu'en résumé.

**Neuvième passe (2026-10-23 ; hélice, régimes, bord d'attaque)**
- **2026-10-23 — Hélice.** Les décisions sont prises dans `hispano-suiza-8a.md` (§ 7.5.8 et journal, sixième passe).
  - Géométrie de référence : Levasseur 485, 2,45 × 1,95 m, C.
  - Table de base inchangée : D' en soufflerie, mise à l'échelle.
  - Deux variantes mesurées en vol, laissées au choix de l'aérodynamicien.
  - L'hélice de l'avion de S46 reste **inconnue**.
- **2026-10-23 — Régime hors vitesse maximale : non trouvé** pour un S.VII (§ 5.6).
  - Aucun chiffre n'a été déduit ni comblé.
  - Le comparable du VE-7 (0,88 à 0,92 fois le régime de palier en montée à pleins gaz) sert de contrôle de vraisemblance, pas de cible.
  - Les cibles du § 5.5.6 ne changent pas. La tolérance de ±3 % sur le régime est signalée comme plus serrée que l'incertitude documentaire sur l'hélice ; la décision revient à l'aérodynamicien.
- **2026-10-23 — Bord d'attaque : non tranché** (§ 2.6).
  - Coffrage de l'extrados en contreplaqué jusqu'au longeron avant : C (XIII par des sources modernes, forum).
  - Bord d'attaque entoilé sur fausses nervures : C (seule description d'époque de la famille, sur le S.XI, S49).
  - Aucune valeur n'est retenue. Le terme de traînée calé sur un avion américain de 1922 reste un comparable, à étiqueter comme tel.
- **2026-10-23 — Origine de S46.** Rien de confirmé. La piste « Air Board summary » (§ 5.6) est notée sans être utilisée : résumés contradictoires, document non lu.

**Dixième passe (2026-10-24 ; conditions réelles des essais de montée)**
- **2026-10-24 — Départ du chronomètre des essais français : non tranché.**
  - Aucune source sur la pratique du STAé ; je n'ai rien comblé.
  - Retenu pour le calcul : trois bornes (en vol au niveau du terrain, au décollage, au début du roulage), § 5.7.6.
  - La borne « au décollage » s'appuie seulement sur une déduction (C) : c'est l'origine naturelle d'un barogramme.
- **2026-10-24 — Temps britanniques réduits : convention du modèle (C).**
  - Tizard (S75, lu dans l'OCR) décrit des temps obtenus par intégration de la vitesse ascensionnelle réduite depuis la hauteur type 0. Ils ne dépendent donc pas du départ réel du chronomètre.
  - Le rapport n° 96 est supposé réduit ainsi (C) : même unité, un mois après la conférence.
  - Sous 2 000 m, l'échelle de densité britannique est de 0,7 à 2,3 % plus dense que l'atmosphère type internationale, et se confond avec elle au-dessus de 3 000 m (calcul C).
  - Conséquence : le rapport n° 96 est le meilleur point de comparaison pour le modèle, dès qu'il sera lu. Ses chiffres restent [RÉSUMÉ].
- **2026-10-24 — Atmosphère type française.** Adoption officielle le 15 avril 1920 (S79 [RÉSUMÉ]). **Retenu (C)** : aucune atmosphère type officielle avant 1920. Les chiffres de S46 sont donc bruts, ou réduits par une méthode du STAé inconnue : **non tranché**.
- **2026-10-24 — Terrain.** Villacoublay à 178 m, retenu comme **hypothèse de travail**. Altitude : B (AIP actuelle, plateau inchangé). Attribution de l'essai de S46 : C, puisque le lieu n'est pas documenté.
- **2026-10-24 — Zéro des altitudes : non tranché.** Deux cas à calculer. Selon la loi de S46 (C), ils agissent en sens opposés : +4 à +8 % avec un zéro au terrain, −29 s avec un zéro à 760 mm. **Consigne : ne pas retenir la convention qui arrange le modèle.**
- **2026-10-24 — Date et saison de l'essai : non trouvées.** Fenêtre de mi-1916 à mi-1917 (C). Température : atmosphère type ±15 K en `E:`, d'après la fourchette de Dines (S77).
- **2026-10-24 — Décollage : non trouvé.** Le chiffre de Tallman (environ 46 m) est classé en D et n'est pas une cible.
- **2026-10-24 — Cibles inchangées.** Le § 5.5.6 n'est pas modifié. Proposition à l'aérodynamicien : comparer chaque cible à l'enveloppe des variantes du § 5.7.6.
- **2026-10-24 — Écart avec le rapport n° 96.** Le modèle serait aussi plus rapide que ce rapport, de 4 à 9 % (calcul C sur des chiffres [RÉSUMÉ]). Les conditions d'essai françaises n'expliqueraient alors pas tout l'écart. Conclusion signalée comme **fragile** tant que le rapport n'est pas lu.