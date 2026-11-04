<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Security Policy

**Policy owner:** Founder / Principal Maintainer  
**Effective date:** 2026-11-04

## 1. Scope

This policy covers security vulnerabilities in OFRL-maintained source code, workflows, build configuration, schemas, release artifacts, and repository-managed services.

It does not make OFRL responsible for vulnerabilities in third-party services, data providers, operating systems, or dependencies, although material dependency vulnerabilities affecting OFRL may be reported and assessed here.

## 2. Supported versions

Until OFRL publishes a formal release-support matrix, security fixes are assessed against:

- The current default branch.
- The latest explicitly published release, if one exists.

Older snapshots may be unsupported. A project may define a stricter support policy in its own documentation.

## 3. Reporting a vulnerability

**Do not publish suspected vulnerabilities, exploit details, credentials, tokens, private keys, or sensitive proof-of-concept material in public Issues or Pull Requests.**

Preferred route:

1. Use GitHub's **Private vulnerability reporting** feature for this repository if it is enabled.
2. If that feature is unavailable, contact the repository owner privately using the contact channel published on the GitHub profile: https://github.com/witthaya-quant
3. If neither route is usable, use GitHub's platform-level reporting mechanism.

A useful report includes the affected component and version or commit, impact, reproduction steps, prerequisites, and any suggested mitigation. Avoid including real customer, client, or confidential data.

## 4. Handling and disclosure

The Founder / Principal Maintainer coordinates triage during Stage 0. A report should be assessed for severity, affected versions, exploitability, scope, and potential impact on users or research integrity.

OFRL will make a reasonable effort to acknowledge and investigate reports, but does not promise a fixed response or remediation time while it is operated by a single maintainer.

Do not disclose a vulnerability publicly until a fix or mitigation is available, or the maintainer and reporter have agreed that coordinated disclosure is appropriate. The parties should avoid unnecessary disclosure of exploit details while affected users remain exposed.

When a fix is published, document the affected versions, severity rationale, mitigation, and relevant release or commit. Do not publish sensitive reporter details without permission.

## 5. Repository security expectations

- Never commit secrets, credentials, private keys, access tokens, or recovery codes.
- Use least-privilege GitHub permissions and scoped credentials.
- Review dependency provenance, licensing, maintenance status, and known vulnerabilities.
- Keep CI permissions minimal; workflows should receive only the permissions they need.
- Avoid exposing restricted datasets or sensitive information in logs and artifacts.
- Treat external contributions, data files, and executable notebooks as untrusted until reviewed.
- Record material security exceptions and their expiry or review condition.

## 6. Future organization stage

Before broad Organization participation, OFRL should define a second security contact, enable and test private vulnerability reporting where available, review organization-level security settings, and assign a Maintainer responsible for dependency and workflow security.

## 7. Related policies

See [GOVERNANCE.md](GOVERNANCE.md), [DATA_POLICY.md](DATA_POLICY.md), [LICENSE_POLICY.md](LICENSE_POLICY.md), and [CONTRIBUTING.md](CONTRIBUTING.md).
