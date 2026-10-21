# SPAD S.VII C.1 — fiche de recherche

- Identifiant : `spad7` (même nom que `assets/aircraft/spad7/`)
- Dernière mise à jour : 2026-10-20
- État : **en cours** (quatrième passe). Sont désormais sourcés par des documents d'époque lus :
  - le profil « Spad » (B) ;
  - les polaires du RAF 15 (A pour ce profil) ;
  - la courbe de puissance et la loi d'altitude du Hispano de 180 hp (B pour un 8Ab) ;
  - une hélice en bois comparable (A pour la mesure, C pour l'usage) ;
  - des lois d'interférence biplan (C pour le SPAD) ;
  - **quatrième passe** : des **vitesses en palier, des masses, une surface et des cotes du S.VII** lues dans *Flight* (1917 et 1919), dont deux tableaux officiels américains d'avions français et italiens. Plusieurs valeurs passent de C à **B**. Aucune n'atteint A, car l'OCR de ces tableaux est désordonné : la lecture des colonnes reste à confirmer sur l'image des pages (§ 9.6).
- Fiches liées : [`hispano-suiza-8a.md`](hispano-suiza-8a.md) (moteur et hélice), [`profil-spad7.md`](profil-spad7.md) (profil d'aile)
- Contexte de jeu visé : escadrille SPA 3 (« Cigognes », Groupe de Combat 12), Flandres, été 1917

## 0. Méthode et limites de cette passe (à lire avant d'utiliser un chiffre)

**Accès réseau.** Depuis le conteneur, l'outil de lecture de pages n'atteint que `github.com` et `raw.githubusercontent.com`. Tous les autres sites testés sont inaccessibles (liste au § 9.5). Le moteur de recherche fonctionne, mais il renvoie un **résumé généré** à partir des pages, pas les pages elles-mêmes.

**Statut de chaque source**, indiqué entre crochets :
- **[LU]** : texte intégral lu. Ce sont des ouvrages du domaine public (Project Gutenberg) récupérés par le miroir GITenberg sur GitHub, puis recherchés et lus ligne à ligne.
- **[RÉSUMÉ]** : vu seulement dans un résumé du moteur de recherche. La page n'a pas été ouverte, et la formulation comme les chiffres peuvent avoir été déformés. **Aucun chiffre [RÉSUMÉ] ne doit être tenu pour vérifié.**
- **[PISTE]** : référence bibliographique repérée, non consultée.

**Conséquences.**
- Les performances sont plafonnées à la **confiance C**. Les deux séries de mesures d'époque (STAé et rapport britannique n° 96) ne sont connues que par un forum vu en résumé.
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
- trois fichiers d'*Aerial Age Weekly*. Ceux du 2 juillet et du 19 novembre 1917 sont des **pages HTML d'archive.org**, pas le texte OCR : inexploitables (§ 9.6). Le volume de 1919 ne contient rien d'utile sur le S.VII.

**Limite de lecture.** Les deux tableaux officiels de 1919 (S46, S47) sont imprimés en colonnes serrées. L'OCR les restitue dans le désordre. Les chiffres retenus ici sont ceux dont la lecture est **validée par un contrôle interne** : sommes des masses, produit masse/surface, vitesse de piston déduite du régime. Les colonnes que ce contrôle ne couvre pas (altitudes des vitesses, temps de montée) restent **à lire sur l'image** des pages. D'où le plafond en B.

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
| fin sept. 1917 | Le capitaine Heurteaux, des Cigognes, blessé de deux balles en combat (nouvelle publiée à Paris le 30 septembre) | S55 [LU-OCR] |
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

## 2. Géométrie

Quatrième passe : une description d'époque du S.VII (*Aerial Age*, reprise par *Flight* le 16 août 1917, S48) donne pour la première fois une **corde**, un **écartement**, une voie et des cotes d'empennage. L'article les dit **approximatives**, et sa corde contredit toutes les surfaces publiées (voir la ligne « Corde »). Décalage, calages, dièdre et gouvernes restent **non trouvés**. Les valeurs du SPAD XIII (§ 2.2) ne valent que comme **comparables** (confiance C au mieux) : Béchereau a dessiné le XIII comme une version agrandie du VII (S15, S21 [RÉSUMÉ]).

### 2.1 SPAD S.VII

| Grandeur | Valeur retenue | Unité d'origine | Source | Confiance | Remarque |
|---|---|---|---|---|---|
| Envergure du plan supérieur | 7,82 m | m | S7, S8 [RÉSUMÉ] ; 7,81 m (S6) ; 7,80 m (S9, S10) ; 25 ft 8 in (S14). **Époque** : 25,7 ft = 7,83 m (S46, tableau officiel 1919) ; 25,8 ft = 7,86 m (S47, tableau italien 1919) ; 25 ft 6 in = 7,77 m, « approximatif » (S48, 1917) [LU-OCR] | B | Les trois documents d'époque encadrent 7,82 m à ±0,05 m (conversions C) |
| Envergure du plan inférieur | **non tranchée** ; retenu provisoirement **égale au plan supérieur** | m | Égale : S48 (1917, 25 ft 6 in pour les deux plans) [LU-OCR] et S16 [RÉSUMÉ]. Plus courte : 7,61 m (S9 [RÉSUMÉ]) | C | Deux sources contre une, dont une d'époque mais « approximative ». À vérifier sur plan (P1-P3) ou sur appareil (P17) (journal) |
| Surface alaire | 17,85 m² | m² | S6, S8, S10, S33 [RÉSUMÉ] ; 18 m² (S7). **Époque** : 192 sq ft = 17,84 m² (S47) ; 195 sq ft = 18,12 m² (ligne 8Aa) et 193 sq ft = 17,93 m² (ligne 8Ab) (S46) [LU-OCR] | B pour le chiffre ; **définition inconnue** | Les tableaux de 1919 donnent la surface dans le libellé de la ligne et dans la colonne des surfaces (lecture recoupée). Ce que la surface inclut n'est précisé nulle part |
| Longueur | 6,08 m | m | S6, S7, S8 ; 6,10 m (S5) ; 6,15 m (S10) ; 19 ft 11 in (S14). **Époque** : 20 ft = 6,10 m (S46, S47, S48) [LU-OCR] | B | Les documents d'époque arrondissent au pied ou au dixième de pied ; écart de 2 cm, sans effet |
| Hauteur | 2,20 m | m | S6 ; 2,35 m (S8) ; 2,12 m (S10) ; 7 ft 2 in (S14). **Époque** : 7,2 ft = 2,19 m (S47 ; « 7-20 » dans S46) [LU-OCR] | **B** (relevée de C) | Position de mesure (ligne de vol ou au sol) toujours non précisée |
| Corde | **non tranchée** : 1,40 m selon S48, incompatible avec la surface | ft-in | S48 (1917) : 4 ft 7 in = 1,397 m pour les deux plans, « approximatif » [LU-OCR] | D pour 1,40 m | **Contradiction** (C, calcul) : 2 × 7,77 × 1,397 ≈ 21,7 m² bruts, contre 17,8 à 18,1 m² dans tous les documents. Une découpe au-dessus du pilote et la partie masquée par le fuselage n'expliquent pas 3,6 m². Avec la surface publiée et des envergures égales de 7,82 m, la corde moyenne serait d'environ 1,15 à 1,2 m (C). La corde de S48 coïncide avec celle du XIII (1,400 m) : coïncidence ou confusion, non tranché |
| Écartement des plans | 1,27 m | ft-in | S48 (1917) : 4 ft 2 in, « approximatif » [LU-OCR] | C | Source unique, approximative. Rapport écartement/corde ≈ 0,91 avec 1,40 m, ≈ 1,06 avec 1,2 m (C) |
| Décalage horizontal | non trouvé | — | — | — | |
| Décalage angulaire, calages | non trouvé pour le VII | — | Comparable de la même famille : biplace S.XI, calage du plan supérieur 2,8° au centre et 2,5° au saumon, plan inférieur 1,5° (rapport allemand, S49) [LU-OCR] | C (famille) | À ne pas transposer sans plan : autre avion, autre cellule |
| Dièdre | non trouvé ; **indice** de dièdre nul | — | S47 (tableau italien 1919) qualifie le SPAD monoplace de « flat-winged » [LU-OCR] ; XIII : 0° (S15) ; S.XI : pas de dièdre (S49) | C | « Flat-winged » s'entend sans doute « ailes sans dièdre » : interprétation, à confirmer |
| Structure de voilure | une seule travée, avec des mâts intermédiaires au croisement des câbles de vol et d'atterrissage, reliés par une entretoise en tube d'acier | — | S48 (1917) : les câbles partent du fuselage vers les mâts extérieurs et se croisent aux mâts intermédiaires, où un tube d'acier relie les mâts intermédiaires avant et arrière ; S49 (rapport allemand, 1918) : les monoplaces ont des « mâts intérieurs divisés » ; S50 (1919) : schéma « familier des SPAD » [LU-OCR] ; S16, S42 [RÉSUMÉ] | **B** (qualitatif, trois descriptions d'époque) | Brevet britannique n° 100,632 de la Société Spad, « Strutting and staying of aerial machines », publié en décembre 1916 (S51) : piste pour le détail |
| Ailerons | sur le plan supérieur seulement ; étroits, articulés sur un longeron auxiliaire à environ 9 in (0,23 m) en arrière du longeron arrière ; commande par bielles logées dans le plan inférieur et renvois au pied des mâts extérieurs | — | S48 (1917, déduction de l'auteur) ; S49 (même schéma décrit sur le S.XI, non compensés) [LU-OCR] ; S41 [RÉSUMÉ] | B (qualitatif) | Surfaces et débattements : non trouvés |
| Empennage, gouvernes | gouvernail articulé à environ 10 in (0,25 m) au-delà de l'extrémité du fuselage ; dérive avançant de 12 in (0,30 m) devant le bord d'attaque du plan fixe ; plan fixe et profondeur « habituels » | in | S48 (1917) [LU-OCR] | C | Sur le S.XI de la même famille, le plan fixe est cambré sur ses deux faces, calé à 0°, profondeur en deux parties et guignols à l'intérieur du fuselage (S49) |
| Voie du train | 1,52 m | ft | S48 (1917) : 5 ft [LU-OCR] | C | XIII : 1,492 m (S15). Essieu coulissant dans des glissières, amortisseurs en sandow (S48) |
| Profil | section nommée **« Spad »** dans le NACA Report 93 (MIT, 1918) : épaisseur ≈ 5,4 %, cambrure ≈ 3,8 %, intrados creux | % de corde | `profil-spad7.md` § 2 [LU] | **B** | Type de SPAD mesuré non précisé. Polaires : calcul calé sur le RAF 15 de vraie grandeur (A pour ce profil, C pour le SPAD) |

### 2.2 Comparable : SPAD S.XIII C.1 (cotes publiées d'après un exemplaire américain)

Source : S15 [RÉSUMÉ], *This Day in Aviation*, d'après l'exemplaire 17956 essayé à McCook Field (projet P-154, 1921). Le rapport original (Air Service Information Circular vol. III n° 286, 1er octobre 1921) n'a pas été consulté (P12).

| Grandeur (XIII) | Valeur | Unité d'origine | Remarque |
|---|---|---|---|
| Envergure (deux plans, égale) | 26 ft 3¾ in (8,020 m) | ft-in | Envergure et corde égales sur les deux plans |
| Corde | 4 ft 7-1/8 in (1,400 m) | ft-in | |
| Écartement | 3 ft 10½ in (1,181 m) | ft-in | |
| Décalage | plan inférieur en arrière de 1¼° | degrés | Décalage exprimé en angle ; convention à vérifier |
| Calage | supérieur 1½°, inférieur 1° | degrés | soit un décalage angulaire de +0,5° (C) |
| Dièdre | 0° | degrés | |
| Longueur du fuselage | 20 ft 4 in (6,198 m) | ft-in | |
| Gouvernail de direction | hauteur 3 ft 10-5/8 in (1,184 m), corde maxi 2 ft 2 in (0,660 m) | ft-in | |
| Dérive | « 2 feet, 7/8-inch (0.876 m) » | ft-in | **Incohérence** : 2 ft 7/8 in = 0,632 m. À vérifier |
| Voie du train | 4 ft 10¾ in (1,492 m) | ft-in | |
| Masses (XIII) | 1 464 lb à vide (664 kg), 2 036 lb en charge (924 kg) | lb | Exemplaire américain de 1921 |

### 2.3 Traînées parasites (inventaire pour le modèle de vol)
Éléments à modéliser, d'après les descriptions : mâts interplans **et mâts intermédiaires** (S16), câbles de vol et d'atterrissage, tringlerie d'ailerons extérieure à renvois (attestée sur le XIII), radiateur frontal du V8 à refroidissement liquide, train fixe, mitrailleuse au-dessus du moteur (§ 7), tête du pilote. **Aucune dimension ni aucun coefficient n'a été trouvé.** Tout devra être calculé (Hoerner) et marqué `C:`, ou estimé (`E:`) et justifié par l'aérodynamicien.

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

## 3. Masses et centrage

| Grandeur | Valeur | Conditions | Source | Confiance | Remarque |
|---|---|---|---|---|---|
| Masse à vide | 500 kg | S.VII 150 ch | S9 [RÉSUMÉ] ; S5 [RÉSUMÉ], qui la tire de Davilla et Soltan (P6). **Époque** : 1 100 lb = 499 kg sur les **deux** lignes « Spa. 7C1 » (8Aa et 8Ab) de S46, et sur la ligne SPAD de S47 [LU-OCR] | **B** (renforcée) | Lecture validée par les sommes : masse à vide + charge utile = masse totale, à 1 lb près (C). Passera en A si l'image des pages confirme l'attribution des lignes |
| Masse au décollage | 705 kg (704,7 kg) | S.VII 150 ch | S5, S9 [RÉSUMÉ] ; **époque** : 1 530 lb = 694 kg sur la ligne 8Aa de S46 [LU-OCR] | B | Fourchette 692 à 705 kg ; 705 kg conservé en attendant l'image (journal) |
| Masse au décollage | **703 kg (1 550 lb)** | S.VII **180 ch** (ligne 8Ab de S46) | S46 [LU-OCR] ; 1 551 lb sur la ligne SPAD de S47 (moteur « 140 » hp selon l'OCR) | **B** (provisoire) | Remplace l'absence de valeur pour le 180 ch. Même ligne que les vitesses de 212 et 203 km/h (§ 5.2) : ensemble cohérent |
| Masse à vide | 510 kg | moteur **non précisé** | S6, S33 [RÉSUMÉ] | C | |
| Masse au décollage | 740 kg (1 632 lb) | moteur non précisé (« avec armement », S6) | S6, S33 [RÉSUMÉ] | C | Gardée en **variante** pour le 180 ch ; 37 kg de plus que S46, origine inconnue |
| Masse au décollage | 692 kg (1 525 lb), charge utile 213 kg (470 lb) | « S.VII » de 1917, moteur annoncé de « 160 hp » | S48 [LU-OCR] | C | Valeurs « approximatives » d'un article de presse américain |
| Masse au décollage | 705 kg « avec un 8-Aa de 180 ch » | — | S14 [RÉSUMÉ] | D | Fiche de musée qui confond 8Aa et 180 ch |
| Répartition de la charge (S46) | pilote 176 lb (80 kg) ; armement 88 lb (40 kg) ; huile 11 lb (5 kg) ; essence 176 lb (80 kg) sur la ligne 8Ab, et sans doute 156 lb (71 kg) sur la ligne 8Aa | lb | S46 [LU-OCR] | B (8Ab) ; C (8Aa) | Ligne 8Ab : 176 + 176 + 88 + 11 = 451 lb, total lu, contrôle exact (C). Ligne 8Aa : total lu 431 lb, mais l'OCR donne 176 lb d'essence ; 156 lb est **déduit** de la somme (C) |
| Carburant | 135 L au total : réservoir principal dans le fuselage sous le siège, réservoir d'appoint dans la partie centrale (cabane) | — | S13 [RÉSUMÉ] | C | 105 L dans une source de jeu (exclue, S39). Les 71 à 80 kg d'essence de S46 (environ 100 à 110 L à 0,72 kg/L, C) correspondent à la charge de l'essai, pas forcément aux réservoirs pleins |
| Huile | 5 kg (11 lb) dans la charge d'essai | lb | S46 [LU-OCR] | B (provisoire) | Contenance du réservoir : non trouvée |
| Munitions | non trouvé (nombre de cartouches) | — | — | — | Armement complet (arme et munitions, sans doute) : 40 kg (88 lb) dans S46 |
| Centrage, et son évolution | non trouvé | — | — | — | |
| Inertie | non trouvé | — | — | — | À estimer par composants (`fdm_lab inertia`) |

## 4. Moteur et hélice (résumé ; détail dans [`hispano-suiza-8a.md`](hispano-suiza-8a.md))
- 150 ch : Hispano-Suiza 8Aa ; 180 ch : Hispano-Suiza 8Ab. V8 à refroidissement liquide, entraînement **direct** de l'hélice.
- **Puissance** (troisième passe) : la forme de la courbe puissance/régime, la loi d'altitude et la consommation d'un Hispano de 180 hp (même alésage, course et compression que le 8Ab) ont été mesurées par le Bureau of Standards (NACA Report 46, lu). Confiance **B pour un 8Ab**. Détail et journal dans `hispano-suiza-8a.md` § 3 à 5.
- **Cote du 8Ab** (quatrième passe) : la contradiction 1 700 / 2 100 tr/min s'explique par **deux conventions de cote**. Le tableau officiel américain des moteurs (*Flight*, 13 février 1919) cote le Hispano de 180 hp **construit aux États-Unis** à 180 hp « normaux » à 1 700 tr/min, et le Hispano de 180 hp **français** à 2 000 tr/min en régime normal et 2 100 tr/min au maximum, avec des puissances au frein supérieures à 200 hp (lecture OCR à confirmer). Le moteur du Report 46, « coté 180 hp à 1 700 tr/min », suit la convention américaine. Détail : `hispano-suiza-8a.md` § 3.2.
- **Loi d'altitude du 150 ch** (quatrième passe) : un essai britannique **en vol** (R&M 462, mars 1918) sur un Hispano de 150 hp sans réducteur, monté sur S.E.5, donne une puissance proportionnelle à la pression à la puissance 1,04 environ, indépendante de la température (exposant lu dans l'OCR, à confirmer). Détail : `hispano-suiza-8a.md` § 4.3.
- **Régime en palier** : le tableau officiel de 1919 semble porter, au-dessus des vitesses du S.VII 180 ch, des régimes d'environ 1 720 et 1 680 tr/min (lecture provisoire, C). Si l'image le confirme, ce serait une donnée précieuse pour caler le pas de l'hélice.
- Hélice bipale en bois à pas fixe. Selon des forums [RÉSUMÉ], diamètre de 2,35 à 2,45 m et pas de 1,75 à 2,10 m pour le 150 ch. Rien de fiable pour le 180 ch. **Table C_T/C_P mesurée** sur une hélice en bois comparable (Navy D' du VE-7 à moteur Wright de 180 hp, 2,39 m, P/D 0,8) : A pour la mesure, **C pour le SPAD** (`hispano-suiza-8a.md` § 7.4).
- **Sens de rotation** : **horaire vu du siège du pilote** (confiance C), déduit de sources concordantes qui ne sont pas primaires. Troisième passe : les hélices du VE-7 portent « Rotation: Right hand » (NACA Report 301, p. 5). L'indice concorde, mais il ne relève pas la confiance, faute de définition de la convention et d'identification certaine du moteur. Conséquence attendue (C) : à pleine puissance et à basse vitesse, l'avion tend à partir à gauche, ce qui demande du pied à droite. C'est l'inverse du SPAD XIII à réducteur, pour lequel McCook note qu'il faut beaucoup de pied à gauche (S15).

## 5. Performances

Les deux séries « officielles » de **montée** (STAé et rapport britannique n° 96) ne sont connues que par un forum (S11), vu en résumé. Leur concordance est remarquable, mais elles transitent par **le même fil de forum** : ce ne sont pas deux transmissions indépendantes. D'où la confiance **C** et la mention « à vérifier sur l'original » (P10, P11).

**Quatrième passe : vitesses d'époque.** Deux documents d'époque lus donnent des vitesses du S.VII :
- **S46**, tableau « French Aeroplanes in Service at the Front ». Il a été compilé par le service aéronautique américain, publié par *Automotive Industries*, puis repris par *Flight* le 13 février 1919. *Flight* le juge « établi officiellement ». Il porte deux lignes « Spa. 7C1 » : l'une à moteur **8Aa** (195 sq ft), l'autre à moteur **8Ab** (193 sq ft). Les colonnes de vitesse sont celles de plusieurs altitudes, avec le régime moteur au-dessus de chaque vitesse ; les intitulés lisibles des colonnes de montée vont jusqu'à 13 100 et 16 400 ft (4 000 et 5 000 m). **L'OCR ne permet pas de dire à quelle altitude correspond la première colonne de vitesse** (sol ou 2 000 m) ;
- **S48**, description du « S.VII » publiée par *Aerial Age* (New York) et reprise par *Flight* le 16 août 1917. Elle donne 132 mph au niveau de la mer et 126 mph à 3 000 m. Le moteur y est annoncé de « 160 hp », mais ce couple de vitesses est exactement celui de la ligne 8Ab de S46.

### 5.1 S.VII 150 ch (Hispano-Suiza 8Aa)

| Essai | Valeur | Conditions (altitude, masse) | Source | Confiance |
|---|---|---|---|---|
| Vitesse max en palier | **192 km/h (119,3 mph)** | première colonne de vitesse de S46 : **sol ou 2 000 m, à lire sur l'image** ; masse de la ligne : 694 kg (1 530 lb) | S46 [LU-OCR] ; S5 [RÉSUMÉ], qui la rattache au **prototype SPAD V** | **B** (relevée de C) ; altitude non sourcée |
| Vitesse à l'altitude suivante | 189 km/h (117,3 mph) ou 179 km/h (111,3 mph) | deuxième colonne de S46 (3 000 m ?) | S46 [LU-OCR] : un chiffre illisible | D |
| Vitesse max (valeur mal attribuée) | 212 km/h (132 mph) à 2 000 m | S9 l'attribue au 150 ch | S9 [RÉSUMÉ] | **Réattribuée au 180 ch** (§ 5.2), où deux documents d'époque la confirment |
| Vitesses d'un SPAD monoplace du service italien | environ 194 km/h (120,5 mph) et un autre chiffre illisible | moteur Hispano « 140 » hp selon l'OCR ; 703 kg (1 551 lb) | S47 [LU-OCR] | D (colonnes non identifiées) |
| Montée à 2 000 m | 6 min 40 s | STAé, 8Aa ; masse non précisée | S11 [RÉSUMÉ] | C |
| Montée à 3 000 m | 11 min 20 s, ou 11 min 30 s selon la transcription | STAé | S11, S12 [RÉSUMÉ] | C |
| Montée à 4 000 m | 19 min 30 s | STAé | S11 [RÉSUMÉ] | C |
| Plafond théorique | 5 500 m | STAé | S11 [RÉSUMÉ] | C |
| Montée à 6 500 ft (1 981 m) | 6,3 min | rapport britannique n° 96, avril 1917 | S11 [RÉSUMÉ] | C |
| Montée à 10 000 ft (3 048 m) | 11,5 min | idem | S11 [RÉSUMÉ] | C |
| Montée à 15 000 ft (4 572 m) | 24,5 min | idem | S11 [RÉSUMÉ] | C |
| Plafond estimé | 19 000 ft (5 791 m) | idem ; c'est une **estimation** du rapport, jugée « probablement un peu faible » | S11 [RÉSUMÉ] | C |
| Montée à 2 000 m (autre valeur) | 4,5 min | **prototype SPAD V** | S5 [RÉSUMÉ] | D pour le S.VII de série |
| Plafond pratique | 5 335 m (17 500 ft) | — | S5, S33 [RÉSUMÉ] | C |
| Endurance | 1,5 à 2 h | — | S13 [RÉSUMÉ] | C |
| Montée en 10 min | 9 300 ft (2 835 m) | « S.VII » de S48, moteur annoncé de « 160 hp », 692 kg | S48 [LU-OCR] | C : sous-version incertaine. Par interpolation (C), la série STAé du 150 ch donne environ 10 min 35 s pour 2 835 m, et les 9 min 50 s à 3 000 m du 180 ch environ 9 min. Le chiffre tombe entre les deux |
| Temps de montée et plafond des lignes « Spa. 7C1 » de S46 | **non lus** | — | S46 : colonnes perdues dans l'OCR | À lire sur l'image (§ 9.6) |
| Vitesse de décrochage ou d'atterrissage | non trouvé | — | — | — |

Taux de montée moyens déduits (C : différences de temps, valeurs STAé) : environ 5,0 m/s de 0 à 2 000 m ; 3,4 à 3,6 m/s de 2 000 à 3 000 m ; 2,0 à 2,1 m/s de 3 000 à 4 000 m. Les valeurs britanniques donnent 5,2, 3,4 et 2,0 m/s sur des tranches voisines (C).

### 5.2 S.VII 180 ch (Hispano-Suiza 8Ab)

| Essai | Valeur | Conditions | Source | Confiance |
|---|---|---|---|---|
| Vitesse max en palier | **212 km/h (132 mph)** | **sol** selon S48 ; **2 000 m** selon S9 ; première colonne de S46, altitude à lire. Régime lu au-dessus : environ 1 720 tr/min (C) ; masse 703 kg (1 550 lb) | S46, S48 [LU-OCR] ; S9 [RÉSUMÉ] | **B** pour la valeur ; altitude **non tranchée** |
| Vitesse en palier à 3 000 m | **203 km/h (126 mph)** | 3 000 m explicitement dans S48 ; deuxième colonne de S46. Régime lu au-dessus : environ 1 680 tr/min (C) | S46, S48 [LU-OCR] | **B** |
| Vitesse max (autres valeurs) | 208 km/h (129 mph), altitude non précisée (S5) ; 188 km/h (source russe citée par S12) | — | [RÉSUMÉ] | C (208) ; D (188) |
| Gain de vitesse sur le 150 ch | +16 km/h à 4 000 m | — | S12 [RÉSUMÉ] | C |
| Montée à 3 000 m | 9 min 50 s (contre 11 min 20 s pour les premiers S.VII) | — | S12 [RÉSUMÉ] | C |
| Montée à 2 000 m | 4 min 40 s | — | S11 [RÉSUMÉ] (un participant du forum) | D : peu cohérent avec 9 min 50 s à 3 000 m (journal) |
| Plafond pratique | 6 200 m | — | S7 [RÉSUMÉ] (Musée de l'Air) | C |
| Plafond (autre valeur) | 6 553 m (21 500 ft) | — | sites de jeux et reprises (S39) | D |
| Endurance | non trouvé hors des jeux | — | — | — |
| Décrochage | non trouvé | — | — | — |

### 5.3 Vitesse de décrochage : seul indice
Selon un fil de forum qui reprend un récit attribué à Frank Tallman, le décrochage moteur réduit d'un S.VII à moteur Wright-Hispano E de 180 ch se produit « vers 47 mph » (75,6 km/h, conversion C). Masse inconnue, fin des années 1950 ou années 1960 (S31 [RÉSUMÉ]). Confiance **D** : attribution incertaine, avion et moteur postérieurs.

### 5.4 Cibles proposées pour `essais.ron` (à valider par l'aérodynamicien)
- **180 ch, confiance B** (quatrième passe) : `MaxLevelSpeed` à **3 000 m = 203 km/h**, à **703 kg**. C'est la seule cible dont la valeur, l'altitude et la masse viennent ensemble de documents d'époque (S46, S48). Le régime d'environ 1 680 tr/min lu au-dessus (C) peut servir de contrôle secondaire, pas de cible.
- **180 ch, confiance B pour la valeur** : 212 km/h en palier bas. L'altitude reste à trancher (sol selon S48, 2 000 m selon S9). En attendant l'image de S46, marquer l'altitude `E:` et tester les deux variantes. Ordre de grandeur de l'ambiguïté (C, interpolation linéaire sur les seuls chiffres de S48) : si 212 km/h est la vitesse au sol, la vitesse à 2 000 m serait d'environ 206 km/h, soit un écart d'environ 6 km/h.
- **150 ch, confiance B pour la valeur** : 192 km/h en palier bas, même réserve d'altitude.
- Montée **pour le 150 ch**, confiance **C** : 2 000 m en 6 min 40 s, 3 000 m en 11 min 20 s à 11 min 30 s, 4 000 m en 19 min 30 s (la série la mieux étayée, transmise par un même forum).
- Plafond, confiance C : environ 5 300 à 5 800 m pour le 150 ch (fourchette S5/S11) ; 6 200 m pour le 180 ch (S7).
- **Conséquence pour le M1** : il existe désormais des cibles B de **vitesse** pour le 180 ch, avec une masse cohérente. Les cibles de **montée** restent en C pour les deux versions, faute d'avoir lu les colonnes de montée de S46 sur l'image.

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

- **Essais allemands d'appareils capturés** : **non trouvés**. Des S.VII capturés ont été utilisés par les Allemands et les Austro-Hongrois (S5 [RÉSUMÉ]), mais aucun rapport d'essai (Idflieg, Adlershof) n'a été localisé.

**Italien**

- **T14 — Francesco Baracca** · Italie, as (70a, puis 91a Squadriglia) · 1917 · S.VII, puis XIII · italien · S25, S26 [RÉSUMÉ].
  Il essaie le S.VII le 20 mars 1917 (S25). Il serait revenu du XIII au VII en octobre 1917, et S26 lui prête cette phrase (en anglais) : peu importe que le VII n'ait qu'une mitrailleuse, une seule suffit à un bon chasseur.
  **Citation non vérifiée sur l'original.** Ses lettres à sa mère forment le corpus principal (S25), conservé au Museo del Risorgimento de Milan et à la Biblioteca Trisi de Lugo (P13). Non consultées.

- **T15 — Sources italiennes secondaires** [RÉSUMÉ] :
  - S23 (Treccani) : le SPAD est supérieur aux Nieuport ; c'est sur lui que la 91a remporte ses victoires.
  - S24 (notice Baracchini) : moteur plus puissant que le Nieuport 11, « ottimi tempi di salita e caratteristiche di picchiata ».
  - S27 (musée de Vigna di Valle, S.153) : avion puissant et difficile à piloter, excellent en montée et en piqué ; apprécié pour sa vitesse en piqué et sa résistance aux dégâts malgré les problèmes de refroidissement et l'armement léger.
  Biais : sources commémoratives. La comparaison se fait **avec le Nieuport 11**, ce qui explique sans doute l'éloge de la montée, contraire à T3.

- **T16 — Accident de Luigi Olivari, 13 (ou 15) octobre 1917** · S28 [RÉSUMÉ] : son S.VII décroche au décollage à Santa Caterina, près d'Udine, accident mortel. C'est un **fait**, pas un témoignage. Une seule source, dont la date diverge.

**Pilotes de répliques modernes** : aucun témoignage publié trouvé sur une réplique de S.VII. Celle d'Old Rhinebeck vole avec un Lycoming de 150 ch (S34 [RÉSUMÉ]) et n'est donc pas représentative. Seul T9 (Tallman, avion original) a été repéré.

### 6.2 Traits de comportement

| Trait | Témoignages (sources, langues, camps) | Concordance | Attente vérifiable pour `essais.ron` | Confiance |
|---|---|---|---|---|
| **Structure robuste, tenue en piqué quasi vertical à pleine puissance** | T2 Guynemer (FR, 1916) [LU] ; T6 Hall (EN, 1917) [LU] ; T13 Fieseler (DE) [RÉSUMÉ] ; T10, T15 (secondaires) | **Confirmé** : deux témoignages directs indépendants, plus des secondaires des deux camps | **M1, robustesse** : depuis le trim à 3 000 m, piqué de −80° gaz pleins pendant 20 s (situation de T6). Attendu : pas de NaN ni de divergence ; le moment de tangage reste rappelant (pas d'engagement en piqué divergent, profondeur au neutre) ; la ressource au manche ramène l'avion en palier. Vitesse et facteur de charge atteints enregistrés. **M4** : aucune limite structurelle chiffrée n'est sourcée ; ne pas en inventer | B (trait) ; aucun chiffre |
| **Montée du 150 ch jugée insuffisante face aux chasseurs allemands** | T3 Guynemer (FR, 1916) [LU] ; T6 Hall (EN, 1917, moteur reconstruit) [LU] ; contre : T15 (IT, face au Nieuport 11) | **Partiellement confirmé** : jugement relatif au comparant | Pas d'essai qualitatif. Cible **quantitative** : temps de montée STAé (§ 5.1). Contrôle : au même rapport puissance/masse, la montée du 150 ch ne doit pas dépasser les cibles | C |
| **Moteur sensible à la température** (surchauffe en montée prolongée en altitude, refroidissement rapide en plané) | T6 Hall (S.VII, 1917) [LU] ; T8 Meister (XIII, 1921) [RÉSUMÉ] ; T10, T15 ; S5 | **Confirmé pour la famille SPAD/Hispano** | **Hors M1** (pas de modèle thermique). Pour plus tard : en montée à pleine puissance au-delà de 3 500-4 000 m, volets de radiateur fermés, la température d'eau monte jusqu'à imposer une réduction ou un piqué ; en plané, elle chute vite | B− (qualitatif ; T8 vu en résumé seulement) |
| Boucle aisée | T1 Guynemer [LU] ; T4 (c), témoin de boucles après combat [LU] | **À confirmer** : un seul jugement de pilote | Provisoire, non bloquant : boucle depuis la vitesse de croisière à pleins gaz, bouclée sans décrochage au sommet | C |
| Vrille : entrée « paresseuse et irrégulière, mais très douce » | T1 Guynemer [LU] | **À confirmer** : un seul témoignage | Provisoire : `SpinEntryRecovery(turns: 3)`. Rotation établie lentement et irrégulière, sortie franche aux commandes croisées. Pas de seuil chiffré | C |
| Moins maniable que le Nieuport ; commandes lourdes | Secondaires : S5, S32/T10, S33, S34 [RÉSUMÉ] ; **contre** : T9 Tallman (commandes légères) [RÉSUMÉ] et T4 (b), Bordeaux (« facilité de conduite » supérieure à celle de l'Albatros) [LU] | **Contesté** ; le comparant change (Nieuport ou Albatros) | Aucune. Ne pas régler le modèle pour l'obtenir : le rayon de virage doit émerger de la charge alaire et de la puissance | — |
| Décrochage brutal à basse vitesse, mauvais plané, atterrissage au moteur | T9 Tallman [RÉSUMÉ] ; secondaires (`profil-spad7.md` A12 : forum, page Memorial Flight ou NASM) ; fait T16 (Olivari) ; S21 (XIII) | **À confirmer** : aucun témoignage direct lu | Provisoire, non bloquant : décrochage moteur réduit avec abattée nette et départ en roulis, plané plus pentu que celui d'un Nieuport. **Aucun seuil chiffré** tant qu'il n'est pas sourcé. Mécanismes physiques maintenant documentés, sans valeur SPAD : chute de portance nette après le maximum du RAF 15 à Reynolds élevé (`profil-spad7.md` § 3.1) ; décrochage du plan supérieur avant le plan inférieur (§ 2.4, S44). Ils doivent **produire** le trait, pas le remplacer | C |
| Très stable en vol, bonne plate-forme de tir | T8 Meister (XIII) [RÉSUMÉ] ; S5 [RÉSUMÉ] | **À confirmer** pour le VII | Provisoire : modes propres. Oscillation d'incidence bien amortie, spirale peu divergente | C |
| Couple marqué, embardée au décollage | T8 (XIII, rotation inverse) | **À confirmer** ; sens déduit pour le VII | M2 (sol) : à pleine puissance, l'avion part à gauche, contré au pied droit (déduit du sens de rotation, C) | C |
| Visibilité latérale et vers l'arrière limitée, angle mort sous le moteur | T4 (a) Bordeaux [LU] ; S9 [RÉSUMÉ] (avant et vers le bas médiocres) | **À confirmer** | Pour la caméra cockpit (M2+), pas pour le modèle de vol | C |
| Au sol : roule bien, capote peu, s'arrête court | T8 (XIII) [RÉSUMÉ] | **À confirmer** | M2 (sol) | C |

### 6.3 Réputation

| Élément | Statut | Sources | Usage au récit |
|---|---|---|---|
| « Le terrible Spad », l'avion des as, arme d'attaque par excellence | **Confirmé** comme réputation (française et allemande) | T4, T11 [LU] ; T15 | Mess : fierté d'escadrille ; le joueur reçoit « un Spad » |
| « Le Spad tient » : la solidité qui sauve | **Confirmé** | T2, T6 [LU] | Carnet de vol après un atterrissage brutal ; pilote qui « doit la vie à sa machine » |
| Rapide, et surtout en piqué : on rompt le combat en piquant | **Confirmé** comme réputation ; chiffre de vitesse en piqué non sourcé | T4, T10, T15 ; T13 | Conseil d'ancien à la recrue : « ne tourne pas, pique » (formule de jeu, pas une citation) |
| « Plongée sûre jusqu'à 400 km/h » | **Légende probable** : chez Bordeaux, les 400 km/h désignent la **vitesse de rapprochement** de deux avions, pas une vitesse de piqué (S1 [LU]). Le chiffre circule ailleurs comme vitesse de piqué (résumé de recherche) | S1 ; résumé non attribué | À ne pas reprendre comme fait ; possible rumeur de mess |
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
- **Anglais** : Charles J. Biddle, *The Way of the Eagle* (Scribner, 1919 ; P7). Lettres de la SPA 73 (GC 12), en Flandre fin 1917 : la source la plus proche du contexte de jeu. Hall et Nordhoff, *The Lafayette Flying Corps* (1920 ; P15).
- **Allemand** : rapports de Jasta et notes de l'Idflieg sur les appareils capturés ; texte complet d'Udet (projekt-gutenberg.org, inaccessible).
- **Italien** : lettres de Baracca (P13) ; G. Alegi et P. Varriale, *Italian SPADs at War* (Windsock Datafile 155 ; P5).

## 7. Armement
- **Une** mitrailleuse Vickers de calibre .303 (7,7 mm), au-dessus du moteur, synchronisée pour tirer à travers le champ de l'hélice (S5, S6 [RÉSUMÉ]). Synchronisation **Birkigt** d'après une source spécialisée non identifiée dans le résumé : à vérifier.
- Une deuxième mitrailleuse a été envisagée puis abandonnée, car elle dégradait les performances (S6 [RÉSUMÉ]). Un appareil britannique, doté d'une Lewis supplémentaire sur le plan supérieur, a été essayé à Martlesham Heath en mai 1917 : perte de performances jugée trop forte (S5 [RÉSUMÉ]).
- Fusées pour l'attaque des ballons : six, sur des supports fixés aux mâts (trois de chaque côté), avec une plaque de tôle protégeant l'entoilage du plan inférieur sous les supports ; mise à feu électrique par un bouton dans le poste (S3 [LU], Escadrille Lafayette, 1917). Hall ne nomme pas le type de fusée : « Le Prieur » est à vérifier.
- Nombre de cartouches : **non trouvé**.
- Pour mémoire : Guynemer constate à l'automne 1916 que les enrayages de sa mitrailleuse viennent du froid, et trouve un remède (S1 [LU]).

## 8. Livrées (bref ; le M1 ne les utilise pas)
- **Premières séries** : toile vernie incolore ou teinte « écru » / beige pâle, capots métalliques un peu plus foncés (S37 [RÉSUMÉ], descriptions divergentes).
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

### 9.3 Pistes bibliographiques [PISTE] (non consultées ; éditions à vérifier)
- **P1** — J. M. Bruce, *Spad 7.C1*, Windsock Datafile n° 8, Albatros Productions, 1988 (fiche de catalogue de libraire).
- **P2** — T. Gronczewski et S. M. Fleischer, *Spad VII C.1*, Aviatik Vintage Aircraft n° 1, 2004 (14 pages de plans selon une revue).
- **P3** — M. Velek, M. Ovčáčík et K. Susa, *SPAD S.VII C1*, 4+ Publications n° 19 (4+019), 2004 (plans au 1/48).
- **P4** — J. Guttman, *SPAD VII Aces of World War I*, Osprey, Aircraft of the Aces 39, 2001.
- **P5** — G. Alegi et P. Varriale, *Italian SPADs at War*, Windsock Datafile n° 155.
- **P6** — J. J. Davilla et A. M. Soltan, *French Aircraft of the First World War*, Flying Machines Press, 1997 (source des masses selon S5).
- **P7** — C. J. Biddle, *The Way of the Eagle*, Charles Scribner's Sons, New York, 1919.
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

### 9.4 Historique des escadrilles
- Albin Denis, historiques des escadrilles (`http://albindenis.free.fr/Site_escadrille/escadrille003.htm`) : **inaccessible**, à consulter en priorité pour la SPA 3 (terrains, appareils, dates).

### 9.5 Sites inaccessibles depuis l'outil de lecture de l'historien (testés le 2026-10-20 ; à consulter plus tard)
albindenis.free.fr ; en.wikipedia.org ; fr.wikipedia.org ; it.wikisource.org ; catalog.hathitrust.org ; gallica.bnf.fr (403 selon le coordinateur) ; www.gutenberg.org (direct) ; projekt-gutenberg.org ; www.theaerodrome.com ; flyingmachines.ru ; www.avionslegendaires.net ; historynet.com ; www.flightglobal.com (archives de *Flight*) ; www.thisdayinaviation.com ; www.hydroretro.net ; digital.library.unt.edu ; bibliotheques-numeriques.defense.gouv.fr ; m-selig.ae.illinois.edu ; aergistal.github.io ; web.archive.org ; www.google.com. Seuls github.com, raw.githubusercontent.com et, par intermittence, api.github.com répondent.

**Cas particulier** : archive.org et ntrs.nasa.gov répondent au `curl` du conteneur (coordinateur, 2026-10-20), mais **pas** à l'outil de lecture de l'historien (DNS en échec). Ils ne sont exploitables que par téléchargement préalable (§ 9.6).

### 9.6 Documents à télécharger en priorité (identifiants vérifiés par le moteur de recherche, contenu non lu)
Pour chaque document d'archive.org, récupérer le texte OCR à l'adresse `https://archive.org/download/<identifiant>/<identifiant>_djvu.txt` et, si possible, le PDF. Pour NTRS, l'adresse est `https://ntrs.nasa.gov/api/citations/<id>/downloads/<id>.pdf`.

| Priorité | Document | Identifiant | Ce qu'on y cherche |
|---|---|---|---|
| 1 | *Flight* 1917 (collection « Flight International Magazine », 1909-1935 ; numéros nommés sur le modèle `Flight_International_Magazine_1919-06-12-pdf`) | collection `Flight_International_Magazine`, numéros 1917 à filtrer par identifiant | Description du SPAD, reprises d'essais britanniques ; chercher « Spad » dans les `_djvu.txt` |
| 1 | *The Aeroplane*, vol. 12 et 13 (1917) | identifiants non relevés (le scan de la revue pour 1911-1922 est signalé sur archive.org) | Idem |
| 1 | *Bibliography of Aeronautics* 1917-1921 (P. Brockett, Smithsonian) | `bibliographyofa191721broc` | **Outil de repérage** : références d'articles d'époque sur le SPAD (*L'Aérophile*, *Flight*, *Aerial Age*…) |
| 2 | *Technical Report of the Advisory Committee for Aeronautics* 1917-18, vol. 2 | `technicalreport1917aero_2` | Essais du NPL et du RAE ; essais en vol éventuels ; profils RAF |
| 2 | Autres volumes de la même série (1916-17, 1918-19, 1919-20, 1920-21, 1921-22) | identifiants non relevés (modèle supposé : `technicalreport19xxaero…`) | ARC R&M 586 (juillet 1921) : courbe de puissance de l'Hispano |
| 3 | *All the World's Aircraft* 1924 (le plus ancien volume trouvé sur archive.org) | `allworldsaircraft1924` | Probablement sans fiche du S.VII ; les volumes 1917-1919 ne sont pas trouvés sur archive.org |
| 3 | NACA Reports n° 233, 309, 279, 304, 331, 93 et 124 | voir `profil-spad7.md` § 3 et 6.1 | **Fait (troisième passe)** : téléchargés par le coordinateur et lus |
| 1 | ARC R&M 586 (juillet 1921), figure 10 | ni archive.org ni NTRS ; archives ARC de Cranfield (AERADE) | Courbe puissance/régime du Hispano de **150 ch** (aucune mesure lue pour le 8Aa) |
| 2 | NACA Reports 220 et 292 | NTRS, identifiants non relevés | Essais en vol des hélices du VE-7 (`hispano-suiza-8a.md` § 9) |
| 2 | ARC R&M 774, 816, 857, 901 et 997 ; NACA TR 256 | archives ARC (Cranfield) ; NTRS | Cellules biplans RAF 15 et méthode de Glauert (§ 2.4) |

## 10. Journal des décisions
- **2026-10-20 — Sous-version pour la SPA 3, été 1917.** S5, S8 et S16 font du 8Ab de 180 ch le moteur des S.VII neufs à partir du printemps 1917 (« avril 1917 » pour S16). Guynemer juge le 150 ch insuffisant dès décembre 1916 (S1). Aucun état des appareils de la SPA 3 n'a été trouvé. **Retenu : S.VII 180 ch (8Ab), confiance C**, en sachant que la SPA 3 alignait sans doute encore des 150 ch et quelques SPAD XIII à partir de fin juillet 1917 (S18, S1). Ce choix sera à revoir dès que l'historique d'Albin Denis ou les journaux de marche seront accessibles.
- **2026-10-20 — Version à utiliser pour valider le modèle de vol.** Les seules mesures d'époque trouvées (STAé et rapport britannique n° 96, via S11) concernent le **150 ch**. **Recommandé** : valider d'abord le modèle sur la configuration 150 ch (masse de 705 kg, S5/S9), puis passer au 180 ch en changeant le moteur et la masse. La cellule est la même ; les cibles du 180 ch sont plus faibles (C ou D). Décision finale au joueur et à l'aérodynamicien.
- **2026-10-20 — Envergure supérieure.** 7,80 à 7,82 m selon les sources : **retenu 7,82 m** (S7, institutionnelle, concordante avec S8 et avec les 25 ft 8 in de S14), confiance B.
- **2026-10-20 — Envergure inférieure.** S9 donne 7,61 m ; S16 décrit des envergures égales. **Non tranché.** Retenu provisoirement 7,61 m (C), à vérifier sur un plan (P1 à P3) ou sur un appareil (P17).
- **2026-10-20 — Surface alaire.** 17,85 m² (S6, S8, S10, S33) contre 18 m² (S7, sans doute arrondi). **Retenu 17,85 m²** (B pour le chiffre). Ce qu'elle inclut est inconnu ; la corde moyenne apparente (environ 1,16 m, C) paraît faible et devra être recoupée.
- **2026-10-20 — Masses.** 500 kg à vide et 705 kg au décollage pour le 150 ch (S9, et S5 qui cite Davilla et Soltan) : **retenu, B**. 510 et 740 kg (S6, S33) : motorisation non précisée, **notées sans attribution** ; leur emploi pour le 180 ch est une hypothèse (C).
- **2026-10-20 — Vitesse maximale.** 192 km/h (150 ch) et 208 km/h (180 ch) (S5), sans altitude ; S5 rattache le 192 km/h au prototype SPAD V. Les 212 km/h à 2 000 m de S9 sont isolés : écartés (D). **Aucune valeur exploitable en confiance B.**
- **2026-10-20 — Montée du 180 ch.** S12 : 3 000 m en 9 min 50 s ; S11 : 2 000 m en 4 min 40 s. Ensemble, ces deux chiffres impliqueraient une tranche de 2 000 à 3 000 m plus lente que celle du 150 ch : incohérent. **Retenu 9 min 50 s à 3 000 m (C)** ; les 4 min 40 s sont écartés (D).
- **2026-10-20 — Plafond du 180 ch.** 6 200 m (S7, musée) contre 6 553 m (sites de jeux, S39). **Retenu 6 200 m (C)**, la source institutionnelle étant préférée.
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
