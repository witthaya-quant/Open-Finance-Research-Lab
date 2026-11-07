<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Agent Handoff and Continuation Protocol

**Project:** OFRL-CM-001 — Computational Methods & Problem-Solving Case Studies  
**Repository:** `witthaya-quant/Open-Finance-Research-Lab`  
**Canonical project-management record:** [PROJECT_CONTROL.md](PROJECT_CONTROL.md)  
**Practice methodology:** [PRACTICE_PLAYBOOK.md](PRACTICE_PLAYBOOK.md)  
**Admission PR (merged):** [#29](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29) — merge commit `42e4a806c950f6efebc260e636e8ff3b98f51dd8`  
**Working branch:** Task-specific follow-on branches must be created from the verified live `main` head; this procedure file does not designate a permanent active branch.

## 1. Non-negotiable first read

At the start of every new chat, agent session, or delegated task:

1. Fetch live `main` and read `PROJECT_CONTROL.md` from that revision. It is the only project-level source of truth for current scope, status, phases, tasks, dependencies, blockers, and next actions.
2. Read this file for continuation and repository-safety procedure.
3. Verify that admission PR #29 remains closed/merged as `42e4a806c950f6efebc260e636e8ff3b98f51dd8`; treat its old admission branch as historical and read-only.
4. Identify the branch and PR for the selected task from live GitHub state. If the task has no active branch, create one from the freshly verified `main` SHA; never infer a write target from a prior conversation.
5. Compare the task branch with `main`; record ahead/behind counts and merge base.
6. Fetch the active task PR's full changed-file list and diff, if a PR exists. Confirm all paths remain within the task's authorized path set and project boundary.
7. Inspect task-PR comments, reviews, review threads, and workflow runs for its exact head SHA.
8. Read only the specialist documents needed for the selected task ID in `PROJECT_CONTROL.md`.
9. Work only on the smallest evidence-backed task. Update the task register and specialist evidence when acceptance criteria are met.

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
- Do not force-push, rewrite history, reset, reopen, retarget, or otherwise mutate PR #29 or its merged admission branch. PR #29 was merged as `42e4a806c950f6efebc260e636e8ff3b98f51dd8`.
- Every material follow-on task must use a separate CM-001 topic branch created from the freshly fetched `main` head and a separate PR. Do not repurpose the admission branch.
- Before every write, re-fetch the current task branch head. Create commits parented to that exact head and update the ref non-force with an expected-SHA check. Abort if the head differs.
- After every write, re-fetch active task-PR metadata, changed paths, compare counts, exact-head workflows, and review state.
- Do not merge `main` into a follow-on branch unless the specific integration need exists and the owner authorizes that exact operation. Do not place solver implementation or unrelated framework work into a documentation/control PR.
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
- CM-T0009 is **PASS — bounded rejection of H-017 as a complete source-grounded explanation**; this does not disprove the conditional candidate answer or solve the puzzle. Current next tasks are CM-T0010 (retrieve and inspect a direct official solution target/body if Jane Street exposes one) and CM-T0011 (independent derivation and correctness/completeness assessment only after an authoritative rule set and acceptance predicate exist). As of 2026-11-07 the live Archive shows “Solution” as unlinked text; no direct target/body has been retrieved.
- H-017's 76/76 conditional exact cover and candidate Antwerp phrase do not authorize solver implementation. VAL-014/015 remain BLOCKED. No Cargo workspace, Rust solver, dependency, benchmark, or project-specific workflow until the formalization and correctness-predicate gates pass.

## 6. End-of-task protocol

Before handing work back:

1. Update the specialist evidence document(s).
2. Update the task status and evidence reference in `PROJECT_CONTROL.md` only when acceptance criteria have been checked.
3. Update `VALIDATION_MATRIX.md` for tests and validation evidence; do not use it as a roadmap.
4. Verify project manifest/document links when affected.
5. Re-fetch the exact GitHub head and CI for the submitted commit.
6. Report repository, branch, PR, head SHA, exact changed paths, validation/CI, remaining blockers, and next task ID.
7. Leave follow-on PRs open and unmerged unless the owner explicitly authorizes the specific merge operation. PR #29 is already merged and must not be treated as pending.

If project state conflicts with this protocol or the control register, stop writing, inspect live evidence, and report the discrepancy. Do not silently reconcile conflicting sources by guesswork.
