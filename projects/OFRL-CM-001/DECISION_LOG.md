<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Decision Log

**Project ID:** OFRL-CM-001  
**Project owner:** Kenny (Witthaya Maidee)  
**Baseline version:** 0.1.0  
**Last reviewed:** 2026-11-05

This log records decisions specific to OFRL-CM-001. Lab-wide matters remain governed by the central OFRL policies and Decision Register.

## 1. Approved project baseline

| ID | Decision | Status | Rationale / consequence | Validation evidence |
|---|---|---|---|---|
| D-001 | Establish OFRL-CM-001 as a separate registered project, isolated from OFRL-IB-001 | Approved | Different research objective, domain, methods, deliverables, and validation requirements require independent project identity and controls | Separate project directory, manifest, issue #29, branch, and PR |
| D-002 | Use one project containing multiple independently admitted case studies | Approved | Shared protocol and governance reduce duplicated administration without conflating individual case results | Project specification and case-study protocol |
| D-003 | Select Jane Street Pressure Points (November 2026) as the first pilot | Approved | A current, publicly accessible puzzle provides a bounded test of problem formalization and source-rights controls | Official puzzle source recorded in DATA_MANIFEST.md |
| D-004 | Treat Jane Street as the original source and do not copy the complete puzzle, images, or official solution into OFRL absent permission | Approved | Public accessibility does not establish redistribution rights | Source references only; no puzzle payload stored |
| D-005 | Require formal rule clarification before selecting or implementing a solver | Approved | Prevents premature algorithm selection and false claims of correctness | Pilot formalization records unresolved semantics |
| D-006 | Limit work in progress to one active case study | Approved | Protects project focus and avoids uncontrolled backlog growth | CASE_STUDY_PROTOCOL.md |
| D-007 | Do not require software or multiple solvers where mathematical analysis or proof is sufficient | Approved | Research output must be justified by the question, not by implementation volume | Research Specification and protocol |
| D-008 | Assess financial-engineering transferability without forcing a positive conclusion | Approved | Preserves OFRL's finance orientation and research integrity | Research Specification and protocol |
| D-009 | Adopt Rust as the default implementation language for computational software in OFRL-CM-001 when implementation is justified | Approved | Aligns the project with OFRL's Rust engineering direction while keeping research method-first; pure mathematical work does not require code | IMPLEMENTATION_APPROACH.md; no executable implementation is authorized by this decision |

## 2. Open decisions

| ID | Question | Options / trade-offs | Owner | Target review | Status |
|---|---|---|---|---|---|
| O-001 | What are the complete semantics and expected output of Pressure Points? | Obtain an authoritative rule explanation or solution reference; otherwise pause the pilot | Project Owner | Before method selection | Open |
| O-002 | Is there a well-defined mathematical model that supports a meaningful computational comparison? | Formal proof, one implementation, multiple methods, or no implementation | Project Owner | After O-001 | Open |
| O-003 | Is there a defensible financial-engineering transfer? | Specific bounded mapping, indirect methodological relevance, or no transfer | Project Owner | After formalization | Open |

## 3. Deferred scope

- Solver implementation and answer submission.
- Reimplementation of Jane Street's official solution.
- Puzzle scraping, automated intake, and answer automation.
- General-purpose solver frameworks or platforms.
- Additional Jane Street puzzles.
- Performance benchmarking between solver implementations.
- Claims of practical trading or investment usefulness.

Each deferred item requires a separate scope and evidence gate before work begins.

## 4. Accepted limitations and risks

| ID | Limitation / risk | Impact | Mitigation | Owner | Review trigger |
|---|---|---|---|---|---|
| R-001 | The visible puzzle structure does not, by itself, fully specify the mapping and output rules | A solver cannot yet be judged correct or complete | Keep unresolved semantics explicit; do not infer missing rules | Project Owner | New authoritative source information |
| R-002 | Jane Street puzzle content is third-party material | Redistribution or licensing risk | Keep source links and limited observations only; do not copy full content or images | Project Owner | Any proposed content inclusion |
| R-003 | A computational puzzle may have no direct financial application | Risk of overstated relevance | Require a separate transfer analysis and accept a negative conclusion | Project Owner | Before publication |
| R-004 | One implementation may contain correlated modelling and coding errors | False confidence in a result | Use an independently reasoned verifier and adversarial tests if implementation is authorized | Project Owner | Before solver results are claimed |

## 5. Change history

| Date | Decision ID | Change | Author | Pull Request / Issue |
|---|---|---|---|---|
| 2026-11-05 | D-001–D-008 | Initial approved project baseline | Kenny | Issue #29; PR #29 |
| 2026-11-05 | D-009 | Rust-first implementation direction adopted for justified computational software | Kenny | PR #29 |
