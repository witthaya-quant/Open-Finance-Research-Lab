<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Project Control & Delivery Register

**Project ID:** `OFRL-CM-001`  
**Project:** Computational Methods & Problem-Solving Case Studies  
**Owner:** Kenny (Witthaya Maidee)  
**Control version:** 1.0.0  
**Last reviewed:** 2026-11-05  
**Lifecycle:** In Development  
**Risk tier:** R1 — Analytical  
**Evidence maturity:** 0 — Proposed  
**Canonical project-management record:** YES

> **This is the one authoritative project-management register for OFRL-CM-001.** Project scope, workstream status, milestones, task ownership, dependencies, blockers, and next actions must be maintained here. Other project documents have specialist purposes and must link back to this register rather than maintain competing roadmaps or status boards.

## 1. Authority and single-source-of-truth rules

### 1.1 Authority hierarchy

1. **OFRL repository-root policies** remain the mandatory common baseline and override conflicting project-local text.
2. **This Project Control document** is the sole source of truth for the current OFRL-CM-001 mandate, scope, workstreams, phase/milestone status, task register, dependencies, blockers, and next actions.
3. **GitHub live state** is authoritative for repository facts that change independently of documents: branch refs, PR state, mergeability, reviews, changed paths, commits, and CI runs. The snapshot in this file is a dated record, never a substitute for a live check.
4. **project.json** is the machine-readable source of project identity, lifecycle, types, risk tier, maturity, validation profiles, data classification, and required documents. It is not a second task tracker.
5. **DECISION_LOG.md** is the historical register of approved and superseded decisions. The current delivery state remains in this file.
6. **VALIDATION_MATRIX.md** is the evidence register for validation requirements, test outcomes, and failures. It is not a roadmap.
7. Specialist documents own only their subject matter: research methods, practice methodology, case facts/formalization, source provenance, and implementation controls.
8. **README.md** is a navigation and public summary only. **HANDOFF.md** contains agent continuation procedures only.

### 1.2 No parallel project-management systems

- Do not create a second roadmap, status dashboard, task board file, milestone tracker, agent handoff summary, or duplicate project charter.
- Do not copy the task register into Issues, PR descriptions, README tables, or chat summaries. GitHub Issues/PRs are used for reviewable work and discussion; this document remains the canonical project task register.
- An agent may maintain task-specific working notes only within an explicitly assigned task and approved path. At task completion, material findings must be integrated into the relevant specialist document and this register.
- If a status differs between this document and another project document, update the specialist document to link here; do not maintain two competing status values.
- Do not create a new file or directory unless a task in this register or an approved Decision Log entry explicitly authorizes it.

### 1.3 Update protocol

Every project-level status change must record:
- Task or milestone ID.
- Previous status and new status.
- Evidence or GitHub reference supporting the transition.
- Commit/PR containing the change, where applicable.
- Remaining limitations or next dependency.

A status may become **PASS / COMPLETE** only after its stated acceptance criteria have been checked. A commit existing, an agent claiming success, or CI passing alone is not evidence of mathematical correctness.

## 2. Project mandate

OFRL-CM-001 develops disciplined quantitative problem-solvers through source-grounded case studies, mathematical reasoning, strategic analysis, and reproducible computational research.

The programme teaches and documents the complete reasoning process:
1. Understand and decompose a problem.
2. Generate competing hypotheses.
3. Formalize the problem mathematically.
4. Select and justify an analytical or computational method.
5. Derive, calculate, or simulate a result.
6. Independently verify the result.
7. Challenge assumptions with counterexamples, sensitivity, and robustness tests.
8. Assess whether the mathematical structure has a defensible application to quantitative finance.
9. Record limitations, failed approaches, and reproducible evidence.

Rust is the preferred computational implementation language when executable computation is justified. It is a means of making calculations explicit, deterministic where possible, testable, and reproducible—not an objective that requires every case to become software.

## 3. Scope boundary

### 3.1 Included

- Mathematical and computational problem-solving case studies.
- Hypothesis formation, falsification, formalization, proof, and method-selection practice.
- Combinatorics, discrete mathematics, logic, graph theory, probability, statistics, information theory, optimization, and algorithm analysis where relevant.
- Strategic reasoning and game theory, including finite games, equilibrium, minimax, Bayesian reasoning, repeated games, auctions, and mechanism design where relevant.
- Mathematics in finance, including time value of money, fixed income, derivatives, stochastic processes, portfolio/risk analysis, hedging, and market microstructure where properly specified.
- Quantitative computational methods: numerical analysis, Monte Carlo, simulation, optimization, sensitivity, and reproducibility.
- A Rust computational workbench, admitted incrementally behind case-specific formalization, design, and validation gates.
- Educational practice material that explains variables, notation, assumptions, derivations, failure modes, and verification.
- Honest financial-engineering transfer analysis, including a documented conclusion that no transfer is justified.

### 3.2 Excluded

- Live trading, order execution, investment recommendations, or production decision support under this project baseline.
- Claims that solving a puzzle predicts trading performance or reveals Jane Street's proprietary methods.
- A generic AI agent, automatic puzzle scraper, automated answer submission, or general-purpose solver framework without a separately approved need.
- A solver implementation for a case whose correctness predicate is undefined.
- Unreviewed third-party puzzle text, artwork, datasets, solutions, or code.
- Changes to OFRL-IB-001 or shared OFRL policies/infrastructure through this project.
- New case folders, crates, benchmarks, or workflows created speculatively.

### 3.3 Scope-change rule

A material change to the research objective, risk, data classification, public interface, software boundary, or release intent requires an owner-approved Decision Log entry and an update to this register before implementation. Root policies cannot be waived locally.

## 4. Canonical workstreams

| Workstream | Purpose | Current status | Canonical evidence |
|---|---|---|---|
| WS-01 — Pressure Points research | Resolve the November 2026 Jane Street case or close it transparently as incomplete/no-go | **BLOCKED — mathematical semantics unresolved** | Case README and PROBLEM_FORMALIZATION.md |
| WS-02 — Practice Playbook | Define the repeatable method for learning, hypothesis generation, analysis, verification, and reflection | **DESIGNED — initial v0.1.0 draft** | PRACTICE_PLAYBOOK.md |
| WS-03 — Mathematical method catalogue | Organize methods by problem structure and explain when each is appropriate | **PLANNED — build incrementally inside the Playbook and admitted cases** | PRACTICE_PLAYBOOK.md and case research records |
| WS-04 — Rust Computational Workbench | Provide exact/reproducible computational capabilities behind formal specifications | **DEFERRED — no admitted implementation case yet** | IMPLEMENTATION_APPROACH.md |
| WS-05 — Research validation and publication | Ensure correctness, independent checks, rights, reproducibility, and claim maturity | **ONGOING — controls defined; case-specific gates remain open** | VALIDATION_MATRIX.md and root policies |

## 5. Delivery phases and gates

A phase may close only when its exit criteria are met. Work may proceed in parallel only where dependencies and file ownership do not conflict.

| Phase | Outcome | Exit criteria | Status |
|---|---|---|---|
| P0 — Repository baseline and PR hygiene | Safe, current GitHub working baseline | Live refs checked; changed paths isolated; divergence/integration strategy explicitly resolved; handoff and validation references fresh; exact-head CI reviewed | **PARTIAL — audit complete; integration decision remains open** |
| P1 — Scope and Playbook design | Approved project mandate and initial practice methodology | Project types/profiles match mandate; canonical control register exists; practice workflow, tracks, hypothesis record, method selection, verification, assessment, and learning progression documented; manifest and links validate | **DOCUMENTED — CI validation pending for this revision** |
| P2 — Pressure Points mathematical research | Solved case or defensible no-go/incomplete disposition | All source-observed elements accounted for; hypotheses tested/rejected; semantics, output condition, derivation, and verification established—or explicit evidence-based no-go recorded | **BLOCKED — see WS-01** |
| P3 — First Rust computational prototype | Smallest justified executable implementation | A case passes formalization and method-selection gates; architecture, numeric policy, solver/verifier boundary, tests, dependencies, reproducibility, and project CI are approved | **NOT STARTED — gated** |
| P4 — Multi-domain practice cases | Demonstrate methodology on different mathematical domains | Each case admitted; at least two real use cases before reusable abstractions; evidence and validation recorded per case | **NOT STARTED** |
| P5 — Verification, benchmark, and Playbook release | Reusable, reproducible learning/research release | Cases and exercises reviewed; answer reasoning and verification criteria available; any benchmark has a defined corpus/protocol; rights, maturity, reproducibility, and release gates pass | **NOT STARTED** |

### 5.1 Phase dependencies

- P0 establishes repository safety; it does not solve the mathematical case.
- P1 defines the programme and its learning method; it does not authorize every future case or every software component.
- P2 is independent of building a generic Rust framework. It may end as SOLVED or INCOMPLETE / NO-GO.
- P3 may use a different admitted, fully specified case if Pressure Points remains blocked. It must not invent a correctness predicate to unblock P2.
- P4 is admission-controlled. One active case study at a time remains the default.
- P5 evaluates the maturity of the Playbook, research results, and software separately.

## 6. Active task register

Only this table is the canonical project task list. Task IDs are stable; statuses change only with evidence.

| Task ID | Phase | Task / required result | Status | Dependency / exit evidence |
|---|---|---|---|---|
| CM-T0001 | P0 | Re-audit PR #29, exact head, main divergence, changed paths, reviews, and CI | **PASS — audit snapshot recorded** | Live GitHub inspection on 2026-11-05; see §7 |
| CM-T0002 | P0 | Resolve branch integration and PR #29 merge readiness | **HOLD — owner decision required** | Main is 212 commits ahead of the merge base; no overlapping paths found in the 42 main-side changed paths; do not rebase/merge or retarget without explicit instruction |
| CM-T0003 | P0/P1 | Establish one canonical project-management register and remove duplicate handoff/roadmap authority | **IMPLEMENTED IN THIS REVISION — CI pending** | PROJECT_CONTROL.md; HANDOFF.md points to this file |
| CM-T0004 | P1 | Define project mandate, workstreams, phase gates, and delivery acceptance | **DOCUMENTED IN THIS REVISION — CI pending** | This file; DECISION_LOG.md D-011–D-014 |
| CM-T0005 | P1 | Establish the initial Quantitative Practice Playbook | **DOCUMENTED IN THIS REVISION — CI pending** | PRACTICE_PLAYBOOK.md |
| CM-T0006 | P1 | Align manifest, README, research specification, implementation approach, protocol, and validation links with the approved scope | **DOCUMENTED IN THIS REVISION — CI pending** | project.json and project-local documents |
| CM-T0007 | P2 | Independently audit all 12 current Pressure Points candidate clusters and their residuals | **OPEN — research task** | Candidate identity/context checks are not proof of puzzle intent; record accepted/rejected hypotheses with explicit falsifiers |
| CM-T0008 | P2 | Account for the remaining seven emoji clues and all 76 proposed positions without assuming the 19×4 model | **BLOCKED** | Requires source-grounded semantics and a total, duplicate-free mapping if that model survives testing |
| CM-T0009 | P2 | Determine the role of Ø, sequence ordering, residual transformation, and terminal notation | **BLOCKED** | Must derive one consistent interpretation or reject the current model |
| CM-T0010 | P2 | Retrieve and inspect a direct official solution, if Jane Street publishes an actual linked target | **OPEN — source-monitoring task** | Recheck the live Archive; never infer a URL from search text |
| CM-T0011 | P2 | Independently derive output and establish soundness/completeness/uniqueness claims as applicable | **BLOCKED** | Formal specification and acceptance predicate required |
| CM-T0012 | P3 | Select the first case suitable for Rust computational implementation | **NOT STARTED** | Case passes formalization and method-selection gates; need not be Pressure Points |
| CM-T0013 | P3 | Approve the smallest Rust workbench architecture, numeric policy, and independent verifier | **NOT STARTED** | CM-T0012 and implementation design review |
| CM-T0014 | P4 | Admit the next practice case after first-case lessons are reviewed | **NOT STARTED** | Candidate intake, source/rights review, owner admission |
| CM-T0015 | P5 | Prepare release assessment for Playbook, research case(s), and software separately | **NOT STARTED** | Each deliverable meets its own evidence and release gates |

## 7. Verified GitHub baseline

**Snapshot date:** 2026-11-05  
**Repository:** `witthaya-quant/Open-Finance-Research-Lab`  
**Active PR:** [#29 — Project admission and Pressure Points research pilot](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29)  
**Authorized branch:** `research/ofrl-cm-001-admission-and-pilot`

Last inspected checkpoint before this control-register revision:
- Branch head: `f9536bc8adf4220a741fa8acfbf29c7f42b88c51` (historical checkpoint only; not a write token).
- Main head observed: `5f8f578c387150214a1f438c3d67da48652eb4cb`.
- PR: Open / Draft / Not merged; GitHub reported mergeable.
- PR changed files: 12, all limited to `PROJECT_CATALOG.md` and `projects/OFRL-CM-001/**`.
- Compare result: branch ahead 16 / behind 212 from merge base `cc2d01b3ffd1d63d32fe87430967a4a86ecbffaa`.
- Main-side changes since that merge base: 42 paths; no path overlapped the 12 PR paths in the inspected comparison.
- PR comments: none; reviews: none; review threads: none.
- Last verified exact-head workflows at the inspected checkpoint:
  - [OFRL Repository Policy #41976641537](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41976641537)
  - [OFRL-IB-001 Numeric and Input Contract Checks #41976641404](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41976641404)
  - [OFRL-IB-001 Rust Runtime Semantic Validator #41976641492](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41976641492)

The two IB-001 workflows are repository-event-triggered checks, not evidence of changes to IB-001 or mathematical correctness for CM-001. Re-fetch PR, branch, main, changed paths, comments/reviews, and exact-head CI at every continuation. Do not copy this snapshot forward as if it were live state.

## 8. Project document ownership map

| Document | Sole responsibility | Must not become |
|---|---|---|
| `PROJECT_CONTROL.md` | Canonical project mandate, status, phases, task register, blockers, and next actions | A duplicate of detailed derivations or test logs |
| `PRACTICE_PLAYBOOK.md` | Learning methodology, hypothesis practice, method selection, verification, curriculum, and rubric | A live project task tracker |
| `project.json` | Machine-readable project metadata and required document registry | A task/status dashboard |
| `README.md` | Navigation and concise project introduction | A competing project plan |
| `RESEARCH_SPECIFICATION.md` | Research objectives, methodology, scope, and research integrity | A second milestone register |
| `CASE_STUDY_PROTOCOL.md` | Reusable lifecycle and case admission/analysis gates | A case-specific answer |
| `DECISION_LOG.md` | Historical approvals, rationale, and supersession | A current task board |
| `VALIDATION_MATRIX.md` | Acceptance checks, test outcomes, and evidence | A project roadmap |
| `DATA_MANIFEST.md` | Source and data provenance, rights, and retrieval limitations | A copy of third-party source data |
| `HANDOFF.md` | Agent startup, verification, permissions, and continuation protocol | A second handoff/status summary |
| `IMPLEMENTATION_APPROACH.md` | Rust implementation principles and gated workbench direction | An authorization to code every case |
| Case `README.md` and `PROBLEM_FORMALIZATION.md` | Case-specific evidence, hypotheses, formalization, derivations, and unresolved semantics | Project-wide scope or milestones |

## 9. Work authorization and agent contract

### 9.1 Task envelope

Every agent or contributor receiving a task must be given:
1. Task ID from §6.
2. Repository, branch, and exact base/head SHA observed at task start.
3. Exact permitted file paths.
4. Objective and required deliverable.
5. Preconditions and dependencies.
6. Acceptance criteria and required validation.
7. Explicit prohibited actions and paths.
8. Required evidence and report format.

An agent must not broaden its task, infer approval from an earlier conversation, invent missing data, or create additional files to make its work appear complete.

### 9.2 Evidence and status transitions

- Hypotheses are not facts; candidate readings are not accepted mappings.
- Contextual identity/position sources do not prove puzzle intent.
- Solver output is not independent verification.
- CI success is repository/software evidence only; it is not a mathematical proof.
- A task with missing prerequisites remains BLOCKED or OPEN; it is never silently marked complete.
- Failed, contradictory, inconclusive, and negative results must be recorded.

### 9.3 Repository safety

- Work only in `witthaya-quant/Open-Finance-Research-Lab` and the authorized CM-001 scope.
- Never modify `projects/OFRL-IB-001/**`, other projects, shared schemas, scripts, fixtures, workflows, or root policies without a separate explicit authorization.
- Do not force-push, rewrite history, reset, retarget, close, merge PR #29, or merge main into the working branch unless the owner explicitly authorizes that exact operation.
- Before each write, re-fetch the live PR head and use a non-force, expected-SHA/lease-checked update. Abort on a mismatch.
- Do not create a follow-on branch while the admission PR is still open unless the owner explicitly changes the lifecycle decision.
- Keep one active case study at a time.
- Do not create Cargo files, solver code, benchmark datasets, workflow files, or extra case directories before their gates are approved.

## 10. Closure definitions

The programme has separate closure states; one must never be used as a substitute for another.

| Deliverable | SOLVED / COMPLETE when | Otherwise |
|---|---|---|
| Pressure Points case | A source-grounded formal specification, complete derivation, independent candidate verification, and justified completeness/uniqueness claims (if required) exist | INCOMPLETE / BLOCKED or documented NO-GO |
| Practice Playbook | Workflow, tracks, hypothesis record, method selection, validation, progression, rubric, and reproducible practice format are documented and reviewed | DRAFT / INCOMPLETE |
| Rust Workbench | An admitted case justifies computation; implementation, independent verifier, tests, locked build, CI, and reproduction evidence pass | NOT STARTED / BLOCKED |
| Project programme | All admitted deliverables reach their separately approved closure criteria and release decisions | IN DEVELOPMENT |

A no-go disposition is a valid research outcome, but it must retain the research evidence and limitations and must not be described as a solved puzzle.

## 11. Next permitted work

The next work item is **CM-T0007 — independently audit and challenge the twelve Pressure Points candidate clusters**, using the case formalization and source manifest as the evidence base. Do not start solver implementation. In parallel, complete review of this initial Playbook design and the new project-control baseline; record any approved changes here.
