<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Pressure Points — Initial Problem Formalization

**Case ID:** OFRL-CM-001-CS-001  
**Formalization version:** 0.7.0  
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

All ten items remain unresolved in version 0.5.0.

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

### Secondary-source status review — 2026-11-05

The additional public sources reviewed in this session do not close a formalization blocker:

- Jane Street's current puzzle page lists correct submissions as of 2026-11-02. This confirms submitted answers were accepted, but the reviewed puzzle page does not provide a worked derivation.
- Kaia's current research note states that the author is establishing the exact 76-entry to 19 × 4 cardinal-vertex bijection and names `ANTWERPSCHE DIAMANTKRING` as the candidate output. The page reviewed does not expose the derivation, intermediate mapping, or independent verification.
- Griffin Pinney's page reports a leaderboard placing and says “Notes to come in December”. It is a solver-status signal, not a mathematical explanation available for verification in this review.
- The Puzzling Stack Exchange page is a user-generated question that records candidate anagrams and near-place-name residuals and asks what the title, Ø, emoji clues, and terminal notation mean. It is not a complete worked solution.
- QueenOnline lists Freddie Mercury, Brian May, Roger Taylor, and John Deacon as the classic band members; this corroborates H-014's member identities, not the music-emoji association or cardinal order.
- Real Madrid's official Gareth Bale profile and FC Barcelona's official profiles for Iniesta, Busquets, and Messi corroborate the player identities in H-015; these references do not establish the candidate grouping or cardinal order.
- The Jane Street Archive search result displays November “Pressure Points” followed by “Puzzle Solution” text. The search result does not expose an anchor destination; the directly opened Archive HTML is a stale cached representation beginning at August 2026. No direct solution URL or solution body was retrieved.

Accordingly, the candidate phrase remains externally reported but **not independently verified by OFRL**. A correct-submission list, a search-result label, or a solver-status statement cannot replace a complete derivation. No new group is promoted from a lead to an accepted mapping by this source review.

### Cross-group evidence boundary

H-007 through H-012 expanded the initial four-cluster inventory to ten themes; H-014 and H-015 brought the recorded candidate inventory to twelve. Their contextual sources corroborate selected facts and conventions but do **not** establish Jane Street's intended emoji mappings or a common residual rule. The CM-T0007 audit separately assessed those twelve clusters without promoting any to fact. H-016 is a new CM-T0008 lead for 💻, based on the four vertices of the published Diamond Model of Intrusion Analysis; it is not part of the completed twelve-cluster CM-T0007 audit.

After H-016's four source entries are included provisionally, 13 candidate clusters account for 52 distinct source sequences. The remaining inventory is 23 sequences plus Ø, or 24 visible items, with six emoji still lacking candidate groups. The equality 24 = 6 × 4 remains arithmetic alignment only.

The residuals from the first twelve clusters remain as audited in CM-T0007. H-016 adds +U for ADVERSARY and −E for INFRASTRUCTURE. Across all 52 provisional assignments there are 27 exact pairs and 25 residual-bearing pairs, with 14 surplus and 12 deficit characters. The aggregate net residual is +A×2, +C, +G, +H, +N×2, +O, −E, −L×2, −S×2, −T. This still does not yield one global transformation or extraction rule.

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




**Evidence required before accepting any of H-001 through H-016:**
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

The CM-T0007 audit covered twelve candidate clusters across H-004, H-006, H-007 through H-012, H-014, and H-015. During CM-T0008, H-016 adds a thirteenth provisional cluster for 💻 based on the Diamond Model of Intrusion Analysis. The current inventory therefore has 13 candidate clusters and 52 distinct provisional source assignments, with 23 source sequences plus Ø remaining and six emoji still unmapped. The first twelve groups had 25 exact and 23 residual-bearing pairs; H-016 adds two exact and two residual-bearing pairs, giving 27 exact and 25 residual-bearing pairs overall. Candidate semantic fit and cardinal orientation remain unequal: Crux is chart-orientation dependent; Colorado remains weak; Queen has no cardinal ordering; football is underdetermined by GARETH; and H-016 is source-grounded as a model but not yet tied to Jane Street's emoji by an explicit rule. The complete 76-item bijection, Ø's role, global residual transformation, output extraction, and uniqueness/completeness remain unestablished. U-001 through U-010 remain unresolved. No algorithm family is selected and no solver is authorized by this update.

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
