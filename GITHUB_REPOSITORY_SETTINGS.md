<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL GitHub Repository Settings Baseline

**Policy ID:** OFRL-GH-001  
**Version:** 1.0.0  
**Last reviewed:** 2026-11-04

This document translates OFRL Governance into GitHub repository controls. A policy written in Markdown does not itself activate a GitHub setting. The settings below must be configured and then verified in the repository.

## 1. Current observed state

As checked on 2026-11-04 through the connected GitHub repository metadata:

| Control | Observed state |
|---|---|
| Repository owner | Personal account: `witthaya-quant` |
| Default branch | `main` |
| Repository visibility | Public |
| Main branch protection | Not enabled |
| Required status checks on main | None configured |
| Merge methods | Merge commit, rebase, and squash enabled |
| Repository permissions available to current account | Admin, maintain, push, pull, triage |

This is a snapshot, not a guarantee that settings remain unchanged. Recheck after configuration changes and after the repository moves to an Organization.

## 2. Stage 0 baseline — one active maintainer

Configure a branch protection rule or repository ruleset targeting the exact default branch, currently `main`.

| Setting | Stage 0 baseline | Reason |
|---|---|---|
| Require a Pull Request before merging | **Enable** | Prevent routine direct changes to the default branch |
| Required approving reviews | **0 / disabled** | A sole maintainer cannot obtain an independent approval from another person; do not create a false review control |
| Require conversation resolution | **Enable** | Keep review discussions resolved before merge |
| Require status checks | Enable only for stable, named checks that reliably run on every relevant PR | Avoid blocking all merges on checks that are absent, conditional, or unstable |
| Require branch to be up to date | Enable only after required checks are selected and reliable | Avoid an unusable or misleading gate |
| Allow force pushes | **Disable** | Preserve default-branch history |
| Allow deletions | **Disable** | Prevent accidental default-branch deletion |
| Require signed commits | Optional; enable only after the signing workflow is adopted and tested | Avoid an unworkable control that causes accidental lockout |
| Require linear history | Optional; decide alongside merge-method policy | Do not enable without confirming the chosen merge workflow |
| Code-owner review | Not applicable until a meaningful `CODEOWNERS` file and eligible owners exist | Avoid a nominal control with no reviewer coverage |
| Admin bypass | Keep restricted; document any emergency bypass and its reason | Preserve accountability without pretending a technical control removes all administrator power |

The Founder may merge a Founder-authored PR under Stage 0 because independent review is not available. The PR checklist and validation evidence remain mandatory. This must be described as a single-maintainer decision, not an independent approval.

## 3. Stage 1 baseline — independent contributors and maintainers

Before accepting ongoing external contributions:

- Keep Pull Requests required for `main`.
- Require at least one approval from an eligible person other than the PR author and the person who made the latest material push.
- Dismiss stale approvals when new commits materially change the reviewed diff.
- Require conversation resolution.
- Require stable CI checks relevant to the changed code and documentation.
- Enable Code Owner review only after `CODEOWNERS` reflects real, active domain ownership.
- Prevent force pushes and deletion of the default branch.
- Restrict direct write access to Maintainers; ordinary contributors should use forks or scoped write access where appropriate.
- Do not use a bot, author, or nominal account to simulate an independent reviewer.

For high-risk changes—licensing, data release, security boundaries, core financial calculations, numerical precision, public model claims, and governance—require an appropriate domain review. A second independent review is recommended where the impact is material and reviewer capacity allows.

## 4. Stage 2 baseline — Organization-owned repository

When the repository is transferred to a GitHub Organization:

- Apply Organization rulesets to the default branch and release tags where appropriate.
- Use teams and least-privilege repository roles rather than broad member permissions.
- Keep Organization Owner membership limited to people who need organization-wide control; establish at least two trusted Owners for continuity.
- Restrict repository creation, deletion, transfer, and visibility changes to authorized roles.
- Review Actions permissions, default token permissions, secrets, variables, environments, webhooks, deploy keys, packages, Pages, and third-party Apps.
- Enable appropriate security features available under the Organization's plan.
- Review access periodically and immediately after a Maintainer leaves or changes role.
- Record material Organization-level exceptions and changes.

GitHub organization roles and repository roles are separate. Organization Owner access should not be granted merely to allow someone to review or merge one repository.

## 5. Required check selection procedure

Do not guess check names.

1. Open a representative Pull Request that changes the relevant code.
2. Confirm that the intended workflow runs for that event and branch.
3. Inspect the exact check-run names reported by GitHub.
4. Confirm that the checks are stable, required for the affected scope, and do not depend on secrets unavailable to forked PRs.
5. Select the exact check names in the branch protection rule or ruleset.
6. Test a failing check and confirm it blocks merge; test a passing check and confirm the control behaves as intended.

A workflow that runs only on pushes to the default branch is not sufficient evidence that it protects Pull Requests.

## 6. Configuration and verification record

Maintain a short record when repository settings are changed:

| Field | Record |
|---|---|
| Date | YYYY-MM-DD |
| Changed by | GitHub user / role |
| Branch or tag target | |
| Rule or ruleset name | |
| Settings enabled | |
| Required check names | |
| Verification PR | |
| Exceptions and expiry / review date | |

Review the settings after changing the default branch, adding or changing workflows, moving to an Organization, changing repository visibility, or changing Maintainer roles.

## 7. How to configure

For branch protection, open the repository's **Settings → Branches** and add a branch protection rule for `main`. For rulesets, open **Settings → Rules → Rulesets** and create a branch ruleset targeting the default branch. GitHub's interface may change; verify the final rule's enforcement status after saving.

Official references:

- [About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [Managing a branch protection rule](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule)
- [Available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)
- [Repository roles for an organization](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization)
- [Roles in an organization](https://docs.github.com/en/organizations/managing-peoples-access-to-your-organization-with-roles/roles-in-an-organization)
- [Transferring a repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository)

## 8. Important limitation

This policy documents the required configuration and verification process. Repository settings must be changed through GitHub's repository or Organization settings by an authorized administrator. A Pull Request that adds this document does not enable branch protection, required reviews, or other platform controls by itself.
