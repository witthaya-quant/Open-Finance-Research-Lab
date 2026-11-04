<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Decimal POC Dependency Review

**Review version:** 0.1.0  
**Review date:** 2026-11-04  
**Scope:** Isolated Decimal and JSON/CSV contract POC only  
**Status:** Initial metadata review completed; full dependency and security audit remains open

## 1. Executive finding

The isolated POC passed its current formatting, Clippy, unit-test, and documentation checks on Rust 1.85.0 and Rust 1.99.1.

This establishes that the selected direct dependency set can compile and pass the defined POC tests on those two toolchains. It does **not** establish that every transitive dependency has been exhaustively reviewed, that the dependency graph is immutable, or that rust_decimal is approved for the full financial calculation engine.

The fixed-point candidate remains conditional. It must be evaluated against the eventual model's maximum monetary size, scale requirements, formulas, and rounding policy.

## 2. Direct dependencies observed in CI

Versions below are the versions resolved by the successful GitHub Actions run for PR #6 on 2026-11-04. Package metadata is taken from the published crate manifests and documentation linked below.

| Crate | Resolved version | Declared licence | Declared MSRV | POC role |
|---|---:|---|---:|---|
| rust_decimal | 1.43.0 | MIT | 1.67.1 | Fixed-point Decimal candidate |
| serde | 1.0.229 | MIT OR Apache-2.0 | 1.56 | Typed serialization/deserialization |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | 1.71 | JSON interchange |
| csv | 1.4.0 | Unlicense/MIT | 1.73 | CSV interchange |

The serde derive feature also resolves serde_derive 1.0.229 in the dependency graph. Its published manifest declares MIT OR Apache-2.0 and Rust 1.71.

### Primary package metadata

- [rust_decimal 1.43.0 manifest](https://docs.rs/crate/rust_decimal/1.43.0/source/Cargo.toml)
- [serde 1.0.229 manifest](https://docs.rs/crate/serde/1.0.229/source/Cargo.toml)
- [serde_derive 1.0.229 manifest](https://docs.rs/crate/serde_derive/1.0.229/source/Cargo.toml)
- [serde_json 1.0.154 documentation and release metadata](https://docs.rs/crate/serde_json/1.0.154)
- [serde_json 1.0.154 manifest metadata](https://chromium.googlesource.com/external/github.com/serde-rs/json/%2B/refs/heads/upstream/master/Cargo.toml)
- [csv 1.4.0 manifest](https://docs.rs/crate/csv/1.4.0/source/Cargo.toml)

These licence identifiers are the package metadata declared by their publishers. They are not a substitute for reviewing the exact source archive, licence texts, notices, feature set, and transitive dependencies of a release build.

## 3. MSRV interpretation

The direct package MSRVs are lower than the POC's tested minimum in some cases. The package declares Rust 1.85 because the observed dependency-resolution process on Cargo 1.73 attempted to parse an optional/transitive manifest using the Rust 2024 edition. Cargo 1.73 failed before compilation because it does not understand the edition = 2024 manifest value.

The current POC CI matrix therefore verifies:

- Rust 1.85.0 — effective minimum supported toolchain for this resolved dependency graph.
- Rust 1.99.1 — newer toolchain compatibility check.

Both matrix entries passed formatting, Clippy, tests, and documentation generation in the successful run recorded below.

The 1.85 declaration is a property of this POC's resolved toolchain/dependency environment. It must be re-evaluated if dependency versions, optional features, or the workspace graph changes.

## 4. Numeric dependency assessment

rust_decimal represents a Decimal with a 96-bit integer mantissa, a sign, and a scale in the range 0–28. It preserves representable trailing zeroes in string form. It is finite fixed-point arithmetic; it is neither arbitrary precision nor exact rational arithmetic.

The POC exposed an input-boundary issue: the raw parser accepted a 29-fractional-digit string and reduced it to the supported scale. The POC adapter now rejects scale above Decimal::MAX_SCALE before parsing and requires the parsed value's string form to match the original input exactly.

This guard addresses the demonstrated parsing hazard at the JSON/CSV boundary. It does not remove the fixed precision limits of arithmetic operations, nor does it define the final reporting-rounding policy for the full financial model.

Observed arithmetic outputs at scale 28:

| Expression | Observed output | Interpretation |
|---|---|---|
| 1 / 365 | 0.0027397260273972602739726027 | Finite approximation at scale 28 |
| 1 / 3 | 0.3333333333333333333333333333 | Repeating quotient is truncated to finite Decimal scale |
| (1 / 3) × 3 | 0.9999999999999999999999999999 | Does not reconstruct exact 1; arithmetic is not rational-exact |

This behaviour is acceptable only if the full model defines and tests calculation precision, error accumulation, materiality, and rounding at the reporting boundary. Those are still open model-engineering requirements.

## 5. Dependency graph evidence and lockfile status

The successful CI build logs show the following additional packages being compiled in the POC build, among others:

- serde_core 1.0.229
- serde_derive 1.0.229
- csv-core 0.1.13
- num-traits 0.2.19
- arrayvec 0.7.8
- memchr 2.8.3
- itoa 1.0.18
- zmij 1.0.23
- ryu 1.0.23

This is an observed subset of the build log, not a complete Software Bill of Materials (SBOM) or exhaustive licence inventory.

**No Cargo.lock is committed in this isolated POC at this stage.** Direct dependency versions are now pinned in Cargo.toml to the tested releases, including rust_decimal 1.43.0, csv 1.4.0, serde 1.0.229, and serde_json 1.0.154. Transitive dependencies still resolve from compatible version ranges and may change on a future resolution. The versions above describe the tested CI resolution on 2026-11-04, not a permanently frozen graph.

Before the full model workspace is promoted beyond research use:

1. Decide the workspace-level lockfile policy and commit the generated Cargo.lock for the application workspace.
2. Generate and review the complete dependency tree and machine-readable SBOM.
3. Review every direct and transitive package's licence, notice obligations, maintenance status, and security advisories.
4. Record the exact Rust toolchain and dependency graph used for each release.
5. Re-run the test matrix after dependency updates.

## 6. Current adoption decision

| Area | POC result | Decision |
|---|---|---|
| Decimal string JSON serialization | Passed | Accepted for the POC contract |
| JSON numeric-token rejection | Passed | Accepted for the POC contract |
| CSV Decimal string round-trip | Passed | Accepted for the POC contract |
| Scale and exact input round-trip guard | Passed, including regression case | Required input-boundary control |
| Checked overflow / division-by-zero tests | Passed for tested boundaries | Keep checked arithmetic as mandatory policy |
| Repeating division | Behaviour observed and documented | Must be incorporated into model precision policy |
| Rust 1.85.0 CI | Passed | Effective POC MSRV |
| Rust 1.99.1 CI | Passed | Current-toolchain compatibility check |
| Complete transitive licence/security audit | Not completed | Open |
| Full model calculation accuracy | Not tested by this POC | Not evaluated |
| XLSX export reconciliation | Not implemented | Open |
| Three-statement invariants | Not implemented by this POC | Open |

**Conclusion:** rust_decimal 1.43.0 is a viable candidate for the next model-engine prototype under the tested string-boundary controls. It is not yet approved as the final financial-engine numeric foundation. That approval requires a model-specific numeric range/scale analysis, calculation tests, a lockfile/SBOM, and dependency/security review.

## 7. Successful CI evidence

- Pull request: [#6 — validate Decimal contract and Rust MSRV](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/6)
- Workflow run: [OFRL-IB-001 Decimal Contract POC](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41763010734)
- Tested commit: 88e34261b6299e7bc1aadc9c567348264e037872
- Rust matrix: 1.85.0 and 1.99.1
- Per toolchain: formatting PASS, Clippy PASS, 9 tests PASS, documentation build PASS

This evidence applies to the isolated POC only. It is not a release certification for OFRL-IB-001 or the future three-statement calculation engine.