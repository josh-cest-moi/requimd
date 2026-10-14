# Project status

As of 2026-10-13. Update this file at the end of every working session.

## State

- **M0 Specification: done.** Spec in `requirements/` (7 use cases, 5
  business rules, 69 functional and 10 quality requirements, all `draft`),
  fixtures in `examples/`, decisions in `docs/decisions/`.
- **CI:** green on Linux, macOS and Windows since 2026-10-12; all three
  are required jobs (NFR-004 `plan`). Windows first failed because Neovim
  on Windows reports garbled file-watch paths below 8.3 short paths such as
  the runner's `C:\Users\RUNNER~1`; the server now rescans on events that
  do not fit the disk. CI runs `requimd check --locked` before the tests.
- **M1 Parser and check command: done.** Cargo
  workspace with `crates/requimd-core` (config, parser, item model),
  `crates/requimd-checks` and `crates/requimd-cli` (binary `requimd`). All `must`
  items of M1 have tests named after their IDs (80 tests, NFR-010 checked by
  a test). `requimd check` passes on this repository and `examples/shop`;
  `examples/broken` yields exactly the M1 rows of `EXPECTED.md`.
- **M2 Language server core: done.** New crate
  `crates/requimd-lsp` (lsp-server, synchronous loop, full text sync, UTF-8
  or UTF-16 positions as the client offers), started by `requimd lsp`. All
  `must` items of M2 have tests; end-to-end tests run over an in-memory
  connection, including NFR-001 (latency), NFR-003 (start) and NFR-009
  (10,000 random edits). NFR-007 is checked against Neovim 0.12.6 without
  plugins by `scripts/nvim-check.lua`, which `cargo test` runs when `nvim`
  is installed; all features pass (see `docs/neovim.md`).
- **M3 Authoring comfort: done.** Glossary (FR-011),
  language checks FR-110, FR-111, FR-113, FR-114, FR-118, FR-119 (core
  module `words.rs`, checks module `language.rs`), language server features
  FR-205 glossary hover, FR-209, FR-213 to FR-215, FR-219 (`actions.rs`,
  `requimd.toml` edits with toml_edit), CLI `fmt`, `show`, `coverage`
  (FR-304 to FR-306). `examples/broken` now yields all 22 rows of
  `EXPECTED.md`. All M3 features also pass in Neovim 0.12.6
  (`scripts/nvim-check.lua`). The server caches parse results of unchanged
  files; NFR-001 p95 is 77 ms in a release build (plan: 100 ms).
- **M4 Impact analysis: implemented.** Lock file per ADR-0007 (core module
  `lock.rs`: TOML, FNV-1a content hash, update, review, rename),
  `suspect-link` check (FR-116), `check --locked` (FR-300), `requimd
  review <ID> | --all` (FR-307), next free ID from the lock (FR-212); in
  the language server: lock loaded and watched, links recorded on save,
  "Mark as reviewed" (FR-218), rename rewrites lock entries (FR-214).
  `requimd.lock` is committed for this repository and both examples. All
  M4 features pass in Neovim 0.12.6.
- **M5 Publishing: implemented.** `requimd export site | matrix | json`
  (FR-400 to FR-402, CLI module `export/`): one self-contained HTML page per
  source file (CSS embedded, no JavaScript), index, traceability matrix as
  page and CSV, JSON with format version. FR-405 deprecated. License MIT
  (`LICENSE`). `.github/workflows/release.yml` builds static binaries for
  Linux (musl), macOS (arm64, x86_64) and Windows (static CRT) on a version
  tag and attaches them to a GitHub release (NFR-005). CI uploads this
  repository's site as artifact `requimd-site`. Open: first release tag.
- **FR-216 Semantic tokens: done** (after M5, moved ahead by the owner).
  Item IDs, references, keys, step labels, glossary terms and the findings
  of the weak-word, modal-verb and glossary-alias checks, as standard LSP
  token types (`docs/neovim.md`, "Highlighting"). Passes in Neovim 0.12.6.
- **FR-403 Requirement changelog: done.** `requimd diff <ref1> [ref2]`
  (CLI module `diff.rs`) reads references with the `git` command (`ls-tree`,
  one `cat-file --batch`), compares by ID and names the changed parts;
  text or `--format json`. Tested against a real git repository with the
  project in a subdirectory.
- **FR-404 Diagram export: done.** `requimd export mermaid <ID> [--depth n]
  [-o file]` (CLI module `export/mermaid.rs`): Mermaid flowchart in a
  Markdown code block. Rendering checked with Mermaid 11.4 in a browser.
  All `could` items after M5 are done.
- **M6 Editor integrations: in progress.** Specified on 2026-10-13
  (FR-500 to FR-504, NFR-011, UC-007.s2a and s2b, ADR-0008 proposed).
  VS Code extension in `editors/vscode/` (FR-500, FR-501, FR-503): one
  language client per project root; server commands registered once by the
  extension and sent to the active editor's project; code lens opens the
  reference view; finding modifiers underlined or italic in every theme.
  19 tests in VS Code 1.91.1 (NFR-011), CI job `vscode` on Linux, run
  twice: against the source and against a packaged `.vsix` with its
  bundled executable. The release attaches one `.vsix` per platform with
  the release binary (FR-502). Guide `docs/vscode.md`. All `must` items of
  M6 are done; open: IntelliJ guide (FR-504).

## Decision log

| Date | Decision | Where recorded |
|------|----------|----------------|
| 2026-10-14 | No store publishing planned; VS Code packages only on GitHub Releases | `docs/roadmap.md` |
| 2026-10-14 | The extension carries the Cargo version, alpha suffix included; the release fails if they differ | `.github/workflows/release.yml` |
| 2026-10-14 | VS Code tests download a pinned VS Code (oldest supported, 1.91.1) in CI, cached, one Linux job | `docs/vscode.md`, `.github/workflows/ci.yml` |
| 2026-10-14 | One language client per project root; the extension registers the server commands once | `editors/vscode/src/extension.ts` |
| 2026-10-14 | Finding modifiers mapped to underline and italic, not colours, since default themes have no warning colour scope | FR-503, `docs/vscode.md` |
| 2026-10-14 | No test framework for the VS Code tests: a small PASS/FAIL runner like `nvim-check.lua` (mocha 12 does not load in VS Code 1.91, mocha 11 has audit findings) | `editors/vscode/src/test/` |
| 2026-10-14 | Extension publisher name `josh-cest-moi` until a Marketplace account exists | `editors/vscode/package.json` |
| 2026-10-13 | Work goes through GitHub pull requests; the owner merges | `CLAUDE.md` |
| 2026-10-13 | New milestone M6 Editor integrations; new ID range FR 500–599 | `docs/roadmap.md`, `requirements/README.md` |
| 2026-10-13 | VS Code extension with the binary bundled per platform; `.vsix` packages on GitHub Releases only, Marketplace and Open VSX later | FR-500 to FR-502, ADR-0008 |
| 2026-10-13 | IntelliJ: setup guide for LSP4IJ, dedicated plugin only on demand | FR-504, ADR-0008 |
| 2026-10-13 | VS Code `must`, IntelliJ guide `should` | FR-500 to FR-504 |
| 2026-10-12 | Mermaid diagram as `requimd export mermaid <ID>`, Markdown code block, direct neighbours by default, `--depth` for more; steps inside their use case, mentions dashed, status per node | FR-404 |
| 2026-10-12 | `requimd diff` calls the `git` command, no git library (NFR-005) | FR-403 |
| 2026-10-12 | `requimd diff <ref>` compares with the working tree, including uncommitted changes | FR-403 |
| 2026-10-12 | Changed = title, text, links, other metadata, steps or file differ (whitespace-insensitive as FR-116); status listed with old and new value | FR-403 |
| 2026-10-12 | FR-216 before FR-403/FR-404; adds step labels, body fields, glossary terms, modal verbs and aliases; EARS keywords move to FR-112 | FR-216, FR-112, `docs/roadmap.md` |
| 2026-10-12 | Weak words, modal verbs and aliases are highlighted only where a diagnostic is reported | FR-216 |
| 2026-10-12 | No Confluence: FR-405 deprecated; stakeholders get PDFs from an external Markdown-to-PDF converter | FR-405 |
| 2026-10-12 | Site shows all items with status badge, `--status` filters; no JavaScript | FR-400 |
| 2026-10-12 | Traceability matrix: one row per step with implementing and verifying items | FR-401 |
| 2026-10-12 | License MIT for now; adding Apache-2.0 later stays possible while there is one author | `LICENSE`, `docs/roadmap.md` |
| 2026-10-12 | Releases as binaries on GitHub Releases | `docs/roadmap.md` |
| 2026-10-12 | Alpha pre-releases (`v0.1.0-alpha.N`) for review rounds before v0.1.0; tags with a suffix become GitHub pre-releases; manual workflow runs build without publishing | `.github/workflows/release.yml` |
| 2026-10-12 | CI builds the site as artifact; GitHub Pages later | `docs/roadmap.md` |
| 2026-10-12 | FR-403 and FR-404 after M5 | `docs/roadmap.md` |
| 2026-10-12 | The language server rewrites lock entries of a rename only once the documents show it, so a cancelled rename changes nothing | FR-214, `crates/requimd-lsp/src/server.rs` |
| 2026-10-12 | Lock file `requimd.lock` in TOML, sorted, one line per link, plus `[ids]` | ADR-0007 |
| 2026-10-12 | `check` adds and removes lock entries but never changes a hash; `--locked` for CI; the language server records on save | ADR-0007, FR-116, FR-300, FR-218 |
| 2026-10-12 | Suspect-link hash ignores `status` and whitespace | ADR-0007, FR-116 |
| 2026-10-12 | `requimd review --all` marks all suspect links as reviewed | FR-307 |
| 2026-10-12 | Hash only; showing what changed via git history is on the "Later" list | ADR-0007, `docs/roadmap.md` |
| 2026-10-12 | No lock entries for items without a unique ID; `[ids]` never decreases; rename rewrites lock entries; unreadable lock stops check (exit 2) | ADR-0007, FR-116, FR-212, FR-214, FR-300 |
| 2026-10-12 | Language server resolves paths to their real spelling and rescans the disk on file events that do not fit it (Neovim on Windows, 8.3 short paths) | FR-217, `crates/requimd-lsp/src/server.rs` |
| 2026-10-12 | Project name requimd (formerly reqmd), spoken "requi-m-d"; binary, config, lock file, marker and crates renamed | ADR-0006 |
| 2026-10-12 | Configured `words` extend the built-in lists; `allow` switches words off | FR-110, FR-118 |
| 2026-10-12 | Built-in weak-word list: the 15 required words plus simple, efficient, sufficient, normally, usually, each with a reason | `crates/requimd-core/src/config.rs` |
| 2026-10-12 | Glossary alias `step` dropped from the spec's own glossary | `requirements/glossary.md` |
| 2026-10-12 | Rename keeps the prefix; changing the type is a manual edit | FR-214 |
| 2026-10-12 | `[lsp] hint-length` (40) and `stub-file`; a dangling step reference adds the step to its use case | FR-209, FR-213 |
| 2026-10-12 | Code lens title lists IDs; running it needs a 4-line handler in the Neovim config, else the server opens the first item | FR-215, `docs/neovim.md` |
| 2026-10-12 | "Add to weak words" writes an empty reason and opens `requimd.toml` there | FR-219 |
| 2026-10-12 | Language checks: title and body text, not metadata; typographic quotes ignored; aliases inside terms not reported; `unused-allow` per marker entry | FR-110, FR-114, FR-119 |
| 2026-10-12 | Performance tests (NFR-001 to NFR-003) enforce their limits in release builds; CI runs them with `--release` | `.github/workflows/ci.yml` |
| 2026-10-11 | Hover length: `[lsp] hover-length`, default 500 characters; glossary hover with FR-011 in M3 | FR-205 |
| 2026-10-11 | FR-212 in M2 uses only IDs in the project; the lock file joins in M4 | this file |
| 2026-10-11 | Go to implementation on a use case includes links to its steps | FR-208 |
| 2026-10-11 | Completion triggers: `-` and `.`; in link values also space and comma; manual completion searches titles | `docs/neovim.md` |
| 2026-10-11 | FR-217: client file watching if offered, else the server polls every 2 s | this file |
| 2026-10-11 | Workspace: `requimd-core`, `requimd-checks`, `requimd-cli`; `requimd-lsp` follows in M2 | this file |
| 2026-10-11 | Built-in defaults = types, links, fields and values of the root `requimd.toml` without `check`/`sev` | FR-014 |
| 2026-10-11 | Project config merged with defaults per entry (`[types.x]`, `[links.x]`, value set) | FR-014 |
| 2026-10-11 | `requimd check [paths]`: whole project loaded, paths only filter output; paths of different projects are a usage error | FR-300 |
| 2026-10-11 | A `dangling-ref` severity override applies to links and mentions | FR-101 |
| 2026-10-11 | Body field keys directly after a heading are body fields, not unknown keys | FR-102 |
| 2026-10-10 | Markdown parser pulldown-cmark, LSP library lsp-server | ADR-0003 |
| 2026-10-09 | Implementation language: Rust | ADR-0001 |
| 2026-10-09 | Hosting and CI: GitHub + Actions | ADR-0004 |
| 2026-10-09 | Duplicate IDs: references to them resolve to no item; only the duplicate is reported | FR-100, `examples/broken/EXPECTED.md` |
| 2026-10-09 | Weak words and glossary aliases are checked in titles too | FR-110, FR-114 |
| 2026-10-09 | Modal verbs ("should", "may", "can"…) get a separate check | FR-118, ADR-0005 |
| 2026-10-09 | Word forms are explicit list entries, no wildcards | FR-110, ADR-0005 |
| 2026-10-09 | False positives silenced by inline HTML comment or project allow list | FR-119, FR-219, ADR-0005 |
| earlier | Markdown dialect: plain CommonMark with conventions | ADR-0002 |

## Open questions

### Business (product owner)

1. **Spec review:** which items move from `draft` to `review` or `approved`?
   Only the owner sets `approved` (BR-004). Proposal: the M1 items
   (FR-001 to FR-014, FR-100 to FR-117, FR-300 to FR-303) to `review`.
2. **Default weak-word list:** draft accepted on 2026-10-12 (see decision
   log). Refine the reasons in `crates/requimd-core/src/config.rs`
   (`WEAK_WORDS`) whenever the wording should change.
3. **Publishing audience:** decided on 2026-10-12: the static site, and PDFs
   from an external Markdown-to-PDF converter; no Confluence (FR-405
   deprecated).
4. **Name:** decided on 2026-10-12: requimd, spoken "requi-m-d"
   (ADR-0006).
5. **Interpretations in M1 to confirm** (implemented, not yet in the spec):
   - CLI columns count bytes (as in compilers and Vim's quickfix list), so
     umlauts before the position shift the column.
   - A link value that is no valid ID (`impl: foo`, `FR-12`, `FR-?`) is a
     `dangling-ref` error.
   - A heading like `## FR-12 Title` (too few digits, BR-001) is silently
     not an item. A separate diagnostic may be worth adding.
   - A required key counts as present if `[defaults]` sets it (FR-105).
   - Steps of a duplicated use case are not reported as uncovered (FR-100:
     the duplicate is the only finding).
   - Unknown check IDs and unknown keys in `requimd.toml` are configuration
     errors (exit code 2).
   - Per-entry merging means default item types cannot be removed, only
     replaced.
6. **Interpretations in M2 to confirm:**
   - Until M4, "Assign ID" may hand out again the ID of a deleted item if
     it was the highest of its prefix (BR-002, accepted for M2).
   - Find references on an item also lists references to its steps.
   - Language features work only in included files (FR-201), also hover
     and go to definition, not only diagnostics and completion.
7. **Interpretations in M3 to confirm:**
   - The glossary file always belongs to the project, also when the
     include patterns do not match it. If `[project] glossary` names a
     missing file, `requimd check` stops with a configuration error (exit
     code 2); the language server then works without a glossary.
   - FR-113: the first paragraph must start with "As a" or "As an" and
     contain "I want" followed by "so that", ignoring case and line
     breaks; the comma is not required.
   - FR-111: "shall" counts anywhere in the text except code, also in
     quotes.
   - `requimd show`: exit code 1 if the ID does not exist or is declared more
     than once, 2 if the argument is no ID.
   - Weak words are checked in types `functional` and `quality`, modal
     verbs and "no shall" in `functional`, the story pattern in `story`:
     fixed type names, like `usecase` for steps.
8. **Interpretations in FR-216 to confirm:**
   - Glossary terms are matched like weak words (FR-110): not in code,
     quoted text or IDs. The glossary file itself gets no tokens.
   - Where tokens overlap, keys, IDs and step labels win over findings,
     findings over glossary terms (an alias "shopping cart" hides the term
     "cart" inside it).
   - Unknown metadata keys are not highlighted as keys.
9. **Interpretations in FR-403 to confirm:**
   - A reference without `requimd.toml` in the project directory has no
     items, so everything is "added" (e.g. a tag from before the project).
   - Output is sorted by prefix and number, as in the site index (FR before
     UC); a summary line ends the text output ("no changes" if none).
   - Exit code 0 also when there are changes (unlike `check`).
   - A use case's text is compared without its steps; steps are compared
     one by one by label. Moving a step in the list is no change.
10. **Interpretations in FR-404 to confirm:**
    - Neighbours follow links and mentions in both directions; the arrow
      label is the link name from `requimd.toml` (e.g. "implements").
    - A use case with shown steps is an untitled box holding the use case
      node and only the steps that shown items point to.
    - Status colours: draft dashed border, review yellow, approved green,
      deprecated grey; other configured values are not styled.
    - Step text is cut at 50 characters.
11. **Verification methods (idea "unverified requirements",
    `docs/roadmap.md`):** besides tests, which kinds of verification
    record should count (review, demonstration, analysis), and should a
    requirement state its expected method (e.g. `verify: test`)?
12. **Scope: design between requirements and tests?** Decides the test
    lifecycle and test suite integration (`docs/roadmap.md`). Options:
    (1) requirements only, tests verify requirements; (2) plus decisions
    (ADRs) as items linked to requirements, tests still verify
    requirements; (3) full V-model with design items and tests per level.
    Proposal: option 2. Once decided: a new ADR and an update of the scope
    in `requirements/vision.md`.
13. **ADR-0008 Editor integrations:** proposed; accept it in the review of
    the spec PR.

### Technical

1. Fixed meanings in code: item type `usecase` has steps, link key `impl`
   covers steps, `ref` is checked for cycles. Fine for now; configurable
   later if needed.

## Next steps

1. Owner: try M3 and M4 in Neovim (`docs/neovim.md`) and review the open
   interpretations above (business questions 5 to 12).
2. Alpha release `v0.1.0-alpha.2` (FR-216, FR-403, FR-404 since
   alpha.1): check the release job and the four archives, then review and
   test the tool on a real project. Further rounds: bump to `-alpha.3`
   etc. in `Cargo.toml` and `editors/vscode/package.json` (a `release/`
   PR); `v0.1.0` when the reviews are done. Test builds without
   publishing, including the four `.vsix` packages: Actions → Release →
   Run workflow.
3. Later: publish the site on GitHub Pages; FR-112 (EARS) and the other
   ideas in `docs/roadmap.md`.

## Related documents outside the repo

- Claude Doc "reqmd Project Status" (2026-10-09) – earlier snapshot of this
  file, under the former name.
- Claude Doc "reqmd Parser and LSP Library Comparison" (2026-10-10) – full
  comparison with sources; summarised in ADR-0003.
