<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Computational Methods & Problem-Solving Case Studies

> **Project ID:** OFRL-CM-001  
> **Lifecycle:** In Development  
> **Project type:** Research  
> **Risk tier:** R1 — Analytical  
> **Evidence maturity:** 0 — Proposed  
> **Project version:** 0.1.0  
> **Owner:** Kenny (Witthaya Maidee)  
> **Manifest:** [project.json](project.json)  
> **Initial case study:** Jane Street — Pressure Points (November 2026)

## Purpose

This project investigates selected mathematical and computational problems as disciplined research case studies. It aims to explain how a problem can be formalized, identify methods that are justified by its actual structure, assess correctness and limitations, and preserve reproducible evidence.

The project is inspired by Jane Street's public puzzle programme. Jane Street describes creative problem-solving as intrinsic to its work, connecting it to identifying new problems in financial markets and developing potentially novel ways to solve them. OFRL uses that statement as context for studying problem-solving methods; it does not infer that an individual puzzle is a financial model, a trading strategy, or a disclosure of Jane Street's internal methods.

## Independence from other OFRL projects

OFRL-CM-001 is a separate project in the OFRL portfolio. It is not part of, an extension of, or a dependency of [OFRL-IB-001](../OFRL-IB-001/README.md).

The projects have separate:
- Project identities, specifications, decision logs, and validation records.
- Research questions, methods, deliverables, and acceptance criteria.
- Case data, fixtures, implementation choices, and release decisions.
- Branches, issues, and pull requests.

The only portfolio-level relationship is the shared OFRL governance and the central Project Catalog. No OFRL-IB-001 implementation, model convention, fixture, schema, or workflow is reused by this project.

## Research question

How can selected mathematical and computational problems be translated into explicit formal models, investigated using suitable alternative methods, and verified in a reproducible way, while identifying which insights have a defensible connection to quantitative research or financial engineering?

The project does not assume that every case requires software, multiple solvers, or a finance application. Executable software is developed only when it adds research value. The project-specific [Implementation Approach](IMPLEMENTATION_APPROACH.md) defines the engineering, verification, reproducibility, and dependency controls.

## Scope

### Included
- Formal problem definition, notation, assumptions, and unresolved rules.
- Mathematical foundations relevant to each admitted case.
- Selection and comparison of methods when a meaningful comparison exists.
- Independent verification of candidate outputs where an implementation is developed.
- Boundary, adversarial, invariant, and reproducibility checks as appropriate.
- Honest assessment of financial-engineering transferability, including a finding of no meaningful transfer where warranted.
- Source attribution and third-party rights controls.

### Excluded
- A general puzzle-solving platform, puzzle scraper, or automated monthly intake pipeline.
- An AI agent or general-purpose solver framework.
- Claims about Jane Street's proprietary trading systems, strategies, or internal processes.
- Investment recommendations, trading signals, or automated execution.
- Automatic admission of every new puzzle.
- Work assigned to, or changes made within, another OFRL project.

## Current status

| Deliverable | Status | Evidence |
|---|---|---|
| Separate project identity and manifest | Prepared in this branch | project.json; repository-policy CI passed on PR revision |
| Project research specification | Prepared | RESEARCH_SPECIFICATION.md |
| Shared case-study protocol | Prepared | CASE_STUDY_PROTOCOL.md |
| First-pilot source record | Prepared | DATA_MANIFEST.md |
| Pressure Points formalization | Initial observations recorded; rule semantics unresolved | cases/2026-11-pressure-points/ |
| Solver or solution | Not started; not authorized by this phase | Deferred |
| Comparative solver benchmark | Not started | Deferred |
| Financial-engineering application claim | Not established | Must be assessed case by case |

## First pilot: Pressure Points

The first pilot is the November 2026 Jane Street puzzle, [Pressure Points](https://www.janestreet.com/puzzles/pressure-points-index/).

The pilot is deliberately limited to source provenance, observable input structure, mathematical formalization, and identification of missing rule semantics. The source page presents letter sequences, emoji clues, a leading marker, and a terminal notation. The exact relationships and required output must be established before selecting an algorithm.

OFRL does not reproduce the full puzzle statement, its images, the complete source data, or Jane Street's solution in this repository. See the [case-study record](cases/2026-11-pressure-points/README.md), [formalization note](cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md), and [Data Manifest](DATA_MANIFEST.md).

## Research method

All case studies follow [CASE_STUDY_PROTOCOL.md](CASE_STUDY_PROTOCOL.md):
1. Problem formalization.
2. Mathematical foundations.
3. Alternative method assessment.
4. Independent verification.
5. Comparative analysis.
6. Financial-engineering transfer assessment.
7. Research publication and reproducibility.

The protocol is a set of research gates, not a requirement to implement every method in every case.

## Shared OFRL policies

OFRL-CM-001 inherits the repository-wide policies as its common governance baseline. The authoritative versions remain at the repository root; this project references them and does not create local copies or amend them.

- [Governance](../../GOVERNANCE.md) — authority, decision rights, review, and change control.
- [Project Lifecycle](../../PROJECT_LIFECYCLE.md) and [Project Standards](../../PROJECT_STANDARDS.md) — project admission, required controls, and lifecycle.
- [Research Standards](../../RESEARCH_STANDARDS.md) — evidence, mathematical integrity, reproducibility, and research claims.
- [Data Policy](../../DATA_POLICY.md), [License Policy](../../LICENSE_POLICY.md), and [Publication and IP Boundary](../../PUBLICATION_AND_IP_BOUNDARY.md) — provenance, third-party rights, licensing, and disclosure.
- [Dependency Policy](../../DEPENDENCY_POLICY.md), [Security Policy](../../SECURITY.md), and [Release Policy](../../RELEASE_POLICY.md) — supply-chain controls, security, and publication authorization.

Project-specific requirements are recorded only in this project’s own specification, protocol, decision log, validation matrix, and implementation approach. They may strengthen the common baseline but must not silently waive or contradict it.

## Project controls

- One active case study at a time.
- A new case requires an explicit intake decision and documented scope.
- A solver is not started until the problem rules and acceptance criteria are sufficiently formalized.
- Research maturity is assigned to claims and results, not inferred from implementation status.
- Third-party content remains under its own rights and terms.
- Failed, blocked, unrun, and inapplicable checks remain visible in the [Validation Matrix](VALIDATION_MATRIX.md).

## Project documents

- [Project manifest](project.json)
- [Working handoff](HANDOFF.md) — continuation instructions and scope safeguards for future sessions.
- [Research specification](RESEARCH_SPECIFICATION.md)
- [Case-study protocol](CASE_STUDY_PROTOCOL.md)
- [Decision log](DECISION_LOG.md)
- [Validation matrix](VALIDATION_MATRIX.md)
- [Data manifest](DATA_MANIFEST.md)
- [Pressure Points case record](cases/2026-11-pressure-points/README.md)
- [Pressure Points problem formalization](cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md)

## References and attribution

Primary source:
- Jane Street, [Puzzles](https://www.janestreet.com/puzzles/) — explains the purpose of the puzzle programme and Jane Street's stated rationale for creative problem-solving.
- Jane Street, [Pressure Points — November 2026](https://www.janestreet.com/puzzles/pressure-points-index/) — original source of the first-pilot puzzle.
- Jane Street, [Puzzles Archive](https://www.janestreet.com/puzzles/archive/) — archive and context for other published puzzles.

Methodological context, not a pilot input:
- Jane Street Blog, [Results from the ASIC puzzle](https://blog.janestreet.com/asic-puzzle-results/), 2 October 2026. This describes a separate hardware-reverse-engineering challenge and is cited only as an example of diverse solution approaches and verification issues.

Access date for the sources above: 2026-11-05. Third-party source material is not relicensed by OFRL.

## Licensing and disclosure

Original OFRL software, if later approved for release, follows the repository's Apache-2.0 software policy. Original OFRL research documentation follows CC BY 4.0. Jane Street materials and all other third-party content remain under their respective terms. Public availability is not treated as redistribution permission.

See the repository's [Data Policy](../../DATA_POLICY.md), [Publication and IP Boundary](../../PUBLICATION_AND_IP_BOUNDARY.md), and [Research Standards](../../RESEARCH_STANDARDS.md).
