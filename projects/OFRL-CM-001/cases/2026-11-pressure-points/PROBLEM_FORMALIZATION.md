<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Pressure Points — Initial Problem Formalization

**Case ID:** OFRL-CM-001-CS-001  
**Formalization version:** 0.1.1  
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

All eight items are unresolved in version 0.1.1.

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

Method selection is BLOCKED until U-001 through U-007 are resolved sufficiently to define a correctness predicate. U-008 must also be resolved before any completeness or uniqueness claim is made.

If source clarification is unavailable, the project must either:
- Keep the case as an explicitly incomplete formalization exercise; or
- Pause this pilot and propose a different case with sufficiently specified rules.

The team must not invent missing rules to make a solver feasible.

## 8. Source and rights boundary

Original source: Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/).

Programme rationale: Jane Street, [Puzzles](https://www.janestreet.com/puzzles/).

This note contains OFRL-authored analysis and limited structural observations. It intentionally excludes the full puzzle payload, images, and official solution. No redistribution rights for those materials are assumed.

## 9. Revision history

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-11-05 | Initial source-grounded structural inventory and formalization blockers |
| 0.1.1 | 2026-11-05 | Added independently checked structural invariants and sequence-length distribution |
