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
| VAL-001 | Project identity and registration | Admission | Manifest schema and project-registry CI | Manifest valid; folder and project ID match; catalog row is consistent | NOT RUN | project.json; Project Catalog; CI pending | Project Owner |
| VAL-002 | Source provenance | Admission | Inspect official source references and access metadata | Original publisher, title, URL, purpose context, and access date are recorded | PASS | DATA_MANIFEST.md; official Jane Street source URLs | Project Owner |
| VAL-003 | Third-party rights boundary | Admission / publication | Review stored artifacts against OFRL data and IP policies | No unlicensed puzzle payload, image, or official solution is committed; unresolved rights are not represented as permission | PASS | DATA_MANIFEST.md; current project file scope | Project Owner |
| VAL-004 | Independence from OFRL-IB-001 | Admission | Compare changed paths and base/head changes | No IB-001 files, issue, branch, or PR modified; only the portfolio catalog is shared | NOT RUN | Admission PR diff and branch review pending | Project Owner |
| VAL-005 | Problem rules sufficiently formalized | Research | Trace each rule to an authoritative source and define input/output semantics | All correctness-critical rules and output conditions are explicit; unresolved items are closed or declared blockers | BLOCKED | cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md | Project Owner |
| VAL-006 | Solver correctness and completeness | Method development | Independent verifier, proof obligations, boundary and adversarial tests | Claims of soundness, completeness, optimality, or uniqueness are supported by appropriate evidence | N/A | No solver implementation in the approved initial scope | Project Owner |
| VAL-007 | Comparative method evaluation | Method development | Common fixtures, environment, stopping criteria, and metrics | Comparisons are reproducible and conditions are materially comparable | N/A | No solver comparison in the approved initial scope | Project Owner |
| VAL-008 | Financial-engineering transfer | Research interpretation | Explicit mapping and assumption analysis | Any claimed transfer identifies preserved properties, changed assumptions, and further evidence; no transfer is an acceptable conclusion | BLOCKED | Research Specification §10; pilot rule semantics unresolved | Project Owner |
| VAL-009 | Reproducibility of published findings | Publication | Clean-environment reproduction from versioned inputs and instructions | A competent reader can reproduce every material reported result | NOT RUN | No computational result is claimed | Project Owner |
| VAL-010 | Research claims and evidence maturity | Publication | Review claims against evidence levels 0–4 | No claim exceeds the evidence actually obtained | PASS | Manifest maturity 0; no solver or solution claim | Project Owner |
| VAL-011 | Rust implementation architecture and supply-chain controls | Implementation admission | Review against IMPLEMENTATION_APPROACH.md before executable code is added | Rust is used where implementation is justified; toolchain, MSRV, dependency, test, verifier, and reproducibility decisions are documented; no IB-001 components are reused | NOT RUN | No executable code or Cargo workspace exists in this revision | Project Owner |

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
| TEST-001 | Project manifest and catalog validation | Repository policy | OFRL-CM-001 project tree | Registry accepts the new independent project | Awaiting PR workflow | NOT RUN | repository-policy |
| TEST-002 | Project documentation link validation | Repository policy | New Markdown files | All local links resolve | Awaiting PR workflow | NOT RUN | repository-policy |
| TEST-003 | Case source attribution review | Manual | DATA_MANIFEST.md | Primary source and purpose correctly attributed | Official URLs and access date recorded | PASS | Manual source review |
| TEST-004 | Puzzle rule completeness | Research review | Original source and formalization | Every correctness-critical rule traced to source | Source semantics remain incomplete | BLOCKED | Manual research gate |
| TEST-005 | Visible sequence structural consistency | Structural inspection | Official source representation | Count, uniqueness, internal ordering, overall ordering, and length totals are internally consistent | 75 distinct sequences; 504 letters; observed ordering properties checked | PASS | Structural analysis recorded in PROBLEM_FORMALIZATION.md |
| TEST-006 | Rust implementation quality gates | Engineering validation | Project-local Cargo workspace and CI, when admitted | Formatting, Clippy, locked tests, documentation, and project-specific validation pass on the declared toolchain | No Rust implementation has been admitted | NOT RUN | Future OFRL-CM-001 workflow |

## 5. Known gaps and exceptions

| ID | Gap / exception | Impact | Compensating control | Owner | Review trigger / expiry |
|---|---|---|---|---|---|
| GAP-001 | The retrieved source representation does not establish every rule or output condition | Prevents a defensible solver specification | No solver work; seek authoritative clarification or record a no-go | Project Owner | Before method selection |
| GAP-002 | Redistribution rights for puzzle text and artwork have not been established | Cannot include the full puzzle payload | Source URLs and limited observations only | Project Owner | Before any source content is copied |
| GAP-003 | No financial transfer has been demonstrated | Prevents claims of direct finance applicability | Perform a bounded mapping analysis after formalization | Project Owner | Before publication |

## 6. Sign-off

**Prepared by:** Kenny (Witthaya Maidee)  
**Project approval:** Founder-authorized scope, 2026-11-05  
**Independent review:** Not yet performed  
**Unresolved blockers:** Pressure Points rule semantics and authoritative output definition  
**Publication status:** Not released
