<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Project Standards

**Policy ID:** OFRL-PRJ-002  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Policy owner:** Founder / Principal Maintainer  
**Applies to:** Every current and future OFRL project

## 1. Purpose

These standards define the common quality baseline and the additional controls required by each project type. They apply across the OFRL portfolio. A project may use any appropriate language, framework, data source, or research method; it must meet the requirements relevant to its own scope and risk.

The purpose is consistency of reasoning, evidence, disclosure, and release—not identical technology or identical document names for unrelated disciplines.

## 2. Universal requirements

Every approved project must maintain:

| Requirement | Minimum standard |
|---|---|
| Identity | Unique project ID, `project.json`, owner, status, project types, risk tier, and evidence maturity |
| Purpose and scope | README, intended use, included/excluded scope, and limitations |
| Specification | A versioned research, product, model, or technical specification appropriate to the project |
| Decision history | Project Decision Log with material assumptions, scope, and design decisions |
| Validation | A validation matrix or test plan with explicit PASS / FAIL / NOT RUN / N/A states |
| Reproducibility | Environment, dependencies, configuration, source versions, commands, and data-acquisition steps where applicable |
| Data rights | Provenance, classification, permitted use, and redistribution status for non-trivial data |
| Intellectual property | Applicable licence, attribution, contributor rights, and public/private disclosure boundary |
| Change control | Pull Requests for material changes, review evidence, and linked Decision Records where required |
| Release readiness | Release status, evidence maturity, version/revision, known limitations, and authorization |
| Security | No secrets or unauthorized restricted information in source, issues, logs, fixtures, or artifacts |
| Maintenance | Known defects, deprecation path, and ownership of future changes |

A document may cover more than one requirement when the content is clear and traceable. OFRL does not require duplicate paperwork merely to create more files.

## 3. Project types and additional requirements

Projects may have multiple types. The manifest must list every type that materially describes the deliverable.

### 3.1 Research

Examples: research notes, theory, literature-based analysis, methodological studies, and empirical investigation.

Required:
- Research question, context, prior work, and intended contribution.
- Hypotheses or explicit research objectives.
- Methods, assumptions, definitions, sources, and limitations.
- Evaluation design and result interpretation.
- Negative and inconclusive results where methodologically relevant.
- Evidence-maturity level for material claims.

### 3.2 Quantitative research

Examples: statistical analysis, time-series research, portfolio analysis, derivative modelling, market microstructure research, and machine learning experiments.

In addition to Research:
- Sample, universe, time window, frequency, timezone, and source provenance.
- Bias controls appropriate to the method: look-ahead, survivorship, leakage, selection, multiple testing, and data snooping.
- Benchmarks, baselines, sensitivity, uncertainty, and robustness.
- Reproducible data preparation and feature/label construction.
- Transaction costs, market impact, liquidity, financing, and operational constraints when claims concern trading or implementation.
- Clear separation of statistical significance, economic significance, and deployability.

### 3.3 Financial model

Examples: valuation, integrated statements, forecasts, risk models, pricing models, and capital allocation models.

In addition to Research:
- Explicit units, currency, signs, periods, day-count, accounting basis, and material conventions.
- Defined variables, formulas, schedules, dependencies, and calculation order.
- Reconciliation identities and invariants.
- Boundary, negative, stress, and sensitivity tests.
- Numerical representation, precision, rounding, overflow, and error-budget policy.
- Independent benchmark or calculation cross-check for material outputs before a strong correctness claim.
- Clear distinction between analytical assumptions and statutory accounting or regulatory compliance.

### 3.4 Software or reusable tooling

Examples: libraries, CLIs, APIs, applications, data pipelines, and developer tools.

Required:
- Supported scope, public interfaces, architecture, and compatibility expectations.
- Reproducible build/test commands and lockfile policy where supported.
- Unit, integration, boundary, and contract tests appropriate to the code.
- Typed errors and safe handling of invalid inputs.
- Dependency, licence, and security review.
- Workflow permissions and secret handling.
- Versioning, changelog, release provenance, SBOM, and checksums for distributable releases.
- No claim of production suitability without deployment, reliability, and security evidence.

### 3.5 Data product or dataset

Examples: synthetic datasets, curated datasets, benchmark corpora, and data dictionaries.

Required:
- Data manifest and schema.
- Provenance, source terms, collection/generation date, coverage, units, and transformations.
- Classification, privacy assessment where personal data may exist, and redistribution rights.
- Integrity checks, quality rules, missingness, and known defects.
- Versioning, change history, and checksum where appropriate.
- Reproducible generation or lawful retrieval instructions.

### 3.6 Benchmark or test corpus

Examples: golden cases, reference outputs, synthetic test corpora, performance datasets, and evaluation suites.

Required:
- Purpose, construction method, expected behavior, and coverage.
- Clear distinction between synthetic/reference and observed production data.
- Fixed benchmark version and reproducible evaluation commands.
- No leakage between training, tuning, and held-out evaluation where relevant.
- Independent reference or oracle methodology for high-impact comparisons.
- Licence and redistribution controls.

### 3.7 Prototype or proof of concept

Examples: experimental adapters, candidate libraries, architecture spikes, and technical feasibility demonstrations.

Required:
- Explicit POC label, question, hypothesis, and time/scope boundary.
- Isolated dependencies and no implied production support.
- Known limitations and failed experiments.
- Acceptance criteria for promotion, redesign, or abandonment.
- No production, model-correctness, or security claim beyond the tests actually performed.

A prototype must not silently become a production dependency. Promotion requires a Decision Record and completion of applicable software, research, security, and release gates.

### 3.8 Infrastructure or integration

Examples: CI tooling, automation, connectors, external system integrations, and deployment components.

Required:
- Architecture, trust boundaries, permissions, data flows, and failure behavior.
- Threat model proportionate to external exposure and privileges.
- Least-privilege credentials and secret handling.
- Logging and observability without sensitive-data leakage.
- Rollback, recovery, and dependency/service outage behavior.
- Explicit approval before interacting with production accounts or external systems.

## 4. Risk tiers

Risk tier measures the potential impact of incorrect behavior, misuse, exposure, or failure. It is not the same as evidence maturity or project importance.

| Tier | Definition | Typical examples | Minimum additional control |
|---|---|---|---|
| **R0 — Informational** | Documentation, educational explanation, or exploratory work with no operational decision output | Literature note, conceptual derivation | Source attribution, scope, review, and limitation statement |
| **R1 — Analytical** | Research or analytical output that may influence understanding but is not presented as an operational decision system | Statistical study, non-deployed model experiment | Reproducible methods, data provenance, relevant bias/robustness checks |
| **R2 — Decision-support / reusable software** | Financial models, reusable libraries, tools, or outputs that users may apply in business or financial decisions | Three-statement model, valuation library, portfolio analytics tool | Formal specification, validation suite, numeric/data controls, release documentation, stronger review |
| **R3 — Operational / sensitive** | Software that executes external actions, handles sensitive personal/regulated data, controls live systems, or can cause material direct harm | Automated trading/execution, privileged integrations, sensitive-data platform | Founder approval via ADR, threat model, privacy/legal review as applicable, independent domain/security review, operational safeguards, incident and rollback plan before release |

The Founder may assign a higher tier than examples suggest. When uncertain, use the higher plausible tier until the risk is understood. A project may not reduce its tier solely to avoid a control.

R3 activity is not automatically prohibited, but it requires explicit approval and additional evidence before implementation against live systems or public release. Research-only simulation must remain isolated from live credentials and execution.

## 5. Cross-project validation profiles

A project may select one or more validation profiles in its manifest:

- `research`
- `quantitative`
- `financial_model`
- `software`
- `data`
- `benchmark`
- `security`
- `privacy`
- `performance`
- `integration`

Profiles identify the relevant checks. They do not replace the project validation matrix. A project must state which checks are PASS, FAIL, NOT RUN, or N/A and why.

## 6. Evidence maturity

Evidence maturity follows the repository's 0–4 scale in [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md). Assign maturity to the specific claim, result, or released component.

Do not infer maturity from lifecycle state:
- `in_development` does not mean evidence level 1.
- `released` does not automatically mean level 4.
- A software component can be implemented while the financial model or research hypothesis remains unvalidated.

Where a project contains multiple materially different deliverables, document their maturity separately in its README or release record.

## 7. Review intensity

Review intensity must reflect risk and the type of claim:

- R0: Founder or delegated Maintainer review; correct attribution and scope.
- R1: Research-method review; data and empirical design reviewed where relevant.
- R2: Domain review of calculations, interfaces, validation evidence, and operational limitations.
- R3: Independent domain and security/privacy review as relevant; live use or external action must not rely solely on Founder self-review.

During Stage 0, the Founder may approve their own work because no independent Maintainer exists. This must be disclosed as Founder approval, not independent review. For R2/R3 claims requiring independent assurance, the project must remain exploratory or unreleased until appropriate external review can be obtained or the claim is narrowed honestly.

## 8. Common non-conformance rules

The following are release blockers unless the affected scope is explicitly excluded and the claim is narrowed:

- Unclear source/data rights or missing attribution.
- Hidden balancing plugs, unreported data leakage, or post-hoc acceptance criteria.
- Unhandled errors or silently substituted values in material calculations.
- Unreproducible material results.
- A mismatch between published specification and actual implementation.
- A public claim stronger than the available validation evidence.
- Known critical security issue or exposed credential.
- Missing project owner or unresolved publication authority.

## 9. Technology neutrality

OFRL does not mandate Rust, Python, Node, SQL, Excel, or a particular orchestration framework across the whole portfolio.

Select technologies by the project's:
- Computational and numerical requirements.
- Reproducibility and maintainability.
- Security, deployment, and integration boundaries.
- Available testing and interoperability ecosystem.
- Contributor capability and long-term ownership.
- Licensing and supply-chain risk.

A project may use a different language or stack from another project. Each choice must be documented and justified at the project level.

## 10. Related policies

- [Project Lifecycle](PROJECT_LIFECYCLE.md)
- [Research Standards](RESEARCH_STANDARDS.md)
- [Governance](GOVERNANCE.md)
- [Dependency Policy](DEPENDENCY_POLICY.md)
- [Data Policy](DATA_POLICY.md)
- [Release Policy](RELEASE_POLICY.md)
