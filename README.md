# Open Finance Research Lab (OFRL)

**Open Knowledge. Rigorous Research. Practical Finance.**

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)

OFRL is an independent, founder-led research initiative focused on financial modelling, quantitative finance, investment research, financial engineering, risk analytics, and computational methods.

I started OFRL with a straightforward objective: build finance research that can be examined, reproduced, challenged, and improved. I am interested in the reasoning behind a model—not only its output. That means making assumptions explicit, showing the mathematics where it matters, testing implementation integrity, and being honest about limitations.

The research agenda is grounded in practical financial questions and developed with the discipline of a research process. Industry requirements can help identify relevant problems, but each project must stand on its own: a clear research question, an independently designed methodology, defensible evidence, and reproducible results.

## What OFRL is here to do

- Develop transparent financial models and quantitative research.
- Connect financial theory, mathematical methods, and working implementations.
- Examine methods across both buy-side and sell-side disciplines.
- Turn relevant industry skill requirements into independent, public research questions.
- Establish practical standards for model validation, data quality, and reproducibility.
- Encourage constructive criticism, replication, and collaboration.

OFRL is a research initiative—not a signal service, copy-trading operation, investment newsletter, or performance leaderboard. A research result is not automatically a trading edge or an investment recommendation.

## Research areas

| Area | Scope |
|---|---|
| **Quantitative Finance** | Time series, statistical methods, signal analysis, statistical arbitrage research |
| **Financial Modelling & Valuation** | Integrated financial statements, forecasting, DCF, DDM, LBO, M&A, comparable-company analysis |
| **Equity Research** | Fundamental analysis, earnings, business drivers, valuation, sector research |
| **Fixed Income & Credit** | Bond pricing, yield curves, duration, convexity, credit spreads, default-risk modelling |
| **Derivatives & Volatility** | Options pricing, Greeks, volatility surfaces, hedging, structured products |
| **Market Microstructure & Liquidity** | Order books, liquidity, market impact, execution analysis |
| **Portfolio & Risk Analytics** | Portfolio construction, risk attribution, VaR, expected shortfall, stress testing |
| **Corporate Finance & Capital Markets** | Capital structure, cost of capital, capital allocation, ECM, DCM, M&A |
| **Financial Data Science & Machine Learning** | Statistical learning, regime analysis, financial data engineering, financial NLP |
| **Research Methodology & Model Validation** | Statistical testing, robustness, model risk, sensitivity analysis, reproducibility |

Research methodology and validation are cross-cutting requirements. They apply to every area rather than serving as a final box to tick.

## Industry-to-Research

OFRL observes publicly available role requirements, extracts recurring competencies, and turns them into independently designed research questions. The objective is to connect practical industry work with open methods, transparent implementation, and reproducible evidence.

Job descriptions are inputs for competency analysis, not material to copy. OFRL does not publish confidential employer material or reproduce proprietary work.

## Current research programme

### OFRL-IB-001 — Integrated Three-Statement Financial Model

**Status:** Proposed — specification stage. Rust-first architecture.

The first project will develop a Rust-native calculation engine for a transparent three-statement model, with a Rust validation suite and analyst-facing Excel reporting. Rust is the canonical implementation; Python is not required by the first release.

- [Project overview](projects/OFRL-IB-001/README.md)
- [Research specification](projects/OFRL-IB-001/RESEARCH_SPECIFICATION.md)
- [Model conventions](projects/OFRL-IB-001/MODEL_CONVENTIONS.md)
- [Rust-first architecture](projects/OFRL-IB-001/RUST_ARCHITECTURE.md)
- [Validation matrix](projects/OFRL-IB-001/VALIDATION_MATRIX.md)
- [Decision log](projects/OFRL-IB-001/DECISION_LOG.md)

## Evidence maturity

| Level | Meaning |
|---|---|
| **0 — Proposed** | A defined question and research plan; no empirical claim yet |
| **1 — Exploratory** | Initial implementation or analysis; findings remain provisional |
| **2 — Reproducible** | Methods, inputs, and outputs can be reproduced from documented instructions |
| **3 — Robustness validated** | Relevant sensitivity, robustness, and failure-mode checks have been completed |
| **4 — Independently replicated** | An independent contributor has reproduced the principal result |

These levels describe the maturity of evidence. They do not certify profitability, guarantee predictive power, or imply academic peer review.

## Research principles

1. **Substance over presentation.** A polished chart or notebook is not evidence by itself.
2. **Assumptions are visible.** Material assumptions must be stated and justified.
3. **Validation is part of the model.** Tests and failure analysis are designed alongside the implementation.
4. **Negative results matter.** A well-supported result that a method does not work is a useful contribution.
5. **Reproducibility matters.** Data provenance, versions, dependencies, and execution steps should be documented.
6. **Evidence is not advice.** Empirical findings must be distinguished from investment recommendations.
7. **Respect data rights.** Public availability does not automatically permit redistribution or commercial reuse.
8. **Protect legitimate proprietary work.** OFRL does not publish confidential information, client data, or private trading strategies.

## Collaboration

OFRL welcomes thoughtful research proposals, technical discussion, constructive review, and independent replication. Contributions should be clear about their sources, assumptions, limitations, and the work performed by each contributor.

Start with [CONTRIBUTING.md](CONTRIBUTING.md) and [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md). For reuse and licensing, read [LICENSE_POLICY.md](LICENSE_POLICY.md).

## Founder

**Kenny**  
Founder, Open Finance Research Lab

I set the research direction, define the questions worth investigating, and take responsibility for the assumptions, validation criteria, and conclusions published under this initiative. The standard is not whether a model looks impressive; it is whether its reasoning and evidence can withstand scrutiny.

---

*OFRL is an independent research initiative. Unless a project states otherwise, published research is educational and analytical in nature and is not personalised investment, legal, tax, or financial advice.*