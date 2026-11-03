<!-- Copyright 2026 Kenny. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Licensing and Reuse Policy

OFRL uses a content-type-specific licensing model. The default for original material is determined by the kind of work, not merely by the fact that it appears in this repository.

## Licence transition note

Earlier repository revisions were published while Apache-2.0 was presented as the repository-wide default. This content-specific policy clarifies the licence allocation for the current and future versions: software and computational models under Apache-2.0, written documentation under CC BY 4.0, and data under the source-specific rules below. This change does not revoke permissions already granted for earlier versions or copies distributed under Apache-2.0.

## 1. Software, code, and computational models — Apache-2.0

Original OFRL software and computational implementations are licensed under the Apache License, Version 2.0.

This includes, unless a file or project states otherwise:

- Rust source code, libraries, binaries, and CLI applications.
- Source code in other programming languages.
- Build scripts, tests, CI workflows, executable notebooks, and machine-readable configuration.
- Schemas and interfaces intended to be consumed by software.
- Financial calculation implementations, including formula-driven model workbooks and reusable computational models.
- Code examples that are distributed as standalone source files.

The full terms are in the repository-root LICENSE file. SPDX identifier: Apache-2.0.

Apache-2.0 permits commercial use, modification, distribution, and sublicensing subject to its conditions. It includes a contributor patent licence with the specific termination provision stated in the licence. It does not grant rights to OFRL trademarks or logos.

## 2. Documentation and research works — CC BY 4.0

Original OFRL written and explanatory works are licensed under the Creative Commons Attribution 4.0 International License.

This includes, unless a file or project states otherwise:

- README files, research specifications, charters, policies, and contribution guidance.
- Research papers, technical reports, explanatory Markdown, and written tutorials.
- Mathematical derivations and explanatory prose.
- Original diagrams, figures, and presentation material.
- Documentation templates and project narratives.

The licence notice and canonical legal code are in LICENSE-docs. SPDX identifier: CC-BY-4.0.

CC BY 4.0 allows sharing and adaptation, including commercial reuse, provided the attribution and other licence conditions are met. Users must identify changes where applicable and must not imply endorsement by OFRL or the original authors.

Executable notebooks are treated as software artifacts under Apache-2.0 because they contain runnable code. Their separate written reports and documentation are CC BY 4.0. Where a notebook contains substantial third-party or separately licensed material, that material must be clearly identified.

## 3. Data — source-specific, with a defined rule for OFRL-created data

Data is not automatically covered by either repository licence. Every dataset must have a documented provenance and rights status.

| Data class | OFRL policy |
|---|---|
| Original synthetic datasets created independently by OFRL | CC BY 4.0 by default, with attribution and dataset metadata |
| Synthetic test fixtures embedded in software tests | Apache-2.0 when they are integral software test assets; otherwise CC BY 4.0 |
| Third-party market data and vendor datasets | Original provider licence and contract terms only |
| Public company filings and financial statements | Source-specific rights and terms; public accessibility alone is not permission to redistribute |
| Curated, transformed, or derived third-party datasets | Review source rights, database rights, terms, and transformation constraints before release |
| Data with unclear rights | Do not redistribute; use a data manifest and lawful acquisition instructions where permitted |

A public URL, free download, or public filing does not automatically grant redistribution or commercial-use rights. Do not copy vendor feeds, exchange data, paid datasets, employer data, client data, or other restricted information into this repository without the required rights.

For each dataset, document at minimum: source/provider, source URL or identifier, access date, coverage, schema, time zone and frequency where relevant, transformations, exclusions, licence or terms, redistribution status, and any required attribution. If redistribution is prohibited or uncertain, publish only the metadata and lawful retrieval instructions.

CC BY 4.0 can apply to original datasets and, where the licensor has the relevant rights, applicable database rights. It cannot clear third-party rights that OFRL does not own.

## 4. Third-party code and content

The default licences do not relicense material OFRL does not own or have authority to license.

Third-party crates, libraries, code snippets, papers, books, charts, images, fonts, company data, and other external content remain subject to their own licences, terms, and applicable rights. Preserve required notices and identify exclusions clearly.

Before adding a dependency or external asset, record its source, version, licence, and any applicable notice or attribution obligation. If compatibility or redistribution rights are unclear, do not include the material until reviewed.

## 5. Contributions and ownership

Contributors retain copyright in their original contributions while licensing submitted work under the applicable OFRL content-type licence:

- Software and computational model contributions: Apache-2.0.
- Documentation and research-writing contributions: CC BY 4.0.
- Original synthetic datasets: CC BY 4.0 unless they are integral software test fixtures, in which case Apache-2.0 may apply as stated above.

By intentionally submitting a contribution for inclusion, a contributor confirms that they have the authority to submit it under the applicable terms. Do not submit employer-owned, client-owned, confidential, or restricted material without appropriate authority.

OFRL does not require a separate CLA for ordinary contributions at this stage. If the project later requires a CLA or a different governance structure, that change must be documented before it takes effect for new contributions.

## 6. Attribution, patents, and trademarks

Keep copyright, licence, attribution, and third-party notices required by the applicable licence. Identify modifications where required.

Apache-2.0 includes a limited contributor patent licence as described in its text. CC BY 4.0 does not grant patent or trademark rights. Neither licence grants permission to use OFRL names, logos, or marks as an endorsement.

## 7. No warranty or endorsement

The materials are provided under the relevant licence terms and disclaimers. Publication does not mean that OFRL endorses a downstream use, model result, investment decision, or modified work.

## 8. Licence changes

Permissions already granted for a version under its applicable licence are not automatically revoked by a later change to the repository's default policy. Any future exception or change must be explicit and versioned.

This policy is an operational guide, not legal advice.