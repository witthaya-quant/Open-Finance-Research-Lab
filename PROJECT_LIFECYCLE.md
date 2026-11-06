<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Project Lifecycle

**Policy ID:** OFRL-PRJ-001  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Policy owner:** Founder / Principal Maintainer  
**Applies to:** Every current and future project maintained or published under the OFRL name

## 1. Purpose

This policy defines the lifecycle through which an idea becomes an approved OFRL project, is developed, validated, released, maintained, and eventually deprecated or archived.

It applies to every project regardless of discipline, programming language, implementation technology, or intended output. A project may be a research study, quantitative method, financial model, software library, application, dataset, benchmark, infrastructure component, or prototype.

OFRL-IB-001 is the first registered project, not the source of rules that every other project must copy mechanically. Project-specific requirements are selected through the project type and risk profile defined in [PROJECT_STANDARDS.md](PROJECT_STANDARDS.md).

## 2. Mandatory project identity

Every project admitted under the OFRL name must have:

- A unique, stable project ID.
- A canonical directory under `projects/<PROJECT-ID>/`.
- A machine-readable `project.json` manifest validated against [the project manifest schema](schemas/ofrl-project-manifest-1.1.0.schema.json).
- A project README describing purpose, scope, status, and limitations.
- A project Decision Log.
- A validation matrix or explicit validation plan proportionate to the project.
- A project specification appropriate to its type. A separate Project Charter is useful when it adds clarity, but it may be integrated into the specification rather than duplicated.
- Type-specific documents created from the templates under [templates/](templates/).
- A Data Manifest and the `data` validation profile whenever `data_classification` is not `none`, including synthetic datasets.
- An identified project owner and lifecycle status.
- Declared project types, risk tier, evidence maturity, licence profile, and data classification.
- Links to the applicable specification, standards, and supporting artifacts.

No project may be represented as an approved OFRL project merely because a folder, branch, Issue, or prototype exists. Admission is a Founder decision recorded through a Project Proposal and the project manifest.

## 3. Lifecycle states

Lifecycle state and evidence maturity are independent dimensions. A project can be in development while its research evidence remains exploratory. A validated research result does not automatically mean a software package is released.

| State | Meaning | Minimum entry criteria | Exit / transition evidence |
|---|---|---|---|
| `proposed` | An idea submitted for consideration | Problem, intended use, owner/proposer, initial scope, project type, and known rights/data issues | Founder accepts, rejects, defers, or requests redesign |
| `approved` | Authorized to enter the OFRL portfolio | Assigned ID, approved scope, project types, preliminary risk tier, initial manifest, licence/data classification, validation approach, and Decision Record where material | Development baseline and required documents are established |
| `in_development` | Active research or implementation | Approved project baseline, project README/specification, Decision Log, validation plan, and project-specific CI where applicable | Candidate implementation or research artifact is ready for validation |
| `validation` | Candidate is under formal testing or review | Scope and acceptance criteria frozen for the candidate; reproducible inputs/environment; validation evidence recorded | Gates pass, limitations are explicitly accepted for a bounded pre-release, or the candidate returns to development |
| `released` | An artifact or research work has been publicly released | Release authorization, licence/data/IP gates, evidence maturity, provenance, tests, version/tag or publication revision, release notes, and applicable SBOM/checksums | Maintained, deprecated, or corrected through a new version |
| `maintenance` | A released project receives fixes and compatible improvements | Support scope, version status, issue route, dependency/security review, and correction process | Continued maintenance, deprecation, or archival |
| `deprecated` | No longer recommended for new use | Reason, affected versions, migration or replacement guidance where available, and notice | Archived after the retention/maintenance period |
| `archived` | Work is preserved for history and reference, with no active development commitment | Final status, preserved source/history, known limitations, and no unresolved disclosure or security issue requiring active mitigation | Reopen only through a recorded decision |

A project may move from `in_development` back to `proposed` or `approved` when scope changes materially. The change must be recorded rather than hidden in a status edit.

## 4. Project admission gate

Before approving a project, the Founder must establish:

1. **Purpose:** the problem, intended users, and practical or research value.
2. **Scope:** what is included, excluded, and explicitly not claimed.
3. **Classification:** all applicable project types and the risk tier.
4. **Evidence approach:** the hypotheses, methods, evaluation criteria, or software acceptance tests.
5. **Data and rights:** expected sources, provenance, classification, redistribution, and restrictions.
6. **Publication boundary:** what can be public and what must remain private or restricted.
7. **Technical approach:** languages, dependencies, interfaces, environments, and alternatives where relevant.
8. **Ownership and contribution:** project owner, expected contributors, and applicable licences.
9. **Resources and dependencies:** major external services, providers, maintainers, and operating assumptions.
10. **Exit criteria:** what would justify release, redesign, pause, or termination.

The approval is a scope and governance decision. It is not evidence that a hypothesis is true or an implementation is correct.

## 5. Project manifest and catalog

The `project.json` file is the authoritative machine-readable metadata for one project. It declares:

- Project identity and summary.
- Lifecycle status and evidence maturity.
- Project types and risk tier.
- Project owner and date metadata.
- Licence profile and data classification.
- Required project documents and validation profiles.
- Release status.
- A versioned `release_readiness` assessment, risk-tier gate evidence, named assessors, and a separate release-authorization record.

The manifest's `release_readiness` contract distinguishes `NOT_ASSESSED`, `BLOCKED`, and `ELIGIBLE_FOR_AUTHORIZATION` from the separate authorization states `NOT_AUTHORIZED`, `AUTHORIZED`, and `WITHDRAWN`. A green assessment is not publication permission. Projects may remain `NOT_ASSESSED` during proposal and development; once an assessment is started or release is requested, the registry requires the full applicable gate inventory and rejects unsupported lifecycle transitions.

The repository-wide [Project Catalog](PROJECT_CATALOG.md) is a human-readable navigation index. Every project directory must appear in the catalog. The repository policy workflow compiles the manifest JSON Schema with Ajv and validates each project manifest, then checks directory/ID consistency, required files, type/risk profiles, workflow paths, lifecycle/release consistency, risk-tier release-readiness evidence, authorization gates, and catalog-field agreement.

Do not duplicate mutable project facts across multiple root registries. Update the project manifest first, then update the catalog links and summary in the same Pull Request.

## 6. Project IDs and directory rules

Use:

`OFRL-<AREA>-<NNN>`

- `OFRL` identifies the lab.
- `AREA` is a short uppercase discipline or portfolio code.
- `NNN` is a three-digit sequence allocated by the Founder.
- IDs are stable and never reused, even when a project is rejected or archived.

Use the exact ID as the project directory name. A material change in research objective, ownership boundary, or independently versioned deliverable may justify a separate project ID; a routine implementation phase does not.

## 7. Project-specific CI and validation ownership

The repository-wide workflow validates shared metadata, local links, configuration syntax, and project registration. It is not a substitute for each project's own test/build workflow.

Every project containing executable code or generated artifacts must define the commands and CI checks needed to validate its supported scope. The workflow must trigger for changes to the project, its shared schemas/fixtures, and relevant build configuration. It must use least-privilege permissions, avoid secrets on untrusted fork Pull Requests, and report failures without silently skipping required checks. A project may reuse common scripts or workflows, but its domain-specific acceptance criteria remain in its own validation matrix.

Pure research or documentation projects must still validate references, data/source rights, calculations where applicable, and reproducibility evidence. Do not add a software build workflow to a project that has no software to build.

## 8. Change control

A project change requires a new Decision Record or a Decision Log entry when it materially changes:

- Research question, hypothesis, intended use, or scope.
- Model equations, financial/accounting conventions, risk assumptions, or data definitions.
- Public interfaces, schemas, numerical representations, or compatibility promises.
- Data provenance, data classification, licence, disclosure, or publication rights.
- Risk tier, evidence maturity, validation criteria, release status, or supported use.
- Major architecture, dependencies, external services, or security boundary.

Routine implementation details may be recorded in the Pull Request without a separate Decision Record if they remain within an accepted project baseline.

A project-level decision may make a project stricter than the lab-wide standard. It may not silently waive a lab-wide requirement, legal obligation, licence, data restriction, security control, or Founder-reserved decision.

## 9. Project pause, rejection, and archival

A project may be rejected, deferred, paused, deprecated, or archived because the question is no longer relevant, evidence is insufficient, rights are unclear, risk cannot be controlled, resources are unavailable, or the work does not meet its stated purpose.

A negative result or failed hypothesis is not by itself a reason to hide or delete a project. Preserve useful findings and limitations. Do not leave abandoned prototypes labelled as released or validated.

## 10. Relationship to other policies

- [PROJECT_STANDARDS.md](PROJECT_STANDARDS.md) defines project type profiles, risk tiers, and minimum evidence requirements.
- [GOVERNANCE.md](GOVERNANCE.md) defines authority and review rights.
- [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md) defines research integrity and evidence.
- [DATA_POLICY.md](DATA_POLICY.md) and [LICENSE_POLICY.md](LICENSE_POLICY.md) define rights and data boundaries.
- [DEPENDENCY_POLICY.md](DEPENDENCY_POLICY.md) defines software supply-chain controls.
- [RELEASE_POLICY.md](RELEASE_POLICY.md) defines publication and release authorization.
- The [project templates](templates/) provide starting materials for README, charter, manifest, decision log, validation matrix, research specifications, financial-model conventions, numerical policy, data, benchmarks, software architecture, threat modelling, prototypes, and release records.

## 11. Review and amendment

Review this policy at least annually and when a new project type, material risk class, or release model is introduced. Amendments require a Pull Request and a lab-wide Decision Record.
