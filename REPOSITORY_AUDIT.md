<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Repository Audit

**Audit ID:** OFRL-AUDIT-2026-001  
**Audit date:** 2026-11-04  
**Scope:** Repository-wide governance and operating controls for every current and future project, including project registration, research operations, software engineering, dependencies, CI/CD, data, publication, release, and project maturity  
**Audit owner:** Founder / Principal Maintainer  
**Status:** Baseline assessment; remediation tracked through Pull Requests and Issues

## 1. Executive assessment

OFRL already has a substantive research and engineering foundation. Its strongest areas are research integrity, financial-model specification, data and licensing boundaries, typed input validation, deterministic synthetic fixtures, and two-toolchain Rust CI.

The main weakness is not an absence of principles. It is incomplete operational enforcement and lifecycle control around those principles. The repository needs stronger default-branch protection, a repository-wide validation gate, dependency and release controls, an explicit account-continuity procedure, and synchronization between decision records and implemented code.

This audit separates three states:

- **Observed on `main`:** controls and files present in the default branch at the audit snapshot.
- **Proposed in PR #14:** governance, contribution, and publication controls that remain inactive on `main` until merged.
- **Manual GitHub configuration:** settings that cannot be activated merely by committing documentation.

## 2. Scope and evidence boundary

The audit inspected the default branch, repository metadata, current project inventory, existing workflows, OFRL-IB-001 specifications/source/tests, and the proposed governance and portfolio-control branch for PR #14. At the snapshot, OFRL-IB-001 was the only project directory under `projects/`; the audit therefore assessed the universal controls required for future projects separately from the detailed maturity of the current project.

At the audit snapshot:

- Repository: public personal-account repository under `witthaya-quant`.
- Default branch: `main`.
- The default branch was reported as unprotected, with required status-check enforcement off.
- PR #14 was open and unmerged. Its proposed Governance and Publication policies were therefore not yet part of the default branch.
- Two GitHub Actions workflows existed for the OFRL-IB-001 Decimal/input contract and Rust semantic validator.
- The PR workflows completed the current five-job validation matrix successfully on Rust 1.85.0 and 1.99.1, including JSON Schema fixture validation.

This is a point-in-time repository audit, not a penetration test, legal opinion, statutory accounting opinion, or independent validation of the future forecast engine. Security settings that were not accessible through repository metadata are marked **Not verified** rather than assumed enabled or disabled.

## 3. Existing strengths

| Area | Observed evidence | Assessment |
|---|---|---|
| Research identity | README, Research Charter, Research Standards | Clear research purpose and evidence philosophy |
| Research integrity | Evidence-maturity levels, negative-result policy, correction/versioning expectations | Strong baseline |
| Data governance | Data classes, manifest requirements, release gate, redistribution restrictions | Strong baseline |
| Licensing | Apache-2.0 software, CC BY 4.0 documentation, source-specific data rights | Clear content-based allocation |
| Financial specification | Three-statement assumptions, scope, sign conventions, schedules, explicit exclusions | Detailed and inspectable |
| Input contract | Draft 2020-12 JSON Schema and deterministic synthetic fixture | Implemented structural contract |
| Runtime validation | Typed Rust structures, strict Decimal-string boundary, date/period checks, historical accounting checks, diagnostics | Implemented input-acceptance gate |
| Numerical discipline | Fixed-precision Decimal candidate, checked arithmetic policy, numeric envelope and error-contract documents | Correctly distinguishes finite Decimal from arbitrary precision; model-level envelope remains open |
| Rust CI | Formatting, Clippy, tests, docs, CLI smoke checks on two toolchains | Positive evidence for current validator scope |
| Workflow permissions | `contents: read`, checkout with `persist-credentials: false`, full-length SHA for checkout | Good workflow baseline |

A green validator workflow establishes only the behavior exercised by its tests. It does not establish forecast-model correctness, accounting-framework compliance, economic usefulness, or independent replication.

## 4. Findings and remediation priority

Priority definitions:

- **P0 — Immediate control:** an exposure or control gap affecting the integrity of the default branch or public disclosure.
- **P1 — Before broad contributions or a software release:** an operational control needed for reliable collaboration, security, or reproducibility.
- **P2 — Before scale or when the relevant capability is introduced:** a control that should not be fabricated before it has a real owner or use case.

| ID | Priority | Finding | Evidence / impact | Required action |
|---|---|---|---|---|
| AUD-001 | P0 | Default branch protection is not enabled | `main` metadata reported `protected: false`; required status enforcement was off | Configure Pull Request requirement, conversation resolution, force-push/deletion restrictions, and verify owner bypass behavior. Tracked in Issue #15 |
| AUD-002 | P1 | Governance package is not active on `main` until PR #14 is merged | PR #14 remains a proposal, not a default-branch policy | Review and merge the complete policy package; do not describe it as active before merge |
| AUD-003 | P1 | No repository-wide, always-running policy check | Existing Rust workflows use path filters; documentation-only changes can bypass those jobs | **Implemented on this PR branch:** always-running repository workflow validates Markdown links, JSON/YAML syntax, project manifests, type/risk requirements, and catalog consistency |
| AUD-004 | P1 | Dependency automation is absent | No Dependabot configuration was present | Configure Cargo and GitHub Actions update proposals; keep human review and no automatic merge |
| AUD-005 | P1 | Lockfile coverage is incomplete | The Cargo workspace has a committed `Cargo.lock`; the isolated Decimal POC has no committed lockfile and its workflow does not use `--locked` | Generate and commit the POC lockfile, use `--locked`, and retain a separate lockfile policy for the isolated package |
| AUD-006 | P1 | Schema-validator transitive npm dependencies are not frozen | At the audit baseline, the schema script installed exact direct versions into a temporary directory with lockfile generation disabled | **Remediated on this PR branch:** commit an isolated npm manifest/lockfile and use `npm ci` with lifecycle scripts disabled |
| AUD-007 | P1 | No documented release lifecycle or changelog | No root release policy or changelog was present | Define release authorization, versioning, evidence labels, provenance, checksums, SBOM, correction, and deprecation requirements |
| AUD-008 | P1 | No release-time SBOM process | The POC dependency review explicitly says its transitive inventory is not a complete SBOM | Produce and review an SPDX or CycloneDX SBOM for distributable software releases |
| AUD-009 | P1 | Founder-account continuity is not fully operationalized | The personal account is the repository owner and the Founder is the sole current Maintainer | Document phishing-resistant MFA, recovery, backups, and GitHub successor settings without sharing credentials |
| AUD-010 | P1 | CI toolchain bootstrap has an unpinned fallback installer | Workflows may download the current rustup shell installer if `rustup` is absent | Prefer the preinstalled runner toolchain manager and fail visibly if unavailable; do not silently fetch an unpinned installer |
| AUD-011 | P1 | Project Decision Log has stale implementation statements | It says the semantic validator/schema gates remain unimplemented even though the project README, runtime-validator document, source, tests, and CI show they are implemented | Update statuses and distinguish completed input-validation work from the still-unimplemented forecast engine |
| AUD-012 | P1 | Branch-protection status is documented but not verified after configuration | A policy file cannot activate a GitHub setting | Keep a configuration record and test a PR against the protected default branch |
| AUD-013 | P2 | Advanced Security settings are not verified | Connector access did not establish Dependabot alert, secret scanning, push protection, or code-scanning configuration state | Review the repository's Security and analysis settings; record each control as enabled, unavailable, or not applicable |
| AUD-014 | P2 | No CODEOWNERS file | There is currently one active Maintainer; a nominal code owner would not create independent review | Add CODEOWNERS only when real domain owners are appointed |
| AUD-015 | P2 | No separate support or service-level commitment | OFRL is a Founder-maintained research project, not an operated support service | Do not promise response times or support obligations until resources and scope are defined |
| AUD-016 | P1 | No portfolio-wide project identity and admission contract on `main` | The only existing project has rich project documents, but no common manifest schema, lifecycle state contract, or automated registration rule existed at the audit baseline | Add a universal project lifecycle, manifest schema, catalog, type/risk profiles, and automated consistency check for every folder under `projects/` |
| AUD-017 | P1 | Project standards were not generalized by project type | Existing documentation is strongest for financial modelling and research; future data, software, benchmark, infrastructure, and prototype projects could be admitted without a common type-specific gate | Define technology-neutral universal requirements and project-type validation profiles; do not copy OFRL-IB-001 requirements mechanically |
| AUD-018 | P1 | Project metadata and catalog could drift | No machine-readable project manifest or CI catalog reconciliation existed at the audit baseline | Require `project.json` per project and validate IDs, directory names, required documents, risk tier, release state, and catalog entry in CI |

## 5. OFRL-IB-001 maturity assessment

The current project status must be described precisely:

| Capability | Observed status | What it establishes |
|---|---|---|
| Versioned JSON input schema | Implemented; CI-validated | Structural and lexical input-contract validity |
| Synthetic reference fixture | Implemented | Deterministic input case for validation |
| Rust typed input parser | Implemented | Closed typed deserialization and Decimal-string boundary |
| Historical accounting validation | Implemented for the documented V1 rules | Opening/historical statement and roll-forward consistency checks under the stated simplifications |
| Typed diagnostics and CLI validation | Implemented | Stable input-acceptance/rejection report and exit status |
| Decimal POC | Implemented and CI-tested | Tested Decimal string interchange and selected checked-arithmetic behavior |
| Model-level numeric envelope and accumulated-error budget | Open | Final financial calculation precision has not been accepted |
| Full dependency/SBOM review | Open | Tested direct versions do not establish complete transitive review |
| Forecast calculation engine | Not started | No forecast statements are generated |
| Scenario Feasibility evaluation | Not implemented | Forecast funding feasibility is not evaluated |
| Analyst-facing XLSX workbook | Not implemented | No spreadsheet output or reconciliation exists |
| Independent financial-model verification | Open | The future engine still requires an independent calculation benchmark and cross-statement review |

The project's overall Evidence Maturity Level 0 is a project-level status, not a denial that the input validator has been implemented. The validator has its own implementation evidence; the financial model itself remains proposed/exploratory until the forecast engine and release gates are completed.

## 6. Portfolio-wide project-control assessment

At the audit baseline, `projects/OFRL-IB-001` was the only registered project directory. Its specification is unusually detailed, but the repository had not yet formalized a contract that a second project—such as a quantitative research study, software library, dataset, benchmark, prototype, or infrastructure tool—must follow.

The new portfolio-control design proposed in this PR adds:

- A universal project lifecycle from proposal through archival.
- A project manifest schema and a manifest for OFRL-IB-001.
- A human-readable catalog linked to machine-readable manifests.
- Technology-neutral project types and risk tiers R0–R3.
- Universal requirements plus additive profiles for research, quantitative research, financial models, software, data, benchmarks, prototypes, and infrastructure.
- An automated registry validator that checks every immediate project directory and requires project ID, manifest, baseline documents, type-specific documents, and catalog registration.
- A repository workflow that runs these checks on every Pull Request, not only when OFRL-IB-001 files change.

These controls are proposed on the PR branch and do not apply to `main` until the PR is merged and its CI passes. The automated registry check validates project metadata and document structure; it does not claim to run every project's domain-specific tests. Each project remains responsible for its own test/build/security workflows according to its manifest and risk tier.

## 7. Recommended remediation sequence

### Gate A — Protect the source of truth

1. Merge the reviewed Foundation and Governance package.
2. Configure and verify branch protection for `main`.
3. Enable an always-running repository-policy check.
4. Confirm the review and merge procedure is accurately described for a single Maintainer.

### Gate B — Make the build reproducible

1. Enable Dependabot alerts and configure update proposals.
2. Commit the isolated POC Cargo lockfile and use locked builds.
3. Freeze the Node schema-validator dependency graph.
4. Remove the unpinned rustup bootstrap fallback.
5. Add release-time SBOM and dependency review.

### Gate C — Control publication and continuity

1. Adopt a release policy and changelog.
2. Apply the Public/Private publication gate to every new artifact.
3. Configure the account recovery and successor process.
4. Maintain backups of Git history and release artifacts.
5. Review Security and analysis settings.

### Gate D — Continue financial-model research

These are OFRL-IB-001-specific gates, not requirements imposed on unrelated future projects:

1. Close the model-specific Numeric Envelope and error budget.
2. Build an independent financial model benchmark for all forecast years.
3. Implement typed forecast schedules and the integrated three statements.
4. Implement Structural Integrity and Scenario Feasibility as separate outcomes.
5. Verify canonical Decimal outputs against analyst-facing exports.
6. Obtain independent review or replication before claiming the highest evidence-maturity levels.

## 8. Remediation state on the proposed branch

| Finding | Proposed remediation in PR #14 | Remaining action |
|---|---|---|
| AUD-001 / AUD-012 | Documented branch-control baseline and opened Issue #15 | Account owner must configure and verify GitHub branch protection manually |
| AUD-002 | Governance and policy package consolidated in this PR | Merge only after full review and CI pass |
| AUD-003 | Always-running repository policy workflow added | Confirm workflow passes; later select its stable job as a required status check after branch protection is configured |
| AUD-004 | Dependabot configuration added for current Cargo and Actions ecosystems | Add new ecosystems and project package paths as future projects introduce them |
| AUD-005 | Dependency policy defines separate workspace/POC lockfiles | Commit the isolated Decimal POC lockfile and use locked CI commands |
| AUD-006 | Dependency policy identifies npm transitive drift | Commit a package manifest/lockfile and move validator to `npm ci` |
| AUD-007 | Release Policy and Changelog added | Implement release workflows only when a distributable project artifact is ready |
| AUD-008 | Release Policy requires SBOM and checksums; Dependency Policy defines the process | Add and test SBOM generation before the first distributable software release |
| AUD-009 | Account Continuity policy added | Founder must configure MFA/recovery/successor and verify backups personally |
| AUD-010 | Dependency Policy records the unpinned rustup fallback | Remove or replace the fallback in both Rust workflows, then test both toolchains |
| AUD-011 | OFRL-IB-001 Decision Log status corrected against implementation | Keep the log synchronized as forecast-engine work proceeds |
| AUD-016 / AUD-017 / AUD-018 | Project Lifecycle, Project Standards, Project Catalog, Manifest Schema, current manifest, and automated registry validation added | Confirm CI passes and require the same manifest/profile contract for every future project |

The proposed branch does not itself activate repository settings, establish independent review, create an SBOM, or close every dependency finding. A policy is closed only when its corresponding evidence is present.

## 9. Audit interpretation

The audit does not recommend adding a GitHub Organization, changing the public repository to a restrictive source-available licence, adopting a CLA/CAA without a licensing need, or creating Maintainer/Code Owner roles before people are appointed.

The recommended operating model remains a Founder-led public repository under the Founder's personal account, with fork-based contributions and a separately controlled private research workstream.

Every finding should be closed by evidence: a merged change, a verified GitHub setting, a passing check, a completed review, or an explicitly accepted residual risk. Updating the wording of a document alone does not close an operational control finding.
