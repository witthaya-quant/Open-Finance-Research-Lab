<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Dependency and Source Review

**Review version:** 0.1.6  
**Task:** IB-T005 — Dependency and source acceptance  
**Issue:** [#16 — Rust supply-chain controls](https://github.com/witthaya-quant/Open-Finance-Research-Lab/issues/16)  
**Status:** OPEN — automated source inventory and advisory evidence are verified on PR #34; human package-level source, licence, notice and release acceptance remain pending  
**Baseline:** `main` at `885dbe64f930bc8096e9240de08f2947e2b918db`  
**Initial lockfile review baseline:** `cdbf345c6e27f1cd82f3419b164eb4a65575c09b`  
**Review date:** 2026-11-08

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

These are lockfile inventory counts, not platform-specific runtime component counts. The application lockfile contains target-specific packages that are not all part of the current Linux CLI binary graph. The POC lockfile is a separate resolution boundary; the same crate identity appearing in both scopes is counted twice in the scope inventory, but only once in the cross-scope unique-identity queue.

The application workspace currently supplies the Rust input domain, historical semantic validator, diagnostics and CLI. It is not yet the five-year forecast engine. The isolated Decimal Contract POC is not a production dependency and must remain separately reviewed.

### 2.1 Application workspace baseline

- Lockfile blob: `21ac76815662e765ee6502da059a312d2e9bc9eb`.
- Registry source: crates.io registry.
- No Git-sourced package is present.
- Duplicate package names across locked versions: `getrandom`, `rand`, `rand_chacha`, `rand_core`, and `syn` (two versions each). The same five version families occur in both lockfile scopes.
- Duplicate versions are inventory findings, not vulnerabilities. Their actual target and feature paths must be reviewed before they are banned or consolidated.

### 2.2 Decimal Contract POC baseline

- Lockfile blob: `eb2ecaed4be37129070271108911ae57f03530ba`.
- Registry source: crates.io registry.
- No Git-sourced package is present.
- The same five multi-version package families identified in §2.1 are also present in the POC lockfile; each exact package identity is shared across both lockfile scopes.
- The POC's existing [DEPENDENCY_REVIEW.md](poc/decimal-contract/DEPENDENCY_REVIEW.md) is limited to this POC and explicitly leaves source-level legal/security review open.

### 2.3 Cross-scope identity reconciliation

The schema 1.3.0 artifact distinguishes lockfile-scope occurrences from unique package identities:

| Measure | Verified count |
|---|---:|
| Application registry records | 52 |
| Decimal Contract POC registry records | 55 |
| Total scope records | 107 |
| Unique (name, version, source) package identities | 55 |
| Unique crate names (versions collapsed) | 50 |
| Identities shared by both lockfiles | 52 |
| Application-only identities | 0 |
| POC-only identities | 3: csv 1.4.0, csv-core 0.1.13, ryu 1.0.23 |

Accordingly, 107 is the number of package occurrences across two separately governed lockfiles, not 107 distinct packages. Human source/licence review should be tracked against the 55 unique package identities while retaining each of the 107 scope occurrences for dependency-context and SBOM applicability. Do not merge the two lockfiles or use the POC as a substitute for the application dependency graph.

## 3. Source provenance and archive integrity

The application and Decimal POC graphs are locked to crates.io registry packages. The committed Cargo.lock records package versions, registry source identifiers, and registry archive checksums.

The new [Dependency Audit workflow](../../.github/workflows/ofrl-ib-001-dependency-audit.yml) runs for relevant pull-request changes, supports manual dispatch, and schedules a weekly source-inventory and RustSec recheck on the default branch after merge (Monday 06:17 UTC). It remains separate from the six required Ruleset contexts and does not add a new branch-protection requirement. The workflow is designed to:

1. Resolve package metadata from each exact committed lockfile with the declared Rust toolchain and `--locked`.
2. Fetch the exact committed-lockfile package archives with `cargo fetch --locked`, without compiling or executing package build scripts.
3. Let Cargo verify each downloaded registry archive against its lockfile checksum.
4. Cross-check publisher-declared `license`, `license-file`, and `repository` fields against the downloaded source archive's `Cargo.toml`.
5. Record candidate licence, copyright, patent, attribution and notice files by path, size and SHA-256. Schema 1.3.0 expands filename discovery to include licence variants, COPYING/UNLICENSE, NOTICE, COPYRIGHT, PATENTS, AUTHORS, CONTRIBUTORS, ATTRIBUTION, ACKNOWLEDGEMENTS, CREDITS and third-party notice/licence files; explicitly declared license-file paths are checked even when their names are unconventional, and empty files do not count as licence-text candidates.
6. Preserve the source revision, lockfile hashes, tool version and advisory database commit in the CI evidence artifact.
7. Keep fetched source archives in the ephemeral runner Cargo cache; upload only the bounded inventory and audit evidence, not third-party source archives.
8. Compare locked registry package identities with Cargo metadata in both directions and fail on missing, extra, or duplicate identities; reject unapproved non-crates.io or Git sources.
9. Parse and cross-check the generated evidence artifact before passing: require complete scope/package coverage, archive checksum equality, metadata matches, legal-file/candidate consistency, exact lockfile hashes, exact source/tool/advisory identity, zero audit exit codes, and empty vulnerability/warning results. Preserve human source and licence review as PENDING; automation cannot grant legal acceptance.
10. Run cargo audit outside the checkout with an isolated temporary HOME and CARGO_HOME, an explicit Rust 1.85.0 toolchain override, the canonical RustSec advisory database URL, and fetching enabled. Enforce empty target-architecture/OS filters, no ignored advisories, no stale-database override, default yanked checks, --deny warnings, and validation of the effective JSON report settings.

The workflow's pull-request path filters include root and project-local Cargo source and audit configuration paths, so changes to registry resolution or cargo-audit policy trigger a new audit.

**Interpretation boundary:** a crates.io source identifier and matching checksum establish which registry archive was retrieved. A package's `repository` field remains publisher-declared metadata; it is not proof that the linked upstream repository is owned or controlled by the publisher. File-name/hash inventory identifies review candidates; it is not a legal interpretation or copyright clearance.

The inventory computes SHA-256 against each lockfile checksum and hashes candidate licence, Unlicense, copyright, patent, attribution and notice files from the downloaded package archive. The package-level inventory is not considered accepted until the relevant source, licence, attribution, notice, patent, exception and redistribution obligations have been reviewed and dispositioned. Missing or inconsistent metadata must remain visible; it must not be inferred or silently accepted.

The latest source-inventory and advisory-evidence run for PR #34 completed successfully against source commit `e04d628f01b18488c76617f00d2f03b375ac482a`:

- Workflow run: [#42475893204 — OFRL-IB-001 Dependency Audit](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42475893204).
- Artifact: `ofrl-ib-001-dependency-audit-e04d628f01b18488c76617f00d2f03b375ac482a` (ID `13615875196`; 18,429 bytes; 30-day retention).
- Artifact archive SHA-256: `9194f7e1202cd07ac4ea992720dc644c439ccf86b554fb39df07f045797c92e8`; the downloaded ZIP was independently hashed and matched GitHub's reported digest.
- Inventory schema: `1.3.0`.
- Exact lockfile SHA-256 values remain unchanged: application `485a9c8d5e7d04a7abf0cd266540c7499dd49c51999c3c553532ad9200864c90`; Decimal Contract POC `288b19b1e97cab000df902961af302d0ba4413bad5dabd731da7feddaf4f9d56`.
- The audit context records the exact source commit, Rust 1.85.0, checksum-pinned `cargo-audit 0.22.2`, canonical RustSec database remote, fetched database commit/time, isolated home/work directory, explicit toolchain override, empty target filters, no ignored advisories and enabled yanked checks.
- The inventory summary reconciles 107 scope records to 55 unique package identities / 50 crate names: 52 shared identities, zero application-only and three POC-only identities.
- Missing/extra/duplicate package identities, source mismatches, missing declared licence files, empty licence candidates, evidence inconsistencies and unexpected non-registry sources fail closed.
- This artifact was generated against PR head `e04d628f01b18488c76617f00d2f03b375ac482a` before merge. That exact source tree is now integrated into `main` as the second parent of merge commit `7dcc2463fc8f9617576f970499c67223ee0640d9`. The run remains a PR-head-triggered scan rather than a scan triggered on the merge commit itself. The weekly default-branch workflow is now active; the first post-merge scheduled run is still pending.

| Scope | Locked registry packages | Archive SHA-256 matches | Cargo metadata matches | Licence-text candidates | Packages without a candidate |
|---|---:|---:|---:|---:|---:|
| Application workspace | 52 | 52 / 52 | 52 / 52 | 52 / 52 | 0 |
| Decimal Contract POC | 55 | 55 / 55 | 55 / 55 | 55 / 55 | 0 |
| **Total scope records** | **107** | **107 / 107** | **107 / 107** | **107 / 107** | **0** |

The downloaded artifact was inspected directly. Every one of the 107 scope records has `metadata_cross_check=MATCH`, matching archive/lockfile SHA-256 values and at least one non-empty licence-text candidate. All 107 records retain `source_review=PENDING` and `licence_review=PENDING`. The corresponding human review queue contains 55 unique package identities, not 107 unique packages.

| Cross-scope inventory summary | Verified count |
|---|---:|
| Unique `(name, version, source)` identities | 55 |
| Unique package names | 50 |
| Identities present in both lockfiles | 52 |
| Application-only identities | 0 |
| POC-only identities | 3 (`csv`, `csv-core`, `ryu`) |

The same source commit passed all four workflow groups:
- Repository Policy [#42475893167](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42475893167).
- Numeric and Input Contract [#42475893173](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42475893173).
- Rust Runtime Semantic Validator [#42475893185](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42475893185).
- Dependency Audit [#42475893204](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42475893204).

All six required Ruleset contexts and the two supplemental Numeric Envelope / Dependency Audit jobs passed on the exact source head. The semantic validator passed on Rust 1.85.0 and 1.99.1.

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

An earlier schema 1.2.0 matcher validation on PR #34 source `0158b1a` was recorded by workflow run [#42229312405](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42229312405); this is historical evidence superseded by the schema 1.3.0 exact-head run documented in §3. Its artifact (`13507007891`; SHA-256 `818c24d7b272f89b206897cfbb607db5dd05ac8759d051ae51aa071a3355a2ef`) records all four embedded headings under `license_text_files` and reports `FOUND_IN_SOURCE_ARCHIVE` for `r-efi 5.3.0` in both dependency scopes. The application and POC inventories each report zero packages without a licence-text candidate.

This is a classification improvement, not a legal approval or a claim that every referenced upstream licence document is reproduced in full.

**Disposition:** the prior missing-text finding is withdrawn. Source provenance and licence metadata are internally consistent at the inspected tag, but the package's licence alternatives, complete referenced terms, required notices and downstream distribution obligations remain subject to human review. Keep `source_review=PENDING` and `licence_review=PENDING` until that review is recorded.


The repository's [Licensing and Reuse Policy](../../LICENSE_POLICY.md) applies to OFRL-owned material. Third-party crates remain under their own terms. OFRL's Apache-2.0 software licence does not relicense dependencies or remove their notice obligations.

### 4.2 SPDX expression distribution and priority source-review queue

The schema 1.3.0 artifact contains 55 unique package identities. Publisher-declared Cargo licence expressions, counted once per unique identity, are:

| Expression recorded in Cargo metadata | Unique identities |
|---|---:|
| MIT OR Apache-2.0 | 35 |
| MIT | 5 |
| Apache-2.0 OR MIT | 3 |
| Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | 3 |
| BSD-2-Clause OR Apache-2.0 OR MIT | 2 |
| Unlicense/MIT (legacy slash form; not an SPDX operator) | 2 |
| (MIT OR Apache-2.0) AND Unicode-3.0 | 1 |
| Apache-2.0 | 1 |
| Apache-2.0 OR BSL-1.0 (Boost Software License 1.0) | 1 |
| MIT OR Apache-2.0 OR LGPL-2.1-or-later | 1 |
| Unlicense OR MIT | 1 |
| Total | 55 |

Priority items for the human disposition queue:

| Package identity / family | Technical observation from the exact source inventory | Required human determination |
|---|---|---|
| csv 1.4.0, csv-core 0.1.13 | Both declare the legacy Unlicense/MIT string. Source candidates include COPYING, LICENSE-MIT and UNLICENSE. | Confirm the upstream dual-licensing statement, whether either option may be selected, attribution obligations, and whether the non-SPDX Cargo metadata string requires upstream clarification. Do not normalize the slash to OR without an explicit source/legal basis. |
| memchr 2.8.3 | Declares Unlicense OR MIT; source candidates include COPYING, LICENSE-MIT and UNLICENSE. | Confirm exact source terms and notices for the selected alternative. |
| r-efi 5.3.0 | Declares MIT OR Apache-2.0 OR LGPL-2.1-or-later; embedded licence material is in AUTHORS. | Confirm the complete alternative terms and whether the chosen alternative creates LGPL source/distribution obligations. The filename/content scan does not choose the alternative. |
| ryu 1.0.23 | Declares Apache-2.0 OR BSL-1.0; source inventory identifies LICENSE-APACHE and LICENSE-BOOST. | Confirm the exact Boost Software License 1.0 terms and required notices; do not confuse BSL-1.0 with the Business Source License. |
| unicode-ident 1.0.27 | Declares (MIT OR Apache-2.0) AND Unicode-3.0; inventory identifies LICENSE-APACHE, LICENSE-MIT and LICENSE-UNICODE. | Preserve the additional Unicode 3.0 notice/terms whichever permissive alternative is selected. |
| wasi 0.11.1+wasi-snapshot-preview1, wasip2 1.0.4+wasi-0.2.12, wit-bindgen 0.57.1 | Declare Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT; archive inventory records the Apache, LLVM-exception and MIT text candidates. | Confirm which alternative is relied upon and preserve any applicable LLVM exception/notice. Keep exact versions and component scopes. |
| zerocopy 0.8.63, zerocopy-derive 0.8.63 | Declare BSD-2-Clause OR Apache-2.0 OR MIT; inventory records the BSD, Apache and MIT text candidates. | Confirm the selected alternative and BSD attribution/disclaimer obligations for source and binary redistribution. |
| rust_decimal 1.43.0 | Core numeric candidate; declares MIT, with a source LICENSE candidate. | Review the actual crate source/version, maintainership, security history and MIT notice obligations. A permissive expression does not imply backend approval. |

The other 35 MIT OR Apache-2.0 identities, five additional MIT identities, three Apache-2.0 OR MIT identities and one Apache-only identity remain in the same 55-identity human review queue. The artifact contains the exact package/version/scope, publisher-declared expression, repository field, source-manifest comparison, legal-file paths, byte sizes and SHA-256 values; a source reviewer must follow the exact package tag/release and record the selected terms, notice requirements, provenance assessment and decision.

**No package is marked legally accepted by this scan.** Cargo metadata and a matching source-archive manifest are publisher assertions consistent with the archive; they do not authenticate control of the linked repository, establish copyright ownership, or determine the legal effect of every licence alternative. The 107 scope records remain PENDING until the responsible reviewer records a package-identity-level decision with its applicability to each dependency scope.

## 5. Security advisory assessment

The earlier [PR #11 dependency audit](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/11) is historical evidence only. The new workflow uses the [verified cargo-audit 0.22.2 release](https://github.com/rustsec/rustsec/releases/tag/cargo-audit%2Fv0.22.2) and [Cargo fetch](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html) against the current committed lockfiles. The historical audit was:

- Tool: RustSec `cargo-audit 0.22.2`.
- Advisory DB commit: `965bf9ad083b58e6340d65829d85217b18be9290`.
- Advisory DB timestamp recorded by that run: `2026-11-03T21:34:28+01:00`.
- Historical result: zero vulnerabilities and zero warnings on the production workspace and two POC snapshots.
- Source: PR #11 head `7ae94f8489a57fa7cbf5ec9d6970cbe88daad8cb`, not current `main); the Decimal Contract POC lockfile used in that run was an ephemeral snapshot.

That scan does not satisfy the current-head advisory gate. The new workflow runs `cargo audit --file <committed-lockfile> --deny warnings --json` against the current application and committed Decimal POC lockfiles, then records the exact RustSec database commit and commit time. No advisory ignore list is supplied. Any non-zero result is blocking until dispositioned.

A clean point-in-time scan does not prove that a package is free of vulnerabilities, establish exploitability or reachability, cover operating-system libraries, or detect defects not represented in the RustSec database.

### 5.1 Current RustSec result — PR #34 isolated-policy dependency-audit run

Workflow run [#42475893204](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42475893204) audited the exact committed application and Decimal Contract POC lockfiles at source commit e04d628f01b18488c76617f00d2f03b375ac482a with checksum-pinned cargo-audit 0.22.2, --deny warnings, no ignored advisories and no target filters.

| Scope | Audit dependency count | Vulnerabilities | Warnings | Exit |
|---|---:|---:|---:|---:|
| Application workspace | 54 | 0 | 0 | 0 |
| Decimal Contract POC | 56 | 0 | 0 | 0 |

RustSec Advisory Database snapshot:
- Canonical remote: https://github.com/RustSec/advisory-db.git.
- Commit: 309392b18f6b17022f82c701f300433126901068.
- Commit timestamp: 2026-11-06T16:34:32+01:00.
- Advisory records reported: 1,422.
- Both JSON reports record vulnerabilities.found=false, vulnerabilities.count=0, and an empty warnings object.
- Effective report settings record ignore=[], target_arch=[], target_os=[], severity=null, and informational categories unmaintained, unsound, notice.

The uploaded artifact is ID 13615875196, size 18,429 bytes, archive SHA-256 9194f7e1202cd07ac4ea992720dc644c439ccf86b554fb39df07f045797c92e8. The downloaded ZIP digest was independently verified against GitHub's artifact digest. The JSON inventory reports schema 1.3.0, 107 scope records and 55 unique package identities; the two lockfile SHA-256 values match the committed baseline.

The audit runs in an isolated temporary working directory with isolated HOME and CARGO_HOME, explicitly selects Rust 1.85.0 via RUSTUP_TOOLCHAIN, pins the canonical advisory database URL, fetches the database, and does not load repository or runner-user .cargo/audit.toml settings. It checks both committed lockfiles, does not filter to the runner target, does not ignore advisories, and retains yanked checking.

This is successful, point-in-time advisory evidence for the two exact lockfiles and database commit above. It does not establish absence of every vulnerability, exploitability, reachability, application defects or system-library risks. It does not resolve the separate cargo-cyclonedx 0.5.9 / yanked xml-rs 0.8.19 preview-generator exception. Repeat the scan after dependency, manifest, lockfile or workflow changes and on the default branch after merge.

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
| Committed application lockfile and package inventory | PASS — 52 registry records; lockfile SHA-256 unchanged |
| Isolated POC lockfile and package inventory | PASS — 55 registry records; independent lockfile SHA-256 unchanged |
| Cross-scope identity reconciliation | PASS — 107 scope occurrences resolve to 55 unique (name, version, source) identities and 50 crate names; 52 shared, zero application-only, three POC-only |
| Registry source-archive integrity | PASS on PR #34 schema 1.3.0 run — 107 / 107 archive SHA-256 values match the committed lockfile checksums |
| Cargo metadata/source-manifest cross-check | PASS — 107 / 107 package records matched for identity, licence expression, licence-file and repository fields |
| Licence/notice candidate discovery | PASS as automated inventory only — all 107 scope records have at least one non-empty licence-text candidate; all 55 unique package identities still require human review |
| RustSec advisory audit | PASS on PR source e04d628 — both committed lockfiles have 0 vulnerabilities and 0 warnings against the recorded 1,422-record database snapshot; no ignores or target filters |
| Duplicate-version and source exception review | OPEN — five package-name families have multiple versions; target/feature paths must be reviewed; no Git sources were found in either current lockfile |
| Package-level source/provenance disposition | OPEN — 55 unique identities remain pending; publisher repository metadata does not authenticate repository ownership/control |
| Package-level licence, copyright, patent and notice disposition | OPEN — 55 unique identities remain pending; exact alternative terms and redistribution obligations require human determination |
| Preview-generator exception | OPEN; confined to the preview-only generator and due 2027-02-04 or earlier if upstream removes the yanked package |
| Official release SBOM/checksums and reproducible release identity | NOT DONE — requires the exact approved release source and supported target matrix |
| Release authorization | NOT AUTHORIZED |

**IB-T005 remains OPEN.** PR #34 has merged the repeatable source-inventory and fail-closed advisory controls into `main` at `7dcc2463fc8f9617576f970499c67223ee0640d9`. The exact dependency scan was executed on the PR source head (now integrated as the merge commit's second parent); it is not represented as a scan triggered on the merge commit. The weekly default-branch recheck is active, with its first scheduled run pending. Human review of the 55 unique identities, duplicate-version target/feature paths, the preview-generator exception, the eventual production dependency graph after IB-T004, official release SBOM/checksums, clean-checkout reproduction and publication authorization remain separate gates. No Forecast Engine work is authorized by this document.
