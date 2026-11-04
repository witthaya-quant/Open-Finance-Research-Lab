# Repository policy documentation-only smoke test

This short-lived Pull Request verifies that the active `OFRL-main-protection` Ruleset reports all six required GitHub Actions contexts for a documentation-only change after the workflow integration was merged to `main`.

It is a control-verification artifact, not a permanent OFRL policy or project specification. It must not be merged into the research repository.

Expected contexts:
- `repository-policy`
- `Model input JSON Schema`
- `Decimal contract (Rust 1.85.0)`
- `Decimal contract (Rust 1.99.1)`
- `Semantic validator (Rust 1.85.0)`
- `Semantic validator (Rust 1.99.1)`

The test passes only when all six exact contexts appear on this Pull Request and complete successfully.
