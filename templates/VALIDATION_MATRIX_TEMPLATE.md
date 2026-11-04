<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# [PROJECT-ID] — Validation Matrix

**Project:**  
**Project version / baseline:**  
**Risk tier:** R0 | R1 | R2 | R3  
**Owner:**  
**Last updated:** YYYY-MM-DD

## 1. Validation rules

Use only these states:

- **PASS:** the stated acceptance criteria were executed and met; link the evidence.
- **FAIL:** the check was executed and did not meet its acceptance criteria.
- **NOT RUN:** the check has not been executed.
- **BLOCKED:** the check cannot be executed because a dependency, decision, or prerequisite is missing.
- **N/A:** the check genuinely does not apply; explain why.

A status without evidence is not a PASS. A green CI job establishes only the behavior exercised by that job. Validation status is separate from project lifecycle and evidence maturity.

## 2. Acceptance matrix

| ID | Requirement / invariant | Risk / gate | Validation method | Acceptance criteria | Status | Evidence / artifact | Owner |
|---|---|---|---|---|---|---|---|
| VAL-001 | Specification consistency | Critical | | | NOT RUN | | |
| VAL-002 | Domain / mathematical correctness | Critical | | | NOT RUN | | |
| VAL-003 | Data quality and rights | Critical | | | NOT RUN | | |
| VAL-004 | Implementation correctness | High | | | NOT RUN | | |
| VAL-005 | Boundary and failure behavior | High | | | NOT RUN | | |
| VAL-006 | Reproducibility | High | | | NOT RUN | | |
| VAL-007 | Security / privacy, where applicable | Critical for R3 | | | NOT RUN | | |
| VAL-008 | Release provenance and licensing | Release gate | | | NOT RUN | | |

Add, remove, or split rows based on project type. Do not delete a failed or unrun check merely to make the matrix appear complete.

## 3. Test inventory

| Test ID | Test name | Type | Input / fixture | Expected result | Actual result | Status | CI job / command |
|---|---|---|---|---|---|---|---|
| TEST-001 | | Unit / Integration / Property / Golden / Statistical / Security / Manual | | | | NOT RUN | |

## 4. Evidence references

Link immutable or versioned evidence where possible: commit SHA, CI run, release artifact, fixture checksum, notebook revision, dataset manifest, independent review, or published report.

## 5. Known gaps and exceptions

| ID | Gap / exception | Impact | Compensating control | Owner | Review trigger / expiry |
|---|---|---|---|---|---|
| GAP-001 | | | | | |

Exceptions must not waive law, third-party terms, licensing, privacy, or mandatory repository controls.

## 6. Sign-off

**Prepared by:**  
**Reviewed by:**  
**Review scope:**  
**Review date:**  
**Unresolved blockers:**  

Do not describe Founder self-review as independent review. For a high-risk claim that requires independent assurance, keep the claim or release scope appropriately limited until a qualified independent review is available.
