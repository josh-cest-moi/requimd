# Project status

As of 2026-10-10. Update this file at the end of every working session.

## State

- **M0 Specification: done.** Spec in `requirements/` (7 use cases, 5
  business rules, 69 functional and 10 quality requirements, all `draft`),
  fixtures in `examples/`, decisions in `docs/decisions/`.
- **M1 Parser and check command: next.** No code exists yet.

## Decision log

| Date | Decision | Where recorded |
|------|----------|----------------|
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
   Only the owner sets `approved` (BR-004).
2. **Default weak-word list:** confirm the words and write a reason for
   each (FR-110 names the minimum set).
3. **Publishing audience:** who reads the requirements besides analysts and
   developers? Decides whether FR-405 (Confluence) moves up.
4. **Name:** keep "reqmd" or choose a final name before the repo is public.

### Technical

1. **ADR-0003, Markdown parser and LSP library:** proposed pulldown-cmark
   and lsp-server; the owner is still comparing. **Blocks the parser in
   M1**; the LSP part is needed from M2.
2. **Workspace layout:** propose crate split (e.g. `core`, `checks`, `cli`,
   `lsp`) at the start of M1.
3. **Lock file format** (JSON or TOML, merge conflict behaviour): own ADR
   before M4.

## Next steps

1. Push this repository to GitHub.
2. Confirm ADR-0003 with the owner.
3. Start M1: workspace, CI, parser (FR-001 to FR-014), then checks and CLI.

## Related documents outside the repo

- Claude Doc "reqmd Project Status" (2026-10-09) – earlier snapshot of this
  file.
- Claude Doc "reqmd Parser and LSP Library Comparison" (2026-10-10) – full
  comparison with sources; summarised in ADR-0003.
