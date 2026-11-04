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

The OFRL-IB-001 Decimal POC is a separate package with its own `Cargo.lock`, generated from its own manifest. Its CI validates the frozen graph with `cargo metadata --locked` and `cargo tree --locked`, then runs Clippy, tests, and documentation with `--locked` on the declared MSRV and the newer compatibility toolchain. The lockfile does not replace source-level licence/security review or release SBOM generation.

### 4.3 JavaScript / Node validation tools

- Commit a package manifest and lockfile for persistent validation tooling.
- Use `npm ci` in CI, not an unconstrained install.
- Direct version pins without a lockfile do not freeze the transitive dependency graph.
- Do not hide dependency resolution by installing unrecorded packages into a temporary directory in a release-critical validation path.

The repository now maintains an isolated schema-validation package at `scripts/schema-validator/`, with exact direct dependencies and a committed lockfile for the resolved transitive graph. CI runs `npm ci` with lifecycle scripts disabled before compiling the project-manifest schema and validating OFRL-IB-001 model-input fixtures. The package is development/CI tooling only and is not a runtime dependency of OFRL projects.

## 5. GitHub Actions and workflow security

- Pin every third-party Action to a verified full-length commit SHA.
- Include the upstream release/tag in a comment for human review where useful.
- Use the narrowest possible workflow permissions; default to read-only.
- Set `persist-credentials: false` for checkout unless a later step demonstrably requires write credentials.
- Do not expose repository secrets to fork Pull Requests.
- Do not use `pull_request_target` to execute untrusted contribution code with privileged credentials.
- Review every Action update as a dependency change, including its transitive behavior and permission requirements.
- Prefer a deterministic toolchain already provided by the runner or a verified versioned installer. Do not silently fetch and execute an unpinned bootstrap script as a fallback.

Current OFRL workflows pin `actions/checkout` to a full-length SHA, use `contents: read`, and disable persisted checkout credentials. They require the runner-provided `rustup` executable and fail closed if it is absent; no mutable bootstrap script is fetched or executed. Each workflow installs its declared Rust toolchain versions and components explicitly.

## 6. Vulnerability and maintenance updates

Dependabot should be configured for every package ecosystem actually introduced into the repository. At the current audit baseline, the configuration covers:

- Cargo dependencies in the OFRL-IB-001 application workspace.
- Cargo dependencies in the isolated Decimal POC.
- GitHub Actions references under `.github/workflows`.
- npm dependencies in `scripts/schema-validator`.

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
- SBOM component scope and the lockfile from which it was generated.
- Source commit timestamp used for reproducible SBOM generation, when supported.

For OFRL-IB-001, the preview workflow uses `cargo-cyclonedx 0.5.9`, pinned by version and installed with `cargo install --locked`. It generates a CycloneDX 1.5 JSON SBOM for the `ofrl_cli` executable and its resolved application-workspace dependencies on the runner's actual target. The exact `projects/OFRL-IB-001/Cargo.lock` is copied into the preview bundle; the workflow verifies that SBOM generation did not modify that lockfile. The separate Decimal POC lockfile is not combined with the application SBOM.

The workflow creates and verifies `SHA256SUMS` for the executable, SBOM, build metadata, and copied lockfile. The preview bundle is uploaded as a GitHub Actions artifact, not published as a Release. The six required `main` checks remain unchanged; this preview workflow is separate and is not a required status check.

An SBOM is an inventory, not proof that every dependency is secure, correctly licensed, or free of vulnerabilities. It must be reviewed alongside the dependency and licence records. Cargo publisher metadata is not a substitute for inspecting exact source archives, licence texts, notices, copyright statements, exceptions, and feature-specific obligations. The workflow artifact archive digest is not a replacement for the per-file SHA-256 manifest.

## 8. Dependency exceptions

An exception must identify:

- The dependency and affected component.
- The requirement that cannot currently be met.
- Risk and impact.
- Compensating controls.
- Responsible owner.
- Expiry or review date.

An exception does not waive third-party licence terms, data rights, or security obligations.

### 8.1 Temporary build-tool exception — CycloneDX preview generator

**Status:** Accepted as a time-bounded build-tool exception for the preview workflow only; review required by **2027-02-04** or earlier if an upstream release removes the yanked dependency.

| Field | Record |
|---|---|
| Direct tool | `cargo-cyclonedx 0.5.9` |
| Transitive package | `xml-rs 0.8.19` |
| Registry source | crates.io |
| Package checksum | `0fcb9cbac069e033553e8bb871be2fbdffcab578eb25bd0f7c508cedc6dcd75a` |
| Publisher-declared licence | MIT |
| Affected scope | The CycloneDX generator used by the preview CI job; not an OFRL-IB-001 application runtime dependency and not part of the generated `ofrl_cli` SBOM |
| Upstream evidence | The `cargo-cyclonedx-0.5.9` source tag and its upstream `Cargo.lock` pin this exact version/checksum. The currently inspected upstream lockfile also retains it. |
| Registry state | The crates.io index marks `xml-rs 0.8.19` as yanked. The reason for yanking was not established from the available upstream changelog or registry metadata. |
| Advisory check | No matching `xml-rs` entry surfaced in the RustSec advisory database search performed on 2026-11-04. This is not a guarantee that no vulnerability or other risk exists. |
| Owner | Founder / Principal Maintainer |
| Review deadline | 2027-02-04, or earlier when an upstream release removes the yanked version |

**Observed behaviour:** `cargo install --locked --version 0.5.9 cargo-cyclonedx` succeeds using the published generator lockfile, but Cargo emits a warning that `xml-rs 0.8.19` is yanked. Cargo's existing-lockfile behaviour permits an already locked yanked version to remain in use; it cannot be selected for a new unconstrained resolution. The preview uses the generator's JSON output mode; the upstream library also contains XML APIs that compile against `xml-rs`.

**Compensating controls:**

- Pin the direct generator version and install with `cargo install --locked`; do not update its lockfile ad hoc or remove `--locked` to silence the warning.
- Keep the generator confined to the ephemeral, read-only release-preview runner. It receives no secrets or repository write permissions.
- Preserve the exact generator version, source ref, lockfile checksum, toolchain, target, generated SBOM, and artifact digest in the workflow evidence.
- Reassess upstream releases and the transitive dependency before extending this generator to official releases.
- Do not infer that the application itself depends on `xml-rs`; it is part of the SBOM generator's build-tool graph.

The exception records the observed condition; it does not explain or endorse the registry's yank decision. If the reason remains unknown at the review deadline, the maintainer must re-evaluate the generator or replace it rather than silently renewing the exception.

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
