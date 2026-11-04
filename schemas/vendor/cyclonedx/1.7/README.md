<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# Vendored CycloneDX 1.7 JSON Schemas

These upstream CycloneDX JSON schemas are vendored so the release-candidate validator can resolve every schema reference locally and validate offline.

**Upstream project:** [CycloneDX/specification](https://github.com/CycloneDX/specification)  
**Source tag:** `1.7.2`  
**Source commit:** `349314a9d7671d7d2ca5b711a725f49a73979da6`  
**Upstream licence:** Apache-2.0 (see the upstream [LICENSE](https://github.com/CycloneDX/specification/blob/1.7.2/LICENSE)).

| File | Upstream path | Git blob SHA at source tag |
|---|---|---|
| `bom-1.7.schema.json` | `schema/bom-1.7.schema.json` | `1ca17a2e3f1f684dfcbe66b4f182d6ea21464278` |
| `spdx.schema.json` | `schema/spdx.schema.json` | `566c4a32518c60778e55a56ccf71bf781320b279` |
| `jsf-0.82.schema.json` | `schema/jsf-0.82.schema.json` | `f46bfb1e52731ad1280123ff3e2bd29bd18d4bc2` |
| `cryptography-defs.schema.json` | `schema/cryptography-defs.schema.json` | `e1781505723ce0c706291f146d24546d1cb6a02e` |

The primary BOM schema references the three companion schemas by relative file name. Keep all four files together and update their source tag, source commit, and blob SHAs together when adopting a later schema revision. The Git blob SHAs above identify the exact upstream source objects; they are not SHA-256 checksums.
