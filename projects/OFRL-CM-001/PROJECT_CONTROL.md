<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Project Control & Delivery Register

**Project ID:** `OFRL-CM-001`  
**Project:** Computational Methods & Problem-Solving Case Studies  
**Owner:** Kenny (Witthaya Maidee)  
**Control version:** 1.0.7  
**Last reviewed:** 2026-11-07  
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
| WS-04 — Rust Computational Workbench | Provide exact/reproducible computational capabilities behind formal specifications | **IN PROGRESS — first case-specific prototype admitted; architecture and numeric policy defined** | IMPLEMENTATION_APPROACH.md and ENGINEERING_ARCHITECTURE.md |
| WS-05 — Research validation and publication | Ensure correctness, independent checks, rights, reproducibility, and claim maturity | **ONGOING — controls defined; case-specific gates remain open** | VALIDATION_MATRIX.md and root policies |

## 5. Delivery phases and gates

A phase may close only when its exit criteria are met. Work may proceed in parallel only where dependencies and file ownership do not conflict.

| Phase | Outcome | Exit criteria | Status |
|---|---|---|---|
| P0 — Repository baseline and PR hygiene | Safe, current GitHub working baseline | Live refs checked; changed paths isolated; divergence/integration strategy explicitly resolved; handoff and validation references fresh; exact-head CI reviewed | **COMPLETE — PR #29 merged after exact-head required checks passed; post-merge scope and isolation verified** |
| P1 — Scope and Playbook design | Approved project mandate and initial practice methodology | Project types/profiles match mandate; canonical control register exists; practice workflow, tracks, hypothesis record, method selection, verification, assessment, and learning progression documented; manifest and links validate | **COMPLETE — initial mandate and practice baseline remain valid; first software-bearing case is admitted under D-015/D-016** |
| P2 — Pressure Points mathematical research | Solved case or defensible no-go/incomplete disposition | All source-observed elements accounted for; hypotheses tested/rejected; semantics, output condition, derivation, and verification established—or explicit evidence-based no-go recorded | **BLOCKED — see WS-01** |
| P3 — First Rust computational prototype | Smallest justified executable implementation | A case passes formalization and method-selection gates; architecture, numeric policy, solver/verifier boundary, tests, dependencies, reproducibility, and project CI are approved | **IN PROGRESS — CS-002 admitted; initial exact-rational package and project-local CI are under validation** |
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
| CM-T0001 | P0 | Re-audit PR #29, exact head, main divergence, changed paths, reviews, and CI | **PASS — live pre-merge audit and exact-head CI verified** | Pre-merge PR head `70975d0717e0d1a72ea1c446e3be7fc0cd3ff0ca`; approved update-branch merge commit `b4d8ead6edee59f39ac00baa12d96cd8085c0b61`; final PR merge commit `42e4a806c950f6efebc260e636e8ff3b98f51dd8`. Fourteen changed paths remained isolated; required checks passed on the exact updated head. See §7.2 |
| CM-T0002 | P0 | Resolve branch integration and PR #29 merge readiness | **PASS — Founder-authorized integration completed** | Founder authorized a one-time non-force merge of `main` into the research branch. Updated HEAD `b4d8ead6edee59f39ac00baa12d96cd8085c0b61` was zero commits behind `main`; six required status contexts and the supplemental Numeric envelope derivation passed. PR #29 was marked Ready and merged with a normal merge commit as `42e4a806c950f6efebc260e636e8ff3b98f51dd8` on 2026-11-07. Final PR diff remained exactly 14 CM-001 paths. No rebase, force-push, ruleset bypass, or unrelated file change. See §7.2 |
| CM-T0003 | P0/P1 | Establish one canonical project-management register and remove duplicate handoff/roadmap authority | **PASS** | PROJECT_CONTROL.md is canonical; HANDOFF.md is procedure-only; exact-head Repository Policy run #42017737205 passed |
| CM-T0004 | P1 | Define project mandate, workstreams, phase gates, and delivery acceptance | **PASS** | PROJECT_CONTROL.md §§2–6 and DECISION_LOG.md D-011–D-014; exact-head Repository Policy run #42017737205 passed |
| CM-T0005 | P1 | Establish the initial Quantitative Practice Playbook | **PASS — initial v0.1.0 baseline** | PRACTICE_PLAYBOOK.md content audit; exact-head Repository Policy run #42017737205 passed |
| CM-T0006 | P1 | Align manifest, README, research specification, implementation approach, protocol, and validation links with the approved scope | **PASS** | project.json v0.2.0, PROJECT_CATALOG.md type row, README and specialist cross-links; Repository Policy run #42017737205 passed |
| CM-T0007 | P2 | Independently audit all 12 current Pressure Points candidate clusters and their residuals | **PASS — audit complete; no candidate promoted to fact** | PROBLEM_FORMALIZATION.md §4 CM-T0007 audit: 48 distinct source assignments, 25 exact / 23 residual-bearing comparisons, 13 surplus / 11 deficit letters, semantic/positional evidence graded separately; no global rule derived |
| CM-T0008 | P2 | Investigate the remaining emoji clues and unassigned visible items without assuming the 19×4 model | **PASS — H-017/H-018 candidate screening complete; no official mapping accepted** | H-017's third-party rosters were independently audited with strict two-sided multiset checks: 19 fixed rosters have one assignment each, conditionally covering 76/76 entries once (38 exact, 19 surplus, 19 deficit; Ø→U). H-018 completed source-led membership/positional challenges for the previously open clues and documented the remaining limits: Queen's Deacon/Taylor left-right swap; Barcelona's dynamic rather than fixed diamond; Sistine Madonna, Pigpen and Beijing relative layouts; approximate D.C. locality geometry; a conditional 45° Monopoly board transform but no accepted monocle-to-Monopoly clue link; and no universal orchestra seating order. No mapping is accepted as Jane Street's intended rule. See PROBLEM_FORMALIZATION.md H-017/H-018, DATA_MANIFEST.md SRC-045–SRC-067, and VALIDATION_MATRIX.md TEST-035–045. CM-T0008 closes screening only; P2 remains BLOCKED with U-001–U-010, VAL-014/015, T0010–T0011, and the solver/workbench gate NO-GO |
| CM-T0009 | P2 | Determine the role of Ø, sequence ordering, residual transformation, and terminal notation | **PASS — current H-017 model rejected as a complete source-grounded explanation** | PROBLEM_FORMALIZATION.md § CM-T0009 records the rejection test and limits: Ø→U, roster/term order, 2/1/1 residual extraction, and (11 12) interpretation remain third-party assumptions or unresolved semantics. Candidate output is retained as conditional, not rejected as false. TEST-046 PASS. U-001–U-010, VAL-014/015, and the Pressure Points case remain open/blocking |
| CM-T0010 | P2 | Retrieve and inspect a direct official solution, if Jane Street publishes an actual linked target | **BLOCKED — no direct official target exposed as of 2026-11-07** | The live Archive page lists the November puzzle and renders “Solution” as unlinked text; the “Puzzle” link opens the puzzle/submission page, not a solution. Search-index text is not a destination. Direct official URL/body remain NOT RETRIEVED; see DATA_MANIFEST.md SRC-003/§4 and VALIDATION_MATRIX.md TEST-044/VAL-017 |
| CM-T0011 | P2 | Independently derive output and establish soundness/completeness/uniqueness claims as applicable | **BLOCKED** | Formal specification and acceptance predicate required |
| CM-T0012 | P3 | Select and admit the first case suitable for Rust computational implementation | **PASS — CS-002 admitted** | Founder authorization dated 2026-11-07 selects Finite-Binomial American Put — Optimal Stopping as the first computational case. Case source/rights review, mathematical domain, no-arbitrage conditions, output predicate, and research-only boundaries are recorded in cases/2026-11-american-put/README.md and PROBLEM_FORMALIZATION.md. This admission does not reopen Pressure Points or authorize live pricing/trading use. |
| CM-T0013 | P3 | Approve the smallest Rust workbench architecture, numeric policy, and independent verifier | **PASS — initial case-specific design recorded** | ENGINEERING_ARCHITECTURE.md and DECISION_LOG.md D-016 define one Rust library package, exact BigRational arithmetic, checked lattice sizing, caller-supplied resource limits, an independently checked Bellman optimality certificate, Rust 1.85.0 MSRV, Rust 1.99.1 compatibility CI, and no CLI/workspace/generic framework. Implementation validation remains tracked separately by CM-T0016. |
| CM-T0016 | P3 | Implement and validate the first case-specific Rust workbench package | **IN PROGRESS — initial implementation submitted for validation** | The American Put library, full-node valuation certificate, independent Bellman verifier, exact-rational reference test, invalid-input/resource-limit tests, two-toolchain CI, committed lockfile, dependency review, and reproducibility record must pass on the exact submitted PR head. No release or production-pricing claim is implied. |
| CM-T0014 | P4 | Admit the next practice case after first-case lessons are reviewed | **NOT STARTED** | Candidate intake, source/rights review, owner admission |
| CM-T0015 | P5 | Prepare release assessment for Playbook, research case(s), and software separately | **NOT STARTED** | Each deliverable meets its own evidence and release gates |

## 7. Integration history and verified repository state

### 7.1 Pre-integration audit (historical; superseded)

The pre-merge review was conducted against PR head `70975d0717e0d1a72ea1c446e3be7fc0cd3ff0ca`, with `main` at `cdbf345c6e27f1cd82f3419b164eb4a65575c09b`. The PR contained 14 paths limited to `PROJECT_CATALOG.md` and `projects/OFRL-CM-001/**`. The 52 paths changed on the `main` side since the merge base had zero path overlap with the PR's 14 paths. All three workflows and seven associated check contexts passed on that pre-merge head.

At that checkpoint, strict required-status policy rejected merging because the branch was behind `main`. The prior recommendation was not to rebase or merge `main` into the research branch without a separate owner decision. The Founder subsequently approved a one-time non-force merge commit; §7.2 records the completed operation. This subsection is historical evidence only and is not the current repository state.

### 7.2 Post-integration verification (2026-11-07)

**Repository:** `witthaya-quant/Open-Finance-Research-Lab`  
**Admission PR:** [#29 — OFRL-CM-001 project admission and Pressure Points research pilot](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29)  
**Admission PR state:** Closed / Merged  
**Final merge commit:** [`42e4a806c950f6efebc260e636e8ff3b98f51dd8`](https://github.com/witthaya-quant/Open-Finance-Research-Lab/commit/42e4a806c950f6efebc260e636e8ff3b98f51dd8)  
**Verified `main` HEAD after merge:** `42e4a806c950f6efebc260e636e8ff3b98f51dd8`

The branch update required by strict status-check policy was completed without rewriting history:
- Pre-update research head: `70975d0717e0d1a72ea1c446e3be7fc0cd3ff0ca`.
- `main` immediately before update: `cdbf345c6e27f1cd82f3419b164eb4a65575c09b`.
- Non-force merge commit: `b4d8ead6edee59f39ac00baa12d96cd8085c0b61`, with the pre-update research head and `main` as its two parents.
- Updated branch compare: zero commits behind `main`; the PR diff remained exactly 14 files.
- PR #29 was marked Ready for Review and merged with the normal `merge` method. GitHub reported `merged: true` and the final merge commit above.

**Exact-head CI on `b4d8ead6edee59f39ac00baa12d96cd8085c0b61`:**
- Repository Policy run [#42351499080](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42351499080): PASS.
- Numeric and Input Contract run [#42351499039](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42351499039): PASS.
- Rust Runtime Semantic Validator run [#42351499065](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42351499065): PASS.

All six required status contexts passed:
`repository-policy`; `Model input JSON Schema`; `Decimal contract (Rust 1.85.0)`; `Decimal contract (Rust 1.99.1)`; `Semantic validator (Rust 1.85.0)`; and `Semantic validator (Rust 1.99.1)`. The supplemental `Numeric envelope derivation` check also passed. These are repository/software checks; they do not prove that Pressure Points has been solved.

**Final merged file scope — 14 paths:**
- `PROJECT_CATALOG.md`
- `projects/OFRL-CM-001/CASE_STUDY_PROTOCOL.md`
- `projects/OFRL-CM-001/DATA_MANIFEST.md`
- `projects/OFRL-CM-001/DECISION_LOG.md`
- `projects/OFRL-CM-001/HANDOFF.md`
- `projects/OFRL-CM-001/IMPLEMENTATION_APPROACH.md`
- `projects/OFRL-CM-001/PRACTICE_PLAYBOOK.md`
- `projects/OFRL-CM-001/PROJECT_CONTROL.md`
- `projects/OFRL-CM-001/README.md`
- `projects/OFRL-CM-001/RESEARCH_SPECIFICATION.md`
- `projects/OFRL-CM-001/VALIDATION_MATRIX.md`
- `projects/OFRL-CM-001/cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md`
- `projects/OFRL-CM-001/cases/2026-11-pressure-points/README.md`
- `projects/OFRL-CM-001/project.json`

**Isolation and security verification:**
- The main protection ruleset `OFRL-main-protection` (ID `26516806`) remained active with strict required checks and no bypass actors. No ruleset change or bypass was used.
- A Git-tree comparison of all 56 entries under `projects/OFRL-IB-001/` before and after PR #29 found identical paths, modes, types, and blob SHAs.
- No IB-001 file, branch, PR, or issue was changed as part of CM-001 integration.
- The final merged PR diff contained no path outside the 14 entries listed above.

**Live-state rule:** This section is a dated verification record, not a substitute for GitHub state. At the start of every subsequent task, fetch the current `main` SHA and the task-specific CM-001 branch/PR (if one exists), compare refs, inspect the exact changed paths, reviews, and exact-head CI. PR #29 and its admission branch are historical/merged artifacts and must not be treated as an active work branch.

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
- Do not force-push, rewrite history, reset, retarget, reopen, or otherwise mutate PR #29 or its merged admission branch. That integration is complete and remains historical.
- Follow-on CM-001 work must use a task-specific branch created from the freshly fetched `main` head and a separate reviewable PR. Do not repurpose the merged admission branch.
- Before each write, re-fetch the current task branch head and use a non-force, expected-SHA/lease-checked update. Abort on a mismatch.
- Do not merge `main` into a follow-on working branch unless a concrete integration requirement exists and the owner authorizes that exact operation.
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

CM-T0008 screening and CM-T0009's bounded model-rejection task are complete; do not reopen either without new independent evidence. CM-T0010 remains BLOCKED: the official Archive discovery result displays a November 2026 “Puzzle Solution” label, but the retrieved official material does not provide a usable direct solution target or worked solution body. A search-index label is not a destination; do not infer a URL. CM-T0011 remains BLOCKED until an authoritative rule set and correctness predicate exist. Keep Pressure Points INCOMPLETE / BLOCKED while U-001–U-010 and VAL-014/VAL-015 remain unresolved. CS-002, Finite-Binomial American Put — Optimal Stopping, is now the admitted first computational case. CM-T0012 and CM-T0013 are PASS on the documented admission and case-specific architecture; CM-T0016 tracks implementation and exact-head validation. Do not create further case directories, generic abstractions, CLI, benchmark suite, or additional packages until the first case's lessons justify them. CM-T0014 remains gated on first-case review; CM-T0015 remains a separate release assessment. All work remains research-only: no market data, live pricing service, investment recommendation, or execution.
