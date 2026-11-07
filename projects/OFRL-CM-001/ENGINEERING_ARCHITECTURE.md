<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Engineering Architecture

**Architecture version:** 1.0.0  
**Project:** OFRL-CM-001  
**First admitted software-bearing case:** OFRL-CM-001-CS-002 — Finite-Binomial American Put  
**Owner:** Project Owner  
**Last reviewed:** 2026-11-07  
**Status:** Initial case-specific architecture approved; implementation validation in progress  
**Project control:** [PROJECT_CONTROL.md](PROJECT_CONTROL.md)

## 1. Purpose and boundary

This document defines the concrete software boundary for the first admitted computational case. It is not a proposal for a general-purpose financial library or solver platform.

The sole initial software artifact is one Rust library package under:

    cases/2026-11-american-put/workbench/

It evaluates a finite-horizon, recombining, no-dividend binomial American Put model using exact rational arithmetic. It returns a full node-value/policy certificate. A verifier checks the certificate against the mathematical model through Bellman inequalities and policy attainability.

The artifact is:

- Research-only and deterministic for identical validated inputs.
- Driven by caller-supplied synthetic model parameters.
- Offline at runtime; it has no network, exchange, market-data, order, account, or external-service interface.
- Not a production pricing service, investment recommendation, suitability assessment, or trading system.
- Case-specific. No generic option framework, multi-asset engine, stochastic-rate model, or reusable solver abstraction is introduced.

The blocked Pressure Points case remains separate. This architecture does not relax its formalization gate.

## 2. Logical design

The package has one mathematical domain and three logical responsibilities:

| Responsibility | Input | Output | Control |
|---|---|---|---|
| Model validation | Exact model parameters and caller resource ceiling | Validated model or typed error | Strict no-arbitrage and positivity checks; checked node-count arithmetic |
| Valuation | Validated finite binomial model | Exact node values and exercise/continue/indifferent decisions | Backward induction on the recombining lattice |
| Certificate verification | Model and returned valuation certificate | Verified or typed rejection | Independently checks node metadata, terminal payoff, Bellman dominance, and policy attainability |

These responsibilities are kept in one package because there is only one implementation boundary and no independently deployable component. The verifier is a separate module-level calculation path, not a second crate or binary. A physical split is not justified by one case.

### 2.1 Domain types

The public domain model contains:

- AmericanPutModel: initial spot, strike, up factor, down factor, per-period risk-free gross factor, and finite period count.
- Rational: an alias for arbitrary-precision BigRational.
- NodeValuation: time index, number of up moves, exact spot, immediate exercise value, optional continuation value, option value, and decision.
- NodeDecision: Exercise, Continue, Indifferent, and Terminal.
- Valuation: risk-neutral up probability, initial value, exact lattice node count, and all node records.
- ModelError and VerificationError: typed failure categories; callers must not treat an error as a value.

All monetary-like values in this model are exact rational numbers expressed in one abstract model unit. There is no currency field because the case is synthetic and no currency conversion is performed.

### 2.2 Valuation contract

For a model satisfying \(0<d<R<u\), the unique risk-neutral up probability is:

\[
q=\frac{R-d}{u-d}
\]

At node \((n,j)\), with \(j\) up moves in \(n\) periods:

\[
S_{n,j}=S_0u^jd^{n-j}
\]

The immediate put exercise value is:

\[
E_{n,j}=\max(K-S_{n,j},0)
\]

At maturity:

\[
V_{N,j}=E_{N,j}
\]

For \(n<N\), continuation value and Bellman value are:

\[
C_{n,j}=\frac{qV_{n+1,j+1}+(1-q)V_{n+1,j}}{R}
\]

\[
V_{n,j}=\max(E_{n,j},C_{n,j})
\]

The output retains the complete recombining lattice. There are \((N+1)(N+2)/2\) nodes. The implementation uses \(O(N^2)\) time and \(O(N^2)\) retained value memory because auditability of the full value/policy certificate is more important than minimizing memory for this small research engine.

### 2.3 Resource boundary

The public valuation function requires a caller-supplied maximum lattice-node count. The implementation:

1. Computes the triangular node count using checked integer arithmetic.
2. Rejects arithmetic overflow.
3. Rejects a request exceeding the caller's limit before allocating the lattice.
4. Uses fallible vector reservation and returns a typed allocation error when reservation fails.

No arbitrary maximum period count is presented as a mathematical property. Resource ceilings are explicit call inputs and must be selected by the caller for the environment in which the calculation is run.

## 3. Independent optimality certificate

The verifier does not call the valuation function and does not trust its internal search or iteration state.

For every non-terminal node, it recomputes spot and exercise value from the input model, then recomputes continuation from the supplied child values. It requires:

\[
V_{n,j}\ge E_{n,j},\qquad V_{n,j}\ge C_{n,j}
\]

It also requires the recorded policy action to attain the candidate value:

- Exercise: \(V_{n,j}=E_{n,j}\).
- Continue: \(V_{n,j}=C_{n,j}\).
- Indifferent: \(V_{n,j}=E_{n,j}=C_{n,j}\).

At maturity it requires \(V_{N,j}=E_{N,j}\) and no continuation value.

These checks establish that the discounted candidate value dominates every exercise payoff and is a supermartingale under the risk-neutral measure. Any admissible stopping time therefore has expected discounted payoff no greater than the candidate root value. The recorded policy attains equality at each decision node, so it achieves that value. The pair of conditions supplies a finite-horizon optimal-stopping certificate for the specified model.

The verifier also checks:

- The expected number and ordering of nodes.
- Every \((n,j)\) coordinate.
- Exact spot values and immediate exercise values.
- The risk-neutral probability.
- The root value against the root node.
- Absence of terminal continuation values and presence of non-terminal continuation values.

This verifies optimality for the stated model. It does not establish that the model is a good description of a real market or that its assumptions are economically appropriate.

## 4. Numeric policy

The calculation core uses:

- num-rational BigRational for exact fractions.
- num-bigint BigInt for arbitrary-precision integer numerators and denominators.
- Exact rational inputs and exact comparisons throughout valuation and verification.
- No f32, f64, decimal truncation, epsilon comparison, implicit rounding, or conversion of an input through binary floating point.

The runtime dependency versions are pinned exactly in Cargo.toml, and the resolved graph is recorded in the package-local Cargo.lock. Only the dependency features needed for arbitrary-precision rational arithmetic are enabled.

The library returns exact rational values. Human-readable decimal formatting is a presentation concern and is deliberately outside the mathematical core in this initial version.

## 5. Package and toolchain

| Item | Decision |
|---|---|
| Package | One library package: american-put-workbench |
| Location | cases/2026-11-american-put/workbench/ |
| Rust edition | 2021 |
| Minimum supported Rust | 1.85.0 |
| Compatibility toolchain | 1.99.1 |
| Runtime dependencies | Exact-pinned num-bigint and num-rational only |
| Lockfile | Package-local committed Cargo.lock; all CI resolution is locked |
| Interface | Rust library API only; no CLI in the initial case |
| Serialization | Not selected; no persistence or wire contract is required |
| Parallelism | Not selected; the model is a small finite lattice |
| Workspace | Not used; one package does not require a workspace |

The chosen library dependencies declare permissive MIT OR Apache-2.0 licensing. The exact resolved versions, checksums, and transitive dependency metadata are preserved in the lockfile and reviewed as part of the implementation PR. The lockfile does not replace source-level licence, security-advisory, or maintenance review.

## 6. Test and verification strategy

The first package includes:

- Exact reference-case assertions for the risk-neutral probability, initial option value, and selected exercise decisions.
- Valid and invalid input tests, including no-arbitrage boundary rejection.
- Zero-period boundary coverage.
- Checked triangular node-count and resource-limit tests.
- Positive certificate verification.
- Negative certificate tests that alter a node value or metadata.
- Exhaustive enumeration of all stopping-time policies on a reduced three-period history tree.
- Exhaustive enumeration of state-based Markov node policies on the same recombining lattice.
- Comparison of both exhaustive maxima to the exact dynamic-programming value.

For \(N=3\), the full history tree has seven non-terminal decision histories and therefore \(2^7=128\) history-dependent stopping policies over eight terminal paths. The recombining lattice has six non-terminal state nodes and therefore \(2^6=64\) state-based policies. Their policy counts differ because recombination merges histories; both exhaustive maxima must equal the same exact Bellman value. Policy non-uniqueness must not be confused with value non-uniqueness.

Required project-local CI runs formatting, Clippy with warnings denied, tests, and rustdoc on Rust 1.85.0 and 1.99.1 using the committed lockfile.

## 7. Workflow and supply-chain controls

The only new software workflow is:

    .github/workflows/ofrl-cm-001-workbench.yml

It is scoped to CM-001 paths and its own workflow file, uses read-only repository permissions, checks out with a full-length verified action SHA, disables persisted checkout credentials, installs explicitly selected Rust toolchains, and uses locked Cargo commands.

The new package path is added to .github/dependabot.yml as one additional Cargo update target, in accordance with the repository dependency policy. No existing OFRL-IB-001 workflow, Cargo manifest, lockfile, source file, schema, fixture, or dependency entry is reused or edited.

The current artifact is not a distributable release. SBOM generation, release checksums, release checks, residual-risk assessment, and any independent assurance required by a future risk/use change remain release gates rather than claims made by this prototype.

## 8. Risks and deliberate non-goals

| Risk / limitation | Treatment |
|---|---|
| Parameter domain may not represent actual market dynamics | Explicit synthetic assumptions; no market calibration |
| No-dividend assumption affects early-exercise behavior | State the limitation; dividends require a separate model extension and formalization |
| Constant u,d,R across periods | Restrict the initial model to time-homogeneous parameters |
| Arbitrary-precision arithmetic can consume resources | Caller-supplied node ceiling and fallible allocation |
| One implementation can contain correlated mistakes | Independent Bellman verifier and exhaustive reduced-instance policies |
| Exactness may be mistaken for economic accuracy | Report exact mathematics only for the defined model; do not claim market validity |

The initial architecture excludes dividends, American calls, Bermudan schedules, path-dependent payoffs, stochastic interest rates, multiple underlyings, volatility calibration, market-data ingestion, Monte Carlo, parallel execution, command-line interfaces, Excel output, performance benchmarking, and live trading integration.

Any extension requires a case-specific mathematical specification, impact analysis, validation additions, and recorded decision before implementation.
