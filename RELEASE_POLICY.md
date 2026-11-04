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
7. Any required SBOM is generated from the exact reviewed source commit and lockfile, validated against its pinned official schema, and attached or linked together with checksums for the binary, SBOM, lockfile, provenance record, and final bundle.
8. The release notes do not overstate correctness, statutory compliance, performance, or independent replication.

A failed gate must be fixed or published as an explicit limitation only when the remaining release claim is still accurate and safe. A release cannot claim a validation level whose criteria have not passed.

## 5. Software release record

A software release should include, where applicable:

- Component name and semantic version.
- Source tag and full commit SHA.
- Supported platform and runtime/toolchain.
- Build and test commands.
- Lockfile and dependency graph.
- A complete, target-specific SBOM (SPDX or CycloneDX) generated from the exact release dependency graph. For Cargo binaries, the OFRL baseline is CycloneDX JSON 1.7 with the official schema revision recorded and schema validation passed.
- SHA-256 checksums for every distributable binary, SBOM, dependency lockfile, provenance record, and packaged release bundle.
- A build provenance record naming the source commit, target triple, toolchains, build command, SBOM generator revision, and schema revision.
- Licence and third-party notices.
- API/schema compatibility notes.
- Security advisories and known defects.
- Upgrade, migration, or rollback instructions.

Do not publish binaries that cannot be traced to a reviewed source commit and documented build environment. A CI-generated candidate artifact is validation evidence only; it must not be represented as an approved software release or as evidence that a larger financial model is complete.

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

## 10. Portfolio-wide release control

Each registered project's `project.json` must reflect its lifecycle and release state. A project is not considered released merely because a source file is public or a GitHub branch exists. A release must be linked to a project ID, reviewed source or publication revision, applicable licence/data profile, evidence maturity, and release record.

Project-specific versioning may differ. Do not impose one version number across unrelated research, data, software, and model deliverables. Where a project contains multiple independently released components, record their versions and compatibility separately.

## 11. Related documents

- [Research Standards](RESEARCH_STANDARDS.md)
- [Dependency Policy](DEPENDENCY_POLICY.md)
- [Licensing and Reuse Policy](LICENSE_POLICY.md)
- [Data Policy](DATA_POLICY.md)
- [Publication and IP Boundary](PUBLICATION_AND_IP_BOUNDARY.md)
- [Repository Audit](REPOSITORY_AUDIT.md)
