<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Research Specification: [Title]

Use this document for the research-specific part of a registered project. Every project must also follow [PROJECT_LIFECYCLE.md](../PROJECT_LIFECYCLE.md) and [PROJECT_STANDARDS.md](../PROJECT_STANDARDS.md), and have a project README, `project.json`, Decision Log, and Validation Matrix. Start a new project with the [Project Charter Template](PROJECT_CHARTER_TEMPLATE.md) and [Project Manifest Template](PROJECT_MANIFEST_TEMPLATE.json).

> **Project ID:** OFRL-XX-NNN  
> **Lifecycle status:** Proposed | Approved | In Development | Validation | Released | Maintenance | Deprecated | Archived  
> **Project types:** Research / Quantitative Research / Financial Model / Software / Data / Benchmark / Prototype / Infrastructure  
> **Risk tier:** R0 | R1 | R2 | R3  
> **Evidence maturity:** 0–4 (separate from lifecycle status)  
> **Owner / contributors:**  
> **Project manifest:** project.json  
> **Last updated:** YYYY-MM-DD

## 1. Executive summary

What is being investigated, why it matters, and what this project will deliver. Do not state results before they have been established.

## 2. Research question

State one primary question. Add secondary questions only where they are necessary.

## 3. Motivation and context

Explain the financial or analytical problem. Identify relevant prior work and distinguish this project from it.

## 4. Hypotheses or objectives

List testable hypotheses, or explicit objectives if formal hypothesis testing is not suitable.

## 5. Conceptual and mathematical framework

Define:

- Variables, parameters, indices, units, and conventions.
- Governing equations or accounting relationships.
- Assumptions and boundary conditions.
- Expected limiting cases or invariants.

## 6. Data

| Item | Description |
|---|---|
| Source / provider | |
| Access date | |
| Coverage / universe | |
| Frequency / timezone | |
| Transformations | |
| Missing data / exclusions | |
| Licence / permitted use | |
| Redistribution permitted? | |

Do not commit restricted data to the repository. Use a data manifest and acquisition instructions where appropriate.

## 7. Methodology and experimental design

Describe the model, algorithms, benchmarks, sample selection, controls, evaluation metrics, and any train/test or temporal separation.

## 8. Implementation

Describe the modules, interfaces, dependencies, configuration, and execution procedure.

## 9. Validation plan

Define pass/fail criteria before interpreting results.

| Test | Rationale | Pass criterion | Result |
|---|---|---|---|
| Mathematical / accounting invariants | | | Pending |
| Unit tests | | | Pending |
| Data checks | | | Pending |
| Benchmark comparison | | | Pending |
| Sensitivity / robustness | | | Pending |
| Failure-mode analysis | | | Pending |
| Reproduction check | | | Pending |

## 10. Results

Report results tied to the methodology. Include uncertainty and negative or inconclusive findings. Clearly label exploratory or post-hoc analysis.

## 11. Limitations and failure analysis

Describe what the analysis does not establish, where it may fail, and what remains unresolved.

## 12. Reproducibility

Provide environment setup, dependency versions, configuration, commands, and any lawful data acquisition steps.

## 13. References and attribution

List academic, technical, data, and software sources. Identify third-party components and contributor roles.

## 14. Change log

Record material changes to assumptions, methodology, implementation, data, or reported conclusions.