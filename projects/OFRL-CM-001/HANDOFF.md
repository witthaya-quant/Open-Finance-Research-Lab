<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Working Handoff and Safe Continuation Record

**Project:** Computational Methods & Problem-Solving Case Studies  
**Project ID:** `OFRL-CM-001`  
**Case:** `OFRL-CM-001-CS-001` — Jane Street, Pressure Points (November 2026)  
**Repository:** `witthaya-quant/Open-Finance-Research-Lab`  
**Pull request:** [#29](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29)  
**Working branch:** `research/ofrl-cm-001-admission-and-pilot`  
**Base branch:** `main`  
**Pre-update checkpoint:** PR open, Draft, not merged; base SHA `cc2d01b3ffd1d63d32fe87430967a4a86ecbffaa`; branch head before this candidate-cluster update `04400787062ecc0b84c6d75b5315d382a84979de`.  
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

- A [Puzzling Stack Exchange question](https://puzzling.stackexchange.com/questions/141272/jane-streets-recent-puzzle-pressure-points-nov-2026) records exploratory anagram/geographic interpretations and explicitly asks about the title, Ø, emoji clues, and “(11 12)”; it is user-generated research, not a rule specification.
- [Kaia's research notes](https://www.kaiaz.me/now/) (reviewed 2026-11-05; page notes updated 2026-11-04) report work toward a one-to-one mapping between 76 marker/sequence entries and 19 emoji clues × 4 cardinal vertices, and state the candidate `ANTWERPSCHE DIAMANTKRING`. The reviewed page does not expose the complete derivation.
- [Griffin Pinney's Pressure Points page](https://griffinpinney.com/puzzles/jane_street/2026-11/) reports a leaderboard placing and says “Notes to come in December”; no worked derivation is present in the page reviewed on 2026-11-05.
- The candidate words have lengths 11 and 12, consistent with the displayed `(11 12)`. This agreement alone does not prove the candidate. The official puzzle page lists correct submissions as of 2026-11-02; accepted submissions confirm that answers were submitted, not how the puzzle is derived.
- The Puzzling question reports candidate readings (BUSQUETS, INIESTA, VIOLINS, MERCURY, INSTABILITY) and near-place-name residuals, but does not supply a complete derivation. The Archive search result displays November puzzle and “Puzzle Solution” text; the search result does not expose an anchor target. The directly opened Archive HTML is an older cached snapshot beginning at August 2026. A direct solution URL and official solution body have NOT been retrieved. Do not describe the search-result text as a verified solution link or inspected derivation.

See hypotheses H-001 to H-015 and open questions O-001, O-004, O-005, O-006, and O-007 in the project documents. Secondary-source status checks are recorded in the 0.7.0 formalization note and DATA_MANIFEST.md SRC-023–SRC-028. Twelve candidate theme/member groups are now recorded: compass, PlayStation-style face buttons, baseball bases, sailing-vessel positions, Southern Cross, clock face, India's cardinal-border states, NFPA 704 hazard diamond, Colorado/Four Corners region, joystick directions, Queen's four classic members, and four footballer candidates (Gareth, Iniesta, Busquets, Messi). The Colorado/desert interpretation remains provisional; the Queen and football clusters have no verified cardinal orientation. Their contextual identities and exact letter residuals are documented, but the emoji-to-group associations remain hypotheses. The community discussion also reports ungrouped candidate readings including VIOLINS and INSTABILITY. This is partial evidence for 12 of 19 themes, not a complete bijection; no global rule or output extraction has been demonstrated.

### Formalization gate

U-001 through U-010 remain unresolved. Ten candidate themes and further ungrouped readings do not close the complete mapping, clarify Ø, or establish a common letter-residual/extraction rule. Method selection remains BLOCKED. Do not select SAT/SMT, CSP, graph, search, string, optimization, or any other solver family merely because the cardinalities match. Do not write solver code, create a Cargo workspace, choose dependencies, or build CI until the rules and acceptance predicate justify those actions.

A no-go or an explicitly incomplete research case is preferable to fabricated semantics. Financial-engineering transfer is not presumed and may conclude that no useful transfer exists.

Official-source retrieval status: the Archive search result displays November “Pressure Points” and “Puzzle Solution” text but does not expose the anchor target; the directly opened HTML is a stale cache. The direct solution URL and body remain unverified. See DATA_MANIFEST.md §4 and GAP-004 in VALIDATION_MATRIX.md.

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

Before this candidate-cluster update, the following workflows succeeded on head `04400787062ecc0b84c6d75b5315d382a84979de`:

- [OFRL Repository Policy run #41975605588](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41973857312)
- [OFRL-IB-001 Numeric and Input Contract Checks run #41975605558](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41973857377)
- [OFRL-IB-001 Rust Runtime Semantic Validator run #41975605580](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41973857189)

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
