# SPAD S.VII C.1 (150 ch) — fiche d'aspect : modèle 3D et livrée

- Identifiant : `spad7`. Fiches liées : [`spad7.md`](spad7.md) (histoire, géométrie, performances), [`spad7-calculs.md`](spad7-calculs.md) (cotes lues sur le dessin de 1917, § 3 et § 3.1), [`hispano-suiza-8a.md`](hispano-suiza-8a.md) (hélice, § 7.5).
- Dernière mise à jour : 2026-10-27 (§ 12.6 : marques de S.254)
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

### 12.6 Marques de S.254 pour la seconde couche (passe du 2026-10-27)

**Objet.** La cigogne, le « 2 », « Vieux Charles », les inscriptions du gouvernail et les autres marques de S.254 dans son état du 20 octobre 1917, pour les peindre dans la texture du modèle (couche 2 du § 12.5). La couche 1 (`usine-1917` de `livrees.ron`) n'est pas reprise ici.

**Accès.** Le 2026-10-27, l'outil de lecture échoue encore (DNS) sur albindenis.free.fr et daniel99725.wixsite.com ; je n'ai pas tenté d'autres sites. Tout ce qui suit vient de **résumés du moteur de recherche** ([RÉSUMÉ]) et des fiches déjà lues. **Aucune image n'a été vue** : aucune position ni aucune taille n'a pu être mesurée. Rien n'est reproduit ici, ni dessin, ni profil, ni photo.

**Bilan.**
- **Établi** : la présence des trois marques le 20 octobre 1917 (**B**) et la posture « ailes basses » de la cigogne (**B**).
- **Probable** : le rouge de la cigogne et du « 2 » (**C**).
- **Fragile** : le noir du nom, qui ne repose que sur Udet (**D**).
- **Non sourcé** : positions, tailles, graphies et sens de vol. La proposition du § 12.6.9 est donc un gabarit `ESTIMÉ` (D), à remplacer par une mesure sur photo ou sur l'avion.

#### 12.6.1 Ce que disent les notices des photos datées

| Photo | Date | Ce que la notice dit des marques | Statut | Fiabilité, usage |
|---|---|---|---|---|
| Musée de l'Armée, gros plan publié dans le 21e épisode de « Les Invalides dans la Grande Guerre » (V5) | billet consacré à l'exposition du 20/10/1917 ; auteur et date du cliché non vus | on y voit le nom « Vieux Charles », la cigogne en vol (emblème de l'escadrille n° 3 depuis juillet 1916) et le chiffre « 2 », numéro personnel de Guynemer. Selon la version anglaise du billet, la queue porterait aussi la bande tricolore des as français (§ 12.6.5) | [RÉSUMÉ] | institutionnelle ; **seule notice qui décrit les marques**, mais sans position, taille ni couleur |
| ECPAD, A. Moreau, deux plaques (V7) | 20/10/1917 | titre seulement | [RÉSUMÉ] | primaire, non vue (D9) |
| ECPAD, SPA 236 M 4636 (V7) | 20/11/1917 selon la notice (1er novembre chez Bordeaux) | avion fleuri | [RÉSUMÉ] | primaire, non vue |
| Roger-Viollet, cliché d'*Excelsior* (V9) | publié le 20/10/1917 | titre seulement | [RÉSUMÉ] | primaire, droits réservés |
| AWM C63172 (V8) | 1917 | titre seulement | [RÉSUMÉ] | primaire |
| Gallica, Agence Rol, « L'avion de Guynemer aux Invalides » (V40) | 1917 | titre seulement | [PISTE] | primaire ; **nouvelle référence**, à télécharger |
| Argonnaute (La Contemporaine), « Paris. Invalides. L'avion le "Vieux Charles" du capitaine Guynemer » (V41) | 1917 probablement | titre seulement | [PISTE] | primaire ; **nouvelle référence** |
| Albin Denis, SPA 3 p. 2 (V10) | fin mai 1917 | Guynemer décolle sur S.254 | [RÉSUMÉ] | état antérieur de cinq mois |
| Albin Denis, SPA 3 p. 4 (V10) | photos de l'avion conservé ; relevé des marques publié en avril 2025 selon un résumé | S.254 codé « 2 » ; l'inscription peinte du gouvernail n'aurait pas été mise à jour au passage au 180 ch, elle donnerait donc encore le moteur d'origine | [RÉSUMÉ] | secondaire ; **à lire en priorité** (domaine bloqué) |
| Museum of Flight 29160 (V29) | « vers 1914-1917 » | Guynemer assis dans un SPAD VII ; le nom est peint sur le fuselage | [RÉSUMÉ] | avion non identifié ; CC BY-NC-ND |
| Biblioteca Virtual de Defensa, registro 163170 (V42) | légende au dos : la Somme, après la 16e victoire | Guynemer près de son avion « Vienx-Charles » (sic) | [RÉSUMÉ] | **pas S.254** : la 16e victoire date de 1916 (25 victoires à la fin de 1916 selon un résumé de recherche), avant la livraison de S.254 fin décembre 1916. Utile seulement pour comparer la graphie du nom sur un avion antérieur |

**Ce qu'on peut affirmer** : le 20 octobre 1917, S.254 portait le nom, la cigogne en vol et le « 2 » (**B** : V5 ; le nom et la cigogne sont confirmés par la brochure du ministère, V35). **Ce qu'on ne peut pas affirmer** : leur emplacement, leur taille, leur couleur, le flanc photographié, ni ce qu'est la « bande tricolore ».

#### 12.6.2 Cigogne de la SPA 3

| Point | Établi | Sources | Confiance | `ESTIMÉ` ou à vérifier |
|---|---|---|---|---|
| Présence le 20/10/1917 | oui, sur le fuselage | V5, V35 [RÉSUMÉ] | B | — |
| Posture | en vol, **ailes basses**, au bas du battement : c'est l'appellation même de l'insigne, « Cigogne de Guynemer (ailes basses) » | V36 (titre de la monographie de F. Blech, 1986) ; S36 (lettre de 1976) ; S20, V47 (wikis) | B | — |
| Origine | pochoir dessiné par le capitaine Brocard et l'adjudant Charles Borzecki, applicable en plusieurs couleurs selon la teinte de la toile. Le même pochoir servait donc à tous les avions de l'escadrille | V34 (J. Guttman) [RÉSUMÉ] | C | Divergences : Wikipédia (en) place Borzecki à la C 43 puis à la N 62, pas à la N 3 ; un autre récit attribue le dessin à un peintre du camouflage (V44). Sans effet sur la forme |
| Date | emblème de l'escadrille n° 3 depuis juillet 1916. Les autres escadrilles du GC 12 adoptent des cigognes en novembre 1916 | V5 ; V34 ; `spad7.md` § 1.4 | C | — |
| Flancs | les deux : un article parle de « la cigogne ornant ses flancs ». Plus généralement, l'insigne était peint ou passé au pochoir sur le flanc du fuselage | V30 [RÉSUMÉ] ; V34 ; S20 | C | aucune photo des deux flancs n'est décrite |
| Couleur sur S.254 | **rouge**. Trois pistes le disent, dont une page qui cite les ouvrages d'Osché (restaurateur de S.254) et la revue *Icare*. C'est cohérent avec la règle du pochoir : couleur sombre sur toile claire | V21, V22 [RÉSUMÉ] ; V34 | C | D'autres couleurs sont signalées, mais sur **d'autres** avions ou pour l'insigne en général : blanche à ailes noires sur le SPAD XIII S.504, camouflé (légende de photo, compilation flyingmachines.ru, piste) ; noire et blanche, bec et pattes rouges (Western Front Association, article de 2008, piste) ; « rouge et bleu » sur les Nieuport de 1916 (HistoryNet, piste). Teinte exacte inconnue. **ESTIMÉ** : le rouge des cocardes (`rouge` de `livrees.ron`) |
| Contour, détails | **non trouvés** | — | — | **ESTIMÉ** (D) : aplat d'une seule couleur, sans œil ni plumes dessinés, comme le donne un pochoir. Seules les rémiges écartées du bout des ailes découpent la silhouette |
| Sens de vol | **non trouvé** | — | — | **ESTIMÉ** (D) : vers le nez sur les deux flancs, le pochoir étant retourné pour le flanc droit. Justification : sens du déplacement de l'avion, pochoir réversible |
| Position le long du fuselage | **non trouvée** | — | — | **ESTIMÉ** (D) : sur la partie entoilée, entre le poste et l'empennage, derrière la fin du tuyau d'échappement (x −1,67) |
| Taille | **non trouvée** | — | — | **ESTIMÉ** (D) : environ 1,0 m de long et 0,48 m de haut, un peu plus de la moitié de la hauteur du flanc à cet endroit. À mesurer sur S.254 (D8) |

**Distinguer la cigogne de la SPA 3 de celles du GC 12** :

| Escadrille | Appellation | Posture | Sources | Confiance |
|---|---|---|---|---|
| **SPA 3** | « Cigogne de Guynemer » | **ailes basses** : fin du battement, ailes entièrement sous la ligne du dos, pointes vers le bas | V36, S36, S20, V47 | B |
| SPA 103 | « Cigogne de Fonck » | ailes hautes : début du battement, ailes au-dessus du dos | S36, V47 | C |
| SPA 26 | « Cigogne de Saint-Galmier » | **divergent** : ailes à mi-course (S36) ; ailes repliées, en piqué (V47, notice de vente) | S36, V47 | D |
| SPA 73 | « Cigogne japonaise » | dessin de style japonais ; tête retournée vers la queue selon S36 ; « en virage » selon V47 | S36, V47 | D |

Règle de contrôle pour le dessin : une cigogne dont les ailes passent au-dessus du dos, restent à l'horizontale, ou dont la tête regarde en arrière, **n'est pas** celle de la SPA 3.

**Mascotte Hispano-Suiza.** Selon la lettre de 1976 (S36), la cigogne de la SPA 3 a servi de modèle à la mascotte de radiateur sculptée par François Bazin vers 1919-1920. Les notices de musée et de vente divergent sur cette origine (V48). Ne pas s'en servir comme gabarit : c'est une sculpture, pas l'insigne peint, et elle est probablement encore protégée (sculpteur mort en 1956).

#### 12.6.3 Le « 2 »

| Point | Établi | Sources | Confiance | `ESTIMÉ` |
|---|---|---|---|---|
| Présence le 20/10/1917 | oui. C'est le numéro personnel de Guynemer à l'escadrille, où chaque pilote avait le sien ; un forum le dit attribué à l'ancienneté | V5 [RÉSUMÉ] ; forum (piste) | B (présence) | — |
| Couleur | rouge | V21, V22 [RÉSUMÉ] | C | teinte `rouge` (D) |
| Flancs | non trouvé. Selon un résumé de Wikipédia, le numéro se peignait sur le dessus et sur les deux flancs du fuselage | V47 [RÉSUMÉ] | D | **ESTIMÉ** : les deux flancs. Pas de « 2 » sur le dessus du fuselage tant qu'aucune photo ne le montre |
| Plan supérieur | rien trouvé | — | — | **ESTIMÉ** : pas de « 2 » sur les ailes |
| Position | non trouvée. Le gros plan du Musée de l'Armée montre le « 2 » avec le nom et la cigogne : même flanc, sans doute pas très loin | V5 | C (même flanc) | **ESTIMÉ** (D) : en arrière de la cigogne. Autre possibilité : entre le poste et la cigogne |
| Taille, graphie | non trouvées | — | — | **ESTIMÉ** (D) : environ 0,40 m de haut ; chiffre bâton droit, sans empattement, trait d'environ 15 % de la hauteur |

#### 12.6.4 « Vieux Charles »

| Point | Établi | Sources | Confiance | `ESTIMÉ` |
|---|---|---|---|---|
| Présence le 20/10/1917 | oui, sur le fuselage | V5, V35 [RÉSUMÉ] ; V29 (date incertaine) | B | — |
| Origine du nom | hérité du Morane-Saulnier L de Charles Bonnard ; porté par la plupart des avions de Guynemer, dont le Nieuport 11 N836, où il était écrit « Le Vieux Charles » | `spad7.md` § 8 ; V23 [RÉSUMÉ] | B | Forme exacte sur S.254 **non vue** : « Vieux Charles », « Vieux-Charles » (Bordeaux) ou « Le Vieux Charles » (Nieuport). **ESTIMÉ** : « VIEUX CHARLES », sans article, comme dans les notices des musées |
| Couleur | lettres **noires** | Udet, *Mein Fliegerleben* (`spad7.md` S29), passage repris par V46 [RÉSUMÉ] | D | souvenir publié en 1935, épisode contesté, témoin unique |
| Position | « sur le fuselage, entre les ailes » | même passage d'Udet | D | **ESTIMÉ** : flanc avant, entre les plans (x 0 à −1,21) |
| Côtés | non trouvés | — | — | **ESTIMÉ** : les deux flancs (D) |
| Graphie | non trouvée : rien sur les capitales, la cursive ou les empattements | — | — | **ESTIMÉ** (D) : capitales bâton peintes à la main, sur une ligne. Udet ne lit que « Vieux » : nom sur deux lignes, ou partie cachée ? Non tranché |
| Taille | non trouvée | — | — | **ESTIMÉ** : capitales d'environ 0,12 m (D) |

**Réputation et faits.** Le combat qu'Udet raconte (juin 1917, près de Lierval) n'a pas de trace dans les sources françaises citées (`spad7.md` T12). Il est précieux pour le jeu, comme récit de mess, mais pas comme preuve d'aspect. Un résumé de forum lui fait aussi décrire un SPAD « brunâtre » : **non retenu**, car non vérifié et contraire à la finition d'usine supposée de S.254.

#### 12.6.5 Gouvernail : « S 254 » et inscriptions

| Point | Établi | Sources | Confiance | `ESTIMÉ` |
|---|---|---|---|---|
| Bandes | verticales bleu, blanc, rouge, le bleu à la charnière (couche 1) ; largeurs égales selon un forum | § 12.1 ; forum (piste) | C | — |
| Inscription peinte | oui. Selon Albin Denis, elle n'a pas été mise à jour au passage au 180 ch et donnerait donc encore la version d'origine | V10 [RÉSUMÉ] | C | Texte exact inconnu. Sur le S.XI, le gouvernail porte une inscription de charges (poids utile, poids de combustible) : S49 [LU-OCR], famille, C |
| « S 254 » | numéro sur le gouvernail | V22 (notice d'une maquette vendue aux enchères) | D | **ESTIMÉ** (D) : en noir sur la bande blanche ; écrit « S 254 » avec une espace, comme dans les notices |
| Faces | non trouvées | — | — | **ESTIMÉ** : les deux faces (D) |
| Taille | non trouvée | — | — | **ESTIMÉ** : chiffres de 0,045 m, pour tenir dans la bande blanche de 0,18 m (D) |

**« Bande tricolore des as » : contradiction ouverte.** Selon la version anglaise du billet du Musée de l'Armée (V5), la queue de S.254 porte la bande tricolore des as français. Deux lectures sont possibles :
- (a) le tricolore réglementaire du gouvernail, commun à tous les avions français, et donc mal qualifié par le billet ;
- (b) une bande tricolore distincte, comme celles qu'on signale sur S.115 en 1916 (pour être reconnu de l'artillerie, selon Albin Denis cité par un miroir de Wikipédia) et sur S.113 (piste).

La lecture (b) contredirait l'anachronisme listé au § 12.5. **Non tranché** : pas de bande supplémentaire par défaut. C'est le **premier point à vérifier** sur les plaques du 20 octobre (D9, V40, V41).

#### 12.6.6 Autres marques d'octobre 1917

| Marque | Établi | Confiance | Proposition |
|---|---|---|---|
| Cocardes de fuselage | aucune source ne les mentionne sur S.254 ; les notices du 20 octobre ne citent que le nom, la cigogne et le « 2 » | — | **ESTIMÉ** : aucune (D) |
| Cocardes d'ailes | couche 1 | — | inchangées |
| Bandes de chef de patrouille, fanions | rien sur S.254. Une revue de maquette signale, sur une planche de décalques Airfix, un fanion rouge sur un carénage d'appui-tête : piste unique, et le § 5 ne trouve pas d'appui-tête | D | rien |
| Inscriptions du constructeur | gouvernail seulement (§ 12.6.5) ; rien d'autre trouvé | — | rien d'autre |
| Code d'unité (« N 3 », « SPA 3 ») | rien trouvé | — | rien |
| Réparations | dégâts le 24/01/1917 (atterrissage sur sol gelé) et à l'été 1917 (jour d'une prise d'armes, Bordeaux) ; aucune description de pièces ni de marques repeintes | C (dégâts) | **ESTIMÉ** : pas de rapiéçage particulier ; usure générale du § 10 (D) |
| Démontage | démonté à Saint-Pol le 19/10/1917, remonté aux Invalides (V7) | C | rien de visible attendu |

#### 12.6.7 L'avion conservé, témoin des marques

- **Restauration de 1981-1982** (Philippe Osché y a participé, V30). Selon un dossier de presse du musée, la toile d'origine, très abîmée, a été **doublée** à la manière d'un tableau. Un résumé ajoute que les marques et la cigogne ont été laissées en l'état ; l'attribution entre les deux documents de V39 est incertaine. V4 parle d'une toile neuve posée sous la toile d'origine et d'un enduit pigmenté pour masquer les lacunes.
- **Conséquence** : les marques visibles aujourd'hui sont **probablement** la peinture de 1917 sur la toile d'origine, retouchée aux lacunes.
  - Leurs **positions et tailles** se mesureraient sur l'avion : **A** une fois mesurées.
  - Leurs **couleurs** restent douteuses (vieillissement, intempéries dans un escalier des Invalides, retouches) : **C** au mieux.
- **2016** : le rapport d'activité signale un constat d'état et une étude de conservation (cinq restaurateurs), et un **fragment de toile portant une cigogne**, venu d'un autre avion (V39 [RÉSUMÉ]). C'est un témoin possible de la couleur et du dessin du pochoir.
- **Propriété** : l'avion est un **dépôt du Musée de l'Armée** au Musée de l'Air (V6, V39). La demande D8 doit donc associer le Musée de l'Armée (D14).

#### 12.6.8 Gabarit de la cigogne pour le dessin vectoriel (`ESTIMÉ`, D)

Ce gabarit ne copie aucune image. Il est construit à partir de deux éléments :
- la posture « ailes basses » (B) ;
- la silhouette en vol de la cigogne blanche (V45) : cou tendu vers l'avant et un peu tombant, longues pattes qui dépassent nettement une queue courte, bec long et droit ; longueur du bec à la queue de 95 à 125 cm selon les fiches, envergure de 155 à 217 cm.

Il permet d'avancer et sera remplacé par un redessin d'après les photos de 1917 (D9, V40, V41) ou d'après l'avion (D8). Comme l'insigne était un pochoir commun à l'escadrille (V34, C), une photo datée d'un autre avion de la N 3 ou de la SPA 3 de 1916-1917 peut aussi servir de référence pour la forme.

**Repère** : vue de profil, oiseau volant vers +u (vers le nez de l'avion) ; u horizontal, v vertical vers le haut ; unité L = longueur hors tout, de la pointe du bec au bout des doigts.

| Élément | Gabarit (u ; v, en fractions de L) | Justification |
|---|---|---|
| Boîte englobante | u de 0 à 1 ; v de 0 à 0,48 | ailes pendantes : presque toute la hauteur vient des ailes |
| Bec | droit, de la base (0,88 ; 0,405) à la pointe (1,00 ; 0,385) ; épaisseur 0,02 à la base ; incliné d'environ 9° vers le bas | bec ≈ 15 % de la longueur bec-queue (D) ; cou tombant (V45) |
| Tête | ovale centré en (0,86 ; 0,405), de 0,045 × 0,030 | petite tête (D) |
| Cou | tendu, de la tête à l'épaule (0,63 ; 0,44) ; épaisseur 0,025 à la tête, 0,045 à l'épaule ; descend d'environ 8° vers la tête | cou tendu en vol, et non replié en S comme chez le héron (V45) |
| Corps | fuseau de u 0,25 à 0,63 (0,38 L), épaisseur maximale 0,10 vers u 0,50 ; axe horizontal à v 0,42 ; le dos (v 0,47) est le point le plus haut | corps ≈ 45 % de la longueur bec-queue (D) |
| Queue | courte, en coin, de u 0,25 à 0,18 ; 0,035 de haut au bout | queue courte (V45) |
| Pattes | deux traits droits et parallèles de 0,010, écartés de 0,008, du dessous du corps (0,30 ; 0,38) aux doigts (0,00 ; 0,35) ; elles dépassent la queue de 0,18 L | pattes dépassant nettement la queue (V45) ; environ 22 % de la longueur bec-queue (D) |
| Aile proche | attache sur le haut du corps, de u 0,44 à 0,60 (corde 0,16). Bord d'attaque de (0,60 ; 0,46) à la pointe (0,46 ; 0,00) ; bord de fuite de (0,44 ; 0,44) à (0,40 ; 0,04). Axe incliné d'environ 75° sous l'horizontale, pointe un peu en arrière de l'attache ; cinq rémiges écartées sur le dernier 0,12 | bas du battement (B pour la posture, D pour les cotes). Longueur visible ≈ 0,46 L, raccourcie par la perspective : la demi-envergure réelle vaut 0,7 à 0,95 fois la longueur bec-queue (V45) |
| Aile éloignée | même forme à 85 %, décalée de 0,06 vers l'arrière, pointe en (0,36 ; 0,06) ; seul son bord arrière dépasse de l'aile proche et du corps | vue de profil (D) |
| Rendu | aplat d'une couleur, sans œil ni plumes ; pas de pont de pochoir à prévoir (aucune zone fermée) | pochoir (V34, C) |

Contrôles avant d'adopter le dessin : les deux pointes d'ailes sont les points les plus bas ; le dos est le point le plus haut ; la tête regarde vers l'avant (règle du § 12.6.2).

#### 12.6.9 Proposition chiffrée pour le modeleur

**Repère** : axes corps du modèle (x vers l'avant depuis le datum, y vers la droite, z vers le bas), en mètres ; sections du fuselage de `maquette.ron`. La valeur de y est celle du flanc (demi-largeur de la section), pour une projection selon y. **Toutes les cotes sont `ESTIMÉ` (D)**, sauf mention contraire.

| Marque | Côté(s) | Centre (x ; y ; z) | Dimensions | Couleur | Confiance |
|---|---|---|---|---|---|
| Cigogne | deux flancs (C) ; vole vers le nez, flanc gauche en miroir du droit (D) | (−2,30 ; ±0,34 ; −0,44) | gabarit du § 12.6.8 avec L = 1,00 m (x −1,80 à −2,80) ; hauteur 0,48 m (z −0,68 à −0,20) | rouge (C) ; teinte `rouge` (D) | présence B, posture B ; position, taille et sens D |
| « 2 » | deux flancs (D) | (−3,35 ; ±0,24 ; −0,45) | hauteur 0,40 m (z −0,65 à −0,25), largeur 0,26 m, trait 0,06 m ; droit, perpendiculaire à l'axe x | rouge (C) ; `rouge` (D) | présence B ; le reste D |
| « VIEUX CHARLES » | deux flancs (D) | (−0,72 ; ±0,48 ; −0,27) : sous le tuyau d'échappement (z −0,49), au-dessus de l'emplanture du plan inférieur | capitales de 0,12 m (z −0,33 à −0,21) ; longueur 0,95 m (x −0,25 à −1,20) ; une ligne | noir (D) ; couleur `noir` à créer, sRGB ≈ (0,09 ; 0,08 ; 0,07) (D) | présence B ; noir et « entre les ailes » D (Udet) ; le reste D |
| Variante du nom | deux flancs (D) | (−1,05 ; ±0,48 ; −0,62) : sous le rebord du poste, au-dessus du tuyau | capitales de 0,09 m ; longueur 0,75 m | noir (D) | D ; à choisir sur photo |
| « S 254 » | deux faces du gouvernail (D) | (−4,975 ; 0 ; −1,00) : milieu de la bande blanche (x −4,883 à −5,067) | chiffres de 0,045 m ; longueur 0,15 m ; horizontal | noir (D) | existence d'une inscription C ; texte et place D |
| Inscription d'usine (type, moteur, charges) | deux faces du gouvernail (D) | (−4,975 ; 0 ; −0,88) | bloc de 0,15 × 0,08 m : trois lignes de 0,015 m, illisibles à distance de jeu | noir (D) | existence C ; contenu D (« 150 HP » probable, V10) |
| Bandes du gouvernail (rappel de la couche 1) | deux faces | bleu de x −4,70 à −4,883 ; blanc de −4,883 à −5,067 ; rouge de −5,067 à −5,25 | trois tiers égaux de la corde de 0,55 m | couche 1 | C (largeurs égales) |
| Cocardes de fuselage | — | aucune | — | — | D |
| Bande tricolore « des as » | — | **non peinte** tant que les plaques du 20/10/1917 n'ont pas été vues | — | — | contradiction ouverte (§ 12.6.5) |
| « 2 » sur le dessus du fuselage ou des ailes | — | non peint | — | — | D |

**Contrôles de place** (C, d'après les sections de `maquette.ron`) :
- à x −2,30, le flanc va de z −0,83 à +0,03 : la cigogne laisse au moins 0,13 m en haut (jusqu'à x −2,55, bout du dos) et 0,23 m en bas ;
- à x −3,35, le flanc va de z −0,755 à −0,14 : le « 2 » laisse 0,10 m en haut et en bas ;
- le bec de la cigogne (x −1,80) reste 0,13 m derrière la fin du tuyau d'échappement (x −1,67) ;
- le nom laisse 0,13 m sous le tuyau et environ 0,1 m au-dessus de l'emplanture du plan inférieur.

**Couleur à ajouter** dans `livrees.ron` par la session principale : `noir`, pour le nom et les inscriptions du gouvernail (ESTIMÉ, aucune référence de couleur).

#### 12.6.10 À vérifier, par ordre d'utilité

1. **Photos de 1917** : plaques du 20/10/1917 (D9) et photos de presse Gallica (V40) et Argonnaute (V41), à télécharger par la session principale. On y cherche le flanc visible, les positions et tailles (en proportion de la hauteur du fuselage), la bande tricolore et la graphie du nom.
2. **Albin Denis**, SPA 3 p. 2 et p. 4 (V10) : relevé des marques de l'avion conservé.
3. **Ouvrages** : Klaeylé et Osché 1998 (V37), *Icare* n° 122 (V38), Blech 1986 (V36) : demande D13.
4. **L'avion lui-même** : mesures sur S.254 et fragment de toile à cigogne (D8, complété) ; gros plan du Musée de l'Armée (D14).
5. **Photos de l'avion conservé** sur Wikimedia Commons (V43) : licence à vérifier sur chaque page.

Domaines à autoriser pour ces lectures : `albindenis.free.fr`, `daniel99725.wixsite.com`, `gallica.bnf.fr`, `argonnaute.parisnanterre.fr`, `bibliotheques-numeriques.defense.gouv.fr`, `commons.wikimedia.org`, `imagesdefense.gouv.fr`, `actualites.musee-armee.fr`, `museeairespace.fr`.

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

Sources de `spad7.md` citées ici : S1, S3 (lues), S13, S15, S27, S29, S36, S20 (résumés), S48, S49 (OCR). Leur référence complète est au § 9 de `spad7.md`. Sources nouvelles, consultées le 2026-10-26 :

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

Sources nouvelles du § 12.6, consultées le 2026-10-27 (toutes vues en résumé de recherche ou par leur seule notice ; aucune page n'a pu être ouverte) :

- **V34** — J. Guttman, « How The French Added Fashion To World War I Airplanes », HistoryNet, https://historynet.com/how-the-french-added-fashion-to-world-war-i-airplanes/ [RÉSUMÉ]. Secondaire (auteur spécialiste, article de vulgarisation) : pochoir de Brocard et Borzecki, couleur choisie selon la toile ; insigne d'escadrille sur le flanc et chiffre arabe pour chaque pilote ; cigognes des autres escadrilles du GC 12 après novembre 1916.
- **V35** — Ministère de la Défense, SGA/DMPA, *Georges Guynemer*, brochure de la collection « Chemins de mémoire », 2007, https://www.cheminsdememoire.gouv.fr/sites/default/files/editeur/BrochureGuynemerSGADMPA2007.pdf [RÉSUMÉ]. Institutionnelle : le fuselage porte le nom et la cigogne.
- **V36** — F. Blech, *Spa 3 : « Cigogne de Guynemer » (ailes basses)*, Paris, F. Blech, 1986, 50 p. multigraphiées ; notice des Bibliothèques numériques de la Défense, https://bibliotheques-numeriques.defense.gouv.fr/impression/document/85943af0-c2e1-4c74-84e5-11c6c88b9749 [RÉSUMÉ de la notice ; contenu : PISTE].
- **V37** — B. Klaeylé et P. Osché, avec la participation de C. Cony, *Guynemer : les avions d'un as*, Lela Presse, coll. « Histoire de l'aviation » (n° 6 ou 7 selon les notices), 1998, 208 p., ISBN 2-9509485-6-1 ; notice BnF, https://catalogue.bnf.fr/ark:/12148/cb39247352j [PISTE]. Osché a participé à la restauration de S.254 (V30). Un second titre, P. Osché, *Les avions de Guynemer* (32 p., Ouest-France selon une fiche de libraire, https://livre.fnac.com/a276085/Philippe-Osche-Les-Avions-de-Guynemer), est à confirmer.
- **V38** — *Icare*, n° 122, « Guynemer et les Cigognes », SNPL, 1987, environ 226 p. ; références : bibliographie de l'*International Encyclopedia of the First World War*, https://encyclopedia.1914-1918-online.net/bibliography/t5gd8qmw/, et fiche de libraire, https://www.windsockdatafilespecials.co.uk/icare-guynemer-special-2164-p.asp [PISTE].
- **V39** — Musée de l'Air et de l'Espace, dossier de presse « Ouverture de la Grande Galerie », 25 octobre 2019, https://www.museeairespace.fr/wp-content/uploads/sites/2/2019/10/Basse-def-DPMuseeAirEspace-A4-25-octobre-web-final.pdf, et *Rapport d'activité 2016*, https://www.museeairespace.fr/wp-content/uploads/sites/2/2017/11/rapport-activite-2016-musee-air-et-espace.pdf [RÉSUMÉ ; répartition des faits entre les deux documents incertaine]. Institutionnelle : toile d'origine doublée, constat d'état de 2016, fragment de toile à cigogne, dépôt du Musée de l'Armée.
- **V40** — Gallica (BnF), Agence Rol, « L'avion de Guynemer aux Invalides », photographie de presse, 1917, https://gallica.bnf.fr/ark:/12148/btv1b53003839j [titre seulement]. Primaire ; conditions de réutilisation de Gallica à vérifier (référence seulement).
- **V41** — La Contemporaine (Université Paris Nanterre), Argonnaute, « Paris. Invalides. L'avion le "Vieux Charles" du capitaine Guynemer », https://argonnaute.parisnanterre.fr/ark:/14707/sr8z7x41vfm6 [titre seulement]. Primaire.
- **V42** — Biblioteca Virtual de Defensa (Espagne), « El Teniente Guynemer junto a su avión "Vienx-Charles" », photographie de 13 × 17,5 cm, registro 163170, https://bibliotecavirtual.defensa.gob.es/BVMDefensa/gran_guerra/en/consulta/registro.do?id=163170 [RÉSUMÉ]. Primaire ; avion antérieur à S.254.
- **V43** — Wikimedia Commons, photos de S.254 au Bourget, venues de Flickr : « Georges Guynemer's SPAD VII, Musee de l'Air et de l'Espace, Le Bourget, Paris. (8122848723).jpg », https://commons.wikimedia.org/wiki/File:Georges_Guynemer's_SPAD_VII,_Musee_de_l'Air_et_de_l'Espace,_Le_Bourget,_Paris._(8122848723).jpg ; « Cockpit detail… (8122869610).jpg » ; « Cowling and propellor detail… (8122873870).jpg » [titres seulement]. Licence à vérifier sur chaque page.
- **V44** — « Escadrille des cigognes et le capitaine Guynemer », PDF, juin 2016, site croixdeguerre-valeurmilitaire.fr, https://croixdeguerre-valeurmilitaire.fr/wp-content/uploads/2018/10/Escadrille-des-cigognes-et-le-capitaine-Guynemer-Juin-2016.pdf [RÉSUMÉ]. Association : piste (cigogne dessinée par un peintre du camouflage).
- **V45** — Cigogne blanche, fiches : INPN (MNHN), https://inpn.mnhn.fr/docs-web/docs/download/440861 ; Futura-Sciences, https://www.futura-sciences.com/planete/definitions/zoologie-cigogne-blanche-12422/ ; DREAL Occitanie, https://www.occitanie.developpement-durable.gouv.fr/IMG/pdf/Cigogne_blanche_cle2294ca.pdf ; GREPOM, https://www.grepom.org/wp-content/uploads/la-cigogne-blanche.pdf [RÉSUMÉ]. Longueur de 95-110, 100-125 ou 110-115 cm selon la fiche ; envergure de 155-215 ou 183-217 cm ; en vol, cou tendu et pattes qui dépassent la queue.
- **V46** — airforces.fr, « A MUST in chivalry history – When Guynemer spared Udet after strafing through his wing », https://airforces.fr/tag/spad-vii/page/2 [RÉSUMÉ]. Reprend le passage d'Udet (lettres noires, « Vieux », sur le fuselage entre les ailes) ; complète `spad7.md` S29.
- **V47** — Pistes sur les cigognes du GC 12 : Wikipédia (fr), « Escadrille des Cigognes », https://fr.wikipedia.org/wiki/Escadrille_des_Cigognes, et « SPA 3 Cigognes », https://fr.wikipedia.org/wiki/SPA_3_Cigognes ; escadrilles.org, https://www.escadrilles.org/armee-de-l-air/1000/ (appellations « Cigogne de Guynemer », « de Saint-Galmier », « japonaise », « de Fonck ») ; notice de vente Aguttes, « La Cigogne de Saint Galmier », https://www.naturaldiamonds.com/fr/culture-et-style/aguttes-la-cigogne-de-saint-galmier/ [RÉSUMÉ]. Wikis et notices : pistes.
- **V48** — Mascotte Hispano-Suiza : Revs Institute, « "Cigogne" ("Stork") for Automobiles Hispano-Suiza », https://museum.revsinstitute.org/exhibit/cigogne-stork-for-automobiles-hispano-suiza/ ; Safran, « 1916 : la cigogne devient l'emblème d'Hispano-Suiza », https://www.safran-group.com/fr/groupe/histoire-patrimoine/timeline/1916-cigogne-devient-embleme-hispano-suiza [RÉSUMÉ]. Origines divergentes.

Détails tirés le 2026-10-27 de sources déjà citées : S36 (*Motor Sport*, mai 1976) donne aussi les postures des SPA 26 (ailes à mi-course), 103 (début du battement) et 73 (tête retournée), et fait de la cigogne de la SPA 3 le modèle de la mascotte Hispano-Suiza ; V21 (page du CAEA) cite les ouvrages d'Osché et la revue *Icare* pour la cigogne rouge de S.254 ; V22 (notice Interenchères) décrit une maquette en bois : nom (écrit en capitales dans la notice, ce qui ne dit rien de la maquette), cigogne rouge, « 2 » rouge, gouvernail tricolore marqué « S 254 ».

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
- **2026-10-27 — Marques de S.254 : présence établie, emplacements non.** Le billet du Musée de l'Armée (V5) décrit un gros plan où l'on voit le nom, la cigogne et le « 2 » ; la brochure du ministère (V35) confirme le nom et la cigogne. **Présence le 20 octobre 1917 : B.** Aucune notice ne donne de position, de taille ni de couleur, et aucune image n'a été vue. **Retenu : gabarit chiffré `ESTIMÉ` (D) au § 12.6.9**, à remplacer par une mesure sur photo (D9, V40, V41) ou sur l'avion (D8).
- **2026-10-27 — Cigogne.**
  - Posture « ailes basses » : appellation de l'insigne (V36), lettre de 1976 (S36), wikis. **B.**
  - Couleur rouge : trois pistes, dont une qui cite Osché et *Icare*, et la règle du pochoir (V34). **C.** Les cigognes blanches ou noires signalées ailleurs concernent d'autres avions.
  - Deux flancs : C (V30, V34).
  - Sens vers le nez, position et taille : non trouvés, **ESTIMÉ** (D).
  - La silhouette du § 12.6.8 est construite à partir de la posture et de l'ornithologie (V45), sans copier aucune image.
- **2026-10-27 — « 2 ».** Présence B (V5) ; rouge C (V21, V22). Position en arrière de la cigogne, taille 0,40 m et graphie : **ESTIMÉ** (D). Pas de « 2 » sur les ailes ni sur le dessus du fuselage, faute de source ; la règle du dessus et des deux flancs, vue dans un résumé de Wikipédia (V47), est à vérifier.
- **2026-10-27 — « Vieux Charles ».** Présence B. Le noir et la position « entre les ailes » ne reposent que sur Udet (S29, V46), souvenir de 1935 d'un épisode contesté : **D**. Graphie et taille non trouvées : **ESTIMÉ**, capitales bâton de 0,12 m sous le tuyau d'échappement, avec une variante au-dessus du tuyau.
- **2026-10-27 — Gouvernail.** Albin Denis (V10, résumé) signale une inscription peinte qui n'a pas été mise à jour après le passage au 180 ch : l'existence d'une inscription est **C**. Le texte « S 254 » (D, notice d'une maquette) et sa place sur la bande blanche sont **ESTIMÉ**.
- **2026-10-27 — « Bande tricolore des as » : contradiction non tranchée.** La version anglaise du billet du Musée de l'Armée (V5) dit que la queue de S.254 porte la bande tricolore des as. Le § 12.5 classe une telle bande en anachronisme pour S.254. Deux lectures possibles : le tricolore réglementaire du gouvernail, ou une bande distincte comme sur S.115 et S.113. **Retenu provisoirement : pas de bande supplémentaire.** Premier point à vérifier sur les plaques du 20 octobre.
- **2026-10-27 — Photo espagnole exclue.** La photo de la Biblioteca Virtual de Defensa (V42) suit la 16e victoire (1916) : ce n'est pas S.254. Elle ne sert qu'à comparer la graphie du nom sur un avion antérieur.
