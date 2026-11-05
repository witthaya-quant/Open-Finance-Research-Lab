<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Case Study — Pressure Points (November 2026)

**Case ID:** OFRL-CM-001-CS-001  
**Parent project:** OFRL-CM-001  
**Original source:** Jane Street — Pressure Points  
**Source publication label:** November 2026  
**Source URL:** https://www.janestreet.com/puzzles/pressure-points-index/  
**Access date:** 2026-11-05  
**Case status:** Formalization in progress; external structural hypothesis identified, rule semantics still BLOCKED  
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

The page content reviewed on 2026-11-05 does not expose a complete prose rule specification that resolves the relationship between these elements. Secondary public research suggests a possible one-to-one mapping between the 76 visible marker/sequence entries and four cardinal vertices for each of the 19 emoji clues. A separate solver-status note reports the candidate phrase ANTWERPSCHE DIAMANTKRING. Neither claim includes a complete derivation in the material reviewed; both remain hypotheses.

A partial reconstruction has identified four candidate cardinal groups: compass directions, PlayStation-style face buttons, baseball bases, and sailing-vessel positions. Their N/E/S/W conventions are corroborated by independent references. The four groups also expose specific character-multiset residuals (+E/−E, +S/−E, +O/−H, +A/−S). These residuals do not yet form a common transformation rule. See H-004 to H-006 in the formalization record.

The full sequence list, emoji sequence, images, and official solution are intentionally not copied into this repository.

## 4. Current research status

| Work item | Status |
|---|---|
| Original source and access date recorded | PASS |
| Visible structure described without reproducing the puzzle | PASS |
| Sequence counts and ordering properties checked | PASS |
| Formal input and output semantics established | BLOCKED |
| Four candidate cardinal groups and independent positional conventions cross-checked | PASS |
| Candidate letter residuals reproduced for those four groups | PASS |
| All 19 emoji groups and 76 cardinal assignments reconstructed | BLOCKED |
| One letter-reallocation rule explained across all 19 groups | BLOCKED |
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

These are questions, not proposed answers. Hypotheses H-001 to H-006 and blockers U-001 to U-010 are recorded in PROBLEM_FORMALIZATION.md §4 and must not be promoted to facts without a complete derivation.

## 6. Next gate

Do not select SAT, CSP, graph search, string matching, anagram search, or another method until the rules support that representation.

The next permitted activity is source-grounded clarification and formal problem analysis. If authoritative rule semantics cannot be established, record the blocker and recommend pausing or replacing the pilot rather than inventing missing rules.

## 7. Attribution

Original puzzle source: Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/).

Programme context: Jane Street, [Puzzles](https://www.janestreet.com/puzzles/).

All puzzle content remains attributable to Jane Street. OFRL does not claim ownership or redistribution rights over the original puzzle.
