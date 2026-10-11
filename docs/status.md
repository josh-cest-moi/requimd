# Project status

As of 2026-10-12. Update this file at the end of every working session.

## State

- **M0 Specification: done.** Spec in `requirements/` (7 use cases, 5
  business rules, 69 functional and 10 quality requirements, all `draft`),
  fixtures in `examples/`, decisions in `docs/decisions/`.
- **CI:** first runs on GitHub (2026-10-11) green on Linux and macOS, so
  NFR-004 `must` is met and M1 to M3 are done. Windows failed in one step:
  FR-217 with Neovim's file watcher, because Neovim on Windows reports
  garbled paths for projects below an 8.3 short path (the runner's
  `C:\Users\RUNNER~1`). Fixed in the server on 2026-10-12 (rescan on events
  that do not fit the disk); waiting for a green Windows run, after which
  Windows can become a required job (NFR-004 `plan`).
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
- **M4 Impact analysis: next** (lock file ADR first).

## Decision log

| Date | Decision | Where recorded |
|------|----------|----------------|
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
3. **Publishing audience:** who reads the requirements besides analysts and
   developers? Decides whether FR-405 (Confluence) moves up.
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

### Technical

1. **Lock file format** (JSON or TOML, merge conflict behaviour): own ADR
   before M4.
2. Fixed meanings in code: item type `usecase` has steps, link key `impl`
   covers steps, `ref` is checked for cycles. Fine for now; configurable
   later if needed.

## Next steps

1. Push the Windows fix and check that the Windows job is green; then
   remove `continue-on-error` for Windows in `.github/workflows/ci.yml`.
2. Owner: try the M3 features in Neovim (`docs/neovim.md`, including the
   optional code lens handler) and review the interpretations above.
3. ADR for the lock file format (technical question 1), then M4: suspect
   links (FR-116), mark reviewed (FR-218), `requimd review` (FR-307), and
   the lock file in FR-212 (BR-002).

## Related documents outside the repo

- Claude Doc "reqmd Project Status" (2026-10-09) – earlier snapshot of this
  file, under the former name.
- Claude Doc "reqmd Parser and LSP Library Comparison" (2026-10-10) – full
  comparison with sources; summarised in ADR-0003.
