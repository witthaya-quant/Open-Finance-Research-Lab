<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Open Finance Research Lab (OFRL)

**Financial Research · Quantitative Methods · Software Engineering**

[![Code: Apache-2.0](https://img.shields.io/badge/Code-Apache--2.0-blue.svg)](LICENSE) [![Docs: CC BY 4.0](https://img.shields.io/badge/Docs-CC--BY--4.0-lightgrey.svg)](LICENSE-docs)

OFRL is an independent finance research and software engineering initiative established by Kenny. It covers financial modelling, quantitative finance, investment research, financial engineering, risk analytics, and the software used to investigate these subjects.

## Why I started OFRL

My work in financial markets began in 2012. Since then, I have worked across proprietary trading, market-making, and the development of financial products. Those experiences shaped the way I approach financial problems: understand the mechanics, make assumptions explicit, test how a model behaves, and be clear about what the results do—and do not—tell us.

I created OFRL to develop and share this work in a form that other people can inspect, challenge, reproduce, and build on. The research questions and direction come from my own interests, experience, and judgement. Software engineering is how I turn those questions into models and tools that can be tested and improved.

I am deliberately building deeper systems-engineering capability with Rust. The aim is to apply it to real financial software and, as the work matures, contribute reusable components and improvements to the wider Rust ecosystem. I would rather release a small component that solves a real problem than publish abstractions before they are ready.

## Vision, mission & foundation

**Vision:** To build an open, evolving body of financial knowledge, research methods, and practical tools that people can inspect, test, use, adapt, and improve together.

**Mission:** To bring together people with different experiences and areas of expertise to investigate financial and business problems, develop and examine alternative methods, and turn useful research into reproducible models, software, and practical resources.

People may learn the same financial and mathematical foundations but apply them differently in practice. Their methods are shaped by different objectives, assumptions, information, markets, business models, and constraints. I see those differences as an opportunity to learn from one another—not a reason to rank people or assume that one approach is universally superior.

OFRL is not here to prove who is smarter or whose method must win. It is here to understand how different methods work, examine where they are useful, identify where they fall short, and work out what can be improved or built from them.

This does not mean that every method is equally sound or that every idea should be accepted. We remain open to different approaches while examining them against clear objectives, mathematical and financial correctness, implementation quality, evidence, robustness, and practical constraints.

Read [FOUNDATION_AND_DIRECTION.md](FOUNDATION_AND_DIRECTION.md) for the full foundation, vision, mission, principles, and collaboration philosophy.

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

Research methodology and model validation apply across all areas. They are part of how work is done, not a separate step added just before publication.

## How projects take shape

Industry requirements are one source of research questions. I review publicly available job requirements, identify the underlying financial, quantitative, and engineering skills, then use those skills to frame independent projects.

The work should move through a clear sequence:

1. Define the financial or technical problem.
2. Review the relevant theory, methods, and prior work.
3. State assumptions and design the model.
4. Implement the method and test its behaviour.
5. Examine edge cases, limitations, and alternative explanations.
6. Publish the method and results in a form others can reproduce.

Job descriptions are used to understand skills and industry needs; their text is not copied into OFRL publications. Confidential employer material, client information, and proprietary trading strategies do not belong in public repositories.

## Engineering approach

Rust is the main implementation language for OFRL's financial software. In the current project, it is intended for the calculation model, validation logic, command-line tools, and reusable components.

Inputs and outputs should have documented schemas and stable conventions. Financial calculations should be testable independently of the reporting layer. Spreadsheets may be useful for analyst review and presentation, but the calculation rules need a clearly identified source of truth.

Other tools may be used where they add a specific benefit—for example, data inspection, independent comparisons, or interoperability. They should support the research rather than create competing implementations of the same financial logic.

As components become generally useful beyond one project, I intend to consider publishing them for the wider Rust community. A separate crate should have a stable purpose, a clear API, tests, documentation, and a reason to exist independently.

## Current research programme

### OFRL-IB-001 — Integrated Three-Statement Financial Model

**Status:** Model-input contract and Rust runtime semantic validator are implemented and CI-verified on Rust 1.85.0 and 1.99.1. The forecast calculation engine and analyst-facing workbook have not been released.

The first project is a financial model for a generic non-financial operating company. The plan is to implement the calculation engine and validation suite in Rust, then provide an Excel report for finance practitioners. The Rust implementation will be the reference for the calculations.

- [Project overview](projects/OFRL-IB-001/README.md)
- [Research specification](projects/OFRL-IB-001/RESEARCH_SPECIFICATION.md)
- [Model conventions](projects/OFRL-IB-001/MODEL_CONVENTIONS.md)
- [Engineering architecture](projects/OFRL-IB-001/ENGINEERING_ARCHITECTURE.md)
- [Validation matrix](projects/OFRL-IB-001/VALIDATION_MATRIX.md)
- [Decision log](projects/OFRL-IB-001/DECISION_LOG.md)
- [Decimal contract POC](projects/OFRL-IB-001/poc/decimal-contract/README.md)
- [Dependency review](projects/OFRL-IB-001/poc/decimal-contract/DEPENDENCY_REVIEW.md)
- [Model input contract](projects/OFRL-IB-001/MODEL_INPUT_CONTRACT.md)
- [Numeric envelope and precision policy](projects/OFRL-IB-001/NUMERIC_ENVELOPE.md)
- [Validation and error contract](projects/OFRL-IB-001/VALIDATION_ERROR_CONTRACT.md)
- [Rust runtime semantic validator](projects/OFRL-IB-001/RUNTIME_VALIDATOR.md)
- [Synthetic reference case A](projects/OFRL-IB-001/SYNTHETIC_REFERENCE_CASE.md)
- [Model input JSON Schema](schemas/ofrl-ib-001-model-input-0.1.0.schema.json)
- [Synthetic Case A input fixture](fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json)
- [Schema validation script](scripts/validate-ofrl-ib-001-model-input-schema.sh)
- [Rust runtime validator CI](.github/workflows/ofrl-ib-001-semantic-validator.yml)


No calculation engine or workbook is claimed to be complete at this stage.

## Research standards

A published project should show:

- The question being investigated and why it matters.
- The relevant theory, equations, assumptions, and data.
- How the model or analysis was implemented.
- Which tests were performed and what those tests establish.
- Where the method works, where it fails, and what remains uncertain.
- How another person can reproduce the work.

OFRL records evidence maturity so that an exploratory result is not presented as a validated finding.

| Level | Meaning |
|---|---|
| **0 — Proposed** | A defined question and research plan; no empirical claim yet |
| **1 — Exploratory** | Initial implementation or analysis; findings remain provisional |
| **2 — Reproducible** | Methods, inputs, and outputs can be reproduced from documented instructions |
| **3 — Robustness validated** | Relevant sensitivity, robustness, and failure-mode checks have been completed |
| **4 — Independently replicated** | An independent contributor has reproduced the principal result |

These levels describe evidence maturity. They do not certify profitability, guarantee predictive power, or imply academic peer review.

## Principles

- **Understand before judging.** Explain the problem, objective, assumptions, and context before comparing approaches.
- **Compare methods, not people.** Technical disagreement is not a contest of credentials, intelligence, or personal worth.
- **Keep more than one approach possible.** Do not assume that one model, formula, framework, or implementation is universally best.
- **Be open-minded, but evidence-driven.** Respect different perspectives without treating every claim as equally supported.
- **Show the work.** Make assumptions, calculations, implementation choices, and sources visible.
- **Test the mechanics.** A model that runs is not necessarily a model that is correct or fit for its intended use.
- **Explain trade-offs and limitations.** Negative and inconclusive findings are part of research.
- **Build for practical use.** Turn useful findings into reusable models, software, tools, and documentation where appropriate.
- **Respect data and contribution rights.** Give accurate credit and do not publish confidential or restricted material.

## Licensing

OFRL uses separate licences by content type:

- **Software, source code, tests, and computational financial models:** Apache-2.0.
- **Research writing, documentation, specifications, and explanatory materials:** CC BY 4.0.
- **Data:** source-specific rights; original OFRL synthetic research datasets are CC BY 4.0 by default.

See [LICENSE](LICENSE), [LICENSE-docs](LICENSE-docs), [LICENSE_POLICY.md](LICENSE_POLICY.md), and [DATA_POLICY.md](DATA_POLICY.md). Third-party materials remain subject to their own terms.

## Collaboration

I welcome research proposals, alternative methods, technical discussion, careful review, independent implementations, and replication. You do not need to agree with the existing approach or the founder to contribute. What matters is whether the work is explained clearly, relevant to the problem, supported by appropriate evidence, and developed with respect for other contributors and their rights.

A contribution is not accepted simply because it is different or novel. Reviews focus on the quality of the work: its assumptions, correctness, evidence, implementation, reproducibility, licensing, and maintainability. Critique methods and claims, not people.

Start with [CONTRIBUTING.md](CONTRIBUTING.md), [RESEARCH_CHARTER.md](RESEARCH_CHARTER.md), and [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md).

## Founder

**Kenny (Witthaya Maidee)**  
Founder, Open Finance Research Lab

OFRL reflects the questions I choose to investigate and the standards I expect of the work. My responsibility is to set the direction, understand the assumptions, and stand behind the methods and conclusions published here. I do not expect contributors to think like me or use the same methods I use. I want the work to become more useful, more understandable, and more capable of helping the next person build something better.

---

*OFRL is an independent research initiative. Its publications are educational and analytical in nature and are not personalised investment, legal, tax, or financial advice.*
