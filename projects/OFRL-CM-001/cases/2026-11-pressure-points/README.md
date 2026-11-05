<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Case Study — Pressure Points (November 2026)

**Case ID:** OFRL-CM-001-CS-001  
**Parent project:** OFRL-CM-001  
**Original source:** Jane Street — Pressure Points  
**Source publication label:** November 2026  
**Source URL:** https://www.janestreet.com/puzzles/pressure-points-index/  
**Access date:** 2026-11-05  
**Case status:** INCOMPLETE — secondary leads reviewed; complete rule semantics and derivation remain BLOCKED  
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

A partial reconstruction now records ten candidate emoji themes: compass, PlayStation-style face buttons, baseball bases, sailing-vessel positions, Southern Cross, clock face, India's cardinal-border states, NFPA 704 hazard diamond, Colorado/Four Corners region, and joystick directions. External sources corroborate several category conventions; the Colorado/desert association is explicitly provisional. Each local letter residual is recorded without forcing the groups into a common transformation rule. A separate community discussion reports further candidate readings, including BUSQUETS, INIESTA, VIOLINS, and MERCURY, but does not supply a complete derivation. See H-004 to H-013 in the formalization record.

The full sequence list, emoji sequence, images, and official solution are intentionally not copied into this repository.

## 4. Current research status

| Work item | Status |
|---|---|
| Original source and access date recorded | PASS |
| Visible structure described without reproducing the puzzle | PASS |
| Sequence counts and ordering properties checked | PASS |
| Formal input and output semantics established | BLOCKED |
| Ten candidate category/position groups documented with independent contextual references | PASS |
| Local letter-count residuals reproduced for the documented candidate groups | PASS |
| Community-reported anagram readings registered as unverified leads | PASS |
| Secondary solver-status sources reviewed and correctly classified as leads, not proofs | PASS |
| Direct official solution URL and worked derivation retrieved | BLOCKED |
| All 19 emoji groups and 76 cardinal assignments reconstructed | BLOCKED |
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

These are questions, not proposed answers. Hypotheses H-001 to H-013 and blockers U-001 to U-010 are recorded in PROBLEM_FORMALIZATION.md §4 and must not be promoted to facts without a complete derivation.

## 6. Next gate

Do not select SAT, CSP, graph search, string matching, anagram search, or another method until the rules support that representation.

The next permitted activity is source-grounded clarification and formal problem analysis. If authoritative rule semantics cannot be established, record the blocker and recommend pausing or replacing the pilot rather than inventing missing rules.

## 7. Attribution

Original puzzle source: Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/).

Programme context: Jane Street, [Puzzles](https://www.janestreet.com/puzzles/).

All puzzle content remains attributable to Jane Street. OFRL does not claim ownership or redistribution rights over the original puzzle.


## 8. Research disposition and closure gate

**Current disposition: INCOMPLETE / BLOCKED — not solved and not ready for solver implementation.**

The current research round has completed its permitted source and candidate-context review, but it has not closed the mathematical case. Ten of 19 emoji themes have provisional candidate groupings; the remaining groups, the role of Ø, a single global letter-residual rule, the output extraction, and independent verification of the candidate phrase remain unresolved. The Archive search-result text does not verify a solution hyperlink or provide the solution body. Public solver-status pages reviewed on 2026-11-05 do not supply a complete derivation.

This is a **no-go for solver implementation under the evidence currently available**, not a claim that the puzzle is unsolvable and not a declaration that the case has been solved. Do not create a Cargo workspace, choose dependencies, or implement a solver while the correctness predicate remains undefined.

A solved-case closure requires, at minimum:
1. A documented 76-entry assignment with no omissions or duplicate assignments, including an explicit treatment of Ø.
2. A source-grounded interpretation of all 19 emoji clues and all four positions for each.
3. One consistent letter-transfer or transformation rule accounting for every residual, or a justified rejection of that proposed mechanism.
4. A derivation of the output and the meaning of “(11 12)” that does not rely on answer-length coincidence.
5. An independent check of the candidate output and any uniqueness/completeness claim.

Until these conditions are met, retain this case as an explicitly incomplete formalization record. An administrative decision to close it as a no-go would need to preserve these limitations and must not be described as a solved research result.