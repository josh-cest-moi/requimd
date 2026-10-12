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

## Later / ideas

- FR-112 (EARS detection, with EARS keyword highlighting)
- Issue tracker links (e.g. `jira: SHOP-412`)
- Test suite integration: import results (JUnit XML covers most
  frameworks) so test items show pass/fail, e.g. on the site; open: the
  lifecycle of tests and how automated tests are matched to items (test
  names or annotations with IDs)
- Editor integrations: a VS Code extension that starts `requimd lsp`
  (Marketplace and Open VSX); for IntelliJ first a setup guide for the
  LSP4IJ plugin, a dedicated plugin only on demand
- On-demand AI review for ambiguity and contradictions (stage 3, ADR-0005)
- Further pattern rules: comparatives without reference, passive voice
  without actor
- Show what changed in the target of a suspect link, from git history
  (ADR-0007)
- Suspect links in highlighting: a `suspect` modifier on references
  (extension of FR-216, like `weakWord`), so editors can colour them
  separately from other warnings
- Glossary entry stub: as an analyst, I can select a word or phrase and
  create a glossary entry stub for it (code action, like "Add to weak
  words" in FR-219)
- Design layer: architecture decisions (ADRs) and other decision entries
  as items, with links to the requirements they answer, so decisions take
  part in tracing, suspect links and the site
- Unverified requirements: a check on functional requirements and stories
  that no item links to with `ver`. Verification is not only by tests:
  review, demonstration or analysis records count too (see the open
  question on verification methods in `docs/status.md`)
- Glossary navigation: go to definition on a glossary term or alias jumps
  to its entry; find references lists every use of the term and its
  aliases (today terms have hover only)
- Near-miss IDs: a warning on headings such as `## FR-12 Title` that look
  like an item but are none (too few digits, BR-001)
- Changelog on the site: `requimd export site --since <ref>` adds a page
  "changed since <ref>" built from `requimd diff` (FR-403)
- Unused glossary terms: an info diagnostic on terms that no item uses
- `requimd export mermaid --plain`: the diagram without the Markdown code
  block, for piping into mermaid-cli (`mmdc`)
