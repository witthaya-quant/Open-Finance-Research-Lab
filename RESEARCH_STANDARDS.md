# OFRL Research Standards

These standards define the minimum structure and quality controls expected of research published under Open Finance Research Lab (OFRL).

They are intended to be rigorous without forcing every project into an academic-paper format. The level of analysis should fit the question, but the reasoning, evidence, and limitations must remain inspectable.

## 1. Required project components

Each substantial project should include the components relevant to its scope.

| Component | Minimum expectation |
|---|---|
| Research question | A precise question, objective, and scope |
| Background | Relevant theory, definitions, and prior work |
| Hypothesis | Testable hypotheses where the project calls for them; otherwise explicit research objectives |
| Mathematical or financial framework | Equations, accounting relationships, model structure, and variable definitions |
| Data methodology | Source, coverage, frequency, transformations, exclusions, rights, and known quality issues |
| Experimental design | Sample definition, benchmarks, controls, train/test separation, and evaluation criteria where relevant |
| Implementation | Readable code or model files, dependencies, and execution instructions |
| Validation | Unit or property tests, reconciliation checks, statistical diagnostics, or other appropriate tests |
| Results | Outputs linked to the stated methodology, including uncertainty where applicable |
| Limitations | Assumptions, failure modes, unresolved questions, and threats to validity |
| Reproducibility | Version information, configuration, data instructions, and steps to reproduce results |
| References | Sources for theory, methods, data, and external material |

A small exploratory study may use a reduced format, but it must be labelled accordingly and must not imply a stronger level of validation than it has achieved.

## 2. Mathematical and model integrity

- Define every material variable, parameter, index, unit, and convention.
- State whether quantities are nominal or real, gross or net, annualised or period-specific, and pre-tax or post-tax where relevant.
- Explain material modelling choices and boundary conditions.
- Check dimensional consistency and mathematical domains.
- For financial statement models, test applicable accounting identities and schedule linkages.
- For pricing and risk models, test limiting cases, monotonicity, numerical stability, and benchmark cases where appropriate.
- Separate model assumptions from observed data and calibrated parameters.

## 3. Data integrity

Every project using external data should document:

- Provider or original source and access date.
- Instrument, entity, universe, geography, and sample coverage.
- Timestamp convention, timezone, calendar, and frequency.
- Corporate-action, adjustment, restatement, and survivorship treatment where relevant.
- Missing-value, outlier, and stale-data handling.
- Transformations and derived fields.
- Licence, terms of use, and redistribution restrictions.

A dataset being publicly accessible does not automatically make it redistributable. If rights are unclear, do not publish the data; publish a data manifest and a lawful acquisition procedure instead.

## 4. Statistical and empirical integrity

Where applicable, research should address:

- Appropriate baselines and comparison methods.
- Sample selection and the possibility of selection bias.
- Multiple testing and data snooping.
- Look-ahead bias, survivorship bias, and information leakage.
- Dependence, autocorrelation, heteroskedasticity, and non-stationarity.
- Parameter sensitivity and stability across samples or regimes.
- Uncertainty intervals and effect sizes, not only statistical significance.
- Economic significance, frictions, constraints, and transaction costs when claiming practical relevance.

A backtest is an experiment under stated assumptions. It is not evidence of future returns and must not be presented as such.

## 5. Software and implementation quality

Substantial implementations should include, where appropriate:

- Clear module boundaries and documented interfaces.
- Reproducible dependency and environment specifications.
- Unit tests for core calculations.
- Integration tests for linked components.
- Property, invariant, or reconciliation tests for financial and mathematical relationships.
- Deterministic test fixtures and seeded randomness where applicable.
- Error handling for invalid inputs and unsupported conditions.
- Examples that demonstrate intended use without silently hiding assumptions.

Tests establish only the conditions they actually exercise. A passing test suite is not proof that a model is economically valid.

## 6. Evidence-maturity levels

Use these labels consistently:

| Level | Label | Required evidence |
|---|---|---|
| 0 | Proposed | Defined question, scope, and proposed methodology |
| 1 | Exploratory | Initial implementation or analysis; results are provisional |
| 2 | Reproducible | Documented execution reproduces the reported results |
| 3 | Robustness Validated | Relevant sensitivity, robustness, and failure-mode analysis completed |
| 4 | Independently Replicated | An independent person or team reproduces the principal result |

Evidence maturity is not a profitability rating, investment endorsement, peer-review designation, or guarantee of external validity.

## 7. Project review gates

Before publication, assess the following gates:

- **Scope:** Is the question specific and the intended claim proportionate?
- **Theory:** Are the financial and mathematical relationships correctly stated?
- **Data:** Are provenance, quality, and usage rights documented?
- **Implementation:** Does the implementation match the stated method?
- **Validation:** Are relevant tests and diagnostics present?
- **Reproducibility:** Can a competent reader follow the documented steps?
- **Disclosure:** Have confidential, proprietary, personal, and restricted materials been excluded?
- **Communication:** Are findings, uncertainty, and limitations clearly separated?

A failed gate should be recorded as a blocker or limitation, not silently waived.

## 8. Reporting negative results

A well-designed experiment that finds no effect, rejects a hypothesis, or exposes a model failure can be valuable. Do not change evaluation criteria after observing results merely to obtain a favourable outcome. Any post-hoc analysis must be labelled as such.

## 9. Corrections and versioning

Material corrections should identify what changed, why it changed, and whether previously reported results are affected. Preserve enough history for readers to understand the evolution of the research.

## 10. Project-specific standards

Projects may add stricter domain standards—for example, accounting reconciliation rules for three-statement models, curve-construction conventions for fixed income, or execution-cost assumptions for market microstructure research. Additional requirements must be documented in the project itself.