<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Governance

**Policy ID:** OFRL-GOV-001  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Policy owner:** Founder / Principal Maintainer  
**Applies to:** The OFRL repository, projects, publications, contributors, maintainers, and work released under the OFRL name

## 1. Purpose

This policy defines who is accountable for OFRL, how decisions are made, how work is reviewed and released, how disagreements are handled, and how the project can accept contributors while remaining under the Founder's personal GitHub account. A GitHub Organization is an optional administrative choice, not a required destination or a measure of maturity.

OFRL adopts governance from its first day of operation. The requirements apply now, even while the founder is the only active maintainer. The process is deliberately proportionate: it creates traceability and safeguards without pretending that independent review exists when only one person is available.

The lab's purpose and values are defined in [FOUNDATION_AND_DIRECTION.md](FOUNDATION_AND_DIRECTION.md) and [RESEARCH_CHARTER.md](RESEARCH_CHARTER.md). This document defines the operating authority and controls that put those commitments into practice.

## 2. Governance model

OFRL is a founder-led, evidence-driven research initiative. It is not a democracy in which every repository participant has identical authority, and it is not a personal workspace in which decisions are exempt from explanation.

Authority is assigned by responsibility. Participation, technical influence, repository permissions, and ownership are distinct.

### 2.1 Founder / Principal Maintainer

While OFRL is maintained under the founder's personal account, Kenny (Witthaya Maidee) is the Founder and Principal Maintainer.

The Founder is accountable for:

- The lab's vision, mission, strategic direction, and research priorities.
- Accepting, declining, or suspending projects published under the OFRL name.
- Maintaining the integrity of the Research Charter, Research Standards, governance, and disclosure policies.
- Final repository-level approval of material changes and releases during the single-maintainer stage.
- Decisions concerning official project status, publication, licensing policy, repository ownership or account structure, and the use of the OFRL name.
- Ensuring that contributors receive accurate attribution and that material corrections are not concealed.

The Founder is not exempt from the lab's research, evidence, conduct, security, or disclosure standards. The Founder's work and decisions may be questioned and corrected through the same documented process as other work.

### 2.2 Maintainer

A Maintainer is a person explicitly appointed by the Founder to administer a repository or a defined project area.

A Maintainer may, within the delegated scope:

- Triage issues and research proposals.
- Coordinate technical discussion and reviews.
- Review and merge contributions when the applicable approval requirements are met.
- Maintain project documentation, labels, milestones, and release readiness.
- Request revisions or recommend that a contribution be declined.

A Maintainer does not automatically have authority to change the lab's mission, grant ownership of another person's intellectual property, change repository licensing, publish restricted data, transfer a repository, or override a mandatory research or security gate. Such reserved decisions require the Founder's explicit approval and a Decision Record where material.

Maintainer appointments must identify the repository or project scope, granted permissions, effective date, and any limitations. Access should follow least privilege and should be reviewed when responsibilities change.

### 2.3 Contributor

A Contributor proposes, researches, implements, documents, tests, reviews, or otherwise makes a substantive contribution.

Contributors retain copyright in their original work, subject to the licences they grant under the repository's [LICENSE_POLICY.md](LICENSE_POLICY.md) and the terms applicable to third-party material. Contribution does not automatically grant repository write, merge, release, or governance authority.

Contributors are expected to disclose relevant conflicts of interest, identify external sources, respect data and intellectual-property rights, and accurately describe their work.

### 2.4 Reviewer

A Reviewer evaluates a contribution within an identified area of competence. A review is evidence and advice for the decision-maker; it is not an ownership claim or a personal ranking.

Reviewers should identify whether a concern relates to correctness, evidence, implementation, risk, scope, assumptions, maintainability, or policy. A reviewer should explain the impact and, where possible, identify a test or a route to resolution.

### 2.5 Project Lead

A Project Lead coordinates the technical or research direction of a defined project. The role may be assigned to a Maintainer or another contributor.

Project Leads may propose project decisions and coordinate implementation. They cannot lower the lab-wide minimum standards or change a reserved governance decision without the required approval.

## 3. Governance stages

The lab moves through the following stages. A stage changes only when the Founder records the change; it does not change automatically because a contributor opens an issue or pull request.

### Stage 0 — Founder-maintained

**Current operating stage.**

- The Founder is the sole active Maintainer and final repository decision-maker.
- Issues, research proposals, material changes, and releases are recorded through GitHub.
- Material changes should use a topic branch and Pull Request, including changes authored by the Founder.
- The Founder may review and merge their own Pull Request when no independent Maintainer exists. This is disclosed as a single-maintainer review, not represented as independent review.
- Tests, validation evidence, risks, and unresolved limitations must be recorded at a level proportionate to the change.
- No approval count is fabricated to satisfy a process designed for a multi-person team.

### Stage 1 — Invited and open contributors

Activated when the Founder begins accepting contributions from other people.

- Contributors submit work through Issues, Discussions where enabled, and Pull Requests.
- Contributors do not approve their own Pull Requests.
- A person who authors or materially pushes a change must not provide the independent approval required for that change.
- When an independent Maintainer with sufficient GitHub permission is available, material changes should receive at least one independent approval before merge.
- On a personal-account repository, GitHub collaborator permissions are comparatively coarse. A person who only needs to contribute should normally use a fork and Pull Request; direct collaborator access should be reserved for trusted people who genuinely need write access.
- If no eligible independent reviewer is available, the Founder records the available review evidence and remains accountable for the merge. The process must not claim independent approval where none exists.
- High-impact changes should receive an additional domain-specific review where qualified review capacity is available.
- The Founder retains final authority over lab-wide policies, project admission, official publications, licensing policy, repository ownership or account structure, and release decisions unless explicitly delegated.
- A project may retain multiple methods when they are useful for different contexts or the evidence does not justify a single choice.

The Founder may accept useful contributions before appointing additional Maintainers. In that case, the Founder remains responsible for review and merge decisions; contributor participation alone does not create Maintainer authority.

### Stage 2 — Delegated Maintainers under the Founder's personal account

This stage can continue indefinitely. It does not require a GitHub Organization.

- The Founder remains the sole repository owner and retains the reserved decision rights in this policy.
- Trusted Maintainers may be appointed for defined projects or technical domains, with their responsibilities, scope, and effective dates recorded.
- Contributors who do not need write access continue to contribute through forks and Pull Requests.
- Direct collaborator access is granted only where the person needs repository write capability and is trusted with that level of access. On a personal-account repository, GitHub provides the owner and collaborators as the repository permission levels; it does not provide the same granular repository-role model available to Organizations.
- Branch protection and Pull Request controls remain in force. A Maintainer's review authority and technical scope must be documented separately from their GitHub permission.
- The Founder reviews access and delegated responsibilities periodically and when a role changes.

The project does not need to transfer its repository or intellectual-property rights merely because it has more contributors or Maintainers.

### Optional Stage 3 — GitHub Organization

An Organization may be considered only when a concrete administrative, access-control, stewardship, or legal-entity requirement cannot be handled proportionately under the personal-account model. It is not an automatic next stage.

Potential triggers include:

- Several Maintainers require different repository permissions that cannot be safely represented by the personal-account collaborator model.
- Multiple repositories need centralized team permissions, security administration, policy enforcement, or billing.
- A legal entity, institutional sponsor, grant, or contractual arrangement must become the repository owner.
- Continuity and stewardship need to be shared formally across multiple trusted Organization Owners.

Do not create an Organization simply to look more professional or because the contributor count has increased. If no material operational need exists, keep the repository under the Founder's account.

A GitHub Organization is an account and access-management structure. Creating or using one does not by itself incorporate a legal entity, transfer copyright ownership, create a partnership, or change licence terms already granted to users.

## 4. Decision rights and reserved matters

The following table defines the default decision authority.

| Decision | Stage 0: Founder-maintained | Stage 1: Contributors | Stage 2: Delegated Maintainers | Optional Stage 3: Organization |
|---|---|---|---|---|
| Research proposal triage | Founder | Founder | Delegated Maintainer; Founder for strategic scope | Project Maintainer; Founder for strategic scope |
| Technical implementation within approved scope | Founder | Founder | Project Lead / Maintainer within delegated scope | Project Lead / Maintainer within delegated scope |
| Research conclusion and evidence maturity | Founder, with limitations disclosed | Founder, with external review where available | Responsible research authors; Maintainer checks standards | Research authors and designated reviewers; publication authority remains defined |
| Merge routine contribution | Founder | Founder | Founder or delegated Maintainer under branch rules | Eligible Maintainer under branch rules |
| Change lab-wide Research Standards | Founder, via documented change | Founder, after consultation and recorded decision | Founder, after consultation and recorded decision | Formal governance amendment; reserved Founder authority until amended |
| Change licence, data policy, disclosure boundary | Founder, with rights review | Founder approval required | Founder approval required | Authorized role under formal policy; rights review required |
| Official publication or release under OFRL name | Founder | Founder | Founder or explicitly delegated release authority | Designated release authority under documented delegation |
| Repository transfer or account-structure change | Founder / account owner | Founder / account owner | Founder / account owner | Organization Owners, if adopted |
| Change mission, vision, or governance constitution | Founder, through versioned amendment | Founder, after documented consultation | Founder, after documented consultation | Formal governance amendment; not a routine Maintainer action |

A person with GitHub write or admin permission does not gain every decision right listed in this table. Platform permissions must be limited to what the person's assigned role requires.

## 5. Decision Records and decision traceability

Material decisions must be recorded in the repository's central [decisions/](decisions/README.md) register or, where the decision is strictly project-specific, in that project's Decision Log.

Use the [Decision Record Template](templates/DECISION_RECORD_TEMPLATE.md).

A Decision Record is required for decisions that materially affect one or more of the following:

- Research scope, objectives, methodology, evaluation criteria, or evidence claims.
- Financial or mathematical conventions, model assumptions, data contracts, or numerical policies.
- System architecture, public interfaces, core dependencies, security boundaries, or compatibility commitments.
- Data provenance, licence interpretation, redistribution, privacy, or disclosure.
- Public release criteria, evidence maturity, material corrections, or deprecation.
- Governance roles, decision rights, review requirements, repository ownership, or account-structure changes.
- A material departure from an earlier accepted decision.

A Decision Record is normally not required for routine refactoring, typo fixes, formatting, or implementation details that remain within an accepted decision and do not alter the relevant risk or contract.

Each record should state the context, decision drivers, options considered (including doing nothing where relevant), selected decision, rationale, consequences and trade-offs, validation or evidence, status, owner, date, and any revisit trigger. A decision must not be rewritten to erase its history. Superseding decisions link to the earlier record.

### 5.1 Decision status

- **Proposed:** Under consideration; not an approved baseline.
- **Accepted:** Approved for the stated scope and version.
- **Rejected:** Considered and not selected; retain the rationale.
- **Superseded:** Replaced by a later Decision Record.
- **Deprecated:** No longer recommended for new work, but retained for historical or compatibility reasons.

An accepted decision is not proof that its implementation works. Implementation and validation status must be tracked separately.

### 5.2 Central and project-level records

The central register is for lab-wide or cross-project decisions. A project Decision Log is the operational record for decisions scoped to that project.

Project records should reference a central Decision Record when they implement, specialize, or depend on a lab-wide decision. Do not copy the full text of the same decision into multiple places; link to the authoritative record and record only project-specific consequences.

## 6. Review and approval controls

1. Material changes should be submitted in a Pull Request against the default branch.
2. A Pull Request must state the problem, proposed change, scope, evidence, tests, risks, data/IP implications, and relevant decision references.
3. The author must not approve their own Pull Request once another eligible reviewer is available.
4. During Stage 0, the Founder may merge their own Pull Request after completing the applicable checklist. The record must not imply independent review.
5. During Stage 1 and Stage 2, the Founder remains accountable for merge decisions; documented independent review is required when an eligible independent Maintainer is appointed and has the necessary GitHub permissions. Optional Stage 3 uses the branch rules and approvals defined in this policy.
6. Changes to licences, data-release permissions, security controls, governance, or material public claims require explicit review by the Founder or an authorized role.
7. Failing tests or research gates must be recorded and resolved, or the publication must clearly state the limitation. Acceptance criteria must not be weakened after observing a failed result merely to obtain a pass.
8. Emergency changes should be limited to restoring service, containing a security issue, or preventing material harm. Record the reason, scope, and follow-up review in a Pull Request or Decision Record as soon as practicable.

A review approval confirms only the scope the reviewer actually examined. It is not a blanket certification of a project's financial, mathematical, statistical, legal, or commercial validity.

## 7. Conflict of interest and impartial review

Contributors and reviewers should disclose financial, professional, commercial, academic, personal, or other interests that could reasonably be perceived as affecting a research proposal, model comparison, data selection, or conclusion.

A conflict does not automatically disqualify a person or invalidate their work. The conflict and any mitigation should be documented. Depending on the circumstances, mitigation may include disclosure, an additional independent review, restricted access to sensitive information, or recusal from a specific decision.

The Founder should disclose relevant conflicts on the affected project and seek independent review where a material conflict cannot be adequately managed through disclosure alone.

## 8. Disagreement, escalation, and corrections

Disagreement is expected in research. It should be handled in this order:

1. State the exact claim, assumption, method, or implementation in dispute.
2. Identify whether the disagreement is factual, mathematical, empirical, methodological, interpretive, or a matter of scope.
3. Provide a reproducible example, source, calculation, test, or other relevant evidence where possible.
4. Record unresolved alternatives and their trade-offs rather than forcing consensus.
5. Escalate policy, conduct, or publication disputes to the Founder or the designated governance contact.

Critique methods and claims, not people. Harassment, discrimination, retaliation, intimidation, and deliberate misrepresentation are not acceptable.

Material errors must be corrected openly. Preserve prior decision and publication history where practical, identify the affected conclusions, and explain whether downstream results are affected.

## 9. Research and publication gates

Governance approval cannot substitute for research validation. Projects must follow [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md), [DATA_POLICY.md](DATA_POLICY.md), and [LICENSE_POLICY.md](LICENSE_POLICY.md).

The lab distinguishes at least:

- Theoretical or mathematical soundness.
- Accounting or financial-model consistency, where applicable.
- Software implementation correctness.
- Data quality and rights.
- Statistical or empirical support.
- Robustness and sensitivity.
- Economic or operational usefulness.
- Independent replication.

A project may be published at an exploratory maturity level when it is clearly labelled and does not overstate its findings. A project must not claim independent replication without an independent person or team having performed it.

## 10. Repository administration and access

- Use the minimum GitHub permission needed for each role.
- Keep the default branch protected by the baseline in [GITHUB_REPOSITORY_SETTINGS.md](GITHUB_REPOSITORY_SETTINGS.md).
- Do not share personal credentials, tokens, private keys, or recovery codes.
- Use GitHub access controls, not shared accounts, to grant collaborator access.
- Review collaborator and Maintainer access when a role ends or changes.
- Restrict release, package, secret, environment, and repository-administration privileges to people who need them.
- Do not place credentials or restricted data in commits, Issues, Pull Requests, CI logs, or public artifacts.
- Record material changes to repository ownership, visibility, rulesets, or security settings.

## 11. Optional GitHub Organization and repository transfer

OFRL has no target date or obligation to become a GitHub Organization. The personal-account repository is the default operating model unless a concrete need justifies a change.

### 11.1 Decision test

Before creating an Organization or transferring the repository, confirm that at least one requirement cannot be handled adequately under the current account:

- Need for granular repository permissions across several Maintainers.
- Centralized management of multiple repositories, teams, security policies, or billing.
- A legal entity or institutional arrangement must become the repository owner.
- A documented continuity or stewardship requirement that justifies shared organization-level administration.

More contributors, greater popularity, or a desire for an institutional appearance are not sufficient reasons by themselves.

### 11.2 Transfer controls

If transfer is justified:

1. Record the decision and reasons in a new Decision Record; do not rewrite the historical record.
2. Decide the Organization's purpose, visibility, billing owner, membership policy, and access model.
3. Confirm that the Founder can create repositories in the target Organization and that its policies permit the transfer.
4. Identify trusted Organization Owners only where they genuinely need organization-wide control. If appropriate ownership continuity cannot be established, defer the transfer.
5. Review repository permissions, default organization permissions, Actions, secrets, environments, webhooks, deploy keys, Pages, packages, releases, links, and integrations.
6. Review contributor attribution, licences, data restrictions, and whether repository transfer could affect a sponsorship or external dependency.
7. Transfer the existing repository rather than creating a replacement; then verify issues, Pull Requests, releases, branches, workflows, permissions, protections, integrations, links, and local Git remotes.

GitHub supports repository transfer to an Organization. The receiving Organization's default repository permissions and membership privileges apply after transfer, so they must be reviewed deliberately. GitHub's current workflow is to move selected work into an Organization while retaining the personal account; do not plan around converting the personal account itself into an Organization.

The transition must preserve Git history, existing licences, contributor attribution, research history, Decision Records, and the Founder's stated governance responsibilities unless a separate, explicit amendment changes them.

### 11.3 No-transfer alternative

If the need is simply to receive ideas, review proposals, or accept code and documentation contributions, continue using public Issues and fork-based Pull Requests. The Founder can remain the repository owner and merge authority. No Organization is required.

## 12. Policy hierarchy and interpretation

The following controls must be read together:

1. Applicable law, contracts, and third-party terms govern what OFRL is legally permitted to use or disclose.
2. File-level licence notices and [LICENSE_POLICY.md](LICENSE_POLICY.md) govern reuse rights; [DATA_POLICY.md](DATA_POLICY.md) governs data provenance and release.
3. [SECURITY.md](SECURITY.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) govern security reporting and community conduct.
4. This Governance policy and the Research Charter govern authority, accountability, and lab-wide operating requirements.
5. [RESEARCH_STANDARDS.md](RESEARCH_STANDARDS.md) governs research quality and evidence requirements.
6. Project specifications and Decision Records define project-level choices within the requirements above.

A project-specific rule may be stricter than a lab-wide minimum. It may not silently waive a legal, licence, data, security, conduct, or lab-wide research requirement. If documents conflict, pause the affected action, identify the conflict, and record an amendment or clarification before proceeding.

## 13. Amendments and policy review

This policy is effective from the date shown above. Material amendments require a Pull Request and a Decision Record. The Founder approves amendments during Stage 0 and retains the reserved authority stated in this policy until a formal governance amendment changes it.

Review this policy at least annually and whenever OFRL changes governance stage, repository ownership or account structure, licensing model, or material risk profile. A review may conclude that no change is required; record material conclusions in the decision register.

## 14. Related policies and references

- [Foundation, Vision, Mission & Principles](FOUNDATION_AND_DIRECTION.md)
- [Research Charter](RESEARCH_CHARTER.md)
- [Research Standards](RESEARCH_STANDARDS.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [Publication and IP Boundary](PUBLICATION_AND_IP_BOUNDARY.md)
- [GitHub Repository Settings](GITHUB_REPOSITORY_SETTINGS.md)
- [Decision Register](decisions/README.md)
- [Decision Record Template](templates/DECISION_RECORD_TEMPLATE.md)
