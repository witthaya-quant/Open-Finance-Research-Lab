<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Account and Repository Continuity

**Policy ID:** OFRL-OPS-001  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Policy owner:** Founder / Principal Maintainer

## 1. Purpose

OFRL is intentionally maintained under the Founder's personal GitHub account. This keeps ownership and administration simple, but it also makes account security and continuity important operational responsibilities.

The goal is to preserve access to the repository and its research history if the Founder loses access to a device, email account, or GitHub account. This policy does not authorize anyone to share or impersonate the Founder's account.

## 2. Account security baseline

The Founder should:

- Enable phishing-resistant multi-factor authentication where supported, preferably a passkey or hardware security key.
- Register more than one independent authentication method so that loss of a device does not lock the owner out.
- Store recovery codes offline in a secure location separate from the primary device.
- Secure the primary email account with strong authentication and recovery options.
- Review connected GitHub Apps, OAuth authorizations, personal access tokens, SSH keys, deploy keys, and sessions periodically.
- Remove unused credentials and revoke any credential suspected to be exposed.
- Use scoped, short-lived credentials where available; never commit secrets to the repository.
- Keep the operating system, browser, password manager, and recovery email secure and updated.

Do not share the GitHub password, MFA seed, passkey, hardware key, recovery codes, or personal access token with a Maintainer or Contributor.

## 3. Successor and stewardship

GitHub provides a successor setting for personal accounts. The Founder should review **Settings → Account → Successor settings** and decide whether to appoint a trusted successor.

A successor is not given the ability to log in as the Founder. GitHub documents a limited process under which an appointed successor may manage user-owned repositories if the account owner is no longer able to do so. See [GitHub's account ownership continuity guidance](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/repository-access-and-collaboration/maintaining-ownership-continuity-of-your-personal-accounts-repositories).

Appointment is a personal and legal decision. This policy does not select a successor or authorize a person to act without the relevant platform process and proof.

## 4. Repository backup

Maintain at least one independent backup of the Git repository outside the primary GitHub account.

Recommended baseline:

- Create a mirror backup of Git refs and objects before a major release or material repository transfer.
- Maintain a periodic encrypted backup on storage controlled by the Founder.
- Verify that a backup can be cloned or restored; an untested backup is not a recovery control.
- Keep release binaries, SBOMs, checksums, and publication artifacts separately from source history.
- Preserve a copy of material governance decisions and release records.

A Git mirror does not automatically back up every GitHub feature. Issues, Pull Requests, Discussions, Settings, Actions secrets, security alerts, and integrations require separate consideration. Do not claim a complete repository backup based only on `git clone --mirror`.

## 5. Recovery and incident response

If the account or a credential is compromised:

1. Revoke or rotate affected credentials immediately.
2. Secure the primary email and authentication methods.
3. Review recent sessions, OAuth grants, Apps, tokens, SSH keys, deploy keys, webhooks, and repository settings.
4. Review recent commits, releases, workflow changes, and unexpected permission changes.
5. Use GitHub's account recovery and security reporting channels where needed.
6. Record material repository changes and publish a security notice when users may be affected.

If the Founder is temporarily unavailable, a Maintainer may continue only within an explicitly delegated scope. A Maintainer must not assume account ownership, use shared credentials, transfer the repository, or change reserved governance decisions without proper authority.

## 6. Periodic review

Review authentication, recovery methods, successor settings, connected applications, backup integrity, and Maintainer access at least annually and after any material account-security event.

## 7. Related documents

- [OFRL Governance](GOVERNANCE.md)
- [Security Policy](SECURITY.md)
- [GitHub Repository Settings](GITHUB_REPOSITORY_SETTINGS.md)
- [Dependency Policy](DEPENDENCY_POLICY.md)
- [Release Policy](RELEASE_POLICY.md)
