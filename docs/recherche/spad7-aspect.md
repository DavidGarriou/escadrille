# SPAD S.VII C.1 (150 ch) — fiche d'aspect : modèle 3D et livrée

- Identifiant : `spad7`. Fiches liées : [`spad7.md`](spad7.md) (histoire, géométrie, performances), [`spad7-calculs.md`](spad7-calculs.md) (cotes lues sur le dessin de 1917, § 3 et § 3.1), [`hispano-suiza-8a.md`](hispano-suiza-8a.md) (hélice, § 7.5).
- Dernière mise à jour : 2026-10-26
- État : **en cours**, première passe. Elle répond au plan [`m3-modele-spad7.md`](../plans/m3-modele-spad7.md), parties B1 (aspect) et B2 (livrée).
- Objet : tout ce qu'il faut pour modéliser et texturer un S.VII de 150 ch de 1917, extérieur et premier niveau du poste, puis choisir une livrée réelle.

## 0. Méthode, accès et limites

**Accès réseau (2026-10-26).** L'outil de lecture de pages échoue (DNS) sur museeairespace.fr, francearchives.gouv.fr, imagesdefense.gouv.fr, commons.wikimedia.org et en.wikipedia.org. Seul raw.githubusercontent.com répond : les livres de Bordeaux (`spad7.md` S1) et de Hall (S3) y ont été relus. Tout ce qui vient d'autres sites est vu **en résumé du moteur de recherche**.

**Statuts** (mêmes conventions que `spad7.md` § 0) :
- **[LU]** : texte intégral lu ;
- **[LU-OCR]** : texte OCR d'archive.org lu comme des données ;
- **[LU-IMG]** : lu sur l'image d'une page ;
- **[RÉSUMÉ]** : vu seulement dans un résumé de recherche, page non ouverte. Aucun chiffre [RÉSUMÉ] n'est tenu pour vérifié ;
- **[PISTE]** : référence repérée, non consultée.

**Documents locaux consultés** (dossier de travail de la session principale, rien n'est versé au dépôt) :
- *Flight*, 16 août 1917, p. 840-841 : texte de S48 [LU-OCR] et vues réduites du dessin trois vues, tirées du rendu à 600 points par pouce [LU-IMG] ;
- *Flight*, 16 mai 1918, p. 527-529 : rapport allemand sur le biplace S.XI (S49) [LU-OCR] ;
- G. Hartmann, « Les héliciers français » (V1) : texte [LU] et p. 13 sur l'image [LU-IMG].

**Comparables.** Le biplace S.XI (S49) et le SPAD XIII sont d'autres cellules : leurs détails ne valent pour le S.VII qu'en **C** au mieux, comme habitudes de construction de Béchereau.

**Non consultés** : les monographies qui répondraient sans doute à la moitié des questions ci-dessous (J. M. Bruce, Windsock Datafile n° 8 et *SPAD Scouts*, Aircam n° 9 ; Velek et al., 4+ Publications n° 19 ; « Project Butterfly ») ; ni l'avion conservé S.254, ni l'aile 2016.16.1 du Musée de l'Air n'ont été vus. Les demandes sont dans [`demandes.md`](demandes.md) (D4, D8 à D12).

**Exclusions.** Profils de jeux, peaux de simulateurs et décalques du commerce ne sont pas des sources (`spad7.md` S39). Les revues de maquettes ne sont que des pistes.

**Droits.** Aucune photo, aucun plan, aucun profil couleur n'entre dans le dépôt. Les photos sont citées comme **références** ; leur licence est indiquée quand elle est connue (§ 11).

**Repère des cotes** : celui de `spad7-calculs.md` § 3 (origine au bord d'attaque du plan inférieur, dans l'axe ; x vers l'avant, z vers le bas).

## 1. Synthèse pour le modélisateur

| Élément | À modéliser | Confiance | Statut |
|---|---|---|---|
| Capot | aluminium **peint** d'une teinte voisine de la toile, un peu plus foncée ; bossages des arbres à cames ; perforations autour des bossages ; panneau à persiennes sous l'échappement | C | forme d'ensemble sourcée ; détails à mesurer sur photo |
| Radiateur (150 ch, début 1917) | ouverture circulaire d'environ 0,71 m, nid d'abeilles **sans volets** ; arbre d'hélice au centre | C | volets verticaux réservés aux fins de série |
| Échappements | par côté, un collecteur de 4 tubulures, puis un tuyau le long du flanc jusque derrière le poste | C (disposition), D (longueur) | fin du tuyau à mesurer |
| Moyeu | sans casserole | C | dessin de 1917 |
| Cellule | plans presque rectangulaires, saumons droits à coins arrondis ; découpe en demi-cercle au-dessus du poste | C | dimensions de la découpe à mesurer |
| Nervures | environ 20 par demi-plan supérieur | **ESTIMÉ** | transposé du S.XI (190 mm) |
| Bord de fuite | fil métallique (« wire », métal non précisé), toile festonnée entre les nervures | C | famille SPAD |
| Œillets de drainage | intrados, près du bord de fuite | B (existence), C (position) | nombre non trouvé |
| Mâts | 4 interplans et 4 intermédiaires, bois profilé, entretoise d'acier entre intermédiaires | B (disposition), C (matériau) | sections non trouvées |
| Câbles | du flanc du fuselage aux mâts extérieurs, croisés aux intermédiaires ; câbles de vol doublés | B / C | diamètres non trouvés |
| Renvois d'ailerons | bielles dans le plan inférieur, renvois au pied des mâts extérieurs, biellettes verticales | B | cotes non trouvées |
| Empennage | guignols à l'intérieur, pas de câbles de gouverne extérieurs | C | famille SPAD |
| Fuselage | profond, arrondi sur les quatre faces ; avant tôlé, arrière entoilé | C | 0,80 × 1,10 m **ESTIMÉ** (`spad7-calculs.md`) |
| Pare-brise | en verre ; probablement trois pans à cadre d'aluminium | C | dimensions non trouvées |
| Train | jambes en bois entoilé, essieu coulissant, sandows, voie 1,52 m | C | roues : **ESTIMÉ** |
| Vickers | une, au-dessus du moteur, synchronisée | B | décalage latéral et goulottes non trouvés |
| Hélice Éclair 6 | 2,40 × 1,95 m, moyeu de 150 mm ; noyer lamellé verni, sans gaine | C | forme de pale non trouvée |
| Poste | siège étroit, manche à détente, palonnier à barre, planche en contreplaqué | C | disposition : **ESTIMÉ** d'après le S.XI |
| Toile | écrue enduite, blanc jaunâtre à jaune pâle | C | aucune référence de couleur |

## 2. Capot et nez

| Élément | Description | Cotes | Sources | Confiance | Inconnu, `ESTIMÉ` |
|---|---|---|---|---|---|
| Capot : matériau | partie avant habillée d'aluminium | — | S48 (1917) [LU-OCR] ; tôlage de l'avant du fuselage : V13 [RÉSUMÉ] | B | limites exactes des tôles |
| Capot : peint ou brut | **peint**. Sur les avions non camouflés, les parties métalliques sont peintes d'un jaune clair plus foncé que la toile vernie | — | V15, V16 (d'après J. M. Bruce), V14 [RÉSUMÉ] : trois pistes concordantes | C | Sur S.254, la restauration a décapé les capots et les a repeints en mat ; **la teinte d'origine est inconnue** (V2, V4 [RÉSUMÉ]). Aucune source pour un aluminium brut |
| Ouverture du radiateur | circulaire, « qui rappelle le capot d'un moteur rotatif » | diamètre dessiné **0,71 m** | S48 [LU-OCR] ; lecture du dessin, `spad7-calculs.md` § 3 | C (forme), D (diamètre : dessin de presse) | Un résumé russe parle d'un radiateur frontal **octogonal** (V12) : peut-être le faisceau derrière l'ouverture ronde. Non tranché |
| Radiateur : évolution | trois états : (1) premières séries, capot serré, **petite ouverture, nid d'abeilles nu, sans volets** ; (2) ouverture agrandie ; (3) fin de série, **neuf volets verticaux** devant le radiateur. Au front, on perçait des trous ou ajoutait des persiennes contre la surchauffe d'été | — | V12, V13, V14 [RÉSUMÉ] ; revue de la maquette Airfix (même famille de pistes) | C | Dates de chaque état et numéros de série : non trouvés. **Pour un 150 ch du début de 1917 : état (1), sans volets (C)** |
| Arbre d'hélice | traverse le radiateur en son centre | — | V12 [RÉSUMÉ] ; vue de face du dessin de 1917 [LU-IMG] | C | diamètre du passage |
| Volets et commande | sur le biplace S.XI : radiateur à volets formant le nez, levier de commande à gauche du pilote | — | S49 [LU-OCR] | C (famille) | sans objet pour l'état (1) |
| Bossages | de chaque côté du capot, là où dépassent les carters des arbres à cames | — | S48 [LU-OCR] ; vue de dessus du dessin | C | forme et longueur exactes |
| Perforations | percées autour des bossages pour l'entrée d'air | — | S48 [LU-OCR] | C | nombre, diamètre, motif |
| Persiennes latérales | sur les avions de construction française, un panneau d'accès **à persiennes** sous le tuyau d'échappement, de chaque côté ; plein et bombé sur les avions britanniques. Les premières séries ont le moins de persiennes | — | V13 (Wikipédia), V14 [RÉSUMÉ] ; grilles amovibles dans les flancs (V12) | C | nombre de persiennes |
| Échappements | dessin de 1917 : par côté, quatre tubulures réunies en collecteur sous le capot, puis un tuyau le long du flanc, qui se termine nettement en arrière du poste. Pistes : longs tuyaux des deux côtés, le gauche servant de marchepied (V18) ; quatre sorties par côté et un support juste derrière le poste (revue de maquette). Le prototype avait des tubulures courtes (V12). Sur le S.XI, collecteurs jusque derrière le siège du pilote (S49) | longueur à mesurer | S48, dessin [LU-IMG] ; S49 [LU-OCR] ; V12, V18 [RÉSUMÉ] | C (disposition), D (longueur) | Le dessin et la piste de maquette divergent sur la fin du tuyau : **à trancher sur photo** |
| Réservoir d'eau, trop-plein | non trouvés pour le S.VII. S.XI : réservoir dans le plan supérieur devant le longeron avant ; trop-plein sous le fuselage | — | S49 [LU-OCR] | C (famille) | — |
| Moyeu, casserole | **pas de casserole** sur le dessin ; moyeu d'hélice Éclair de 150 mm d'épaisseur | moyeu à x 0,78 (données) ou 0,83 (relecture) | dessin [LU-IMG] ; `spad7-calculs.md` § 3.1 ; V1 [LU] | C | plateau, boulons (8 selon un forum, V20 : D) |

## 3. Cellule

| Élément | Description | Cotes | Sources | Confiance | Inconnu, `ESTIMÉ` |
|---|---|---|---|---|---|
| Forme en plan | deux plans presque rectangulaires, extrémités droites et non inclinées, coins légèrement arrondis ; pas de flèche | envergures égales de 7,82 m ; corde de travail 1,21 m | S48 [LU-OCR] et dessin ; `spad7.md` § 2.1 ; `spad7-calculs.md` § 3 | C | rayon des coins |
| Découpe du plan supérieur | « profonde », au-dessus du siège ; en **demi-cercle** sur la vue de dessus du dessin | à mesurer | S48 [LU-OCR] ; dessin [LU-IMG] | C (forme) | **largeur et profondeur** : à mesurer sur le rendu à 600 dpi, puis sur photo |
| Plan inférieur | part du flanc du fuselage | 0,40 m de l'axe | `spad7-calculs.md` § 3 | C | forme de l'emplanture (le S.XI a des plans inférieurs échancrés près du fuselage, S49) |
| Ailerons | plan supérieur seulement, articulés sur un faux longeron | 1,95 à 1,98 m depuis le saumon ; 29 % de la corde | S48, S49 [LU-OCR] ; `spad7-calculs.md` § 3.1 | C | — |
| Nervures | S.XI : espacées de **190 mm** (supérieur) et **175 mm** (inférieur), avec de **fausses nervures** sur l'extrados, du bord d'attaque au longeron avant. Forum : nervures de compression plutôt que des entretoises | S.VII : non trouvées | S49 [LU-OCR] ; V19 [RÉSUMÉ] | C (famille) | **ESTIMÉ** : environ **20 nervures par demi-plan supérieur** (3,91 m / 0,19 m), à vérifier sur S.254 ou sur l'aile 2016.16.1 |
| Bord d'attaque | coffré ou entoilé : **non tranché** | — | `spad7.md` § 2.6 | — | — |
| Bord de fuite | S.XI : un **fil** forme le bord de fuite, la toile y est cousue ; la tension donne donc un bord de fuite festonné entre les nervures (déduction) | — | S49 [LU-OCR] | C | profondeur des festons |
| Entoilage | S.XI : toile clouée sur des nervures renforcées et cousue aux nervures et au fil de bord de fuite | — | S49 [LU-OCR] | C (famille) | pas des coutures |
| Rubans | d'abord en toile écrue enduite comme le reste ; plus tard noirs, bleu horizon, puis de la teinte voisine | largeur non trouvée | V17 [RÉSUMÉ], forum | D | — |
| Œillets | à l'intrados, près du bord de fuite, pour équilibrer la pression et drainer l'humidité (S.XI) ; présents sur S.254 | — | S49 [LU-OCR] ; V4 [RÉSUMÉ] | B (existence), C (position) | nombre et pas |
| Trappes de visite | **non trouvées** sur la voilure. Les ouvertures de l'aile 2016.16.1 ont été découpées après le service pour inspecter la structure : à ne pas modéliser | — | V3 [RÉSUMÉ] | — | — |
| Tôles d'aluminium | S.254 porte des tôles d'aluminium sur les ailes, décapées et repeintes en mat comme les capots | — | V2 ou V4 [RÉSUMÉ] (attribution entre les deux incertaine) | C | **position et forme** : à relever |
| Réservoir d'appoint | dans la partie centrale du plan supérieur | — | `spad7.md` S13 [RÉSUMÉ] ; S.XI : 10 L entre les longerons (S49) | C | trappe ou bouchon visibles : non trouvés |

## 4. Mâts et câblage

| Élément | Description | Cotes | Sources | Confiance | Inconnu, `ESTIMÉ` |
|---|---|---|---|---|---|
| Mâts interplans | par côté, un mât avant et un mât arrière, verticaux ; pas de décalage | avant à 17 %, arrière à 49 % de la corde | `spad7.md` § 2.3 ; `spad7-calculs.md` § 3 | B (disposition), C (positions) | **sections** et cordes |
| Mâts intermédiaires | au croisement des câbles ; une **entretoise en tube d'acier** relie l'intermédiaire avant à l'arrière | — | S48 [LU-OCR] ; S49 (« mâts intérieurs divisés ») | B | position en envergure à mesurer sur le dessin |
| Matériau des mâts | S.XI : bois profilé, ferrures fixées sur un tube d'acier logé dans le mât | — | S49 [LU-OCR] | C (famille) | finition : bois verni ou peint, **non trouvé** |
| Cabane | **non décrite** dans les sources lues ; le dessin est trop réduit. Une revue de maquette dit les mâts de cabane et de train « marouflés », entourés de toile collée | — | V14 [RÉSUMÉ] | D | nombre, forme, ferrures : à prendre sur photo |
| Câbles de vol | partent des flancs du fuselage vers les mâts extérieurs et se croisent aux mâts intermédiaires. S.XI : **doublés**, l'intervalle rempli de lattes de bois | — | S48 [LU-OCR] (S.VII) ; S49 [LU-OCR] (S.XI) | B (disposition), C (doublés) | type (rond ou profilé), diamètre |
| Câbles d'atterrissage | simples sur le S.XI | — | S49 [LU-OCR] | C | — |
| Câble de traînée | S.XI : de la jonction du mât intérieur au longeron supérieur avant vers l'attache de la jambe avant du train | — | S49 [LU-OCR] | C (famille) | présence sur le S.VII |
| Câbles d'incidence | **non trouvés** ; le dessin n'en montre pas entre les mâts avant et arrière | — | dessin [LU-IMG] | — | à vérifier sur photo |
| Tendeurs | **non trouvés** | — | — | — | **ESTIMÉ** : tendeurs à chaque extrémité de câble, petite géométrie |
| Renvois d'ailerons | bielles poussées-tirées logées dans le plan inférieur derrière le longeron arrière ; renvois au pied des mâts extérieurs ; biellettes verticales articulées sous l'aileron | — | S48 (déduction de l'auteur) ; S49 (S.XI) [LU-OCR] | B | cotes des renvois et des biellettes |
| Profondeur et gouvernail | S.XI : guignols **à l'intérieur** du fuselage et de la dérive ; profondeur en deux parties sur un tube d'acier. Donc aucun câble ni guignol visible à l'empennage | — | S49 [LU-OCR] ; S48 (empennage « habituel ») | C | — |

## 5. Fuselage

| Élément | Description | Cotes | Sources | Confiance | Inconnu, `ESTIMÉ` |
|---|---|---|---|---|---|
| Section | « exceptionnellement profond », fond arrondi sous les longerons inférieurs comme les flancs et le dessus | **ESTIMÉ** 0,80 × 1,10 m | S48 [LU-OCR] ; `spad7-calculs.md` § 3 | C (forme), D (cotes) | — |
| Structure | quatre longerons, dessus et dessous arrondis par des couples et des lisses (S.XI) ; avant tôlé, arrière entoilé | — | S49 [LU-OCR] ; V13 [RÉSUMÉ] | C | position de la limite tôle-toile |
| Poste | ouverture de 0,9 à 1,4 m derrière le bord d'attaque | — | `spad7-calculs.md` § 3 | C | forme de l'ouverture |
| Bourrelet | en cuir : les éléments en cuir du poste de S.254 sont des restitutions, ce qui suppose des originaux en cuir | — | V4 [RÉSUMÉ] | C | forme et section |
| Pare-brise | en **verre** : à la SPA 124, en 1917, les mécaniciens en polissent « le verre » avant la patrouille. S.XI : en trois pans, cadre d'aluminium. S.254 a gardé un pare-brise, cassé. Le prototype en avait un plus petit | — | S3, Hall [LU] ; S49 [LU-OCR] ; V4, V12 [RÉSUMÉ] | C | hauteur, largeur, nombre de pans sur le S.VII |
| Appui-tête | **non trouvé** | — | — | — | n'en pas modéliser sans photo |
| Marchepied | piste : le tuyau d'échappement gauche sert de marchepied (V18). à l'école, Hall « monte la marche » d'un chasseur de type non nommé | — | V18 [RÉSUMÉ] ; S3 [LU] | D | à vérifier sur photo |
| Trappes | **non trouvées** pour le S.VII (S.XI : une trappe dans le plancher derrière l'observateur) | — | S49 [LU-OCR] | — | — |
| Cloison moteur | S.XI : un rideau de toile sépare le moteur du siège | — | S49 [LU-OCR] | C | — |
| Carénage de mitrailleuse | sur des S.VII de construction britannique, un carénage prolongé vers l'arrière remplaçait le pare-brise ; retiré sur les avions du front | — | V13 [RÉSUMÉ] | C | ne concerne pas un avion français |

## 6. Train et béquille

| Élément | Description | Cotes | Sources | Confiance | Inconnu, `ESTIMÉ` |
|---|---|---|---|---|---|
| Jambes | fixes. S.XI : jambes en V en bois lamellé collé, entoilées ; croisillons de câbles dans les deux plans. Piste : jambes « marouflées » sur le S.VII | — | S49 [LU-OCR] ; V14 [RÉSUMÉ] | C | sections |
| Essieu | coulisse dans des glissières orientées vers le haut et l'arrière, dans l'axe des jambes arrière ; S.XI : deux demi-essieux entre deux tubes transversaux entoilés, débattement de 125 mm | voie 1,52 m (5 ft) | S48 [LU-OCR] ; S49 [LU-OCR] | C | carénage de l'essieu sur le S.VII : non trouvé |
| Amortisseurs | sandows (cordes de caoutchouc) | — | S48 [LU-OCR] | C | nombre de tours |
| Roues | diamètre dessiné 0,58 m ; sur la légende du dessin coté du S.XI, une dimension « 700 × … » illisible | — | `spad7-calculs.md` § 3 ; `spad7.md` § 2.5.1 | D | **ESTIMÉ** : roue de 700 mm, d'après la légende du S.XI (famille), en supposant que le premier nombre est le diamètre extérieur en mm (convention **à vérifier**) ; largeur du pneu inconnue (75 mm : simple hypothèse). Le dessin de 1917 (0,58 m) est plus petit : dessin de presse, D. Rayons, voiles de roue : **non trouvés**, à vérifier sur photo |
| Béquille | sous l'arrière du fuselage, inclinée vers l'arrière ; fixe sur le XIII | — | dessin [LU-IMG] ; `spad7.md` S15 [RÉSUMÉ] | C | cotes, ferrure, sabot |

## 7. Armement

| Élément | Description | Sources | Confiance | Inconnu |
|---|---|---|---|---|
| Mitrailleuse | **une** Vickers de 7,7 mm au-dessus du moteur, tirant à travers le champ de l'hélice | `spad7.md` § 7 (S5, S6) [RÉSUMÉ] | B | — |
| Synchronisation | Birkigt, commande par tige depuis l'arbre à cames droit (S.XI) | S49 [LU-OCR] ; V13 [RÉSUMÉ] | C | — |
| Détente | sur le manche (S.XI) | S49 [LU-OCR] | C | — |
| Position latérale | **non trouvée** (axe ou décalée à droite) | — | — | à lire sur photo de face ou de dessus |
| Viseur | à la SPA 124 en 1917 : « deux viseurs circulaires » à aligner (sans doute deux anneaux) | S3, Hall [LU] | C (un témoignage) | Aldis sur certains SPAD : piste pour le XIII seulement |
| Bande, goulottes | bande de cartouches visible sur un chasseur français de type non nommé, vu par Hall à l'école ; goulottes d'éjection des étuis et des maillons : **non trouvées** | S3 [LU] | — | à prendre sur photo |
| Fusées (option) | SPA 124, 1917 : six fusées sur des supports fixés aux mâts, trois par côté ; une tôle protège la toile du plan inférieur ; mise à feu par un bouton dans le poste | S3 [LU] | B pour la SPA 124 | équipement de mission, **pas** pour le modèle de base |

## 8. Hélice Éclair type 6

| Élément | Description | Cotes | Sources | Confiance | Inconnu, `ESTIMÉ` |
|---|---|---|---|---|---|
| Type | Éclair type 6, « approuvé » pour le Type VII à Hispano de 150 ch ; Éclair (types 6 et 100) présentée comme le fournisseur principal du S.VII | 2,40 × 1,95 m ; moyeu de 150 mm d'épaisseur | V1, tableau p. 19 et texte p. 13 [LU] ; choix du joueur du 2026-10-24 (`hispano-suiza-8a.md` § 7.5) | C | — |
| Bois | en France, le **noyer** est le plus employé ; 4 à 8 lattes de 5 à 20 mm collées sous presse ; épaisseur de 90 à 180 mm | — | V1, p. 9-10 [LU] (pratique générale) | C | essence et nombre de lattes d'une Éclair 6 |
| Finition | poncée, puis vernie ou laquée au pinceau | — | V1, p. 10 [LU] | C | teinte du vernis |
| Gaine du bord d'attaque | le « blindage » (tôle de cuivre, cuir ou toile collée) est cité pour les hélices **marines** des hydravions | — | V1, p. 8 [LU] | C | **ESTIMÉ** : pas de gaine sur l'hélice d'un chasseur terrestre |
| Forme de pale | une Éclair conservée (type non identifié) a une pale large à bout arrondi | — | V1, photo p. 13, collection R. Lemaire [LU-IMG] | C pour une Éclair, D pour le type 6 | **plan de pale de l'Éclair 6 : non trouvé**. Un forum donne 180 mm de largeur de pale (V20, D) |
| Fixation | 6 boulons chez Chauvière, 8 chez Ratmanoff ; 8 boulons pour une Éclair de S.VII selon un forum | — | V1, p. 9 [LU] ; V20 [RÉSUMÉ] | D | — |
| Marque | logo rond Éclair, à anneau vert et hélice blanche au centre (photo, collection R. Lemaire) | — | V1, p. 13 [LU-IMG] | C | **marque déposée** (le 19 juin 1916 selon un article) : ne pas reproduire le logo, un décalque générique suffit |
| Sens de rotation | horaire vu du pilote | — | `spad7.md` § 6.5.3 | B | — |

## 9. Poste de pilotage (premier niveau)

**Aucune source ne donne la planche de bord d'un S.VII de 1917.** Le poste de S.254 est vide d'instruments (V4). La seule description détaillée d'un poste de SPAD est celle du biplace S.XI capturé (S49). Elle sert de base, en **C**, avec les témoignages de pilotes.

| Élément | Description | Sources | Confiance | Inconnu |
|---|---|---|---|---|
| Siège | très étroit « comme sur tous les SPAD » ; réservoir principal sous le siège (sur le S.XI, le réservoir sous pression forme le siège) | S49 [LU-OCR] ; `spad7.md` S13 [RÉSUMÉ] | C | forme, garniture |
| Harnais | bretelles d'épaules | S3, Hall, en piqué sur S.VII [LU] | C | — |
| Manche | détente de la mitrailleuse sur le manche | S49 [LU-OCR] ; S3 [LU] (« control-stick ») | C | forme de la poignée |
| Palonnier | barre | S3, Hall [LU] (« rudder-bar ») ; S49 | C | — |
| Planche de bord | en contreplaqué de **cinq plis** sur un S.VII d'origine (n° 11 583, livré à la Tchécoslovaquie après la guerre) | V11 [RÉSUMÉ] | C | contour, instruments |

**Instruments et commandes attestés** :

| Instrument ou commande | Position sur le S.XI (S49) | Autres sources | Confiance pour le S.VII |
|---|---|---|---|
| Compte-tours | au centre, **cadran à plat**, lu à l'envers par le pilote (critique du rapport allemand) | Hall (chasseur non nommé, vu à l'école) et Bordeaux (Guynemer, 1915) [LU] | B pour la présence ; position non trouvée |
| Manomètre (pression d'essence) | au centre | — | C |
| Deux contacts (magnétos) | au centre | — | C |
| Robinet trois voies : réservoir sous pression, pompe à main ou pompe moteur | au centre | piste : interrupteurs des pompes et des réservoirs principal et auxiliaire (V14) | C |
| Robinet d'essence vers le carburateur | au centre | — | C |
| Robinet d'isolement de la pompe moteur | au centre | — | C |
| Magnéto de départ (« starter ») et pompe à air à main | à droite | — | C |
| Manette des gaz, correcteur | à gauche | — | C |
| Levier des volets de radiateur | à gauche | — | sans objet si le radiateur n'a pas de volets (§ 2) |
| Avance à l'allumage | **absente** | — | C |
| Altimètre | — | Hall, Bordeaux [LU] | C |
| Compas | — | Hall, Bordeaux [LU] | C |
| Porte-cartes à rouleau | — | Hall, Bordeaux [LU] | C |
| Indicateur de vitesse | — | Bordeaux cite un *speedometer* à côté du *tachometer*, mais la même traduction emploie *speedometer* pour un régime (« 1800 », `spad7.md` T1) : **ambigu** | non trouvé |
| Montre | — | **non trouvée** | — |
| Thermomètres d'huile et d'eau ; jauge d'essence à flotteur ou à réglette | — | guide de maquette (V14) | D |

**Disposition de travail pour le M2 (ESTIMÉ)** : reprendre la répartition du S.XI (à droite la magnéto de départ et la pompe à main ; au centre les contacts, les robinets, le manomètre et le compte-tours ; à gauche la manette des gaz et le correcteur), et ajouter l'altimètre et le compas sur la planche. Justification : seule disposition décrite pour un SPAD de 1917, même constructeur, même moteur Hispano. Instruments en décalques génériques, sans copie d'un cadran de marque.

## 10. Matériaux et finitions

| Matériau | Description | Sources | Confiance | `ESTIMÉ` |
|---|---|---|---|---|
| Toile, finition d'usine non camouflée | « blanc jaunâtre » sur le S.XI de juillet 1917 ; « jaune » verni, avec les parties métalliques d'un jaune plus foncé, sur les S.VII de 1916-1917 | S49 [LU-OCR] ; V15, V16 [RÉSUMÉ] ; `spad7.md` § 8 | C | teinte de travail : crème jaune pâle. **Aucune référence de couleur trouvée** |
| Toile vue à contre-jour | structure visible à travers la toile enduite | — | — | effet attendu du rendu (plan B5), pas une source |
| Enduit d'époque | non trouvé. La restauration de S.254 a employé un enduit de tension cellulosique, **pigmenté** pour masquer les lacunes : ses teintes actuelles ne sont pas un témoin fiable | V4 [RÉSUMÉ] | — | — |
| Métal | capot en aluminium peint (§ 2) ; ferrures en acier sur les mâts (S.XI) | S48, S49 [LU-OCR] | C | acier peint ou brut : non trouvé |
| Bois | mâts en bois profilé ; jambes de train en bois lamellé entoilé ; hélice en noyer vernie | S49 [LU-OCR] ; V1 [LU] | C | — |
| Cuir | bourrelet du poste | V4 [RÉSUMÉ] | C | teinte |
| Verre | pare-brise | S3 [LU] | C | — |
| Usure en service | avions d'occasion « décolorés par la fumée et l'huile », fuselage et ailes couverts de pièces sur d'anciennes blessures (SPA 124, été 1917) | S3, Hall [LU] | C | traînées d'échappement et d'huile, rapiéçages : niveau d'usure à décider |

## 11. Appareils conservés et références photographiques

| Appareil | Où | État, intérêt | Droits des images |
|---|---|---|---|
| **S.254 « Vieux Charles »** (Guynemer, SPA 3) | Musée de l'Air et de l'Espace, Le Bourget ; en réserve (fuselage photographié dans les réserves le 9 avril 2025 selon V10) | appareil d'origine restauré : toile neuve posée sous la toile d'origine, enduit pigmenté ; capots et tôles décapés et repeints ; ni moteur, ni instruments ; cuir restitué ; Vickers rapportée (V2, V4) | photos du musée et des photographes d'airhistory.net : droits réservés, référence seulement |
| **Aile de SPAD VII**, inv. 2016.16.1 (plan inférieur droit) | Musée de l'Air et de l'Espace | extrados au camouflage beige, marron et vert ; intrados de teinte aluminium, « hors des règlements connus » selon le musée ; réparation de combat avec croix allemandes et date manuscrite ; découpes d'inspection d'après-guerre (V3) | référence ; **meilleur témoin des teintes réelles** |
| Planche de bord d'un S.VII n° 11 583 | Vojenský historický ústav, Prague | planche d'origine en contreplaqué cinq plis, conservée en réserve ; une réplique la remplace sur l'avion (V11) | référence |
| B9916 | San Diego Air & Space Museum | construit en 1917 par Mann, Egerton & Co (Norwich), « 95 % d'origine » selon le musée, restauré à partir de 1990 (V24) | type de construction britannique : panneau moteur plein (§ 2) |
| AS 94099 (identifiant selon une page de *This Day in Aviation*) | National Museum of the US Air Force, Dayton | restauré de 1962 à 1966 par le 1st Fighter Wing (V25) | photos de l'US Air Force : en principe du domaine public américain, **à vérifier** pour chaque image |
| S.2489 (Baracca) | Museo Francesco Baracca, Lugo | restauré de 1990-1991 à 1993 par le GAVS de Turin (V26) | référence |
| S.153 | Museo storico dell'Aeronautica, Vigna di Valle | `spad7.md` S27 | référence |
| SPAD VII n° 115 (réplique) | Conservatoire de l'air et de l'espace d'Aquitaine | réplique de 1989 (Dakar), refaite de 2016 à 2018 comme copie du n° 115 de Guynemer, poste reconstitué d'après des plans d'époque et des photos de S.254 (V21) | **pas un original** : indice seulement |
| B9913 | Seattle, 1919 (photo CAVM-17652) ; un « B9913 » ultérieur serait une réplique | identité de l'appareil exposé au Canada : **contradictoire** (V31) | — |

**Photos d'époque datées de S.254** (V5, V7, V8, V9, V10, V29) :
- deux plaques d'Albert Moreau (Section photographique de l'armée), cour des Invalides, **20 octobre 1917** ;
- « avions du capitaine Guynemer fleuri », cote **SPA 236 M 4636**, même photographe, datée du 20 novembre 1917 par la notice : peut-être une erreur de catalogue (Bordeaux place la visite fleurie le 1er novembre, S1 [LU]) ;
- Australian War Memorial, **C63172** (titre seulement vu) ;
- Roger-Viollet, cliché publié par *Excelsior* le 20 octobre 1917 ;
- Albin Denis, SPA 3 page 2 : Guynemer décollant sur S.254 **fin mai 1917** ;
- Museum of Flight, item 29160 : Guynemer assis dans un SPAD VII, licence **CC BY-NC-ND**.

Les photos ECPAD sont en noir et blanc : elles donnent formes, positions et contrastes, **pas les couleurs**.

## 12. Livrée (B2)

### 12.1 Règles de 1917 (France)

| Point | Ce qui est établi | Sources | Confiance |
|---|---|---|---|
| Finition d'usine 1916-début 1917 | toile vernie jaune pâle, métal peint d'un jaune plus foncé (« période jaune ») | V15, V16 [RÉSUMÉ] ; S49 | C |
| Camouflage | quatre ou cinq teintes sur le dessus et les flancs : écru (ou beige, ocre), vert clair, vert foncé, marron (châtaigne), et noir dans les schémas à cinq teintes. Un schéma par constructeur, fourni par la Section technique | V15, V17 [RÉSUMÉ] ; aile 2016.16.1 (beige, marron, vert, V3) | C |
| Date d'introduction | **contestée** : « 1917 » (spécification officielle, selon un wiki) ; **octobre 1917** pour le passage de la période jaune aux cinq teintes (Wings of Linen) | V17 [RÉSUMÉ] | D |
| Dessous des avions camouflés | écru ou jaune clair, parfois bleu clair ; l'aluminium de l'aile 2016.16.1 est hors règlement | V17, V3 [RÉSUMÉ] | C |
| Peintures | enduits camouflants de Nauton frères et de Marsac (brevet du chimiste Tesse), pigments analysés par « Project Butterfly » ; peinture à l'huile approchante sur les tôles | V17, forum [RÉSUMÉ] | D |
| Références de couleur | **non trouvées** (ni nuancier d'époque, ni équivalent moderne vérifié) | — | — |
| Cocardes | bleu au centre, blanc, rouge à l'extérieur | V32 [RÉSUMÉ] ; récit de restauration d'un musée vu en résumé (page non identifiée) | B |
| Emplacement et diamètre des cocardes | **non trouvés** pour le S.VII | — | **ESTIMÉ** (§ 12.2) |
| Gouvernail | bandes verticales bleu, blanc, rouge ; bleu côté charnière (convention connue, **non sourcée ici**) | — | C |
| Numéro de série | peint sur le gouvernail (S.254 : « S 254 », piste V22). Le S.XI portait aussi sur le gouvernail une inscription de charges (poids utile 255 kg, poids de combustible 120 kg) | S49 [LU-OCR] ; V22 [RÉSUMÉ] | C |
| Insigne de la SPA 3 | cigogne en vol, ailes **basses** ; les autres escadrilles du GC 12 ont d'autres cigognes ; peinte sur les flancs du fuselage | `spad7.md` § 8 (S36, S20) ; V5 | C |
| Marques personnelles | numéro et nom peints en grand (Guynemer : « 2 » et « Vieux Charles ») ; à la SPA 124, tête d'Indien sur les flancs et marque personnelle (triangle, losange, bande, initiale) | V5 [RÉSUMÉ] ; S3, Hall [LU] | B |

### 12.2 Candidat 1 — S.254 « Vieux Charles », Georges Guynemer, N 3 puis SPA 3

| Point | Établi | Sources | Confiance | `ESTIMÉ` ou à vérifier |
|---|---|---|---|---|
| Avion | SPAD VII n° **S 254**, pièces produites en 1916-1917 par les établissements Blériot (SPAD) | V6 [RÉSUMÉ] | B | — |
| Moteur | **Hispano « surcomprimé » de 180 ch** : premier exemplaire ainsi équipé, reçu fin décembre 1916 | V2, V10 [RÉSUMÉ] | **B** | **Écart avec le modèle de vol de 150 ch** (§ 12.5) |
| Service | reçu fin décembre 1916 ; endommagé à l'atterrissage sur sol gelé le 24 janvier 1917, revenu le 4 février ; piloté de février à juillet 1917 ; légende ECPAD : de décembre 1916 à juin 1917 | V2, V6, V7, V10 [RÉSUMÉ] | B | — |
| Flandres, été 1917 | resté à **Saint-Pol-sur-Mer**, terrain des Cigognes, puis démonté le 19 octobre 1917 et envoyé à Paris. Le 20 août 1917, Guynemer vole « sur son vieux Vieux-Charles » (Bordeaux) | V7 [RÉSUMÉ] ; S1 [LU] | C (une seule notice, vue en résumé) | que l'avion du 20 août soit S.254 : **non établi** (S.504 selon d'autres, `spad7.md` § 1.4) |
| Exposition | cour d'honneur des Invalides le 20 octobre 1917 ; visité et fleuri le 1er novembre 1917 | V5, V7, V9 [RÉSUMÉ] ; S1 [LU] | B | — |
| Victoires | 16 homologuées (musées) ; 19 dont 18 homologuées (autre notice) ; « dix-neuf » chez Bordeaux | V2, V6 ; V33 ; S1 [LU] | — | divergence sans effet sur la livrée |
| Finition | **non établie**. Livré avant les camouflages : finition jaune d'usine probable | § 12.1 | C | à vérifier sur les plaques de 1917 (contrastes) et sur l'avion (sous la restauration) |
| Insigne | cigogne de l'escadrille n° 3 sur le fuselage | V5 [RÉSUMÉ] | B | **rouge** et « ailes déployées » selon trois pistes (peintre, site d'amateur, notice de vente d'une maquette, V22) : C. Forme exacte et position : à relever |
| Numéro | « 2 », numéro personnel de Guynemer à l'escadrille | V5 [RÉSUMÉ] ; V22 | B | rouge (V22, C) ; position et taille à relever |
| Nom | « Vieux Charles » peint sur le fuselage | V5 [RÉSUMÉ] | B | couleur, graphie, côté : à relever. Udet se souvient de lettres noires « Vieux » (récit de 1935, classé légende, `spad7.md` T12) |
| Gouvernail | tricolore, inscription « S 254 » | V22 (notice de vente) | D | à vérifier sur photo |
| Cocardes | présentes | V22 | C | **ESTIMÉ, pour le gabarit seulement** : sur les ailes, diamètre d'environ 1,0 m (il doit tenir dans la corde de 1,21 m), centre à environ un diamètre du saumon ; présence sur le fuselage inconnue. À remplacer par la mesure sur photo |
| Hélice | **non établie** : Hartmann compte Guynemer parmi les pilotes qui préféraient les hélices Chauvière ; une notice Wikipédia cite l'Éclair « dont celui de Guynemer » | V1, p. 8 [LU] ; `hispano-suiza-8a.md` H35 [RÉSUMÉ] | — | contradiction ; l'Éclair 6 du modèle reste un choix par défaut |

### 12.3 Candidat 2 — S.113, code « 11 », lieutenant Alfred Heurtaux, N 3

| Point | Établi | Sources | Confiance | `ESTIMÉ` ou à vérifier |
|---|---|---|---|---|
| Avion et pilote | SPAD VII **S 113**, codé « 11 », du lieutenant Heurtaux ; photo d'une collection familiale | V10 [RÉSUMÉ] | C | **contradiction** : une autre page attribue S.113 à Guynemer (V28) |
| Moteur | série de la première commande, donc **150 ch** probable | `spad7.md` § 1.1 | C | — |
| Pilote | Heurtaux reçoit un SPAD VII à la fin de 1916 ; 21 victoires au 4 mai 1917 ; gravement blessé en septembre 1917 | V27 [RÉSUMÉ] ; `spad7.md` S55, S68 | B | — |
| Date | **inconnue** : rien ne prouve que S.113 volait encore en 1917 | — | — | — |
| Livrée | « 11 », cigogne ; finition jaune d'usine probable | V10 ; § 12.1 | C | presque tout le reste |

### 12.4 Candidat 3 — S.2489, Francesco Baracca, 91a Squadriglia

| Point | Établi | Sources | Confiance | `ESTIMÉ` ou à vérifier |
|---|---|---|---|---|
| Avion | SPAD VII **S 2489**, construit en 1917, **conservé** au musée Baracca de Lugo | V26 [RÉSUMÉ] | B | — |
| Service | 91a Squadriglia, de la fin de 1917 au début de 1918 ; 30e victoire de Baracca le 7 décembre 1917 | V26 [RÉSUMÉ] | C | — |
| Restauration | 1990-1993, GAVS de Turin | V26 [RÉSUMÉ] | B | dossier non consulté |
| Livrée | cheval cabré noir, emblème personnel de Baracca, sur le flanc du fuselage | V26 [RÉSUMÉ] | B | finition, cocardes italiennes, gouvernail : à relever |
| Moteur | **non trouvé** | — | — | — |
| Contexte de jeu | front italien, fin 1917 : hors du secteur du jeu (SPA 3, Flandres) | — | — | — |

### 12.5 Recommandation

**Pour un premier modèle : S.254, dans l'état photographié le 20 octobre 1917.**
- **Pourquoi** :
  - seul S.VII à la fois **conservé** (avec réserves de restauration), photographié à une **date sûre** (plaques ECPAD) et documenté par deux musées ;
  - c'est un avion de la **SPA 3**, resté à Saint-Pol-sur-Mer, en **Flandre**, jusqu'en octobre 1917 selon une notice ECPAD (C) : le contexte du jeu ;
  - son état d'octobre 1917 serait donc celui de la fin de son service en Flandre. Des réparations entre juillet et octobre restent possibles (Bordeaux le dit endommagé lors d'une prise d'armes de l'été 1917).
- **Réserves à trancher par le joueur** :
  1. **Moteur** : S.254 avait le 180 ch, pas le 150 ch du modèle de vol. De l'extérieur, la différence ne se voit pas, sauf peut-être des persiennes ou des trous de capot à vérifier. La fiche principale retient d'ailleurs le 180 ch pour la SPA 3 de l'été 1917 (`spad7.md` § 1.4, journal du 2026-10-20).
  2. **Personnage réel** : c'est l'avion personnel de Guynemer. Je propose de l'employer pour la scène de studio, pour un as piloté par l'IA, ou comme livrée historique au choix. L'avion du joueur pourrait garder la même finition d'usine avec un numéro fictif.
  3. **Couleurs** : les photos de 1917 sont en noir et blanc, et l'avion a été repeint en partie. Teinte de la toile, couleur de la cigogne, du « 2 » et du nom : à confirmer par la demande D8.
- **Couches de décalques proposées** :
  1. finition d'usine de 1917, non camouflée, avec cocardes et gouvernail (couche de base, utile à tous les S.VII de ce lot) ;
  2. marques de S.254 (cigogne, « 2 », « Vieux Charles », « S 254 »).
- **Si le joueur tient à un 150 ch strict** : S.113 de Heurtaux, mais presque tout y serait `ESTIMÉ`. Je ne le recommande pas pour un premier modèle.
- **Pour une variante camouflée plus tard** : l'aile 2016.16.1 donne les teintes réelles, mais aucun S.VII camouflé de la SPA 3 en 1917 n'a été identifié.

**Anachronismes à éviter sur ce modèle** :
- les **neuf volets verticaux** de fin de série sur un avion de début 1917 ;
- un **camouflage à cinq teintes** sur S.254 ;
- la **bande tricolore des as** : une piste la place sur S.115 en 1916, rien ne la met sur S.254 ;
- une cigogne d'une autre escadrille du GC 12 ;
- **deux Vickers** ou le grand radiateur du SPAD XIII.

## 13. Témoignages sur l'aspect (français, anglais, allemand, italien)

| Témoin | Camp, date | Ce qu'il montre de l'aspect | Statut | Biais |
|---|---|---|---|---|
| James Norman Hall, *High Adventure* (S3) | volontaire américain, SPA 124, été 1917 | SPAD alignés aile contre aile ; avions d'occasion rapiécés, décolorés par la fumée et l'huile ; tête d'Indien et marque personnelle peintes en grand ; pare-brise en verre ; deux viseurs circulaires ; supports de fusées sur les mâts avec tôle de protection ; palonnier à barre, bretelles d'épaules | [LU] | récit pour l'*Atlantic Monthly* ; S.VII à moteur refait |
| Hall, même livre, à l'école de pilotage | élève pilote, avant son affectation (1916-1917) | chasseur français de type non nommé : Vickers sur le capot avec sa bande, insigne d'escadrille sur le fuselage, marche d'accès, siège rembourré, compas, altimètre, compte-tours, carte sur rouleau | [LU] | type d'avion inconnu (sans doute un Nieuport) |
| Henry Bordeaux, *Guynemer* (S1) | biographe, 1918 | instruments que Guynemer vérifiait (1915) ; le *Vieux-Charles*, endommagé le jour même, aligné pour une prise d'armes à l'été 1917 ; l'avion aux dix-neuf victoires visité aux Invalides le 1er novembre 1917 | [LU] | hagiographie ; traduction anglaise |
| Rapport allemand sur le S.XI (S49) | Allemagne, 1918 | poste étroit, instruments « assez mal disposés », compte-tours à plat lu à l'envers, pare-brise en trois pans, toile peinte « blanc jaunâtre » | [LU-OCR] | autre type ; regard technique, sans affection |
| Ernst Udet, *Mein Fliegerleben* | Allemagne, 1935 | lettres noires « Vieux » sur le fuselage du SPAD de Guynemer | [RÉSUMÉ], légende (`spad7.md` T12) | souvenir tardif, épisode contesté |
| Sources italiennes | Italie | Baracca peint le cheval cabré noir de sa famille sur ses avions | [RÉSUMÉ] (V26) | sources commémoratives |

Recherches menées sans résultat utile sur l'aspect : en allemand (examen d'un S.VII capturé : rien), en italien (livrée de S.2489 : rien de détaillé), en tchèque (instruments de la planche de Prague : rien de détaillé), en russe (volets et échappements : V12 seulement).

## 14. Écarts avec `spad7.md` (reportés le 2026-10-26)

1. **§ 1.2, moteur de S.254** : la fiche le dit « non vérifié ». Le Musée de l'Air (V2) et Albin Denis (V10) le donnent comme le premier exemplaire à moteur surcomprimé de **180 ch**, reçu fin décembre 1916 : **B**.
2. **§ 1.5, S.254 en Flandre** : une notice ECPAD (V7) place S.254 à Saint-Pol-sur-Mer jusqu'au 19 octobre 1917. La mise en garde reste juste pour le 20 août et le 11 septembre, mais S.254 semble bien avoir suivi la SPA 3 en Flandre (C).
3. **§ 1.4, Heurtaux** : un SPAD VII S.113 codé « 11 » lui est attribué (V10) ; une autre page attribue S.113 à Guynemer (V28). Contradiction à noter.
4. **§ 1.4, date de l'appellation « SPA 3 »** : un résumé de recherche sur S.254 (sans doute V5) la place en **octobre 1917**, ce qui fait une troisième date après janvier 1917 (S18) et « après mars 1917 » (S20). Le même résumé date la cigogne de l'escadrille de **juillet 1916** ; Albin Denis (V10) dit « vers le milieu de 1916 ». À vérifier avant tout texte de jeu.
5. **`hispano-suiza-8a.md` § 7.5** : Hartmann (V1, p. 8 [LU]) range Guynemer parmi les pilotes qui préféraient les hélices **Chauvière**. Cela nuance la mention de l'Éclair « dont celui de Guynemer » (H35).

## 15. Documents à télécharger ou à faire rendre (session principale)

| Priorité | Document | Adresse ou cote | Ce qu'on y cherche |
|---|---|---|---|
| 1 | Rendu à 600 dpi du dessin de *Flight* du 16 août 1917, p. 841 (déjà dans le dossier de travail) | `Flight_International_Magazine_1917-08-16-pdf` | découpe du plan supérieur (largeur, profondeur) ; fin du tuyau d'échappement ; mâts de cabane ; position en envergure des mâts intermédiaires ; diamètre de roue ; position de la Vickers sur la vue de face |
| 1 | Notices et vignettes ImagesDéfense de S.254 | https://imagesdefense.gouv.fr/fr/l-avion-de-guynemer-dans-la-cour-des-invalides.html ; https://imagesdefense.gouv.fr/fr/paris-aux-invalides-avions-du-capitaine-guynemer-fleuri-legende-d-origine.html | positions et tailles de la cigogne, du « 2 », du nom, des cocardes et du numéro ; capot ; échappement ; hélice ; conditions de réutilisation |
| 1 | Albin Denis, escadrille SPA 3, pages 1 à 4 | http://albindenis.free.fr/Site_escadrille/escadrille003.htm ; `…escadrille003_02.htm` ; `…escadrille003_04.htm` | photos légendées de S.254 (fin mai 1917), S.113, S.420 « Ma Renée » ; état des appareils de la SPA 3 |
| 1 | Musée de l'Air, pages « Spad VII » et « Aile de Spad VII » | https://www.museeairespace.fr/aller-plus-haut/collections/spad-vii/ ; https://www.museeairespace.fr/aller-plus-haut/collections/aile-de-spad-vii/ | texte intégral des notices (V2, V3) |
| 1 | Notice Google Arts & Culture de S.254 | https://artsandculture.google.com/asset/avion-spad-vii-le-vieux-charles-spad-manufacturer/3gGCxL6d9cc0og | texte intégral (V4) : tôles, capots, pare-brise, œillets |
| 2 | Musée de l'Armée, deux billets | V5, V6 (§ 16) | légendes des photos des Invalides, récolement |
| 2 | Australian War Memorial C63172 | https://www.awm.gov.au/collection/C63172 | date, droits (beaucoup de photos de l'AWM sont libres) |
| 2 | Planche de bord de Prague | V11 (§ 16) | photos des instruments |
| 2 | Hartmann, p. 19-20, déjà rendues | dossier de travail | rien de neuf ; p. 13 déjà vue |
| 3 | SDASM sur Flickr Commons (« no known copyright restrictions ») | https://commons.flickr.org/tags/spadvii/ | photos d'époque de S.VII, possiblement réutilisables après vérification |

## 16. Sources

Sources de `spad7.md` citées ici : S1, S3 (lues), S13, S15, S27, S36, S20 (résumés), S48, S49 (OCR). Leur référence complète est au § 9 de `spad7.md`. Sources nouvelles, consultées le 2026-10-26 :

- **V1** — G. Hartmann, « Les héliciers français », PDF de 23 p., hydroretro.net, 2008, https://www.hydroretro.net/etudegh/heliciersfrancais.pdf. Copie locale du dossier de travail (`sources/hartmann/`). **[LU]** p. 7 à 13 et annexe p. 19-20 ; **[LU-IMG]** p. 13 (photo et logo d'une hélice Éclair, collection René Lemaire). Secondaire. Même document que `spad7.md` S78 et `hispano-suiza-8a.md` H9.
- **V2** — Musée de l'Air et de l'Espace, « Spad VII », https://www.museeairespace.fr/aller-plus-haut/collections/spad-vii/ [RÉSUMÉ]. Institutionnelle. (= `spad7.md` S7, relu pour S.254.)
- **V3** — Musée de l'Air et de l'Espace, « Aile de Spad VII », inv. 2016.16.1, https://www.museeairespace.fr/aller-plus-haut/collections/aile-de-spad-vii/ [RÉSUMÉ]. Institutionnelle.
- **V4** — Google Arts & Culture, « Avion Spad VII "Le Vieux Charles" », https://artsandculture.google.com/asset/avion-spad-vii-le-vieux-charles-spad-manufacturer/3gGCxL6d9cc0og [RÉSUMÉ]. Notice muséale (restauration, état du poste). Certains détails (capots, tôles, Vickers) viennent d'une « fiche de collection » que le résumé n'attribue pas clairement à V2 ou à V4.
- **V5** — Musée de l'Armée, « Les Invalides pendant la Grande Guerre, 21e épisode : le "Vieux Charles" », https://actualites.musee-armee.fr/expositions-en/les-invalides-during-the-great-war-twenty-first-episode-the-vieux-charles/?lang=en [RÉSUMÉ]. Institutionnelle.
- **V6** — Musée de l'Armée, blog des collections, « Sur les traces d'un as de la Grande Guerre, le récolement au musée de l'Air et de l'Espace », https://collections.musee-armee.fr/sur-les-traces-dun-as-de-la-premiere-guerre-mondiale-le-recolement-au-musee-de-lair-et-de-lespace/ [RÉSUMÉ]. Institutionnelle.
- **V7** — ImagesDéfense (ECPAD), notices « L'avion de Guynemer dans la cour des Invalides » (A. Moreau, deux négatifs sur verre, 20/10/1917) et « Paris, aux Invalides, avions du capitaine Guynemer fleuri » (SPA 236 M 4636, datée du 20/11/1917) ; adresses au § 15 [RÉSUMÉ]. Primaires (photos d'époque), non vues.
- **V8** — Australian War Memorial, C63172, « Invalides, Paris. Spad 7 "Vieux Charles" of Capt Guynemer on display in the court of honour… », https://www.awm.gov.au/collection/C63172 [RÉSUMÉ], titre seulement.
- **V9** — Roger-Viollet, « World War One. The "Vieux-Charles", plane used by Georges… », https://www.galerie-roger-viollet.fr/en/photo-world-war-one-the-vieux-charles-plane-used-by-1026079-4267010578 [RÉSUMÉ] : cliché publié par *Excelsior* le 20 octobre 1917.
- **V10** — Albin Denis, « L'escadrille SPA 3 », pages 2 et 4, http://albindenis.free.fr/Site_escadrille/escadrille003_02.htm et http://albindenis.free.fr/Site_escadrille/escadrille003_04.htm [RÉSUMÉ]. Secondaire (historique d'escadrille illustré, sources citées par photo).
- **V11** — Vojenský historický ústav (Prague), « Palubní deska letounu Spad VII C.I, výr. č. 11 583 », https://www.vhu.cz/exhibit/palubni-deska-letounu-spad-vii-c-i-vyr-c-11-583/, et « 06 Instrument Panel from French SPAD S.VII.C.1 Aircraft, 1916 », https://vhu.cz/en/exhibit/06-francouzska-palubni-deska-z-letounu-spad-s-vii-c-1-1916 [RÉSUMÉ]. Institutionnelle. La notice attribue au 8Aa une puissance de 180 ch, ce qui contredit `spad7.md` § 1.2 (8Aa de 150 ch, 8Ab de 180 ch) : à ne pas reprendre.
- **V12** — flyingmachines.ru, « SPAD S.VII (Spa 7C-1) », https://flyingmachines.ru/Site2/Crafts/Craft25466.htm [RÉSUMÉ]. Compilation en russe de textes et de légendes de photos (= `spad7.md` S12). Piste.
- **V13** — Wikipédia (en), « SPAD S.VII », https://en.wikipedia.org/wiki/SPAD_S.VII [RÉSUMÉ] (= `spad7.md` S5). Wiki : piste.
- **V14** — Revues de maquettes de la Roden au 1/32 : T. Cleaver, https://modelingmadness.com/review/w1/tc/tmcs7.htm ; iModeler, https://imodeler.com/2017/04/roden-132-spad-vii-c-1/ ; guide de montage cité par un résumé (page non identifiée). [RÉSUMÉ]. Pistes.
- **V15** — B. Pearson, « SPAD VII Colours & Markings – A Brief Look », CBRNP, http://www.cbrnp.com/profiles/quarter1/spad7.htm [RÉSUMÉ]. Secondaire d'amateur informé : piste.
- **V16** — Wings Palette, « SPAD S.VII – Great Britain », http://wp.scn.ru/en/ww1/f/257/9/1 [RÉSUMÉ]. Profils d'après J. M. Bruce : piste.
- **V17** — Aeroscale, fil « French Multi-Colour Camouflage 101 », https://archive.aeroscale.net/forums/90429-page=3.htm ; Wings of Linen, « French Five-Color Camouflage », https://linen.miraheze.org/wiki/French_Five-Color_Camouflage ; Military History wiki, « Aircraft camouflage », https://military-history.fandom.com/wiki/Aircraft_camouflage [RÉSUMÉ]. Forum et wikis : pistes.
- **V18** — all-aero.com, « SPAD V / VII / Swanson SPAD S.VII », https://all-aero.com/2025/04/14/spad-v-vii-swanson-spad-s-vii/ [RÉSUMÉ]. Vulgarisation : piste.
- **V19** — The Aerodrome, fil « Spad VII », https://theaerodrome.com/forum/threads/spad-vii.32454/ [RÉSUMÉ]. Forum : piste.
- **V20** — Aviation ancienne (forum), « recherche infos sur hélice Hs 150 P », https://aviation-ancienne.forumactif.com/t6589-recherche-infos-sur-helice-hs-150-p [RÉSUMÉ]. Forum : piste.
- **V21** — CAEA, « SPAD VII », https://www.caea.fr/avions/spad-vii/ ; « Les SPAD de G. Guynemer », https://daniel99725.wixsite.com/spad-7-115-caea/les-spad-de-g-guynemer ; Aerobuzz, « La renaissance d'un SPAD VII construit… à Dakar », https://www.aerobuzz.fr/culture-aero/la-renaissance-dun-spad-vii-construit-a-dakar/ [RÉSUMÉ]. Réplique (rang 3) et pages associées.
- **V22** — Pistes sur les marques de S.254 : Locris (notes d'une peinture), https://www.locris.org/product-page/s-vii-spad ; « As oubliés de 14-18 », fiche Guynemer, https://asoublies1418.fr/index.php?Itemid=287&catid=99&id=1735&option=com_content&view=article ; Interenchères, lot 88550381 (maquette en bois), https://www.interencheres.com/en-US/art-decoration/vente-de-maquettes-modeles-reduits-et-jouets-anciens-et-modernes-2eme-partie-687144/lot-88550381.html [RÉSUMÉ]. Pistes, jamais seules sources d'un fait.
- **V23** — HistoryNet, « Georges Guynemer: France's World War I Ace Pilot », https://www.historynet.com/georges-guynemer-frances-world-war-i-ace-pilot.htm [RÉSUMÉ] (extrait d'Udet).
- **V24** — San Diego Air & Space Museum, « Artifact Spotlight: The SPAD VII », https://sandiegoairandspace.org/newsletters/article/artifact-spotlight-the-spad-vii [RÉSUMÉ] (= `spad7.md` S14).
- **V25** — National Museum of the US Air Force, « SPAD VII », https://www.nationalmuseum.af.mil/Visit/Museum-Exhibits/Fact-Sheets/Display/Article/197407/spad-vii/ ; This Day in Aviation, tag « a s 94099 », https://www.thisdayinaviation.com/tag/a-s-94099/ [RÉSUMÉ].
- **V26** — Ministère italien de la Culture, « Casa museo F. Baracca », https://cultura.gov.it/luogo/casa-museo-f-baracca ; ASI Musei, https://asimusei.it/museo/museo-francesco-baracca/ ; SMBR, « Lugo, primo museo Baracca », https://www.smbr.it/lugo-primo-museo-baracca/ [RÉSUMÉ].
- **V27** — Musée de l'Armée, « Une vie consacrée, épisode 15 : Alfred Heurtaux », https://actualites.musee-armee.fr/expositions-en/a-dedicated-life-episode-15-alfred-heurtaux/?lang=en [RÉSUMÉ].
- **V28** — HistoryOfWar.org, « SPAD VII and SPAD XIII », https://www.historyofwar.org/articles/weapons_spadVII.html [RÉSUMÉ]. Vulgarisation.
- **V29** — Museum of Flight, « [Captain Georges Guynemer seated in SPAD VII (S.7) aircraft] », https://digitalcollections.museumofflight.org/items/show/29160 [RÉSUMÉ]. Licence CC BY-NC-ND.
- **V30** — Passion pour l'aviation, « Guynemer le spadassin », https://www.passionpourlaviation.fr/2016/03/19/guynemer-le-spadassin/ [RÉSUMÉ] : restauration de S.254 par Philippe Osché en 1981-1982 ; S.132 avant S.254.
- **V31** — Ingenium, CAVM-17652 (B9913 à Seattle, 23 avril 1919), https://collection.ingenium.ca/en/id/CAVM-17652 ; ABPic, https://abpic.co.uk/pictures/view/1297813 [RÉSUMÉ].
- **V32** — techno-science.net, « Cocarde », https://www.techno-science.net/fr/definitions/cocarde [RÉSUMÉ] : cocarde obligatoire en 1912, bleu, blanc et rouge du centre vers l'extérieur. Vulgarisation.
- **V33** — AirHistory.net, photos de S.254 au Bourget n° 52994 (2005), 47039 (2010) et 436341 (2015), https://airhistory.net/photo/52994/254, https://www.airhistory.net/photo/47039/S254, https://airhistory.net/photo/436341/254 [RÉSUMÉ]. Légendes de photographes (dont « 19 victoires, dont 18 homologuées ») ; photos sous droits.

**Pistes non consultées, prioritaires** : J. M. Bruce, *Spad 7.C1*, Windsock Datafile n° 8 (`spad7.md` P1) et *SPAD Scouts S.VII-S.XIII*, Aircam Aviation Series n° 9 ; M. Velek, M. Ovčáčík et K. Susa, *SPAD S.VII C1*, 4+ Publications n° 19 (P3) ; Hardesty, Hastings et Toelle, « Project Butterfly », *Cross & Cockade* (US) vol. 9 n° 1 (1968), vol. 13 n° 2 et 4 (1972) (P14) ; C. Ravel, « La restauration du Spad VII n° 5248 », *Cahiers du RSA* n° 255, 2007 (appareil non identifié).

## 17. Journal des décisions

- **2026-10-26 — Capot peint, pas brut.** Trois pistes concordantes (V14, V15, V16) donnent des parties métalliques peintes d'un jaune plus foncé que la toile ; aucune source ne parle d'aluminium brut ; la teinte d'origine de S.254 est perdue (V4). **Retenu : peint, C.**
- **2026-10-26 — Radiateur sans volets.** Les volets verticaux ne viennent qu'en fin de série (V12, V13, V14). Pour un 150 ch du début de 1917 : **ouverture d'origine, nid d'abeilles nu, C**. Le diamètre de 0,71 m du dessin reste en D.
- **2026-10-26 — Échappement.** Disposition retenue d'après le dessin de 1917 et le S.XI (C). La longueur diverge entre le dessin et une piste de maquette : **non tranchée**, à lire sur photo.
- **2026-10-26 — Nervures.** Aucune cote du S.VII. **ESTIMÉ** : pas de 190 mm du S.XI, soit environ 20 nervures par demi-plan supérieur. À contrôler sur S.254 ou sur l'aile 2016.16.1.
- **2026-10-26 — Poste.** Aucune planche de S.VII de 1917 décrite. **ESTIMÉ** : disposition du S.XI (S49), complétée par l'altimètre et le compas (Hall, Bordeaux).
- **2026-10-26 — Livrée.** Recommandé : **S.254 tel que photographié le 20 octobre 1917**. Seul S.VII réunissant avion conservé, photos datées et notices de deux musées, et présent en Flandre avec la SPA 3. Réserves : moteur de 180 ch, personnage réel, couleurs à confirmer. S.113 (Heurtaux, 150 ch probable) et S.2489 (Baracca, conservé) restent des variantes, moins documentées. **Décision au joueur.**
- **2026-10-26 — Cocardes.** Emplacements et diamètres non trouvés. **ESTIMÉ** de gabarit seulement (environ 1,0 m sur les ailes), à remplacer par une mesure sur photo.
- **2026-10-26 — Décision du joueur sur la livrée.** **S.254 « Vieux Charles »** dans son état du 20 octobre 1917, en deux couches de décalques : finition d'usine, puis marques de Guynemer. Le moteur de 180 ch est accepté, la différence étant invisible de l'extérieur. **Usage réservé** : scène de studio, as piloté par l'IA ou livrée historique au choix. L'avion du joueur garde la finition d'usine avec un numéro fictif.
