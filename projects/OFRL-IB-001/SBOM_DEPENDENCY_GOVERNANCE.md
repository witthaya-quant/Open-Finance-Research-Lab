<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Dependency Governance and SBOM Policy

**Policy version:** 0.1.0  
**Status:** SBOM inventory automated; latest RustSec scan found zero vulnerabilities/warnings; source and licence approval remain open  
**Scope:** OFRL-IB-001 production Rust workspace and its isolated Rust POCs

## 1. Purpose

Make the Rust dependency graph reviewable, reproducible and attributable to a source revision before the production Forecast Engine is approved.

A dependency inventory is evidence, not approval. A CycloneDX SBOM does not by itself prove that a package is secure, maintained, correctly licensed, or legally acceptable.

The policy separates:

1. **Resolution control:** exact dependency graph is locked and changes are reviewable.
2. **Inventory:** machine-readable SBOM records packages, versions, relationships, package URLs and available licence metadata.
3. **Security review:** known advisories, unmaintained/unsound packages and source trust are reviewed.
4. **Licence review:** direct and transitive licence expressions and notices receive an explicit policy decision.
5. **Release evidence:** the exact dependency graph and checks are attached to a source commit.

## 2. Current workspace inventory

| Workspace | Purpose | Lockfile state | Governance status |
|---|---|---|---|
| `projects/OFRL-IB-001` | Production model library + CLI | Committed `Cargo.lock` | Locked; SBOM generated |
| `poc/model-numeric-envelope` | Isolated arithmetic research POC | Committed `Cargo.lock` | Locked; SBOM generated |
| `poc/decimal-contract` | Isolated Decimal / interchange POC | No committed `Cargo.lock` | **Open:** CI generates an ephemeral lock snapshot for discovery only |

The isolated POCs are not production runtime dependencies. Their dependency graphs must nevertheless remain visible because they inform design decisions and may later be retired or promoted through a separately reviewed change.

## 3. SBOM generation

Workflow: `.github/workflows/ofrl-ib-001-dependency-sbom.yml`

Tooling:
- `cargo-cyclonedx 0.5.9`, pinned by official release and SHA-256 checksum.
- `cargo-audit 0.22.2`, pinned by official release and SHA-256 checksum.
- CycloneDX JSON specification 1.5.
- Rust toolchain 1.99.1 used only to run Cargo metadata; the inventory and audit tools are verified prebuilt binaries, and the application MSRV remains unchanged.
- `SOURCE_DATE_EPOCH` is set from the source commit timestamp so generated SBOM metadata avoids a random serial number and uses a stable timestamp.
- CI uses all Cargo features and all target platforms for a conservative dependency inventory.
- Separate SBOMs are produced for the two production workspace crates and each isolated POC.
- SHA-256 checksums and source commit metadata accompany the artifact.
- RustSec advisory checks run separately for the production lockfile and each POC lockfile using `cargo audit --deny warnings --json`. No advisory IDs are ignored by default. A non-zero exit (finding or database/tool error) fails the workflow after the JSON reports have been uploaded, so the evidence remains available for triage.

The workflow does not add `cargo-cyclonedx` or any audit tool to the production Cargo workspace. It does not change production or numeric-POC lockfiles.

### Decimal Contract POC exception

The Decimal Contract POC does not yet have a committed lockfile. The workflow creates a temporary resolution for that POC, copies the generated lockfile into the SBOM artifact as `decimal-contract.Cargo.lock.ephemeral`, and removes it from the runner checkout after generation.

This snapshot is useful for package discovery but is **not reproducible evidence** and must not be presented as a locked dependency graph. It does not close the lockfile gate. Before this POC is used as a release or production input, its lockfile must be reviewed and committed or the POC must be formally retired.

## 4. Security and licence review status

The following controls are required but are not claimed as complete by the SBOM workflow:

| Control | Required evidence | Status |
|---|---|---|
| Complete production dependency inventory | CycloneDX SBOM for each production crate | Automated |
| POC dependency inventory | Separate SBOMs and explicit lockfile status | Automated; Decimal POC remains ephemeral |
| Lockfile immutability | CI proves committed lockfiles unchanged during generation | Automated |
| Advisory review | RustSec scan for each lockfile, with JSON report and no implicit ignores | Automated fail-closed; any finding requires triage |
| Maintenance / unsoundness review | `cargo-audit --deny warnings` findings and documented exception rationale | Automated fail-closed; manual disposition remains required |
| Licence expression review | Direct + transitive package licence inventory and approved policy | Open |
| Notice / source obligations | Required notices and source obligations for actual release distribution | Open |
| Duplicate / untrusted dependency sources | Source and duplicate-version review | Open |
| Release attestation | SBOM tied to release source/build identity | Open |

No licence allow-list is introduced in this change. A licence policy must be approved from the actual transitive graph rather than guessing an allow-list before the SBOM is available. Unknown or ambiguous licence metadata must be reviewed, not silently accepted.

The workflow does not claim a vulnerability-free result merely because the scanner is configured. The current-head run must pass against the latest fetched RustSec advisory database, and any warning must be triaged with an explicit disposition. The Decimal Contract POC audit is only a snapshot of that run's ephemeral lock resolution until its lockfile is committed.

## 5. Dependency change control

1. Any dependency change must include the relevant `Cargo.toml` and `Cargo.lock` diff.
2. Do not use wildcard dependency versions in production.
3. Do not update dependencies implicitly as part of unrelated feature work.
4. Review direct and transitive changes, duplicate versions, source changes and platform-specific dependencies.
5. Run the full applicable test/toolchain matrix and regenerate the SBOM.
6. Record material exceptions, rejected advisories and licence decisions with rationale and review date.
7. POC-only dependencies must not leak into the production workspace without a separate architecture and licence/security review.

## 6. Phase 2 exit criteria

The Numeric Foundation phase remains blocked until all of the following are evidenced:

- Production and retained POC dependency graphs have committed lockfiles or explicit retirement decisions.
- SBOMs are generated from the reviewed locked graphs.
- Applicable vulnerability and maintenance advisories are reviewed and dispositioned.
- Every direct and transitive licence expression is reviewed against an approved licence policy.
- Required notices and source obligations are identified.
- The Rust numeric backend and precision policy are approved separately; dependency approval does not select the financial numeric representation.

**Current conclusion:** SBOM inventory automation is implemented as a project-scoped workflow. The latest pinned RustSec scan returned zero vulnerabilities and zero warnings for all three audited lockfile snapshots; see [Dependency Audit Results](DEPENDENCY_AUDIT_RESULTS.md). This is a point-in-time advisory scan, not a completed software-supply-chain, source, licence or legal audit, and it does not authorize the Production Forecast Engine.


## 7. License expressions observed in the current SBOM

The 2026-11-04 inventory recorded the following SPDX expressions across the four workspaces. These are expressions as declared by package metadata; an `OR` expression records alternative licensing options and is not a conclusion that every listed licence is imposed simultaneously.

- `(MIT OR Apache-2.0) AND Unicode-3.0`
- `Apache-2.0`
- `Apache-2.0 OR BSL-1.0`
- `Apache-2.0 OR MIT`
- `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`
- `BSD-2-Clause OR Apache-2.0 OR MIT`
- `MIT`
- `MIT OR Apache-2.0`
- `MIT OR Apache-2.0 OR LGPL-2.1-or-later`
- `Unlicense OR MIT`

No component in these four generated SBOMs had missing license metadata. This does not replace verification against the original licence texts, notice files, distribution obligations or a founder-approved licence policy. No allow-list or legal acceptance is inferred here.
