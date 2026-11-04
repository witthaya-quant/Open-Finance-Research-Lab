<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL Publication and Intellectual-Property Boundary

**Policy ID:** OFRL-IP-001  
**Version:** 1.0.0  
**Effective date:** 2026-11-04  
**Policy owner:** Founder / Principal Maintainer

## 1. Purpose

OFRL is a **Founder-led Open Source Research Project**. Its public repository is intended to make selected research, software, models, and tools inspectable, reproducible, reusable, and open to further development.

OFRL is not described as "proprietary open source". That phrase mixes two different licensing concepts. Content released under an open-source licence can be reused in accordance with that licence, including commercially where the licence permits it. Content that must remain exclusive, confidential, or subject to restrictions belongs in a private workstream and must not be represented as open source.

This policy establishes a publication boundary so that the Founder can share useful research without accidentally disclosing confidential know-how, restricted data, or material intended to remain commercially private.

## 2. Three distinct concepts

These concepts must not be confused:

- **Repository control:** who administers the GitHub repository, controls its settings, and decides what is accepted or published under the OFRL name.
- **Copyright and other intellectual-property rights:** who owns each original work or has authority to license it.
- **Licence permissions:** what users are allowed to do with a particular released work.

The personal GitHub account that owns a repository does not automatically own every contribution stored in it. Contributors retain rights in their original work unless a separate valid agreement says otherwise. They grant the permissions stated by the applicable licence.

Founder-led governance means the Founder controls project direction, admission, review, and official publication decisions. It does not mean the Founder owns copyright in another person's contribution.

## 3. Publication classifications

Every material item should be classified before it is committed, uploaded, attached to an Issue, included in a Pull Request, or published as a release.

### 3.1 Open release — public repository

Use this classification only when the rights holder intends the material to be public and reusable under the applicable licence.

Examples may include:

- General-purpose financial calculation libraries and reusable software.
- Clearly scoped financial models whose assumptions and implementation are intended for public inspection and reuse.
- Research notes, mathematical explanations, technical documentation, and educational material.
- Synthetic data and fixtures generated independently without disclosing or reconstructing restricted source data.
- Public benchmarks, test harnesses, schemas, and reproducibility instructions.

Default licence allocation:

| Content | Default licence / rule |
|---|---|
| Software, source code, tests, schemas, and computational models | Apache-2.0 |
| Research writing, specifications, explanatory documentation, and diagrams | CC BY 4.0 |
| Original synthetic research data | CC BY 4.0 by default |
| Synthetic fixtures integral to software tests | Apache-2.0 where expressly identified |
| Third-party data and content | Original licence and terms; no automatic relicensing |

Apache-2.0 permits commercial use, modification, and redistribution subject to its terms. CC BY 4.0 permits commercial sharing and adaptation with attribution and compliance with its conditions. A public release must be made on the assumption that another person or company may use it commercially.

### 3.2 Private know-how — do not publish

Keep information private when its value depends on confidentiality, it is not authorised for disclosure, or the Founder has not decided to release it.

Examples include:

- Unreleased trading strategies, signal logic, execution methods, and proprietary market-making workflows.
- Production parameters, calibration values, thresholds, position sizing, live risk limits, and strategy configurations that reveal an operational edge.
- Non-public research hypotheses, model variants, or implementation details that the Founder deliberately intends to retain as confidential know-how.
- Private datasets, data transformations, and derived records that reveal restricted information.
- Credentials, private keys, API tokens, account identifiers, and security-sensitive configurations.
- Confidential business plans, commercial negotiations, employer information, client information, or partner material.

Store such information only in an appropriately access-controlled private system or repository. Do not place it in public Git history, public Issues, Pull Requests, CI logs, release artifacts, screenshots, or examples.

Removing a secret from the latest version of a file does not remove it from the repository's history or from copies already obtained. If a secret is exposed, revoke or rotate it immediately and follow the security incident process.

### 3.3 Restricted third-party material — do not redistribute without authority

A public URL, free download, API response, exchange display, or public filing does not by itself establish a right to redistribute the underlying data or content.

Do not publish vendor feeds, paid datasets, employer-owned material, client records, licensed research, or third-party code unless the relevant terms and permissions allow the exact intended use.

If redistribution is not permitted or is unclear, publish only the permitted description, source reference, schema, methodology, and lawful retrieval instructions.

### 3.4 Candidate protected intellectual property — review before disclosure

If work may contain a patentable technical invention, commercially sensitive design, or other protectable subject matter, pause public disclosure until the Founder has considered the relevant protection route and obtained appropriate professional advice where necessary.

Patent rules and the effect of public disclosure vary by jurisdiction. Do not assume that a later filing can restore rights lost through an earlier disclosure.

This policy does not claim that a financial formula, trading idea, mathematical model, or abstract method is patentable. That is a fact-specific legal question.

## 4. Important limits of copyright and secrecy

Copyright generally protects original expression, such as source code, written explanations, diagrams, and documentation. It does not, by itself, give exclusive rights over an underlying idea, procedure, method of operation, or mathematical concept. See the [World Intellectual Property Organization's copyright guidance](https://www.wipo.int/en/web/copyright/protection).

Accordingly:

- Publishing source code under an open-source licence gives users the permissions stated in that licence for the protected code and other covered material.
- Publishing a mathematical explanation does not create a copyright monopoly over the underlying mathematical idea or method.
- A person may be able to implement a similar method independently, subject to other applicable rights and laws.
- Do not rely on copyright as the protection for a method whose commercial value depends on secrecy.

Trade-secret protection generally depends on commercial value derived from secrecy, limited access, and reasonable measures to preserve confidentiality. Publicly releasing the information is inconsistent with keeping that information secret. See [WIPO's trade-secret guidance](https://www.wipo.int/en/web/trade-secrets/tradesecrets_faqs).

The practical rule is simple: **publish only what OFRL is prepared to let the public inspect, use, modify, and reuse under the stated licence. Keep the rest out of the public repository.**

## 5. Public and private workstreams

OFRL uses a two-lane operating model:

| Lane | Purpose | Default access |
|---|---|---|
| **Public OFRL repository** | Reusable research, code, models, documentation, synthetic fixtures, and contributions intentionally released under declared licences | Public |
| **Private research and know-how** | Unreleased methods, confidential strategy details, restricted data, commercial experiments, credentials, and work not approved for release | Founder-controlled, need-to-know |

This is a publication boundary, not a requirement to create a second repository immediately. The Founder may use an appropriately secured private storage location until there is enough private software or research to justify a separate repository.

Do not create a mixed public/private folder inside the public repository and assume that a hidden branch, ignored file, or unlinked directory is confidential. Anything pushed to a public repository must be treated as disclosed.

If a future commercial product combines open and private components, define the boundary explicitly. Keep public interfaces and open components under their declared licences; keep proprietary services, data, deployment configuration, and confidential implementation in separate controlled systems. Do not copy third-party or contributor-owned code into a proprietary component unless the licence and rights permit it.

## 6. Contribution rights and future licensing choices

Under the current OFRL policy:

- Contributors retain copyright in their original contributions.
- Submitted software contributions are licensed under Apache-2.0.
- Submitted written research and documentation contributions are licensed under CC BY 4.0.
- Third-party material remains under its own licence and terms.
- OFRL does not require a CLA for ordinary contributions at this stage.

A Founder-controlled repository does not give the Founder an automatic right to relicense another contributor's work under a closed or proprietary licence.

If OFRL later wants dual licensing, commercial relicensing, exclusive rights, or a source-available proprietary product incorporating external contributions, the rights model must be reviewed before accepting those contributions. A suitable Contributor Licence Agreement (CLA) or Copyright Assignment Agreement (CAA) may be considered with legal advice and explicit contributor consent. A CLA generally grants the project additional licensing permissions while the contributor retains copyright; a CAA transfers copyright ownership. These are materially different arrangements.

Do not introduce a CLA or CAA merely to appear enterprise-grade. Use one only when there is a defined licensing or ownership requirement and contributors understand the agreement.

A later licence change cannot silently revoke permissions already granted under earlier releases.

## 7. Pre-publication gate

Before publishing a material research or software item, the Founder or designated release authority must be able to answer:

1. **Rights:** Do we own the work or have permission to publish it under the proposed licence?
2. **Intent:** Are we willing for others, including commercial users, to use and adapt this material under that licence?
3. **Confidentiality:** Does it reveal private strategy, client, employer, partner, or operational information?
4. **Data:** Are source data, transformations, examples, and outputs permitted for this use?
5. **IP strategy:** Could disclosure affect a patent, trade-secret, contractual, or other protection decision?
6. **Attribution:** Are contributors and third-party sources identified accurately?
7. **Security:** Are credentials, sensitive configurations, and exploitable details excluded?
8. **Claims:** Are research maturity, limitations, and validation status described accurately?

If any answer is unknown, pause publication and resolve the issue. Do not treat an open-source licence as a substitute for permission to disclose.

For high-impact disclosure decisions, record the outcome in the central Decision Register.

## 8. Commercial direction

The open-source licence does not prevent OFRL from building a sustainable commercial model around complementary value, provided third-party rights and licence obligations are respected.

Potential models to evaluate later include:

- Paid implementation, integration, and engineering services.
- Professional support, training, and research services.
- Hosted or managed services where OFRL adds operational value.
- Proprietary datasets, research processes, or deployment configurations that are independently owned and lawfully controlled.
- Private enterprise extensions that are separate from, and licence-compatible with, the open components.

This policy does not commit OFRL to any business model. Commercialisation decisions require a separate business case and rights review.

## 9. Review and related policies

Review this boundary before major public releases, before accepting contributions that may be commercially sensitive, and whenever the licensing or commercialization strategy changes.

Related documents:

- [OFRL Governance](GOVERNANCE.md)
- [Licensing and Reuse Policy](LICENSE_POLICY.md)
- [Data Policy](DATA_POLICY.md)
- [Security Policy](SECURITY.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Central Decision Register](decisions/README.md)

This policy is an operational control, not legal advice. Obtain qualified advice for jurisdiction-specific IP, confidentiality, employment, contract, or patent questions.
