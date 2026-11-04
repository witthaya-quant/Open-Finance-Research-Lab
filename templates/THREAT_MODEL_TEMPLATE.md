<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# [PROJECT-ID] — Threat Model

**Threat model version:** 0.1.0  
**Owner:**  
**Risk tier:**  
**Last reviewed:** YYYY-MM-DD

## 1. Scope and assumptions

Define the system, deployment, data, users, external services, and components included. List assumptions and exclusions. Link the architecture.

## 2. Assets and security objectives

| Asset | Classification | Confidentiality / integrity / availability need | Owner |
|---|---|---|---|
| | | | |

## 3. Actors and trust boundaries

Identify legitimate users, administrators, maintainers, services, external providers, untrusted contributors, and potential adversaries. Describe trust boundaries and privileged operations.

## 4. Data flows and attack surface

Identify entry points, APIs, files, packages, CI workflows, credentials, storage, network interfaces, and external dependencies.

## 5. Threat analysis

Use STRIDE or another documented method where appropriate.

| ID | Threat / abuse case | Asset / component | Preconditions | Likelihood | Impact | Existing controls | Residual risk |
|---|---|---|---|---|---|---|---|
| THR-001 | | | | Low / Medium / High | Low / Medium / High | | |

## 6. Security requirements and mitigations

For each material threat, define preventive, detective, and recovery controls. Include owner and validation evidence.

## 7. Identity, access, and secrets

Document authentication, authorization, least privilege, credential lifecycle, rotation, storage, and access reviews. Never include actual secret values.

## 8. Supply-chain and build security

Consider dependency provenance, lockfiles, Action pinning, artifact integrity, CI token permissions, untrusted Pull Requests, SBOM, and release provenance.

## 9. Privacy and sensitive data

Identify personal or sensitive data, purpose limitation, minimisation, retention, deletion, access, redaction, and logging controls where applicable.

## 10. Security validation

Define static analysis, dependency scanning, secret scanning, fuzzing, penetration testing, abuse-case tests, or manual review as appropriate to risk. Link the Validation Matrix.

## 11. Residual risk and acceptance

| Risk ID | Residual risk | Decision | Compensating control | Approver | Review trigger / expiry |
|---|---|---|---|---|---|
| | | Open / Accepted / Blocked | | | |

R3 residual risks require explicit Founder approval and appropriate independent review before operational release. Do not represent an unreviewed risk as accepted.

## 12. Incident and recovery

Link the reporting and response procedure. Describe containment, credential rotation, evidence preservation, service recovery, and user notification responsibilities.
