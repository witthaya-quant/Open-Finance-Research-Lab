<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Research Specification

**Research version:** 0.1.0  
**Project types:** Research  
**Owner:** Kenny (Witthaya Maidee)  
**Evidence maturity:** 0 — Proposed  
**Last updated:** 2026-11-05

## 1. Research question

How can selected mathematical and computational problems be formalized and investigated using methods that are appropriate to their actual structure, with correctness and reproducibility assessed independently of whether a candidate method returns an answer?

A secondary question is whether a method or insight has a defensible, clearly bounded application to quantitative research or financial engineering.

## 2. Motivation and intended contribution

Jane Street's public puzzle programme states that creative problem-solving is intrinsic to its work and connects the skill to identifying new problems in financial markets and developing potentially novel ways to solve them. This provides a legitimate reason to study problem-solving as a research discipline.

OFRL's contribution is independent analysis: state the problem, identify established mathematical foundations, examine suitable methods, validate results, and explain limitations. The work is not intended to reverse-engineer Jane Street's proprietary business or trading methods.

The intended audience is researchers, quantitative practitioners, software engineers, and technically curious readers who benefit from transparent mathematical reasoning and reproducible computational analysis.

## 3. Shared foundations and prior work

The research may draw on, where relevant:
- Discrete mathematics, combinatorics, and graph theory.
- Logic, constraint satisfaction, SAT and SMT.
- Search, dynamic programming, and state-space methods.
- Optimization, including linear, integer, and combinatorial optimization.
- Probability, statistics, information theory, and simulation.
- Algorithm analysis, formal methods, software testing, and independent verification.

These are candidate foundations only. A case study must select concepts because they follow from the formal problem, not because they appear in this list.

### Source context

Jane Street's Puzzles page says it posts puzzles periodically and explains its own rationale: creative problem-solving is relevant to finding new problems in financial markets and developing new approaches. The source also notes that puzzles may have clearly or unclearly defined rules. OFRL treats this as the source author's description, not as evidence that a particular puzzle has a financial interpretation.

The first pilot is the official November 2026 Pressure Points puzzle. Its source references and access date are recorded in [DATA_MANIFEST.md](DATA_MANIFEST.md).

A separate Jane Street article, Results from the ASIC puzzle (2 October 2026), describes a hardware reverse-engineering challenge and diverse solver approaches. It is methodological context only; it is not the source or subject of the first pilot.

## 4. Objectives and research outputs

For each admitted case:
1. Preserve source provenance without redistributing material whose rights have not been established.
2. Separate directly observed facts from interpretations, hypotheses, and unknowns.
3. Define the input domain, output domain, constraints, objective, and success conditions.
4. Identify assumptions and prove or test that they are justified.
5. Select mathematical methods and computational approaches appropriate to the formal structure.
6. Develop an implementation only where it adds research value.
7. Verify candidate outputs independently of the method that generated them.
8. Compare alternatives only under documented, comparable conditions.
9. assess financial-engineering relevance without forcing a transfer claim.
10. Publish methods, evidence, limitations, and reproduction instructions at an evidence level proportionate to what has actually been established.

Expected outputs may include mathematical notes, formal specifications, proofs, hand-worked examples, code, test fixtures created independently, performance results, and a research report. Not every case must produce all output types.

## 5. Research methodology

The project uses the seven-stage protocol in [CASE_STUDY_PROTOCOL.md](CASE_STUDY_PROTOCOL.md). The protocol requires problem formalization before algorithm selection and separates solver behavior from verification evidence.

A candidate algorithm is not evidence that the problem has been correctly understood. A passing example is not proof of completeness, uniqueness, or correctness over the full input domain.

## 6. Evaluation design

Evaluation criteria are selected per case and documented before interpreting comparative results where practicable.

Potential criteria include:
- Semantic correctness against the formal specification.
- Soundness of a returned candidate.
- Completeness, where finding all solutions is part of the stated objective.
- Proof of uniqueness, where uniqueness is claimed.
- Time and space complexity.
- Runtime and memory under a fixed environment.
- Robustness to boundary and adversarial inputs.
- Interpretability and auditability.
- Reproducibility from a clean environment.

These criteria are not interchangeable. For example, a solver that returns one valid answer may be sound for that answer but incomplete if other answers exist.

## 7. Data and sample design

The first pilot uses the official Jane Street puzzle page as a third-party source. OFRL stores source references and a limited description of the observed format, not a copy of the full puzzle content. No redistribution permission for puzzle text, graphics, or the official solution is assumed.

Any independently created synthetic fixtures must be clearly labelled and must not reproduce the source puzzle in a way that substitutes for or republishes protected expression. A fixture's inclusion requires a documented purpose and a separate data-rights check.

## 8. Reproducibility

Every material result must record:
- Project version and relevant commit.
- Source URL and access date.
- Formal specification version.
- Assumptions and parameter definitions.
- Toolchain, dependencies, environment, and configuration if code is used.
- Test inputs and expected outputs, subject to rights.
- Execution commands and validation results.
- Known limitations and unresolved questions.

Randomized methods must record seeds and explain whether repeated runs are required.

## 9. Results and interpretation

Distinguish:
- Facts visible in an original source.
- Formal definitions adopted by OFRL.
- Assumptions introduced to make a model tractable.
- Outputs produced by a method.
- Independently verified properties.
- Empirical performance observations.
- Interpretations and potential applications.

Do not describe an answer as correct merely because it matches an unofficial expectation or because one implementation returns it.

## 10. Financial-engineering transfer

A financial application is considered only when the mathematical structure and operating assumptions support a meaningful mapping.

Examples that may be investigated include constraint optimization, graph algorithms, dynamic programming, simulation, combinatorial search, and formal verification. These examples are not claims that the first puzzle uses those methods or that the methods directly produce a trading edge.

Any transfer analysis must identify:
- The source problem structure and the proposed financial analogue.
- Which mathematical properties remain invariant under the mapping.
- Which assumptions differ materially.
- Operational, data, market, regulatory, and economic constraints.
- What additional tests would be needed before practical use.

A conclusion of no direct or useful transfer is acceptable.

## 11. Limitations and non-claims

This project does not:
- Claim access to Jane Street's internal research, trading systems, or proprietary methods.
- Claim that puzzle-solving performance predicts trading performance.
- Claim that a puzzle is a financial model.
- Guarantee a unique solution or a particular solution method.
- Treat heuristic success as proof.
- Publish third-party puzzle text, graphics, or official solutions without permission.
- Authorize live trading, investment advice, or external operational actions.

## 12. References and attribution

- Jane Street, [Puzzles](https://www.janestreet.com/puzzles/), accessed 2026-11-05.
- Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/), accessed 2026-11-05.
- Jane Street, [Puzzles Archive](https://www.janestreet.com/puzzles/archive/), accessed 2026-11-05.
- Jane Street Blog, [Results from the ASIC puzzle](https://blog.janestreet.com/asic-puzzle-results/), published 2026-10-02; contextual reference only.

All third-party source content remains attributable to its respective rights holder and is not covered by OFRL's documentation or software licences.
