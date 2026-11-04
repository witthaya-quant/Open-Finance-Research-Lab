<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-IB-001 — Decimal POC Dependency Review

**Review version:** 0.1.0  
**Review date:** 2026-11-04  
**Scope:** Isolated Decimal and JSON/CSV contract POC only  
**Status:** POC-local lockfile committed; locked CI passed on Rust 1.85.0 and 1.99.1 at commit `35850951dd51a361baa5394a47bcc5bca00e6f75`; source-level licence/security review and release SBOM remain open

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

## 5. Resolved graph and lockfile evidence

The isolated POC now has its own committed `Cargo.lock`, generated by Cargo 1.85.0 from `poc/decimal-contract/Cargo.toml` itself. It was not copied from the parent OFRL-IB-001 workspace. The generated lockfile uses format version 4 and contains 56 package records: the local POC package plus 55 registry packages.

The bootstrap run executed:

- `cargo generate-lockfile` with Rust/Cargo 1.85.0.
- `cargo metadata --locked --format-version 1`.
- `cargo tree --locked --edges normal,build,dev`.
- The existing formatting, Clippy, test, and documentation checks.

The 55 external package records all resolve from the crates.io registry source and all expose a non-empty `license` value in their Cargo package metadata. The POC workflow now reproduces this check on both Rust toolchains: it runs `cargo metadata --locked`, counts registry packages, fails if any registry package has an empty/missing Cargo `license` field, and prints the name, version, and publisher-declared expression for review. The lockfile stores the selected versions and registry checksums; the metadata inventory includes the packages represented by Cargo's resolved metadata, not only crates compiled under the default feature set.

This is a metadata-presence control only. It does not validate the syntax or legal interpretation of every expression and does not verify the exact source archive, licence text, copyright notice, exception, or feature-specific obligations. Those remain release-time review gates.

### Licence metadata items requiring source-level confirmation

| Package(s) | Cargo metadata licence string | Review note |
|---|---|---|
| `csv 1.4.0`, `csv-core 0.1.13` | `Unlicense/MIT` | Publisher-declared slash form; verify both included licence texts and attribution/notice obligations before distribution. Do not treat this legacy string as a normalized SPDX expression. |
| `r-efi 5.3.0` | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` | Alternative licensing expression; retain the exact source licence notices and record the downstream licence choice where relevant. |
| `ryu 1.0.23` | `Apache-2.0 OR BSL-1.0` | Alternative licensing; confirm the applicable choice and terms from the source archive for a release. |
| `unicode-ident 1.0.27` | `(MIT OR Apache-2.0) AND Unicode-3.0` | Combined licence obligations; preserve the Unicode licence notice as well as the selected permissive licence. |
| `wasi`, `wasip2`, `wit-bindgen` | Apache-2.0 with LLVM exception, Apache-2.0, or MIT alternatives | Preserve the exact notices from the source packages and validate SBOM expression normalization. |
| `zerocopy`, `zerocopy-derive` | `BSD-2-Clause OR Apache-2.0 OR MIT` | Alternative licensing; source archive and notices still require release-time verification. |

All other resolved packages likewise expose licence metadata, commonly MIT OR Apache-2.0, Apache-2.0, or MIT. Metadata presence is not proof of correct legal classification: the source archives, bundled licence files, copyright notices, and any feature-specific dependency obligations must be checked for a distributable release.

### Reproducibility and compatibility

The lockfile was generated with the declared effective MSRV toolchain (Rust 1.85.0). The committed file was subsequently verified on both Rust 1.85.0 and 1.99.1 at PR #20 head `35850951dd51a361baa5394a47bcc5bca00e6f75`. On both toolchains, `cargo metadata --locked`, `cargo tree --locked`, formatting, Clippy, all 9 tests, and documentation generation passed. This closes the lockfile reproducibility acceptance criterion for the POC package; it does not constitute a source-level legal review or security audit.

The committed lockfile freezes registry package versions and checksums for the isolated POC. It does not freeze the Rust toolchain installation, operating-system image, compiler/linker, or external services; those remain separately identified in CI configuration.

Before the full model workspace is promoted beyond research use:

1. Keep the isolated POC lockfile separate from the parent application's lockfile.
2. Run all supported toolchains with locked metadata, dependency-tree, Clippy, tests, and documentation commands.
3. Review the complete source-level licence notices and security advisories before distributing a binary.
4. Generate the release SBOM from the exact source commit and lockfile, then publish matching SHA-256 checksums.
5. Re-run the test matrix after any lockfile change and review the full dependency diff.


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
| Resolved dependency metadata inventory | 55 registry packages; all declare a Cargo `license` value; no `UNDECLARED` records | Inventory and declared licence expressions reviewed; source-archive/notice verification remains a release gate |
| Locked dependency graph verification | Rust 1.85.0 and 1.99.1; metadata, tree, Clippy, 9 tests, docs | PASS on PR #20 head `35850951dd51a361baa5394a47bcc5bca00e6f75` |
| Release SBOM and source-level licence/security review | Not completed | Open |
| Full model calculation accuracy | Not tested by this POC | Not evaluated |
| XLSX export reconciliation | Not implemented | Open |
| Three-statement invariants | Not implemented by this POC | Open |

**Conclusion:** rust_decimal 1.43.0 remains a viable candidate for the next model-engine prototype under the tested string-boundary controls. The isolated POC now has a committed, separately generated lockfile and locked CI evidence, but final adoption is not approved. Approval still requires model-specific numeric range/scale analysis, calculation tests, release SBOM generation, source-level licence/notice verification, and security review.

## 7. Successful CI evidence

- Pull request: [#6 — validate Decimal contract and Rust MSRV](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/6)
- Workflow run: [OFRL-IB-001 Decimal Contract POC](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41763010734)
- Tested commit: 88e34261b6299e7bc1aadc9c567348264e037872
- Rust matrix: 1.85.0 and 1.99.1
- Per toolchain: formatting PASS, Clippy PASS, 9 tests PASS, documentation build PASS

This evidence applies to the isolated POC only. It is not a release certification for OFRL-IB-001 or the future three-statement calculation engine.

### Locked dependency verification — PR #20

- PR: [#20 — lock isolated Decimal POC dependencies](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/20)
- Tested commit: `35850951dd51a361baa5394a47bcc5bca00e6f75`
- [Repository Policy](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41800093232): PASS
- [Numeric and Input Contract Checks](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41800093248): PASS — JSON Schema plus Decimal POC on Rust 1.85.0 and 1.99.1
- [Rust Runtime Semantic Validator](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41800093217): PASS — Rust 1.85.0 and 1.99.1
- Per POC toolchain: locked metadata/tree validation, formatting, Clippy, 9 tests, and documentation build all PASS