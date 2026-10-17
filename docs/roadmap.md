# Roadmap

Each milestone lists the requirements in scope. A milestone is done when
all its `must` items are implemented, each has a test containing its ID
(NFR-010), and `requimd check` passes on this repository once available.

## M0 Specification (done)

Spec in `requirements/`, decisions in `docs/decisions/`. Rust chosen
(ADR-0001). Current state and open questions: `docs/status.md`.

## M1 Parser and check command (done)

Goal: `requimd check` validates this repository and the examples in CI.

- Prerequisite: ADR-0003 (Markdown parser) confirmed
- Setup: cargo workspace, CI workflow per ADR-0004
- Format: FR-001 to FR-010, FR-012 to FR-014
- Checks: FR-100 to FR-109, FR-115, FR-117
- CLI: FR-300 to FR-303
- Quality: NFR-002, NFR-004, NFR-006, NFR-008; NFR-010 applies to every milestone
- Rules: BR-001, BR-003, BR-005

## M2 Language server core (done)

Goal: writing and navigating in Neovim 0.12 without plugins.

- LSP: FR-200 to FR-208, FR-210 to FR-212, FR-217, FR-308
- Quality: NFR-001, NFR-003, NFR-007, NFR-009
- Editor guide in `docs/neovim.md` verified feature by feature

## M3 Authoring comfort (done)

- Format: FR-011 (glossary)
- Checks: FR-110, FR-111, FR-113, FR-114, FR-118, FR-119
- LSP: FR-209, FR-213 to FR-215, FR-219
- CLI: FR-304 to FR-306

## M4 Impact analysis (done)

- Lock file format: ADR-0007 (accepted)
- FR-116, FR-218, FR-307; lock file in FR-212, FR-214 and FR-300
  (`--locked`)

## M5 Publishing (implemented)

- FR-400 to FR-402; FR-405 deprecated (no Confluence; PDFs come from an
  external Markdown-to-PDF converter)
- NFR-005 and release packaging: binaries on GitHub Releases for Linux,
  macOS and Windows; license MIT
- CI builds the site of this repository as a downloadable artifact;
  publishing to GitHub Pages later

## After M5

- FR-216 (semantic tokens): done on 2026-10-12, moved ahead by the product
  owner; EARS keywords follow with FR-112
- FR-403 (`requimd diff`): done on 2026-10-12
- FR-404 (`requimd export mermaid`): done on 2026-10-12

## M6 Editor integrations (done)

Goal: requimd in VS Code with one installation, and in IntelliJ IDEA with a
guide. Decisions: ADR-0008 (proposed).

- VS Code: FR-500 to FR-503, NFR-011; `.vsix` packages per platform on
  GitHub Releases, built with the binaries
- IntelliJ: FR-504 (`should`), setup guide for LSP4IJ; moved to the first
  release after v0.1.0

## M7 Release v0.1.0

Goal: the first release without alpha suffix. The specification is
verified and approved by the product owner; writing has no silent
failures; the stability of the file formats is promised; stakeholders can
read the site and the matrix. Scope from `docs/feature-report.md`
(section G, IDs such as D1 refer to it) and the decision log of
`docs/status.md`.

The work runs in five steps. Within a step, pull requests can run side by
side; a `spec/` or `decision/` pull request comes before the code that
implements it.

### 1. Verdicts and decisions (product owner)

- Verdicts per item in `docs/verification-v0.1.0.md`, with the scenarios
  and acceptance tests of `docs/testing-guide.md` (E1)
- Accept or change ADR-0008 (E9)
- Finish or drop the prepared `release/v0.1.0-alpha.3` (E9)

### 2. Specification (`spec/` and `decision/` pull requests)

- Consolidate the spec from the verdicts and from sections 6 and 7 of the
  testing guide: contradictions, gaps, the code's answers, the open
  interpretations Q5 to Q10 (E3)
- Vision and scope: analysts write in German and English; the design
  level as long-term goal (E2, F1, F3)
- Configurable language checks: required verb, story pattern, built-in
  lists that can be switched off, the item types each check applies to,
  the name of the `ac` field and an own check ID for a missing one
  (FR-110, FR-111, FR-113, FR-118; F1, F4)
- The names with a fixed meaning, in FR-014 or a new requirement (F4)
- The key `method` with its value set for the type `test` (FR-014, F2)
- FR-012 keeps robustness only; a new FR-120 for formatting slips: a
  colon after the ID, `key:value` without space, a blank line before the
  metadata, an unclosed code fence, near-miss IDs (D1, D2)
- HTML comments leave the hash of suspect links (FR-116; a decision that
  amends ADR-0007; D7)
- Stability promises for the format, `requimd.toml`, `requimd.lock` with
  a version field, and the JSON export: a new ADR (E4)
- The version in the site footer (FR-400); a new FR-406, a guide to
  publishing the site from CI, covers UC-006.s3 (C2)
- NFR-006 covers all commands; network only for explicit opt-in commands
  with their own ADR (F5)
- `--version` and `--help` as requirements (E5)
- The acceptance tests in `acceptance/` follow each change; their `ver`
  links become suspect when a requirement changes
- Then the product owner sets `status: approved` (BR-004), in the order of
  section 1 of the testing guide

### 3. Implementation (`feature/` and `fix/` pull requests)

The hash and the lock file come first, since they decide what existing
lock files mean.

| Work | Requirements | Effort |
|------|--------------|--------|
| Hash without HTML comments; version field in the lock file | FR-116, ADR-0007 amendment, E4 | S |
| Configurable language checks, and a German example configuration in the documentation | FR-110, FR-111, FR-113, FR-118 | M |
| Check for formatting slips | new FR-120 | M |
| Site renders tables, strikethrough and task lists; version in the footer | FR-400 | S |
| Matrix CSV for Excel: byte order mark, semicolon as option | FR-401 | S |
| Exclude patterns; nested projects left out | FR-013 | S |
| `requimd fmt` records the IDs it assigns in the lock file | FR-304, FR-212 | S |
| The key `method` in the defaults | FR-014 | S |
| Tests for `--version` and `--help` | E5 | S |

### 4. Release readiness

| Work | Kind | Effort |
|------|------|--------|
| Release workflow: site on GitHub Pages on full releases, site ZIP on every release; publishing guide | CI, new FR-406 | S |
| Checksums for the release files; smoke test of the macOS x86_64 build | CI | S |
| Licence notices of the bundled libraries in the archives and `.vsix` packages | CI | S to M |
| A guide for analysts: getting started and the format by example | docs | M |
| The NFR-010 test lists the `must` items of M7 | test | S |

### 5. Acceptance and release

- A release candidate as pre-release (for example `v0.1.0-rc.1`); the
  scenarios and acceptance tests run on it, results in
  `docs/verification-v0.1.0.md`
- Product owner: Pages source "GitHub Actions" in the repository settings
- Tag the baseline of the specification and release `v0.1.0`

### Done when

- every item has a verdict, and every item that is not deprecated is
  `approved`
- every `must` item of M7 is implemented and has a test containing its ID
  (NFR-010)
- CI passes on Linux, macOS and Windows, and
  `requimd check --strict --locked` passes on this repository
- the acceptance tests ran on the release candidate without open bugs
- the stability ADR is accepted
- the release has binaries, `.vsix` packages, checksums, licence notices
  and the site ZIP, and the site is on GitHub Pages

## After v0.1.0

### Next release

- FR-504 IntelliJ setup guide, from M6 (A1)
- Unverified requirements: a check on functional and quality
  requirements and stories that no item links to with `ver`; every `ver`
  link counts, with its `method` (B10, F2)
- Show what changed in the target of a suspect link, from git history
  (ADR-0007; B6)
- Suspect links in highlighting: a `suspect` modifier on references
  (extension of FR-216, like `weakWord`), so editors can colour them
  separately from other warnings (B7)
- Glossary entry stub: as an analyst, I can select a word or phrase and
  create a glossary entry stub for it (code action, like "Add to weak
  words" in FR-219; B8)
- Glossary navigation: go to definition on a glossary term or alias jumps
  to its entry; find references lists every use of the term and its
  aliases (today terms have hover only; B11)
- Changelog on the site: `requimd export site --since <ref>` adds a page
  "changed since <ref>" built from `requimd diff` (FR-403; B13)
- Unused glossary terms: an info diagnostic on terms that no item uses
  (B14)
- `requimd export mermaid --plain`: the diagram without the Markdown code
  block, for piping into mermaid-cli (`mmdc`; B15)
- VS Code Marketplace and Open VSX, once publisher accounts exist (C1)
- `.gitignore` honoured when collecting the files of a project (D5)
- The site export removes files of earlier exports (D9)
- An ADR on the traceability model: decisions as items first (B9), or the
  design layer directly (F3)

### Later

- FR-112 EARS detection with EARS keyword highlighting; keywords
  configurable per language (A2, F1)
- Decisions as items or the design layer, as the ADR on the traceability
  model decides: architecture decisions and design items with links to
  the requirements they answer or satisfy, so they take part in tracing,
  suspect links and the site; tests per level (B9, F3)
- Issue tracker links (e.g. `jira: SHOP-412`; B1)
- Test suite integration: import results (JUnit XML covers most
  frameworks) so test items show pass/fail, e.g. on the site; open: the
  lifecycle of tests and how automated tests are matched to items (test
  names or annotations with IDs; B2)
- Further pattern rules: comparatives without reference, passive voice
  without actor; configurable per language (B5)

### On demand

- Dedicated IntelliJ plugin (ADR-0008; B3)
- AI review for ambiguity and contradictions (stage 3, ADR-0005), as an
  explicit opt-in command with its own ADR (NFR-006; B4, F5)
