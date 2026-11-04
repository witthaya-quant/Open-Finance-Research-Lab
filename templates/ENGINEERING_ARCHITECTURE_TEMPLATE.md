<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# [PROJECT-ID] — Engineering Architecture

**Architecture version:** 0.1.0  
**Owner:**  
**Last updated:** YYYY-MM-DD

## 1. Purpose and architecture scope

Describe the system boundary, intended capabilities, users, environments, and architectural questions. Link the approved project specification.

## 2. Quality attributes

Prioritise the relevant attributes: correctness, determinism, reproducibility, performance, latency, availability, security, privacy, maintainability, portability, and interoperability. Define measurable targets where possible.

## 3. Context and actors

Describe users, external systems, data providers, trust boundaries, privileged actors, and system dependencies.

## 4. Component view

| Component | Responsibility | Inputs | Outputs | Failure behavior |
|---|---|---|---|---|
| | | | | |

Link a diagram or interface specification where it materially improves understanding.

## 5. Data flow and contracts

Document schemas, formats, units, identifiers, timestamp conventions, validation boundaries, persistence, data lineage, and ownership of canonical values.

## 6. Interfaces and compatibility

List APIs, CLI commands, file contracts, message interfaces, public library surfaces, schema versions, and compatibility promises.

## 7. State, persistence, and recovery

Describe what state is stored, its authoritative source, retention, backup, restore, idempotency, transaction boundaries, and recovery behavior.

## 8. Security and privacy

Identify sensitive assets, trust boundaries, authentication/authorization, secret handling, input validation, logging constraints, and the linked Threat Model where applicable.

## 9. Failure modes and observability

Describe expected failure categories, error propagation, diagnostic identifiers, logs/metrics/traces, alerting, and conditions that require fail-closed behavior.

## 10. Deployment and operating model

List supported platforms, runtime/toolchain, configuration, infrastructure, external services, operational dependencies, and rollback strategy.

## 11. Alternatives and trade-offs

Record material architectural alternatives, rejected options, reversibility, and the reason for the selected design. Link Decision Records.

## 12. Validation and evolution

Link build/test workflows and the Validation Matrix. State known architecture risks and conditions that would justify a redesign.
