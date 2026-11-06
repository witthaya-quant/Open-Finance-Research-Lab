<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Pressure Points — Initial Problem Formalization

**Case ID:** OFRL-CM-001-CS-001  
**Formalization version:** 0.11.0  
**Source:** Jane Street, Pressure Points (November 2026)  
**Source URL:** https://www.janestreet.com/puzzles/pressure-points-index/  
**Access date:** 2026-11-05  
**Status:** INCOMPLETE — BLOCKED ON RULE SEMANTICS

## 1. Purpose of this note

This note records the visible structure of the original source and the questions that must be resolved before a formal model can be claimed. It is not a proposed solution and does not reconstruct the complete puzzle.

## 2. Source-observed facts

The official page is labelled “Pressure Points” and “November 2026 : Puzzle”. Inspection of the visible source representation identifies:
- One leading symbol, shown as “Ø”.
- A block of 75 uppercase-letter sequences.
- A separate block of 19 emoji clues.
- A terminal notation shown as “(11 12)”.

A structural check of the 75 visible letter sequences establishes the following properties:
- All 75 sequences are distinct.
- Each sequence contains uppercase letters in nondecreasing alphabetical order.
- The sequence block itself is in nondecreasing lexicographical order.
- The sequences contain 504 letters in total.
- The observed length distribution is:

| Sequence length | Count |
|---:|---:|
| 1 | 2 |
| 2 | 3 |
| 3 | 6 |
| 4 | 12 |
| 5 | 9 |
| 6 | 11 |
| 7 | 7 |
| 8 | 7 |
| 9 | 4 |
| 10 | 3 |
| 11 | 4 |
| 12 | 2 |
| 13 | 1 |
| 15 | 1 |
| 16 | 1 |
| 17 | 1 |
| 19 | 1 |
| **Total** | **75** |

These observations establish representation-level properties only. They do not establish what any sequence means or how the puzzle is solved.

No assumption is made that:
- The uppercase sequences are words, anagrams, labels, states, or encoded outputs.
- Each emoji maps to exactly one letter sequence.
- The sequence order is meaningful for the puzzle rules merely because the page displays it in sorted order.
- The leading symbol is a variable, zero, null marker, or instruction.
- The terminal notation is a coordinate, range, index pair, dimension, or another constraint.
- The puzzle has a unique answer.

## 3. Preliminary abstract representation

For purposes of organizing the questions only, the visible page can be represented as:

- A leading marker: (m).
- A finite ordered collection of visible letter sequences: (L = (L_1, L_2, ldots, L_n)), with (n = 75).
- A finite ordered collection of emoji clues: (E = (E_1, E_2, ldots, E_k)), with (k = 19).
- A terminal notation: (t = (11, 12)), preserving the source's displayed order.

The fact that each letter sequence is alphabetically canonicalized means it can be represented as a character multiset without losing the visible multiplicities. This is a representation observation, not proof that multiset semantics are the intended puzzle rule.

This is not yet a mathematical model of the puzzle. No mapping function, objective function, constraint set, or output domain is defined at this stage.

## 4. Unknown semantics and formalization blockers

| ID | Unknown | Why it matters | Evidence required |
|---|---|---|---|
| U-001 | Meaning of the leading marker | It may alter the input interpretation or be decorative | Authoritative rule explanation or solution reference |
| U-002 | Meaning of each letter sequence | Determines the domain and representation of the main data | Source-grounded definition |
| U-003 | Whether sequence ordering matters to the puzzle | Changes whether the data should be treated as a list, multiset, or set | Explicit rule or authoritative worked example |
| U-004 | Meaning and role of emoji clues | Determines the clue domain and any mapping relation | Explicit mapping rules or authoritative examples |
| U-005 | Relationship between letter sequences and emoji clues | Determines the constraint structure and search space | Formal rule statement |
| U-006 | Meaning of terminal notation “(11 12)” | May define a parameter, index, coordinate, or another condition | Authoritative explanation |
| U-007 | Required output and its schema | Needed to define success and test correctness | Explicit answer format and acceptance condition |
| U-008 | Completeness or uniqueness requirement | Determines whether one solution or all solutions must be found | Source statement or authoritative solution context |
| U-009 | How the 19 emoji clues partition and orient the 76 candidate entries | Determines whether four-member theme groups and cardinal positions are valid | Complete source-grounded grouping and a reproducible orientation rule |
| U-010 | Whether letter-count residuals between candidate words are intentional | Determines whether the title implies a defined letter-transfer or transformation operation | A rule that explains every residual across all groups and independently reproduces the output |

All ten items remain unresolved at version 0.11.0. H-017 and H-018 add a conditional candidate map and a source-led evidence audit, but do not close the source-rule, output-schema, or puzzle-intent blockers.

### 4.1 External research leads — hypotheses, not established rules

A review of public discussion and solver-status material adds a potentially useful structural lead. These sources are secondary and do not constitute Jane Street's authoritative rule specification or a worked proof.

**H-001 — Four cardinal vertices per emoji.** A public solver-status note describes a one-to-one correspondence between 76 visible marker/sequence entries and the 19 emoji clues interpreted as four cardinal vertices each. The count is suggestive:

- Let `M = {m}` be the leading marker and `L = {L_1, ..., L_75}` the letter-sequence entries.
- The visible entry collection is `X = M ∪ L`, so `|X| = 76` if the marker is treated as an entry.
- Let `I` be the 19 emoji clues and `D = {N, E, S, W}` the four cardinal directions.
- The proposed vertex collection is `V = I × D`, with `|V| = 19 × 4 = 76`.

The equality `|X| = |V|` is a cardinality match only. It does not prove that the marker belongs in the mapping, that every emoji denotes a four-point diamond, or that the mapping is bijective. Those are the exact properties that need to be derived and verified.

**H-002 — Terminal notation as output lengths.** The notation “(11 12)” may specify two output-token lengths. This is consistent with the externally reported candidate below, but the source does not define the notation in prose. It must not be treated as an established output schema.

**H-003 — Candidate final phrase.** A secondary solver-status page reports the candidate `ANTWERPSCHE DIAMANTKRING` and associates it with the Flemish Diamond. The words have lengths 11 and 12 respectively, matching the displayed terminal numbers. The page does not expose the complete derivation in the material reviewed. This is a candidate to test, not an OFRL-verified solution and not an official Jane Street solution.

**H-004 — Partial four-member semantic clusters.** Independent inspection of decoded candidate terms produces four plausible groups associated with four emoji themes:

| Emoji theme | Candidate four-member set | Letter-multiset audit |
|---|---|---|
| Compass | NORTH, EAST, SOUTH, WEST | The four source entries, taken together, contain exactly the same letter multiset as these four direction names (18 letters). The individual entries are not all exact one-to-one anagrams: one candidate has an additional E while another lacks E. |
| Game controller | TRIANGLE, SQUARE, CROSS, CIRCLE | The combined candidate entries have one additional S and one missing E relative to the four singular button names. |
| Baseball | FIRST, SECOND, THIRD, HOME | The combined candidate entries have one additional O and one missing H relative to the four base names. |
| Sailing | STARBOARD, BOW, STERN, PORT | The combined candidate entries have one additional A and one missing S relative to the four nautical terms. |

The emoji associations are plausible, but they are not an authoritative mapping. The compass cluster's aggregate equality is a useful exact invariant; the other three residual pairs are observations against the stated candidate words, not proof of a shared transformation. No original sequence list is reproduced here.

**H-005 — Letter-pressure / reallocation mechanism.** The residuals above may indicate a deliberate letter movement or substitution among four related terms, potentially connected to the title “Pressure Points”. This is only a hypothesis. It is equally possible that one or more candidate words or groupings are wrong. The hypothesis cannot be accepted unless one rule explains all 19 groups, including any empty or exceptional entry, and yields a reproducible output without hand-selected exceptions.

**What this partial reconstruction establishes:** four candidate emoji themes can be associated with recognizable four-position sets, and their positional conventions can be checked independently. It does not establish that these are Jane Street's intended groups, does not explain the residual letters, and does not establish the remaining 15 groups, the role of Ø, or the final answer.

**H-006 — Four candidate groups admit independently corroborated cardinal layouts.** The following four emoji themes each have a plausible four-term interpretation that fits a north/east/south/west diamond. The positional conventions are corroborated by external reference material, but the association of each emoji with a group and the role of these groups in Jane Street's intended extraction remain hypotheses.

| Emoji candidate | North | East | South | West | Candidate letter residuals |
|---|---|---|---|---|---|
| Compass | NORTH | EAST | SOUTH | WEST | Candidate source for NORTH contains one extra E; candidate source for EAST lacks one E; SOUTH and WEST are exact anagrams. Aggregate multiset equality holds over all four entries. |
| Game controller | TRIANGLE | CIRCLE | CROSS | SQUARE | Candidate source for the singular button name TRIANGLE contains one additional S (+S); candidate source for CIRCLE lacks E; CROSS and SQUARE are exact anagrams. |
| Baseball | SECOND | FIRST | HOME | THIRD | Candidate source for SECOND contains one extra O; candidate source for THIRD lacks H; FIRST and HOME are exact anagrams. |
| Sailing vessel | BOW | STARBOARD | STERN | PORT | Candidate source for BOW contains one extra A; candidate source for STERN lacks S; STARBOARD and PORT are exact anagrams. |

**Independent positional conventions:**
- Compass: the four cardinal directions are north, east, south, and west.
- Controller: the Sony DualSense manual's front-view diagram places Triangle at the top, Circle at the right, Cross at the bottom, and Square at the left. This is specific to the PlayStation-style face-button arrangement; the puzzle emoji itself is a generic game-controller symbol.
- Baseball: MLB's official field diagram places second base at the top of the diamond, first base at the right, home plate at the bottom, and third base at the left.
- Sailing: bow is the front, stern is the back, and when facing the bow, starboard is right and port is left. Mapping these relative vessel directions to N/E/S/W requires choosing the bow as the forward/North reference; it does not establish a geographic heading.

**Reproducible letter-count residual audit:** compare the candidate alphagram with the target term using character multisets, not edit distance or visual similarity. The observed residuals are:
- Compass: +E on NORTH and −E on EAST.
- Controller: +S relative to singular TRIANGLE and −E relative to CIRCLE.
- Baseball: +O on SECOND and −H on THIRD.
- Sailing: +A on BOW and −S on STERN.

Across these four partial groups, the residuals do not establish a common letter-transfer rule. Their net multiset is not zero. The fact that the positional categories form recognizable diamonds is a separate observation from the unexplained residual letters. Do not infer the remaining 15 groups or an extraction rule from these four examples.

The source alphagrams themselves are not copied into this document. A researcher can reproduce each comparison from the official puzzle source URL and the candidate terms stated above.

**H-007 — Southern Cross / Crux.** Four candidate entries match the four main stars of Crux:
- Top: GACRUX — the source alphagram has one extra C.
- East arm: MIMOSA — exact anagram.
- Foot: ACRUX — the source alphagram is missing U.
- West arm: IMAI — exact anagram.

The European Southern Observatory confirms Acrux, Mimosa, Gacrux, and Imai as the four main stars of Crux. An astronomy education reference describes Gacrux at the top, Acrux at the foot, Mimosa on the eastern arm, and Imai on the western arm of its chart. A Southern Cross navigation reference notes that the apparent sky orientation rotates around the celestial pole. Therefore these are chart-relative positions, not a fixed terrestrial N/E/S/W assignment; any cardinal interpretation would require an explicit orientation convention. The star emoji → Crux association remains an unverified interpretation of Jane Street's clue.

**H-008 — Clock face.** A candidate set is TWELVE (12 o'clock), THREE (3 o'clock), SIX (6 o'clock), and NINE (9 o'clock). The source alphagram for TWELVE has one extra H; the source alphagram for SIX has one extra A and is missing S; THREE and NINE are exact anagrams. The four positions are conventional clock positions, but the residuals and intended extraction rule remain unexplained.

**H-009 — Lotus / India.** The lotus may identify India, whose national flower is the lotus. A political map of India places the four candidate states at the proposed cardinal extremes:
- North: HIMACHAL PRADESH — source alphagram has one extra G.
- East: WEST BENGAL — exact anagram.
- South: TAMIL NADU — source alphagram is missing L.
- West: GUJARAT — exact anagram.

The Survey of India map and an Indian government reference support the geographic and national-symbol context. This supports a plausible theme mapping, not a rule supplied by Jane Street.

**H-010 — Warning / NFPA 704 hazard diamond.** The warning emoji may refer to the NFPA 704 hazard diamond, which has four quadrants:
- Top: FLAMMABILITY (often shortened to FIRE in this candidate set) — source alphagram for FIRE has one extra E.
- Right: INSTABILITY — exact anagram.
- Bottom: SPECIAL — source alphagram is missing S.
- Left: HEALTH — exact anagram.

The NFPA standard identifies the hazard-diamond system; a U.S. Department of Homeland Security training reference describes the four quadrants and their positions. The category is precisely *flammability*, not a separate “fire” label. `FIRE` is a candidate lexical surrogate for the top quadrant, not the standard's canonical field name; this weakens the exact semantic match. The puzzle-specific association is not proven.

**H-011 — Desert / Colorado and the Four Corners region (provisional).** A candidate cardinal-neighbour set around Colorado is:
- North: WYOMING — source alphagram has one extra N.
- East: KANSAS — exact anagram.
- South: NEW MEXICO — source alphagram is missing E.
- West: UTAH — exact anagram.

Colorado also borders Nebraska to the northeast and Oklahoma to the southeast, and meets Arizona at the Four Corners point. The Four Corners region includes high-desert terrain. This makes a desert/Colorado interpretation plausible but weaker than the other candidate themes: the emoji-to-region link is inferential, and the four cardinal neighbours are not the complete set of all Colorado neighbours. Treat H-011 as provisional.

**H-012 — Joystick directions.** A candidate set is UP (north), RIGHT (east), DOWN (south), and LEFT (west). The source alphagram for UP has one extra H; RIGHT and DOWN are exact anagrams; the source alphagram for LEFT is missing T. The four directional terms fit the joystick emoji directly, but the role of their residual letters in the full puzzle remains unknown.

**H-013 — Community-reported candidate readings.** The Puzzling Stack Exchange discussion reports further decoded examples including BUSQUETS, INIESTA, VIOLINS, MERCURY, and INSTABILITY, alongside candidates such as GUJARAT, KANSAS, UTAH, WEST BENGAL, and STARBOARD. It also reports near-matches to place names: WYOMING with an additional N, MONTANA with an additional D, NEW MEXICO missing E, and HIMACHAL PRADESH with an additional G. These are user-generated candidate readings, not an authoritative grouping or a worked derivation. The discussion itself says that the puzzle page gives no written rules and explicitly leaves the meaning of the title, Ø, the emoji clues, and “(11 12)” unresolved. MONTANA + D is currently unassigned to any candidate emoji group.

**H-014 — Music / Queen four-member candidate.** Four source alphagrams admit the candidate surnames DEACON, MERCURY, MAY, and TAYLOR, which are the classic four Queen members listed by the band's official site:
- DEACON — exact anagram of source `ACDENO`.
- MERCURY — exact anagram of source `CEMRRUY`.
- MAY — source `AMNY` contains one additional N.
- TAYLOR — source `AORTY` is missing L.

The Queen-member identity is independently corroborated by QueenOnline; the letter residuals are preserved exactly. This makes the music-emoji association plausible, but it does not prove Jane Street intended this grouping, establish the four cardinal positions, or explain the residual transfer. H-014 is a candidate semantic cluster only.

**H-015 — Football / four-player candidate.** Four source alphagrams admit the footballer-name candidates GARETH, INIESTA, BUSQUETS, and MESSI:
- GARETH — exact anagram of source `AEGHRT`; Gareth Bale is the candidate identity.
- INIESTA — exact anagram of source `AEIINST`.
- BUSQUETS — exact anagram of source `BEQSSTUU`.
- MESSI — source `EEIMSS` contains one additional E.

Official Real Madrid and FC Barcelona profiles corroborate possible identities for the names. However, `GARETH` is a given name while INIESTA, BUSQUETS, and MESSI are surnames; the letters do not identify Gareth Bale rather than another footballer named Gareth. The four-person set has no independently established common criterion or cardinal orientation, and the +E residual remains unexplained. H-015 is a low-confidence candidate semantic cluster only.

**H-016 — Computer / Diamond Model of Intrusion Analysis (CM-T0008 lead).** Four currently unassigned source sequences admit the four core features of the Diamond Model of Intrusion Analysis:
- **North / top: ADVERSARY** — source `AADERRSUVY` contains one additional U (+U).
- **East / right: CAPABILITY** — exact anagram of source `AABCIILPTY`.
- **South / bottom: VICTIM** — exact anagram of source `CIIMTV`.
- **West / left: INFRASTRUCTURE** — source `ACFINRRRSTTUU` is missing E (−E).

The original technical report by Caltagirone, Pendergast, and Betz defines an intrusion event through four core features—adversary, infrastructure, capability, and victim—connected by edges and arranged as a diamond. Its Figure 1 places adversary at the top, infrastructure at the left, capability at the right, and victim at the bottom. The Threat Intelligence Academy's official model page summarizes the same four-feature structure and cites Technical Report ADA586960 (5 July 2013). The published geometry therefore supports the proposed top/right/bottom/left assignment without inventing a new orientation convention.

A laptop emoji is semantically compatible with computer intrusion analysis, and the four target terms match an established four-vertex diamond model. This is stronger contextual and positional support than a generic set of computer-related words. However, the mapping from Jane Street's 💻 emoji to this particular model remains inferred, not source-stated. The group residual is +U/−E; it is not explained by the current global residual hypothesis. H-016 is a high-quality candidate hypothesis, not an accepted puzzle rule or a solved group.

### CM-T0008 — Preliminary lexical lead screen

After provisionally assigning the four H-016 source sequences, the unassigned inventory is 23 letter sequences plus Ø (24 visible items), alongside six emoji without a candidate group. The numerical equality 24 = 6 × 4 is consistent with, but does not prove, the proposed four-vertices-per-emoji model.

A local exploratory scan of the remaining alphabetized sequences used the installed `cmudict` Python package (version 1.1.3) as an English pronunciation-lexicon candidate generator. It produced exact single-word candidates including **PALISADES**, **HEAVEN**, **VIOLINS**, **GO**, **XI**, **MOO**, and **SUN**. **PUTTI** was independently checked as an exact character-multiset match but was not returned by that dictionary. The National Gallery glossary defines *putti* as winged infants appearing in religious and other artworks; this makes 🖼️ a possible semantic lead, but one word does not establish a four-member group.

Near-match candidates include **CONDUCTOR** (one missing R), **PERCUSSION** (one additional X), **EXCURSIONS** (one additional P), and the community-reported **MONTANA** (one additional D). Some can suggest art, music, geography, or other themes, but no complete quartet with independently supported positions has been derived. Short forms such as GO, XI, MOO, SUN, SW, T, and V are low-information lexical hits and must not be promoted to semantic assignments without stronger evidence.

This scan is deliberately non-exhaustive. CMUdict is an English pronunciation dictionary, not a comprehensive proper-name, multilingual, or phrase lexicon; its maintainers explicitly warn of possible errors, omissions, and inconsistencies. The Python wrapper is not a project dependency, no dictionary file or package code has been copied into the repository, and dictionary matches are leads only. Exact anagrams can arise by chance and do not prove emoji intent, cardinal position, or the global extraction mechanism.

### CM-T0008 — Structured family screening of remaining source sequences

To distinguish plausible semantic themes from lexical matches, four known four-member structures were screened against the 23 remaining source sequences. The screen used a character-multiset L1 distance:

\[
D(s,t)=\sum_{a\in\{A,\ldots,Z\}}\left|n_s(a)-n_t(a)\right|,
\]

where \(n_s(a)\) and \(n_t(a)\) are the counts of letter \(a\) in source sequence \(s\) and candidate term \(t\). For each four-term candidate set, an injective one-to-one assignment to four distinct remaining source sequences was chosen to minimize the sum of the four distances. This is a minimum-cost bipartite matching screen; it is not a solver and does not encode a puzzle rule. No acceptance threshold was defined in advance. The scores are used only to compare candidate lexical fit with the H-016 lead, whose four terms have aggregate distance 2.

| Candidate family | Best one-to-one source assignment | Minimum total distance | Independent structural evidence | Screen disposition |
|---|---|---:|---|---|
| Four Dragon Kings: AOGUANG, AOQIN, AORUN, AOSHUN | GO→AOGUANG (5); IX→AOQIN (5); MOO→AORUN (6); NSU→AOSHUN (3) | **19** | A four-king sea/dragon tradition is a possible semantic lead for 🐉. The candidate roster came from a secondary search result; direct page retrieval failed, so canonical names and cardinal correspondences were not verified in this review | **NOT ACCEPTED** — poor multiset fit and no verified roster/orientation source |
| Four Heavenly Kings: DHRITARASHTRA, VIRUDHAKA, VIRUPAKSHA, VAISHRAVANA | AAAABBIRRST→DHRITARASHTRA (8); V→VIRUDHAKA (8); AADEILPSS→VIRUPAKSHA (9); AEEHNV→VAISHRAVANA (9) | **34** | Japan Tourism Agency describes a Four Heavenly Kings statue group that guards the cardinal directions: Tamonten north, Zochoten south, Jikokuten east, Komokuten west | **NOT ACCEPTED** — strong cardinal semantics for a temple-like clue, but very poor lexical fit to this literal transliteration roster |
| Four U.S. swine-industry breeds: DUROC, HAMPSHIRE, LANDRACE, YORKSHIRE | CCDNOOTU→DUROC (5); AEILLNPSU→HAMPSHIRE (8); AADEILPSS→LANDRACE (7); GO→YORKSHIRE (9) | **29** | U.S. Livestock Genetics Export identifies these four breeds as the backbone of the U.S. swine industry | **NOT ACCEPTED** — plausible 🐷 theme, but no independent cardinal-order rule and poor lexical fit |
| Classical architectural orders: DORIC, IONIC, CORINTHIAN, COMPOSITE | GO→DORIC (5); IILNOSV→IONIC (4); AADMNNOT→CORINTHIAN (8); MOO→COMPOSITE (6) | **23** | University of Illinois identifies **five** classical orders: Doric, Ionic, Corinthian, Tuscan, and Composite | **NOT ACCEPTED** — selecting only four omits Tuscan without a puzzle-derived reason; no cardinal-order rule and poor lexical fit |

**Interpretation and limits.**
- The Dragon Kings candidate is not accepted: the roster and any directional association were not verified from an accessible source in this review, and the minimum multiset distance is 19.
- The Four Heavenly Kings are the strongest positional concept screened because the Japan Tourism Agency explicitly maps four guardians to north, south, east, and west. Their best lexical assignment still has total distance 34. This is evidence against this *literal transliteration roster* matching the remaining strings; it is not proof that 🏛️ cannot refer to a different temple, guardian, or architectural concept.
- The four pig breeds are a legitimate industry grouping, but the external source does not arrange them as cardinal vertices. Their minimum lexical distance is 29.
- Classical architecture is a useful false-positive control: the standard set contains five, not four, orders. Selecting four because the puzzle is hypothesized to use four vertices would be circular; its minimum distance is 23.

The assignments above are minimum-cost mappings among the 23 remaining strings for those exact candidate spellings only. They do not test all transliterations, epithets, languages, compound names, or alternative four-member sets. These are **tested-but-not-accepted leads**, not rejected meanings of the emojis. Their purpose is to preserve negative research evidence and prevent another agent from repeating the same unproductive literal roster without new evidence.

The lexical scan and structured family screen do not establish any additional four-member group. The live inventory therefore remains 13 provisional clusters / 52 provisional source assignments, with 23 sequences plus Ø and six emoji unassigned. The count 24 = 6×4 remains arithmetic alignment only.




### CM-T0008 — Expanded lexical ambiguity screen (2026-11-06)

The remaining 23-entry inventory was screened again with the locally installed CMU Pronouncing Dictionary wrapper, version 1.1.3. This is a lexical lead generator only; it does not define the meaning of any source sequence. Exact single-word hits include **PALISADES**, **HEAVEN**, and **VIOLINS**. **PUTTI** is an exact character-multiset match checked separately; it was not returned by this CMUdict build. The National Gallery glossary supports the art-related meaning of *putti*, but does not establish a four-member set.

A focused review of exact and one-character multiset-distance candidates exposes ambiguity:

| Remaining source alphagram | Candidate reading(s) | Source-to-target residual | Research disposition |
|---|---|---|---|
| `AADMNNOT` | MONTANA; MADONNA | +D against MONTANA; +T against MADONNA | Two different readings have the same distance of 1. The letters alone do not select a state or an art/religious term. |
| `AEILLNPSU` | PINELLAS | +U | One-character lexical lead; no four-member positional set established. |
| `AGIJLOOT` | GALIOTO | +J | Proper-name lead without a case-supported identity or category. |
| `CCDNOOTU` | COCONUT; CONDUCT; CONDUCTOR | +D; +O; −R, respectively | Multiple plausible readings at distance 1; no unique decoding. |
| `CEINOPRSSUX` | EXCURSIONS; PERCUSSION | +P; +X, respectively | Both are one-character leads with unrelated semantics; neither establishes a quartet. |

These findings supplement the earlier near-match leads **MONTANA**, **CONDUCTOR**, **PERCUSSION**, and **EXCURSIONS**. They show why a nearest-word result cannot be treated as a decoded answer: one source can admit multiple equally close readings with unrelated meanings. Short entries such as GO, XI, MOO, SUN, SW, T, and V remain low-information and are not promoted.

No lexical candidate from this screen is assigned to an emoji. In particular, MADONNA is an alternative reading of the same source alphagram previously discussed as MONTANA; it is not a new hypothesis for 👸 or 🖼️.

### CM-T0008 — Alternate directional-family and transliteration screens (2026-11-06)

The earlier screen tested the Four Dragon Kings, Four Heavenly Kings in Sanskrit-derived names, four U.S. swine breeds, and four selected classical architectural orders. To reduce the chance of a false negative caused solely by spelling choice, additional source-backed directional structures and name variants were tested against the same 23 unused source sequences.

For each four-term roster, every injective assignment of four distinct source sequences was examined. The score is the previously defined character-multiset L1 distance, summed over the four assignments. There are (23\times22\times21\times20=212{,}520) possible injective assignments per roster. The minimum and number of tied optima were checked by exhaustive enumeration. No acceptance threshold is defined; scores are comparative lexical-fit evidence only.

| Candidate structure / spelling | Minimum-cost assignment (one optimum) | Minimum total distance | Optimal-assignment count | Evidence and disposition |
|---|---|---:|---:|---|
| Four Symbols — Japanese names: SEIRYU, SUZAKU, BYAKKO, GENBU | SEIRYU→IX (6); SUZAKU→SW (6); BYAKKO→GO (6); GENBU→NSU (4) | **22** | 1 | The four directional guardians are externally documented, but the best literal fit is poor. 🐉 would identify only one member of a set whose others are a bird, tiger, and tortoise. **NOT ACCEPTED.** |
| Four Symbols — Chinese pinyin: QINGLONG, ZHUQUE, BAIHU, XUANWU | QINGLONG→GO (6); ZHUQUE→T (7); BAIHU→IX (5); XUANWU→NSU (5) | **23** | 2 | Same directional structure under a second naming form; no improvement sufficient to support a mapping. **NOT ACCEPTED.** |
| Four Symbols — English labels: AZURE DRAGON, VERMILION BIRD, WHITE TIGER, BLACK TORTOISE | AZURE DRAGON→AADMNNOT (9); VERMILION BIRD→IILNOSV (8); WHITE TIGER→IPTTU (9); BLACK TORTOISE→AGIJLOOT (9) | **35** | 4 | Canonical English labels fit the directional concept but not the remaining alphagrams. **NOT ACCEPTED.** |
| Four Heavenly Kings — Japanese names: JIKOKUTEN, ZOCHOTEN, KOMOKUTEN, TAMONTEN | JIKOKUTEN→IPTTU (8); ZOCHOTEN→CCDNOOTU (6); KOMOKUTEN→MOO (6); TAMONTEN→AADMNNOT (4) | **24** | 3 | Cardinal order is Tamonten north, Jikokuten east, Zochoten south, Komokuten west. The Japanese-name fit remains poor (the earlier Sanskrit-derived name screen scored 34). **NOT ACCEPTED.** |

The Shanghai Museum and the National Research Institute for Cultural Properties support the Four Symbols and their east/south/west/north assignments. The Japan Tourism Agency supports the Four Heavenly Kings and their north/east/south/west guardians. These references establish the external traditions and positions, not Jane Street's intended emoji mapping.

**Research disposition by unresolved emoji:**

| Emoji | Current evidence boundary |
|---|---|
| 👸 | No source-backed four-member cardinal roster established. |
| 🐷 | The four U.S. swine-breed roster remains a semantic grouping without a cardinal-order rule; its lexical screen scored 29. |
| 🖼️ | PALISADES and PUTTI remain lexical/context leads; MADONNA is an equally close alternative to MONTANA for one source. No coherent four-term set or positions are established. |
| 🧐 | No source-backed four-member cardinal roster established; isolated lexical hits are not promoted. |
| 🐉 | Four Dragon Kings scored 19; Four Symbols transliterations scored 22, 23, and 35. No roster is accepted. |
| 🏛️ | Four Heavenly Kings scored 34 in Sanskrit-derived names and 24 in Japanese names. The four-order architectural subset remains structurally incomplete because the cited classical set contains five orders. No roster is accepted. |

Historical pre-H-017 disposition (2026-11-06): the expanded negative screens had not established an additional group at that point. The inventory was still 13 provisional clusters / 52 assignments, with six emoji and 23 sequences plus Ø outside those earlier provisional groups. H-017 was added afterward as a separate conditional third-party map; it does not promote these earlier hypotheses to accepted facts. No global residual or extraction rule was inferred by the earlier screen.

### CM-T0008 — Source-led directional-roster and visual false-positive screens (2026-11-06)

The next screen tested additional source-backed candidate families for the picture and temple/building clues. The same 23-entry residual inventory was used. For each four-term roster, every injective assignment of four distinct source sequences to the ordered N/E/S/W targets was enumerated: 23P4 = 212,520 assignments per roster. The objective remained the previously defined character-multiset L1 distance. No acceptance threshold was defined in advance; a low score is not a correctness predicate, and tied optima are evidence of ambiguity rather than a reason to choose one assignment.

| Candidate family and target order (N / E / S / W) | Minimum total distance | Number of tied optima | External evidence and disposition |
|---|---:|---:|---|
| Four Anemoi: BOREAS / EURUS / NOTUS / ZEPHYRUS | 22 | 9 | The University of Chicago's Encyclopaedia Romana identifies the four cardinal winds and their directions. The connection from a generic framed-picture emoji to this mythological family is not established. PUTTI is a separate art-motif lexical lead, not evidence that the clue depicts the Anemoi. **NOT ACCEPTED** — weak clue-to-family linkage, poor lexical fit, and multiple optima. |
| Char Dham: BADRINATH / PURI / RAMESHWARAM / DWARKA | 25 | 3 | An Odisha Review article describes the four dhams at the north, east, south, and west; the Government of India's Ministry of Tourism independently lists the four destinations. **NOT ACCEPTED** — the directional temple tradition is source-supported, but the lexical fit is poor and the optimum is not unique. |
| Four Amnaya Peethams: JYOTIRMATH / PURI / SRINGERI / DWARKA | 24 | 2 | The Madhya Pradesh Government's Statue of Oneness page describes the four monastic centres at the cardinal corners of India. This roster overlaps geographically with Char Dham, so the building/temple clue does not distinguish the two traditions. **NOT ACCEPTED** — poor lexical fit and multiple optima. |

The three scores and tie counts were independently cross-checked with a second exhaustive enumeration. The source rosters have coherent external directional conventions, but no mapping is accepted: none supplies a unique lexical assignment, a puzzle-derived membership rule, or a global residual explanation. These screens do not establish that the picture or temple emoji represents any of these families.

**Monocle visual false-positive check for 🧐.** Asmodee UK's Monopoly rules explainer states that the familiar monocle association with the Monopoly mascot is a Mandela Effect; Rich Uncle Pennybags is monocle-free. This is useful negative evidence against identifying the emoji as the Monopoly mascot solely from the monocle. It does not identify the intended clue or rule out other monocle-related concepts.

Historical pre-H-017 disposition (2026-11-06): the expanded negative screens did not establish another accepted group at that point. H-017 and H-018 are recorded in the following sections and later update the candidate inventory; this earlier screen's negative findings remain valid for the specific rosters it tested. No hypothesis ID or Jane Street mapping was accepted by that earlier screen.

### CM-T0008 — H-017: Third-party 19-theme candidate map and independent strict audit (2026-11-06)

**Disposition: NOT ACCEPTED as Jane Street's solution.** H-017 records a complete candidate map published by an independent third party. It is a material research lead and a reproducible conditional exact-cover result; it is not an official Jane Street solution, and the original puzzle page does not state the candidate rule in prose.

**External source and provenance.** The reviewed material is the November 2026 folder in [Aditi-Sahay/Jane-Street-Puzzles](https://github.com/Aditi-Sahay/Jane-Street-Puzzles/tree/b8cfe34d46347b1ec7f894d63a4b74e255bdc517/2026/November), pinned to commit `b8cfe34d46347b1ec7f894d63a4b74e255bdc517`. The folder contains a README, a copied question representation, a solution note, and `solver.py`. The README states that its author submitted an answer and appeared on Jane Street's leaderboard; OFRL has not independently verified that claim. The repository and solution are therefore treated as third-party research, not an official Jane Street publication. No puzzle payload, screenshot, source code, or third-party file has been copied into OFRL.

**Candidate rule stated by the third party.** Each of 19 emoji themes is proposed to have four target terms. Each four-term group is required to produce exactly two exact character-multiset matches, one source sequence with one surplus character, and one source sequence missing one character. This 2/1/1 rule is a third-party hypothesis; it is not a rule statement located on Jane Street's puzzle page.

**Strict comparison used for the independent audit.** For a target term `w` and source sequence `s`, let `n_w(c)` and `n_s(c)` be the multiplicity of character `c`. Define `Delta(c) = n_s(c) - n_w(c)`, `P = sum_c max(Delta(c), 0)`, and `M = sum_c max(-Delta(c), 0)`. These definitions count surplus and deficit multiplicities separately. The only accepted states are: exact when (P=0,M=0); source surplus when (P=1,M=0); and target deficit when (P=0,M=1). A comparison with both positive and negative character differences is inadmissible, even when the source-side positive counts sum to one. This two-sided test is stricter than the comparator in the published third-party script.

**Source ordinal convention.** To avoid reproducing the puzzle's sequence payload, the 75 displayed letter strings are referred to by OFRL analysis identifiers `S001`–`S075`, in their displayed source order. The leading empty marker is denoted `Ø`. These ordinals are not Jane Street identifiers. Each cell records `target term → source ordinal (operation)`; `+X` means the source has one surplus X, `−X` means the target has one additional X, and `=` means exact equality. Terms appear in the order used by the pinned third-party source. Where that source supplies T/R/B/L slot labels, those are code labels, not proof of a universal geographic N/E/S/W convention.

| Emoji / candidate theme | Four target-to-source assignments in source order | Position metadata in the third-party source |
|---|---|---|
| 👸 Queen band | MAY → S037 (+N); MERCURY → S046 (=); DEACON → S018 (=); TAYLOR → S038 (−L) | No positions supplied (`?`) |
| 🧭 Compass | NORTH → S057 (+E); EAST → S039 (−E); SOUTH → S065 (=); WEST → S060 (=) | T/R/B/L |
| 🐷 Pigpen X-grid | S → S073 (+W); U → Ø (−U); V → S075 (=); T → S074 (=) | T/R/B/L |
| ⚽ Barcelona diamond | MESSI → S054 (+E); XAVI → S036 (−V); BUSQUETS → S040 (=); INIESTA → S027 (=) | T/R/B/L |
| 🎮 PlayStation face buttons | TRIANGLE → S025 (+S); CIRCLE → S042 (−E); CROSS → S048 (=); SQUARE → S029 (=) | T/R/B/L |
| 🖼️ Sistine Madonna | MADONNA → S009 (+T); SAINTBARBARA → S001 (−N); PUTTI → S068 (=); SAINTSIXTUS → S034 (=) | T/R/B/L |
| 🧐 Monopoly corners | FREEPARKING → S022 (+R); GOTOJAIL → S030 (=); GO → S064 (=); JAILJUSTVISITING → S032 (−G) | T/R/B/L |
| ⚾ Baseball diamond | SECOND → S043 (+O); FIRST → S061 (=); HOME → S056 (=); THIRD → S049 (−H) | T/R/B/L |
| 💻 Diamond Model | ADVERSARY → S007 (+U); CAPABILITY → S003 (=); VICTIM → S047 (=); INFRASTRUCTURE → S020 (−E) | T/R/B/L |
| 🐉 Beijing imperial altars | EARTH → S024 (+G); SUN → S071 (=); HEAVEN → S023 (=); MOON → S070 (−N) | T/R/B/L |
| 🕹️ Joystick / D-pad | UP → S066 (+H); RIGHT → S062 (=); DOWN → S050 (=); LEFT → S055 (−T) | T/R/B/L |
| 🏛️ Washington, D.C. diamond | NORTHPORTALESTATES → S010 (+E); NORTHEASTBOUNDARY → S004 (=); BLUEPLAINS → S028 (−B); PALISADES → S006 (=) | T/R/B/L |
| 🎶 Orchestra seating | PERCUSSION → S045 (+X); DOUBLEBASSES → S013 (=); CONDUCTOR → S041 (−R); VIOLINS → S067 (=) | T/R/B/L |
| ⭐ Crux / Southern Cross | GACRUX → S017 (+C); IMAI → S033 (=); ACRUX → S021 (−U); MIMOSA → S035 (=) | T/R/B/L |
| 🕓 Clock face | TWELVE → S052 (+H); THREE → S053 (=); SIX → S069 (−S); NINE → S058 (=) | T/R/B/L |
| ⛵ Ship | BOW → S016 (+A); STARBOARD → S005 (=); STERN → S059 (−S); PORT → S072 (=) | T/R/B/L |
| 🏜️ Colorado neighbours | WYOMING → S063 (+N); KANSAS → S012 (=); NEWMEXICO → S044 (−E); UTAH → S031 (=) | T/R/B/L |
| 🪷 India | HIMACHALPRADESH → S002 (+G); WESTBENGAL → S014 (=); TAMILNADU → S008 (−L); GUJARAT → S011 (=) | T/R/B/L |
| ⚠️ NFPA 704 hazard diamond | FIRE → S051 (+E); INSTABILITY → S015 (=); SPECIAL → S019 (−S); HEALTH → S026 (=) | T/R/B/L |

**Independent strict-audit result.** Two separately implemented character-multiset checkers (Python and JavaScript) independently reproduced the candidate assignments. For each of the 19 fixed target rosters, exactly one four-entry assignment satisfies the strict 2-exact / 1-surplus / 1-deficit condition. Across all rosters:
- 76 of 76 visible entries are consumed exactly once; no source ordinal is reused and no entry is left over.
- The 19 groups contain 38 exact matches, 19 source-surplus matches, and 19 target-deficit matches.
- The leading `Ø` marker is treated as an empty source sequence and matches target `U` as a one-character deficit (−U).
- The exact-cover result is conditional on the third-party's 19 target rosters and proposed 2/1/1 constraint. It is not uniqueness over all possible semantic rosters, nor proof that Jane Street intended these themes, slot labels, or rule.

**Conditional residual extraction.** Following the emoji/theme and target-term order in the pinned third-party source, surplus characters concatenate to `NEWESTROUGHEXCHANGE`; deficit characters concatenate to `LEUVENGHENTBRUSSELS`. These are observations conditional on that ordering and map, not yet an official extraction rule.

The deficit phrase contains LEUVEN, GHENT, and BRUSSELS, three cities named in the Flemish Diamond description already recorded in SRC-008; Antwerp is the omitted fourth city. The surplus phrase can be read as “NEWEST ROUGH EXCHANGE”. The Antwerpsche Diamantkring's own history describes its founding on 10 October 1929 in response to rough-diamond trade, and its About page identifies it as one of the four Antwerp exchanges (SRC-007 and SRC-058). A municipal heritage account dates it to 1930 rather than 1929 (SRC-059), so the exact year is source-dependent; both records place it after the older exchanges they identify. Together these facts make `ANTWERPSCHE DIAMANTKRING` a coherent candidate output. Its 11/12 word lengths also match the visible terminal notation `(11 12)`. Length coincidence alone is not proof, and the semantic chain remains an interpretation of third-party residuals rather than a Jane Street rule.

**Published-code defect — the third-party solver is not a valid independent verifier.** In `solver.py`, `compare_letters` computes `Counter(sequence) - Counter(word)`. When that positive difference sums to one, it returns a source-surplus result without checking the reverse difference `Counter(word) - Counter(sequence)`. For example, comparing target `MAY` with source `T` returns a false `+T` candidate even though M, A, and Y are absent. Source `V` is another false candidate. In the Queen roster, the published candidate-generation step therefore has three apparent choices (AMNY, T, V), not one. The script's `len(valid_solutions) != 1` guard consequently raises before the first theme is solved against the pinned input. The map above was not accepted on the authority of that script; it was reconstructed using the strict two-sided test.

The third-party README also says “79 letter strings”, while its `question.md` representation and `solver.py` input contain 76 marker/sequence entries and the script asserts an input length of 76. This is a source-documentation inconsistency. It does not change the 75-string inventory recorded from Jane Street.

**Semantic and positional evidence remains heterogeneous.**

| Candidate area | External cross-check | Current evidence boundary |
|---|---|---|
| 👸 Queen band | QueenOnline identifies the four classic members (SRC-024); the princess/queen cue is direct | Strong member-set context; no cardinal positions are provided. This competes with the earlier H-014 placement of the same Queen roster under 🎶; neither association is source-stated by Jane Street. |
| 🐷 Pigpen X-grid | A cipher reference places S/T/U/V in the X-shaped portion of the Pigpen alphabet (SRC-048) | Plausible direct wordplay and four-position layout; variants exist and no source establishes the puzzle-specific association. |
| ⚽ Barcelona diamond | A FIGC-hosted tactical thesis depicts a dynamic diamond involving Messi, Xavi, Iniesta, and Yaya Touré—not Busquets (SRC-052) | The published diagram does not corroborate this exact four-player roster. H-015's Gareth/Iniesta/Busquets/Messi roster is a competing lead; neither quartet is accepted. |
| 🖼️ Sistine Madonna | The National Gallery's account identifies the Virgin, Saint Sixtus, Saint Barbara, and two cherubs at the bottom (SRC-049) | Strong composition/member-set context; text confirms flanking and bottom placement but does not itself specify left/right assignment. |
| 🧐 Monopoly corners | A secondary board reference lists GO, Jail/Just Visiting, Free Parking, and Go to Jail as the four corner spaces (SRC-050) | Four corners are not automatically four cardinal vertices; the source does not establish a T/R/B/L or N/E/S/W convention. The earlier monocle-to-mascot association remains a false positive (SRC-044). |
| 🐉 Beijing imperial altars | Visit Beijing explicitly locates Earth north, Sun east, Moon west, and Heaven south; The Met describes the dragon as a symbol of imperial authority (SRC-046/SRC-047) | Strong directional and contextual alignment, but the emoji-to-imperial-altars association remains inferential. |
| 🏛️ Washington, D.C. diamond | D.C. sources identify Blue Plains at the District's southern tip, Palisades in the upper northwest, and North Portal Estates / Northeast Boundary as localities (SRC-053–SRC-057) | The District does not have official neighborhood boundaries; available neighborhood layers are descriptive/labeling products, not authoritative geometric vertices. The proposed four-location diamond is provisional. |
| 🎶 Orchestra seating | Berliner Philharmoniker explains that conductors choose arrangements; percussion is generally at the back, while double basses may be left or right and string layouts vary (SRC-051) | The candidate roster is plausible, but it does not have one universal four-point seating geometry. |

The source map also changes some earlier associations: Queen moves from the 🎶 hypothesis H-014 to 👸, while 🎶 becomes an orchestra-seating candidate; the football roster H-015 is replaced by a Barcelona-diamond roster that is not corroborated by the cited diagram. These are competing hypotheses, not silent corrections to the historical record. Prior H-001–H-016 records remain intact.

**Research disposition.** H-017 demonstrates a conditional 19-roster exact cover and a plausible output interpretation, but it does not close U-001–U-010, VAL-014, or VAL-015. The role of Ø, source-authorized membership of all 19 groups, the relationship between T/R/B/L and any actual cardinal frame, the extraction order, the meaning of “(11 12)”, and uniqueness/completeness conditions remain unproved. The case remains INCOMPLETE / BLOCKED — NO-GO FOR SOLVER IMPLEMENTATION. Do not copy or repair the third-party Python script in OFRL, and do not create a Rust solver or Cargo workspace until the formalization and correctness-predicate gates pass.

### CM-T0008 — H-018: Evidence-led positional challenge of H-017 (2026-11-06)

**Purpose and disposition.** H-018 is a post-H-017 semantic and positional stress test. It re-examines the six clues that remained open at the pre-H-017 checkpoint and the main H-017 conflicts. It is not a new official solution, does not change the 76-entry multiset audit, and does not promote any third-party theme into a Jane Street rule.

| Candidate area | Independent evidence | Position assessment | Disposition |
|---|---|---|---|
| 👸 Queen band | Sotheby's catalogue of Freddie Mercury's original c.1972 Queen Crest artwork identifies the zodiac signs: crab for Brian May, fairies for Freddie Mercury, and lions for John Deacon and Roger Taylor (SRC-061). QueenOnline independently lists the four classic band members (SRC-024). | The crest supports May at the upper crab and Mercury at the lower fairies. Deacon and Taylor share the two lion symbols, one on each side, but the catalogue does not identify which individual is left versus right. The two cardinal slots therefore remain swappable; the third-party code correctly leaves this roster's position labels unknown. | **MEMBER SET SUPPORTED; CARDINAL ASSIGNMENT INCOMPLETE.** Do not claim a unique four-slot mapping. |
| ⚽ Barcelona diamond | A 2010/11 tactical retrospective describes Busquets, Xavi, Iniesta, and Messi as forming a box or asymmetric diamond. It describes Busquets as the deeper anchor, Xavi on the right and deeper, Iniesta on the left and higher, and Messi dropping in front of Xavi (SRC-060). | This is consistent with the H-017 target order MESSI / XAVI / BUSQUETS / INIESTA as one representative phase, but the same source stresses rotations and fluidity. It is contextual secondary tactical analysis, not a fixed geometric diagram or Jane Street rule. The earlier Gareth/Iniesta/Busquets/Messi cluster H-015 is a competing lexical reading, not silently deleted. | **QUARTET CONTEXT STRENGTHENED; POSITIONAL SUPPORT CONDITIONAL.** The FIGC diagram cited in SRC-052 depicts a different season/shape and is not treated as proof against the 2010/11 quartet. |
| 🖼️ Sistine Madonna | The National Gallery identifies the Virgin, Saint Sixtus, Saint Barbara, and two cherubs (SRC-049). The Royal Collection Trust's reproduction record explicitly places Sixtus left, Barbara right, and the cherubs at lower centre (SRC-062). | The H-017 order MADONNA / SAINT BARBARA / PUTTI / SAINT SIXTUS is consistent with top-or-centre, right, bottom, left respectively. The Virgin is central and elevated rather than a mathematically exact top vertex; this is pictorial composition, not a geographic compass. | **COMPOSITION AND RELATIVE POSITIONS SUPPORTED.** Emoji intent remains contextual, not author-confirmed. |
| 🐷 Pigpen X-grid | The common Pigpen cipher X-grid variant places S at top, U at right, V at bottom, and T at left (SRC-048). | These positions match the H-017 S / U / V / T order exactly. Pigpen alphabets have variants, so the cited layout establishes a conventional variant, not which variant Jane Street intended. | **COMMON VARIANT FITS; PUZZLE INTENT UNPROVEN.** |
| 🐉 Beijing imperial altars | Visit Beijing describes the Earth Altar north, Sun Altar east, Heaven Altar south, and Moon Altar west; The Met supplies cultural context for the dragon/imperial association (SRC-046/SRC-047). | The four altar positions match the H-017 EARTH / SUN / HEAVEN / MOON top/right/bottom/left ordering. The link from the dragon emoji to imperial altars remains interpretive. | **DIRECTIONAL ROSTER STRONGLY SUPPORTED; EMOJI LINK INFERENTIAL.** |
| 🏛️ Washington, D.C. diamond | The North Portal Estates Civic League describes its area as at the District's northernmost corner (SRC-063); Mapcarta's OSM-derived record describes Northeast Boundary as the easternmost neighborhood (SRC-064); D.C. government locates Blue Plains at the southernmost tip (SRC-054); D.C. Planning places Palisades in the upper northwest quadrant (SRC-065). | The four locality labels support an approximate top/right/bottom/left arrangement around the District. The D.C. Office of Planning explicitly states that the District has no official neighborhood boundaries and that it does not attempt to draw them (SRC-066). Accordingly, this is an approximate locality/label geometry, not a formally defined polygon, exact cardinal-extreme proof, or official four-vertex boundary. | **APPROXIMATE GEOGRAPHIC FIT; NOT A STRICT GEOMETRIC PROOF.** |
| 🧐 Monopoly corners | Hasbro identifies the distinctive board design and its four corner squares as part of the Monopoly property (SRC-067). The independent board reference lists GO, Jail/Just Visiting, Free Parking, and Go to Jail as the four classic corner spaces (SRC-050). | In the conventional square-board orientation, the H-017 clockwise corner order Free Parking → Go to Jail → GO → Jail/Just Visiting maps to top/right/bottom/left after an analyst-imposed 45-degree clockwise rotation of the board. This is a coherent geometric transformation, but the rotation is not stated by Jane Street. Asmodee's false-positive check still matters: the Mr. Monopoly mascot does not have a monocle (SRC-044), so a direct monocle-to-mascot identification is unsupported. | **CORNER GEOMETRY CONDITIONAL; EMOJI-TO-MONOPOLY LINK NOT ACCEPTED.** |
| 🎶 Orchestra seating | Berliner Philharmoniker explains that conductors choose the seating layout, percussion is generally at the back, and string arrangements vary (SRC-051). | The source supports only a general back/front and variable lateral arrangement; it does not establish a universal PERCUSSION / DOUBLE BASSES / CONDUCTOR / VIOLINS four-vertex order. A reviewed stage-acoustics PDF search excerpt described several arrangements, but its page screenshot was unavailable during this check; no additional figure-level claim is relied upon. | **NO UNIVERSAL FOUR-POINT LAYOUT ESTABLISHED.** Keep the orchestra grouping provisional. |

**Cross-theme conclusion.** H-018 improves the evidence grade for the Barcelona quartet, Sistine Madonna composition, Pigpen X-grid convention, Beijing altar directions, and approximate D.C. localities. It also identifies exactly why the Queen two-lion positions remain non-unique, why the Monopoly corner transform is conditional and its monocle association is a false lead, and why orchestra seating is not fixed. The third-party roster map therefore has complete *source-entry coverage* under its own 2/1/1 hypothesis, but it does not establish 19 source-authorized semantic groups or 76 uniquely supported cardinal positions. Queen still has a left/right swap ambiguity; the 🧐 clue-to-Monopoly association is not accepted; and several positions are conventions or approximate geometries rather than fixed N/E/S/W points.

**CM-T0008 disposition.** The approved screening task is complete: the six previously open emoji have been investigated, the candidate/negative outcomes and positional limits are recorded, and no unsupported candidate is promoted to fact. This closes the *screening task* only. It does not close the Pressure Points case, U-001–U-010, VAL-014/VAL-015, or the no-go solver gate. Those remain controlled by CM-T0009–CM-T0011 and the case closure criteria.

### Secondary-source status review — 2026-11-05 (dated pre-H-017 checkpoint)

The additional public sources reviewed in this session do not close a formalization blocker:

- Jane Street's current puzzle page lists correct submissions as of 2026-11-02. This confirms submitted answers were accepted, but the reviewed puzzle page does not provide a worked derivation.
- Kaia's current research note states that the author is establishing the exact 76-entry to 19 × 4 cardinal-vertex bijection and names `ANTWERPSCHE DIAMANTKRING` as the candidate output. The page reviewed does not expose the derivation, intermediate mapping, or independent verification.
- Griffin Pinney's page reports a leaderboard placing and says “Notes to come in December”. It is a solver-status signal, not a mathematical explanation available for verification in this review.
- The Puzzling Stack Exchange page is a user-generated question that records candidate anagrams and near-place-name residuals and asks what the title, Ø, emoji clues, and terminal notation mean. It is not a complete worked solution.
- QueenOnline lists Freddie Mercury, Brian May, Roger Taylor, and John Deacon as the classic band members; this corroborates H-014's member identities, not the music-emoji association or cardinal order.
- Real Madrid's official Gareth Bale profile and FC Barcelona's official profiles for Iniesta, Busquets, and Messi corroborate the player identities in H-015; these references do not establish the candidate grouping or cardinal order.
- The Jane Street Archive search result displays November “Pressure Points” followed by “Puzzle Solution” text. The search result does not expose an anchor destination; the directly opened Archive HTML is a stale cached representation beginning at August 2026. No direct solution URL or solution body was retrieved.

At this 2026-11-05 review checkpoint, the candidate phrase was externally reported but **not independently verified by OFRL**. A correct-submission list, search-result label, or solver-status statement could not replace a complete derivation. H-017, documented separately above from a pinned third-party repository reviewed on 2026-11-06, later adds a reproducible conditional candidate map; it does not authenticate the third-party submission claim or establish Jane Street's intended groups.

### Cross-group evidence boundary

H-007 through H-012 expanded the initial four-cluster inventory to ten themes; H-014 and H-015 brought the recorded candidate inventory to twelve. Their contextual sources corroborate selected facts and conventions but do **not** establish Jane Street's intended emoji mappings or a common residual rule. The CM-T0007 audit separately assessed those twelve clusters without promoting any to fact. H-016 is a new CM-T0008 lead for 💻, based on the four vertices of the published Diamond Model of Intrusion Analysis; it is not part of the completed twelve-cluster CM-T0007 audit.

At the pre-H-017 checkpoint, H-016 brought the provisional inventory to 13 candidate clusters / 52 source assignments, leaving 23 sequences plus Ø and six emoji without candidate rosters. This remains the historical CM-T0008 intermediate state. H-017 records a separate third-party candidate map covering 19 rosters / 76 entries conditionally; it does not retroactively convert earlier hypotheses into accepted facts or prove the 19×4 interpretation.

The residuals from the first twelve clusters remain as audited in CM-T0007; H-016 adds +U for ADVERSARY and −E for INFRASTRUCTURE. Those 52 provisional assignments are a historical intermediate audit, not the H-017 candidate map. Under H-017's third-party rosters the independent strict audit instead finds 38 exact / 19 source-surplus / 19 target-deficit matches and the two conditional extraction strings recorded in H-017. The relationship between these strings and Jane Street's intended rule remains unproved.

### CM-T0007 — Candidate-cluster independent audit (2026-11-05)

**Audit purpose.** Test the twelve recorded candidate clusters against the complete visible sequence inventory, correct the singular/plural and terminology ambiguities, and grade semantic fit separately from positional support. This is an audit of candidate hypotheses, not an acceptance of Jane Street's intended solution.

**Method.** For each proposed target term, uppercase it, remove spaces, and compare character-frequency multisets with the corresponding alphabetized source sequence. For each letter \(c\), record:
- **surplus**: source count minus target count when positive;
- **deficit**: target count minus source count when positive.

No edit-distance normalization, silent spelling change, or unrecorded pluralization is permitted. The only explicit semantic alias below is the NFPA candidate FIRE for the standard's canonical top-quadrant label *flammability*; that alias is marked as weaker evidence.

**Inventory reconciliation.** The official source contains 75 distinct, lexicographically ordered alphabetized sequences totaling 504 letters, plus the leading Ø marker and 19 emoji clues. The twelve candidate clusters consume 48 distinct source sequences; all 48 occur in the official sequence list and none is reused. The audit therefore leaves 27 source sequences plus Ø, or 28 visible items, unassigned. Seven emoji clues are not represented by a candidate cluster. The arithmetic identity \(28=7\times4\) is consistent with H-001's proposed four vertices per remaining emoji, but it does not prove that Ø is a mapped entry, that each emoji has four vertices, or that the 19×4 bijection is intended.

**Lexical residual summary.** Against the 48 candidate target terms as stated below:
- 25 source/target pairs are exact anagrams.
- 23 pairs have one or more character residuals.
- Across those 23 pairs there are 13 surplus letters and 11 deficit letters. The AIX/SIX pair contributes both one surplus (+A) and one deficit (−S).
- After cancelling identical surplus and deficit letters across groups, the net residual is **+A×2, +C, +G, +H, +N×2, +O, −L×2, −S×2, −T, −U**. This is a net surplus of two characters; no common global transfer/extraction rule follows from this aggregate.

| Emoji candidate | Candidate N / E / S / W terms and source residuals | Net group residual | Semantic fit | Positional evidence | Audit disposition |
|---|---|---|---|---|---|
| 🧭 Compass | NORTH (+E), EAST (−E), SOUTH (exact), WEST (exact) | balanced | High: clue and four terms directly denote compass directions | High: N/E/S/W are intrinsic to the terms | Strongest candidate; author intent still unproven |
| 🎮 Controller | TRIANGLE (+S), CIRCLE (−E), CROSS (exact), SQUARE (exact) | +S / −E | Moderate–high: four PlayStation-style face-button names; emoji is generic, not brand-specific | High conditional on the Sony face-button layout | Candidate layout supported; keep TRIANGLE singular and preserve +S |
| ⚾ Baseball | SECOND (+O), FIRST (exact), HOME (exact), THIRD (−H) | +O / −H | High: four baseball-diamond bases/positions | High: standard diamond diagram supplies top/right/bottom/left | Strong candidate; residual rule unresolved |
| ⛵ Sailing | BOW (+A), STARBOARD (exact), STERN (−S), PORT (exact) | +A / −S | Moderate–high: four vessel-relative directions/parts | Conditional: right/left are defined while facing the bow; mapping bow to North is an added reference-frame choice | Plausible, but not an intrinsic geographic cardinal mapping |
| ⭐ Crux / Southern Cross | GACRUX (+C), MIMOSA (exact), ACRUX (−U), IMAI (exact) | +C / −U | Moderate: four main stars of Crux fit a star/constellation clue | Unestablished as fixed N/E/S/W: east/west are chart-relative and the apparent sky orientation rotates | Retain star-identity hypothesis; do not treat chart directions as proven cardinal vertices |
| 🕓 Clock | TWELVE (+H), THREE (exact), SIX (+A / −S), NINE (exact) | +H, +A / −S | High: four conventional clock labels | High: 12/3/6/9 form the expected four clock positions | Strong candidate; residual rule unresolved |
| 🪷 Lotus / India | HIMACHAL PRADESH (+G), WEST BENGAL (exact), TAMIL NADU (−L), GUJARAT (exact) | +G / −L | Moderate–high: lotus–India connection is contextually supported | Moderate: map supports broad north/east/south/west placement, not an exact geometric vertex rule | Plausible geographic candidate; state-selection/orientation rule remains unproven |
| ⚠️ NFPA 704 | FIRE (+E), INSTABILITY (exact), SPECIAL (−S), HEALTH (exact) | +E / −S | Moderate: warning emoji can suggest a hazard diamond; FIRE is not the canonical term | High for the standard quadrant arrangement: top flammability, right instability, bottom special, left health | Retain as a qualified candidate; record FIRE as a non-canonical shorthand |
| 🏜️ Colorado / Four Corners | WYOMING (+N), KANSAS (exact), NEW MEXICO (−E), UTAH (exact) | +N / −E | Low: desert → Four Corners → Colorado is inferential | Moderate for the four selected cardinal neighbours, but not a complete border set: Nebraska is northeast, Oklahoma southeast, and Arizona touches at Four Corners | Provisional / weakest geographic candidate; do not promote without a stronger emoji-to-region rule |
| 🕹️ Joystick | UP (+H), RIGHT (exact), DOWN (exact), LEFT (−T) | +H / −T | High: four directional controls directly fit a joystick clue | High: the words themselves define the four directions | Strong candidate; residual rule unresolved |
| 🎶 Queen | DEACON (exact), MERCURY (exact), MAY (+N), TAYLOR (−L) | +N / −L | Moderate–high: QueenOnline corroborates the four classic-member surnames | Unestablished: no source or rule assigns these four members to N/E/S/W | Retain as a semantic member-set candidate only; no cardinal mapping |
| ⚽ Footballers | GARETH (exact), INIESTA (exact), BUSQUETS (exact), MESSI (+E) | +E | Low: football is a broad theme and GARETH is an underdetermined given name | Unestablished: no common quartet rule or N/E/S/W order established | Low-confidence lead; do not accept Gareth Bale or a four-player grouping as uniquely identified |

**Context-source limits.** SRC-009 supports the PlayStation-style face-button layout, not that the generic controller emoji uniquely means Sony. SRC-010 supports the conventional baseball-diamond positions. SRC-011–SRC-013 support nautical and cardinal-direction conventions, not a ship's geographic heading. SRC-014 confirms the Crux star identities; SRC-015 is chart context, and SRC-029 documents the changing apparent sky orientation. SRC-016–SRC-017 support Indian geography and the lotus symbol. SRC-018–SRC-019 support NFPA 704 and its canonical quadrant names. SRC-020–SRC-021 support Colorado/Four Corners context but also expose the omitted diagonal neighbours. SRC-024 supports Queen member identities. SRC-025–SRC-028 support possible footballer identities but do not disambiguate the given name GARETH. No contextual source establishes Jane Street's intended emoji-to-group mapping.

**Result.** CM-T0007 is complete as an audit task: all twelve candidate clusters, 48 source assignments, and letter residuals have been independently recounted and the semantic/positional evidence has been graded separately. This does **not** solve the case. H-001 through H-015 remain hypotheses; H-005 is not established; U-001 through U-010 remain open; VAL-014 and VAL-015 remain BLOCKED; no method family, correctness predicate, or solver is authorized.




**Evidence required before accepting any of H-001 through H-017:**
1. Explain the semantic role of the leading marker and account for all 76 entries.
2. Define the four positions and show why each emoji produces exactly four relevant vertices.
3. Provide a reproducible mapping from every source entry to exactly one vertex, with no omissions or collisions.
4. Explain the relation between the letter multiset of each entry and its assigned vertex.
5. Derive the terminal notation and output format from the puzzle rather than from answer-length coincidence alone.
6. Establish whether the output is unique; if so, prove or independently verify uniqueness.
7. Check the complete construction against the source and independently verify the final phrase.
8. Identify all 19 emoji groups, including the status and meaning of Ø, without leaving unmatched entries.
9. Derive one consistent letter-transformation rule that accounts for all observed residuals across every group, or reject H-005.
10. Derive the cardinal orientation of each group and show how those positions lead to the output.
11. Distinguish a source-backed category convention from the interpretation of an emoji; corroboration of a star chart, map, clock, controller, or hazard label is not proof of puzzle intent.
12. Reconcile every local character residual and the leading Ø under one explicit global rule, or document why a proposed group is false.
13. Independently validate community-reported readings against the primary input, and distinguish those readings from the missing global mapping.

Historical pre-H-017 inventory checkpoint (2026-11-06): the CM-T0007 audit covered twelve candidate clusters across H-004, H-006, H-007 through H-012, H-014, and H-015. During CM-T0008, H-016 added a thirteenth provisional cluster for 💻 based on the Diamond Model of Intrusion Analysis. At that checkpoint there were 13 candidate clusters and 52 distinct provisional source assignments, with 23 source sequences plus Ø outside those provisional groups and six emoji without candidate rosters. The first twelve groups had 25 exact and 23 residual-bearing pairs; H-016 added two exact and two residual-bearing pairs, giving 27 exact and 25 residual-bearing pairs overall. Candidate semantic fit and cardinal orientation remained unequal: Crux was chart-orientation dependent; Colorado remained weak; Queen had no cardinal ordering; football was underdetermined by GARETH; and H-016 was source-grounded as a model but not tied to Jane Street's emoji by an explicit rule. H-017 and H-018, recorded above this historical checkpoint in this document's current reading order, later add the conditional 76-entry candidate map and positional review. They do not resolve U-001 through U-010, the global residual transformation, output extraction, or uniqueness/completeness. No algorithm family is selected and no solver is authorized by this update.

## 5. Candidate model structure — intentionally unselected

A later formal specification may define:
- Input domain and normalization rules.
- Decision variables or state representation.
- Constraint set.
- Objective function, if the task is an optimization problem.
- Output domain and acceptance predicate.
- Soundness, completeness, optimality, or uniqueness obligations where applicable.

No candidate family is selected yet. SAT/SMT, constraint programming, graph methods, search, string methods, and optimization remain unassessed alternatives, not design decisions.

## 6. Research questions

1. Can every visible element be assigned a precise semantic role from authoritative instructions?
2. What is the minimum formal representation that preserves all puzzle-relevant information?
3. What conditions define a valid output?
4. Does the task require finding one valid output, all valid outputs, an optimum, or a proof?
5. Can any properties be derived analytically before implementation?
6. Which verification claims can be established independently of a solver?

## 7. Method-selection gate

Method selection is BLOCKED until U-001 through U-007 and U-009 through U-010 are resolved sufficiently to define a correctness predicate. U-008 must also be resolved before any completeness or uniqueness claim is made.

If source clarification is unavailable, the project must either:
- Keep the case as an explicitly incomplete formalization exercise; or
- Pause this pilot and propose a different case with sufficiently specified rules.

The team must not invent missing rules to make a solver feasible.

## 8. Source and rights boundary

Original source: Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/).

Secondary research leads, accessed 2026-11-05:
- [Puzzling Stack Exchange discussion](https://puzzling.stackexchange.com/questions/141272/jane-streets-recent-puzzle-pressure-points-nov-2026) — exploratory observations and questions; not an authoritative rule statement.
- [Kaia's current research notes](https://www.kaiaz.me/now/) — reports work toward a 76-to-(19 × 4) cardinal-vertex bijection and the candidate phrase; the reviewed page does not provide the full derivation.
- [Griffin Pinney, November 2026 Pressure Points](https://griffinpinney.com/puzzles/jane_street/2026-11/) — reports a leaderboard placing and says notes are forthcoming in December; no worked derivation was present in the reviewed page.
- [QueenOnline, The Band](https://www.queenonline.com/the_band) — verifies the four classic Queen member names used in H-014.
- [Real Madrid, Gareth Bale](https://www.realmadrid.com/en-US/the-club/history/football-legends/gareth-bale/) and [FC Barcelona, Iniesta](https://www.fcbarcelona.com/en/football/barca-legends/players/4231989/iniesta), [Busquets](https://www.fcbarcelona.com/en/card/3569210/sergio-busquets), and [Messi](https://www.fcbarcelona.com/en/card/2214377/leo-messi) — corroborate the player identities used in H-015.
- [Antwerpsche Diamantkring — About](https://diamantkring.be/about/) — confirms the organization is an Antwerp rough-diamond bourse and one of Antwerp's four diamond exchanges; entity corroboration only, not proof of the puzzle answer.
- [Flemish Government / BELSPO, GARMON Final Report](https://omgeving.vlaanderen.be/sites/default/files/2022-02/ENG_GARMON_final_report_nov_2020.pdf), p. 45 — describes the Flemish Diamond as four agglomerations (Brussels, Ghent, Antwerp, Leuven) at the corners of an abstract diamond; contextual geography only, not puzzle-rule evidence.
- [European Southern Observatory, Navigating the stars: the stories behind the Southern Cross](https://www.eso.org/public/blog/navigating-the-stars/) — confirms the four main stars of Crux.
- [AstroSpeedrun, The far southern sky](https://astrospeedrun.com/learn/o1/o1-6/) — describes the conventional chart positions of Gacrux, Acrux, Mimosa, and Imai; chart-orientation context only.
- [Survey of India, Political Map of India](https://surveyofindia.gov.in/pages/political-map-of-india) — authoritative map reference for the candidate state positions.
- [Embassy of India, Ashgabat, Information on India](https://eoiashgabat.gov.in/info-on-india.php) — identifies lotus as India's national flower.
- [National Fire Protection Association, NFPA 704](https://www.nfpa.org/codes-and-standards/nfpa-704-standard-development/704) — identifies the NFPA hazard-diamond system.
- [U.S. Department of Homeland Security, Hazardous Materials Facility Markings](https://cdp.dhs.gov/shared/se/courses/default/AWR-358%20dL%20040921/groups/119.html) — describes the four NFPA 704 quadrants and their conventional positions.
- [State Maps, Colorado](https://www.state-maps.org/colorado-map/) — secondary reference for Colorado's cardinal neighbouring states; notes additional northeast/southeast neighbours.
- [U.S. Department of Energy, Atomic Legacy Discovery Guide: Colorado Plateau](https://www.energy.gov/lm/atomic-legacy-discovery-guide-colorado-plateau) — describes the Four Corners high-desert region as contextual geography only, not proof of the emoji interpretation.
- [Threat Intelligence Academy, Diamond Model of Intrusion Analysis](https://www.threatintel.academy/diamond/) — cites Caltagirone, Pendergast, and Betz's Technical Report ADA586960 (5 July 2013) and describes the four core intrusion-analysis features arranged as a diamond; contextual framework evidence for H-016, not proof of Jane Street's intended emoji mapping.
- [Carnegie Mellon University, CMU Pronouncing Dictionary](https://github.com/cmusphinx/cmudict) — exploratory English pronunciation lexicon used through the locally installed `cmudict` 1.1.3 wrapper; its maintainers warn that errors, omissions, and inconsistencies may remain. Lexical candidate generation only, not an authoritative word or proper-name inventory.
- [PyPI, cmudict 1.1.3](https://pypi.org/project/cmudict/) — documents the Python wrapper and its GPL-3.0-or-later package licence; the wrapper is not a project dependency and no package code or dictionary files are included in OFRL.
- [The National Gallery, Putti glossary](https://www.nationalgallery.org.uk/paintings/glossary/putti) — defines putti as winged infants commonly depicted in religious and other artworks; supports the lexical/context lead only, not a four-member emoji grouping.
- [Aditi-Sahay, Jane-Street-Puzzles — November 2026 folder, pinned commit](https://github.com/Aditi-Sahay/Jane-Street-Puzzles/tree/b8cfe34d46347b1ec7f894d63a4b74e255bdc517/2026/November) — third-party candidate map, prose solution note, and solver code; not an official Jane Street solution and not copied into OFRL.
- [Visit Beijing, Four Great Temples of Beijing](https://english.visitbeijing.com.cn/article/47OMbo9hGyQ) — gives the four imperial-altar locations around the Forbidden City; contextual evidence only.
- [The Metropolitan Museum of Art, Tray with dragon and Chinese characters](https://www.metmuseum.org/art/collection/search/40224) — describes the dragon as a symbol of imperial authority in the particular object record; contextual evidence only.
- [CipherPad, Pigpen Cipher Alphabet](https://cipherpad.dev/learn/pigpen-cipher-alphabet) — documents the common S/T/U/V X-grid layout and notes variants exist; not a Jane Street clue rule.
- [The National Gallery, London, Jakob Schlesinger after Raphael, The Sistine Madonna](https://www.nationalgallery.org.uk/paintings/jakob-schlesinger-after-raphael-the-sistine-madonna) — identifies the Virgin, Saint Sixtus, Saint Barbara, and two cherubs at the bottom; its text does not state the left/right assignment.
- [MonopolyClub, Classic U.S. Board Layout](https://monopolyclub.com/rules/classic/board-layout) — lists the four classic corner spaces; independent secondary board reference, not evidence that board corners are cardinal vertices.
- [Berliner Philharmoniker, Who decides where the sections of the orchestra sit?](https://www.berliner-philharmoniker.de/en/stories/who-decides-the-seating-arrangement-of-the-musicians-on-stage/) — confirms conductor discretion, percussion toward the back, and variable positions for strings and double basses.
- [FIGC, Occupying spaces and positional fluidity](https://www.figc.it/media/256232/thesis-occupying-spaces-and-positional-fluidity-ridotta.pdf) — its Barcelona diamond diagram identifies Messi, Xavi, Iniesta, and Yaya Touré; it does not corroborate Busquets in that depicted quartet.
- [D.C. GIS, Administrative / Other Boundaries layers](https://maps2.dcgis.dc.gov/dcgis/rest/services/DCGIS_DATA/Administrative_Other_Boundaries/MapServer/layers) — warns that D.C. has no official neighborhood boundaries and that neighborhood layers are descriptive / label-placement products, not authoritative geometric vertices.
- [D.C. Office of Motion Picture and Television Development, Blue Plains in Southwest DC](https://entertainment.dc.gov/release/blue-plains-southwest-dc-recognized-one-city-location-month) — states Blue Plains sits at the southernmost tip of the District.
- [D.C. Office on Aging, FY2014 ADRC RFA](https://opgs.dc.gov/sites/default/files/dc/sites/opgs/publication/attachments/2014_ADRC_RFA.pdf) — includes North Portal Estates in its Ward 4 community list; does not establish an exact northernmost point.
- [D.C. Office of Advisory Neighborhood Commissions, ANC 7C](https://oanc.dc.gov/anc-profile/anc-7c) — lists Northeast Boundary among the neighborhoods represented by ANC 7C; it does not establish a precise east vertex.
- [D.C. Office of Planning, About Ward 3](https://planning.dc.gov/ar/page/about-ward-3) — places Palisades among neighborhoods in the upper northwest quadrant; quadrant context, not an exact west boundary point.
- [Antwerpsche Diamantkring, History](https://diamantkring.be/about/history/) — gives the bourse's founding date as 10 October 1929 and describes the rough-diamond trade context.
- [City of Antwerp, Open Monumentendag — Beurs voor Diamanthandel](https://magazine.antwerpen.be/open-monumentendag-2023/beurs-voor-diamanthandel?category=default) — municipal heritage account describes the four exchanges and dates the Antwerpse Diamantkring to 1930; note the one-year difference from the bourse's own history.
- [University of Chicago, Encyclopaedia Romana — Anemoi: The Greek Winds](https://penelope.uchicago.edu/~grout/Encyclopaedia_romana/greece/architecture/winds.html) — contextual reference for the four Homeric cardinal winds and their directions; it does not establish a picture-emoji association.
- [Orissa Review, The Four Dhams (July 2008)](https://magazines.odisha.gov.in/orissareview/2008/July-2008/engpdf/July-2008-or.pdf) — cultural article describing Badrinath north, Dwarka west, Rameshwaram south, and Puri east; the issue states that published views are not necessarily those of the Government of Odisha.
- [Government of India, Ministry of Tourism, Rajya Sabha Starred Question No. 22 (8 December 2022)](https://tourism.gov.in/sites/default/files/2022-12/SQ.22%20for%2008.12.2022.pdf) — confirms the four Char Dham destinations; the table does not itself specify cardinal directions.
- [Government of Madhya Pradesh, Statue of Oneness — Establishing the Four Amnaya Mathas](https://oneness.mp.gov.in/About/timeline/b1kvUVB4NDdHUkZpSlNtcXVBL2U1QT09) — identifies Sringeri south, Puri east, Dwarka west, and Jyotirmath north; historical/contextual evidence only.
- [Asmodee UK, Monopoly Rules](https://www.asmodee.co.uk/blogs/news/monopoly-rules) — secondary false-positive check noting that the Monopoly mascot is monocle-free; not evidence of Jane Street's intended 🧐 clue.

- [Shanghai Museum, Sheng plaque with Four-Celestial-Creatures Design](https://www.shanghaimuseum.net/mu/frontend/pg/m/article/id/CI00004310) — collection record for the Four Symbols and their east/south/west/north assignments; contextual evidence only.
- [National Research Institute for Cultural Properties, Japanese Garden Dictionary — Four Directional Deities](https://www.nabunken.go.jp/org/bunka/jgd/pages/FourDirectionalDeities.html) — corroborates the Japanese names Seiryū, Suzaku, Byakko, and Genbu and their cardinal positions; contextual evidence only.

Programme rationale: Jane Street, [Puzzles](https://www.janestreet.com/puzzles/).

This note contains OFRL-authored analysis and limited structural observations. It intentionally excludes the full puzzle payload, images, and official solution. No redistribution rights for those materials are assumed.

## 9. Revision history

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-11-05 | Initial source-grounded structural inventory and formalization blockers |
| 0.1.1 | 2026-11-05 | Added independently checked structural invariants and sequence-length distribution |
| 0.2.0 | 2026-11-05 | Added secondary research leads and explicitly unverified cardinal-vertex/output hypotheses; no formalization blockers closed |
| 0.3.0 | 2026-11-05 | Added partial four-member category hypotheses, letter-multiset audits, contextual entity corroboration, and two additional formalization blockers |
| 0.4.0 | 2026-11-05 | Added four explicit candidate cardinal layouts, independently sourced positional conventions, and reproducible letter-residual audits; no complete puzzle rule established |
| 0.5.0 | 2026-11-05 | Added six further candidate themes, community-reported decoded readings, source cross-checks, and explicit evidence boundaries; no formalization blocker closed |
| 0.6.0 | 2026-11-05 | Audited current secondary solver-status sources and corrected Archive search-result semantics; no complete derivation retrieved and no formalization blocker closed |
| 0.7.0 | 2026-11-05 | Added two source-cross-checked candidate clusters (Queen members and footballers) with exact residual audits; no cardinal orientation or global rule established |
| 0.8.0 | 2026-11-05 | Completed CM-T0007: audited all twelve candidate clusters, 48 unique source assignments, exact/residual counts, aggregate residual vector, evidence strength, and the 27+Ø inventory remainder; no puzzle rule established |
| 0.9.0 | 2026-11-05 | Added H-016, a source-grounded but unaccepted Diamond Model candidate for 💻; recorded four feature terms, diagram positions, +U/−E residuals, six remaining emoji, and a caveated CMUdict lexical lead scan; CM-T0008 remains in progress |
| 0.9.1 | 2026-11-05 | Screened four established four-member structures against the 23 remaining sequences with one-to-one minimum-cost character-multiset distance; recorded Dragon Kings (19), Heavenly Kings (34), swine breeds (29), and four selected architectural orders (23) as tested but not accepted; no new group promoted |
| 0.9.2 | 2026-11-06 | Expanded the remaining-inventory lexical ambiguity audit; documented equally close alternative readings; screened Chinese Four Symbols and Japanese Four Heavenly Kings transliterations by exhaustive minimum-cost matching; no new group or global rule accepted |
| 0.9.3 | 2026-11-06 | Exhaustively screened Four Anemoi, Char Dham, and Four Amnaya Peetham rosters; recorded the Monopoly monocle false-positive check; no emoji mapping or new hypothesis accepted |
| 0.10.0 | 2026-11-06 | Added H-017, a third-party 19-roster conditional exact-cover map independently checked with a strict two-sided multiset test; documented residual extraction, semantic conflicts, the published comparator defect, and the continuing no-go solver gate |
| 0.11.0 | 2026-11-06 | Completed H-018 evidence-led positional challenges for the Queen, Barcelona, Sistine Madonna, Pigpen, Beijing, D.C., Monopoly, and orchestra candidates; recorded the remaining ambiguity and completed CM-T0008 screening without accepting an official mapping |
