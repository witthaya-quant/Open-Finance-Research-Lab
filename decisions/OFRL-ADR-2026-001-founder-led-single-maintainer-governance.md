<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-ADR-2026-001: Founder-Led Governance and Staged Organization Transition

> **Decision ID:** OFRL-ADR-2026-001  
> **Status:** Accepted  
> **Decision date:** 2026-11-04  
> **Decision owner:** Kenny (Witthaya Maidee), Founder / Principal Maintainer  
> **Author:** Kenny (Witthaya Maidee)  
> **Reviewer:** No independent reviewer; approved by the Founder during Stage 0  
> **Scope:** Lab-wide  
> **Affected projects / components:** All OFRL repositories, projects, publications, and contribution processes  
> **Supersedes:** None  
> **Related policy:** [GOVERNANCE.md](../GOVERNANCE.md)

## 1. Context

OFRL is currently developed under the Founder's personal GitHub account. The Founder intends to develop the research programme independently for an initial period and invite contributors later. At that point, the intention is to move the repository and its operating structure to a GitHub Organization.

The Founder wants professional governance, decision traceability, review expectations, and repository controls to exist from the beginning rather than being introduced after a contributor community has already formed.

The governance model must be workable for one active maintainer and must not create fictitious independent reviews or approval controls that cannot be satisfied.

A GitHub Organization is an account and access-management structure. It is not, by itself, a legal entity or a transfer of copyright ownership.

## 2. Decision drivers

- Preserve a clear Founder-led research direction and accountability.
- Make decisions and material changes auditable from the first day.
- Retain openness to alternative methods and well-reasoned challenges.
- Apply the same research and conduct standards to the Founder and contributors.
- Avoid unnecessary administrative overhead while work is maintained by one person.
- Make later team-based access and governance possible without rewriting the repository's history.
- Preserve contributor attribution, existing licences, data rights, and research decisions through any ownership transition.

## 3. Options considered

| Option | Advantages | Disadvantages / risks |
|---|---|---|
| Delay formal governance until contributors arrive | Low initial administration | Early decisions become implicit; standards may be inconsistent; future contributors inherit unclear authority and history |
| Create a full multi-person Organization governance structure immediately | Institutional structure from the outset | Creates roles and review requirements before there are people to fill them; risks nominal controls and unnecessary complexity |
| Adopt founder-led single-maintainer governance now, with explicit transition stages | Real controls are active from the start; responsibilities are clear; later expansion is documented | Founder remains a single point of operational and approval responsibility until trusted maintainers are appointed |

## 4. Decision

Adopt the third option.

OFRL will use a **Founder-led, evidence-driven governance model** with three defined stages:

- **Stage 0 — Founder-maintained:** one active Founder / Principal Maintainer; material changes use Pull Requests and documented validation; independent review is not claimed where none exists.
- **Stage 1 — Contributors and delegated Maintainers:** contributors submit through documented workflows; independent review is required once eligible Maintainers are appointed; reserved Founder decisions remain explicit.
- **Stage 2 — Organization-owned research lab:** the repository moves to a GitHub Organization when defined operational triggers justify it; team permissions, ownership continuity, and organization controls are established.

Adopt the following repository-wide instruments from the effective date:

- Governance policy and decision-rights matrix.
- Code of Conduct and Security Policy.
- GitHub repository settings baseline.
- Pull Request and Research Proposal templates.
- Central Decision Register and Decision Record template.

Project-specific Decision Logs remain authoritative for project-only decisions. Central records govern lab-wide and cross-project decisions.

## 5. Rationale

Delaying governance would allow the earliest decisions to become undocumented precedents. A full organization structure today would create administrative roles without actual role-holders. The staged model preserves accountability while remaining proportionate to the current operating reality.

The model treats the Founder as accountable, not infallible. Founder-authored work may be merged during Stage 0, but it must be identified as a single-maintainer decision and supported by applicable tests and evidence. When an independent Maintainer becomes available, the review standard changes accordingly.

## 6. Consequences and trade-offs

### Positive consequences

- Governance and decision history begin before external contributions.
- Founder responsibilities and reserved matters are explicit.
- Contributors will have a predictable route to propose, review, and challenge work.
- The lab can scale permissions and review controls without changing its underlying principles.
- Project-specific decision logs can coexist with a central lab-wide decision register.

### Negative consequences and risks

- The Founder remains a single point of failure for repository administration and approval during Stage 0.
- A Pull Request workflow adds some overhead even for founder-authored changes.
- Written policies require periodic review to avoid becoming stale or inconsistent.
- A GitHub Organization transfer will require a deliberate permissions, integrations, billing, and settings review.

## 7. Validation and acceptance criteria

| Item | Acceptance criterion | Result |
|---|---|---|
| Governance policy | Defines roles, stages, reserved decisions, review, conflicts, escalation, and transition | Documented |
| Decision traceability | Provides central and project-level record boundaries and stable IDs | Documented |
| Contribution workflow | Provides reusable PR and research proposal intake templates | In progress until files are merged |
| Conduct and security | Provides reporting and enforcement routes appropriate to a solo maintainer | Documented; platform configuration remains to be verified |
| Branch controls | Defines Stage 0, Stage 1, and Stage 2 settings | Documented; not yet enabled on GitHub |
| Organization transition | Defines triggers, preparation, transfer, and post-transfer checks | Documented |

This decision approves the governance model. It does not establish that GitHub branch protections, security features, or Organization settings have already been configured.

## 8. Conflicts of interest and attribution

The decision was authored and approved by the Founder, who has a direct interest in OFRL's direction and governance. This is disclosed. No independent review was available during Stage 0.

## 9. Revisit conditions

Reconsider this decision when:

- The Founder invites the first ongoing external contributor.
- An independent Maintainer is appointed.
- Two or more people require continuing repository or project permissions.
- Multiple active projects need delegated project leadership.
- An institutional partnership, grant, sponsorship, or operating model requires organization ownership.
- A security, legal, licensing, data-rights, or operational event materially changes OFRL's risk profile.

## 10. Change history

| Date | Change | Author |
|---|---|---|
| 2026-11-04 | Initial decision accepted | Kenny (Witthaya Maidee) |
