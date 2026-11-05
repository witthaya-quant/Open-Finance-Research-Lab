<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Project Catalog

This catalog lists the projects officially admitted to the OFRL portfolio. A branch, experiment, Issue, or unregistered folder is not automatically an approved project.

Every project below must have a current `project.json` manifest. The manifest is the machine-readable source of truth for its status, owner, types, risk tier, evidence maturity, licence profile, and data classification.

## Registered projects

| Project ID | Project | Types | Lifecycle | Risk | Evidence |
|---|---|---|---|---|---|
| [OFRL-IB-001](projects/OFRL-IB-001/README.md) | Integrated Three-Statement Financial Model | Research, Financial Model, Software | In Development | R2 | 0 — Proposed |
| [OFRL-CM-001](projects/OFRL-CM-001/README.md) | Computational Methods & Problem-Solving Case Studies | Research | In Development | R1 | 0 — Proposed |

## Adding a project

1. Submit a Research / Project Proposal using the repository Issue Form.
2. Obtain Founder approval and an assigned project ID.
3. Create `projects/<PROJECT-ID>/` from the project templates.
4. Add a valid `project.json` manifest and the required project documents.
5. Add the project to this catalog in the same Pull Request.
6. Pass the repository policy workflow.

See [Project Lifecycle](PROJECT_LIFECYCLE.md), [Project Standards](PROJECT_STANDARDS.md), and [Governance](GOVERNANCE.md).
