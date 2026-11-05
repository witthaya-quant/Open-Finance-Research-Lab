<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Case-Study Protocol

**Protocol version:** 0.1.0  
**Applies to:** All case studies admitted to OFRL-CM-001  
**Owner:** Project Owner  
**Status:** Approved baseline for the initial pilot

## 1. Purpose

This protocol provides one consistent research process for different mathematical and computational case studies. It standardizes the quality of problem definition, reasoning, verification, comparison, and reporting without forcing every problem into the same algorithm, programming language, or software architecture.

A case study must not select an algorithm before the problem has been formalized sufficiently to justify that choice.

## 2. Stage 1 — Problem formalization

### Required work
- Identify the original source, version or publication date, and access date.
- Record the source's visible inputs, outputs, notation, and instructions without silently filling gaps.
- Define the problem domain and intended task in plain language.
- Define sets, variables, indices, parameters, units, and data types.
- State constraints, objective function, output domain, and acceptance conditions where they are known.
- Distinguish explicit rules from assumptions and unresolved semantics.
- Identify whether the problem asks for one valid answer, all answers, an optimum, a proof, or an explanation.

### Gate
The case may move to method selection only when the rules needed to establish correctness are sufficiently clear. Otherwise it remains in formalization with a BLOCKED status and a list of precise questions.

## 3. Stage 2 — Mathematical foundations

Identify only the theories needed to model the problem. Possible areas include discrete mathematics, combinatorics, graph theory, logic, probability, statistics, information theory, optimization, and algorithm analysis.

For each material equation:
- Define every symbol, index, parameter, and domain.
- Explain the assumptions and boundary conditions.
- Check dimensional and logical consistency.
- Explain why the relationship applies to the problem.
- Distinguish a theorem or derivation from an empirical observation or implementation result.

## 4. Stage 3 — Alternative method assessment

Identify plausible methods after formalization. Candidate families may include exhaustive search, backtracking, dynamic programming, constraint programming, SAT/SMT, integer or mixed-integer optimization, graph algorithms, simulation, and heuristics.

For each selected method, document:
- Why it fits the problem's structure.
- Required assumptions and input representation.
- Expected complexity and resource implications.
- Completeness or optimality guarantees, if any.
- Known failure modes and limitations.

Do not implement multiple methods merely to increase the number of implementations. A comparative study is justified only when the alternatives answer a real research question.

## 5. Stage 4 — Independent verification

Where a solver or computational method is implemented, separate the candidate generator from the verifier.

The verifier should be independently reasoned and should check the formal specification directly. It should not merely call the solver's internal validation routine or duplicate its unexamined assumptions.

Relevant controls may include:
- Constraint and invariant checks.
- Boundary and malformed-input tests.
- Adversarial and counterexample tests.
- Property-based tests.
- Exhaustive cross-checks on reduced instances.
- Independent implementations or trusted reference calculations.
- Proofs of soundness, completeness, optimality, or uniqueness when those claims are made.

A valid candidate, a complete search, an optimal solution, and a unique solution are different claims and require different evidence.

## 6. Stage 5 — Comparative analysis

Before comparing methods, define the question and evaluation criteria. Hold inputs, environment, hardware, stopping conditions, and resource limits constant where practical.

Report applicable measures:
- Correctness and specification coverage.
- Soundness, completeness, and optimality guarantees.
- Time and space complexity.
- Runtime and peak memory.
- Robustness and failure behavior.
- Interpretability and ease of independent audit.
- Reproducibility and implementation complexity.

Report trade-offs and non-comparable conditions. Do not infer universal superiority from one case or one implementation.

## 7. Stage 6 — Financial-engineering transfer assessment

Consider transfer only after understanding the original mathematical structure.

The assessment must describe the proposed mapping, preserved properties, changed assumptions, financial context, and additional evidence required. Potential analogies include portfolio constraints, network models, dynamic allocation, scenario simulation, combinatorial optimization, and software verification.

Do not claim that an analogy is a direct application. Do not convert a puzzle result into a trading claim without separate empirical and economic evidence. A documented finding of no defensible transfer is an acceptable outcome.

## 8. Stage 7 — Research publication

A published case study should include, as appropriate:
- Original source and attribution.
- Problem statement or an independently written description that respects source rights.
- Formal model and mathematical derivation.
- Assumptions and unresolved issues.
- Methods and implementation details.
- Verification design and results.
- Comparative evidence and resource measurements.
- Financial-transfer assessment.
- Limitations, failed attempts, and negative findings.
- Reproduction instructions and version references.

Publication must pass the OFRL Data Policy and Publication and IP Boundary. Third-party material must not be relicensed by OFRL.

## 9. Case-study lifecycle and work-in-progress control

Each case moves through:
1. Candidate intake.
2. Source and rights review.
3. Formalization.
4. Method approval.
5. Implementation and verification, if justified.
6. Comparative review, if justified.
7. Publication review.
8. Closure, archival, or documented deferral.

Only one case study may be in active implementation at a time. Other cases remain in a candidate backlog and do not receive branches or implementation issues until admitted.

## 10. Required status vocabulary

Use the repository's validation states:
- PASS
- FAIL
- NOT RUN
- BLOCKED
- N/A

Do not promote a gate because a deadline has arrived or because a candidate solution appears plausible. Every PASS must identify evidence.
