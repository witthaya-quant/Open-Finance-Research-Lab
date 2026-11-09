<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Computational Methods & Problem-Solving Case Studies

> **Project ID:** OFRL-CM-001  
> **Lifecycle:** In Development  
> **Project type:** Research, Quantitative Research, Software  
> **Risk tier:** R1 — Analytical  
> **Evidence maturity:** 0 — Proposed  
> **Project version:** 0.3.0  
> **Owner:** Kenny (Witthaya Maidee)  
> **Manifest:** [project.json](project.json)  
> **Original research pilot:** Jane Street — Pressure Points (November 2026)  
> **First computational case:** Finite-Binomial American Put — Optimal Stopping (CS-002)

## Purpose

This project investigates selected mathematical and computational problems as disciplined research case studies. It aims to explain how a problem can be formalized, identify methods that are justified by its actual structure, assess correctness and limitations, and preserve reproducible evidence.

The project is inspired by Jane Street's public puzzle programme. Jane Street describes creative problem-solving as intrinsic to its work, connecting it to identifying new problems in financial markets and developing potentially novel ways to solve them. OFRL uses that statement as context for studying problem-solving methods; it does not infer that an individual puzzle is a financial model, a trading strategy, or a disclosure of Jane Street's internal methods.

## Project management and practice authority

[PROJECT_CONTROL.md](PROJECT_CONTROL.md) is the sole project-management source of truth for scope, status, phases, task register, blockers, and next actions. Every new chat or assisting agent must read it first and follow [HANDOFF.md](HANDOFF.md). This README is navigation only and must not maintain a competing roadmap.

[PRACTICE_PLAYBOOK.md](PRACTICE_PLAYBOOK.md) defines the learner-facing practice methodology, mathematical tracks, hypothesis process, method selection, verification, and assessment rubric.

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

The programme combines mathematical problem-solving, strategic reasoning and game theory, mathematics in finance, and quantitative computational methods. It does not assume that every case requires software, multiple solvers, or a finance application. Executable software is developed only when a formal specification and research objective justify it. The project-specific [Implementation Approach](IMPLEMENTATION_APPROACH.md) defines the engineering, verification, reproducibility, and dependency controls.

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
| Separate project identity and manifest | Registered on `main` | project.json; PR #29 merged as `42e4a806c950f6efebc260e636e8ff3b98f51dd8` |
| Project research specification | Published on `main` | RESEARCH_SPECIFICATION.md |
| Shared case-study protocol | Published on `main` | CASE_STUDY_PROTOCOL.md |
| First-pilot source record | Published on `main` | DATA_MANIFEST.md |
| Pressure Points formalization | INCOMPLETE / BLOCKED; H-017/H-018 rejected as a complete source-grounded explanation under CM-T0009 | PROBLEM_FORMALIZATION.md § CM-T0009; TEST-046 |
| Official Jane Street solution | Live Archive page lists the November puzzle and displays “Solution” as unlinked text; no direct target/body retrieved | CM-T0010 BLOCKED; DATA_MANIFEST.md §4; TEST-044 |
| Pressure Points solver or solution | Not started; solver remains NO-GO while source semantics are unresolved | CM-T0010/T0011 BLOCKED; deferred |
| First computational case — American Put | ADMITTED and IMPLEMENTED — finite-horizon no-dividend binomial American Put; exact-rational valuation and independent Bellman verifier passed two-toolchain CI on code revision 65d19861; PR #37 remains unreleased and under review | cases/2026-11-american-put/README.md; PROBLEM_FORMALIZATION.md; ENGINEERING_ARCHITECTURE.md; VALIDATION_MATRIX.md |
| Comparative solver benchmark | Not started; no benchmark programme authorized | Deferred |
| Financial-engineering application claim | Not established | Must be assessed case by case |

## First pilot: Pressure Points

The first pilot is the November 2026 Jane Street puzzle, [Pressure Points](https://www.janestreet.com/puzzles/pressure-points-index/).

The pilot is deliberately limited to source provenance, observable input structure, mathematical formalization, and identification of missing rule semantics. The source page presents letter sequences, emoji clues, a leading marker, and a terminal notation. The exact relationships and required output must be established before selecting an algorithm.

OFRL does not reproduce the full puzzle statement, its images, the complete source data, or Jane Street's solution in this repository. See the [case-study record](cases/2026-11-pressure-points/README.md), [formalization note](cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md), and [Data Manifest](DATA_MANIFEST.md).

## First computational case: Finite-Binomial American Put

The first computational case is [Finite-Binomial American Put — Optimal Stopping](cases/2026-11-american-put/README.md). It studies a finite-horizon, recombining binomial model with a fixed gross risk-free return, no dividends, and an American put payoff. The formalization defines the admissible parameter domain, risk-neutral probability, state lattice, stopping-time objective, Bellman recurrence, and correctness predicate before computation.

The initial implementation is a case-specific Rust library using exact arbitrary-precision rational arithmetic. It returns a full node-value and exercise-policy certificate; a separately implemented verifier checks the terminal conditions, Bellman dominance inequalities, and policy attainability. All inputs are synthetic. No market data, production pricing interface, trading recommendation, or external execution path is in scope. The project remains R1 and research-only; any public reusable pricing or decision-support release requires a separate risk and release assessment.

## First-case learning record

The initial Practice Playbook v0.2.0 incorporates lessons from CS-002 on state sufficiency, recombination, stopping-policy enumeration, exact rational arithmetic, Bellman certificates, resource guards, dependency controls and research/market-use boundaries. This learning update does not admit a second case; future case admission remains a separate owner decision.

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

- [Canonical project control and delivery register](PROJECT_CONTROL.md) — the only project-level status, roadmap, and task register.
- [Quantitative Problem-Solving Practice Playbook](PRACTICE_PLAYBOOK.md) — practice workflow, learning tracks, hypothesis method, and assessment.
- [Project manifest](project.json)
- [Working handoff](HANDOFF.md) — continuation instructions and scope safeguards for future sessions.
- [Research specification](RESEARCH_SPECIFICATION.md)
- [Engineering architecture](ENGINEERING_ARCHITECTURE.md) — case-specific software architecture and validation boundary
- [American Put case record](cases/2026-11-american-put/README.md)
- [American Put mathematical formalization](cases/2026-11-american-put/PROBLEM_FORMALIZATION.md)
- [Implementation principles](IMPLEMENTATION_APPROACH.md)
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
