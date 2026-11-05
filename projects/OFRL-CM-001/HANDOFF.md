<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Working Handoff and Safe Continuation Record

**Project:** Computational Methods & Problem-Solving Case Studies  
**Project ID:** `OFRL-CM-001`  
**Case:** `OFRL-CM-001-CS-001` — Jane Street, Pressure Points (November 2026)  
**Repository:** `witthaya-quant/Open-Finance-Research-Lab`  
**Pull request:** [#29](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29)  
**Working branch:** `research/ofrl-cm-001-admission-and-pilot`  
**Base branch:** `main`  
**Pre-update checkpoint:** PR open, Draft, not merged; base SHA `cc2d01b3ffd1d63d32fe87430967a4a86ecbffaa`; branch head before this handoff update `dd4bd1b9f686a92b460e71953959960c852ca737`.  
**Important:** These SHAs are historical checkpoints, not current write tokens. A new session MUST retrieve the current PR head SHA and branch state from GitHub before any write.

## 1. Purpose

This file is the continuity and safety contract for a new ChatGPT conversation or another authorized coding agent. It is not a substitute for checking GitHub. It exists to prevent a new session from guessing project state, using the wrong repository or branch, overwriting newer work, or crossing the isolation boundary.

The human project owner is Kenny (Witthaya Maidee). The assistant is a research and engineering aide, not an independent repository owner. Do not claim that work is complete unless the relevant evidence exists on the current PR head.

## 2. Mandatory startup sequence — read before acting

In this order:

1. Open this file from the live PR branch.
2. Fetch PR #29 metadata. Confirm repository, state, base, head branch, current head SHA, and whether it is merged or Draft.
3. Fetch the current branch head and compare `main...research/ofrl-cm-001-admission-and-pilot`. Record ahead/behind counts and the base SHA.
4. Fetch the current PR changed-file list and full diff. Confirm that every changed path remains within the allowed set in §3.
5. Read the current branch versions of:
   - `projects/OFRL-CM-001/README.md`
   - `projects/OFRL-CM-001/RESEARCH_SPECIFICATION.md`
   - `projects/OFRL-CM-001/CASE_STUDY_PROTOCOL.md`
   - `projects/OFRL-CM-001/DECISION_LOG.md`
   - `projects/OFRL-CM-001/VALIDATION_MATRIX.md`
   - `projects/OFRL-CM-001/DATA_MANIFEST.md`
   - `projects/OFRL-CM-001/IMPLEMENTATION_APPROACH.md`
   - `projects/OFRL-CM-001/cases/2026-11-pressure-points/README.md`
   - `projects/OFRL-CM-001/cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md`
6. Read the latest PR comments/reviews and workflow results. Do not assume prior CI status still applies to a new commit.
7. Only after steps 1–6 agree, determine the next smallest evidence-backed task.

If the PR is closed, merged, retargeted, the branch is missing, `main` has moved, the branch has unexpected commits, or the changed-file list contains an unexpected path, STOP WRITING. Inspect and explain the discrepancy first. Do not recreate a branch, reopen or retarget a PR, merge, or reset anything without explicit owner authorization.

## 3. Strict repository and change boundary

### Authorized repository and workstream

- Repository: `witthaya-quant/Open-Finance-Research-Lab` only.
- Project: `OFRL-CM-001` only.
- Existing workstream: PR #29 on `research/ofrl-cm-001-admission-and-pilot`.
- `PROJECT_CATALOG.md` is the one permitted shared portfolio file, and only the OFRL-CM-001 registration row may be changed when necessary.

### Explicitly prohibited

- Do not modify any file under `projects/OFRL-IB-001/`.
- Do not modify, rebase, force-push, close, retarget, or otherwise alter any existing branch, PR, or Issue other than making scoped commits to the already-authorized PR #29 branch.
- Do not modify `.github/`, shared workflows, scripts, schemas, templates, fixtures, repository-wide policy documents, or any other project.
- Do not create a second PR or branch for this case unless the owner explicitly authorizes it.
- Do not merge PR #29, mark it ready for review, or publish/release the project without explicit authorization.
- Do not write to `main`.

### Safe write procedure

Before every write batch, fetch the live PR head SHA again. Prefer one atomic Git-data commit containing only reviewed paths. Update the branch with a non-force fast-forward and an expected-SHA/lease check. If the expected SHA no longer matches, the write must fail; fetch and inspect the new state instead of retrying blindly. Never force-push or overwrite a newer commit.

After a write, fetch PR metadata, changed-file names, compare metadata, and CI for the exact new head. Confirm no prohibited paths were added.

### Project layout and future case admission

The project shares OFRL's root governance and technical policies by reference; it does not copy or fork those policies. Project-local project specifications, protocol, decision log, validation matrix, data manifest, implementation approach, and handoff remain under `projects/OFRL-CM-001/`.

The current admitted pilot is stored only in:
`projects/OFRL-CM-001/cases/2026-11-pressure-points/`

No additional case directories, empty scaffolds, duplicate root documents, or speculative implementation trees should be created. A future case must first pass the candidate-intake, source/rights, and owner admission decision. Add its date-prefixed directory under `cases/` only then, starting with its own README and problem formalization. Shared protocol and project-wide controls remain at the project root.

Project-wide policy inheritance and the project/case directory rules are recorded in D-010. Repository-root policies remain the single source of truth; do not copy them into the project or case folders.

PR #29 is only the admission and first-pilot documentation PR. It remains Draft and unmerged. After it is merged by explicit owner decision, follow-on changes to this project should use a separate topic branch and PR under the same project ID. Do not create a new OFRL project ID for another case unless its research objective, ownership, risk, or independently managed deliverable is materially different. Before PR #29 is merged, do not create follow-on branches based on a main branch that does not yet contain this project.

## 4. Current research state — Pressure Points

### Verified source observations

The official page is [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/). The page contains one leading marker, 75 uppercase-letter sequences, 19 emoji clues, and terminal notation `(11 12)`. The 75 sequences are distinct, internally alphabetized, and displayed in lexicographic order; their length distribution and 504-letter total are documented in `PROBLEM_FORMALIZATION.md`.

Jane Street's [Puzzles introduction](https://www.janestreet.com/puzzles/) explicitly notes that its collection can include puzzles with or without clearly defined rules. This is context, not a solution to this case.

### Newly discovered secondary leads — not verified solutions

- A [Puzzling Stack Exchange discussion](https://puzzling.stackexchange.com/questions/141272/jane-streets-recent-puzzle-pressure-points-nov-2026) records exploratory anagram/geographic interpretations and unresolved questions.
- [Kaia's research notes](https://www.kaiaz.me/now/) (last updated 2026-11-04) report a possible one-to-one mapping between 76 marker/sequence entries and 19 emoji clues × 4 cardinal vertices, plus the candidate `ANTWERPSCHE DIAMANTKRING`.
- The candidate words have lengths 11 and 12, consistent with the displayed `(11 12)`. This agreement alone does not prove the candidate.
- The retrieved secondary note does not contain the complete derivation. The official Archive search-index representation displayed a November 2026 “Puzzle Solution” label, but the opened Archive snapshot was stale and did not expose the November entry or a direct solution page. The official solution has therefore NOT been retrieved or validated. Recheck the Archive and inspect the actual solution page before treating it as authoritative evidence.

See hypotheses H-001 to H-006 and open questions O-001, O-004, O-005, O-006, and O-007 in the project documents. Four candidate theme groups have been partially reconstructed (compass, PlayStation-style face buttons, baseball bases, sailing-vessel positions). Their standard N/E/S/W conventions are independently corroborated by National Geographic, Sony, MLB, NOAA, and the U.S. Naval History and Heritage Command. Candidate residuals are explicitly recorded: compass +E/−E, controller +S/−E, baseball +O/−H, sailing +A/−S. The residuals do not yet yield a common rule. This is partial evidence for 4 of 19 themes, not a complete bijection. Treat the candidate mapping and answer as unverified until all 76 entries are assigned with a reproducible derivation and the output condition is independently established.

### Formalization gate

U-001 through U-010 remain unresolved. The four partial groups do not close the full mapping or letter-residual rule. Method selection remains BLOCKED. Do not select SAT/SMT, CSP, graph, search, string, optimization, or any other solver family merely because the cardinalities match. Do not write solver code, create a Cargo workspace, choose dependencies, or build CI until the rules and acceptance predicate justify those actions.

A no-go or an explicitly incomplete research case is preferable to fabricated semantics. Financial-engineering transfer is not presumed and may conclude that no useful transfer exists.

## 5. Implementation approach

The project Decision Log D-009 records the selected implementation-language direction for justified computational software. `IMPLEMENTATION_APPROACH.md` is the technical source of truth for domain types, deterministic computation, typed failures, candidate-generation and verification boundaries, Cargo controls, testing, and supply-chain review.

Avoid promotional language or repeated stack labels. Describe the actual interfaces, crates, build metadata, and tests when code is legitimately admitted. The language decision belongs in the technical design record; it does not need to be repeated in unrelated headings or project summaries. Pure proof or mathematical analysis may remain code-free.

Do not introduce a general-purpose solver framework, unnecessary crate splitting, unreviewed dependencies, AI orchestration, or unrelated infrastructure.

## 6. Research acceptance evidence before implementation

Before moving beyond formalization, the case must provide:

1. A precise semantic interpretation of the marker and all 75 sequences.
2. A grounded explanation of the 19 emoji clues and any proposed four-position structure.
3. A total one-to-one mapping, if that is the actual model, with no missing or duplicate assignments.
4. A derived output domain and acceptance predicate; interpret `(11 12)` from evidence, not coincidence.
5. Explicit treatment of whether one answer, all answers, uniqueness, or another result is required.
6. A mathematical explanation and independent verification of the candidate output.
7. A documented method-selection rationale and a clear reason why executable software adds value.

Only then design the smallest useful implementation and independent verifier. A candidate generator returning the expected phrase is not enough to establish the model or prove uniqueness.

## 7. CI and release status

Before this handoff update, the following workflows succeeded on head `dd4bd1b9f686a92b460e71953959960c852ca737`:

- [OFRL Repository Policy run #41968623100](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41968623100)
- [OFRL-IB-001 Numeric and Input Contract Checks run #41968623105](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41968623105)
- [OFRL-IB-001 Rust Runtime Semantic Validator run #41968623153](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41968623153)

The two OFRL-IB-001 workflows were triggered by repository event behavior; their execution does not authorize changes to OFRL-IB-001. These results apply only to the checkpoint SHA above and must be rechecked after this handoff commit.

PR #29 must remain Draft and unmerged unless Kenny explicitly instructs otherwise. The project has not been released. No Cargo project, solver, verifier, CLI, dependency selection, or project-specific CI workflow is claimed to exist.

## 8. Required end-of-session report

Every work session should report:

- Current repository, branch, PR state, base SHA, and head SHA.
- Exact changed paths and whether any boundary check failed.
- What was researched or implemented, with source/evidence references.
- Validation/CI results for the exact head SHA.
- Remaining blockers and the next permitted task.
- Whether PR #29 remains Draft and unmerged.

Never report a future action as completed. Never claim independent review where none occurred. Never hide BLOCKED, FAIL, or NOT RUN states.
