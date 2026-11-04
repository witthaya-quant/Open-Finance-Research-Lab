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

### Shared foundations, different ways of applying them

Many of us enter finance and business through a shared body of knowledge. We study familiar concepts, equations, and frameworks through degrees, professional qualifications, certificates, or independent study. Black–Scholes is one well-known example: many people learn the same foundational model, yet develop different ways to apply, extend, parameterise, test, or combine it in practice.

The differences often come from the work built around those foundations: the assumptions we choose, the information we rely on, how we structure a model, the constraints we face, and the experience we gain from applying it. Over time, each person or team develops a practical method of its own—a personal recipe of techniques, assumptions, and judgement refined for a particular purpose.

I do not want OFRL to become a place where people compete over whose recipe is best, whose qualifications carry more weight, or who gets to claim the one correct way. I want people to put methods they are able to share on the table, explain how they work, and help one another understand what each approach can contribute.

### Focus on what we can build from the work

The questions that matter to us are practical:

- What problem is the method intended to solve, and for whom?
- Under which assumptions, data, market conditions, or business constraints is it useful?
- What does it do well, and where does it become limited or unsuitable?
- Which parts can be improved, simplified, extended, combined, or reused?
- What would another person need to understand, test, reproduce, or build on it?

A contribution does not have to replace an existing method. It may fill a gap, improve one part of a process, make an implementation more reliable, provide a better fit for a particular use case, or combine useful ideas from different approaches. Different methods may complement one another rather than compete for a single place.

### Open to different approaches, serious about the work

OFRL does not assume that every method is equally reliable or suitable for every purpose. We examine mathematical and financial logic, assumptions, data quality, implementation, robustness, and practical results. When a method is incorrect or does not meet its stated objective, we should say so clearly and explain why. That is a judgement about the work under defined conditions—not a judgement about the intelligence, credentials, or worth of the person who developed it.

We are not trying to avoid disagreement. We are trying to make disagreement useful. The goal is to understand what the evidence tells us, identify what remains uncertain, and decide what can be learned or developed next—not to turn every difference into a contest over who is right or better.

**OFRL is a place to develop ideas together, not a place to defend personal territory.** Bring what you have learned and built, share what you are entitled to share, make the reasoning understandable, and help turn useful work into something others can inspect, adapt, and use.

You can contribute by proposing a research question, presenting an alternative method, reproducing an existing result, improving a model, testing an implementation, or building reusable tools. Start with the [Contributing Guide](CONTRIBUTING.md) or submit a [Project / Research Proposal](.github/ISSUE_TEMPLATE/research_proposal.yml).

Read [FOUNDATION_AND_DIRECTION.md](FOUNDATION_AND_DIRECTION.md) for the full foundation, vision, mission, principles, and collaboration philosophy.

## Project portfolio and operating standards

Every current and future project follows the same admission, identity, decision-history, evidence, data-rights, publication, and release baseline. Project-specific controls are selected by project type and risk; OFRL does not mandate one programming language, framework, or validation tool for unrelated disciplines.

- [Project Catalog](PROJECT_CATALOG.md) — registered projects and their current status.
- [Project Lifecycle](PROJECT_LIFECYCLE.md) — proposal, admission, development, validation, release, maintenance, and archival.
- [Project Standards](PROJECT_STANDARDS.md) — universal requirements, project-type profiles, risk tiers, and validation profiles.
- [Project Manifest Schema](schemas/ofrl-project-manifest-1.0.0.schema.json) — machine-readable project metadata contract.
- [Project Templates](templates/README.md) — reusable starting documents for every project type.

OFRL establishes governance from the beginning, including while it is maintained by one person. [GOVERNANCE.md](GOVERNANCE.md) defines founder-led authority, contributor roles, review controls, and Decision Records. OFRL is designed to remain under the Founder's personal GitHub account; a GitHub Organization is optional, not a required destination.

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

Rust is the main implementation language selected for the current financial-model project. This is not a repository-wide language mandate: each project selects its technology according to numerical, research, interoperability, security, performance, and maintenance requirements, with the choice documented in its project specification.

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

I welcome people who want to bring a method they have developed, question an established approach, reproduce someone else's work, contribute a different implementation, or help turn a useful finding into a tool others can use. You do not need to agree with the founder, hold a particular qualification, or use the same technique as an existing contributor.

A useful contribution does not have to replace what is already there. It may explain a limitation, improve one part of a workflow, provide a more suitable method for a specific situation, make an implementation easier to reproduce, or show that two approaches can complement one another.

When we review work, we ask what problem it addresses, what assumptions it makes, how it behaves, what evidence supports it, where it is useful, and what remains to be improved. We do not use credentials, seniority, reputation, or personal preference as a substitute for examining the work.

Contributors should explain their reasoning and give others enough information to inspect and build on it. Where an approach is not yet supported, we can identify what evidence or improvement is needed rather than dismissing the person who proposed it. Where a method fails, the failure can still teach us something useful.

Start with [CONTRIBUTING.md](CONTRIBUTING.md), [RESEARCH_CHARTER.md](RESEARCH_CHARTER.md), and [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md).

## Founder

**Kenny (Witthaya Maidee)**  
Founder, Open Finance Research Lab

OFRL reflects the questions I choose to investigate and the standards I expect of the work. My responsibility is to set the direction, understand the assumptions, and stand behind the methods and conclusions published here. I do not expect contributors to think like me or use the same methods I use. I want the work to become more useful, more understandable, and more capable of helping the next person build something better.

---

*OFRL is an independent research initiative. Its publications are educational and analytical in nature and are not personalised investment, legal, tax, or financial advice.*
