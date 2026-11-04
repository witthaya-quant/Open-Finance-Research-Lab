<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Dependency Audit Results

**Evidence date:** 2026-11-04  
**Workflow:** [Dependency SBOM run #41782449058](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41782449058)  
**Pull request:** [PR #11](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/11)  
**PR head:** `af4c0d84bf8f98cd76c021fdd7edcdb3ef15f79d`  
**Artifact source commit:** `1fd6de84d7a0d8034b189fce0f84262dcc51d947` (GitHub pull-request merge ref used by the workflow)  
**Artifact ID:** `13299916484`  
**Artifact SHA-256:** `9ce1f660e51da303637e0e70818cb19eb2cd24e52775d00e6f5e6e9b0adc54b3`  
**Result:** Inventory and advisory scan PASS; licence/source/legal review remains OPEN

## 1. Executive result

The pinned CI workflow generated four CycloneDX 1.5 JSON SBOM files and ran RustSec `cargo-audit 0.22.2` against three lockfile snapshots.

| Dependency scope | Audit dependency count | SBOM components (excluding root component) | Vulnerabilities | Warnings | Exit |
|---|---:|---:|---:|---:|---:|
| Production `ofrl_model` / workspace | 54 | 52 | 0 | 0 | 0 |
| Production `ofrl_cli` | Included in workspace graph | 53 | — | — | — |
| Numeric Envelope POC | 58 | 57 | 0 | 0 | 0 |
| Decimal Contract POC | 56 | 55 | 0 | 0 | 0 |

The production audit is against the production workspace lockfile once, not two independent audit runs for each workspace member. Two production SBOMs are emitted because the workspace contains two first-party packages.

All three JSON reports state `vulnerabilities.found=false`, `vulnerabilities.count=0`, and an empty `warnings` object. All three audit commands returned exit code 0. No advisory IDs were ignored.

## 2. Toolchain and advisory database

- CycloneDX generator: `cargo-cyclonedx 0.5.9`, official x86_64 Linux release binary, SHA-256 pinned in workflow.
- Advisory scanner: `cargo-audit 0.22.2`, official x86_64 Linux release binary, SHA-256 pinned in workflow.
- CycloneDX specification: 1.5.
- Rust toolchain for metadata/tool execution: 1.99.1; the application MSRV is unchanged.
- Cargo features and target selection: all features, all targets.
- RustSec Advisory Database commit: `965bf9ad083b58e6340d65829d85217b18be9290`.
- Advisory database timestamp: `2026-11-03T21:34:28+01:00`.
- Advisory records in database at scan time: 1,403.
- Audit options: `cargo audit --file <lockfile> --deny warnings --json`; ignore list was empty.

In cargo-audit 0.22.2, `--deny warnings` includes the informational categories Unmaintained, Unsound and Notice. The report's empty `warnings` object and exit code 0 show no matching warning records in these lockfile snapshots at the recorded database revision.

This is a point-in-time RustSec advisory result. It is not proof of absence of vulnerabilities, does not assess application reachability or exploitability, and does not cover non-Rust/system libraries or defects not represented in the advisory database.

## 3. SBOM files generated

- `production-ofrl_model.cdx.json`
- `production-ofrl_cli.cdx.json`
- `poc-model-numeric-envelope.cdx.json`
- `poc-decimal-contract.cdx.json`

Every file parsed as JSON, declared `bomFormat=CycloneDX`, used `specVersion=1.5`, and contained a components array. The four SBOMs preserve package references and dependency relationships, and include available package licence expressions. They are separate by workspace/package purpose rather than being flattened into one ambiguous dependency graph.

The artifact contains:
- All four SBOM JSON files.
- Three per-workspace cargo-audit JSON reports.
- `audit-exit-codes.txt`.
- `sbom-metadata.json`.
- `SHA256SUMS`.
- `decimal-contract.Cargo.lock.ephemeral`.

The latest downloaded artifact (`ofrl-ib-001-sbom-1fd6de84d7a0d8034b189fce0f84262dcc51d947`, 39,787 bytes) had SHA-256 `9ce1f660e51da303637e0e70818cb19eb2cd24e52775d00e6f5e6e9b0adc54b3`, matching the GitHub artifact digest. All 10 entries in `SHA256SUMS` were re-computed locally and matched. The archive contains 11 files total including `SHA256SUMS` itself.

## 4. Lockfile reproducibility

| Scope | Evidence | Interpretation |
|---|---|---|
| Production workspace | Committed `projects/OFRL-IB-001/Cargo.lock`; checksum unchanged during SBOM generation | Locked dependency graph |
| Numeric Envelope POC | Committed POC `Cargo.lock`; checksum unchanged during SBOM generation | Locked dependency graph |
| Decimal Contract POC | No committed `Cargo.lock`; ephemeral lock snapshot included in the artifact and deleted from runner checkout | Discovery only; not reproducible |

The Decimal Contract POC must either receive a reviewed, committed lockfile or be formally retired before it can be treated as a reproducible project input or promoted toward production. The successful advisory scan applies only to the ephemeral resolution captured in this artifact.

## 5. License inventory and open decisions

The four SBOMs recorded these SPDX expressions:

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

All component entries had licence metadata in this artifact. Alternative expressions are preserved as declared; an `OR` expression does not by itself mean that every alternative is simultaneously imposed.

**No legal acceptance is implied.** The project still needs an explicit policy decision on accepted licence alternatives, review of package licence texts and notices, and distribution/source obligations. SBOM metadata can be incomplete or inaccurate and must not be treated as legal advice.

## 6. Remaining gates

The following remain OPEN:

1. Review and commit the Decimal Contract POC lockfile or formally retire the POC.
2. Review package source provenance, duplicate versions and unexpected registry/git sources.
3. Approve a licence policy and disposition all expressions/notice obligations.
4. Decide whether maintenance/unsound advisories require additional policy beyond the current RustSec warning gate.
5. Retain a repeatable scan cadence and pin/update tool binaries through an explicit change review.
6. Complete a broader security review appropriate to release artifacts and build provenance.

**Production Forecast Engine remains gated.** These results do not approve the numeric backend, precision/error budget, financial formulas, reconciliation tolerance, or XLSX reporting policy.
