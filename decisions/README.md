<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Central Decision Register

This register records lab-wide and cross-project decisions that materially affect OFRL's direction, standards, governance, architecture, shared methods, data, licensing, security, and publication commitments.

It complements—not replaces—project-specific Decision Logs. For example, [OFRL-IB-001 Decision Log](../projects/OFRL-IB-001/DECISION_LOG.md) remains the authoritative record for decisions scoped to the Integrated Three-Statement Financial Model.

## 1. When to create a central Decision Record

Create a central record when a decision:

- Applies to multiple projects or defines a lab-wide policy.
- Changes governance, authority, review, security, licensing, data, or disclosure.
- Establishes or changes a shared architecture, common data contract, numerical convention, or public interface.
- Selects a research methodology or evaluation framework intended for reuse across projects.
- Changes a previous accepted central decision.

For project-only decisions, use that project's Decision Log. Reference a central record when the project applies or specializes a lab-wide rule. Do not duplicate the full rationale across multiple records.

Routine implementation details, editorial corrections, and reversible choices within an approved scope do not normally need a separate record. Use judgement and document the decision in the relevant Pull Request where a full record would add no meaningful traceability.

## 2. Record format

Use [DECISION_RECORD_TEMPLATE.md](../templates/DECISION_RECORD_TEMPLATE.md).

IDs are stable and sequential:

`OFRL-ADR-YYYY-NNN`

- `YYYY` is the calendar year of the initial proposal.
- `NNN` is a zero-padded sequence number within that year.
- Do not reuse an ID, even if a decision is rejected or superseded.

## 3. Status definitions

| Status | Meaning |
|---|---|
| Proposed | Under consideration; not an approved baseline |
| Accepted | Approved for the stated scope and version |
| Rejected | Considered and not selected; rationale retained |
| Superseded | Replaced by a later Decision Record |
| Deprecated | No longer recommended for new work; retained for history or compatibility |

A proposal may be discussed in an Issue or Pull Request before its record is created. Once accepted, the record should be linked to the approving Pull Request or governance decision.

## 4. Decision register

| ID | Decision | Status | Scope | Date |
|---|---|---|---|---|
| [OFRL-ADR-2026-001](OFRL-ADR-2026-001-founder-led-single-maintainer-governance.md) | Founder-led governance and staged transition to an Organization | Accepted | Lab-wide | 2026-11-04 |

## 5. Lifecycle rules

1. A record must not be silently rewritten to conceal its previous position.
2. Material changes create a new record that links to the record being superseded.
3. An accepted decision does not prove that its implementation is correct or complete.
4. Record validation status separately from decision status.
5. A lower-level project decision cannot waive applicable law, third-party terms, licences, data policy, security policy, conduct policy, or lab-wide minimum standards.
6. During Stage 0, the Founder is the decision authority. The record must identify that it was founder-approved and must not imply independent review.

## 6. Related governance documents

- [OFRL Governance](../GOVERNANCE.md)
- [Research Charter](../RESEARCH_CHARTER.md)
- [Research Standards](../RESEARCH_STANDARDS.md)
- [Decision Record Template](../templates/DECISION_RECORD_TEMPLATE.md)
