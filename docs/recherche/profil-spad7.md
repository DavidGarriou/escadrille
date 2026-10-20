# Profil d'aile du SPAD VII — fiche de recherche

- Identifiant : `profil-spad7` (fichier de polaires visé : `airfoils/spad.ron`, voir `format-avion.md`)
- Dernière mise à jour : 2026-10-20
- État : **en cours** ; profil réel **inconnu** ; profil comparable proposé en confiance **C**
- Avion concerné : [`spad7.md`](spad7.md)
- Méthode et statuts des sources (**[LU]**, **[RÉSUMÉ]**, **[PISTE]**) : voir `spad7.md` § 0.

## 1. La question
Quel profil portaient les ailes du SPAD S.VII ? Existe-t-il, pour ce profil ou pour un profil comparable, des données de soufflerie publiées (NACA, ACA/ARC britanniques, Eiffel, Göttingen) que le modèle de vol puisse utiliser ?

## 2. Ce que disent les sources sur le profil réel

| Constat | Source | Statut | Portée |
|---|---|---|---|
| **Aucune** cote de profil (coordonnées, épaisseur relative, cambrure) du S.VII n'a été trouvée | toutes les recherches de cette passe | — | Le profil réel reste inconnu |
| Différents profils d'aile auraient été essayés sur le S.VII sans être adoptés en série | A11 | [RÉSUMÉ], wiki | Le profil de série a pu varier, ou non : à vérifier |
| Les plans originaux du **SPAD XIII** auraient été détruits pendant la Seconde Guerre mondiale. Le constructeur d'une reproduction a relevé les cotes sur des appareils originaux (NASM, collection Cole Palen) | A10 | [RÉSUMÉ] | Rien n'est dit des plans du VII : ne pas extrapoler |
| Le SPAD serait « tout sauf facile » à basse vitesse, son **profil mince** provoquant souvent des décrochages brutaux | A12 | [RÉSUMÉ] ; page source mal identifiée (Memorial Flight ou NASM) | Qualificatif seulement ; aucune mesure |
| Un article relayé par un forum classe le SPAD parmi les avions (avec le S.E.5a et le Camel) à **profil très mince**, recherché pour la vitesse : l'avion « tomberait » en perdant sa vitesse et se poserait au moteur | A12 | [RÉSUMÉ], forum | Réputation (voir `spad7.md` § 6.3), pas une donnée |
| *Flugsport*, 1919 : comparaison de profils d'avions français, dont « Spad », Morane-Saulnier, Nieuport et Martinsyde | A8 | [RÉSUMÉ] (index des dessins de la revue *WW1 Aero*) | **La meilleure piste** pour la forme réelle du profil : relevé allemand d'après guerre, sans doute sur appareils capturés. Numéro et page non obtenus |

**Conclusion** : on sait seulement, et par des sources secondaires faibles, que le profil serait **mince**. C'est insuffisant pour une valeur de confiance B.

## 3. Données de soufflerie publiées : inventaire

### 3.1 Laboratoire Eiffel (Auteuil)
- **A5** — G. Eiffel, *Résumé des principaux travaux exécutés pendant la guerre au Laboratoire aérodynamique Eiffel, 1915-1918*, Librairie aéronautique (E. Chiron), Paris, [1919]. Une copie en accès intégral (Université du Michigan) est signalée sur HathiTrust (notice 001035741) [RÉSUMÉ]. **Son contenu n'a pas pu être vérifié** : on ignore s'il contient l'aile ou une maquette du SPAD. C'est la source primaire française la plus probable pour un essai lié au SPAD, à consulter en priorité.
- **A6** — G. Eiffel, *Nouvelles recherches sur la résistance de l'air et l'aviation faites au laboratoire d'Auteuil* (texte et atlas), essais de 1911 à 1913 [RÉSUMÉ]. Antérieur au SPAD VII, mais il contient les profils « Eiffel » de l'époque, ce qui peut servir de comparaison.

### 3.2 NACA : compilations « Aerodynamic Characteristics of Aerofoils » (domaine public)
- **A1** — NACA Report No. 93, *Aerodynamic Characteristics of Aerofoils*, 1921 (NTRS : date 1er janvier 1921 ; 83 p.). https://ntrs.nasa.gov/citations/19930091156 ; PDF : https://ntrs.nasa.gov/api/citations/19930091156/downloads/19930091156.pdf [RÉSUMÉ]. Compilation, sous une forme unifiée (coefficients absolus), des résultats publiés par plusieurs laboratoires américains et européens. Le rapport avertit lui-même que les résultats de laboratoires différents ne sont pas strictement comparables. **Liste des profils non vérifiée** (PDF inaccessible ; l'OCR est jugé illisible dans le résumé).
- **A2** — NACA Report No. 124, *Aerodynamic Characteristics of Aerofoils II* (suite du n° 93), NTRS : https://ntrs.nasa.gov/citations/19930091187 (date de 1923 sur NTRS, 1921 ailleurs : **à vérifier**) ; copie UNT : https://digital.library.unt.edu/ark:/67531/metadc65774/ [RÉSUMÉ].
- **A3** — Suites de la série : Reports n° 182, 244 et 286. Le n° 286 prolonge les n° 93, 124, 182 et 244 [RÉSUMÉ]. Identifiants NTRS non relevés.
- **A4** — NACA Report No. 304 (1929), Crowley et Green, *An Investigation of the Aerodynamic Characteristics of an Airplane Equipped with Several Different Sets of Wings*. Essais en vol plané d'un Sperry Messenger équipé tour à tour d'ailes R.A.F. 15, U.S.A. 5, U.S.A. 27 et Göttingen 387. https://ntrs.nasa.gov/citations/19930091373 [RÉSUMÉ]. Intérêt : des mesures **en vraie grandeur** sur le RAF 15, utiles pour juger de l'effet d'échelle sur des données de soufflerie de 1917.

### 3.3 Grande-Bretagne (ACA, puis ARC ; NPL)
- Les essais de profils de la série RAF au National Physical Laboratory sont publiés dans les *Reports and Memoranda* de l'Advisory Committee for Aeronautics [RÉSUMÉ]. Numéros précis non relevés.
- **A13** — Pankhurst, « N.P.L. Aerofoil Sections: Tabulated Details », ARC R&M 3311 (archives Cranfield), https://reports.aerade.cranfield.ac.uk/bitstream/handle/1826.2/3891/arc-rm-3311.pdf [RÉSUMÉ]. Profils conçus au NPL, postérieurs : utilité douteuse pour 1917.
- **A14** — L. Bairstow, *Applied Aerodynamics*, Longmans, 1920 [PISTE] : synthèse britannique d'époque, à vérifier.
- Aucune trace d'un essai britannique du profil SPAD (maquette ou aile) n'a été trouvée.

### 3.4 Allemagne (Göttingen)
- **A7** — M. Munk et E. Hückel, « Systematische Messungen an Flügelprofilen », *Technische Berichte* (Flugzeugmeisterei), 1917 ; A. S. Engels (comp.), *Flügelprofile : Untersuchungen der Modell-Versuchsanstalt für Aerodynamik der Jahre 1917/18* [RÉSUMÉ]. Profils allemands et profils d'industriels (Albatros, Junkers, Fokker…). **Aucun profil « Spad » signalé.**

### 3.5 États-Unis (McCook Field)
- L'essai du SPAD XIII à McCook Field en 1921 (Air Service Information Circular vol. III n° 286 ; voir `spad7.md` P12) porte sur les performances et les qualités de vol, pas sur le profil [RÉSUMÉ].

## 4. Profil comparable proposé (confiance C)

### 4.1 Proposition
- **Profil de base : RAF 15.** Confiance **C**.
- **Variante de sensibilité : RAF 6.** Confiance **C**.

### 4.2 Géométrie de ces profils (coordonnées lues)
Coordonnées lues dans le miroir GitHub de la base UIUC (A9 [LU]). L'épaisseur et la cambrure ci-dessous ont été **calculées ici** à partir de ces coordonnées (C).

| Profil | Points | Épaisseur maxi | Position | Cambrure maxi | Position | Remarque |
|---|---|---|---|---|---|---|
| RAF 15 | 31 | ≈ 6,5 % | x ≈ 0,15 à 0,20 | ≈ 2,6 % | x ≈ 0,3 | profil **mince** ; intrados en S près du bord d'attaque |
| RAF 6 | 25 | ≈ 10,0 % | x ≈ 0,3 | ≈ 4,6 % | x ≈ 0,3 à 0,4 | intrados presque plat ; bord de fuite épais (±0,0015) |

Calcul (C) : épaisseur = extrados − intrados à abscisse égale ; cambrure = (extrados + intrados) / 2. RAF 15 à x = 0,15 : 0,05203 + 0,01287 = 0,0649. RAF 6 à x = 0,30 : 0,09578 + 0,00402 = 0,0998.

### 4.3 Justification
1. **Cohérence avec le peu que l'on sait** : les sources secondaires décrivent un profil *mince* (§ 2). Le RAF 15 est mince ; le RAF 6, plus épais et plus cambré, encadre l'incertitude.
2. **Contemporanéité** : deux profils britanniques de la Grande Guerre, essayés au NPL. Le RAF 15 dispose en outre de mesures en vraie grandeur (A4).
3. **Disponibilité** : coordonnées publiques et lisibles ; rapports NACA du domaine public.
4. **Ce que le choix n'affirme pas** : rien n'indique que Béchereau ait utilisé un profil RAF. Le choix est purement **analogique**. Un profil Eiffel serait plus « français », mais aucune donnée Eiffel n'a pu être consultée (A5, A6 inaccessibles).

### 4.4 Précautions pour l'aérodynamicien
- **Données de polaires** : la présence du RAF 15 et du RAF 6 dans le Report 93 n'a **pas** été vérifiée (PDF inaccessible). À contrôler avant de citer A1 dans `airfoils/spad.ron`. À défaut, les polaires seront un calcul (`C:`, par exemple panneaux plus couche limite) à partir des coordonnées A9, recalées sur A4.
- **Effet d'échelle** : les essais de soufflerie de 1917 sont faits à bas nombre de Reynolds. La corde du S.VII n'est pas connue, mais elle est de l'ordre du mètre : environ 1,16 m de corde moyenne apparente (`spad7.md` § 2, C) et 1,400 m sur le XIII (`spad7.md` § 2.2). L'avion vole à plusieurs dizaines de m/s : son Reynolds est donc de plusieurs millions, bien plus que celui des essais (ordre de grandeur, C). Le C_L max et la brutalité du décrochage en dépendent.
- **Décrochage** : la réputation de « décrochage brutal » (`spad7.md` § 6.2, trait **à confirmer**) ne doit **pas** servir à régler la polaire. Elle sera, ou non, un résultat.
- **Biplan** : l'interférence entre les plans relève du modèle (écartement, décalage), pas du profil. Ces cotes manquent pour le S.VII ; voir les comparables du XIII dans `spad7.md` § 2.2.

## 5. Ce qu'il faudrait pour passer en confiance B, puis A
1. ***Flugsport* 1919** (A8) : identifier le numéro et la page (index *WW1 Aero*), puis consulter une collection numérisée (bibliothèques allemandes). Cela donnerait la **forme** du profil SPAD relevée à l'époque.
2. **Eiffel 1919** (A5) : vérifier sur HathiTrust si le volume contient l'aile ou une maquette du SPAD. On aurait alors des **polaires** françaises d'époque.
3. **Relevé sur appareil original** : Musée de l'Air (S.254, le plus accessible pour un projet français), San Diego Air & Space Museum, Canada Aviation and Space Museum, Museo Baracca (Lugo), Vigna di Valle (`spad7.md` P17). Un gabarit de nervure à plusieurs stations donnerait la forme réelle.
4. **Monographies avec plans** : Bruce, Windsock Datafile n° 8 ; Gronczewski et Fleischer ; 4+ Publications n° 19 (`spad7.md` P1 à P3). Leurs plans au 1/48 donnent parfois des coupes de nervures, mais à cette échelle la précision est faible.
5. **Notice de réglage du SPAD XIII** (`spad7.md` S38) : calages et réglages d'une cellule voisine.

## 6. Sources (consultées le 2026-10-20)
- **A1** — NACA, Report No. 93, *Aerodynamic Characteristics of Aerofoils*, 1921. https://ntrs.nasa.gov/citations/19930091156 [RÉSUMÉ]. Primaire (compilation), domaine public.
- **A2** — NACA, Report No. 124, *Aerodynamic Characteristics of Aerofoils II*, 1921 ou 1923. https://ntrs.nasa.gov/citations/19930091187 ; https://digital.library.unt.edu/ark:/67531/metadc65774/ [RÉSUMÉ].
- **A3** — NACA, Reports No. 182, 244 et 286 (suites de la série) [RÉSUMÉ, identifiants non relevés].
- **A4** — Crowley et Green, NACA Report No. 304, 1929. https://ntrs.nasa.gov/citations/19930091373 ; notice : https://core.ac.uk/works/24852094 [RÉSUMÉ].
- **A5** — G. Eiffel, *Résumé des principaux travaux exécutés pendant la guerre au Laboratoire aérodynamique Eiffel, 1915-1918*, E. Chiron, Paris, [1919]. https://catalog.hathitrust.org/Record/001035741 ; https://onlinebooks.library.upenn.edu/webbin/book/lookupid?key=ha001035741 [RÉSUMÉ].
- **A6** — G. Eiffel, *Nouvelles recherches sur la résistance de l'air et l'aviation faites au laboratoire d'Auteuil*, texte et atlas (essais de 1911 à 1913) [RÉSUMÉ, notice de libraire].
- **A7** — M. Munk et E. Hückel, « Systematische Messungen an Flügelprofilen », *Technische Berichte*, 1917 ; A. S. Engels (comp.), *Flügelprofile : Untersuchungen der Modell-Versuchsanstalt für Aerodynamik der Jahre 1917/18* [RÉSUMÉ ; PISTE].
- **A8** — *Flugsport*, 1919, comparaison de profils d'avions français (MS, Nieuport, Spad, Martinsyde…), signalée par l'index des dessins de *WW1 Aero* : https://ww1aeroinc.org/ww1-aero-drawing-index/ [RÉSUMÉ ; numéro et page à trouver].
- **A9** — UIUC Airfoil Coordinates Database (M. Selig), miroir GitHub `kennyjensen/uiuc_airfoil_database` : https://raw.githubusercontent.com/kennyjensen/uiuc_airfoil_database/master/raf15.dat et https://raw.githubusercontent.com/kennyjensen/uiuc_airfoil_database/master/raf6.dat [LU]. Base d'origine : https://m-selig.ae.illinois.edu/ads/coord_database.html (inaccessible).
- **A10** — Museum of Flight (Seattle), « SPAD XIII S13 », https://www.museumofflight.org/exhibits-and-events/aircraft/spad-xiii-s13 [RÉSUMÉ].
- **A11** — Wikipedia (en), « SPAD S.VII », https://en.wikipedia.org/wiki/SPAD_S.VII [RÉSUMÉ ; wiki : piste].
- **A12** — Résumés de recherche non attribués avec certitude : Memorial Flight, « SPAD XIII », https://www.memorial-flight.com/spad-xiii?lang=en, ou NASM, « Spad XIII "Smith IV" », https://airandspace.si.edu/collection-objects/spad-xiii-smith-iv/nasm_A19200001000 ; forum relayant un article sur les profils minces [RÉSUMÉ].
- **A13** — Pankhurst, ARC R&M 3311, https://reports.aerade.cranfield.ac.uk/bitstream/handle/1826.2/3891/arc-rm-3311.pdf [RÉSUMÉ].
- **A14** — L. Bairstow, *Applied Aerodynamics*, Longmans, Green & Co, 1920 [PISTE ; édition à vérifier].

## 7. Journal des décisions
- **2026-10-20 — Profil réel.** Aucune donnée trouvée. **Inconnu** ; seule la qualification « mince » est rapportée, par des sources secondaires faibles (A12).
- **2026-10-20 — Profil comparable.** **Retenu : RAF 15 (base) et RAF 6 (sensibilité), confiance C.** Raisons : cohérence avec un profil mince ; profils contemporains ; coordonnées publiques (A9) ; mesures en vraie grandeur disponibles pour le RAF 15 (A4). Aucune parenté historique avec le SPAD n'est affirmée.
- **2026-10-20 — Polaires.** Ne pas citer le Report 93 pour le RAF 15 ou le RAF 6 avant d'avoir vérifié leur présence. À défaut, polaires calculées (`C:`), raccordées à la plaque plane au-delà du décrochage (plan du M1, étape 3).
- **2026-10-20 — Priorités de recherche.** (1) *Flugsport* 1919 ; (2) Eiffel 1919 ; (3) relevé sur S.254 au Musée de l'Air.
