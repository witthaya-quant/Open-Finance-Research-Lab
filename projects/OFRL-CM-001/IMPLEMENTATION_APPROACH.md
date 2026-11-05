<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Implementation Approach

**Document version:** 0.1.0  
**Project:** OFRL-CM-001  
**Owner:** Project Owner  
**Last reviewed:** 2026-11-05  
**Status:** Approved project-level direction; implementation gates remain conditional

## 1. Purpose and governing decision

Rust is the default implementation language for computational software developed within OFRL-CM-001 when an executable implementation is justified by the research question.

This is a project-level direction. It does not amend OFRL-wide technology policy, does not make Rust mandatory for projects in other disciplines, and does not require every case study to contain software.

A case may be completed through mathematical derivation, proof, or analytical comparison without code. Implementation must add research value rather than exist merely to demonstrate a language or framework.

## 2. Stack overview

| Layer | Default | Responsibility |
|---|---|---|
| Mathematical and domain core | Rust | Explicit problem representation, domain rules, algorithms, constraints, and deterministic computation |
| Domain types | Rust structs, enums, and newtypes | Encode valid states and make invalid or ambiguous states difficult to represent |
| Candidate generation | Rust | Implement only methods justified by the formal specification |
| Verification | Rust | Check candidate outputs against the specification through separately reasoned validation logic |
| Command interface | Rust CLI, where useful | Reproducible execution, explicit inputs, stable exit codes, and inspectable outputs |
| Testing | Rust test tooling | Unit, integration, property, boundary, regression, and reduced-instance exhaustive tests as applicable |
| Build and dependency management | Cargo | Reproducible builds, locked dependencies, documented toolchain, and isolated project build graph |

No external solver, framework, serialization library, benchmark library, or CLI framework is selected in advance. Each dependency requires a defined need and review of maintenance, licence, security, compatibility, and MSRV.

## 3. Architecture principles

### 3.1 Formal domain before implementation

The authoritative problem definition is the relevant case-study specification, not the solver implementation.

The domain model must make important distinctions explicit:
- Input data versus derived state.
- Source observations versus OFRL assumptions.
- Valid, invalid, and unresolved states.
- Candidate outputs versus verified outputs.
- One solution versus all solutions, optimality, and uniqueness claims.

### 3.2 Determinism and numeric integrity

Prefer deterministic algorithms and exact discrete representations where the problem permits them.

- Use integer, fixed-domain, bitset, or rational representations when mathematically appropriate.
- Use checked arithmetic when range overflow is possible.
- Do not introduce floating-point arithmetic into a discrete problem without a mathematical reason.
- If floating point is necessary, define precision, rounding, tolerance, domain, and error implications explicitly.
- Randomized methods must expose and record their seed and stopping conditions.
- Do not hide invalid inputs, infeasible states, or resource limits by silently coercing values.

### 3.3 Typed boundaries and failure handling

Public and internal interfaces should use domain-specific types and explicit error results. Avoid using display strings as machine-readable error contracts.

Invalid input, unsupported rules, infeasible states, resource exhaustion, and internal defects must remain distinguishable. Do not convert a failure into a plausible-looking answer.

### 3.4 Solver and verifier separation

The solver generates candidate outputs. The verifier evaluates a candidate against the independently stated rules.

The verifier must not rely on the solver's internal search state, cached assumptions, or the same unexamined constraint-generation code. Shared domain types may be appropriate, but the acceptance predicate must be independently derived from the case specification.

The degree of physical separation—independent module, binary, or crate—is selected when the implementation design is known. A separate crate is justified when it materially strengthens dependency or trust boundaries; it is not created merely for appearance.

### 3.5 Minimal architecture, deliberate boundaries

Begin with the smallest Rust package structure that preserves clear domain, solver, and verifier interfaces. Use a Cargo workspace when multiple packages have a concrete ownership or build-boundary reason.

Do not split crates, introduce plugin systems, generic solver abstractions, or reusable frameworks before at least two real use cases demonstrate that the abstraction is warranted.

## 4. Suggested logical components

The following are conceptual boundaries, not a pre-approved directory or crate layout:

- **domain:** formal input types, normalized representations, invariants, and output types.
- **methods:** one or more case-specific methods selected after formalization.
- **verification:** independent checks against the case specification.
- **interface:** optional CLI or machine-readable reporting layer.
- **experiments:** isolated benchmarks or alternative implementations that must not silently become canonical.

The physical layout is decided in the implementation proposal for the first case that passes its formalization gate.

## 5. Toolchain and supply-chain controls

Before the first executable implementation:
1. Select and document the Rust toolchain and minimum supported Rust version (MSRV) based on the chosen dependency graph and target environment.
2. Pin the toolchain for reproducible CI and local reproduction.
3. Commit the Cargo.lock for each deployable or executable workspace and use locked dependency resolution in CI.
4. Keep dependencies minimal and review direct and transitive package provenance, licences, security advisories, maintenance, and compatibility.
5. Generate and validate a project-scoped software bill of materials when a distributable software artifact is proposed.
6. Record build commands, target platform, environment, and artifact integrity information.

OFRL-CM-001 will make its own toolchain and dependency decisions. It will not inherit OFRL-IB-001 source code, Cargo manifests, lockfiles, crates, schemas, workflows, or implementation decisions.

## 6. Verification and test strategy

Select tests from the actual mathematical properties of each case. Relevant controls include:
- Unit tests for domain transformations and local rules.
- Integration tests for the candidate-generation and verification boundary.
- Property tests for invariants and metamorphic relationships.
- Boundary and malformed-input tests.
- Exhaustive cross-checks on small instances where tractable.
- Golden or reference cases whose expected values have an independently documented derivation.
- Adversarial tests and minimized counterexamples.
- Deterministic reproduction of any reported result.

A solver and verifier passing the same small set of example tests is not, by itself, proof of completeness or uniqueness.

## 7. Performance and benchmarking

Correctness and specification coverage precede optimization.

A performance study must record:
- Exact source revision and toolchain.
- Hardware and operating environment.
- Input dimensions and characteristics.
- Warm-up, repetition, timeout, and stopping policy.
- Runtime and memory measurement method.
- Whether the comparison is algorithmic, implementation-specific, or end-to-end.

Do not compare methods using materially different conditions without explaining the limitation. Avoid benchmarking infrastructure until a real comparison question exists.

## 8. Continuous integration boundary

No project-specific software workflow is required while the project contains only research documentation and no executable source.

When code is admitted, add a dedicated OFRL-CM-001 validation workflow with path-scoped triggers for this project and its own build/test requirements. Do not modify OFRL-IB-001 workflows or weaken/change repository-wide controls to accommodate this project.

Expected Rust checks, when applicable:
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features --locked`
- `cargo doc --workspace --no-deps --all-features`

The exact commands will be adapted to the eventual workspace. A command is not considered passed until it has actually run against the submitted revision.

## 9. Deferred decisions

The following are intentionally unselected:
- Rust edition and MSRV.
- Workspace and crate layout.
- CLI argument and output formats.
- Serialization and error-reporting dependencies.
- Property-testing and benchmarking dependencies.
- SAT/SMT, constraint, optimization, or graph libraries.
- Parallel execution and performance architecture.

Each is decided only when the formal problem and implementation requirements justify it.

## 10. Current implementation status

For the Pressure Points pilot, executable implementation is **BLOCKED** because the complete rule semantics and required output are not yet established.

No Rust package, Cargo workspace, solver, verifier, CLI, dependency, or project-specific CI workflow is claimed to exist at this stage.
