<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL GitHub Repository Settings Baseline

**Policy ID:** OFRL-GH-001  
**Version:** 1.0.0  
**Last reviewed:** 2026-11-04

This document translates OFRL Governance into GitHub repository controls for a repository that remains under the Founder's personal GitHub account. A GitHub Organization is optional. A policy written in Markdown does not itself activate a GitHub setting; settings must be configured and verified in GitHub.

## 1. Current observed state

As checked on 2026-11-04 through the GitHub repository and Rulesets APIs:

| Control | Observed state |
|---|---|
| Repository owner | Personal account: `witthaya-quant` |
| Default branch | `main` |
| Repository visibility | Public |
| Main branch ruleset | `OFRL-main-protection` — Active; ruleset ID `26516806` |
| Ruleset target | `~DEFAULT_BRANCH`, currently resolving to `main` |
| Pull Request requirement | Enabled; 0 required approvals; conversation resolution required |
| Deletion / force-push protection | Enabled |
| Bypass actors | None; API reports current user bypass as `never` |
| Up-to-date requirement | Enabled (`strict_required_status_checks_policy: true`) |
| Required status checks | **Exactly six** GitHub Actions contexts; all six are bound to integration ID `15368` and passed on the PR #14 / PR #18 heads checked |
| Code Quality / Copilot Review | Both disabled; neither appears in the current Ruleset response |
| Merge methods | Merge commit, rebase, and squash enabled |
| Repository permissions available to current account | Admin, maintain, push, pull, triage |

The Ruleset is active and its Stage 0 configuration has passed the API read-back: exactly six required check contexts are stored, each bound to GitHub Actions integration ID `15368`; no malformed extra context remains. All six checks passed on the PR #14 head targeting `main` and the PR #18 audit head. The documentation-only smoke test should be repeated after the workflow changes merge to `main`, but the Ruleset configuration itself is complete.

This is a snapshot, not a guarantee that settings remain unchanged. Recheck after configuration changes and after the repository moves to an Organization.

## 2. Stage 0 baseline — one active maintainer

Configure a branch protection rule or repository ruleset targeting the exact default branch, currently `main`.

| Setting | Stage 0 baseline | Reason |
|---|---|---|
| Require a Pull Request before merging | **Enable** | Prevent routine direct changes to the default branch |
| Required approving reviews | **0 / disabled** | A sole maintainer cannot obtain an independent approval from another person; do not create a false review control |
| Require conversation resolution | **Enable** | Keep review discussions resolved before merge |
| Require status checks | **Enable and require the six exact GitHub Actions check names listed in Section 6** | All current workflows now run on every Pull Request, so the required checks are stable for documentation and code changes |
| Require branch to be up to date | **Enable** after all six required checks are selected | Require testing against the latest `main` commit; all six checks now run for every Pull Request |
| Allow force pushes | **Disable** | Preserve default-branch history |
| Allow deletions | **Disable** | Prevent accidental default-branch deletion |
| Require signed commits | Optional; enable only after the signing workflow is adopted and tested | Avoid an unworkable control that causes accidental lockout |
| Require linear history | Optional; decide alongside merge-method policy | Do not enable without confirming the chosen merge workflow |
| Code-owner review | Not applicable until a meaningful `CODEOWNERS` file and eligible owners exist | Avoid a nominal control with no reviewer coverage |
| Bypass list | **Leave empty**; do not grant a bypass role, user, or app | Keep the same merge gate for the repository owner and collaborators; verify the owner is not exempt after saving |

The Founder may merge a Founder-authored PR under Stage 0 because independent review is not available. The PR checklist and validation evidence remain mandatory. This must be described as a single-maintainer decision, not an independent approval.

## 3. Stage 1 baseline — invited contributors under the personal account

- Keep Pull Requests required for `main`.
- Continue to accept external work through forks and Pull Requests; a public repository does not require every contributor to become a repository collaborator.
- Keep direct collaborator access limited to people who genuinely need write access and are trusted with it.
- When an eligible independent reviewer has write permission, require one approval for material changes where the branch rule can be applied reliably. Do not configure a required approval when no eligible reviewer exists.
- Dismiss stale approvals when new commits materially change the reviewed diff.
- Require conversation resolution.
- Prevent force pushes and deletion of the default branch.
- Do not use a bot, author, or nominal account to simulate an independent reviewer.

On a personal-account repository, GitHub has two permission levels: the owner and collaborators. Collaborators can read and push changes; private repositories owned by personal accounts cannot grant collaborators read-only access. Therefore, do not invite a person as a collaborator merely because they want to propose research or review a public Pull Request. Use fork-based contribution instead. See [GitHub's personal repository permission model](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/repository-access-and-collaboration/permission-levels-for-a-personal-account-repository).

For high-risk changes—licensing, data release, security boundaries, core financial calculations, numerical precision, public model claims, and governance—seek appropriate domain review. The Founder remains accountable for the final merge unless authority is explicitly delegated.

## 4. Stage 2 baseline — delegated Maintainers under the personal account

- Apply the same default-branch controls as Stage 0 and Stage 1.
- Appoint Maintainers by documented scope, not by informal title alone.
- Grant collaborator write access only to trusted Maintainers who need it; do not use shared accounts.
- Keep ordinary Contributors on fork-based Pull Requests.
- Record which projects or technical domains each Maintainer may review and merge.
- Keep policy, licensing, data-release, account ownership, and official publication decisions reserved to the Founder unless explicitly delegated.
- Review collaborator access and delegated authority at least annually and whenever a role changes.

This stage can continue indefinitely. If the personal-account permission model remains adequate, no Organization is required.

## 5. Optional Stage 3 baseline — Organization-owned repository

Use this only after a documented decision establishes a concrete need for centralized administration or finer permissions:

- Apply Organization rulesets to the default branch and release tags where appropriate.
- Use teams and least-privilege repository roles rather than broad member permissions.
- Keep Organization Owner membership limited to people who need organization-wide control; establish continuity without granting Owner access solely for convenience.
- Restrict repository creation, deletion, transfer, and visibility changes to authorized roles.
- Review Actions permissions, default token permissions, secrets, variables, environments, webhooks, deploy keys, packages, Pages, and third-party Apps.
- Enable appropriate security features available under the Organization's plan.
- Review access periodically and immediately after a Maintainer leaves or changes role.
- Record material Organization-level exceptions and changes.

## 6. Required check selection procedure

Do not guess check names.

1. Open representative Pull Requests for code, schema, fixture, and documentation changes.
2. Confirm which workflows run for each path and event.
3. Inspect the exact check-run names reported by GitHub.
4. Confirm that a check runs reliably for every Pull Request to which it will apply and does not depend on secrets unavailable to forked PRs.
5. Select only the exact check names that are stable and applicable.
6. Test a failing check and confirm it blocks merge; test a passing check and confirm the control behaves as intended.

**Current OFRL status (2026-11-04):** both OFRL-IB-001 project workflows have been changed to run on every Pull Request (their `paths` filters have been removed). The six exact check-run names were observed on the latest successful PR #18 head and on PR #14, which targets `main`. All six are reported by the GitHub Actions app and completed successfully.

The Rulesets API read-back at 2026-11-04 confirmed **exactly six required entries**, each with integration ID `15368` (GitHub Actions), and no other required context:

- `repository-policy`
- `Model input JSON Schema`
- `Decimal contract (Rust 1.85.0)`
- `Decimal contract (Rust 1.99.1)`
- `Semantic validator (Rust 1.85.0)`
- `Semantic validator (Rust 1.99.1)`

All six exact check runs were present and successful on the PR #14 head targeting `main` and on the PR #18 audit head. The malformed extra context previously observed has been removed. Revalidate after any workflow-name or Ruleset changes.

**Post-merge smoke test:** after PR #18 and the resulting PR #14 changes are merged to `main`, create or use a documentation-only PR to verify that all six required checks still report and complete. This has not yet been performed on `main`.

GitHub documents that checks skipped by path or branch filtering remain pending and can block merges; see [Skipping workflow runs](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/skip-workflow-runs) and [Troubleshooting required status checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks).

A workflow that runs only on pushes to the default branch is not sufficient evidence that it protects Pull Requests.

## 7. Configuration and verification record

Maintain a short record when repository settings are changed:

| Field | Record |
|---|---|
| Date | 2026-11-04 |
| Changed by | Repository owner (user-reported configuration; actor identity is not returned by the Rulesets API) |
| Branch or tag target | `~DEFAULT_BRANCH` (`main`) |
| Rule or ruleset name | `OFRL-main-protection` / ID `26516806` |
| Settings enabled | Active; Pull Request required; 0 approvals; conversation resolution; strict up-to-date; deletion restriction; force-push block; no bypass actors |
| Required check names | Exactly six; all bound to GitHub Actions integration ID `15368` |
| Verification PR | PR #14 targeting `main`; PR #18 validates all six workflow checks on the audit branch |
| Exceptions and expiry / review date | No configuration exception. Repeat a documentation-only PR smoke test after the workflow changes are merged to `main`. |

Review the settings after changing the default branch, adding or changing workflows, moving to an Organization, changing repository visibility, or changing Maintainer roles.

## 8. How to configure

For branch protection, open the repository's **Settings → Branches** and add a branch protection rule for `main`. For rulesets, open **Settings → Rules → Rulesets** and create a branch ruleset targeting the default branch. These controls are available for public repositories on GitHub Free. GitHub's interface may change; verify the final rule's enforcement status after saving.

Official references:

- [About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [Managing a branch protection rule](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/managing-a-branch-protection-rule)
- [Available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)
- [Repository roles for an organization](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization)
- [Roles in an organization](https://docs.github.com/en/organizations/managing-peoples-access-to-your-organization-with-roles/roles-in-an-organization)
- [Transferring a repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository)

## 9. Important limitation

This policy documents the required configuration and verification process. Repository settings must be changed through GitHub's repository settings by the account owner or an authorized administrator. Organization controls apply only if OFRL later elects to transfer the repository. A Pull Request that adds this document does not enable branch protection, required reviews, or other platform controls by itself.
