# Heure légale en 1917 : France, Belgique, Britanniques, Allemands, et lieu par défaut du soleil

- **Objet** : donner au calcul du soleil l'heure du jeu, c'est-à-dire l'heure légale française de 1917, heure d'été comprise (`docs/plans/m3-heure-du-jour.md`, étape 1 et décision 2), et le lieu par défaut.
- **Rédaction** : subagent `historien`, le 2026-11-11, en deux temps (une coupure de l'API a interrompu la première passe).
- **Statut** : première passe complète. Trois textes ont été lus dans des documents d'époque : la loi française de mars 1917 (au Sénat), l'ordre britannique du 30 mars 1917 pour l'île de Man, pris le même jour que celui du Royaume-Uni, et la Bekanntmachung allemande du 16 février 1917, reproduite par deux bulletins officiels. Le décret français qui fixe l'heure exacte du passage n'a pas été lu, car Gallica refuse l'outil web : les heures françaises restent en confiance **B** (§ 10).
- **En bref** :
  - **France** : l'heure légale est l'heure de Greenwich (loi du 9 mars 1911). En 1917, l'heure d'été court du **samedi 24 mars à 23 h** (on passe à minuit) au **dimanche 7 octobre à 24 h** (on revient à 23 h). Soit **UTC + 1** entre le 24 mars à 23 h 00 UTC et le 7 octobre à 23 h 00 UTC, **UTC + 0** le reste de l'année. Confiance **A** pour le cadre, **B** pour les dates et les heures ;
  - **Britanniques en France (BEF, donc RFC)** : ils suivent l'heure française, y compris ses dates d'heure d'été (**B**). Le Royaume-Uni lui-même passe à l'heure d'été du 8 avril au 17 septembre 1917 (**A**) ;
  - **Allemands** : heure d'Europe centrale (UTC + 1), et heure d'été du **16 avril** au **17 septembre 1917** (UTC + 2), dans le Reich comme dans les territoires occupés (**A** pour le Reich, **B** pour l'armée et les territoires occupés) ;
  - **Belgique non occupée et front de l'Yser** : heure de Greenwich, et probablement l'heure d'été française. Confiance **C** : aucun texte belge de 1917 n'a été trouvé ;
  - **Lieu par défaut** : Ypres, halle aux draps, **50,851° N ; 2,885° E** (**A**). Les 50,85° N et 2,89° E du plan conviennent, à 400 m près. Le terrain de la SPA 3 est à **Bergues-Bierne**, vers **50,97° N ; 2,40° E** (commune, **C**), sauf du 15 août au 25 septembre 1917, où elle est à Saint-Pol-sur-Mer ;
  - **à la date du jeu, le 20 octobre 1917**, l'heure française est l'heure UTC et l'heure allemande a une heure d'avance. À Ypres, le soleil passe au méridien à **11 h 33**, à **28,9°** de hauteur (USNO, § 7).
- **Règles** : skill `recherche-historique`. Les sources sont numérotées **H1, H2…** (§ 11). Les citations font moins de quinze mots et restent dans leur langue d'origine ; le reste est paraphrasé.
- **Marques de lecture** :
  - [LU] : document lu sur la page citée (image ou texte) ;
  - [LU-WEB] : page lue par l'outil web, qui la résume fidèlement ;
  - [RÉSUMÉ] : vu seulement dans un résumé de moteur de recherche (page refusée à l'outil) ;
  - [PISTE] : titre ou mention seulement.

## 1. France

### 1.1 Méridien de référence : l'heure de Greenwich depuis 1911

| Point | Valeur | Source | Confiance |
|---|---|---|---|
| Texte | La loi du 9 mars 1911 définit l'heure légale de la France et de l'Algérie comme le temps moyen de Paris retardé de 9 min 21 s | H3 ; H1 (le texte de 1917 renvoie à « l'heure légale fixée par la loi du 9 mars 1911 ») | **A** |
| Ce que cela donne | **le temps moyen de Greenwich**. La longitude de l'Observatoire de Paris, 2° 20′ 14″ E, vaut 9 min 20,9 s de temps (C: 2,3372° / 15 × 60) | H3, qui l'assimile au temps universel ; calcul | **A** |
| Entrée en vigueur | nuit du 10 au 11 mars 1911, à minuit ; les horloges sont arrêtées 9 min 21 s | H2 (zone `Europe/Paris`, « PMT » jusqu'au « 1911 Mar 11 ») ; message de la liste tz citant *Le Gaulois* et *Le Petit Journal* du 11 mars 1911 (H2b) | **B** |
| Durée | en vigueur jusqu'en 1978 ; donc en vigueur en 1917 | H3 | **A** |

Il n'y a donc **pas d'« heure de Paris »** à gérer en 1917 : hors heure d'été, l'heure française est l'heure UTC, à la seconde près.

### 1.2 Cadre légal de l'heure d'été de 1917

- **1916, première expérience.** Le député André Honnorat dépose sa proposition en mars 1916. La loi est promulguée le 9 juin 1916, et l'heure d'été s'applique du 15 juin au 1er octobre 1916 (H6, Guillaume Tronchet, historien, IHMC). Le 14 juin 1916, un ordre général de l'armée britannique en France aligne ses horloges sur ce décret français (H10, § 3).
- **1917, loi pour la durée de la guerre.** Le Sénat adopte le 15 mars 1917, en urgence et sans discussion générale, une proposition de loi « relative à l'avance de l'heure légale » (H1, p. 269-270, [LU]) :
  - **article unique** : pendant la guerre, et jusqu'au décret qui fixera la cessation des hostilités, le Gouvernement peut **avancer d'une heure par décret**, en France et en Algérie, l'heure légale de la loi de 1911 ;
  - **fenêtre** : l'avance ne peut s'appliquer qu'entre le **premier dimanche de mars** et le **premier dimanche d'octobre**, « dates extrêmes pour l'application ou le retrait de l'avance » ;
  - le rapporteur, M. Guilloteaux, la présente comme un compromis : ce n'est plus la mesure définitive voulue par Honnorat, mais l'expérience de 1916 étendue à la durée de la guerre, pour économiser le charbon. Il presse le Sénat, car le texte doit encore retourner à la Chambre, être promulgué, puis suivi du décret qui fixera la date.
- **Vote final à la Chambre** : le 19 mars 1917, par 291 voix contre 177, selon un site de calendriers qui ne cite pas sa source (H4). **Non vérifié** ; la date de promulgation de la loi n'a pas été lue (§ 10).
- **Conséquence** : en 1917, le retour à l'heure normale ne pouvait pas avoir lieu après le **dimanche 7 octobre**, premier dimanche d'octobre (C: le 1er janvier 1917 est un lundi, donc le 7 octobre est un dimanche).

### 1.3 Dates et heures du changement en 1917

| Changement | Heure lue sur les horloges | Instant en UTC | Sources | Confiance |
|---|---|---|---|---|
| Passage à l'heure d'été | **samedi 24 mars 1917, 23 h**, qui devient minuit (dimanche 25 mars, 0 h). L'heure de 23 h à 24 h n'existe pas | **24 mars, 23 h 00 UTC** | H2 (`Rule France 1917 only - Mar 24 23:00s`) ; H4 ; H5 ; H11 (un Britannique en Belgique note le 25 mars « Up at 7 by new time – 6 in reality ») | **B** |
| Retour à l'heure normale | **dimanche 7 octobre 1917, 24 h** (heure d'été), qui redevient 23 h. L'heure de 23 h à 24 h du 7 octobre se lit deux fois | **7 octobre, 23 h 00 UTC** | H2 (`Rule France 1916 1919 - Oct Sun>=1 23:00s`) ; H4 (« 7 octobre 1917, 23 h ») ; H5 (8 octobre, minuit) ; compatible avec la borne de la loi (H1) | **B** |
| Décalage | UTC + 0 l'hiver, **UTC + 1** entre ces deux instants | — | H1, H2 | **A** |

- Le `s` de la base tz signifie « heure normale » : 23 h 00 en heure normale, c'est 23 h 00 UTC (H2).
- La base tz suit pour la France l'*Annuaire du Bureau des longitudes* (édition 2020), sauf exceptions qui ne touchent pas 1917 (H2, commentaire de P. Eggert du 21 juillet 2025).
- **Ce qui manque pour passer en A** : le décret de mars 1917 qui fixe l'heure du passage, et le texte qui fixe le retour (§ 10).

### 1.4 Pour mémoire, si la campagne déborde de 1917

| Année | Heure d'été (instants UTC) | Sources | Confiance |
|---|---|---|---|
| 1916 | du 14 juin, 23 h 00 UTC, au 1er octobre, 23 h 00 UTC | H2 ; H10 (ordre britannique du 14 juin 1916, passage à 23 h) | **B** |
| 1918 | du 9 mars, 23 h 00 UTC, au 6 octobre, 23 h 00 UTC | H2 ; H9 (la Chambre des communes confirme le début français du 9 mars 1918) | **B** pour le début, **C** pour la fin (H2 seul) |

Ces deux lignes n'ont pas été recherchées plus avant.

## 2. Belgique non occupée et front des Flandres

| Point | Constat | Sources | Confiance |
|---|---|---|---|
| Heure légale belge d'avant-guerre | heure de Greenwich, depuis la loi du 28 avril 1892, en vigueur le 1er mai 1892 | H15 | **A** |
| Territoire occupé | heure allemande imposée à partir de novembre 1914 (8 novembre à Bruxelles), puis l'heure d'été allemande. Le moment du passage variait avec l'avance ou le recul de l'occupant | H15 ; H2 (zone `Europe/Brussels`, d'après Pluvinage 2014, H23) | **B** |
| Territoire non occupé, règle générale | l'Observatoire royal écrit que le reste du territoire suivait l'heure de Greenwich, sans parler d'heure d'été | H15 | **B** pour l'heure normale |
| Heure d'été de 1916 derrière l'Yser | selon la VRT, la zone derrière l'Yser s'aligne sur la France, qui passe à l'heure d'été le 15 juin 1916. Un musée virtuel note qu'au printemps 1916, il était deux heures de moins dans le Westhoek non occupé que de l'autre côté de l'Yser | H16 ; H24 | **C** (presse et médiation, sans source citée) |
| Heure d'été de 1918 en Belgique non occupée | du 9 mars 1918, 0 h UTC, au 5 octobre 1918, 23 h UTC : presque les dates françaises | H15 (tableau à partir de 1918) ; H2 (« they apply to unoccupied Belgium ») | **B** |
| **Heure d'été de 1917 en Belgique non occupée** | **aucun texte trouvé**. Les cas de 1916 et de 1918 suggèrent l'alignement sur la France : UTC + 1 du 24 mars au 7 octobre 1917 | déduction de H15, H16 et H2 | **C** |
| Troupes françaises en Belgique | la 1re Armée française tient le secteur de Boesinghe, en Belgique (`flandres-1917.md` § 2.2). Rien n'indique qu'elle ait quitté l'heure légale française | déduction | **C** |
| Troupes britanniques en Belgique | un infirmier du 58th Field Ambulance, à La Clytte (De Klijte, à 10 km au sud-ouest d'Ypres), avance sa montre le 25 mars 1917, deux semaines avant le Royaume-Uni : le BEF en Belgique suit l'heure française | H11 | **B** |

**Pour le jeu** : dans le secteur d'Ypres et de l'Yser, les trois armées alliées lisent la même heure en 1917, l'heure française. Seuls les Allemands ont une autre heure.

## 3. Les Britanniques : Royaume-Uni, BEF et RFC

### 3.1 Au Royaume-Uni

| Point | Valeur | Sources | Confiance |
|---|---|---|---|
| Heure normale | temps moyen de Greenwich (GMT) | H7 | **A** |
| Base légale | *Summer Time Act, 1916*. Pendant la guerre, un ordre en conseil peut remettre la loi en vigueur une année donnée et en fixer la période | H7 (préambule de l'ordre) | **A** |
| Heure d'été de 1917 | du **dimanche 8 avril, 2 h GMT**, au **lundi 17 septembre, 2 h GMT** : à 2 h 00 GMT, les horloges passent à 3 h le 8 avril ; elles reviennent de 3 h à 2 h le 17 septembre | H7 : ordre en conseil du 30 mars 1917 pour l'île de Man, lu dans la *London Gazette* du 3 avril 1917, p. 3200 [LU] ; H8 et H2 pour l'ordre du Royaume-Uni (S.R.&O. 1917, n° 362 selon H8, n° 358 selon H2) | **A** pour les dates. L'ordre du Royaume-Uni lui-même n'a pas été lu |
| Décalage | UTC + 1 du 8 avril, 2 h 00 UTC, au 17 septembre, 2 h 00 UTC ; UTC + 0 sinon | H7 | **A** |

### 3.2 Le BEF en France et en Belgique, et donc le RFC

- **Règle** : les armées britanniques en France adoptent l'heure française.
  - **1916** : un ordre général (G.R.O. 2334 du 14 juin 1916) dit que les armées en France « will adopt the same time as ordered by the recent decree of the French Government ». Le 14 juin à 23 h, les horloges sont avancées d'une heure (H10, D. Tattersfield, *Western Front Association*, 2020, [LU-WEB]).
  - **1917, printemps** : le 25 mars 1917, l'infirmier Linfoot se lève « by new time », à La Clytte, en Belgique (H11). Un journal de marche britannique cité sur le *Great War Forum* note que les armées britanniques en France passent à l'heure d'été le 24 mars à 23 h, qui devient minuit (H12, [RÉSUMÉ]).
  - **1917, automne** : d'après le même forum, le BEF revient à l'heure d'hiver dans la nuit du 7 au 8 octobre, à 1 h du matin, selon un journal de marche du 2nd ANZAC Corps (H12, [RÉSUMÉ]). Ce serait une heure après le retour français (24 h). Cette heure n'est **pas vérifiée**.
  - **1918** : le ministre de l'Intérieur confirme aux Communes, le 14 mars 1918, que les forces britanniques en France ont adopté l'heure d'été française, entrée en vigueur le 9 mars (H9, [LU-WEB]).
- **Le RFC fait partie du BEF** : ses rapports de combat, ses carnets d'escadrille et ses comptes rendus quotidiens sont donc à l'heure française en 1917. Confiance **B** : la règle du BEF est bien attestée, mais aucun document du RFC de 1917 n'a été lu pour la vérifier directement.
- **Piège** : un fil du *Great War Forum* signale des carnets de vol de pilotes du RFC tenus avec une heure d'écart (H12, [RÉSUMÉ]). L'heure d'un carnet individuel est à vérifier pièce par pièce.
- **Non vérifié** : les escadrilles du RNAS de Dunkerque (Saint-Pol, Coudekerque) dépendaient de l'Amirauté, pas du BEF. Aucune source lue ne dit si elles suivaient l'heure française.

**Décalage BEF retenu pour 1917** : le même que la France, UTC + 1 du 24 mars à 23 h 00 UTC au 7 octobre à 23 h 00 UTC. Confiance **B** pour le début, **C** pour l'heure exacte du retour.

**Conséquence pour les lettres et les journaux** : l'heure du BEF et celle de Londres diffèrent d'une heure du 25 mars au 8 avril, puis du 17 septembre au 7 octobre 1917. Le reste de l'année, elles sont égales.

## 4. Les Allemands

### 4.1 Dans le Reich

| Point | Valeur | Sources | Confiance |
|---|---|---|---|
| Heure normale | heure d'Europe centrale, UTC + 1, depuis avril 1893 | H2 (zone `Europe/Berlin` : temps local jusqu'en avril 1893, puis « 1:00 C-Eur ») | **A** |
| Texte de 1917 | Bekanntmachung du 16 février 1917, signée par le vice-chancelier Helfferich, qui reconduit l'heure d'été de 1916 | H13, H14 [LU] | **A** |
| Heure d'été de 1917 | du **lundi 16 avril 1917, 2 h**, les horloges étant avancées à 3 h, au **lundi 17 septembre 1917, 3 h**, les horloges étant reculées à 2 h. L'heure de 2 h à 3 h du 17 septembre se lit deux fois | H13 (*Schulverordnungsblatt* du grand-duché de Bade, n° 7, 22 mars 1917, p. 66, citant la Bekanntmachung) ; H14 (*Marineverordnungsblatt*, n° 7, 15 mars 1917, p. 70-71, qui la reproduit) ; H2 (`Rule C-Eur 1917 1918 - Apr Mon>=15 2:00s` et `Sep Mon>=15 2:00s`) ; H15 | **A** |
| Instants en UTC | du 16 avril, **1 h 00 UTC**, au 17 septembre, **1 h 00 UTC** | C: 2 h en heure d'Europe centrale = 1 h UTC ; H15 (« 1 h UTC » aux deux dates) | **A** |
| Décalage | **UTC + 2** pendant l'heure d'été, **UTC + 1** sinon | H2, H13, H15 | **A** |

### 4.2 L'armée allemande et les territoires occupés

- **Belgique occupée** : l'Observatoire royal donne pour 1917 le passage à UTC + 2 le 16 avril à 1 h UTC, et le retour à UTC + 1 le 17 septembre à 1 h UTC : ce sont les dates du Reich (H15). Confiance **B**.
- **Nord de la France occupé** :
  - à Lille, une affiche allemande du 18 octobre 1914 ordonne de mettre toutes les horloges à l'heure allemande, en avance d'une heure sur l'heure française (H18, [RÉSUMÉ]) ;
  - le *Bulletin de Lille*, journal publié sous contrôle allemand, annonce le 7 mai 1916 que l'autorité allemande avance encore les horloges publiques d'une heure à partir du 1er mai : elles ont alors deux heures d'avance sur l'heure française (H17, [LU-WEB]) ;
  - c'est la même règle qu'en Belgique occupée. Pour 1917, aucun document lillois n'a été lu : confiance **B**.
- **Les rapports allemands** : les unités du front de Flandre (4. Armee, Jastas, Kofl 4) stationnent en Belgique occupée. **Leurs heures sont donc à l'heure allemande** : UTC + 1 l'hiver, UTC + 2 du 16 avril au 17 septembre 1917. Confiance **B** : aucun ordre de l'armée allemande sur l'heure n'a été lu, mais l'occupant imposait son heure aux civils (H15, H17), ce qui rend une autre heure pour la troupe très improbable.

### 4.3 Écart entre l'heure allemande et l'heure alliée en 1917

| Période (instants UTC) | Heure allemande moins heure française | Exemple |
|---|---|---|
| 1er janvier → 24 mars, 23 h 00 | **+1 h** | — |
| 24 mars, 23 h 00 → 16 avril, 1 h 00 | **0 h** : les deux camps lisent la même heure | Arras (9 avril) |
| 16 avril, 1 h 00 → 17 septembre, 1 h 00 | **+1 h** | Messines (7 juin), Pilckem (31 juillet), 11 septembre |
| 17 septembre, 1 h 00 → 7 octobre, 23 h 00 | **0 h** | route de Menin (20 septembre), Broodseinde (4 octobre) |
| 7 octobre, 23 h 00 → 31 décembre | **+1 h** | **20 octobre** (date du jeu), Passchendaele |

C: d'après les §§ 1.3 et 4.1. **C'est le principal piège pour confronter une victoire allemande et une perte alliée** : l'écart n'est pas toujours d'une heure.

## 5. Tableau récapitulatif des décalages par rapport à Greenwich, 1917

Instants de début et de fin en UTC. « +1 » signifie UTC + 1 h.

| Période (UTC) | France, heure légale (heure du jeu) | Belgique non occupée | BEF et RFC en France et en Belgique | Royaume-Uni | Allemands (Reich, armée, territoires occupés) |
|---|---|---|---|---|---|
| 1er janv., 0 h → **24 mars, 23 h 00** | +0 | +0 | +0 | +0 | +1 |
| 24 mars, 23 h 00 → **8 avril, 2 h 00** | **+1** | +1 (C) | **+1** | +0 | +1 |
| 8 avril, 2 h 00 → **16 avril, 1 h 00** | +1 | +1 (C) | +1 | **+1** | +1 |
| 16 avril, 1 h 00 → **17 sept., 1 h 00** | +1 | +1 (C) | +1 | +1 | **+2** |
| 17 sept., 1 h 00 → **17 sept., 2 h 00** | +1 | +1 (C) | +1 | +1 | **+1** |
| 17 sept., 2 h 00 → **7 oct., 23 h 00** | +1 | +1 (C) | +1 (fin : C) | **+0** | +1 |
| 7 oct., 23 h 00 → 31 déc., 24 h | **+0** | +0 | **+0** (peut-être à partir du 8 oct., 0 h 00, § 3.2) | +0 | +1 |
| **Confiance** | **A** pour le cadre, **B** pour les dates | **C** en 1917 | **B**, retour d'octobre **C** | **A** | **A** pour le Reich, **B** pour l'armée et les territoires occupés |

## 6. Lieu par défaut

### 6.1 Ypres

| Point | Valeur | Sources | Confiance |
|---|---|---|---|
| Halle aux draps (Lakenhalle), Grote Markt | **50,8512° N ; 2,8849° E** | H20 (OpenStreetMap, bâtiment « historic ») | **A** |
| Centre de la ville | 50° 51′ 03″ N ; 2° 53′ 06″ E, soit 50,8508° N ; 2,8850° E | H20 (Wikipédia, à titre de contrôle) | **A** |
| **Retenu** | **50,851° N ; 2,885° E** | — | **A** |
| Écart avec le plan (50,85° N ; 2,89° E) | environ 350 m vers l'est (C: 0,005° × 70,3 km/° de longitude) ; sans effet sur le soleil | calcul | — |
| Temps moyen local | UTC + 11 min 32 s (C: 2,885° / 15° par heure) | calcul | **A** |

Les coordonnées sont celles d'aujourd'hui. Comme pour les communes de `flandres-1917.md` § 1.3, le centre de la ville n'a pas bougé à l'échelle du jeu.

### 6.2 Terrains de la SPA 3 à l'été et à l'automne 1917

Repris de `flandres-1917.md` § 1 (sources F1 à F3 et F6 de cette fiche). Cette passe n'a rien trouvé de plus précis.

| Période | Terrain | Coordonnées | Confiance |
|---|---|---|---|
| 12 juillet → 14 août 1917 | Bergues-Bierne | **50,972° N ; 2,403° E**, centre de la commune de Bierne ; emplacement du pré non établi | **B** pour le terrain, **C** pour le point |
| 15 août → 25 septembre 1917 | Saint-Pol-sur-Mer, près de Dunkerque | **≈ 51,03° N ; 2,35° E** | **B** pour le terrain, **D** pour le point |
| 26 septembre → 10 décembre 1917 (donc le **20 octobre**) | Bergues-Bierne | 50,972° N ; 2,403° E | **B** / **C** |

- **Emplacement exact du terrain de Bierne : non trouvé.** L'ouvrage qui le donnerait probablement est l'*Atlas Nord 1914-1918* de Jérôme Grosse (220 fiches de terrains d'aviation du Nord, avec un extrait de carte d'état-major par terrain ; H22, [PISTE]).
- Entre Ypres et Bierne, le soleil se décale de **1,9 min** (C: écart de longitude de 0,482°) et de moins d'un demi-degré en hauteur. Le choix du lieu n'a donc pas d'effet visible sur le ciel : Ypres pour les vols au-dessus du front, Bierne pour les scènes de terrain.

## 7. Valeurs de contrôle pour les tests

Éphémérides de l'US Naval Observatory pour Ypres (50,851° N ; 2,885° E), calculées par son service en ligne (H19, [LU-WEB], domaine public). Heures en UTC ; heure légale française entre parenthèses. Les hauteurs sont les hauteurs calculées (Hc) du service de navigation astronomique : centre du disque, sans réfraction ni parallaxe, que le service donne à part (réfraction de −0,03° à 29° de hauteur, le 20 octobre).

| Date (1917) | Aube civile | Lever | Passage au méridien | Coucher | Fin du crépuscule civil | Hauteur au méridien | Déclinaison |
|---|---|---|---|---|---|---|---|
| 21 juin | 2 h 48 | 3 h 34 (4 h 34) | **11 h 50 (12 h 50)** | 20 h 06 (21 h 06) | 20 h 52 | **62,60°** | +23,45° |
| 31 juillet (Pilckem) | 3 h 32 | 4 h 12 (5 h 12) | **11 h 55 (12 h 55)** | 19 h 37 (20 h 37) | 20 h 16 | **57,51°** | +18,36° |
| **20 octobre (date du jeu)** | 5 h 45 | **6 h 19 (6 h 19)** | **11 h 33 (11 h 33)** | **16 h 47 (16 h 47)** | 17 h 21 | **28,92°** | −10,23° |
| 22 décembre | 7 h 09 | 7 h 49 (7 h 49) | **11 h 47 (11 h 47)** | 15 h 45 (15 h 45) | 16 h 25 | — | — |

Positions ponctuelles (même service) :

| Date et heure UTC (heure légale) | Hauteur | Azimut | Usage |
|---|---|---|---|
| 1er septembre 1917, 14 h 00 (**15 h 00**) | **39,6°** | **224,2°** | **redonne le soleil actuel du jeu (225°, 40°)** : date et heure proposées pour le vol d'essai (`just run`) |
| 20 octobre 1917, 14 h 00 (14 h 00) | 21,1° | 219,0° | après-midi de la date du jeu |
| 31 juillet 1917, 14 h 30 (15 h 30) | 45,3° | 237,8° | contrôle d'été |
| 31 juillet 1917, 15 h 00 (16 h 00) | 41,1° | 245,7° | contrôle d'été |

- Tolérance proposée pour un algorithme de la NOAA : une minute pour les heures, quelques centièmes de degré pour les angles (`ESTIMÉ`). Le service arrondit ses heures à la minute.
- **À noter pour la direction artistique** : le 20 octobre 1917, le soleil ne dépasse pas **29°** à Ypres. Un soleil à 40° demande une date comprise à peu près entre la fin de mars et la mi-septembre.

## 8. Ce que le jeu doit faire

1. **Garder le temps en UTC** dans la simulation et dans l'algorithme du soleil, qui attend du temps universel. L'heure légale ne sert qu'à l'affichage et à la saisie des scènes et des missions.
2. **Heure du jeu = heure légale française** :
   - **UTC + 0** jusqu'au **24 mars 1917, 23 h 00 UTC** ;
   - **UTC + 1** du **24 mars 1917, 23 h 00 UTC** au **7 octobre 1917, 23 h 00 UTC** ;
   - **UTC + 0** ensuite ;
   - pour 1916 et 1918, les dates du § 1.4 (B). Mieux vaut une table de changements dans un fichier de données, avec la source de chaque ligne, qu'une règle codée en dur.
3. **Les deux heures ambiguës** :
   - le 24 mars 1917, de 23 h 00 à 23 h 59 en heure légale, **n'existe pas** : une scène qui la demande doit être refusée au chargement ;
   - le 7 octobre 1917, de 23 h 00 à 23 h 59, **existe deux fois**. Il faut soit l'interdire dans les scènes, soit exiger qu'on précise « heure d'été » ou « heure normale ».
4. **L'heure allemande**, pour les textes qui citent un document allemand (revendication de victoire, rapport intercepté, dialogue d'un prisonnier) : UTC + 1, et **UTC + 2 du 16 avril, 1 h 00 UTC, au 17 septembre 1917, 1 h 00 UTC**. Afficher à l'heure allemande une heure tirée d'un document allemand, en le signalant (« heure allemande »), plutôt que de la convertir en silence.
5. **L'heure britannique** : les rapports du RFC sont à l'heure du jeu. Seules les lettres écrites au Royaume-Uni, ou qui citent l'heure de Londres, diffèrent, d'une heure, du 25 mars au 8 avril et du 17 septembre au 7 octobre 1917.
6. **Le lieu par défaut** : Ypres, **50,851° N ; 2,885° E**. Les scènes de terrain de la SPA 3 peuvent prendre Bierne (50,972° N ; 2,403° E), sans différence visible.
7. **La date du jeu** : le 20 octobre 1917, l'heure du jeu est l'heure UTC. À Ypres, midi solaire tombe à 11 h 33, et le soleil plafonne à 29° (§ 7).
8. **Tests** : reprendre les valeurs du § 7. Elles couvrent à la fois l'heure d'été (21 juin, 31 juillet, 1er septembre) et l'heure normale (20 octobre, 22 décembre).

## 9. Pièges pour la lecture des documents d'époque

- **Victoire allemande et perte alliée** : l'écart d'heure varie dans l'année (§ 4.3). Le même combat peut être noté à 11 h 30 côté français et à 12 h 30 côté allemand en juillet, mais à la même heure le 20 septembre.
- **Éphémérides d'époque** : jusqu'au 31 décembre 1924, les astronomes faisaient commencer le jour de « Greenwich Mean Time » à midi, et non à minuit (H25, [RÉSUMÉ]). Une heure lue dans un almanach nautique de 1917 peut donc différer de 12 h de l'heure civile. Le service de l'USNO (§ 7) donne, lui, l'heure civile.
- **Carnets de vol individuels** : une heure d'écart est possible (§ 3.2) ; croiser avec le compte rendu de l'unité.
- **Heure du chemin de fer** : avant 1911, l'heure des gares françaises retardait de 5 min sur l'heure civile (H2). Cela ne concerne plus 1917.

## 10. Ce qui manque, et documents à récupérer

### 10.1 Pour la session principale (téléchargements, sans demande extérieure)

1. **Gallica, *Journal officiel de la République française. Lois et décrets*** (notice BnF `ark:/12148/cb34378481r`), numéros quotidiens à l'adresse `https://gallica.bnf.fr/ark:/12148/cb34378481r/dateAAAAMMJJ`. Gallica refuse l'outil web (403).
   - **Du 19 au 24 mars 1917** (`date19170319` à `date19170324`) : la loi sur l'avance de l'heure (date de promulgation) et le **décret qui fixe le passage au samedi 24 mars à 23 h**. Relever si le même décret fixe aussi le retour d'octobre.
   - **Du 1er au 8 octobre 1917** : le texte du retour, s'il est distinct, et l'heure exacte (24 h le 7 octobre, ou autre).
   - Ce sont les deux documents qui feraient passer les dates françaises de **B** à **A**.
2. **Gallica, *Journal officiel*, Débats parlementaires, Chambre des députés**, séance du 19 mars 1917 : le vote de 291 voix contre 177 rapporté par H4 (ark de la collection à relever).
3. **Mémoire des hommes (Service historique de la Défense)**, journaux de marches et opérations de la **SPA 3** et du **GC 12** : les jours du 24-25 mars et du 7-8 octobre 1917, pour voir si l'unité note l'avance ou le retard de l'heure. Cote à relever dans l'inventaire des JMO de l'aéronautique ; à joindre à la demande D17 de `demandes.md` si les JMO ne sont pas numérisés.
4. **FranceArchives** : recherche « avance de l'heure 1917 » et « heure d'été 1917 » (affiches préfectorales de mars 1917, notamment aux Archives départementales du Nord). Adresse : `https://francearchives.gouv.fr/fr/search?q=avance%20de%20l%27heure%201917`.
5. **Musée de l'Air et de l'Espace** : rien à y chercher pour cette question.
6. ***Great War Forum*** (refusé à l'outil, 403) : fils 305962 (« End of summer time 1917 »), 186024 (« British summer time on the front ») et 252240 (« Time difference between UK, Belgium and France »). Y relever les citations exactes des journaux de marche britanniques (unité, date, référence TNA WO 95) pour le 24 mars et le 7-8 octobre 1917.
7. **Moniteur belge de 1917**, publié au Havre : un arrêté sur l'heure d'été en territoire non occupé, s'il existe. Lieu à trouver (KBR, BelgicaPress, bibliothèque de la Chambre).
8. **Pluvinage, « Brussels on German time »**, *Cahiers bruxellois*, 2014, sur Cairn : `https://www.cairn.info/revue-cahiers-bruxellois-2014-1E-page-15.htm` (H23).

### 10.2 Ce qui reste non trouvé

- Le décret français de mars 1917 et le texte du retour d'octobre (heures exactes).
- Un texte belge sur l'heure d'été de 1917 en territoire non occupé.
- Un ordre du BEF de 1917 sur l'heure (General Routine Order), et l'heure exacte de son retour en octobre.
- Un ordre de l'armée allemande sur l'heure de la troupe en Flandre.
- L'heure suivie par le RNAS de Dunkerque.
- L'emplacement exact du terrain de Bierne.

**Italien** : recherche non faite, volontairement. Aucune unité italienne ne volait en Flandre, et l'heure italienne ne servirait ni aux rapports ni au soleil du jeu.

## 11. Sources

- **H1** — Sénat, *Journal officiel de la République française*, Débats parlementaires, Sénat, session ordinaire de 1917, 18e séance, « Séance du jeudi 15 mars », JO du 16 mars 1917, p. 269-270, point 7 « Adoption d'une proposition de loi relative à l'avance de l'heure légale » (rapporteur M. Guilloteaux).
  - https://www.senat.fr/comptes-rendus-seances/3eme/pdf/1917/03/S19170315_0269_0284.pdf (consulté le 2026-11-11) [LU, p. 269-270].
  - Primaire.
- **H2** — IANA, base de données des fuseaux horaires (tzdb), fichier `europe` : règles `France`, `GB-Eire`, `C-Eur` et `Belgium` ; zones `Europe/Paris`, `Europe/Brussels` et `Europe/Berlin` ; commentaires de P. Eggert (Annuaire du Bureau des longitudes, chemins de fer en 1911).
  - https://ftp.iana.org/tz/code/europe (consulté le 2026-11-11) [LU-WEB].
  - Compilation sourcée, de référence pour les logiciels ; secondaire.
  - **H2b** : P. Eggert, correctif « French clocks stopped for 9′21″ on 1911-03-11 », liste tz, 10 juin 2020, citant *Le Gaulois* et *Le Petit Journal* du 11 mars 1911 : https://lists.iana.org/hyperkitty/list/tz@iana.org/message/CNVGRIAZCH2HYIG2CVD6Z4OFHHM3AJMJ/ (consulté le 2026-11-11) [LU-WEB].
- **H3** — IMCCE et Observatoire de Paris, page « promenade » sur l'heure légale, https://promenade.imcce.fr/fr/pages5/502.html (consulté le 2026-11-11) [LU-WEB]. Résume la loi du 9 mars 1911. Secondaire institutionnelle.
- **H4** — iCalendrier, « Petite histoire de l'heure d'été », https://icalendrier.fr/calendriers-saga/etudes-thematiques/heure-ete-hiver (consulté le 2026-11-11) [LU-WEB]. Sans auteur ni source pour 1916-1918. **Piste.**
- **H5** — timeanddate.com, « Time change 1917, Paris », https://www.timeanddate.com/time/change/france/paris?year=1917 [RÉSUMÉ ; page refusée à l'outil, 403]. **Piste**, concordante.
- **H6** — Guillaume Tronchet (IHMC, ENS-PSL, Paris 1, CNRS), « Heure d'été, heure d'hiver : le couvre-feu oublié de la Grande Guerre », *The Conversation France*, 25 octobre 2020, https://theconversation.com/heure-dete-heure-dhiver-le-couvre-feu-oublie-de-la-grande-guerre-148671 (consulté le 2026-11-11) [LU-WEB]. Secondaire, historien.
- **H7** — *The London Gazette*, n° 30008, mardi 3 avril 1917, p. 3199-3200 : ordre en conseil du 30 mars 1917 qui met en vigueur pour 1917 le *Summer Time (Isle of Man) Act, 1916*, avec son préambule sur le *Summer Time Act, 1916*.
  - https://www.thegazette.co.uk/London/issue/30008/page/3200/data.pdf (consulté le 2026-11-11) [LU].
  - Primaire.
- **H8** — WebExhibits, « Daylight Saving Time — United Kingdom law », https://www.webexhibits.org/daylightsaving/uk.html (consulté le 2026-11-11) [LU-WEB]. Tableau des dates de 1916-1918 et références des ordres (S.R.&O. 1917 n° 362, du 30 mars 1917). Secondaire.
- **H9** — Chambre des communes, HC Deb 14 March 1918, vol. 104, c. 463, « Summer Time » (réponse de Sir George Cave à M. Tyson Wilson), https://api.parliament.uk/historic-hansard/commons/1918/mar/14/summer-time (consulté le 2026-11-11) [LU-WEB]. Primaire.
- **H10** — David Tattersfield, « Spring Forward, Fall Back. The changing of the clocks and the First World War », *Western Front Association*, 24 octobre 2020, https://www.westernfrontassociation.com/world-war-i-articles/spring-forward-fall-back-the-changing-of-the-clocks-and-the-first-world-war (consulté le 2026-11-11) [LU-WEB]. Cite le G.R.O. 2334 du 14 juin 1916. Secondaire.
- **H11** — *The Diary of Arthur L. Linfoot*, 58th Field Ambulance, R.A.M.C., entrée du dimanche 25 mars 1917 (lieu indiqué : La Clytte), transcrite de la sténographie Pitman, https://www.arthurlinfoot.org.uk/2017/03/25/25-march-1917-sunday/ (consulté le 2026-11-11) [LU-WEB]. Primaire (journal intime). La note de bas de page est des éditeurs du site.
- **H12** — *Great War Forum*, fils 305962, 186024, 252240 et 287230, https://www.greatwarforum.org/topic/305962-end-of-summer-time-1917/ etc. [RÉSUMÉ ; pages refusées à l'outil, 403]. Citations de journaux de marche britanniques de 1917. **Pistes.**
- **H13** — *Schulverordnungsblatt für das Großherzogtum Baden*, n° 7, Karlsruhe, 22 mars 1917, p. 66, « Die Einführung der Sommerzeit betreffend », citant la Bekanntmachung du Reichskanzler du 16 février 1917.
  - Badische Landesbibliothek, https://digital.blb-karlsruhe.de/download/pdf/6491541.pdf (consulté le 2026-11-11) [LU, p. 65-66].
  - Primaire (bulletin officiel).
- **H14** — *Marineverordnungsblatt*, n° 7, Berlin, 15 mars 1917, n° 77, p. 70-71, « Bekanntmachung über die Vorverlegung der Stunden während der Zeit vom 16. April bis 17. September 1917 », datée de Berlin le 16 février 1917, signée par le Stellvertreter des Reichskanzlers, Dr Helfferich.
  - Bibliothèque d'Elbląg, https://dlibra.bibliotekaelblaska.pl//Content/53204/PDF/07.pdf (consulté le 2026-11-11) [LU, images de faible définition : le titre, les dates, les heures et la signature sont lisibles, pas tout le corps du texte].
  - Primaire. La référence au *Reichs-Gesetzblatt* n'a pas été relevée.
- **H15** — Observatoire royal de Belgique (Le Binh San Pham), « Les dates de changements d'heures en Belgique » et « Le fuseau horaire de la Belgique et le système du changement d'heure », robinfo.oma.be :
  - https://robinfo.oma.be/fr/astro-info/heure/le-fuseau-horaire-belge-et-le-systeme-du-changement-dheure/les-dates-de-changements-dheures-en-belgique/ ;
  - https://robinfo.oma.be/fr/astro-info/heure/le-fuseau-horaire-belge-et-le-systeme-du-changement-dheure/ ;
  - consultés le 2026-11-11 [LU-WEB]. Secondaire institutionnelle, sans référence citée.
- **H16** — Tim Trachet et Jan Ouvry, « De zomertijd, een erfenis van de Eerste Wereldoorlog », VRT NWS, 8 février 2018, https://www.vrt.be/vrtnws/nl/2018/02/08/de-zomertijd--een-erfenis-van-de-eerste-wereldoorlog/ (consulté le 2026-11-11) [LU-WEB]. Presse, sans source citée.
- **H17** — *Bulletin de Lille*, n° 155, dimanche 7 mai 1916, Project Gutenberg n° 20824, https://www.gutenberg.org/cache/epub/20824/pg20824-images.html (consulté le 2026-11-11) [LU-WEB]. Primaire (journal publié sous contrôle allemand).
- **H18** — Affiche de l'autorité allemande, Lille, 18 octobre 1914, imposant l'heure allemande, signalée par un résultat de recherche sur museum-digital (Rhénanie-Palatinat, objet 7838) ; page introuvable (404) [RÉSUMÉ]. **Piste.**
- **H19** — US Naval Observatory, Astronomical Applications Department, API « Complete Sun and Moon Data for One Day » (`/api/rstt/oneday`) et « Celestial Navigation Data » (`/api/celnav`), pour 50,851° N ; 2,885° E, aux dates et heures du § 7, https://aa.usno.navy.mil/api/ (consulté le 2026-11-11) [LU-WEB]. Calcul de référence, domaine public.
- **H20** — Coordonnées d'Ypres :
  - OpenStreetMap, Nominatim, « Lakenhalle, Grote Markt, Ieper » : https://nominatim.openstreetmap.org/search?q=Lakenhalle%2C+Ieper&format=json (consulté le 2026-11-11) [LU-WEB] ;
  - Wikipédia (en), « Ypres », https://en.wikipedia.org/wiki/Ypres (consulté le 2026-11-11) [LU-WEB], en contrôle.
  - Seules les coordonnées, des faits, sont reprises.
- **H21** — `docs/recherche/flandres-1917.md` § 1 (sources F1, F2, F3 et F6), pour les terrains de la SPA 3.
- **H22** — Jérôme Grosse (illustrations de Jean-Michel Borde, préface de Philippe Van Damme), *Atlas Nord 1914-1918 : terrains d'aviation militaires, plates-formes aéronautiques temporaires principales et secondaires*, Éditions Anciens Aérodromes, 2017, 272 p., ISBN 978-2-919572-11-3 (notice Mollat consultée le 2026-11-11) [PISTE].
- **H23** — G. Pluvinage, « Brussels on German time », *Cahiers bruxellois – Brusselse cahiers*, XLVI (1E), 2014, p. 15-38, cité par H2 [PISTE].
- **H24** — FAAM (Geheugen Collectief), « De Duitse zomertijd », ErfgoedApp, https://en.erfgoed.app/de-duitse-zomertijd/ (consulté le 2026-11-11) [LU-WEB]. Médiation patrimoniale.
- **H25** — NIST, « Everyday Time and Atomic Time, Part 4 », blog *Taking Measure*, https://www.nist.gov/blogs/taking-measure/everyday-time-and-atomic-time-part-4 [RÉSUMÉ] : adoption du jour civil par les astronomes le 1er janvier 1925.

## 12. Journal des décisions

- **2026-11-11 — Méridien : Greenwich, confiance A.**
  - La loi du 9 mars 1911 est citée par le texte même de 1917 (H1) et résumée par l'IMCCE (H3).
  - Les 9 min 21 s sont la longitude de l'Observatoire de Paris, arrondie à la seconde.
  - Aucune heure de Paris à gérer en 1917.
- **2026-11-11 — Dates françaises de 1917 : du 24 mars, 23 h 00 UTC, au 7 octobre, 23 h 00 UTC, confiance B.**
  - Le cadre (avance d'une heure, entre le premier dimanche de mars et le premier dimanche d'octobre) est lu dans la séance du Sénat du 15 mars 1917 (A).
  - Les dates et les heures viennent de la base tz, qui suit l'*Annuaire du Bureau des longitudes*. Deux pistes concordent (H4, H5), et un témoin britannique en Belgique confirme le passage dans la nuit du 24 au 25 mars (H11).
  - Le retour du 7 octobre tombe sur la borne extrême fixée par la loi.
  - Le décret n'a pas été lu : B, et non A. Les documents à télécharger sont au § 10.1.
- **2026-11-11 — BEF et RFC : l'heure française, confiance B.**
  - La règle est écrite pour 1916 (H10) et pour 1918 (H9).
  - Pour 1917, elle est attestée au printemps par un journal intime en Belgique (H11) et, en piste, par un journal de marche (H12).
  - Pour le retour d'octobre, deux pistes du forum placent le changement britannique à 1 h du matin dans la nuit du 7 au 8 octobre, contre 24 h pour la France. On retient l'heure française : l'écart possible, d'une heure, tombe en pleine nuit et ne touche pas le jeu. Il est signalé comme C.
- **2026-11-11 — Royaume-Uni : du 8 avril au 17 septembre 1917, à 2 h GMT, confiance A.**
  - L'ordre en conseil pour l'île de Man, du même jour que celui du Royaume-Uni, a été lu dans la *London Gazette* (H7).
  - H2 et H8 donnent les mêmes dates pour le Royaume-Uni, mais pas le même numéro d'ordre (358 ou 362). L'écart de numéro est sans effet sur les dates.
- **2026-11-11 — Allemagne : du 16 avril au 17 septembre 1917, de 1 h 00 à 1 h 00 UTC, confiance A ; armée et territoires occupés, B.**
  - Deux bulletins officiels allemands de mars 1917 (H13, H14) concordent avec la base tz et l'Observatoire royal de Belgique.
  - Pour la troupe, on déduit son heure de celle qu'elle imposait aux civils occupés (H15, H17) ; aucun ordre militaire n'a été lu.
- **2026-11-11 — Belgique non occupée en 1917 : alignée sur la France, confiance C.**
  - L'Observatoire royal dit seulement « heure de Greenwich » pour la zone libre (H15).
  - La VRT (1916) et le tableau de 1918 (H15, H2) montrent l'alignement sur la France ; on le suppose pour 1917.
  - Sans effet sur le jeu, qui suit l'heure française.
- **2026-11-11 — Lieu par défaut : Ypres, 50,851° N ; 2,885° E.**
  - La valeur du plan (50,85° N ; 2,89° E) est juste à 350 m près.
  - On garde Ypres plutôt que le terrain, parce que l'emplacement exact du terrain de Bierne n'est pas établi, et que l'écart entre les deux ne se voit pas dans le ciel (1,9 min de temps solaire).
- **2026-11-11 — Valeurs de contrôle tirées de l'USNO plutôt que calculées à la main** : c'est un service de référence en domaine public, qui accepte les dates de 1917, et dont les heures civiles évitent le piège du jour astronomique d'avant 1925 (H25).
- **2026-11-11 — Vol d'essai : 1er septembre 1917, 15 h 00 heure légale, à Ypres**, qui redonne à peu près le soleil actuel (224°, 40°). Le 20 octobre ne le permet pas : le soleil n'y dépasse pas 29°.
