<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL GitHub Repository Settings Baseline

**Policy ID:** OFRL-GH-001  
**Version:** 1.0.0  
**Last reviewed:** 2026-11-04

This document translates OFRL Governance into GitHub repository controls for a repository that remains under the Founder's personal GitHub account. A GitHub Organization is optional. A policy written in Markdown does not itself activate a GitHub setting; settings must be configured and verified in GitHub.

## 1. Current observed state

As checked on 2026-11-04 through repository metadata and the GitHub Rulesets API:

| Control | Observed state |
|---|---|
| Repository owner | Personal account: `witthaya-quant` |
| Default branch / visibility | `main` / Public |
| Active ruleset | `OFRL-main-protection`, ID `26516806` |
| Enforcement / target | Active; `~DEFAULT_BRANCH` (currently `main`) |
| Pull Request requirement | Enabled |
| Required approving reviews | 0 (Stage 0 single-maintainer policy) |
| Conversation resolution | Required |
| Required status checks | Exactly six GitHub Actions contexts listed below |
| Up-to-date requirement | Enabled (`strict_required_status_checks_policy: true`) |
| Branch deletion | Blocked |
| Non-fast-forward / force-push | Blocked |
| Bypass actors | None; API reports current user bypass as `never` |
| Code Quality / Copilot Review rules | Not configured in this Ruleset |
| Merge methods | Merge commit, rebase, and squash enabled |
| Repository permissions available to connected account | Admin, maintain, push, pull, triage |

Required contexts (exact spelling and case; each is bound to GitHub Actions integration ID `15368`):

1. `repository-policy`
2. `Model input JSON Schema`
3. `Decimal contract (Rust 1.85.0)`
4. `Decimal contract (Rust 1.99.1)`
5. `Semantic validator (Rust 1.85.0)`
6. `Semantic validator (Rust 1.99.1)`

The Ruleset is active and contains exactly these six required entries. PR #14 introduces the always-running `repository-policy` workflow. PR #18 removes path filters from both OFRL-IB-001 validation workflows so their required job contexts are reported on documentation-only as well as model-code Pull Requests.

**Base-branch integration status:** the current `main` snapshot still contains the older path-filtered project workflows and does not yet contain `repository-policy.yml`. The PR #14 / PR #18 branch stack includes the corrected workflows. After the stack is integrated into `main`, verify the workflow files on `main` and create fresh documentation-only and model-code PRs to confirm all six contexts report and complete. Do not weaken or remove a required check to bypass this sequencing dependency.

This is a dated snapshot, not a guarantee that settings remain unchanged. Re-read the Rulesets API after any configuration change and after the repository moves to an Organization.

## 2. Stage 0 baseline — one active maintainer

Configure a branch protection rule or repository ruleset targeting the exact default branch, currently `main`.

| Setting | Stage 0 observed baseline | Reason |
|---|---|---|
| Require a Pull Request before merging | **Enabled** | Prevent routine direct changes to the default branch |
| Required approving reviews | **0 / disabled** | A sole maintainer cannot obtain an independent approval; do not simulate one |
| Require conversation resolution | **Enabled** | Keep review discussions resolved before merge |
| Required status checks | **Exactly six named contexts; enabled** | Validate repository policy, schema, Decimal contract, and Rust semantic validation |
| Require branch to be up to date | **Enabled** | Require testing against the latest base branch |
| Allow force pushes | **Disabled** | Preserve default-branch history |
| Allow deletions | **Disabled** | Prevent accidental default-branch deletion |
| Bypass actors | **None** | Keep the same merge gate for the owner and collaborators |
| Code Quality / Copilot Review rules | Not configured in this Ruleset | No extra automated review rule is currently active |
| Signed commits / linear history | Not configured as Ruleset requirements | Do not impose controls without a tested signing or linear-history workflow |

The Founder may merge a Founder-authored PR under Stage 0 because independent review is not available. The PR checklist and validation evidence remain mandatory. This must be described as a single-maintainer decision, not an independent approval. The Rulesets API read-back reports an empty bypass list and confirms that the connected current user cannot bypass this Ruleset.

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

**Current OFRL integration note:** PR #14 introduces the always-running `repository-policy` workflow, and PR #18 removes path filters from both OFRL-IB-001 validation workflows. These changes make the six required contexts available on the combined PR #14 / PR #18 branch stack. They must reach `main` before the six-context configuration is considered fully exercised on the protected base branch. The active Ruleset itself has already been verified through the API.

A workflow that runs only on pushes to the default branch is not sufficient evidence that it protects Pull Requests.

## 7. Configuration and verification record

Latest observed record — 2026-11-04:

| Field | Record |
|---|---|
| Date | 2026-11-04 |
| Changed by | Not returned by the Rulesets API; repository owner has Admin permission |
| Branch or tag target | `~DEFAULT_BRANCH` (`main`) |
| Rule or ruleset name | `OFRL-main-protection`, ID `26516806` |
| Settings enabled | Active; PR required; 0 approvals; conversation resolution; strict up-to-date; deletion and non-fast-forward restrictions; no bypass actors |
| Required check names | Exactly six listed in Section 1; all bound to GitHub Actions integration ID `15368` |
| Verification PRs | PR #14 targeting `main`; PR #18 validates the same six contexts on the audit branch |
| Exceptions / follow-up | No Ruleset exception. Documentation-only and model-code smoke tests remain required after workflow integration into `main`. |

Re-read the Rulesets API after changing the default branch, adding or changing workflows, moving to an Organization, changing repository visibility, or changing Maintainer roles. Keep the read-back and the corresponding verification PR references together.

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
