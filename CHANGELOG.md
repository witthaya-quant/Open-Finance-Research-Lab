<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Changelog

All notable changes to OFRL's public research, software, policies, and repository workflows are recorded here.

This changelog follows the structure of Keep a Changelog. Software components may also maintain their own version histories where their release cadence differs from the repository.

## [Unreleased]

### Added
- Founder-led governance, decision rights, contributor roles, and staged delegation policy.
- Code of Conduct, Security Policy, and personal-account continuity guidance.
- Public/private publication and intellectual-property boundary.
- Repository-wide audit, dependency policy, release policy, and account continuity policy.
- Central Decision Register and reusable Decision Record template.
- Research Proposal, Project Charter, Project Manifest, Project Decision Log, and Pull Request templates.
- Project Lifecycle, Project Standards, Project Catalog, and versioned Project Manifest Schema.
- Automated validation of every project directory, manifest, required document, project type, risk tier, and catalog entry.
- Always-running repository-policy validation workflow.
- Dependabot configuration for Cargo and GitHub Actions.

### Changed
- Clarified that a GitHub Organization is optional and not a required destination.
- Clarified contribution rights and the distinction between repository administration, copyright ownership, and licence permissions.
- Clarified permissive Apache-2.0 downstream reuse and the absence of a reciprocal source-sharing obligation.
- Updated OFRL-IB-001 decision status to distinguish implemented input validation from the forecast calculation engine that remains unimplemented.
- Improved dependency and release traceability requirements.

### Security
- Documented account recovery, successor, secret-handling, and repository-continuity controls.
- Added an always-running policy validation gate; manual GitHub security and branch settings remain separately tracked until verified.

---

## Changelog rules

- Add notable changes under `[Unreleased]` as work is merged.
- When a component is released, move the relevant entries under its version and release date.
- Do not claim a release version before the release tag exists.
- Record breaking interface, schema, numeric, and model-convention changes explicitly.
- Link material changes to the relevant Pull Request, Issue, or Decision Record.
