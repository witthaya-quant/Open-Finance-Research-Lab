<!-- Copyright 2026 Kenny and contributors. SPDX-License-Identifier: CC-BY-4.0 -->

# OFRL-CM-001 — Data Manifest

**Dataset ID:** OFRL-CM-001-SOURCE-REGISTER  
**Dataset version:** 0.3.0  
**Data classification:** Public Source (reference-only; no source puzzle payload stored)  
**Data owner / steward:** Project Owner  
**Created or retrieved:** 2026-11-05  
**Last reviewed:** 2026-11-05

## 1. Purpose and intended use

This manifest records provenance for the first OFRL-CM-001 case study and the contextual material used to explain the research motivation.

The first pilot is the November 2026 Jane Street puzzle, Pressure Points. The research objective is to formalize the problem and assess appropriate mathematical and computational methods independently. The objective is not to reproduce Jane Street's proprietary research or infer a trading strategy.

## 2. Provenance and rights

| ID | Source / provider | Reference | Access date | Intended use | Rights / redistribution status |
|---|---|---|---|---|---|
| SRC-001 | Jane Street | https://www.janestreet.com/puzzles/ | 2026-11-05 | Primary source for Jane Street's stated purpose of its puzzle programme | Third-party website; no redistribution of page content |
| SRC-002 | Jane Street | https://www.janestreet.com/puzzles/pressure-points-index/ | 2026-11-05 | Original source of the November 2026 Pressure Points puzzle | Third-party puzzle content; redistribution permission not established; URL reference only |
| SRC-003 | Jane Street | https://www.janestreet.com/puzzles/archive/ | 2026-11-05 | Verify puzzle archive and identify the source context | Third-party website; no archive content copied |
| SRC-004 | Jane Street Blog | https://blog.janestreet.com/asic-puzzle-results/ | 2026-11-05 | Methodological context on diverse solution methods and verification in a separate ASIC challenge | Third-party article; citation and link only; not a first-pilot input |
| SRC-005 | Puzzling Stack Exchange | https://puzzling.stackexchange.com/questions/141272/jane-streets-recent-puzzle-pressure-points-nov-2026 | 2026-11-05 | Community discussion identifying exploratory anagram/geographic interpretations and unresolved questions | User-generated secondary discussion; not authoritative rules or a verified solution |
| SRC-006 | Kaia | https://www.kaiaz.me/now/ | 2026-11-05 | Secondary solver-status lead reporting a 76-entry / 19-by-4 cardinal-vertex hypothesis and candidate phrase ANTWERPSCHE DIAMANTKRING | Personal research notes; the reviewed page does not provide a complete derivation; hypothesis only, not an official or OFRL-verified solution |
| SRC-007 | Antwerpsche Diamantkring | https://diamantkring.be/about/ | 2026-11-05 | Verify the existence and nature of the candidate entity named in H-003 | Official organization website; citation only; no content reproduced; does not verify the puzzle answer |
| SRC-008 | Flemish Government / BELSPO | https://omgeving.vlaanderen.be/sites/default/files/2022-02/ENG_GARMON_final_report_nov_2020.pdf | 2026-11-05 | Contextual corroboration that the Flemish Diamond is a four-agglomeration urban region (Brussels, Ghent, Antwerp, Leuven) | Public government report; citation only; contextual geography, not evidence of puzzle rules or answer |
| SRC-009 | Sony Interactive Entertainment | https://www.playstation.com/content/dam/global_pdc/en/corporate/support/manuals/accessories/ps5-accessories/dualsense-wireless-controller-cfi-zct1w/EN_DualSense_Wireless_Controller_CFI-ZCT1W_Instruction_Man_Web.pdf | 2026-11-05 | Verify the names and physical positions of the four PlayStation DualSense face buttons used in H-006 | Official hardware manual; cited and linked only; controller layout context, not puzzle-rule evidence |
| SRC-010 | Major League Baseball | https://img.mlbstatic.com/mlb-images/image/upload/mlb/hhvryxqioipb87os1puw.pdf | 2026-11-05 | Verify the standard top/right/bottom/left placement of second base, first base, home plate, and third base on a baseball diamond | Official Baseball Rules, Appendix 2, Diagram 2; cited and linked only; field-layout context, not puzzle-rule evidence |
| SRC-011 | National Oceanic and Atmospheric Administration | https://oceanservice.noaa.gov/facts/port-starboard.html | 2026-11-05 | Verify port/starboard as left/right when facing the bow | Official NOAA fact page; citation and link only; nautical convention context |
| SRC-012 | U.S. Naval History and Heritage Command | https://www.history.navy.mil/content/history/museums/nmusn/education/educational-resources/ships-to-the-sea-1/nautical-vocabulary-list.html | 2026-11-05 | Verify bow/front and stern/back terminology | Official naval educational reference; citation and link only; nautical convention context |
| SRC-013 | National Geographic Society | https://education.nationalgeographic.org/resource/cardinal-directions-and-maps/ | 2026-11-05 | Verify the standard four compass directions used as N/E/S/W labels | Educational reference; citation and link only; general directional convention |

Jane Street's Puzzles page states that puzzles are published every month or two and explains the firm's own rationale for using puzzles: creative problem-solving is relevant to identifying new problems in financial markets and developing potentially novel ways to solve them. This is recorded as the source's stated motivation, not as an OFRL claim that every puzzle represents a financial problem.

The ASIC article was published on 2026-10-02 and concerns a separate hardware reverse-engineering challenge. It is contextual material only; it is not described here as an item in the monthly puzzle archive. The Puzzling Stack Exchange and Kaia references are exploratory secondary sources only; neither is treated as an authoritative rule source or as proof of a solution. SRC-007 confirms the candidate organization's existence and role as an Antwerp rough-diamond bourse; SRC-008 describes the Flemish Diamond's four-city geometry. Neither source establishes that H-003 is the puzzle answer. SRC-009 through SRC-013 corroborate the conventional positions and vocabulary used in the four candidate layouts in H-006; they do not verify that Jane Street intended those groupings.

## 3. Classification and privacy

OFRL stores only:
- Source URLs and source titles.
- Access date and a short, independently written description of the visible source structure.
- Research notes that distinguish observations from unknown semantics.

OFRL does not store:
- The full Pressure Points puzzle text or sequence list.
- Puzzle screenshots, images, or copied artwork.
- Jane Street's official solution or solution code.
- Personal data about puzzle submitters or any unrelated participant information.

No personal or confidential data is intentionally collected for this pilot.

## 4. Coverage and structure

The retrieved Pressure Points page visibly presents a leading symbol, a block of uppercase-letter sequences, a separate emoji-clue block, and a terminal notation. The formal meaning of these elements has not been established.

No normalized puzzle dataset or redistribution-ready fixture is created in version 0.3.0.

### Official-solution retrieval status

The Jane Street Archive search-index representation returned during the 2026-11-05 review displayed “November 2026: Pressure Points — Puzzle Solution”. However, the directly opened Archive representation was an older cached snapshot beginning at August 2026 and did not contain the November entry. Attempts to open likely solution-page paths did not retrieve solution content. The live puzzle page still exposes the puzzle and correct-submission list, but no complete official solution text was retrieved in this review. Therefore official-solution availability is **UNCONFIRMED**; do not treat the search-index label as proof that an accessible solution page has been inspected. Recheck the Archive and obtain the actual link/content before relying on an official solution.

## 5. Collection or generation method

Source references were reviewed manually on 2026-11-05. The source URLs are retained as the retrieval path. OFRL has not implemented a scraper, automated data collection, or a periodic source monitor.

## 6. Transformations and exclusions

No source puzzle data has been transcribed into a local dataset. The case-specific formalization note contains only limited structural observations, not a reconstruction of the full puzzle.

## 7. Quality controls

- Confirm that each source reference points to the original publisher.
- Record the date accessed.
- Do not infer redistribution permission from public accessibility.
- Keep source observations separate from interpretations and assumptions.
- Recheck the original source before any later update to the formalization.

## 8. Integrity and versioning

No third-party source file is stored, so no source-file checksum is claimed. If an independently created synthetic fixture is proposed later, its generation method, schema, checksum, and rights assessment must be documented before inclusion.

## 9. Limitations and known risks

- The live source may change after the access date.
- The page's visible content does not, by itself, establish all problem rules.
- The Archive's indexed “Puzzle Solution” label could not be reconciled with the older page snapshot returned by direct retrieval; official solution access remains unconfirmed.
- No right to reproduce the puzzle text, images, or official solution is assumed.
- The separate ASIC article provides context on methods and verification but is not evidence about the Pressure Points puzzle's underlying structure.

## 10. Access, retention, and release decision

**Stored material:** Source references and limited OFRL-authored notes.  
**Storage:** Public OFRL repository.  
**Redistribution of third-party puzzle material:** Not established; not included.  
**Release approval:** This manifest is an OFRL-authored provenance record. It does not grant permission to redistribute Jane Street materials.
