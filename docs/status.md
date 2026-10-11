# Project status

As of 2026-10-11. Update this file at the end of every working session.

## State

- **M0 Specification: done.** Spec in `requirements/` (7 use cases, 5
  business rules, 69 functional and 10 quality requirements, all `draft`),
  fixtures in `examples/`, decisions in `docs/decisions/`.
- **M1 Parser and check command: implemented, CI not yet run.** Cargo
  workspace with `crates/reqmd-core` (config, parser, item model),
  `crates/reqmd-checks` and `crates/reqmd-cli` (binary `reqmd`). All `must`
  items of M1 have tests named after their IDs (80 tests, NFR-010 checked by
  a test). `reqmd check` passes on this repository and `examples/shop`;
  `examples/broken` yields exactly the M1 rows of `EXPECTED.md`. Open for
  M1: NFR-004 needs a green CI run on GitHub (repository not pushed yet).
- **M2 Language server core: next.**

## Decision log

| Date | Decision | Where recorded |
|------|----------|----------------|
| 2026-10-11 | Workspace: `reqmd-core`, `reqmd-checks`, `reqmd-cli`; `reqmd-lsp` follows in M2 | this file |
| 2026-10-11 | Built-in defaults = types, links, fields and values of the root `reqmd.toml` without `check`/`sev` | FR-014 |
| 2026-10-11 | Project config merged with defaults per entry (`[types.x]`, `[links.x]`, value set) | FR-014 |
| 2026-10-11 | `reqmd check [paths]`: whole project loaded, paths only filter output; paths of different projects are a usage error | FR-300 |
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
2. **Default weak-word list:** confirm the words and write a reason for
   each (FR-110 names the minimum set).
3. **Publishing audience:** who reads the requirements besides analysts and
   developers? Decides whether FR-405 (Confluence) moves up.
4. **Name:** keep "reqmd" or choose a final name before the repo is public.
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
   - Unknown check IDs and unknown keys in `reqmd.toml` are configuration
     errors (exit code 2).
   - Per-entry merging means default item types cannot be removed, only
     replaced.

### Technical

1. **Lock file format** (JSON or TOML, merge conflict behaviour): own ADR
   before M4.
2. Fixed meanings in code: item type `usecase` has steps, link key `impl`
   covers steps, `ref` is checked for cycles. Fine for now; configurable
   later if needed.

## Next steps

1. Push this repository to GitHub and check the CI run (NFR-004).
2. Owner: review the spec changes and interpretations above.
3. Start M2: `reqmd-lsp` crate with lsp-server, FR-200 ff.

## Related documents outside the repo

- Claude Doc "reqmd Project Status" (2026-10-09) – earlier snapshot of this
  file.
- Claude Doc "reqmd Parser and LSP Library Comparison" (2026-10-10) – full
  comparison with sources; summarised in ADR-0003.
