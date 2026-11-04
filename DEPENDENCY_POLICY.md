<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Dependency and Software Supply-Chain Policy

**Policy ID:** OFRL-DEP-001  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Policy owner:** Founder / Principal Maintainer

## 1. Purpose

OFRL depends on external software, build tools, GitHub Actions, and data-processing utilities. A dependency is part of the software supply chain and must be treated as an explicit engineering choice.

This policy defines how dependencies are introduced, pinned, reviewed, updated, and recorded. It applies to production code, research prototypes, CI workflows, validation scripts, and release tooling.

## 2. Dependency classes

| Class | Examples | Required control |
|---|---|---|
| Application/runtime | Rust crates and libraries used by OFRL binaries or libraries | Manifest, lockfile, licence review, vulnerability review, tests |
| Research / prototype | Isolated POCs, notebooks, benchmark packages | Clearly labelled scope, isolated dependencies, reproducible lockfile where package tooling supports it |
| Build / validation | JSON Schema validators, generators, compilers, formatters | Pinned direct and transitive dependency resolution; documented execution |
| CI workflow actions | Checkout, setup, security, or publishing Actions | Full-length commit SHA pinning; minimal workflow permissions |
| External tools | Rust toolchains, Node, Ruby, spreadsheet calculation engines | Explicit version, supported platform, and recorded release environment |

## 3. Admission requirements

Before adding a material dependency, record:

- The problem it solves and why a dependency is preferable to a small local implementation.
- Package name, source repository or registry, version, and enabled features.
- Direct and transitive dependency impact where available.
- Licence identifiers and any notice, attribution, patent, or redistribution obligations.
- Maintenance activity, release history, known security advisories, and ownership/provenance.
- Minimum supported Rust or runtime version and platform constraints.
- Performance, precision, determinism, and operational implications where relevant.
- The tests that establish the dependency is suitable for the intended use.

A dependency must not be admitted solely because it is popular or convenient. If the licence or provenance is unclear, do not introduce it until reviewed.

## 4. Lockfiles and deterministic resolution

### 4.1 Rust application workspace

- Commit `Cargo.lock` for executable applications and financial calculation workspaces.
- Use `--locked` in CI and documented release builds.
- Review lockfile changes as part of the dependency update Pull Request.
- Keep direct dependency versions and feature choices explicit in Cargo manifests.
- Re-run formatting, Clippy, tests, documentation, and applicable financial invariants after an update.

The OFRL-IB-001 application workspace currently uses a committed lockfile and locked CI commands.

### 4.2 Isolated Rust POCs

An isolated package excluded from the main Cargo workspace is a separate dependency-resolution boundary.

- Maintain its own `Cargo.lock` when it is used in CI or to support a technical decision.
- Use `--locked` in its CI commands after the lockfile is committed.
- Do not assume the workspace lockfile governs an excluded package.
- Label POC conclusions as conditional on the tested lockfile, toolchain, features, and test coverage.

The OFRL-IB-001 Decimal POC is currently a separate package. Its dependency review identifies that no POC-local lockfile was committed at the audit snapshot; closing that gap is tracked as remediation.

### 4.3 JavaScript / Node validation tools

- Commit a package manifest and lockfile for persistent validation tooling.
- Use `npm ci` in CI, not an unconstrained install.
- Direct version pins without a lockfile do not freeze the transitive dependency graph.
- Do not hide dependency resolution by installing unrecorded packages into a temporary directory in a release-critical validation path.

The current JSON Schema validator installs exact direct package versions into a temporary directory while disabling lockfile generation. This passes the current schema tests, but it does not freeze transitive dependency resolution. It must be migrated to a committed manifest and lockfile before the validator becomes a mandatory release-wide gate.

## 5. GitHub Actions and workflow security

- Pin every third-party Action to a verified full-length commit SHA.
- Include the upstream release/tag in a comment for human review where useful.
- Use the narrowest possible workflow permissions; default to read-only.
- Set `persist-credentials: false` for checkout unless a later step demonstrably requires write credentials.
- Do not expose repository secrets to fork Pull Requests.
- Do not use `pull_request_target` to execute untrusted contribution code with privileged credentials.
- Review every Action update as a dependency change, including its transitive behavior and permission requirements.
- Prefer a deterministic toolchain already provided by the runner or a verified versioned installer. Do not silently fetch and execute an unpinned bootstrap script as a fallback.

Current OFRL workflows pin `actions/checkout` to a full-length SHA, use `contents: read`, and disable persisted checkout credentials. They also contain a fallback that downloads the current rustup installation script when `rustup` is absent; that fallback is scheduled for removal.

## 6. Vulnerability and maintenance updates

Dependabot should be configured for every package ecosystem actually introduced into the repository. At the current audit baseline, the configuration covers:

- Cargo dependencies in the OFRL-IB-001 application workspace.
- Cargo dependencies in the isolated Decimal POC.
- GitHub Actions references under `.github/workflows`.

When a future project introduces another ecosystem (for example npm, Python, or container images), its package path and update policy must be added to `.github/dependabot.yml` in the same Pull Request that establishes the dependency boundary. Do not add unused ecosystem entries merely for appearance.

Dependabot Pull Requests are proposals, not automatic approvals. A maintainer must review the version change, release notes, licence, transitive graph, compatibility, and test evidence. Do not automatically merge dependency updates.

Security updates take priority over routine version updates. Record accepted risk, temporary mitigations, or deferrals in an Issue or Decision Record. A high or critical advisory affecting released software must be assessed promptly and may require an out-of-band patch release.

## 7. SBOM and release provenance

For each distributable software release, provide a machine-readable Software Bill of Materials (SBOM) in SPDX or CycloneDX format where the build ecosystem supports it.

The release record should identify:

- Source commit and version/tag.
- Direct and transitive dependencies.
- Toolchain/runtime version and target platform.
- Build command and relevant feature flags.
- SBOM format and generation tool/version.
- Test and validation results.
- SHA-256 checksums for published binary artifacts.

An SBOM is an inventory, not proof that every dependency is secure, correctly licensed, or free of vulnerabilities. It must be reviewed alongside the dependency and licence records.

## 8. Dependency exceptions

An exception must identify:

- The dependency and affected component.
- The requirement that cannot currently be met.
- Risk and impact.
- Compensating controls.
- Responsible owner.
- Expiry or review date.

An exception does not waive third-party licence terms, data rights, or security obligations.

## 9. Project-wide application

Every project that introduces a runtime, build tool, data-processing package, external Action, service integration, or distributable artifact must identify its dependency boundary in its project specification and manifest. A project may use a technology not listed in the current Dependabot configuration; the project owner must add the corresponding update mechanism before the project relies on that ecosystem for repeatable builds or releases.

The common repository workflow validates repository metadata and project registration. It does not replace project-specific build, test, security, or release workflows. Each software-bearing project must own the checks appropriate to its stack and risk tier.

## 10. Related documents

- [OFRL Governance](GOVERNANCE.md)
- [Research Standards](RESEARCH_STANDARDS.md)
- [Licensing and Reuse Policy](LICENSE_POLICY.md)
- [Data Policy](DATA_POLICY.md)
- [Release Policy](RELEASE_POLICY.md)
- [Repository Audit](REPOSITORY_AUDIT.md)
- [OFRL-IB-001 Dependency Review](projects/OFRL-IB-001/poc/decimal-contract/DEPENDENCY_REVIEW.md)
