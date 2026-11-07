<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Case Study — Pressure Points (November 2026)

**Case ID:** OFRL-CM-001-CS-001  
**Parent project:** OFRL-CM-001  
**Original source:** Jane Street — Pressure Points  
**Source publication label:** November 2026  
**Source URL:** https://www.janestreet.com/puzzles/pressure-points-index/  
**Access date:** 2026-11-05  
**Last research review:** 2026-11-07  
**Case status:** INCOMPLETE / BLOCKED — CM-T0008 screening complete; CM-T0009 rejected H-017 as a complete source-grounded model; CM-T0010 and CM-T0011 remain blocked  
**Evidence maturity:** 0 — Proposed

## 1. Why this case was selected

Pressure Points is used as the first pilot for the project's case-study protocol because it presents a compact, structured puzzle with multiple visible clue types. It provides an opportunity to test whether OFRL can distinguish source observations from assumptions before selecting a mathematical model or solver.

The case is not selected as a financial model or a representation of Jane Street's internal trading methods.

## 2. Research objective

Establish a precise, source-traceable problem definition for Pressure Points. Determine the meaning and relationship of its visible elements, the required output, and the correctness conditions before choosing any algorithm.

## 3. Source and observed structure

The official source page, accessed on 2026-11-05, visibly presents:
- A leading symbol.
- 75 uppercase-letter sequences.
- 19 emoji clues.
- A terminal notation rendered as “(11 12)”.

The visible sequences are distinct, internally alphabetically ordered, and displayed in lexicographical order. Their total length is 504 letters. This is a structural observation only; it does not establish the intended semantic role of the sequences.

The page content reviewed on 2026-11-05 does not expose a complete prose rule specification that resolves the relationship between these elements. Secondary public research suggests a possible one-to-one mapping between the 76 visible marker/sequence entries and four cardinal vertices for each of the 19 emoji clues. A separate solver-status note reports the candidate phrase ANTWERPSCHE DIAMANTKRING. A further solver-status page says a write-up is forthcoming in December, but provides no worked derivation in the reviewed content. The 2026-11-05 review therefore found multiple public solver-status signals, but no complete derivation that OFRL can independently inspect. The phrase remains a hypothesis.

The earlier CM-T0007 / H-016 checkpoint contained 13 provisional clusters and 52 source assignments, leaving 23 sequences plus Ø and six emoji without candidate rosters. H-017 now records a third-party 19-theme candidate map that conditionally assigns all 76 visible entries (75 strings plus Ø) exactly once under a 2-exact / 1-surplus / 1-deficit multiset rule. Two independent strict implementations reproduce one assignment per fixed roster, with 38 exact, 19 surplus and 19 deficit comparisons, including Ø→U (−U). This is a conditional exact-cover result for the proposed rosters, not proof that Jane Street intended those themes or the rule. H-017 also records residual strings NEWESTROUGHEXCHANGE / LEUVENGHENTBRUSSELS and a candidate Antwerp output interpretation. The third-party solver code has a comparator defect, and several semantic or positional claims remain weak or conflicting; see H-017 in PROBLEM_FORMALIZATION.md.

Historical pre-H-017 screen (2026-11-06): the Four Anemoi (22; 9 optima), Char Dham (25; 3), and Four Amnaya Peethams (24; 2) were tested against the then-unassigned 23-entry inventory and were not accepted. The Monopoly-mascot-with-monocle association was also rejected as a false visual memory. H-017 was added later and supplies separate candidate rosters for all 19 emoji; the earlier six-emoji / 13-cluster count is retained only as a dated intermediate state, not as the current conditional inventory. See PROBLEM_FORMALIZATION.md H-017 and the evidence register.

H-017 supplies candidate rosters for the six previously unresolved emojis, but their evidence grades differ. Beijing's imperial altars and the Sistine Madonna composition have contextual support; Pigpen X-grid is a plausible geometric wordplay lead. The Washington D.C. neighborhoods lack precise official boundary vertices; Monopoly corners are not cardinal directions; orchestra seating is variable; and the Barcelona diamond reference depicts Yaya Touré rather than Busquets. The Queen roster also moves from the earlier 🎶 hypothesis H-014 to 👸, while the earlier football H-015 competes with the new Barcelona roster. None is accepted as Jane Street's intended association.

H-018 completed the source-led positional challenge. Sotheby's confirms Queen crest symbols for May, Mercury, Deacon and Taylor, but the two lions do not distinguish Deacon from Taylor left/right. A 2010/11 Barcelona tactical retrospective supports the Messi/Xavi/Busquets/Iniesta box or asymmetric-diamond interpretation, while emphasizing fluid rotations. The Royal Collection Trust confirms Sistine Madonna's Sixtus-left / Barbara-right / cherubs-lower layout. The common Pigpen X-grid and Beijing altar directions align with H-017. The Washington D.C. locality set is directionally plausible but not built from official neighborhood boundaries. Monopoly's four corners can be transformed to T/R/B/L by a declared 45-degree clockwise board rotation, but the monocle-to-Mr.-Monopoly clue link remains unsupported. Orchestra seating is conductor-dependent, so no universal four-point layout is accepted. Details are in PROBLEM_FORMALIZATION.md H-018 and DATA_MANIFEST.md SRC-060–SRC-067.

The full sequence list, emoji sequence, images, and official solution are intentionally not copied into this repository.

## 4. Current research status

| Work item | Status |
|---|---|
| Original source and access date recorded | PASS |
| Visible structure described without reproducing the puzzle | PASS |
| Sequence counts and ordering properties checked | PASS |
| Formal input and output semantics established | BLOCKED |
| CM-T0007 audit of twelve candidate clusters: 48 distinct source assignments checked; 25 exact candidate anagrams and 23 residual-bearing pairs; semantic fit and positional evidence graded separately | PASS — audit only, not proof of puzzle intent |
| CM-T0008 candidate work: H-017 complete-map audit plus H-018 evidence-led semantic/positional challenge of the previously open clues | PASS — screening complete; no candidate mapping accepted as an official Jane Street rule; Queen slot swap, 🧐 clue link, and exact global semantics remain unresolved |
| CM-T0009 — bounded model-rejection assessment | PASS — H-017/H-018 are rejected as a complete source-grounded explanation; Ø→U, 2/1/1 residual operation, extraction order and `(11 12)` interpretation remain unproven. Candidate `ANTWERPSCHE DIAMANTKRING` is retained as conditional, not disproved and not verified |
| Local letter-count residuals reproduced for the documented candidate groups | PASS |
| Community-reported anagram readings registered as unverified leads | PASS |
| Secondary solver-status sources reviewed and correctly classified as leads, not proofs | PASS |
| Official Jane Street Archive / solution retrieval | BLOCKED — live Archive lists the November puzzle and unlinked “Solution” text; the “Puzzle” link opens the puzzle/submission page; no direct solution target/body or worked derivation retrieved (CM-T0010; DATA_MANIFEST.md §4; TEST-044) |
| Source-validated 19 emoji groups and 76 cardinal assignments reconstructed | BLOCKED — H-017 conditionally covers 76/76 source entries, but Queen's left/right member assignment, 🧐 clue membership, and source-authorized 19-group/cardinal rules remain unverified |
| One global letter-reallocation / extraction rule explained across all 19 groups | BLOCKED |
| Candidate output phrase independently verified | BLOCKED |
| Candidate mathematical model selected | NOT RUN |
| Solver implementation | NOT STARTED / DEFERRED |
| Independent verifier | NOT STARTED / DEFERRED |
| Comparative method evaluation | NOT STARTED / DEFERRED |
| Financial-engineering transfer | BLOCKED pending formalization |

## 5. Questions to resolve

1. What does the leading symbol represent?
2. What does each uppercase-letter sequence represent, and does list order carry meaning?
3. How are the emoji clues related to the letter sequences?
4. Does each clue correspond to one sequence, multiple sequences, or another structure?
5. What is the role of the terminal notation “(11 12)”?
6. What output is required: a word, sequence, assignment, path, set, or another object?
7. What establishes a valid answer, and is uniqueness part of the task?
8. Do the 19 emoji clues each identify a four-member semantic set, and what determines the N/E/S/W orientation?
9. Do the letter-count residuals represent a deliberate transfer operation, and can one rule explain all 19 groups?
10. What is the correct treatment of Ø within the 76-position model?
11. Which community-reported readings are intended by the emoji clues, and how is ambiguity between valid anagrams resolved?

These are questions, not proposed answers. Hypotheses H-001 to H-017 and blockers U-001 to U-010 are recorded in PROBLEM_FORMALIZATION.md §4; H-018 is an evidence audit, not a new solution hypothesis. None should be promoted to fact without a complete derivation.

## 6. Next gate

Do not select SAT, CSP, graph search, string matching, anagram search, or another method until the rules support that representation.

**CM-T0008 is complete and CM-T0009 is PASS through the bounded rejection path.** The candidate H-017/H-018 construction has been rejected as a complete source-grounded explanation because the primary source does not define the role of Ø, the 2/1/1 residual operation, a traversal/extraction order, or the meaning of `(11 12)`. This does not establish that the conditional candidate answer is false. The next research tasks are CM-T0010 (retrieve a direct official solution target/body only if Jane Street exposes one) and CM-T0011 (independent derivation and correctness assessment only after authoritative rules and an acceptance predicate exist). Do not infer a solution URL from the Archive's unlinked label or search-index text. The latest retrieval is inconsistent across indexed and cached page representations; keep the case INCOMPLETE / BLOCKED and the solver/Cargo gate NO-GO.

## 7. Attribution

Original puzzle source: Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/).

Programme context: Jane Street, [Puzzles](https://www.janestreet.com/puzzles/).

All puzzle content remains attributable to Jane Street. OFRL does not claim ownership or redistribution rights over the original puzzle.


## 8. Research disposition and closure gate

**Current disposition: INCOMPLETE / BLOCKED — not solved and not ready for solver implementation.**

The current research round has reproduced a third-party candidate map for all 19 themes and 76 visible source entries under a strict 2-exact / 1-surplus / 1-deficit multiset rule. This is a conditional exact cover of nominated rosters, not a source-validated solution. H-018 improves selected positional evidence but leaves Queen's Deacon/Taylor left/right swap unresolved, the 🧐-to-Monopoly association unsupported, D.C. positions approximate, and orchestra seating non-universal. CM-T0009 subsequently rejects H-017/H-018 as a complete source-grounded formal explanation: Ø→U, source-authorized group membership, the 2/1/1 operation, extraction order, and the meaning of `(11 12)` remain unproved. The candidate phrase `ANTWERPSCHE DIAMANTKRING` remains conditional; it has not been disproved or independently verified as Jane Street's intended answer. The initial 2026-11-07 direct-page audit recorded “Solution” as plain text without an exposed target. A later search-index result displays “Puzzle Solution”, while the page-open representation is stale/cached and the direct puzzle-page open returns a cache miss. The current official solution target and body therefore remain unverified and NOT RETRIEVED.

This is a **no-go for solver implementation under the evidence currently available**, not a claim that the puzzle is unsolvable and not a declaration that the case has been solved. Do not create a Cargo workspace, choose dependencies, or implement a solver while the correctness predicate remains undefined.

A solved-case closure requires, at minimum:
1. A documented 76-entry assignment with no omissions or duplicate assignments, including an explicit treatment of Ø.
2. A source-grounded interpretation of all 19 emoji clues and all four positions for each.
3. One consistent letter-transfer or transformation rule accounting for every residual, or a justified rejection of that proposed mechanism.
4. A derivation of the output and the meaning of “(11 12)” that does not rely on answer-length coincidence.
5. An independent check of the candidate output and any uniqueness/completeness claim.

Until these conditions are met, retain this case as an explicitly incomplete formalization record. An administrative decision to close it as a no-go would need to preserve these limitations and must not be described as a solved research result.