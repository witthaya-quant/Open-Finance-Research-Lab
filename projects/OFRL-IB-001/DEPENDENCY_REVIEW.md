<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Dependency and Source Review

**Review version:** 0.1.0  
**Task:** IB-T005 — Dependency and source acceptance  
**Issue:** [#16 — Rust supply-chain controls](https://github.com/witthaya-quant/Open-Finance-Research-Lab/issues/16)  
**Status:** OPEN — inventory controls are being established; human source, licence, notice and release acceptance remain pending  
**Baseline:** `main` at `cdbf345c6e27f1cd82f3419b164eb4a65575c09b`  
**Review date:** 2026-11-06

## 1. Purpose and authority

This document is the IB-001 evidence register for direct and transitive dependency provenance, source-archive integrity, licence and notice review, advisory disposition, dependency exceptions, and reproducible release evidence.

It supports OFRL's Vision and Mission by making the software used to produce financial research inspectable, attributable, reproducible, and open to review. It does not select the financial arithmetic backend, approve financial formulas, or authorize a software release.

The canonical execution status and task dependencies remain in [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md). This file records IB-T005 evidence and decisions; it is not a second task board.

Applicable policies:
- [Dependency Policy](../../DEPENDENCY_POLICY.md)
- [Licensing and Reuse Policy](../../LICENSE_POLICY.md)
- [Release Policy](../../RELEASE_POLICY.md)
- [Engineering Architecture](ENGINEERING_ARCHITECTURE.md)
- [Engineering Audit](ENGINEERING_AUDIT.md)

## 2. Dependency boundaries

The lockfiles below are separate trust and review boundaries. They must not be combined or treated as interchangeable.

| Scope | Manifest / lockfile | Locked package records | Registry packages | Local packages | Git sources |
|---|---|---:|---:|---:|---:|
| Application workspace | `Cargo.toml` / `Cargo.lock` | 54 | 52 | 2 (`ofrl_model`, `ofrl_cli`) | 0 |
| Decimal Contract POC | `poc/decimal-contract/Cargo.toml` / `Cargo.lock` | 56 | 55 | 1 (`ofrl-decimal-contract-poc`) | 0 |

These are lockfile inventory counts, not platform-specific runtime component counts. The application lockfile contains target-specific packages that are not all part of the current Linux CLI binary graph.

The application workspace currently supplies the Rust input domain, historical semantic validator, diagnostics and CLI. It is not yet the five-year forecast engine. The isolated Decimal Contract POC is not a production dependency and must remain separately reviewed.

### 2.1 Application workspace baseline

- Lockfile blob: `21ac76815662e765ee6502da059a312d2e9bc9eb`.
- Registry source: crates.io registry.
- No Git-sourced package is present.
- Duplicate package names across locked versions: `getrandom`, `rand`, `rand_chacha`, `rand_core`, and `syn`.
- Duplicate versions are inventory findings, not vulnerabilities. Their actual target and feature paths must be reviewed before they are banned or consolidated.

### 2.2 Decimal Contract POC baseline

- Lockfile blob: `eb2ecaed4be37129070271108911ae57f03530ba`.
- Registry source: crates.io registry.
- No Git-sourced package is present.
- The POC's existing [DEPENDENCY_REVIEW.md](poc/decimal-contract/DEPENDENCY_REVIEW.md) is limited to this POC and explicitly leaves source-level legal/security review open.

## 3. Source provenance and archive integrity

The application and Decimal POC graphs are locked to crates.io registry packages. The committed Cargo.lock records package versions, registry source identifiers, and registry archive checksums.

The new [Dependency Audit workflow](../../.github/workflows/ofrl-ib-001-dependency-audit.yml) is designed to:

1. Resolve package metadata from each exact committed lockfile with the declared Rust toolchain and `--locked`.
2. Download the corresponding source packages with `cargo vendor --locked`, without compiling or executing package build scripts.
3. Let Cargo verify each downloaded registry archive against its lockfile checksum.
4. Cross-check publisher-declared `license`, `license-file`, and `repository` fields against the downloaded source archive's `Cargo.toml`.
5. Record candidate licence, copyright, patent, attribution and notice files by path, size and SHA-256.
6. Preserve the source revision, lockfile hashes, tool version and advisory database commit in the CI evidence artifact.
7. Keep the vendored source tree outside the repository and outside the uploaded evidence artifact.

**Interpretation boundary:** a crates.io source identifier and matching checksum establish which registry archive was retrieved. A package's `repository` field remains publisher-declared metadata; it is not proof that the linked upstream repository is owned or controlled by the publisher. File-name/hash inventory identifies review candidates; it is not a legal interpretation or copyright clearance.

The package-level inventory is not considered accepted until the relevant source, licence, attribution, notice, patent, exception and redistribution obligations have been reviewed and dispositioned. Missing or inconsistent metadata must remain visible; it must not be inferred or silently accepted.

## 4. Licence, copyright and notice review

The repository's [Licensing and Reuse Policy](../../LICENSE_POLICY.md) applies to OFRL-owned material. Third-party crates remain under their own terms. OFRL's Apache-2.0 software licence does not relicense dependencies or remove their notice obligations.

Known POC metadata items requiring source-level confirmation include:

| Package(s) | Publisher-declared expression observed | Review requirement |
|---|---|---|
| `csv 1.4.0`, `csv-core 0.1.13` | `Unlicense/MIT` in the POC review metadata | Inspect both source archive licence texts and required notice/attribution obligations; do not silently reinterpret the legacy slash form. |
| `r-efi 5.3.0` | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` | Confirm alternatives and any applicable LGPL distribution/source obligations. |
| `ryu 1.0.23` | `Apache-2.0 OR BSL-1.0` | Confirm the applicable alternative and its terms. |
| `unicode-ident 1.0.27` | `(MIT OR Apache-2.0) AND Unicode-3.0` | Preserve the Unicode notice in addition to the selected permissive licence terms. |
| `wasi`, `wasip2`, `wit-bindgen` | Apache-2.0 with LLVM exception, Apache-2.0, or MIT alternatives across the inspected package metadata | Inspect exact package notices and exception text; preserve the exact licence expression. |
| `zerocopy`, `zerocopy-derive` | `BSD-2-Clause OR Apache-2.0 OR MIT` | Confirm the source archive and notices for each package/version. |

This list is a review queue, not a complete package-by-package decision. It is derived from the earlier POC inventory and must be rechecked against the current source archives.

No blanket licence allow-list or legal approval is created by this document. In particular, an SPDX expression with `OR` represents alternatives, while `AND` and `WITH` can impose combined terms or an exception. The correct downstream choice and required notices must be recorded against the exact package/version.

## 5. Security advisory assessment

The earlier PR #11 dependency audit is historical evidence only:

- Tool: RustSec `cargo-audit 0.22.2`.
- Advisory DB commit: `965bf9ad083b58e6340d65829d85217b18be9290`.
- Advisory DB timestamp recorded by that run: `2026-11-03T21:34:28+01:00`.
- Historical result: zero vulnerabilities and zero warnings on the production workspace and two POC snapshots.
- Source: PR #11 head `7ae94f8489a57fa7cbf5ec9d6970cbe88daad8cb`, not current `main); the Decimal Contract POC lockfile used in that run was an ephemeral snapshot.

That scan does not satisfy the current-head advisory gate. The new workflow runs `cargo audit --file <committed-lockfile> --deny warnings --json` against the current application and committed Decimal POC lockfiles, then records the exact RustSec database commit and commit time. No advisory ignore list is supplied. Any non-zero result is blocking until dispositioned.

A clean point-in-time scan does not prove that a package is free of vulnerabilities, establish exploitability or reachability, cover operating-system libraries, or detect defects not represented in the RustSec database.

## 6. SBOM and release provenance

The existing PR #22 preview generated a CycloneDX 1.5 SBOM for the `ofrl_cli` input validator on `x86_64-unknown-linux-gnu`:

- Workflow run: [#41838318926](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41838318926).
- Source commit: `3ca98dd26de97098f68fa0ca45b92fbb466f1d19`, triggered by `pull_request`.
- Its application lockfile blob matches the current baseline lockfile blob, but the run was not a build of the current `main` source commit.
- The target-specific SBOM contained 41 components: 40 registry packages plus local `ofrl_model`.
- The preview bundle included the exact lockfile and verified per-file SHA-256 checksums.

This validates the recorded Linux preview dependency graph, not every target in the application lockfile, a future forecast engine, or an official release.

An official release still requires a fresh SBOM generated from the exact reviewed release source and supported target matrix, release binaries with matching SHA-256 checksums, reproducible build identity, applicable licence/data/security review, and explicit release authorization.

## 7. Temporary CycloneDX generator exception

The current preview workflow uses `cargo-cyclonedx 0.5.9`. Its published locked dependency graph includes yanked `xml-rs 0.8.19` (crates.io checksum `0fcb9cbac069e033553e8bb871be2fbdffcab578eb25bd0f7c508cedc6dcd75a`). The reason for the yank has not been established. The recorded RustSec lookup on 2026-11-04 did not surface a matching `xml-rs` advisory; that is not proof of safety.

The exception remains restricted to the ephemeral, read-only preview SBOM generator. It is not an OFRL-IB-001 application runtime dependency and does not authorize use in a distributable production runtime.

The upstream release page currently identifies `cargo-cyclonedx 0.5.9` as the latest `cargo-cyclonedx` release found in this review. No later `cargo-cyclonedx` release resolving the transitive dependency was identified. The upstream project also publishes a prebuilt Linux binary with a release checksum; using that binary can avoid compiling the tool's dependency graph in OFRL CI, but does not remove or independently clear the dependency embedded in the generator. It is therefore an execution-boundary mitigation, not a replacement for source/security review.

Alternative generators or an SPDX-based pipeline may be evaluated only against the same acceptance requirements: complete component/dependency coverage, exact lockfile identity, licence metadata fidelity, deterministic output, target handling, checksum verification, maintenance and source review. No replacement is selected by this record.

**Review deadline:** 2027-02-04, or earlier if upstream publishes a release that removes the yanked package. If the reason remains unknown at the deadline, the exception must be replaced or explicitly re-approved with updated evidence; it must not renew silently.

## 8. Current disposition and release boundary

| Control | Current disposition |
|---|---|
| Committed application lockfile and package inventory | Baseline verified |
| Isolated POC lockfile and package inventory | Baseline verified; separate boundary |
| crates.io source identifiers and lockfile checksums | Inventory verified; current source-archive pass pending new workflow |
| Source archive licence/notice candidate inventory | Workflow added; exact-head run pending |
| Package-by-package source and repository provenance review | OPEN |
| Package-by-package licence, copyright, patent and notice disposition | OPEN |
| Current RustSec audit on both committed lockfiles | OPEN pending exact-head workflow result |
| Duplicate-version and source exception review | OPEN |
| Preview-generator exception review | OPEN; due 2027-02-04 |
| Official release SBOM/checksums and reproducible release identity | NOT DONE |
| Release authorization | NOT AUTHORIZED |

**IB-T005 remains OPEN.** The new workflow provides repeatable evidence collection; it does not approve a package or authorize distribution. Production dependency acceptance must be revisited against the eventual approved backend and dependency graph after IB-T004. No forecast engine work is authorized by this document.
