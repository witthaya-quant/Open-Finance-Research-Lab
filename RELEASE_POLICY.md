<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Release Policy

**Policy ID:** OFRL-REL-001  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Release authority:** Founder / Principal Maintainer

## 1. Purpose

A release is a published statement about the state of one project or deliverable at a specific point in its development. This policy applies to every current and future OFRL project, regardless of language, discipline, or artifact type. It must be possible to identify what was released, which source/data/dependencies produced it, what validation was performed, and what limitations remain.

OFRL does not release work merely because code compiles or a document is complete. Publication maturity must match the evidence.

## 2. Release types

| Type | Purpose | Minimum communication |
|---|---|---|
| Research note / report | Publish a research question, derivation, experiment, result, or negative finding | Scope, method, sources, assumptions, evidence maturity, limitations, revision date |
| Software pre-release | Make an implementation available for evaluation before a stable contract is promised | Version, supported scope, known gaps, test evidence, compatibility warning |
| Stable software release | Publish a supported component for reuse | Version, API/contract, compatibility, build provenance, dependencies, SBOM, checksums, tests, known issues |
| Correction / security release | Correct a material error or mitigate a vulnerability | Affected versions, issue, correction, impact, migration or mitigation guidance |

A research report and a software artifact may have different versions and release dates. Do not create a single repository-wide version merely to make unrelated projects appear synchronized.

## 3. Versioning

### 3.1 Software

Use Semantic Versioning for independently versioned software components when a public API or compatibility contract exists:

- **MAJOR:** incompatible public API or data-contract change.
- **MINOR:** backward-compatible functionality.
- **PATCH:** backward-compatible bug or security correction.

Before a stable public API is established, use an explicit pre-1.0 version and document that interfaces may change. A schema version is independent from the crate version and must be updated only when its own compatibility rules require it.

Do not bump a package version solely because documentation or an unrelated project changed.

### 3.2 Research and documentation

Research works use a dated revision history or publication version appropriate to the work. A material correction must state what changed and whether the conclusions or downstream outputs are affected.

Do not silently replace a published research conclusion. Preserve the prior revision or clearly identify the correction.

### 3.3 Git tags

- Release tags must point to the exact reviewed release commit.
- Do not move or reuse a published release tag.
- Prefer signed annotated tags when the release workflow and signing keys are established.
- Record the tag, source commit, and published artifacts in the release record.

## 4. Evidence and release gates

A release must identify its evidence maturity using [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md). Evidence maturity belongs to the released claim or component—not merely the repository.

Before a release is approved, confirm:

1. Scope and intended use are documented.
2. Source, licence, data rights, and disclosure checks pass.
3. Required tests and validation are complete for the claimed scope.
4. Known limitations, failed tests, unresolved findings, and unsupported conditions are visible.
5. Reproduction instructions and dependency/toolchain versions are recorded.
6. The release artifact corresponds to the reviewed source commit.
7. Any required SBOM and checksums are attached or linked.
8. The release notes do not overstate correctness, statutory compliance, performance, or independent replication.

A failed gate must be fixed or published as an explicit limitation only when the remaining release claim is still accurate and safe. A release cannot claim a validation level whose criteria have not passed.

## 5. Software release record

A software release should include, where applicable:

- Component name and semantic version.
- Source tag and full commit SHA.
- Supported platform and runtime/toolchain.
- Build and test commands.
- Lockfile and dependency graph.
- SBOM (SPDX or CycloneDX).
- SHA-256 checksums for distributable artifacts.
- Licence and third-party notices.
- API/schema compatibility notes.
- Security advisories and known defects.
- Upgrade, migration, or rollback instructions.

Do not publish binaries that cannot be traced to a reviewed source commit and documented build environment.

## 6. Research publication record

A research release should include:

- Research question and intended use.
- Method, equations, assumptions, variables, and units.
- Data provenance, rights, coverage, and transformation steps.
- Baseline and evaluation design where relevant.
- Versioned code/configuration and reproduction instructions.
- Results, uncertainty, sensitivity, failure modes, and negative findings.
- Evidence-maturity level and explicit limits.
- References and attribution.
- A correction route for material errors.

For financial models, separately report mathematical/accounting consistency, implementation correctness, statistical evidence, robustness, economic relevance, and independent replication. Passing one category does not imply passing another.

## 7. Release authorization and publication

During the Founder-maintained stage, the Founder / Principal Maintainer authorizes releases under the OFRL name. A release Pull Request or release checklist should identify the relevant project owner, validation evidence, licence, and decision records.

A Contributor or Maintainer may prepare an artifact within delegated scope but does not automatically receive authority to publish a release under the OFRL name.

No review or approval should be described as independent unless an independent person actually performed it.

## 8. Corrections, withdrawals, and deprecation

- Correct material errors openly and identify affected conclusions, versions, and downstream artifacts.
- Do not delete or rewrite history merely to conceal an error.
- If an artifact is unsafe, unlawfully disclosed, or materially misleading, restrict or withdraw access where possible and publish a correction notice.
- Deprecation notices must identify the affected version, reason, replacement if available, and support status.
- Security fixes may require a release outside the normal cadence.

## 9. Current OFRL-IB-001 release boundary

At the audit date, OFRL-IB-001 has an implemented input schema and Rust runtime semantic validator. It does **not** yet have the forecast calculation engine, Scenario Feasibility evaluation, or analyst-facing XLSX workbook.

The input validator may be documented and shared as an implemented component, but it must not be presented as a completed integrated three-statement forecast model. A full model release remains blocked until the relevant numerical, accounting, calculation, workbook, and research gates are complete.

### 9.1 Input-validator preview artifact

The repository maintains a preview-only workflow at [OFRL-IB-001 Release Candidate Preview](.github/workflows/ofrl-ib-001-release-candidate.yml). It builds the current `ofrl_cli` input-validation executable for the runner's explicit target, generates a binary-scoped CycloneDX SBOM from the application workspace lockfile, packages the lockfile and build metadata, and verifies a SHA-256 manifest.

The preview contract is:

- **Component scope:** `ofrl_cli` and its application-workspace dependency graph. The Decimal POC is excluded because it is a separate Cargo dependency boundary with its own lockfile.
- **Target scope:** the actual Rust host target reported by the runner. The current workflow uses `ubuntu-24.04`; this verifies that Linux runner target only and does not imply Windows, macOS, ARM, or other platform support.
- **SBOM format:** CycloneDX JSON 1.5 generated by `cargo-cyclonedx 0.5.9` from Cargo metadata and the exact application `Cargo.lock`. This generator supports versions 1.3–1.5; OFRL selects its highest supported version and does not claim CycloneDX 1.6 or 1.7 output.
- **Target-aware completeness:** the workflow compares the SBOM component set against `cargo metadata --filter-platform` for the actual build target and requires registry components to include publisher-declared licence metadata and package hashes. The workspace lockfile may contain additional target-specific packages outside the current binary's graph; they are not silently described as Linux runtime components.
- **Reproducibility:** the generator receives `SOURCE_DATE_EPOCH` derived from the source commit timestamp. The workflow hashes the application lockfile before and after SBOM generation and fails if it changes.

The preview generator is subject to a temporary, documented exception because its published lockfile pins yanked `xml-rs 0.8.19`; see [Dependency Policy §8.1](DEPENDENCY_POLICY.md).
- **Integrity:** the preview bundle contains the executable, SBOM, exact lockfile, build metadata, and `SHA256SUMS`; the workflow verifies the per-file checksums before upload. GitHub Actions also reports a separate SHA-256 digest for the uploaded archive.

A preview artifact is **not a software release**. The workflow does not create a Git tag, publish a GitHub Release, change `project.json`, or change the project's `not_released` status. It does not establish the forecast engine, workbook, or full model as complete. Release authority must still review the artifact and its Release Record.

A future distributable release must build from the exact reviewed release tag/commit, generate a fresh SBOM from that release's dependency graph and target configuration, publish the binary/SBOM/checksum files together, and complete the applicable risk-tier, source-level licence/notice, security, data-rights, and publication-authorization gates. The preview workflow must not be treated as satisfying those approvals.

## 10. Portfolio-wide release control

Each registered project's `project.json` must reflect its lifecycle and release state. A project is not considered released merely because a source file is public or a GitHub branch exists. A release must be linked to a project ID, reviewed source or publication revision, applicable licence/data profile, evidence maturity, and release record.

Project-specific versioning may differ. Do not impose one version number across unrelated research, data, software, and model deliverables. Where a project contains multiple independently released components, record their versions and compatibility separately.

### 10.1 Machine-readable release readiness and authorization

Manifest schema 1.1.0 is the authoritative machine-readable release-readiness contract for each registered project. It keeps four separate dimensions distinct: lifecycle state, risk tier, evidence maturity, and release authorization.

Assessment states:
- `NOT_ASSESSED`: no formal release assessment has been completed. This is the normal state for proposed and in-development work; no gate list is fabricated.
- `BLOCKED`: formal assessment has begun and at least one required gate is missing, failed, not run, or blocked.
- `ELIGIBLE_FOR_AUTHORIZATION`: every required gate for the project type, data classification, lifecycle and risk tier is PASS or has a permitted, reasoned NOT APPLICABLE disposition.

Gate dispositions are `PASS`, `FAIL`, `NOT RUN`, `BLOCKED`, and `NOT APPLICABLE`. A PASS must identify the assessor, role, independence/conflict declaration, evidence and review reference. A NOT APPLICABLE decision must state why the gate does not apply. R3 cannot waive founder approval, independent domain/security review, operational safeguards, incident response or rollback/recovery. Privacy/legal review may be N/A only when the rationale is recorded.

Authorization is separate:
- `NOT_AUTHORIZED`: no publication/live-use permission exists.
- `AUTHORIZED`: an explicit authority, role, date, decision record and exact 40-character source commit are recorded.
- `WITHDRAWN`: a previously authorized release has been withdrawn and the manifest lifecycle/release state must reflect withdrawal.

The `residual_risk_assessment` is part of the same project manifest, not a separate risk tracker. It records who reviewed residual risk, when it was reviewed, the next review date, evidence references, named control owners, dispositions, review due dates, expiry dates and expiry conditions. An `ACCEPTED` residual risk requires the accepting authority, role, date and decision reference. A release cannot be eligible while any residual risk remains `OPEN`; accepted or transferred risks remain time-bounded and reviewable. Store only sanitized risk summaries in the repository; confidential evidence must remain in an access-controlled system and the manifest may reference it without embedding its content. Any non-`none` data classification requires both `data_rights` and `privacy_legal_review`; privacy/legal may be NOT APPLICABLE only with a substantive scope-based rationale. R3 requires `privacy_legal_review` regardless of data classification. The registry requires a full applicable gate inventory once assessment is BLOCKED or ELIGIBLE. A project in `NOT_ASSESSED` may remain in proposal/development with an empty gate list. A `pre_release` or `released` artifact requires eligible assessment and explicit authorization. `LIVE_EXTERNAL` requires a released, authorized R3 project; lower risk tiers cannot declare this mode. A project does not become released because CI is green or a preview artifact exists.

The registry is a structural and state-transition control. It verifies required gate coverage, evidence references, named assessors, independence flags for gates that require an independent reviewer, reasoned N/A states, lifecycle consistency and the separate authorization record. It cannot independently authenticate a person's competence, verify a reviewer's GitHub identity against an external evidence URL, determine legal applicability, or prove that cited evidence is true. The Founder and actual qualified reviewers remain accountable for those decisions. Never treat AI-assisted review as independent human review.

The project manifest template and the existing release record template carry the same gate vocabulary. Project-specific validation matrices continue to own domain acceptance criteria; this policy does not create a second task tracker or replace those matrices.

## 11. Related documents

- [Research Standards](RESEARCH_STANDARDS.md)
- [Dependency Policy](DEPENDENCY_POLICY.md)
- [Licensing and Reuse Policy](LICENSE_POLICY.md)
- [Data Policy](DATA_POLICY.md)
- [Publication and IP Boundary](PUBLICATION_AND_IP_BOUNDARY.md)
- [Repository Audit](REPOSITORY_AUDIT.md)
