<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Decision Log

**Project ID:** OFRL-CM-001  
**Project owner:** Kenny (Witthaya Maidee)  
**Baseline version:** 0.3.0  
**Last reviewed:** 2026-11-07

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
| D-010 | Inherit OFRL-wide policies by reference and enforce a project-root / case-directory boundary | Approved | Common governance, data, research, security, dependency, and release policies remain authoritative at the repository root; CM-001 keeps project-specific decisions and case evidence in its own tree, creates case folders only after admission, and avoids duplicated or speculative structure | README.md shared-policy references; CASE_STUDY_PROTOCOL.md §§9.1–9.3; project.json required_documents; HANDOFF.md |
| D-011 | Expand OFRL-CM-001 into a Quantitative Problem-Solving Research Programme | Approved by Founder on 2026-11-05 | The case-study objective includes a learner-facing practice Playbook, strategic reasoning/game theory, mathematics in finance, quantitative computational methods, and a gated Rust workbench. This does not authorize live trading, an unformalized solver, or indiscriminate software implementation. | PROJECT_CONTROL.md §§2–5; PRACTICE_PLAYBOOK.md; RESEARCH_SPECIFICATION.md |
| D-012 | Establish PROJECT_CONTROL.md as the sole project-management source of truth | Approved by Founder on 2026-11-05 | Current scope, workstreams, phase status, task register, blockers, and next actions must exist in one canonical register. README and HANDOFF are navigation/procedure; specialist documents own only their domain evidence. | PROJECT_CONTROL.md §1; README.md; HANDOFF.md |
| D-013 | Classify the programme as Research and Quantitative Research | Approved by Founder on 2026-11-05 | The programme studies quantitative research methods and financial mathematics as well as general mathematical problem-solving. Empirical controls apply when an admitted case uses empirical or market data. | project.json; RESEARCH_SPECIFICATION.md; root PROJECT_STANDARDS.md |
| D-014 | Defer the Software project type and executable Workbench authorization until the first implementation case is admitted | Approved by Founder on 2026-11-05 | The Rust workbench is an approved target direction, but no executable code, build workflow, Cargo workspace, or dependency is authorized until a case passes formalization and method-selection gates and software architecture/review controls are satisfied. | IMPLEMENTATION_APPROACH.md; PROJECT_CONTROL.md P3; root PROJECT_STANDARDS.md and PROJECT_LIFECYCLE.md |

| D-015 | Admit Finite-Binomial American Put — Optimal Stopping as OFRL-CM-001-CS-002 and the first computational case | Approved by Founder on 2026-11-07 | The finite-horizon no-dividend binomial American Put has explicit state transitions, no-arbitrage conditions, payoff, stopping set and Bellman acceptance predicate. It supports exact, reproducible dynamic-programming computation and independent optimal-stopping verification. This is a research-only synthetic case; it does not reopen Pressure Points and does not authorize market-data calibration, production valuation, investment advice or trading. | cases/2026-11-american-put/README.md; PROBLEM_FORMALIZATION.md; DATA_MANIFEST.md SRC-068–SRC-069 |
| D-016 | Establish the initial case-specific Rust architecture and software controls for CS-002 | Approved for execution under the Founder-authorized CM-T0013 mandate; recorded 2026-11-07 | Use one library package at cases/2026-11-american-put/workbench, exact arbitrary-precision rational arithmetic (num-rational/num-bigint), a full-node valuation certificate, a separately implemented Bellman verifier, explicit lattice-node resource limits and path-scoped project CI. Keep the case research-only at R1; re-assess the risk tier before any decision-support or reusable public release. Add the Software project type and validation profile because the admitted deliverable contains executable code. Do not add a workspace, CLI, benchmark framework, extra crate, or generic abstraction. | ENGINEERING_ARCHITECTURE.md; IMPLEMENTATION_APPROACH.md §11; case PROBLEM_FORMALIZATION.md; project-local CI workflow |

## 2. Open decisions

| ID | Question | Options / trade-offs | Owner | Target review | Status |
|---|---|---|---|---|---|
| O-001 | What are the complete semantics and expected output of Pressure Points? | Obtain an authoritative rule explanation or solution reference; otherwise pause the pilot | Project Owner | Before method selection | Open |
| O-002 | Is there a well-defined mathematical model that supports a meaningful computational comparison? | Formal proof, one implementation, multiple methods, or no implementation | Project Owner | After O-001 | Open |
| O-003 | Is there a defensible financial-engineering transfer? | Specific bounded mapping, indirect methodological relevance, or no transfer | Project Owner | After formalization | Open |
| O-004 | Does each emoji define four cardinal vertices, creating a bijection with the 76 visible marker/sequence entries? | Reconstruct and verify all 76 assignments; reject the hypothesis if the evidence does not support it | Project Owner | Before method selection | Open |
| O-005 | Does “(11 12)” define the lengths of a two-word output, and is ANTWERPSCHE DIAMANTKRING the valid output? | Derive the output schema from the puzzle and independently verify the candidate phrase; length agreement alone is insufficient | Project Owner | Before any solution claim | Open |
| O-006 | Can the 19 emoji clues be assigned to 19 complete four-member semantic groups and cardinal orientations? | Establish all groups, account for all 76 entries including Ø, and derive the orientation rule | Project Owner | Before method selection | Open |
| O-007 | Do candidate word-set residuals encode a consistent letter-reallocation operation? | Test one rule against all groups; reject it if exceptions or hand-selected substitutions remain | Project Owner | Before method selection | Open |

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
| 2026-11-05 | D-009 | Project implementation-language default recorded for justified computational software | Kenny | PR #29 |
| 2026-11-05 | D-010 | Shared-policy inheritance and controlled project/case directory boundaries | Kenny | PR #29 |
| 2026-11-05 | D-011–D-014 | Quantitative practice programme, single PM source of truth, programme classification, and gated Rust workbench | Founder-authorized; recorded by project aide | PR #29 project-scope documentation update |
| 2026-11-07 | D-015 | Admit the finite-binomial American Put as CS-002 and the first computational case | Founder-authorized | Current CM-001 workstream |
| 2026-11-07 | D-016 | Record the case-specific exact-rational Rust architecture, verifier and software controls | Approved for execution within the authorized scope | ENGINEERING_ARCHITECTURE.md; CM-T0013 |
