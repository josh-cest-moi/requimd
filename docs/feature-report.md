# Feature report

Open, planned and possible features of requimd, as the basis for the scope
of v0.1.0 and the roadmap after it (step 3 in section 1 of
`docs/testing-guide.md`). As of 2026-10-16. The proposals are mine; the
product owner decides.

## Summary

- **43 entries in six groups:** specified but not built (2), the
  "Later / ideas" list of `docs/roadmap.md` (15 more), deferred in
  decisions (2), possible features found during the verification (10),
  release readiness (9), and open questions that decide several entries
  at once (5).
- **Five questions decided much of the rest** (section F, decided with the
  product owner on 2026-10-16):
  - requirement texts in German and English through configurable
    language checks, in v0.1.0 (F1);
  - every `ver` link is a verification record, with a `method` (F2);
  - v0.1.0 covers requirements; a design level is the long-term goal
    (F3);
  - fixed names are specified, the language checks get configurable
    (F4);
  - offline by default, network only for explicit opt-in commands (F5).
- **Proposal:** v0.1.0 is about trust, not new capabilities:
  - a verified specification;
  - no silent failures when writing (D1, D2, D5, D6);
  - files whose format stays stable (D7, E4);
  - a site and a matrix that stakeholders can read (D3, D4).
- **After v0.1.0:** the "Later" list follows, starting with the small
  entries that help with impact analysis and the glossary.

## How to read

**Value** names the business objective of `requirements/vision.md` a
feature serves:

| | Objective |
|-|-----------|
| O1 | Writing a linked requirement takes no more keystrokes than in a wiki |
| O2 | Broken or missing links are found automatically |
| O3 | The impact of a change becomes visible when it is made |
| O4 | Requirements get the workflow of code |
| O5 | Readers without git or an editor can read the requirements |

**Effort**, estimated from the code read in phase 2 of the testing guide,
including tests and spec text:

- **S:** up to a day.
- **M:** a few days.
- **L:** a week or more, or a design is needed first.

**Proposal:**

- **v0.1.0:** in the first release.
- **next:** the first release after it.
- **later:** a later release.
- **on demand:** only when someone asks for it.
- **decide:** a decision is needed, not code.

## A. Specified, not built

| # | Feature | Source | Value | Effort | Depends on | Proposal |
|---|---------|--------|-------|--------|------------|----------|
| A1 | IntelliJ setup guide (FR-504) | M6, `should` | O1 for IntelliJ users | M | IntelliJ IDEA with LSP4IJ | next, unless your analysts use IntelliJ |
| A2 | EARS patterns with hover and highlighting (FR-112) | `could`, "Later" | wording quality | M | F1: keywords configurable per language | later |

**A1:** the effort is mostly manual: following the guide in IntelliJ and
checking each feature, as `docs/neovim.md` did for Neovim. No code change
is expected, since the server uses only standard LSP (NFR-007). M6 is done
apart from this item.

**A2:** the EARS keywords ("When", "While", "If … then", "Where") are
English. FR-111 already reports texts without "shall"; EARS would add the
pattern on top.

## B. The "Later / ideas" list

The roadmap lists 16 ideas; FR-112 is A2. Three of them are also
deferred in decision records: B3 and B4 in ADR-0008 and ADR-0005, B6 in
ADR-0007.

| # | Feature | Value | Effort | Depends on | Proposal |
|---|---------|-------|--------|------------|----------|
| B1 | Issue tracker links such as `jira: SHOP-412` | O4 | M | link keys to external IDs with a URL template; offline, so their targets are not checked (NFR-006) | later |
| B2 | Test results from JUnit XML, shown as pass or fail, for example on the site | O2, O5 | L | F2, F3; how automated tests are matched to items | later |
| B3 | Dedicated IntelliJ plugin | O1 | L | A1; demand | on demand |
| B4 | AI review for ambiguity and contradictions (stage 3 of ADR-0005) | wording quality | L | F5: an explicit opt-in command with its own ADR; not for CI | on demand |
| B5 | More pattern rules: comparatives without reference, passive voice without actor | wording quality | M each | F1: patterns configurable per language; false positives | later |
| B6 | Show what changed in the target of a suspect link, from git history | O3 | M | the `git` command, as in FR-403 | next |
| B7 | A `suspect` modifier in highlighting | O3 | S | FR-216, FR-503 | next |
| B8 | Glossary entry stub: a code action on a selected word | O1 | S | – | next |
| B9 | Decisions as items, linked to requirements | O4 | M | F3; an ADR on the traceability model | after v0.1.0: decide whether this comes before the design layer |
| B10 | Unverified requirements: a check on items that no item links to with `ver` | O2 | S to M | F2: any `ver` link counts | next |
| B11 | Glossary navigation: go to definition and find references on terms and aliases | O1 | S | – | next |
| B12 | Near-miss IDs: a warning on headings such as `## FR-12 Title` | O2 | S | – | v0.1.0, as part of D1 |
| B13 | Changelog page on the site, `export site --since <ref>` | O4, O5 | M | FR-403 | next |
| B14 | Unused glossary terms: an info diagnostic | wording quality | S | – | next |
| B15 | `export mermaid --plain` without the Markdown code block | O5 | S | – | next |

**B6:** the lock file stores only a hash. To show the reviewed version,
requimd has to search the git history of the target's file for the
version with that hash. Feasible with the same `git` calls as
`requimd diff`, and offline.

**B10:** the acceptance tests in `acceptance/` already link with `ver`;
this check would show which requirements have no test or other
verification record. What counts as a record is F2.

## C. Deferred in decisions

| # | Feature | Source | Value | Effort | Depends on | Proposal |
|---|---------|--------|-------|--------|------------|----------|
| C1 | VS Code Marketplace and Open VSX | ADR-0008, log 2026-10-14 | O1: installation and automatic updates | S | publisher accounts of the owner; the publisher name `josh-cest-moi` is a placeholder | next |
| C2 | Site of the specification on GitHub Pages | `docs/roadmap.md` M5, `docs/status.md` | O5 | S | your Pages setting (source: GitHub Actions); a requirement for UC-006.s3 | v0.1.0 (decided) |

**C2:** step UC-006.s3, "Analyst publishes the website", is covered only
by FR-405, which is deprecated. Decided on 2026-10-17: the repository is
public anyway; the release workflow publishes the site on GitHub Pages on
full releases only, from v0.1.0, and attaches it as ZIP to every release,
pre-releases included; the site footer shows the version. A new FR-406, a
guide to publishing the site from CI on GitHub Pages and GitLab Pages,
covers UC-006.s3, with this repository's workflow as example.

## D. Possible features from the verification

Found while writing the testing guide (sections 6 and 7). Some are fixes
of silent behaviour rather than features.

| # | Feature | Value | Effort | Depends on | Proposal |
|---|---------|-------|--------|------------|----------|
| D1 | Warnings for silent slips: a colon after the ID (TC-035), `key:value` without space (TC-036), a blank line between heading and metadata (TC-010), an unclosed code fence (TC-028), near-miss IDs (B12) | O2 | M | – | v0.1.0 |
| D2 | FR-012 asks for a diagnostic on malformed content, and the code records none: either add one or adjust FR-012 | O2 | S (spec) | D1 | v0.1.0 (decided): FR-012 keeps robustness; a new FR-120 lists the cases of D1 |
| D3 | The site renders tables, strikethrough and task lists as GitHub does (TC-412) | O5 | S | items stay plain CommonMark (ADR-0002); only the rendering changes | v0.1.0 |
| D4 | A matrix CSV that Excel opens correctly: byte order mark, semicolon as an option (TC-403) | O5 | S | – | v0.1.0 |
| D5 | Control over the files of a project: exclude patterns, nested projects left out, possibly `.gitignore` (TC-032) | O2: no findings from unrelated files | S, `.gitignore` M | `.gitignore` needs one more library | v0.1.0 for exclude and nested projects, `.gitignore` next |
| D6 | `requimd fmt` records the IDs it assigns in the lock file (TC-310) | BR-002 | S | – | v0.1.0 |
| D7 | Inline allow markers do not count in the hash of a suspect link (FR-116) | O3: no false suspect links | S | changes the hash of every item with a comment, so it must come before E4 | v0.1.0 (decided): all HTML comments leave the hash |
| D8 | Fixed type and key names (`usecase`, `functional`, `impl`, `ac` …): specify them as fixed, or make them configurable | robustness | S | F4 | v0.1.0: specify; type scope of the language checks and the `ac` field configurable with D10; structural names with the traceability model |
| D9 | Site: labels in other languages; files of an earlier export removed | O5 | S | F1: output stays English | stale files next; labels dropped |
| D10 | Requirement texts in another language than English: word lists, modal verbs, "shall", the story pattern | O1, O2 for non-English teams | M | F1 | v0.1.0: configurable, English shipped |

**D1:** each slip silently turns a requirement or a link into plain text,
which is exactly what requimd promises to catch (O2). One check, for
example `suspicious-format`, with a message per case, keeps the
configuration simple.

**D7:** today, "Allow here" (FR-219) on a word in a target item makes
every link to that item suspect. Changing the hash later would make many
links of all projects suspect at once; before v0.1.0 only alpha users are
affected. Decided on 2026-10-17: the hash leaves out all HTML comments,
so it covers what readers see, as the site, the language checks and
GitHub do. Alpha users run `requimd review --all` once after the
update.

**D10:** a German analyst writing "Das System muss …" gets a `no-shall`
info on every requirement, and "soll", "kann" or "schnell" go unnoticed
unless added to `requimd.toml`: weak words and modal verbs can be
configured today, while "shall", the default lists and the story pattern
are English. Decided in F1: everything English becomes configurable
(required verb, story pattern, built-in lists switched off, an own check
ID for a missing `ac`); requimd ships English only, and the documentation
gets a German example configuration.

## E. Release readiness for v0.1.0

Not features, but needed for a first release without alpha suffix.

| # | Topic | Kind | Effort |
|---|-------|------|--------|
| E1 | Verdicts and approvals: record verdicts, set `status: approved` (BR-004), tag a baseline | owner | – |
| E2 | Rewrite vision and scope in `requirements/vision.md` | spec | S |
| E3 | Consolidate the spec from the verdicts: contradictions, gaps, the code's answers (guide sections 6 and 7) | spec | M |
| E4 | Stability promises for the format, `requimd.toml`, `requimd.lock` and the JSON export; a version field in the lock file; the hash frozen (decide D7 first) | ADR | S to M |
| E5 | `--version` and `--help` as requirements; both exist | spec | S |
| E6 | A guide for analysts: getting started and the format by example | docs | M |
| E7 | Checksums for the release files; a smoke test of the macOS x86_64 build | CI | S |
| E8 | Licence notices of the libraries bundled in the executable and the `.vsix` packages | CI, licence | S to M |
| E9 | Accept or change ADR-0008; finish or drop `release/v0.1.0-alpha.3`; the version `0.1.0` in `Cargo.toml` and the extension | owner, release | S |

**E4:** v0.1.0 is still a 0.x version, so breaking changes remain
possible. Analysts will nevertheless commit `requimd.toml` and
`requimd.lock` to their repositories. A short ADR that says what may
change and how requimd handles an older lock file avoids surprises.

## F. Questions that decide several entries

Decided with the product owner on 2026-10-16 and recorded in the decision
log of `docs/status.md`.

| # | Question | Decision | Affects |
|---|----------|----------|---------|
| F1 | Which languages can requirement texts be written in? | German and English, through configuration: the required verb, the story pattern and the built-in lists become configurable, a missing `ac` gets its own check ID; English is shipped. Messages, hover and site stay English. In v0.1.0. | A2, B5, D9, D10 |
| F2 | Which verification records count, and should a requirement state its method (open business question 11)? | Every item with a `ver` link; records get a `method` key (test, inspection, analysis, demonstration) in the defaults, in v0.1.0. No `verify:` on requirements for now. | B2, B10 |
| F3 | Design between requirements and tests (open business question 12) | v0.1.0 covers requirements only. The long-term goal is a model with requirement and design level, recorded in the vision. After v0.1.0, decide whether decisions as items (B9) come first or the design layer directly; an ADR on the traceability model comes first and decides status values per type. | B2, B9 |
| F4 | Fixed names in the code (technical question 1) | The spec records them as current behaviour, not as a lasting promise. With F1, the type scope of the language checks and the name of the `ac` field become configurable; the structural names (`usecase`, `impl`, `ref`, `ver`, `status`) follow with the traceability model. | D8 |
| F5 | May an optional command use the network (NFR-006)? | Offline by default: NFR-006 covers all commands from v0.1.0; a future command may use the network only when invoked explicitly and configured, with its own ADR; `check`, `fmt` and `lsp` stay offline. | B4, possibly B1 |

### Spec changes that follow

For the consolidation (E3), each in a `spec/` pull request:

- `requirements/vision.md`: users write in German and English; the design
  level as long-term goal (E2).
- FR-110, FR-118: built-in lists can be switched off; the item types a
  check applies to are configurable.
- FR-111: the required verb is configurable ("shall" by default).
- FR-113: the story pattern is configurable; a missing `ac` field gets its
  own check ID, and the field name is configurable.
- FR-014: the type `test` gets the key `method` with its value set; the
  new check IDs.
- A requirement, or text in FR-014, on the names with a fixed meaning.
- NFR-006: the scale covers all commands; the opt-in principle.
- FR-012: robustness only; a new FR-120 with its own check ID for the
  cases of D1 (D2).
- FR-116 and ADR-0007: the hash leaves out HTML comments; ADR-0007 is
  accepted, so a superseding or amending decision is needed (D7).
- FR-400: the version in the site footer; a new FR-406, the publishing
  guide, implements UC-006.s3 (C2).
- ADR candidates: the language configuration (F1) and the network
  principle (F5).

## G. Proposal

| When | Entries |
|------|---------|
| v0.1.0 | E1 to E9; D1 with B12 as FR-120 (D2), D3, D4, D5 (exclude, nested projects), D6, D7 (comments out of the hash), D8 (specified), D10 with the configurable scope of D8 (F1, F4); the `method` key (F2); NFR-006 for all commands (F5); C2 (Pages on full releases, site ZIP on every release, version in the footer, FR-406 guide) |
| next | A1, B6, B7, B8, B10, B11, B13, B14, B15, C1, D5 (`.gitignore`), D9 (stale files); the ADR on the traceability model, with the choice between B9 and the design layer (F3) |
| later | A2, B1, B2, B5, B9 or the design layer (F3) |
| on demand | B3, B4 |
| dropped | D9 (site labels in other languages, F1) |

The new code for v0.1.0 is mostly small (S); D1, D10 and E3 take a few
days each. Every entry would become its own pull request, with a `spec/`
part first where requirements change. The decisions on F1 to F5, D2, D7
and C2 are in the decision log of `docs/status.md`. The result is
milestone M7 and the list after it in `docs/roadmap.md` (step 4 of
section 1 of the testing guide).
