<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Dependency and Source Review

**Review version:** 0.1.2  
**Task:** IB-T005 — Dependency and source acceptance  
**Issue:** [#16 — Rust supply-chain controls](https://github.com/witthaya-quant/Open-Finance-Research-Lab/issues/16)  
**Status:** OPEN — automated source inventory and advisory evidence are verified on PR #34; human package-level source, licence, notice and release acceptance remain pending  
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

The new [Dependency Audit workflow](../../.github/workflows/ofrl-ib-001-dependency-audit.yml) runs for relevant pull-request changes, supports manual dispatch, and schedules a weekly source-inventory and RustSec recheck on the default branch after merge (Monday 06:17 UTC). It remains separate from the six required Ruleset contexts and does not add a new branch-protection requirement. The workflow is designed to:

1. Resolve package metadata from each exact committed lockfile with the declared Rust toolchain and `--locked`.
2. Fetch the exact committed-lockfile package archives with `cargo fetch --locked`, without compiling or executing package build scripts.
3. Let Cargo verify each downloaded registry archive against its lockfile checksum.
4. Cross-check publisher-declared `license`, `license-file`, and `repository` fields against the downloaded source archive's `Cargo.toml`.
5. Record candidate licence, copyright, patent, attribution and notice files by path, size and SHA-256.
6. Preserve the source revision, lockfile hashes, tool version and advisory database commit in the CI evidence artifact.
7. Keep fetched source archives in the ephemeral runner Cargo cache; upload only the bounded inventory and audit evidence, not third-party source archives.

**Interpretation boundary:** a crates.io source identifier and matching checksum establish which registry archive was retrieved. A package's `repository` field remains publisher-declared metadata; it is not proof that the linked upstream repository is owned or controlled by the publisher. File-name/hash inventory identifies review candidates; it is not a legal interpretation or copyright clearance.

The inventory computes SHA-256 against each lockfile checksum and hashes candidate licence, Unlicense, copyright, patent, attribution and notice files from the downloaded package archive. The package-level inventory is not considered accepted until the relevant source, licence, attribution, notice, patent, exception and redistribution obligations have been reviewed and dispositioned. Missing or inconsistent metadata must remain visible; it must not be inferred or silently accepted.

### 3.1 Final PR-head source archive inventory — PR #34

The final source-inventory and advisory-evidence run for PR #34 completed successfully against source commit `9bd7f44c5ee81e852cd47ce498e475ed9b0cec3f`:

- Workflow run: [#42186587738 — OFRL-IB-001 Dependency Audit](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42186587738).
- Artifact: `ofrl-ib-001-dependency-audit-9bd7f44c5ee81e852cd47ce498e475ed9b0cec3f` (ID `13488210692`; 17,549 bytes; retained through 2026-12-06).
- Artifact archive SHA-256: `1945a83d8730d036f3e7666df42aa5cc1b394c00ff3b29ddf5f4d0ba259676a4`; the downloaded archive was independently hashed and matched GitHub's reported digest.
- Inventory schema: `1.2.0`, with dedicated licence-filename and embedded licence-section detection.
- Exact lockfile SHA-256 values: application `485a9c8d5e7d04a7abf0cd266540c7499dd49c51999c3c553532ad9200864c90`; Decimal Contract POC `288b19b1e97cab000df902961af302d0ba4413bad5dabd731da7feddaf4f9d56`.
- The artifact's `audit-context.json` records the same source commit, Rust 1.85.0, checksum-pinned `cargo-audit 0.22.2`, and RustSec database commit `26fd2d43f7280a1fb925a3743ec463017afe7119`.
- This is a pull-request source commit, not a `main` run. The later synchronization commits for this PR change documentation only; they do not change the audited workflow, Cargo manifests or lockfiles.

| Scope | Locked registry packages | Source archives matched lockfile SHA-256 | Cargo metadata cross-checks | Licence-text candidates | Packages without a licence-text candidate |
|---|---:|---:|---:|---:|---:|
| Application workspace | 52 | 52 / 52 | 52 / 52 | 52 / 52 | 0 |
| Decimal Contract POC | 55 | 55 / 55 | 55 / 55 | 55 / 55 | 0 |
| **Total records** | **107** | **107 / 107** | **107 / 107** | **107 / 107** | **0** |

The downloaded artifact was inspected directly. Every package record has `metadata_cross_check=MATCH`, `repository_metadata_matches_source_archive=true`, and `license_text_status=FOUND_IN_SOURCE_ARCHIVE`. The 107 records still carry `source_review=PENDING` and `licence_review=PENDING`; candidate discovery is not legal acceptance.

The application RustSec JSON reports 54 locked dependencies, zero vulnerabilities and zero warnings. The Decimal Contract POC report records 56 locked dependencies, zero vulnerabilities and zero warnings. Both exit codes are zero; the exact database snapshot and interpretation are recorded in §5.1.

The corrected matcher detects `r-efi 5.3.0` licence material embedded in `AUTHORS` in both dependency scopes, including the four headings `LICENSE`, `AUTHORS-MIT`, `AUTHORS-ASL`, and `AUTHORS-LGPL`.

## 4. Licence, copyright and notice review

### 4.1 r-efi 5.3.0 — licence material embedded in AUTHORS

The first schema 1.1 inventory classified `r-efi 5.3.0` as `NOT_FOUND_IN_SOURCE_ARCHIVE` for licence text because it used a filename-only rule. That was a **false negative**: the archive contains an `AUTHORS` file, and the upstream source stores its licence material inside that file rather than in a separately named `LICENSE` file.

The tagged upstream source was checked directly:

- Upstream release tag: [`r-efi/r-efi@v5.3.0`](https://github.com/r-efi/r-efi/tree/v5.3.0).
- [README.md — License section](https://github.com/r-efi/r-efi/blob/v5.3.0/README.md) declares `MIT OR Apache-2.0 OR LGPL-2.1-or-later` and explicitly says to see `AUTHORS` for details.
- [Cargo.toml](https://github.com/r-efi/r-efi/blob/v5.3.0/Cargo.toml) declares the identical Cargo `license` expression and the same repository URL as the source archive metadata.
- [AUTHORS](https://github.com/r-efi/r-efi/blob/v5.3.0/AUTHORS) contains the headings `LICENSE`, `AUTHORS-MIT`, `AUTHORS-ASL`, and `AUTHORS-LGPL`. These sections state the triple-licensing arrangement, reproduce MIT terms, reference the Apache-2.0 text, and state the LGPL-2.1-or-later terms with a reference to the GNU licence text.
- The file also contains copyright and author attribution sections. It is not merely an authors list.

The package occurs once in the application lockfile and once in the isolated Decimal POC lockfile. The source-archive inventory records `AUTHORS` in both scopes (3,693 bytes; archive SHA-256 `ff92bed461f50338dd703a9ba9aee496a425957873df1d91192776e4bdf5dda7`). The original schema 1.1 result did not inspect embedded section headings and therefore must not be interpreted as proof that the package omitted its licence material.

The corrected schema 1.2.0 matcher was verified on the final dependency-audit source commit by [workflow run #42186587738](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42186587738). Its artifact (`13488210692`; SHA-256 `1945a83d8730d036f3e7666df42aa5cc1b394c00ff3b29ddf5f4d0ba259676a4`) records all four embedded headings under `license_text_files` and reports `FOUND_IN_SOURCE_ARCHIVE` for `r-efi 5.3.0` in both dependency scopes. The application and POC inventories each report zero packages without a licence-text candidate.

This is a classification improvement, not a legal approval or a claim that every referenced upstream licence document is reproduced in full.

**Disposition:** the prior missing-text finding is withdrawn. Source provenance and licence metadata are internally consistent at the inspected tag, but the package's licence alternatives, complete referenced terms, required notices and downstream distribution obligations remain subject to human review. Keep `source_review=PENDING` and `licence_review=PENDING` until that review is recorded.


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

The earlier [PR #11 dependency audit](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/11) is historical evidence only. The new workflow uses the [verified cargo-audit 0.22.2 release](https://github.com/rustsec/rustsec/releases/tag/cargo-audit%2Fv0.22.2) and [Cargo fetch](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html) against the current committed lockfiles. The historical audit was:

- Tool: RustSec `cargo-audit 0.22.2`.
- Advisory DB commit: `965bf9ad083b58e6340d65829d85217b18be9290`.
- Advisory DB timestamp recorded by that run: `2026-11-03T21:34:28+01:00`.
- Historical result: zero vulnerabilities and zero warnings on the production workspace and two POC snapshots.
- Source: PR #11 head `7ae94f8489a57fa7cbf5ec9d6970cbe88daad8cb`, not current `main); the Decimal Contract POC lockfile used in that run was an ephemeral snapshot.

That scan does not satisfy the current-head advisory gate. The new workflow runs `cargo audit --file <committed-lockfile> --deny warnings --json` against the current application and committed Decimal POC lockfiles, then records the exact RustSec database commit and commit time. No advisory ignore list is supplied. Any non-zero result is blocking until dispositioned.

A clean point-in-time scan does not prove that a package is free of vulnerabilities, establish exploitability or reachability, cover operating-system libraries, or detect defects not represented in the RustSec database.

### 5.1 Current RustSec result — PR #34 final dependency-audit run

Workflow run [#42186587738](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42186587738) audited the exact committed application and Decimal Contract POC lockfiles at source commit `9bd7f44c5ee81e852cd47ce498e475ed9b0cec3f` with checksum-pinned `cargo-audit 0.22.2`, `--deny warnings`, and no ignore list.

| Scope | Audit dependency count | Vulnerabilities | Warnings | Exit |
|---|---:|---:|---:|---:|
| Application workspace | 54 | 0 | 0 | 0 |
| Decimal Contract POC | 56 | 0 | 0 | 0 |

RustSec Advisory Database snapshot:

- Commit: `26fd2d43f7280a1fb925a3743ec463017afe7119`.
- Timestamp: `2026-11-05T09:50:49+01:00`.
- Advisory records reported by the audit: 1,409.
- Both JSON reports record `vulnerabilities.found=false`, `vulnerabilities.count=0`, and an empty `warnings` object. Informational warning categories Unmaintained, Unsound and Notice were enabled as deny-worthy warnings.

The uploaded exact-run artifact is ID `13488210692`, size 17,549 bytes, archive SHA-256 `1945a83d8730d036f3e7666df42aa5cc1b394c00ff3b29ddf5f4d0ba259676a4`. The audit context records source commit `9bd7f44c5ee81e852cd47ce498e475ed9b0cec3f`, Rust toolchain `1.85.0`, the checksum-pinned scanner archive, and the exact advisory database revision.

This is successful, point-in-time evidence for the two lockfiles and database commit above. It is not proof of absence of vulnerabilities, exploitability, application defects, system-library risks or issues outside RustSec. It does not resolve the `xml-rs 0.8.19` exception in the separate CycloneDX generator graph. The scan must be repeated after dependency/manifest/lockfile/workflow changes and on the default branch after merge as required by the release process.

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

The [upstream cargo-cyclonedx release page](https://github.com/CycloneDX/cyclonedx-rust-cargo/releases) currently identifies `cargo-cyclonedx 0.5.9` as the latest `cargo-cyclonedx` release found in this review. No later `cargo-cyclonedx` release resolving the transitive dependency was identified. The upstream project also publishes a prebuilt Linux binary with a release checksum; using that binary can avoid compiling the tool's dependency graph in OFRL CI, but does not remove or independently clear the dependency embedded in the generator. It is therefore an execution-boundary mitigation, not a replacement for source/security review.

Alternative generators or an SPDX-based pipeline may be evaluated only against the same acceptance requirements: complete component/dependency coverage, exact lockfile identity, licence metadata fidelity, deterministic output, target handling, checksum verification, maintenance and source review. No replacement is selected by this record.

**Review deadline:** 2027-02-04, or earlier if upstream publishes a release that removes the yanked package. If the reason remains unknown at the deadline, the exception must be replaced or explicitly re-approved with updated evidence; it must not renew silently.

## 8. Current disposition and release boundary

| Control | Current disposition |
|---|---|
| Committed application lockfile and package inventory | PASS — baseline identities verified; 52 registry packages in the application lockfile |
| Isolated POC lockfile and package inventory | PASS — separate baseline; 55 registry packages in the committed POC lockfile |
| Registry source-archive integrity | PASS on PR #34 dependency-audit source commit `9bd7f44` — 107 / 107 archive SHA-256 values matched committed lockfile checksums; rerun on `main` after merge |
| Cargo metadata/source-manifest cross-check | PASS on the same source commit — 107 / 107 package records matched for identity, licence expression, licence-file and repository fields |
| Licence/notice candidate discovery | PASS as inventory only — 107 / 107 package records have a detected licence-text candidate; all 107 remain pending human review |
| Package-by-package source and repository provenance review | OPEN — all 107 records retain `source_review=PENDING`; publisher repository metadata is not proof of ownership/control |
| Package-by-package licence, copyright, patent and notice disposition | OPEN — all 107 records retain `licence_review=PENDING`; alternatives, exceptions and distribution obligations require human disposition |
| Current RustSec audit on both committed lockfiles | PASS on PR #34 source commit — 0 vulnerabilities and 0 warnings on both lockfiles against the recorded 1,409-entry database snapshot; point-in-time only, rerun after merge and relevant changes |
| Duplicate-version and source exception review | OPEN — five duplicate package names in the application lockfile require target/feature review; no Git sources were found in either current lockfile |
| Preview-generator exception review | OPEN; restricted to the preview-only generator and due 2027-02-04 or earlier if upstream removes the yanked package |
| Official release SBOM/checksums and reproducible release identity | NOT DONE — requires the exact approved release source and supported target matrix |
| Release authorization | NOT AUTHORIZED |

**IB-T005 remains OPEN.** PR #34 delivers repeatable technical inventory and point-in-time advisory evidence; it does not approve any package or authorize distribution. The exact source audit is on an unmerged PR commit, not `main`; the scheduled default-branch recheck becomes active after merge. Human source/licence/notice dispositions, the generator exception, the eventual production dependency graph after IB-T004, official release SBOM/checksums, reproduction and publication authorization remain separate gates. No forecast engine work is authorized by this document.


