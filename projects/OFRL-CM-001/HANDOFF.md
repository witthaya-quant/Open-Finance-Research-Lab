<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Agent Handoff and Continuation Protocol

**Project:** OFRL-CM-001 — Computational Methods & Problem-Solving Case Studies  
**Repository:** `witthaya-quant/Open-Finance-Research-Lab`  
**Canonical project-management record:** [PROJECT_CONTROL.md](PROJECT_CONTROL.md)  
**Practice methodology:** [PRACTICE_PLAYBOOK.md](PRACTICE_PLAYBOOK.md)  
**Current admission PR:** [#29](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29)  
**Authorized working branch:** `research/ofrl-cm-001-admission-and-pilot`

## 1. Non-negotiable first read

At the start of every new chat, agent session, or delegated task:

1. Read `PROJECT_CONTROL.md` from the live authorized branch. It is the only project-level source of truth for current scope, status, phases, tasks, dependencies, blockers, and next actions.
2. Read this file for continuation and repository-safety procedure.
3. Fetch live PR #29 metadata and verify repository, state, base, head branch, exact head SHA, Draft/merged state, and mergeability.
4. Fetch live `main` SHA and compare `main...${branch}`; record ahead/behind counts and merge base.
5. Fetch the full PR changed-file list and diff. Confirm all paths remain within the task's authorized path set and project boundary.
6. Inspect PR comments, reviews, review threads, and workflow runs for the exact head SHA.
7. Read only the specialist documents needed for the selected task ID in `PROJECT_CONTROL.md`.
8. Work only on the smallest evidence-backed task. Update the task register and specialist evidence when acceptance criteria are met.

Never use a SHA in a prior conversation, handoff, or copied report as a write token. Live GitHub state must be fetched before every write batch.

## 2. Source-of-truth boundaries

- `PROJECT_CONTROL.md`: sole project-management truth.
- `project.json`: machine-readable project metadata.
- `DECISION_LOG.md`: historical approvals and decision rationale.
- `VALIDATION_MATRIX.md`: validation evidence and test outcomes.
- `PRACTICE_PLAYBOOK.md`: learner-facing methodology and assessment.
- `RESEARCH_SPECIFICATION.md`, `CASE_STUDY_PROTOCOL.md`, and `IMPLEMENTATION_APPROACH.md`: specialist project methods and engineering rules.
- Case files: case-specific facts, hypotheses, formalization, derivations, and unresolved questions.
- `DATA_MANIFEST.md`: provenance, rights, and retrieval limitations.
- `README.md`: navigation only.

Repository-root policies have higher authority than project-local documents. GitHub live refs, PR state, reviews, and CI are authoritative for repository operations.

Do not create another roadmap, status dashboard, task tracker, project charter, or duplicate agent handoff file.

## 3. Delegation contract for external agents

Every delegated task must carry a complete task envelope:

- Task ID from `PROJECT_CONTROL.md`.
- Repository and authorized branch.
- Exact starting head SHA obtained live.
- Exact permitted paths.
- Objective and expected artifact.
- Preconditions and dependencies.
- Acceptance criteria and validation commands/evidence.
- Prohibited paths and actions.
- Required report with changed paths, evidence, test results, limitations, and proposed task-status transition.

Agents must not create additional files, branches, issues, PRs, dependencies, or abstractions unless explicitly allowed by the task or the project owner. Conflicting or ambiguous instructions mean stop and report; do not guess.

## 4. GitHub write safety

- Repository: `witthaya-quant/Open-Finance-Research-Lab` only.
- Project boundary: `projects/OFRL-CM-001/**`; `PROJECT_CATALOG.md` only for an explicitly authorized catalog change.
- Never modify `projects/OFRL-IB-001/**`, other project directories, shared schemas, scripts, fixtures, workflows, or root policies without separate explicit authorization.
- Do not force-push, rewrite history, reset, close, retarget, mark PR #29 ready, merge PR #29, or merge `main` into the working branch unless the owner explicitly authorizes that exact operation.
- Before every write, re-fetch the current PR head. Create a commit parented to that exact head and update the ref non-force with an expected-SHA check. Abort if the head differs.
- After every write, re-fetch PR metadata, changed paths, compare counts, exact-head workflows, and review state.
- Do not create a follow-on branch while PR #29 remains open unless the owner explicitly approves a lifecycle change. Do not place solver implementation or unrelated framework work into the admission PR.
- No Cargo workspace, solver, benchmark suite, or project-specific software workflow until an admitted case passes formalization and method-selection gates.

## 5. Research and evidence discipline

- Separate observed facts, formal definitions, assumptions, hypotheses, outputs, and independently verified claims.
- Contextual sources do not prove that a puzzle author intended a candidate interpretation.
- A candidate returned by a solver is not proof of soundness, completeness, optimality, or uniqueness.
- Record exact residuals, failed hypotheses, counterexamples, and unresolved semantics. Never normalize evidence to fit a preferred answer.
- Distinguish exact arithmetic, numerical approximation, statistical estimate, and heuristic result.
- Do not claim financial applicability or trading performance without a separate defensible mapping and relevant empirical/economic validation.
- Respect the Data Policy, License Policy, and Publication/IP Boundary. Public availability is not redistribution permission.
- CM-T0008's H-017/H-018 candidate-screening round is **PASS — screen complete; no official Jane Street mapping accepted**. Do not reopen completed negative rosters without new independent evidence. Queen left/right remains ambiguous; the 🧐-to-Monopoly link is not accepted; D.C. locality geometry is approximate; orchestra seating is not universal.
- The pinned third-party `solver.py` has a two-sided multiset-comparison defect; do not rely on it as an executable verifier, copy it, or adapt it into OFRL. Use the strict criterion documented in PROBLEM_FORMALIZATION.md H-017 for any further manual cross-check.
- Current next tasks are CM-T0009 (Ø, displayed ordering, residual transformation, terminal notation), CM-T0010 (retrieve the direct official solution target/body if available), and CM-T0011 (independent derivation and correctness/completeness assessment). The official Archive search index now displays a November “Puzzle Solution” label, but no direct href or body has yet been retrieved.
- H-017's 76/76 conditional exact cover and candidate Antwerp phrase do not authorize solver implementation. VAL-014/015 remain BLOCKED. No Cargo workspace, Rust solver, dependency, benchmark, or project-specific workflow until the formalization and correctness-predicate gates pass.

## 6. End-of-task protocol

Before handing work back:

1. Update the specialist evidence document(s).
2. Update the task status and evidence reference in `PROJECT_CONTROL.md` only when acceptance criteria have been checked.
3. Update `VALIDATION_MATRIX.md` for tests and validation evidence; do not use it as a roadmap.
4. Verify project manifest/document links when affected.
5. Re-fetch the exact GitHub head and CI for the submitted commit.
6. Report repository, branch, PR, head SHA, exact changed paths, validation/CI, remaining blockers, and next task ID.
7. Leave PR #29 Draft and unmerged unless the owner explicitly instructs otherwise.

If project state conflicts with this protocol or the control register, stop writing, inspect live evidence, and report the discrepancy. Do not silently reconcile conflicting sources by guesswork.
