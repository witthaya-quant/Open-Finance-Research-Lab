<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Validation Matrix

**Project:** Computational Methods & Problem-Solving Case Studies  
**Project version / baseline:** 0.1.0  
**Risk tier:** R1  
**Owner:** Kenny (Witthaya Maidee)  
**Last updated:** 2026-11-05

## 1. Validation rules

Use only these states:
- **PASS:** acceptance criteria were executed and met; evidence is linked.
- **FAIL:** a check was executed and did not meet its criteria.
- **NOT RUN:** the check has not been executed.
- **BLOCKED:** a prerequisite or decision is missing.
- **N/A:** the check does not apply to the stated scope; explain why.

A green metadata workflow does not prove mathematical correctness. Evidence maturity, implementation status, and validation status are tracked separately.

## 2. Acceptance matrix

| ID | Requirement / invariant | Gate | Validation method | Acceptance criteria | Status | Evidence / artifact | Owner |
|---|---|---|---|---|---|---|---|
| VAL-001 | Project identity and registration | Admission | Manifest schema and project-registry CI | Manifest valid; folder and project ID match; catalog row is consistent | PASS | [Repository Policy run 41963988332](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41963988332) | Project Owner |
| VAL-002 | Source provenance | Admission | Inspect official source references and access metadata | Original publisher, title, URL, purpose context, and access date are recorded | PASS | DATA_MANIFEST.md; official Jane Street source URLs | Project Owner |
| VAL-003 | Third-party rights boundary | Admission / publication | Review stored artifacts against OFRL data and IP policies | No unlicensed puzzle payload, image, or official solution is committed; unresolved rights are not represented as permission | PASS | DATA_MANIFEST.md; current project file scope | Project Owner |
| VAL-004 | Independence from OFRL-IB-001 | Admission | Inspect PR changed-file list, base/head refs, and repository branch state | No IB-001 files, branch, PR, or issue modified; only the central Project Catalog is shared | PASS | [PR #29 changed files](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29/files); base main remained at the recorded starting commit | Project Owner |
| VAL-005 | Problem rules sufficiently formalized | Research | Trace each rule to an authoritative source and define input/output semantics | All correctness-critical rules and output conditions are explicit; unresolved items are closed or declared blockers | BLOCKED | cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md | Project Owner |
| VAL-006 | Solver correctness and completeness | Method development | Independent verifier, proof obligations, boundary and adversarial tests | Claims of soundness, completeness, optimality, or uniqueness are supported by appropriate evidence | N/A | No solver implementation in the approved initial scope | Project Owner |
| VAL-007 | Comparative method evaluation | Method development | Common fixtures, environment, stopping criteria, and metrics | Comparisons are reproducible and conditions are materially comparable | N/A | No solver comparison in the approved initial scope | Project Owner |
| VAL-008 | Financial-engineering transfer | Research interpretation | Explicit mapping and assumption analysis | Any claimed transfer identifies preserved properties, changed assumptions, and further evidence; no transfer is an acceptable conclusion | BLOCKED | Research Specification §10; pilot rule semantics unresolved | Project Owner |
| VAL-009 | Reproducibility of published findings | Publication | Clean-environment reproduction from versioned inputs and instructions | A competent reader can reproduce every material reported result | NOT RUN | No computational result is claimed | Project Owner |
| VAL-010 | Research claims and evidence maturity | Publication | Review claims against evidence levels 0–4 | No claim exceeds the evidence actually obtained | PASS | Manifest maturity 0; no solver or solution claim | Project Owner |
| VAL-011 | Project implementation architecture and isolation | Architecture | Review the project-level implementation decision and IMPLEMENTATION_APPROACH.md | Engineering controls are explicit; executable work is justified by the research question; no IB-001 components are reused | PASS | D-009 in DECISION_LOG.md; IMPLEMENTATION_APPROACH.md | Project Owner |
| VAL-012 | Executable Rust implementation quality and supply-chain controls | Implementation | Project-local Cargo CI and dependency review, when code is admitted | Declared toolchain/MSRV, locked dependencies, formatting, Clippy, tests, documentation, verifier boundary, and supply-chain evidence pass | NOT RUN | No executable code or Cargo workspace exists in this revision | Project Owner |
| VAL-013 | Project organization and shared-policy inheritance | Governance / admission | Inspect project tree, manifest requirements, README policy links, and case protocol | Project-local work is isolated; root policies remain authoritative by reference; no duplicate policies or speculative folders exist | PASS | project.json; README.md; CASE_STUDY_PROTOCOL.md §§9.1–9.3; HANDOFF.md | Project Owner |
| VAL-014 | Complete semantic grouping and cardinal mapping | Research formalization | Reconstruct every emoji group and account for all 76 entries using a consistent rule | All 19 groups and cardinal assignments are derived without unmatched entries or ad hoc exceptions | BLOCKED | PROBLEM_FORMALIZATION.md U-001–U-010; H-001–H-005 | Project Owner |

## 3. Initial-pilot structural checks

The official Pressure Points page was accessed on 2026-11-05. The visible source contains:
- One leading marker.
- 75 distinct uppercase-letter sequences.
- 19 emoji clues.
- A terminal notation shown as “(11 12)”.

A structural check of the visible sequence block found that every sequence is internally alphabetically ordered, the list is lexicographically ordered, and the 75 sequences contain 504 letters in total. The length distribution is recorded in PROBLEM_FORMALIZATION.md.

These checks establish representation-level properties only; they do not resolve puzzle semantics.

## 4. Test inventory

| Test ID | Test name | Type | Input / fixture | Expected result | Actual result | Status | CI job / command |
|---|---|---|---|---|---|---|---|
| TEST-001 | Project manifest and catalog validation | Repository policy | OFRL-CM-001 project tree | Registry accepts the new independent project | Manifest and catalog accepted | PASS | [Repository Policy run 41963988332](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41963988332) |
| TEST-002 | Project documentation link validation | Repository policy | New Markdown files | All local links resolve | Local link and document validation passed | PASS | [Repository Policy run 41963988332](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41963988332) |
| TEST-003 | Case source attribution review | Manual | DATA_MANIFEST.md | Primary source and purpose correctly attributed | Official URLs and access date recorded | PASS | Manual source review |
| TEST-004 | Puzzle rule completeness | Research review | Original source and formalization | Every correctness-critical rule traced to source | Source semantics remain incomplete | BLOCKED | Manual research gate |
| TEST-005 | Visible sequence structural consistency | Structural inspection | Official source representation | Count, uniqueness, internal ordering, overall ordering, and length totals are internally consistent | 75 distinct sequences; 504 letters; observed ordering properties checked | PASS | Structural analysis recorded in PROBLEM_FORMALIZATION.md |
| TEST-006 | Executable implementation quality gates | Engineering validation | Project-local build workspace and CI, when admitted | Formatting, static analysis, locked tests, documentation, and project-specific validation pass on the declared toolchain | No executable implementation has been admitted | NOT RUN | Future OFRL-CM-001 workflow |
| TEST-007 | Source cardinality cross-check | Structural analysis | Official source representation | Visible count is internally consistent: one leading marker plus 75 letter sequences, alongside 19 emoji clues; any proposed 19-by-4 grouping is recorded as a hypothesis rather than a proven rule | 76 visible marker/sequence entries; 19 emoji clues; 19 × 4 equals 76 arithmetically | PASS | PROBLEM_FORMALIZATION.md §4.1; arithmetic cross-check | Project Owner |
| TEST-008 | Compass-group aggregate letter-multiset equality | Manual mathematical cross-check | Candidate decoded terms NORTH / EAST / SOUTH / WEST | The combined letters in the four candidate source entries equal the combined letter multiset of the four direction names | Equality verified over 18 letters; individual word assignments remain hypotheses | PASS | PROBLEM_FORMALIZATION.md H-004; independent character-count audit | Project Owner |
| TEST-009 | Complete four-member group / cardinal assignment | Research analysis | All 19 emojis and 76 visible entries | Every entry is assigned exactly once; one consistent rule explains group membership and orientation | Four candidate clusters identified, but complete coverage and orientation are not established; three clusters have unresolved letter residuals | BLOCKED | PROBLEM_FORMALIZATION.md H-004/H-005, U-009/U-010 | Project Owner |
| TEST-010 | Candidate entity and regional context corroboration | Source cross-check | Official organization and public regional reference | The named candidate entity exists and the contextual Flemish Diamond geography is accurately described; this is not a test of puzzle correctness | Antwerpsche Diamantkring confirms itself as an Antwerp rough-diamond bourse; government report describes four-city Flemish Diamond geometry | PASS | DATA_MANIFEST.md SRC-007/SRC-008; cited source pages | Project Owner |

## 5. Known gaps and exceptions

| ID | Gap / exception | Impact | Compensating control | Owner | Review trigger / expiry |
|---|---|---|---|---|---|
| GAP-001 | The retrieved source representation does not establish every rule or output condition | Prevents a defensible solver specification | No solver work; seek authoritative clarification or record a no-go | Project Owner | Before method selection |
| GAP-002 | Redistribution rights for puzzle text and artwork have not been established | Cannot include the full puzzle payload | Source URLs and limited observations only | Project Owner | Before any source content is copied |
| GAP-003 | No financial transfer has been demonstrated | Prevents claims of direct finance applicability | Perform a bounded mapping analysis after formalization | Project Owner | Before publication |
| GAP-004 | Official November 2026 solution content could not be directly retrieved despite an Archive search-index label | Prevents using an official solution as a verified source of rules | Record the discrepancy; retry the Archive and inspect a direct solution page before relying on it | Project Owner | Before any official-solution claim |

## 6. Sign-off

**Prepared by:** Kenny (Witthaya Maidee)  
**Project approval:** Founder-authorized scope, 2026-11-05  
**Independent review:** Not yet performed  
**Unresolved blockers:** Pressure Points rule semantics and authoritative output definition  
**Publication status:** Not released
