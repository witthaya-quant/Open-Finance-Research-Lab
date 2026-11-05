<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Validation Matrix

**Project:** Computational Methods & Problem-Solving Case Studies  
**Project version / baseline:** 0.1.0  
**Risk tier:** R1  
**Owner:** Kenny (Witthaya Maidee)  
**Last updated:** 2026-11-05

## 1. Validation rules

Use only these states:
- **PASS:** acceptance criteria were executed and met; evidence is linked.
- **FAIL:** a check was executed and did not meet its criteria.
- **NOT RUN:** the check has not been executed.
- **BLOCKED:** a prerequisite or decision is missing.
- **N/A:** the check does not apply to the stated scope; explain why.

A green metadata workflow does not prove mathematical correctness. Evidence maturity, implementation status, and validation status are tracked separately.

## 2. Acceptance matrix

| ID | Requirement / invariant | Gate | Validation method | Acceptance criteria | Status | Evidence / artifact | Owner |
|---|---|---|---|---|---|---|---|
| VAL-001 | Project identity and registration | Admission | Manifest schema and project-registry CI | Manifest valid; folder and project ID match; catalog row is consistent | PASS | [Repository Policy run 41963988332](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41963988332) | Project Owner |
| VAL-002 | Source provenance | Admission | Inspect official source references and access metadata | Original publisher, title, URL, purpose context, and access date are recorded | PASS | DATA_MANIFEST.md; official Jane Street source URLs | Project Owner |
| VAL-003 | Third-party rights boundary | Admission / publication | Review stored artifacts against OFRL data and IP policies | No unlicensed puzzle payload, image, or official solution is committed; unresolved rights are not represented as permission | PASS | DATA_MANIFEST.md; current project file scope | Project Owner |
| VAL-004 | Independence from OFRL-IB-001 | Admission | Inspect PR changed-file list, base/head refs, and repository branch state | No IB-001 files, branch, PR, or issue modified; only the central Project Catalog is shared | PASS | [PR #29 changed files](https://github.com/witthaya-quant/Open-Finance-Research-Lab/pull/29/files); base main remained at the recorded starting commit | Project Owner |
| VAL-005 | Problem rules sufficiently formalized | Research | Trace each rule to an authoritative source and define input/output semantics | All correctness-critical rules and output conditions are explicit; unresolved items are closed or declared blockers | BLOCKED | cases/2026-11-pressure-points/PROBLEM_FORMALIZATION.md | Project Owner |
| VAL-006 | Solver correctness and completeness | Method development | Independent verifier, proof obligations, boundary and adversarial tests | Claims of soundness, completeness, optimality, or uniqueness are supported by appropriate evidence | N/A | No solver implementation in the approved initial scope | Project Owner |
| VAL-007 | Comparative method evaluation | Method development | Common fixtures, environment, stopping criteria, and metrics | Comparisons are reproducible and conditions are materially comparable | N/A | No solver comparison in the approved initial scope | Project Owner |
| VAL-008 | Financial-engineering transfer | Research interpretation | Explicit mapping and assumption analysis | Any claimed transfer identifies preserved properties, changed assumptions, and further evidence; no transfer is an acceptable conclusion | BLOCKED | Research Specification §10; pilot rule semantics unresolved | Project Owner |
| VAL-009 | Reproducibility of published findings | Publication | Clean-environment reproduction from versioned inputs and instructions | A competent reader can reproduce every material reported result | NOT RUN | No computational result is claimed | Project Owner |
| VAL-010 | Research claims and evidence maturity | Publication | Review claims against evidence levels 0–4 | No claim exceeds the evidence actually obtained | PASS | Manifest maturity 0; no solver or solution claim | Project Owner |
| VAL-011 | Project implementation architecture and isolation | Architecture | Review the project-level implementation decision and IMPLEMENTATION_APPROACH.md | Engineering controls are explicit; executable work is justified by the research question; no IB-001 components are reused | PASS | D-009 in DECISION_LOG.md; IMPLEMENTATION_APPROACH.md | Project Owner |
| VAL-012 | Executable Rust implementation quality and supply-chain controls | Implementation | Project-local Cargo CI and dependency review, when code is admitted | Declared toolchain/MSRV, locked dependencies, formatting, Clippy, tests, documentation, verifier boundary, and supply-chain evidence pass | NOT RUN | No executable code or Cargo workspace exists in this revision | Project Owner |
| VAL-013 | Project organization and shared-policy inheritance | Governance / admission | Inspect project tree, manifest requirements, README policy links, and case protocol | Project-local work is isolated; root policies remain authoritative by reference; no duplicate policies or speculative folders exist | PASS | project.json; README.md; CASE_STUDY_PROTOCOL.md §§9.1–9.3; HANDOFF.md | Project Owner |
| VAL-014 | Complete semantic grouping and cardinal mapping | Research formalization | Reconstruct every emoji group and account for all 76 entries using a consistent rule | All 19 groups and cardinal assignments are derived without unmatched entries or ad hoc exceptions | BLOCKED | PROBLEM_FORMALIZATION.md U-001–U-010; H-001–H-013 | Project Owner |
| VAL-015 | Consistent letter-residual / extraction rule | Research formalization | Explain residual letters and output extraction across all 19 groups | One rule accounts for all residuals and derives the required output without ad hoc exceptions | BLOCKED | PROBLEM_FORMALIZATION.md H-005–H-013; U-010 | Project Owner |
| VAL-016 | Contextual cross-checks for partial candidate themes | Research evidence | Review cited star, clock, state-map, lotus, hazard-diamond, desert-region, and joystick conventions | Each source supports only the contextual fact claimed; no source is used as proof of Jane Street's intended emoji grouping | PASS | TEST-015–TEST-020; DATA_MANIFEST.md SRC-014–SRC-021 | Project Owner |
| VAL-017 | Official solution retrieval | Source verification | Verify the Archive anchor/target and inspect the direct November 2026 solution body | A direct URL and body are retrieved; material rules are mapped to formalization blockers | BLOCKED | DATA_MANIFEST.md §4; search-result text observed, anchor target and direct solution URL not verified; body not retrieved | Project Owner |

## 3. Initial-pilot structural checks

The official Pressure Points page was accessed on 2026-11-05. The visible source contains:
- One leading marker.
- 75 distinct uppercase-letter sequences.
- 19 emoji clues.
- A terminal notation shown as “(11 12)”.

A structural check of the visible sequence block found that every sequence is internally alphabetically ordered, the list is lexicographically ordered, and the 75 sequences contain 504 letters in total. The length distribution is recorded in PROBLEM_FORMALIZATION.md.

These checks establish representation-level properties only; they do not resolve puzzle semantics.

## 4. Test inventory

| Test ID | Test name | Type | Input / fixture | Expected result | Actual result | Status | CI job / command |
|---|---|---|---|---|---|---|---|
| TEST-001 | Project manifest and catalog validation | Repository policy | OFRL-CM-001 project tree | Registry accepts the new independent project | Manifest and catalog accepted | PASS | [Repository Policy run 41963988332](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41963988332) |
| TEST-002 | Project documentation link validation | Repository policy | New Markdown files | All local links resolve | Local link and document validation passed | PASS | [Repository Policy run 41963988332](https://github.com/witthaya-quant/Open-Finance-Research-Lab/actions/runs/41963988332) |
| TEST-003 | Case source attribution review | Manual | DATA_MANIFEST.md | Primary source and purpose correctly attributed | Official URLs and access date recorded | PASS | Manual source review |
| TEST-004 | Puzzle rule completeness | Research review | Original source and formalization | Every correctness-critical rule traced to source | Source semantics remain incomplete | BLOCKED | Manual research gate |
| TEST-005 | Visible sequence structural consistency | Structural inspection | Official source representation | Count, uniqueness, internal ordering, overall ordering, and length totals are internally consistent | 75 distinct sequences; 504 letters; observed ordering properties checked | PASS | Structural analysis recorded in PROBLEM_FORMALIZATION.md |
| TEST-006 | Executable implementation quality gates | Engineering validation | Project-local build workspace and CI, when admitted | Formatting, static analysis, locked tests, documentation, and project-specific validation pass on the declared toolchain | No executable implementation has been admitted | NOT RUN | Future OFRL-CM-001 workflow |
| TEST-007 | Source cardinality cross-check | Structural analysis | Official source representation | Visible count is internally consistent: one leading marker plus 75 letter sequences, alongside 19 emoji clues; any proposed 19-by-4 grouping is recorded as a hypothesis rather than a proven rule | 76 visible marker/sequence entries; 19 emoji clues; 19 × 4 equals 76 arithmetically | PASS | PROBLEM_FORMALIZATION.md §4.1; arithmetic cross-check | Project Owner |
| TEST-008 | Compass-group aggregate letter-multiset equality | Manual mathematical cross-check | Candidate decoded terms NORTH / EAST / SOUTH / WEST | The combined letters in the four candidate source entries equal the combined letter multiset of the four direction names | Equality verified over 18 letters; individual word assignments remain hypotheses | PASS | PROBLEM_FORMALIZATION.md H-004; independent character-count audit | Project Owner |
| TEST-009 | Complete four-member group / cardinal assignment | Research analysis | All 19 emojis and 76 visible entries | Every entry is assigned exactly once; one consistent rule explains group membership and orientation | Ten candidate theme/position groups have been identified, but complete coverage, emoji intent, Ø treatment, and orientation are not established; several groups have local letter residuals | BLOCKED | PROBLEM_FORMALIZATION.md H-004–H-013, U-001–U-010 | Project Owner |
| TEST-010 | Candidate entity and regional context corroboration | Source cross-check | Official organization and public regional reference | The named candidate entity exists and the contextual Flemish Diamond geography is accurately described; this is not a test of puzzle correctness | Antwerpsche Diamantkring confirms itself as an Antwerp rough-diamond bourse; government report describes four-city Flemish Diamond geometry | PASS | DATA_MANIFEST.md SRC-007/SRC-008; cited source pages | Project Owner |
| TEST-011 | Compass four-direction candidate normalization | Structural / lexical | Candidate terms EAST, NORTH, SOUTH, WEST | Each candidate entry is an exact anagram or differs by the explicitly recorded one-letter residual; the aggregate multiset is checked | EAST is short E; NORTH has extra E; SOUTH and WEST are exact; aggregate 18-letter multiset equality holds | PASS | PROBLEM_FORMALIZATION.md H-004/H-006; DATA_MANIFEST.md SRC-013 | Project Owner |
| TEST-012 | Game-controller face-button layout candidate | Structural / positional | TRIANGLE, CIRCLE, CROSS, SQUARE | Candidate terms and top/right/bottom/left positions agree with a documented controller layout; letter residuals are explicitly counted | Sony DualSense layout corroborates positions; TRIANGLES has +S and CIRCLE has −E; CROSS and SQUARE are exact | PASS | PROBLEM_FORMALIZATION.md H-006; DATA_MANIFEST.md SRC-009 | Project Owner |
| TEST-013 | Baseball diamond four-base layout candidate | Structural / positional | SECOND, FIRST, HOME, THIRD | Candidate terms agree with standard top/right/bottom/left field positions; letter residuals are explicitly counted | MLB Appendix 2 shows second base top, first base right, home bottom, third base left; SECOND has +O and THIRD has −H | PASS | PROBLEM_FORMALIZATION.md H-006; DATA_MANIFEST.md SRC-010 | Project Owner |
| TEST-014 | Sailing-vessel four-position candidate | Structural / positional | BOW, STARBOARD, STERN, PORT | Bow/stern and port/starboard positions agree in the vessel-relative frame; residuals are explicitly counted | Bow/front, stern/back, port/left, starboard/right when facing bow; BOW has +A and STERN has −S | PASS | PROBLEM_FORMALIZATION.md H-006; DATA_MANIFEST.md SRC-011/SRC-012 | Project Owner |
| TEST-015 | Southern Cross four-star candidate | Astronomical / positional | ACRUX, GACRUX, MIMOSA, IMAI | Four main Crux stars are independently identified; cited chart conventions support the relative top/east/foot/west positions; residuals are counted | ESO confirms the four main stars; chart reference supports relative positions; GACRUX +C and ACRUX −U | PASS | PROBLEM_FORMALIZATION.md H-007; DATA_MANIFEST.md SRC-014/SRC-015 | Project Owner |
| TEST-016 | Clock-face four-position candidate | Structural / positional | TWELVE, THREE, SIX, NINE | Candidate terms occupy conventional 12/3/6/9 clock positions; residuals are explicitly counted | Positions align with the clock face; TWELVE +H; SIX +A/−S; THREE and NINE exact | PASS | PROBLEM_FORMALIZATION.md H-008; independent character-count audit | Project Owner |
| TEST-017 | Lotus / Indian-state candidate | Geographic / contextual | HIMACHAL PRADESH, WEST BENGAL, TAMIL NADU, GUJARAT | Lotus–India national-symbol association and four candidate state positions are corroborated; residuals are explicitly counted | Embassy of India identifies lotus as national flower; Survey of India map supports state positions; HIMACHAL PRADESH +G; TAMIL NADU −L | PASS | PROBLEM_FORMALIZATION.md H-009; DATA_MANIFEST.md SRC-016/SRC-017 | Project Owner |
| TEST-018 | NFPA 704 four-quadrant candidate | Standard / positional | HEALTH, FIRE, INSTABILITY, SPECIAL | Four hazard quadrants and conventional positions are corroborated; source terminology is recorded accurately | DHS describes top flammability, right instability, bottom special, left health; NFPA 704 confirms standard context; FIRE +E; SPECIAL −S | PASS | PROBLEM_FORMALIZATION.md H-010; DATA_MANIFEST.md SRC-018/SRC-019 | Project Owner |
| TEST-019 | Desert / Colorado candidate geography | Geographic / provisional | WYOMING, KANSAS, NEW MEXICO, UTAH | Four candidate cardinal neighbours and Four Corners high-desert context are supported; emoji interpretation remains provisional | Cardinal neighbours are confirmed by secondary map; Four Corners is a high-desert region; Colorado also borders NE Nebraska and SE Oklahoma and touches Arizona at Four Corners | PASS | PROBLEM_FORMALIZATION.md H-011; DATA_MANIFEST.md SRC-020/SRC-021 | Project Owner |
| TEST-020 | Joystick four-direction candidate | Structural / semantic | UP, RIGHT, DOWN, LEFT | Candidate terms align with four cardinal directional controls; residuals are explicitly counted | UP +H; LEFT −T; RIGHT and DOWN exact | PASS | PROBLEM_FORMALIZATION.md H-012; independent character-count audit | Project Owner |
| TEST-021 | Community-reported anagram leads | Source cross-check | Puzzling Stack Exchange discussion | User-generated examples and near-matches are registered as hypotheses, not promoted to formal rules | Discussion reports BUSQUETS, INIESTA, VIOLINS, MERCURY, INSTABILITY and near-place-name residuals; it explicitly says rules are not written and leaves Ø/emoji/output unresolved | PASS | DATA_MANIFEST.md SRC-005; PROBLEM_FORMALIZATION.md H-013 | Project Owner |
| TEST-022 | Official Archive index status | Source verification | Jane Street Archive search result and directly opened Archive HTML | Search-result text is treated as discovery evidence only; anchor target and solution body are verified separately | Search result displays November “Pressure Points” followed by “Puzzle Solution” text but exposes no href/target; opened HTML snapshot begins at August 2026; no direct URL/body retrieved | PASS | DATA_MANIFEST.md SRC-003 §4; official Archive search-result/direct-page comparison | Project Owner |
| TEST-023 | Secondary solver-status evidence classification | Source review | Kaia research-status page, Griffin Pinney solver page, and Puzzling Stack Exchange question | Public solver claims are registered as leads, while claims lacking a complete derivation remain unverified | Kaia reports the 76↔19×4 hypothesis and candidate phrase without a full derivation; Griffin's page says notes are due in December; the Puzzling post remains a question outlining candidate words and unresolved rules | PASS | DATA_MANIFEST.md SRC-005/SRC-006/SRC-023; PROBLEM_FORMALIZATION.md §4 | Project Owner |

## 5. Known gaps and exceptions

| ID | Gap / exception | Impact | Compensating control | Owner | Review trigger / expiry |
|---|---|---|---|---|---|
| GAP-001 | The retrieved source representation does not establish every rule or output condition | Prevents a defensible solver specification | No solver work; seek authoritative clarification or record a no-go | Project Owner | Before method selection |
| GAP-002 | Redistribution rights for puzzle text and artwork have not been established | Cannot include the full puzzle payload | Source URLs and limited observations only | Project Owner | Before any source content is copied |
| GAP-003 | No financial transfer has been demonstrated | Prevents claims of direct finance applicability | Perform a bounded mapping analysis after formalization | Project Owner | Before publication |
| GAP-004 | Archive search result displays November puzzle and “Puzzle Solution” text, but the anchor target, direct URL, and solution body could not be verified | Prevents using the official solution as a verified source of rules | Treat search-result text as a lead only; obtain the direct link and inspect its body before relying on it | Project Owner | Before any official-solution claim |

## 6. Sign-off

**Prepared by:** Kenny (Witthaya Maidee)  
**Project approval:** Founder-authorized scope, 2026-11-05  
**Independent review:** Not yet performed  
**Unresolved blockers:** Pressure Points rule semantics and authoritative output definition  
**Publication status:** Not released
