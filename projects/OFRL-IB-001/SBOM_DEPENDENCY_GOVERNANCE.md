<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Dependency Governance and SBOM Policy

**Policy version:** 0.1.0  
**Status:** Inventory automation implemented; security and licence approval remain open  
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
- `cargo-cyclonedx 0.5.9`, pinned at the tool version.
- CycloneDX JSON specification 1.5.
- Rust toolchain 1.99.1 used only to run the inventory tool; the application MSRV remains unchanged.
- `SOURCE_DATE_EPOCH` is set from the source commit timestamp so generated SBOM metadata avoids a random serial number and uses a stable timestamp.
- CI uses all Cargo features and all target platforms for a conservative dependency inventory.
- Separate SBOMs are produced for the two production workspace crates and each isolated POC.
- SHA-256 checksums and source commit metadata accompany the artifact.

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
| Advisory review | RustSec advisory snapshot and disposition of every applicable finding | Open |
| Maintenance / unsoundness review | Package status and documented exception rationale | Open |
| Licence expression review | Direct + transitive package licence inventory and approved policy | Open |
| Notice / source obligations | Required notices and source obligations for actual release distribution | Open |
| Duplicate / untrusted dependency sources | Source and duplicate-version review | Open |
| Release attestation | SBOM tied to release source/build identity | Open |

No licence allow-list is introduced in this change. A licence policy must be approved from the actual transitive graph rather than guessing an allow-list before the SBOM is available. Unknown or ambiguous licence metadata must be reviewed, not silently accepted.

The workflow also does not claim a vulnerability-free result. A separate advisory and maintenance check must be configured and run against the committed lockfiles before Phase 2 can exit.

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

**Current conclusion:** SBOM inventory automation is implemented as a project-scoped workflow. It is not a completed dependency/security/licence audit, and it does not authorize the Production Forecast Engine.
