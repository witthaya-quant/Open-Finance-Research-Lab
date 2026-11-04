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

**Vision:** Build an open and evolving body of financial knowledge, research methods, and practical tools that people can inspect, test, use, adapt, and improve together.

**Mission:** Bring people with different backgrounds and experience together to examine financial and business problems, develop and compare methods, and turn useful work into models, software, and resources that others can build on.

### Shared foundations, different methods

Many of us come into finance and business through a shared body of knowledge. We learn common concepts, equations, and frameworks through degrees, professional qualifications, certificates, or independent study. Black–Scholes is one familiar example: people may learn the same foundational model, then apply, extend, parameterise, test, or combine it in different ways.

What differs is often the practical work built around those foundations: the assumptions we choose, the information we rely on, how we structure a model, the constraints we face, and the experience we gain from applying it. Over time, each person or team develops its own practical approach—its own **“secret sauce”**: a combination of methods, assumptions, techniques, processes, and judgement refined through experience for a particular purpose.

I do not see those differences as a reason to decide who is better, whose qualifications carry more weight, or whose method must be the one correct answer. I see an opportunity to put different approaches alongside one another, understand how each was developed, and learn what each can contribute.

A common formula or qualification is a starting point, not a complete answer to every problem. The way people apply a shared foundation can differ for good reasons: they may be solving different problems, working with different information, or facing different constraints. Those differences are worth examining before we decide what is useful.

### Focus on useful outcomes

I do not want OFRL to become a place that declares one approach universally right and another universally wrong. A method can be shown to be mathematically incorrect, inconsistent with its own assumptions, or unsuitable for the objective it claims to serve; when that happens, we should explain it and show the evidence. But that should not become a judgement about the person, nor a reason to dismiss methods that address a different problem or operate under different constraints.

The questions that matter to us are practical:

- What problem is a method intended to solve, and for whom?
- Under which assumptions, data, market conditions, or business constraints is it useful?
- What does it do well, and where does it become limited or unsuitable?
- What can be improved, simplified, extended, combined, or reused?
- What would another person need to understand, test, reproduce, or build on it?

A contribution does not have to replace an existing method. It may fill a gap, improve one part of a process, make an implementation more reliable, fit a particular use case better, or combine useful ideas from different approaches. Different methods may complement one another rather than compete for a single place.

### Examine the work, not the person

Being open to different approaches does not mean treating every method as equally reliable or suitable for every purpose. We still examine the mathematics, financial logic, assumptions, data, implementation, robustness, and practical results. When a method is incorrect or does not meet its stated objective, we should explain that clearly and show why.

The distinction is that we assess the work against its purpose and evidence—not the intelligence, credentials, reputation, or status of the person who developed it. A degree or professional qualification can provide useful context, but it cannot replace a clear explanation or a testable result.

We are not trying to avoid disagreement. We are trying to make disagreement useful: understand what the evidence tells us, identify what remains uncertain, and work out what should be improved next.

### Build together, share responsibly

OFRL welcomes research proposals, alternative methods, independent implementations, replication, constructive criticism, and practical tools. People do not need to agree with me or use the same methods I use to contribute.

No one is expected to disclose confidential know-how, employer or client material, or a method they do not have the right to publish. The aim is to share what can be shared responsibly and make that work understandable and useful to others.

**We do not have to think alike to make progress together. We do have to take the work seriously.**

Read [RESEARCH_CHARTER.md](RESEARCH_CHARTER.md), [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md), and [CONTRIBUTING.md](CONTRIBUTING.md) to understand how this philosophy is put into practice.

## Project portfolio and operating standards

Every current and future project follows the same admission, identity, decision-history, evidence, data-rights, publication, and release baseline. Project-specific controls are selected by project type and risk; OFRL does not mandate one programming language, framework, or validation tool for unrelated disciplines.

- [Project Catalog](PROJECT_CATALOG.md) — registered projects and their current status.
- [Project Lifecycle](PROJECT_LIFECYCLE.md) — proposal, admission, development, validation, release, maintenance, and archival.
- [Project Standards](PROJECT_STANDARDS.md) — universal requirements, project-type profiles, risk tiers, and validation profiles.
- [Project Manifest Schema](schemas/ofrl-project-manifest-1.0.0.schema.json) — machine-readable project metadata contract.
- [Project Templates](templates/README.md) — reusable starting documents for all project types.

The full foundation and principles are in [FOUNDATION_AND_DIRECTION.md](FOUNDATION_AND_DIRECTION.md). [GOVERNANCE.md](GOVERNANCE.md) defines Founder-led authority, contributor roles, review controls, and Decision Records. OFRL is designed to remain under the Founder's personal GitHub account; a GitHub Organization is optional, not a required destination.

[PUBLICATION_AND_IP_BOUNDARY.md](PUBLICATION_AND_IP_BOUNDARY.md) explains what belongs in the public research repository and what should remain private or confidential.

- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [GitHub Repository Settings Baseline](GITHUB_REPOSITORY_SETTINGS.md)
- [Central Decision Register](decisions/README.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Research Standards](RESEARCH_STANDARDS.md)
- [Dependency Policy](DEPENDENCY_POLICY.md)
- [Release Policy](RELEASE_POLICY.md)
- [Repository Audit](REPOSITORY_AUDIT.md)

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

Rust is the main implementation language selected for the current financial-model project, where it is used for the calculation model, validation logic, command-line tools, and reusable components. It is not a repository-wide requirement. Each project should choose its tools and architecture according to its research, numerical, interoperability, security, performance, and maintenance needs, and explain that choice.

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

- **Understand before judging.** Establish the problem, objective, assumptions, and context before comparing approaches.
- **Compare methods, not people.** Credentials, seniority, reputation, and personal preference are not substitutes for examining the work.
- **Keep alternatives open.** Do not assume that one formula, model, framework, or implementation is universally best.
- **Be open-minded and evidence-driven.** Respect different approaches while examining their logic, data, implementation, and results.
- **Focus on what can be built.** Identify what is useful, what is limited, and what can be improved, combined, or developed further.
- **Show the work.** Make assumptions, calculations, implementation choices, and sources visible.
- **Test the mechanics.** A model that runs is not necessarily correct or fit for its intended use.
- **Report limitations honestly.** Negative, inconclusive, and conflicting findings are part of research.
- **Respect data and contribution rights.** Credit the work accurately and do not publish material without the right to share it.
- **Separate evidence from advice.** Research outputs are not personalised investment recommendations.

## Licensing

OFRL uses separate licences by content type:

- **Software, source code, tests, and computational financial models:** Apache-2.0.
- **Research writing, documentation, specifications, and explanatory materials:** CC BY 4.0.
- **Data:** source-specific rights; original OFRL synthetic research datasets are CC BY 4.0 by default.

See [LICENSE](LICENSE), [LICENSE-docs](LICENSE-docs), [LICENSE_POLICY.md](LICENSE_POLICY.md), and [DATA_POLICY.md](DATA_POLICY.md). Third-party materials remain subject to their own terms.

## Collaboration

I welcome people who want to bring a method they have developed, question an established approach, reproduce someone else's work, contribute a different implementation, or help turn a useful finding into something others can use.

A contribution does not have to prove that an existing approach is wrong or replace it. It can explain a limitation, improve one part of a workflow, provide a better fit for a particular situation, make the work easier to reproduce, or show how two approaches complement one another.

When reviewing a contribution, we focus on the problem, assumptions, method, evidence, behaviour, practical use, and remaining limitations. Critique the method or claim; do not dismiss the person who proposed it. Contributors should identify their sources, assumptions, limitations, and the work they performed.

No one is expected to disclose confidential or restricted material. Share only work and information that you are entitled to publish.

Start with [CONTRIBUTING.md](CONTRIBUTING.md) and [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md).

## Founder

**Kenny (Witthaya Maidee)**  
Founder, Open Finance Research Lab

OFRL reflects the questions I choose to investigate and the standards I expect of the work. My responsibility is to set the direction, understand the assumptions, and stand behind the methods and conclusions published here.

---

*OFRL is an independent research initiative. Its publications are educational and analytical in nature and are not personalised investment, legal, tax, or financial advice.*
