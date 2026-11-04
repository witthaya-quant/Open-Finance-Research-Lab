<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-ADR-2026-002: Founder-Controlled Open Source Model; Organization Not Required

> **Decision ID:** OFRL-ADR-2026-002  
> **Status:** Accepted  
> **Decision date:** 2026-11-04  
> **Decision owner:** Kenny (Witthaya Maidee), Founder / Principal Maintainer  
> **Author:** Kenny (Witthaya Maidee)  
> **Reviewer:** No independent reviewer; approved by the Founder during Stage 0  
> **Scope:** Lab-wide  
> **Affected projects / components:** All OFRL repositories, publications, contribution processes, and publication boundaries  
> **Supersedes:** The mandatory/progressive Organization-transition assumption in OFRL-ADR-2026-001; see also [OFRL-ADR-2026-001](OFRL-ADR-2026-001-founder-led-single-maintainer-governance.md)  
> **Related policies:** [GOVERNANCE.md](../GOVERNANCE.md), [PUBLICATION_AND_IP_BOUNDARY.md](../PUBLICATION_AND_IP_BOUNDARY.md)

## 1. Context

OFRL is currently maintained under the Founder's personal GitHub account. The Founder intends to develop it independently before inviting external contributors. The long-term objective is to enable useful collaboration while retaining a clear Founder-led research direction, publication standard, and accountability model.

A GitHub Organization was initially described as a possible later destination. Further analysis shows that this is not necessary to meet the core objective. A public personal-account repository can accept Issues, fork-based Pull Requests, external reviews, and selected collaborators. An Organization offers more granular repository roles and centralized administration, but those capabilities are not required merely to make a public project collaborative.

The Founder also wants the public research repository to remain reusable while keeping unreleased strategies, sensitive implementation details, restricted data, and other commercially confidential know-how private.

The term "proprietary open source" is not adopted because it can misdescribe a source-available or restricted licence as open source. OFRL must describe each release according to its actual licence.

## 2. Decision drivers

- Keep the repository under the Founder's personal account unless a concrete operational need justifies transfer.
- Preserve Founder accountability for vision, direction, project admission, publication, and reserved decisions.
- Accept public contributions without granting unnecessary write or administrative access.
- Keep research collaboration open while protecting confidential and restricted information.
- Distinguish repository control, copyright ownership, and licence permissions.
- Preserve existing open-source and open-content commitments.
- Avoid custom licensing and contributor agreements without a defined legal or commercial need.
- Make any future organizational change optional, evidence-based, and reversible where practical.

## 3. Options considered

| Option | Advantages | Disadvantages / risks |
|---|---|---|
| Transfer to a GitHub Organization as the default future path | Granular roles, teams, centralized settings | Additional administration; changes repository account ownership and default permission context; not required for a small founder-led project |
| Use a source-available proprietary licence across the public repository | Could restrict some downstream uses | No longer OSI open source; reduces reuse and contribution incentives; custom licence compatibility and enforcement burden; does not protect abstract methods by copyright |
| Keep everything public under permissive licences | Easy reuse, transparent research, low contribution friction | Commercial users may reuse released material; confidential know-how cannot be recovered after disclosure |
| Founder-controlled public open-source repository with a separate private know-how boundary | Preserves public collaboration and Founder control while keeping sensitive work private; no Organization required | Requires disciplined publication classification and careful separation of public and private material |

## 4. Decision

Adopt the fourth option.

OFRL will operate as a **Founder-led Open Source Research Project under the Founder's personal GitHub account**. The repository may remain there indefinitely.

A GitHub Organization is optional. It will be considered only if a concrete requirement for granular team permissions, centralized administration, legal-entity ownership, or shared stewardship makes the personal-account model inadequate.

The public licensing baseline remains content-specific:

- Software, code, tests, schemas, and computational financial models: Apache-2.0.
- Research writing and documentation: CC BY 4.0.
- Data: source-specific rights, with original synthetic research data under the stated OFRL policy.

OFRL will maintain a clear boundary between:

- **Public Open Release:** material intentionally made available for public inspection, reuse, and contribution under its declared licence.
- **Private Research and Know-how:** unreleased methods, proprietary strategy details, production parameters, restricted datasets, and confidential information that must not be placed in the public repository.
- **Restricted Third-Party Material:** content that cannot be redistributed without the relevant rights and permissions.
- **Candidate Protected IP:** material requiring review before public disclosure where patent or other protection may be relevant.

The Founder remains the repository owner and final authority for reserved decisions. Contributors retain copyright in their own contributions and grant rights under the applicable published licence. No contributor copyright assignment or general right to relicense contributions under a proprietary licence is implied.

## 5. Rationale

The key requirement is control over the project and its publication decisions, not ownership of a GitHub Organization. A personal account already provides a clear owner. Public contributors can submit work through forks and Pull Requests without direct repository write access.

GitHub's personal-account permission model is less granular than an Organization: it distinguishes the owner and collaborators, and collaborators can push changes. This makes fork-based contribution the safer default. A small number of trusted people may receive collaborator access if they need it.

The existing Apache-2.0 and CC BY 4.0 model already supports public reuse, including commercial reuse under their respective terms. Replacing those licences with a custom restrictive licence would change the project's nature and likely weaken the stated collaboration objective. The Founder should protect commercial advantage by deciding what not to publish, not by describing a restrictive licence as open source.

Copyright does not by itself protect underlying ideas, procedures, methods of operation, or mathematical concepts. Trade-secret protection depends on confidentiality and reasonable secrecy measures. Therefore, the publication boundary is a central control, not an optional administrative note.

## 6. Consequences and trade-offs

### Positive consequences

- No planned or mandatory GitHub Organization migration.
- Founder retains clear control over the repository and official project decisions.
- External contributors can participate through public Issues and fork-based Pull Requests.
- Publicly released work remains genuinely open under its stated licences.
- Commercially sensitive know-how can remain private without restricting the entire public repository.
- No immediate CLA, CAA, or custom proprietary licence is required.
- Governance remains valid whether the project has one contributor or a larger community.

### Risks and mitigations

| Risk | Mitigation |
|---|---|
| Personal-account permission model lacks granular roles | Default to fork PRs; grant collaborator write access only to trusted maintainers who need it |
| Founder becomes a single point of failure | Maintain account security, recovery methods, periodic backups, and documented procedures; appoint a trusted maintainer only when justified |
| Accidental disclosure of proprietary know-how | Apply the publication classification and pre-publication gate before committing or releasing material |
| Contributors misunderstand ownership | State clearly that repository ownership does not transfer copyright in their contributions |
| Public work is reused commercially by others | Treat this as an intentional consequence of Apache-2.0 / CC BY 4.0; keep non-public commercial advantage outside the public release boundary |
| Future dual licensing becomes difficult | Decide before accepting contributions intended for dual-licensed distribution; use an appropriate, legally reviewed contributor agreement prospectively |

## 7. Validation and acceptance criteria

| Item | Acceptance criterion | Status |
|---|---|---|
| No mandatory Organization | Governance states personal account is the default and Organization is optional | Documented |
| Personal-account contribution model | Fork-based PR workflow and collaborator access limitations are documented | Documented |
| Public/private IP boundary | Publication classifications and pre-release gate exist | Documented |
| Licence consistency | Existing Apache-2.0 / CC BY 4.0 allocation remains unchanged | Preserved |
| Contributor rights | No automatic copyright assignment or proprietary relicensing right is implied | Documented |
| Decision history | Earlier Organization-transition assumption is marked superseded, not erased | To be completed in this PR |
| Repository controls | Branch protection baseline is documented; GitHub settings still require configuration and verification | Not yet configured |

This Decision Record approves the strategic model. It does not itself configure GitHub settings or provide jurisdiction-specific legal advice.

## 8. Conflicts of interest and attribution

The decision was proposed and approved by the Founder, who has a direct interest in OFRL's direction and publication strategy. This is disclosed. No independent reviewer was available during Stage 0.

## 9. Revisit conditions

Reconsider this decision if:

- Several Maintainers need distinct GitHub permissions that cannot be represented safely under the personal-account model.
- Organization-level administration becomes materially necessary across multiple repositories.
- A legal entity or institutional arrangement must own the repository.
- OFRL intends to adopt dual licensing, a source-available proprietary licence, a CLA, or a CAA.
- A significant commercial partnership or funding arrangement changes the intellectual-property or confidentiality requirements.
- The risk profile or applicable law changes materially.

## 10. Change history

| Date | Change | Author |
|---|---|---|
| 2026-11-04 | Initial decision accepted | Kenny (Witthaya Maidee) |
