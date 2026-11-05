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

A provisional inventory now records thirteen candidate theme/structure clusters: the twelve audited under CM-T0007 plus H-016, a Diamond Model of Intrusion Analysis candidate for 💻. H-016 uses ADVERSARY, CAPABILITY, VICTIM, and INFRASTRUCTURE in the top/right/bottom/left positions of the published model; the letter residuals are +U and −E. The source establishes the framework and its geometry, not Jane Street's intended emoji association. Across 13 candidates, 52 distinct source sequences are provisionally assigned; 23 sequences plus Ø remain and six emojis have no candidate group. The Colorado/desert association remains weak; Queen has no established cardinal ordering; football remains underdetermined because GARETH is a given name. CM-T0007's 12-group audit found 25 exact / 23 residual-bearing pairs; H-016 adds two exact / two residual-bearing pairs, for 27 exact / 25 residual-bearing comparisons overall. The 24 = 6×4 remainder is arithmetic only. See CM-T0007 and H-016 / CM-T0008 in PROBLEM_FORMALIZATION.md.

The full sequence list, emoji sequence, images, and official solution are intentionally not copied into this repository.

## 4. Current research status

| Work item | Status |
|---|---|
| Original source and access date recorded | PASS |
| Visible structure described without reproducing the puzzle | PASS |
| Sequence counts and ordering properties checked | PASS |
| Formal input and output semantics established | BLOCKED |
| CM-T0007 audit of twelve candidate clusters: 48 distinct source assignments checked; 25 exact candidate anagrams and 23 residual-bearing pairs; semantic fit and positional evidence graded separately | PASS — audit only, not proof of puzzle intent |
| CM-T0008 preliminary H-016 Diamond Model lead: four core intrusion features mapped to top/right/bottom/left with independently counted +U/−E residuals | IN PROGRESS — source supports the model, but laptop-emoji intent and the remaining six groups are unresolved |
| Local letter-count residuals reproduced for the documented candidate groups | PASS |
| Community-reported anagram readings registered as unverified leads | PASS |
| Secondary solver-status sources reviewed and correctly classified as leads, not proofs | PASS |
| Direct official solution URL and worked derivation retrieved | BLOCKED |
| All 19 emoji groups and 76 cardinal assignments reconstructed | BLOCKED — 23 sequences plus Ø remain unassigned; 24 equals 6×4 only as a cardinality coincidence |
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

These are questions, not proposed answers. Hypotheses H-001 to H-015 and blockers U-001 to U-010 are recorded in PROBLEM_FORMALIZATION.md §4 and must not be promoted to facts without a complete derivation.

## 6. Next gate

Do not select SAT, CSP, graph search, string matching, anagram search, or another method until the rules support that representation.

**CM-T0008 remains in progress.** The Diamond Model candidate H-016 is a new lead for 💻, not an accepted puzzle mapping. Continue investigating the six remaining emoji clues against the 23 unused source sequences plus Ø. Treat the 24 = 6×4 equality as a count check only, not as proof of a four-vertex bijection. If semantic rules cannot be established, preserve the blocker rather than inventing missing rules.

## 7. Attribution

Original puzzle source: Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/).

Programme context: Jane Street, [Puzzles](https://www.janestreet.com/puzzles/).

All puzzle content remains attributable to Jane Street. OFRL does not claim ownership or redistribution rights over the original puzzle.


## 8. Research disposition and closure gate

**Current disposition: INCOMPLETE / BLOCKED — not solved and not ready for solver implementation.**

The current research round has completed its permitted source and candidate-context review, but it has not closed the mathematical case. Thirteen of 19 emoji/structure themes have provisional candidate groupings, including H-016 as a source-grounded Diamond Model lead but not a proven Jane Street clue association. Six groups, the role of Ø, a single global letter-residual rule, output extraction, and independent verification of the candidate phrase remain unresolved. The Archive search-result text does not verify a solution hyperlink or provide the solution body. Public solver-status pages reviewed on 2026-11-05 do not supply a complete derivation.

This is a **no-go for solver implementation under the evidence currently available**, not a claim that the puzzle is unsolvable and not a declaration that the case has been solved. Do not create a Cargo workspace, choose dependencies, or implement a solver while the correctness predicate remains undefined.

A solved-case closure requires, at minimum:
1. A documented 76-entry assignment with no omissions or duplicate assignments, including an explicit treatment of Ø.
2. A source-grounded interpretation of all 19 emoji clues and all four positions for each.
3. One consistent letter-transfer or transformation rule accounting for every residual, or a justified rejection of that proposed mechanism.
4. A derivation of the output and the meaning of “(11 12)” that does not rely on answer-length coincidence.
5. An independent check of the candidate output and any uniqueness/completeness claim.

Until these conditions are met, retain this case as an explicitly incomplete formalization record. An administrative decision to close it as a no-go would need to preserve these limitations and must not be described as a solved research result.