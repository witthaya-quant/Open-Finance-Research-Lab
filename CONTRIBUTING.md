<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Contributing to OFRL

Thank you for taking an interest in Open Finance Research Lab. OFRL values careful thinking, technical clarity, reproducible work, and constructive disagreement.

All contributors are expected to follow the [OFRL Code of Conduct](CODE_OF_CONDUCT.md), [Governance Policy](GOVERNANCE.md), and applicable [Security Policy](SECURITY.md). The governance model applies now, including during the founder-maintained stage.

The objective is not to maximise the number of notebooks or commits. It is to produce research that another person can understand, test, and improve.

## Before opening a contribution

For a new project or research direction, use the GitHub [Project / Research Proposal issue form](.github/ISSUE_TEMPLATE/research_proposal.yml) or the reusable [Research Proposal Template](templates/RESEARCH_PROPOSAL_TEMPLATE.md). An approved project must use the [Project README Template](templates/PROJECT_README_TEMPLATE.md) and [Project Manifest Template](templates/PROJECT_MANIFEST_TEMPLATE.json), then be listed in the [Project Catalog](PROJECT_CATALOG.md). Complete the project's purpose, objectives, scope, risks, and deliverables using the [Project Charter Template](templates/PROJECT_CHARTER_TEMPLATE.md) or integrate those contents into its primary specification; do not create duplicate documents solely to satisfy a template list. Select type-specific documents from [PROJECT_STANDARDS.md](PROJECT_STANDARDS.md), using the corresponding templates under [templates/](templates/). Any project using non-trivial external or synthetic data must also include a [Data Manifest](templates/DATA_MANIFEST_TEMPLATE.md) and the `data` validation profile. Project IDs are assigned by the Founder; contributors must not invent or reuse IDs. Open an issue or discussion with:

1. The problem and why it matters.
2. The proposed research question.
3. Relevant financial or mathematical concepts.
4. Data requirements and the intended source.
5. The proposed method and validation approach.
6. Known risks, limitations, and licensing considerations.

For a code contribution, explain the problem being solved, the approach taken, the tests added or run, and any assumptions introduced.

## Contribution principles

- Keep claims proportionate to evidence.
- Cite external research, data, and implementation sources.
- Do not copy substantial text from job advertisements, books, papers, vendor documentation, or other protected materials.
- Do not submit confidential employer information, client data, private trading strategies, credentials, or restricted datasets.
- Do not include a dataset unless its terms permit the intended redistribution.
- Separate exploratory findings from validated results.
- Include tests for material calculations and invariants.
- Explain breaking changes and material model changes.
- Be respectful and specific when reviewing another person's work.

## Research contributions

All projects must follow [PROJECT_LIFECYCLE.md](PROJECT_LIFECYCLE.md) and [PROJECT_STANDARDS.md](PROJECT_STANDARDS.md). Research contributions also follow [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md); software, data, benchmark, prototype, and infrastructure contributions follow the additional profiles applicable to their manifest. The registry workflow checks the manifest against the published schema and cross-checks the catalog and type-specific document requirements. A proposal does not need to be fully developed before discussion, but it should make the intended question, scope, risk, data rights, and evidence requirements clear.

OFRL welcomes established methods, alternative methods, independent implementations, replications, and well-reasoned challenges to existing work. A contributor does not need to agree with the founder or the current project approach to participate.

When proposing an alternative, explain the problem it is intended to solve, the conditions under which it may be useful, the assumptions it makes, and how its results could be examined. Do not present novelty or personal confidence as evidence of superiority.

Where approaches serve different objectives or perform differently under different constraints, it may be more useful to document those trade-offs than to force a single winner. A conclusion that the evidence is insufficient to choose between methods is acceptable when that is what the work supports.

## Code contributions

Where relevant, include:

- A concise description of the change.
- Tests covering the changed behaviour.
- Reproduction steps or a minimal example.
- Dependency and environment changes.
- Notes on numerical precision, units, and edge cases.
- Documentation for public interfaces.

Avoid bundling unrelated changes into one pull request.

Use the repository's [Pull Request template](.github/PULL_REQUEST_TEMPLATE.md). During the current single-maintainer stage, the Founder may author, review, and merge a Pull Request, but that action is recorded as a single-maintainer decision—not independent review. Once other eligible Maintainers are appointed, authors must not approve their own contributions and the approval requirements in [GOVERNANCE.md](GOVERNANCE.md) apply.

## Attribution and contribution rights

Describe each contributor's substantive role accurately. Do not claim authorship of work performed by someone else. Review the repository's [LICENSE_POLICY.md](LICENSE_POLICY.md) and [DATA_POLICY.md](DATA_POLICY.md) before contributing.

Unless a separate written agreement or an explicit file-level notice states otherwise, contributions intentionally submitted for inclusion are licensed according to their content type:

- Software, source code, tests, build/configuration files, executable notebooks, and computational financial models: Apache-2.0.
- Documentation, research writing, reports, diagrams, and explanatory materials: CC BY 4.0.
- Original synthetic datasets: CC BY 4.0, except synthetic fixtures that are integral software test assets and are expressly identified as Apache-2.0.
- Third-party data or content: its original licence and terms; submitting a contribution does not give OFRL permission to relicense it.

By submitting a contribution, you confirm that you have the rights and authority to make it available under the applicable licence. Contributors retain copyright in their original contributions while granting the rights stated in the relevant licence.

Do not submit employer-owned, client-owned, confidential, restricted, or third-party material unless you have the necessary rights and permissions. Identify third-party components, data sources, and their licences. A publicly accessible source is not automatically redistributable.

OFRL does not require a separate CLA for ordinary contributions at this stage.

## Review approach

Reviews should focus on correctness, evidence, clarity, reproducibility, and scope. Disagreement is welcome when it is reasoned and supported. A review comment should identify the issue, explain its impact, and, where possible, suggest a test or correction.

Review the contribution, not the contributor's credentials, seniority, reputation, or personal background. The founder's work and assumptions are open to the same scrutiny as anyone else's. Reviewers should be specific about whether a concern is a correctness issue, an evidence gap, a difference in modelling assumptions, a trade-off, or a matter of project scope.

A contribution is not accepted simply because it is different or novel; nor should a sound alternative be rejected merely because it differs from the current implementation. Acceptance depends on the contribution's relevance, quality, rights, and maintainability. When a project needs to retain more than one approach, document the scope and trade-offs rather than treating disagreement itself as a defect.

## Conduct

Treat contributors and reviewers with respect. Critique methods and claims, not people. Harassment, discrimination, personal attacks, and deliberate misrepresentation are not acceptable.
