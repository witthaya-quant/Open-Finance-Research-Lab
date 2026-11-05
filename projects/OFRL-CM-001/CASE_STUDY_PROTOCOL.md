<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Case-Study Protocol

**Protocol version:** 0.2.0  
**Applies to:** All case studies admitted to OFRL-CM-001  
**Owner:** Project Owner  
**Status:** Approved baseline for the initial pilot

## 1. Purpose

This protocol provides one consistent research process for different mathematical and computational case studies. It standardizes the quality of problem definition, reasoning, verification, comparison, and reporting without forcing every problem into the same algorithm, programming language, or software architecture.

A case study must not select an algorithm before the problem has been formalized sufficiently to justify that choice. The learner-facing workflow, hypothesis register, method-selection questions, and assessment rubric are defined in [PRACTICE_PLAYBOOK.md](PRACTICE_PLAYBOOK.md). Project scope and task status are maintained only in [PROJECT_CONTROL.md](PROJECT_CONTROL.md).

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

When executable computational work is justified, follow [IMPLEMENTATION_APPROACH.md](IMPLEMENTATION_APPROACH.md) for typed domain boundaries, deterministic computation, independent verification, dependency controls, and testing. Software must add research value to the case rather than exist merely to demonstrate a tool.

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

### 9.1 Project and case directory boundaries

The project root contains project-wide documents and controls. Case-specific material belongs under `cases/<YYYY-MM>-<short-slug>/` and must not be placed in the repository root or in another OFRL project.

For every admitted case:
- Create its case directory only after the case passes the candidate-intake, source/rights, and project-owner admission decision.
- Start with a case `README.md` and `PROBLEM_FORMALIZATION.md` (or an equivalently named formal specification when the problem is already fully defined).
- Add derivations, data manifests, implementation, tests, or result reports only when the case requires them and the corresponding evidence gate authorizes them.
- Keep shared case-study protocol, project-level research specification, decision log, validation matrix, and handoff at the project root. Do not copy these documents into each case directory.
- Record case-specific material decisions in the project Decision Log using the case ID; create a separate case-level decision record only when a distinct lifecycle or ownership boundary demonstrably requires it.
- Keep the project manifest's `required_documents` synchronized with the documents that are mandatory for the admitted project and its active case.

Do not create empty case folders, speculative backlog folders, placeholder source trees, or duplicate templates. A future case receives a directory only after admission. The project may have many completed case directories over time, but only one case may be in active implementation at a time.

### 9.2 Follow-on branches and pull requests

PR #29 is the project-admission and first-pilot documentation PR. It must not be used to smuggle unrelated projects, shared infrastructure changes, or implementation work that is outside its reviewed scope.

After the project admission PR is merged, each separately authorized material implementation or research change should use a new topic branch and PR under the same `OFRL-CM-001` project. A new case does not automatically require a new OFRL project ID. A new project ID is considered only when the research objective, ownership, risk, or independently managed deliverable is materially different.

Before PR #29 is merged, do not create follow-on branches based on `main` that assume the project files already exist there. Do not merge, close, or retarget PR #29 without the owner's explicit instruction.

### 9.3 Shared policy inheritance

OFRL-CM-001 adopts the repository-root policies as the single authoritative common baseline. The project links to those policies rather than copying them into its directory. Project-specific specifications, decisions, and controls may strengthen the common baseline but must not silently waive or contradict it. The README identifies the applicable policy set; the handoff instructs future sessions to re-read the live root policy when scope or policy changes are material. The project manifest and repository validation continue to enforce the required project and case documents.

## 10. Required status vocabulary

Use the repository's validation states:
- PASS
- FAIL
- NOT RUN
- BLOCKED
- N/A

Do not promote a gate because a deadline has arrived or because a candidate solution appears plausible. Every PASS must identify evidence.
