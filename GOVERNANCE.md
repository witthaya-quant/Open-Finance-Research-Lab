<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Governance

**Policy ID:** OFRL-GOV-001  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Policy owner:** Founder / Principal Maintainer  
**Applies to:** The OFRL repository, projects, publications, contributors, maintainers, and work released under the OFRL name

## 1. Purpose

This policy defines who is accountable for OFRL, how decisions are made, how work is reviewed and released, how disagreements are handled, and how the lab can move from a founder-maintained personal repository to an organization-owned research community without losing its principles or decision history.

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
- Decisions concerning official project status, publication, licensing policy, repository transfer, and the use of the OFRL name.
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
- At least one eligible independent Maintainer approval is required for material changes before merge, once an independent Maintainer is appointed and available.
- High-impact changes may require two independent reviews or a domain-specific review, as defined by the project risk classification.
- The Founder retains final authority over lab-wide policies, project admission, official publications, licensing policy, repository transfer, and release decisions unless explicitly delegated.
- A project may retain multiple methods when they are useful for different contexts or the evidence does not justify a single choice.

The Founder may accept useful contributions before appointing additional Maintainers. In that case, the Founder remains responsible for review and merge decisions; contributor participation alone does not create Maintainer authority.

### Stage 2 — Organization-owned research lab

Activated when OFRL is transferred to a GitHub Organization and the Founder records the transition.

- The Organization owns the repository as a GitHub account resource; the Founder remains the research and governance lead unless the governance policy is formally amended.
- Organization Owners manage organization-level administration. Organization ownership does not automatically confer authority to approve research conclusions or change research methodology.
- Repository and project access are granted through teams and least-privilege repository roles wherever practical.
- At least two trusted Organization Owners should be established to maintain ownership continuity. Organization Owner access is reserved for people who genuinely need organization-wide administrative authority.
- The organization maintains a named Founder / Research Director, repository Maintainers, project Leads, and review responsibilities.
- Organization-level rulesets and repository controls should enforce the applicable baseline in [GITHUB_REPOSITORY_SETTINGS.md](GITHUB_REPOSITORY_SETTINGS.md).
- The lab periodically reviews membership, access, maintainership, security, conflicts of interest, and governance effectiveness.

A GitHub Organization is an account and access-management structure. Creating or using one does not by itself incorporate a legal entity, transfer copyright ownership, create a partnership, or change the licence terms already granted to users.

## 4. Decision rights and reserved matters

The following table defines the default decision authority.

| Decision | Stage 0: Founder-maintained | Stage 1: Contributors | Stage 2: Organization |
|---|---|---|---|
| Research proposal triage | Founder | Maintainer; Founder for strategic scope | Project Maintainer; Founder for strategic scope |
| Technical implementation within approved scope | Founder | Project Lead / Maintainer | Project Lead / Maintainer |
| Research conclusion and evidence maturity | Founder, with limitations disclosed | Responsible research authors; Maintainer checks standards | Research authors and designated reviewers; publication authority remains defined |
| Merge routine contribution | Founder | Eligible Maintainer after required review | Eligible Maintainer under branch rules |
| Change lab-wide Research Standards | Founder, via documented change | Founder, after consultation and recorded decision | Governance process and reserved Founder authority until formally amended |
| Change licence, data policy, disclosure boundary | Founder, with rights review | Founder approval required | Founder / authorized governance role; legal review where appropriate |
| Official publication or release under OFRL name | Founder | Founder or explicitly delegated release authority | Designated release authority under documented delegation |
| Repository transfer or Organization settings | Founder / account owner | Founder / account owner | Organization Owners under governance |
| Change mission, vision, or governance constitution | Founder, through versioned amendment | Founder, after documented consultation | Formal governance amendment; not a routine Maintainer action |

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
- Governance roles, decision rights, review requirements, repository ownership, or organization transition.
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
5. During Stage 1 and Stage 2, branch rules and this policy determine the minimum independent approvals.
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

## 11. Transition from personal repository to GitHub Organization

The repository should remain under the founder's personal account while it is being developed independently and the Organization structure adds no operational benefit. It should not be recreated merely to look institutional.

Consider transfer when at least one of these conditions is met:

- Two or more people have ongoing responsibilities that require coordinated team permissions.
- Multiple active projects have distinct Maintainers or project Leads.
- The lab needs centralized membership, repository, security, or policy administration.
- An institutional partnership, grant, sponsorship, or operating model requires the Organization to be the repository owner.

Before transfer:

1. Decide the Organization's name, purpose, visibility, billing owner, and membership policy.
2. Identify at least two trusted people for Organization Owner continuity where feasible; do not grant Owner access solely for convenience.
3. Define teams, base repository permissions, outside-collaborator policy, and least-privilege roles.
4. Review licensing, contributor attribution, data restrictions, repository secrets, Actions, webhooks, deploy keys, Pages, packages, and external links.
5. Confirm that the target Organization can receive the repository and that the repository name is available there.
6. Record the transfer as a Decision Record and announce the canonical repository location.

Use GitHub's repository transfer process rather than recreating the repository. After transfer, verify issues, Pull Requests, releases, branches, workflows, permissions, protections, integrations, links, and local Git remotes. GitHub documents that repository transfers preserve core Git history and several repository assets, while organization defaults apply to the transferred repository; individual details must still be checked.

The transition must preserve existing licences, contributor attribution, research history, Decision Records, and the founder's stated accountability unless they are explicitly changed through a separate governance amendment.

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

Review this policy at least annually and whenever OFRL changes governance stage, repository ownership, licensing model, or material risk profile. A review may conclude that no change is required; record material conclusions in the decision register.

## 14. Related policies and references

- [Foundation, Vision, Mission & Principles](FOUNDATION_AND_DIRECTION.md)
- [Research Charter](RESEARCH_CHARTER.md)
- [Research Standards](RESEARCH_STANDARDS.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [GitHub Repository Settings](GITHUB_REPOSITORY_SETTINGS.md)
- [Decision Register](decisions/README.md)
- [Decision Record Template](templates/DECISION_RECORD_TEMPLATE.md)
