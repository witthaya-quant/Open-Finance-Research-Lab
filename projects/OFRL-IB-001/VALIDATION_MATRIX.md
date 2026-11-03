# OFRL-IB-001 — Validation Matrix

**Version:** 0.1.0  
**Status:** Proposed — acceptance criteria must be approved before implementation  
**Purpose:** Define objective PASS/FAIL gates before any model results are reviewed.

## 1. Result taxonomy

The project reports three independent result groups:

1. **Structural Integrity** — statement identities, schedule roll-forwards, cross-statement links, and input validity.
2. **Scenario Feasibility** — whether the modelled scenario can fund its cash requirements under the stated financing assumptions.
3. **Research Release Readiness** — documentation, reproducibility, data rights, and review requirements.

A PASS in one group does not imply a PASS in another. In particular, a mathematically balanced model may still have negative cash, while a cash-positive model may still contain broken statement links.

## 2. Numerical tolerance

For monetary reconciliation, define:

Tolerance = 1e-6 + 1e-8 × max(|A|, |B|, 1)

where A and B are the two monetary values being reconciled, expressed in the model's unrounded internal units.

A reconciliation passes when:

|A − B| ≤ Tolerance

The tolerance is applied to unrounded values. Display-rounded values must never be used to conceal a failed reconciliation.

For ratios and percentages, tests should use a separately documented absolute and relative tolerance appropriate to the calculation. Exact integer/date/string metadata checks should not use floating-point tolerance.

## 3. Structural integrity gates

| ID | Test | PASS criterion | FAIL condition |
|---|---|---|---|
| SI-01 | Historical Balance Sheet | Total assets reconcile to total liabilities plus equity for every historical period within monetary tolerance | Any historical period fails the accounting identity |
| SI-02 | Forecast Balance Sheet | Total assets reconcile to total liabilities plus equity for every forecast period within monetary tolerance | Any forecast period fails the accounting identity |
| SI-03 | Cash roll-forward | Opening cash + net change in cash = closing cash for every period | Cash movement does not reconcile |
| SI-04 | Cash cross-statement | Cash Flow Statement closing cash equals Balance Sheet cash for each period | Closing cash differs between statements |
| SI-05 | Net income link | Income Statement net income agrees with Cash Flow Statement starting net income | Cross-statement net income mismatch |
| SI-06 | Working-capital cash link | Cash-flow working-capital effect equals negative change in operating NWC | Sign or amount differs from the defined convention |
| SI-07 | PP&E roll-forward | Gross PP&E, accumulated depreciation, and net PP&E schedules reconcile | Any PP&E schedule fails |
| SI-08 | PP&E bounds | Accumulated depreciation does not exceed gross PP&E; net PP&E is not negative beyond tolerance | Depreciation produces an invalid asset balance |
| SI-09 | Debt roll-forward | Opening debt + drawdowns − repayments = closing debt | Debt schedule does not reconcile |
| SI-10 | Debt bounds | Debt balances and repayments are non-negative; repayment does not exceed available debt | Negative debt or excess repayment |
| SI-11 | Retained earnings | Opening retained earnings + net income − dividends = closing retained earnings | Equity roll-forward fails |
| SI-12 | Dividend bounds | Payout ratio is within 0%–100%; baseline dividends do not exceed positive net income | Invalid payout assumption or distribution formula |
| SI-13 | Input integrity | Required values are present, finite, correctly mapped, and expressed in consistent units | Missing, non-finite, malformed, or mixed-unit inputs |
| SI-14 | Domain integrity | Restricted inputs remain within the domains defined in MODEL_CONVENTIONS.md | Invalid tax rate, day metric, or prohibited negative balance |
| SI-15 | Formula coverage | All forecast periods contain formulas or explicit assumptions as defined; no unexplained blanks | Broken formula chain or missing forecast period |
| SI-16 | No balancing plug | No unexplained cash plug, other-account plug, or balance-sheet adjustment is used to force balance | A plug hides an inconsistency |
| SI-17 | No circular dependency | The baseline dependency graph has no circular references | Circular reference exists |
| SI-18 | Determinism | Identical inputs and model version produce identical unrounded outputs within the numerical tolerance | Output changes without a documented input or version change |

All SI tests are hard gates for Structural Integrity PASS.

## 4. Scenario feasibility gates

| ID | Test | PASS criterion | FAIL condition |
|---|---|---|---|
| SF-01 | Cash funding | Closing cash is non-negative in every forecast period, given the explicit debt and equity assumptions | Any forecast closing cash is negative |
| SF-02 | Funding transparency | No hidden financing plug or unapproved automatic borrowing is used | Model silently funds a deficit |
| SF-03 | Debt capacity under scope | Forecast debt drawdowns and repayments follow the explicit assumptions and debt bounds | Debt schedule violates its permitted mechanics |

A negative closing cash balance is not automatically a structural accounting failure. It is a **Scenario Feasibility FAIL** because the model does not include an approved funding source for that deficit.

The model must display both result groups clearly rather than collapsing them into one overall PASS indicator.

## 5. Boundary and negative tests

The implementation must include deterministic test cases for at least:

- Zero revenue with zero revenue-driven working capital.
- Negative revenue growth while revenue remains non-negative.
- COGS greater than revenue and negative EBITDA.
- Negative EBIT and negative EBT.
- A loss-making period with zero baseline tax expense.
- Zero debt with zero interest expense.
- Zero CapEx with the PP&E schedule still reconciling.
- Debt drawdown followed by principal repayment.
- A repayment equal to all available debt.
- An attempted repayment greater than available debt, which must be rejected.
- DSO, DIO, or DPO equal to zero.
- A zero denominator in historical ratio calculations, which must produce N/A rather than infinity.
- Dividend payout ratios at 0% and 100%.
- Negative cash, which must trigger Scenario Feasibility FAIL without inserting a plug.
- Missing, NaN, or infinite assumptions, which must be rejected.

The tests should also confirm that valid negative operating results are not incorrectly rejected merely because they are economically unfavourable.

## 6. Cross-statement and independent verification

Where the implementation provides both a spreadsheet model and an independent calculation/validation implementation:

- Recalculate the workbook using a documented, supported spreadsheet calculation engine.
- Compare key outputs period by period.
- Compare all three statements and supporting schedules, not only net income or closing cash.
- Record the engine, version, calculation settings, and any unsupported spreadsheet functions.
- Fail the cross-check if any material value exceeds the monetary tolerance.

A workbook that has not been recalculated by a supported engine must not be described as formula-validated merely because the file opens.

## 7. Sensitivity and scenario checks

At minimum, the validation report should show the effect of changing:

- Revenue growth.
- COGS percentage.
- DSO / DIO / DPO.
- CapEx percentage.
- Interest rate.
- Debt drawdown and repayment assumptions.

Sensitivity analysis is intended to establish model behaviour and identify funding pressure. It is not evidence that the assumptions are probable or that the forecast is accurate.

## 8. Research release gates

| ID | Gate | PASS criterion |
|---|---|---|
| RR-01 | Specification | Scope, conventions, and validation criteria have been approved and versioned |
| RR-02 | Synthetic case | Synthetic data is labelled, deterministic, and documented |
| RR-03 | Data rights | No restricted data or unlicensed third-party content is redistributed |
| RR-04 | Assumption register | All material drivers, units, sources, and rationales are recorded |
| RR-05 | Reproduction | A reviewer can follow the documented setup and reproduce reported outputs |
| RR-06 | Test evidence | Test results are recorded with pass/fail status and version reference |
| RR-07 | Limitations | Exclusions and known weaknesses are clearly stated |
| RR-08 | Review | A review has checked formula logic, accounting links, and presentation claims |
| RR-09 | Rust toolchain | Formatting, Clippy, workspace tests, and documentation build pass in the recorded toolchain | 
| RR-10 | Decimal fidelity | Canonical JSON/CSV Decimal strings round-trip without precision loss; checked arithmetic errors are surfaced | 
| RR-11 | Excel export boundary | Workbook values follow the documented display precision; full-precision canonical outputs are preserved separately | 
| RR-12 | Rust-first architecture | Financial calculations and authoritative validation are implemented in Rust; no Python dependency is required for v1 |

A failed release gate blocks a Reproducible status claim. It must be recorded, fixed, or explicitly scoped out before release.

## 9. Rust-specific engineering gates

The following commands are proposed baseline checks once the Cargo workspace exists:

~~~bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo doc --workspace --no-deps
~~~

The implementation must also demonstrate:
- No binary floating-point arithmetic in authoritative monetary calculations.
- Checked Decimal arithmetic for operations that may overflow or divide by zero.
- Decimal-string serialization and deserialization round-trip.
- Deterministic output for identical inputs and model version.
- Property-based tests for the accounting and schedule invariants.
- Excel export precision explicitly documented and reconciled to canonical output.

A passing compiler or test command does not, by itself, prove accounting integrity.

## 10. Overall status rules

- **Structural Integrity PASS:** every SI hard gate passes.
- **Scenario Feasibility PASS:** every SF hard gate passes.
- **Research Release PASS:** every RR gate passes.
- **Reproducible / Evidence Level 2:** structural integrity, release readiness, and reproducibility criteria pass. Scenario failures must remain disclosed and cannot be represented as viable forecasts.
- **Robustness Validated / Evidence Level 3:** additional defined sensitivity, scenario, and failure-mode evidence is completed and independently reviewed.
- **Independently Replicated / Evidence Level 4:** an independent reviewer reproduces the principal results.

No failed test may be re-labelled as passed by changing its threshold after seeing the result. Any change to a criterion requires a documented rationale, a versioned specification update, and rerunning the affected tests.