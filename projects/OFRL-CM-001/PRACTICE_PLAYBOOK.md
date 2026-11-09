<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Quantitative Problem-Solving Practice Playbook

**Document version:** 0.2.0  
**Project:** OFRL-CM-001  
**Status:** Initial programme design updated from the first admitted computational case  
**Companion control:** [PROJECT_CONTROL.md](PROJECT_CONTROL.md)  
**Owner:** Kenny (Witthaya Maidee)  
**Last reviewed:** 2026-11-07

## 1. Purpose

This Playbook defines a repeatable way to practise the work of a quantitative researcher: understanding an unfamiliar problem, producing and challenging hypotheses, formalizing the mathematical structure, selecting a defensible method, computing where useful, and independently verifying the result.

It is a method for thinking and research, not a collection of answer recipes. A learner must be able to explain why a model and method apply, what would falsify the hypothesis, and what the result does not establish.

The project-level task register and delivery status live only in [PROJECT_CONTROL.md](PROJECT_CONTROL.md). This document owns the practice methodology and learning standards, not project scheduling.

## 2. Operating principles

1. **Source before interpretation.** Preserve original wording, provenance, version, and rights boundary. Separate visible facts from inferred meaning.
2. **Multiple hypotheses before commitment.** Generate plausible alternatives and record what evidence would distinguish them.
3. **Formalize before coding.** Define the domain, variables, indices, assumptions, constraints, objective, and output/acceptance predicate.
4. **Choose methods by structure.** A method must follow from the formal problem, not from familiarity with a tool or language.
5. **Derive before compute when practical.** Hand derivations, limiting cases, invariants, and small examples expose mistaken assumptions early.
6. **Falsify actively.** Search for counterexamples, boundary cases, contradictions, and failure conditions rather than only confirming examples.
7. **Separate generation from verification.** A method that proposes an answer should not be the sole judge of its own answer.
8. **Reproduce every material result.** Record source revision, assumptions, parameters, environment, toolchain, and commands.
9. **Transfer honestly.** A mathematical analogy to finance is not a trading edge. Identify preserved structure, changed assumptions, market frictions, and missing evidence.
10. **Keep negative results.** Rejecting a hypothesis or demonstrating that the source is underspecified is a legitimate outcome.

## 3. The standard practice loop

Every admitted practice case should use the following loop. Not every case needs code or every analysis stage, but omissions must be justified.

| Step | Practice | Questions to answer | Required output |
|---|---|---|---|
| 1. Observe | Inspect source and record facts | What is explicitly given? What is missing? What may not be copied? | Source record and observation list |
| 2. Decompose | Split the problem into entities, relations, decisions, and unknowns | What are the objects, units, states, and dependencies? | Problem statement and unknown register |
| 3. Hypothesize | Generate competing explanations | What could explain the observations? What evidence would falsify each? | Hypothesis register |
| 4. Formalize | Define mathematical objects | What are the sets, variables, domains, constraints, objective, and output? | Formal specification |
| 5. Derive | Work analytically | Can a theorem, invariant, bound, recurrence, or small example simplify the problem? | Derivation and assumptions |
| 6. Select method | Compare suitable approaches | Proof, exhaustive enumeration, search, DP, optimization, probability, simulation, or another method—and why? | Method-selection rationale |
| 7. Compute | Execute only justified computation | Are inputs, parameters, seeds, precision, and stopping conditions explicit? | Reproducible computation record |
| 8. Verify | Check independently | Can a separate calculation, verifier, invariant, proof, or reduced exhaustive check validate it? | Verification report |
| 9. Challenge | Attack the result | What counterexample, perturbation, edge case, or regime change breaks it? | Robustness and failure report |
| 10. Reflect and transfer | Explain what was learned | What was difficult, what failed, what generalizes, and what does not transfer to finance? | Research postmortem and bounded transfer assessment |

### 3.1 Hypothesis register

Every material hypothesis should include:

| Field | Meaning |
|---|---|
| Hypothesis ID | Stable identifier |
| Claim | One falsifiable proposition |
| Rationale | Why it is plausible |
| Assumptions | Conditions required for it to hold |
| Supporting evidence | Sources, calculations, or observations |
| Contradicting evidence | Known counterexamples or inconsistencies |
| Falsification test | What observation would reject it |
| Confidence | Qualitative and explicitly provisional unless calibrated |
| Status | Proposed, supported, rejected, unresolved, or superseded |
| Next action | Smallest evidence-gathering or calculation step |

Do not assign numerical probabilities to subjective confidence unless a calibration methodology exists. Do not count repeated copies of the same source as independent evidence.

### 3.2 Formal mathematical specification

For each case, define where applicable:

- Sets, elements, indices, cardinalities, and relationships.
- Variables, parameters, domains, units, and sign conventions.
- Given data versus derived variables.
- Constraints, equations, boundary conditions, and objective function.
- Input and output spaces.
- Acceptance predicate: exactly what makes a candidate valid.
- Whether the goal is one solution, all solutions, an optimum, a bound, a proof, an estimate, or a decision.
- Assumptions and unresolved semantics.

For every equation, explain every symbol and its domain. Check logical consistency, dimensional consistency where units apply, and limiting cases. Distinguish exact identities from approximations and empirical estimates.

## 4. Learning tracks

The tracks organize practice topics. They are not separate project task boards; the admitted case and its learning objectives determine which topics are used.

### Track A — Mathematical and logical problem solving

- Discrete mathematics, sets, relations, functions, and invariants.
- Combinatorics, permutations, combinations, generating functions, and counting.
- Logic, proof construction, contradiction, induction, and counterexamples.
- Graphs, networks, paths, flows, connectivity, and matching.
- Constraint satisfaction, search spaces, pruning, and state representation.
- Dynamic programming, recurrences, complexity, and approximation.

**Core capability:** convert an ambiguous verbal problem into a well-defined mathematical object and establish what can or cannot be proved.

### Track B — Strategic reasoning and game theory

- Normal-form and extensive-form games.
- Best responses, dominance, and Nash equilibrium.
- Zero-sum games, minimax, and mixed strategies.
- Bayesian games, incomplete information, and belief updates.
- Repeated games, reputation, commitment, and dynamic incentives.
- Auctions, mechanism design, and strategic allocation.

**Core capability:** distinguish individual optimization from strategic interaction, make information assumptions explicit, and reason about equilibrium and incentives.

### Track C — Probability, statistics, and stochastic systems

- Probability spaces, conditional probability, Bayes' rule, and dependence.
- Random variables, expectation, variance, covariance, and distributions.
- Sampling, estimation, hypothesis testing, and uncertainty intervals.
- Markov chains, Poisson processes, Brownian motion, and stochastic dynamics where relevant.
- Monte Carlo estimation, variance reduction, convergence, and simulation error.
- Time-series dependence, stationarity, regime change, and multiple testing.

**Core capability:** distinguish randomness, estimation error, model uncertainty, and evidence; know what a sample can and cannot establish.

### Track D — Mathematics in finance

- Time value of money, discounting, compounding, and annuities.
- Fixed income: cash flows, yield, duration, convexity, curve construction, and day-count.
- Derivatives: forwards, futures, options, payoff decomposition, put-call relationships, Greeks, and hedging.
- Risk: exposure, scenario analysis, drawdown, tail risk, and stress testing.
- Portfolio theory: constraints, covariance, allocation, rebalancing, and transaction costs.
- Market microstructure: order books, spreads, liquidity, impact, execution, and adverse selection.

**Core capability:** connect mathematical quantities to financial contracts, units, conventions, market assumptions, and economic meaning without confusing model output with an executable trading decision.

### Track E — Quantitative computation and numerical methods

- Exact arithmetic, rational computation, decimal/fixed-point conventions, and floating-point limits.
- Linear algebra, numerical conditioning, stability, interpolation, root finding, and optimization.
- Combinatorial and constrained optimization.
- Numerical integration, Monte Carlo, stochastic simulation, and reproducible random streams.
- Algorithm complexity, data structures, memory, parallelism, and performance measurement.
- Sensitivity, error budgets, convergence, and independent reference calculations.

**Core capability:** know when computation is necessary, select an appropriate numeric representation, quantify approximation error, and reproduce results.

## 5. Strategic method selection

The learner must compare methods only after defining the problem structure.

| Method family | Appropriate when | Main risks / questions |
|---|---|---|
| Analytical proof | A general property or exact result can be derived | Are assumptions sufficient? Does the proof cover the full domain? |
| Exhaustive enumeration | The state space is finite and tractable | Is the enumeration truly exhaustive? Is pruning sound? |
| Backtracking / search | Partial assignments can be extended or rejected incrementally | Can search order bias results? Are all branches covered? |
| Dynamic programming | Subproblems overlap and a valid state recurrence exists | Is the state sufficient? Are transitions complete? |
| Graph algorithms | Entities and relations form a graph with relevant structure | Is the graph representation faithful? Are directed/weighted properties needed? |
| Constraint programming / SAT / SMT | Rules can be encoded as explicit constraints | Does the encoding preserve semantics? Is the model complete? |
| Mathematical optimization | There is a defensible objective and feasible set | Is the objective appropriate? Are optimality and feasibility certified? |
| Probability / statistics | The target is uncertain or inferred from samples | Are dependence, bias, uncertainty, and multiple testing addressed? |
| Simulation | Analytical solutions are unavailable or simulation answers a specific question | Is the model valid? Is simulation error controlled? |
| Heuristics / approximation | Exact methods are infeasible and approximation is acceptable | What is the error or guarantee? How is failure detected? |

A method-selection record must state the problem feature that justifies the method, expected complexity/resource needs, completeness or optimality guarantees, assumptions, alternatives rejected, and the verification plan.

## 6. Computational and numeric integrity

Before computation, define:
- Input schema and validation rules.
- Numeric representation and why it is appropriate.
- Precision, rounding, tolerance, and overflow behavior.
- Random seed, generator, sampling method, and stopping criteria for randomized work.
- Runtime/memory limits and failure behavior.
- Output schema and machine-readable status for success, invalid input, infeasible model, unsupported semantics, resource limit, or internal error.

Use exact discrete representations where possible. Do not use floating point for a discrete or monetary calculation by habit. Do not silently coerce invalid input or turn resource exhaustion into an answer.

For quantitative-finance cases, record units, currency, timestamps/time zone, calendars, day-count, compounding, contract specifications, market-data provenance, missing/stale data, corporate actions where relevant, costs, liquidity, market impact, and operational constraints.

## 7. Independent verification and robustness

Verification must be derived from the formal specification rather than copied from the candidate generator.

Applicable techniques include:
- Independent analytical derivation or trusted benchmark calculation.
- Conservation laws, identities, invariants, and dimensional checks.
- Boundary, malformed, and adversarial inputs.
- Property-based and metamorphic tests.
- Exhaustive checks on smaller instances.
- Counterexample minimization and regression fixtures.
- Sensitivity to assumptions, parameters, sample, and market regime.
- Numerical error, stability, and convergence diagnostics.

Separate the following claims:
- A candidate was generated.
- The candidate satisfies stated constraints.
- The method is sound for returned candidates.
- The method is complete over the stated domain.
- The answer is optimal.
- The answer is unique.
- The result is economically useful.

Each claim requires its own evidence. Passing software tests cannot establish economic validity.

## 8. Quantitative-finance research integrity

Where a case uses empirical or market data, define the sample and universe before evaluating results. Address relevant forms of:
- Look-ahead bias.
- Survivorship bias.
- Selection bias.
- Data leakage.
- Multiple testing and data snooping.
- Autocorrelation and dependence.
- Non-stationarity and regime sensitivity.
- Missing, stale, revised, or adjusted data.
- Transaction costs, spread, market impact, liquidity, financing, and operational constraints.

Distinguish statistical significance, economic significance, and deployability. A backtest is an experiment under stated assumptions, not evidence of future returns.

A puzzle or abstract combinatorial problem may have no direct finance transfer. In that case, document only the transferable reasoning method, if any, and state that no financial application was established.

## 9. Learning progression

Progress is determined by demonstrated capability, not by the number of topics read or packages installed.

| Level | Capability | Evidence required |
|---|---|---|
| L1 — Foundations | Translate a clear prompt into variables and simple relationships | Correct definitions, worked examples, and basic checks |
| L2 — Structured analyst | Generate alternatives and build a valid formal model | Hypothesis register, assumptions, and justified method choice |
| L3 — Quantitative practitioner | Derive and compute results with controlled numeric and statistical methods | Reproducible calculations, boundary tests, and uncertainty/error analysis |
| L4 — Researcher | Challenge a model, falsify hypotheses, and independently verify claims | Counterexamples, independent validation, sensitivity/robustness, and limitations |
| L5 — Research designer | Compare methods and design a reusable but evidence-based computational approach | Comparative study, justified abstractions across multiple cases, and peer/independent review where available |

A learner may be at different levels in different tracks. Do not infer mastery in financial mathematics from skill in discrete puzzles, or mastery in software engineering from successful use of a solver.

## 10. Practice-case report standard

A completed practice case should provide, as applicable:
1. Source and rights record.
2. Problem statement and observed facts.
3. Unknowns and hypothesis register.
4. Formal mathematical specification and notation.
5. Derivations, bounds, or proof.
6. Method-selection rationale.
7. Computational experiment and reproducibility instructions, if code is used.
8. Independent verification and robustness analysis.
9. Results, uncertainty, and limitations.
10. Financial-transfer assessment.
11. Reflection: failed hypotheses, mistakes, and reusable lessons.

The case-specific README and formalization record remain under the admitted case directory. Do not create one file per exercise or one directory per speculative idea. Add case-specific artifacts only when the project control task and admission gate authorize them.

## 11. Assessment rubric

Score each applicable dimension from 0 to 4 and provide evidence. A score is not a substitute for a written explanation.

| Dimension | 0 | 2 | 4 |
|---|---|---|---|
| Problem framing | Restates prompt without structure | Identifies key objects and unknowns | Precise, source-grounded, and scope-aware model |
| Hypothesis quality | Guesses one answer | Offers alternatives | Falsifiable alternatives with discriminating tests |
| Formalization | Ambiguous variables/rules | Mostly explicit definitions | Complete domains, assumptions, constraints, and acceptance predicate |
| Method choice | Tool-first | Plausible method with partial rationale | Method follows from structure with trade-offs and guarantees |
| Derivation | Unsupported conclusion | Partial reasoning | Correct, transparent derivation/proof with defined notation |
| Computation | Unreproducible output | Reproducible basic run | Controlled numerics, deterministic metadata, and error handling |
| Verification | Self-confirmation only | Some independent checks | Independent verifier/proof, counterexamples, and robustness |
| Interpretation | Overclaims | States some limitations | Separates mathematical, computational, statistical, and economic claims |
| Reproducibility | Missing steps | Inputs and commands available | Clean reproduction with versions, provenance, and configuration |
| Reflection | No postmortem | Notes difficulties | Explains failed approaches and reusable lessons |

A critical failure in formalization, source rights, or independent verification cannot be compensated for by a high total score.

## 12. Lessons from the first computational case — CS-002

The first computational case, [Finite-Binomial American Put — Optimal Stopping](cases/2026-11-american-put/README.md), provides an initial worked example of how the general practice loop becomes a bounded, executable research artifact. These lessons are case-derived guidance, not assumptions that every later case must share the same model or software architecture.

### 12.1 Admission is separate from implementation

A computational case must pass its own source/rights and mathematical-formalization gate. The blocked Pressure Points pilot was not relaxed or repurposed to justify software work; CS-002 was admitted because it has a self-contained model, explicit parameter domain, objective, transition law, and correctness predicate.

Before selecting a method or creating code, state:
- What is given, what is synthetic, and what is derived.
- The exact admissible parameter domain and boundary conditions.
- The state representation and why it is sufficient for future decisions.
- The objective being optimized and the mathematical meaning of the output.
- A correctness predicate that can reject a candidate independently of the candidate generator.

A missing rule or acceptance condition remains a blocker. It must not be invented merely to make a solver possible.

### 12.2 State sufficiency and policy spaces must be explicit

In CS-002, a recombining binomial state is identified by \((n,j)\), where \(n\) is the time index and \(j\) is the number of up moves. Distinct path histories can recombine to the same state. Therefore, a Markov-node policy and a full-history stopping policy do not have the same policy space.

For the three-period reference case:
- The recombining lattice has 10 nodes in total and 6 non-terminal state nodes.
- The non-recombining history tree has 7 non-terminal decision histories and 8 terminal paths.
- There are \(2^6=64\) deterministic Markov-node policies and \(2^7=128\) deterministic history-dependent policies.
- Exhaustive enumeration finds 4 and 8 maximizing policies respectively, while both policy spaces attain the same exact optimal value \(18325/2058\).

The correct lesson is to count the actual decision locations in the chosen state/history representation. A unique optimal value does not imply a unique maximizing policy, and a tie in one state does not by itself imply multiple values.

### 12.3 Separate candidate generation from certificate verification

Backward induction generates the candidate node values and decisions. The verifier then derives each node's spot directly from the model inputs and checks:
- Node inventory and \((n,j)\) metadata.
- Exact state prices and immediate exercise payoffs.
- Terminal payoff equality.
- The one-step continuation value from the two child values.
- Bellman dominance over both immediate exercise and continuation.
- Attainability of the candidate value by the reported decision.
- Consistency of the reported root value.

The verifier must not call the valuation routine or accept its internal state as proof. For a reduced finite instance, a second method such as exhaustive stopping-policy enumeration adds a valuable cross-check. Deliberately corrupted values and metadata should be rejected.

For stopping-policy enumeration, define the probability weighting carefully. The CS-002 reference test sums complete terminal paths and applies the payoff at the policy's first stopping time. Every terminal path is weighted by its full path probability; the conditional probabilities of unobserved suffixes sum to one. An equivalent prefix-probability calculation is also valid, provided that stopped suffixes are not counted twice or omitted.

### 12.4 Numeric representation and resource limits are part of the model contract

When every input and model operation is rational and the horizon is finite, exact rational arithmetic can eliminate floating-point rounding and tolerance decisions. It does not eliminate modelling error or make the model economically accurate.

For a recombining binomial lattice with \(N\) periods, the node count is:

\[
M(N)=\frac{(N+1)(N+2)}{2}
\]

The number of lattice-node operations is \(O(N^2)\); arbitrary-precision integer bit complexity is a separate cost and must not be described as constant. A computational API should:
- Validate positive parameters and the strict no-arbitrage domain before calculation.
- Use checked integer arithmetic for size calculations.
- Require an explicit caller resource ceiling.
- Reject overflow and limit violations before allocation.
- Return typed errors rather than partial or plausible-looking values.

Choose the numeric representation from the mathematical domain. Exact rational arithmetic is justified here; it is not a blanket requirement for continuous, empirical, or stochastic cases.

### 12.5 Minimal architecture is a research control

CS-002 requires one library package, one locked dependency graph, one path-scoped project workflow, and a small public API. It does not justify a Cargo workspace, CLI, database, serialization layer, network service, benchmark harness, or generic option framework.

Abstractions should be introduced only after repeated requirements across admitted cases demonstrate that they reduce complexity without weakening domain boundaries. A single case is not evidence for a general platform.

### 12.6 Distinguish mathematical exactness from financial validity

The CS-002 reference parameters are synthetic. The exact value is exact only for the finite-horizon, no-dividend, constant-factor binomial model stated in the formalization. The result is not a calibrated market price, forecast, investment recommendation, or executable trading input.

A case report must separately state:
1. Mathematical correctness for the formal model.
2. Implementation and reproducibility evidence.
3. Economic interpretation and applicability.
4. Release and operational authorization.

Passing the first two does not establish the latter two.

### 12.7 First-case admission checklist for future computational work

Before authorizing a later case to create software, require evidence for each applicable item:

| Gate | Required evidence |
|---|---|
| Source and rights | Primary source, access/version record, permissible stored material, and original versus third-party content boundary |
| Formal model | Complete symbols, parameter domains, units, assumptions, transitions, objective, boundary conditions, and output predicate |
| State sufficiency | Explicit state/history definition and proof or rationale that it contains the information required by the decision rule |
| Method selection | Why the chosen method follows from the problem structure; alternatives and complexity/resource limits |
| Independent verification | Separate verifier, proof obligations, reference calculation, exhaustive reduced case, or another genuinely independent check |
| Numeric integrity | Representation, rounding/overflow behavior, exactness or error budget, and resource ceiling |
| Software boundary | Smallest justified package/API; no workspace, CLI, service, framework, or benchmark without a demonstrated requirement |
| Supply chain | Pinned direct dependencies, committed lockfile, transitive inventory/licence review, toolchain/MSRV, and update path |
| Reproducibility | Deterministic inputs, commands, exact toolchains, path-scoped CI, and evidence on the submitted code revision |
| Claims and release | Research-only versus decision-support use, evidence maturity, residual risks, SBOM/release controls, and separate authorization |

An unpassed correctness-critical gate blocks implementation. A passing admission checklist authorizes only the stated case and scope, not future case folders or general abstractions.

## 13. Playbook maintenance


The Playbook changes only when practice evidence, multiple cases, or an approved project decision justifies the change. Do not add methods merely to expand a catalogue. A method entry should include its mathematical structure, applicable problem conditions, assumptions, failure modes, verification requirements, and at least one admitted case or worked synthetic example.

Project progress, tasks, and release readiness remain exclusively in [PROJECT_CONTROL.md](PROJECT_CONTROL.md). Validation outcomes remain in [VALIDATION_MATRIX.md](VALIDATION_MATRIX.md).
