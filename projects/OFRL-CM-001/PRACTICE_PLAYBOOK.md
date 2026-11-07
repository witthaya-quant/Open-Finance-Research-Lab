<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Quantitative Problem-Solving Practice Playbook

**Document version:** 0.1.0  
**Project:** OFRL-CM-001  
**Status:** Initial programme design  
**Companion control:** [PROJECT_CONTROL.md](PROJECT_CONTROL.md)  
**Owner:** Kenny (Witthaya Maidee)  
**Last reviewed:** 2026-11-05

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

## 12. Playbook maintenance

The Playbook changes only when practice evidence, multiple cases, or an approved project decision justifies the change. Do not add methods merely to expand a catalogue. A method entry should include its mathematical structure, applicable problem conditions, assumptions, failure modes, verification requirements, and at least one admitted case or worked synthetic example.

Project progress, tasks, and release readiness remain exclusively in [PROJECT_CONTROL.md](PROJECT_CONTROL.md). Validation outcomes remain in [VALIDATION_MATRIX.md](VALIDATION_MATRIX.md).
