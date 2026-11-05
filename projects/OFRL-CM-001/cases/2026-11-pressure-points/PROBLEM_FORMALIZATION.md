<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Pressure Points — Initial Problem Formalization

**Case ID:** OFRL-CM-001-CS-001  
**Formalization version:** 0.6.0  
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
| Game controller | TRIANGLE | CIRCLE | CROSS | SQUARE | Candidate source for TRIANGLE is the plural TRIANGLES (+S); candidate source for CIRCLE lacks E; CROSS and SQUARE are exact anagrams. |
| Baseball | SECOND | FIRST | HOME | THIRD | Candidate source for SECOND contains one extra O; candidate source for THIRD lacks H; FIRST and HOME are exact anagrams. |
| Sailing vessel | BOW | STARBOARD | STERN | PORT | Candidate source for BOW contains one extra A; candidate source for STERN lacks S; STARBOARD and PORT are exact anagrams. |

**Independent positional conventions:**
- Compass: the four cardinal directions are north, east, south, and west.
- Controller: the Sony DualSense manual's front-view diagram places Triangle at the top, Circle at the right, Cross at the bottom, and Square at the left. This is specific to the PlayStation-style face-button arrangement; the puzzle emoji itself is a generic game-controller symbol.
- Baseball: MLB's official field diagram places second base at the top of the diamond, first base at the right, home plate at the bottom, and third base at the left.
- Sailing: bow is the front, stern is the back, and when facing the bow, starboard is right and port is left. Mapping these relative vessel directions to N/E/S/W requires choosing the bow as the forward/North reference; it does not establish a geographic heading.

**Reproducible letter-count residual audit:** compare the candidate alphagram with the target term using character multisets, not edit distance or visual similarity. The observed residuals are:
- Compass: +E on NORTH and −E on EAST.
- Controller: +S on TRIANGLE(S) and −E on CIRCLE.
- Baseball: +O on SECOND and −H on THIRD.
- Sailing: +A on BOW and −S on STERN.

Across these four partial groups, the residuals do not establish a common letter-transfer rule. Their net multiset is not zero. The fact that the positional categories form recognizable diamonds is a separate observation from the unexplained residual letters. Do not infer the remaining 15 groups or an extraction rule from these four examples.

The source alphagrams themselves are not copied into this document. A researcher can reproduce each comparison from the official puzzle source URL and the candidate terms stated above.

**H-007 — Southern Cross / Crux.** Four candidate entries match the four main stars of Crux:
- Top: GACRUX — the source alphagram has one extra C.
- East arm: MIMOSA — exact anagram.
- Foot: ACRUX — the source alphagram is missing U.
- West arm: IMAI — exact anagram.

The European Southern Observatory confirms Acrux, Mimosa, Gacrux, and Imai as the four main stars of Crux. An astronomy education reference describes Gacrux at the top, Acrux at the foot, Mimosa on the eastern arm, and Imai on the western arm. The east/west labels are chart-orientation dependent, not a universal sky-coordinate convention. The star emoji → Crux association remains an unverified interpretation of Jane Street's clue.

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

The NFPA standard identifies the hazard-diamond system; a U.S. Department of Homeland Security training reference describes the four quadrants and their positions. The category is precisely *flammability*, not a separate “fire” label. The puzzle-specific association is not proven.

**H-011 — Desert / Colorado and the Four Corners region (provisional).** A candidate cardinal-neighbour set around Colorado is:
- North: WYOMING — source alphagram has one extra N.
- East: KANSAS — exact anagram.
- South: NEW MEXICO — source alphagram is missing E.
- West: UTAH — exact anagram.

Colorado also borders Nebraska to the northeast and Oklahoma to the southeast, and meets Arizona at the Four Corners point. The Four Corners region includes high-desert terrain. This makes a desert/Colorado interpretation plausible but weaker than the other candidate themes: the emoji-to-region link is inferential, and the four cardinal neighbours are not the complete set of all Colorado neighbours. Treat H-011 as provisional.

**H-012 — Joystick directions.** A candidate set is UP (north), RIGHT (east), DOWN (south), and LEFT (west). The source alphagram for UP has one extra H; RIGHT and DOWN are exact anagrams; the source alphagram for LEFT is missing T. The four directional terms fit the joystick emoji directly, but the role of their residual letters in the full puzzle remains unknown.

**H-013 — Community-reported candidate readings.** The Puzzling Stack Exchange discussion reports further decoded examples including BUSQUETS, INIESTA, VIOLINS, MERCURY, and INSTABILITY, alongside candidates such as GUJARAT, KANSAS, UTAH, WEST BENGAL, and STARBOARD. It also reports near-matches to place names: WYOMING with an additional N, MONTANA with an additional D, NEW MEXICO missing E, and HIMACHAL PRADESH with an additional G. These are user-generated candidate readings, not an authoritative grouping or a worked derivation. The discussion itself says that the puzzle page gives no written rules and explicitly leaves the meaning of the title, Ø, the emoji clues, and “(11 12)” unresolved. MONTANA + D is currently unassigned to any candidate emoji group.

### Secondary-source status review — 2026-11-05

The additional public sources reviewed in this session do not close a formalization blocker:

- Jane Street's current puzzle page lists correct submissions as of 2026-11-02. This confirms submitted answers were accepted, but the reviewed puzzle page does not provide a worked derivation.
- Kaia's current research note states that the author is establishing the exact 76-entry to 19 × 4 cardinal-vertex bijection and names `ANTWERPSCHE DIAMANTKRING` as the candidate output. The page reviewed does not expose the derivation, intermediate mapping, or independent verification.
- Griffin Pinney's page reports a leaderboard placing and says “Notes to come in December”. It is a solver-status signal, not a mathematical explanation available for verification in this review.
- The Puzzling Stack Exchange page is a user-generated question that records candidate anagrams and near-place-name residuals and asks what the title, Ø, emoji clues, and terminal notation mean. It is not a complete worked solution.
- The Jane Street Archive search result displays November “Pressure Points” followed by “Puzzle Solution” text. The search result does not expose an anchor destination; the directly opened Archive HTML is a stale cached representation beginning at August 2026. No direct solution URL or solution body was retrieved.

Accordingly, the candidate phrase remains externally reported but **not independently verified by OFRL**. A correct-submission list, a search-result label, or a solver-status statement cannot replace a complete derivation. No new group is promoted from a lead to an accepted mapping by this source review.

### Cross-group evidence boundary

H-007 through H-012 expand the partial candidate inventory from four to ten emoji themes. The external sources independently corroborate the named star system, conventional clock positions, India's national flower and state geography, the NFPA quadrant layout, and the Colorado/Four Corners geography. They do **not** establish that Jane Street intended these exact groups, that their proposed orientations are the puzzle's orientations, or that the remaining nine emojis follow the same construction.

The exact local character-count residuals are preserved above. They must not be normalized away: Crux +C/−U; clock +H and +A/−S; India +G/−L; NFPA +E/−S; Colorado +N/−E; joystick +H/−T. These observations do not yet yield one consistent global transformation or extraction rule.



**Evidence required before accepting any of H-001 through H-013:**
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

Ten candidate emoji themes have now been recorded across H-004, H-006, and H-007 through H-012, with additional ungrouped community readings in H-013. Their category-level meanings and several positional conventions have supporting references, but Jane Street's intended grouping, the complete 76-entry bijection, Ø's role, the cross-group residual mechanism, and output extraction are not established. U-001 through U-010 remain unresolved. No algorithm family is selected and no solver is authorized by this update.

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
