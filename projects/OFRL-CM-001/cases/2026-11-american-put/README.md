<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Case Study — Finite-Binomial American Put: Optimal Stopping

**Case ID:** OFRL-CM-001-CS-002  
**Parent project:** OFRL-CM-001  
**Case type:** Quantitative mathematical research with a case-specific Rust research prototype  
**Admission date:** 2026-11-07  
**Case status:** ADMITTED — case-specific implementation merged to `main`; research-only and unreleased  
**Evidence maturity:** 2 — Reproducible (synthetic reference result only; not independently replicated or market-validated)  
**Primary academic reference:** Cox, Ross and Rubinstein (1979)

## 1. Research objective

Specify, compute and independently verify the value and an optimal stopping policy for a finite-horizon American put in a recombining binomial model.

The case is intended to exercise:

- Mathematical model construction and explicit no-arbitrage assumptions.
- Risk-neutral valuation and discrete-time dynamic programming.
- Optimal stopping and the distinction between immediate exercise and continuation.
- Exact rational computation and numerical integrity.
- An independently checked optimality certificate.
- Exhaustive reduced-instance verification of stopping policies.

This is the first computational case for OFRL-CM-001. It is not a live financial model or a production pricing engine.

## 2. Scope and boundaries

### Included

- One underlying asset with a finite, discrete, recombining up/down price tree.
- One American put with a fixed strike and exercise dates at every model time from 0 through maturity.
- A constant per-period risk-free gross return and constant up/down factors.
- No dividends or other distributions.
- Exact arbitrary-precision rational inputs, intermediate values and outputs.
- Full lattice node values and exercise/continue/indifferent labels.
- A verifier for Bellman optimality and policy attainability.
- Exhaustive enumeration on the three-period synthetic reference case.
- A small Rust library package with case-specific tests and CI.

### Excluded

- Market-data downloads, calibration, historical or implied volatility estimation.
- Dividends, borrow costs, funding spreads, transaction costs, taxes or market impact.
- Bermudan exercise schedules, American calls, multiple underlyings, stochastic rates, stochastic volatility, jumps or path-dependent payoffs.
- Monte Carlo, comparative performance benchmarking, parallel computation or generalized option abstractions.
- CLI, API, database, workbook, hosted service, investment recommendation, suitability analysis or trade execution.
- Any claim that exact model arithmetic establishes economic accuracy or market-model validity.

The only input dataset is an OFRL-authored synthetic parameter tuple. No market observation or third-party numerical dataset is stored.

## 3. Primary sources and academic context

1. John C. Cox, Stephen A. Ross and Mark Rubinstein, “Option Pricing: A Simplified Approach,” Journal of Financial Economics, 7(3), 229–263 (September 1979), DOI [10.1016/0304-405X(79)90015-1](https://doi.org/10.1016/0304-405X(79)90015-1). This is the primary academic reference for the discrete-time binomial option-pricing framework and its treatment of American options with possible early exercise.
2. Leonid Kogan, MIT OpenCourseWare 15.450, “Introduction to Dynamic Programming,” Fall 2010, [lecture PDF](https://ocw.mit.edu/courses/15-450-analytics-of-finance-fall-2010/434ff3cbe0f9753416e4979d9ef1e34a_MIT15_450F10_lec05.pdf). This provides academic context for adapted stopping decisions and the Bellman recursion.

OFRL cites these sources but does not copy article or lecture text, figures, tables, slides, or source code. Bibliographic records and rights notes are in DATA_MANIFEST.md, SRC-068–SRC-069.

## 4. Method

The case uses backward induction on a recombining binomial state lattice. This method is justified because:

- The model has a finite horizon and a finite state set at every time.
- The stock price is Markov with respect to the lattice state.
- The exercise decision is made using information available at the current time.
- The payoff and risk-neutral one-step transition law are explicit.
- The dynamic-programming recursion is exact over rational inputs.

The implementation returns the complete lattice and a policy certificate. An independent verifier checks the certificate against the model rather than calling the valuation function. Reduced instances are separately cross-checked by exhaustive stopping-policy enumeration.

## 5. Formal model and reference result

The full mathematical specification is in [PROBLEM_FORMALIZATION.md](PROBLEM_FORMALIZATION.md). The synthetic reference parameters are:

\[
S_0=100,\quad K=100,\quad u=\frac65,\quad d=\frac45,\quad R=\frac{21}{20},\quad N=3
\]

The risk-neutral probability is:

\[
q=\frac{R-d}{u-d}=\frac58
\]

The exact time-zero American put value is:

\[
V_{0,0}=\frac{18325}{2058}\approx 8.9042759961
\]

The lattice exercise region is state-dependent. In this reference case, immediate exercise is optimal at nodes \((1,0)\) and \((2,0)\); continuation is optimal at \((0,0)\), \((1,1)\) and \((2,1)\); node \((2,2)\) is indifferent; terminal nodes take the payoff.

The exact value is unique for this model. The stopping policy need not be unique: a tie or an unreachable history may admit more than one maximizing policy. The formalization distinguishes full path histories from recombining Markov states when it reports exhaustive policy counts.

## 6. Reproduction

The Rust library and tests are located under the workbench subdirectory. From the repository root:

    cargo fmt --manifest-path projects/OFRL-CM-001/cases/2026-11-american-put/workbench/Cargo.toml -- --check
    cargo clippy --manifest-path projects/OFRL-CM-001/cases/2026-11-american-put/workbench/Cargo.toml --locked --all-targets -- -D warnings
    cargo test --manifest-path projects/OFRL-CM-001/cases/2026-11-american-put/workbench/Cargo.toml --locked --all-targets
    RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path projects/OFRL-CM-001/cases/2026-11-american-put/workbench/Cargo.toml --locked --no-deps

The exact compiler versions and CI results are recorded in the case PR and project Validation Matrix. The package has its own Cargo.lock; it is not part of a workspace and does not share dependencies or code with OFRL-IB-001.

## 7. Rights, limitations and intended use

The model, synthetic inputs, OFRL-authored code and explanations are original case material under the applicable OFRL content-type licensing policy. Academic references remain third-party sources under their respective rights and terms.

This case is an educational and analytical research artifact. It is not calibrated to a market, does not include real contract specifications, and must not be used to price or trade an actual instrument. Any future extension or public reusable pricing release requires a new mathematical and risk assessment, dependency review, independent review where applicable, and separate release authorization.

## 8. Current disposition

**Evidence-maturity boundary:** Level 2 applies only to reproduction of the specified synthetic model result through the documented Rust package, committed lockfile, exact-result tests, verifier and post-sync CI. It does not establish robustness across a broader parameter/model domain, independent external replication, empirical performance, market calibration, economic usefulness or production readiness. The parent project remains at its own separately recorded maturity level.


**Research status:** Formal model admitted.  
**Implementation status:** The case-specific library, exact-rational tests, independent Bellman verifier and exhaustive policy cross-check passed all 9 post-sync checks on Rust 1.85.0 and 1.99.1 (reviewed branch head `38c4da0cb175ed39d46ad0ef11616aba4ee6a3c9`). PR #37 was merged to `main` as `9004693006b425956364461de529b9a6a43da4a8` on 2026-11-09. See [Workbench CI](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42554995949), [Repository Policy](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42554995969), [Numeric/Input Contract](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42554995917), and [Semantic Validator](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/42554995925).  
**Release status:** Not released.  
**Financial application:** No production or trading application is claimed.
