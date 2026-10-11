# CLAUDE.md

## What this project is

requimd (spoken "requi-m-d", formerly reqmd, ADR-0006) is a requirements
management tool for business analysts. Requirements live as Markdown files in a git repository; a CLI
checks them and a language server supports writing them in any LSP editor,
primarily Neovim 0.12. The repository owner is a business analyst and acts
as product owner.

## Status

Specification done (M0); M1 (parser and `requimd check`), M2 (language
server, `requimd lsp`), M3 (glossary, language checks, authoring features)
and M4 (lock file and suspect links, ADR-0007) implemented in the cargo
workspace under `crates/`. Next is milestone M5 (publishing) in
`docs/roadmap.md`. **Read
`docs/status.md` first**: it holds the decision log, open questions and
next steps.

Implementation language is Rust (ADR-0001, accepted). Markdown parser
pulldown-cmark and LSP library lsp-server (ADR-0003, accepted 2026-10-10).

## Starting a milestone

1. Read `docs/status.md`, `requirements/README.md` and the items of the
   milestone in `docs/roadmap.md`.
2. Check the items against the code and the fixtures; ask the owner about
   gaps and ambiguities before implementing, then record the answers in the
   spec (separate commit) and in the decision log.
3. Implement, test, and verify language server features in real Neovim
   with `scripts/nvim-check.lua`.

## Source of truth

- `requirements/` is the specification, written in the requimd format itself.
  Read `requirements/README.md` first, then the items relevant to the task.
- The format is specified normatively in `requirements/functional/format.md`
  (FR-001 to FR-014) and `requirements/rules.md`. When in doubt about syntax,
  these files win over examples elsewhere.
- `docs/roadmap.md` defines milestones; `docs/decisions/` records decisions.

## Working rules

- Implement against requirement IDs. Put the ID in test names, in a comment
  at the implementing code (e.g. `// FR-006`), and in commit messages
  (`FR-006: parse use case steps`).
- If a requirement is ambiguous, incomplete, or contradicts another, stop and
  ask. Do not silently interpret. Propose changes to requirement files as a
  separate commit and list them in your summary.
- Never set `status: approved` on an item (BR-004). You may propose `review`.
- Never renumber item IDs or step labels (BR-002, BR-003). New items get the
  next free number in their range (see `requirements/README.md`).
- Work on `prio: must` items of the current milestone unless asked otherwise.
- Fenced code blocks and inline code in requirement files are examples, not
  items or references (FR-009).

## Tests

- `examples/shop/` is a valid sample project. `examples/broken/` contains
  intentional errors; the expected diagnostics are listed in
  `examples/broken/EXPECTED.md`. Use both as fixtures.
- Every `must` requirement of a milestone needs at least one test whose name
  contains its ID (NFR-010).
- As soon as `requimd check` works, run it on this repository. The
  specification must stay free of errors.

## Rust conventions

- Stable toolchain. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`
  and `cargo test` must pass before every commit.
- No `unwrap()` or `panic!` on user input; the parser must survive any
  input (FR-012, NFR-009). Errors become diagnostics.
- Keep positions as byte offsets internally; convert to LSP line/UTF-16
  columns only at the LSP boundary.

## Conventions

- Keep dependencies few; the tool ships as a single binary (NFR-005).
- At the end of a session, update `docs/status.md` (state, decision log,
  open questions).
- CLI diagnostic output follows FR-300.
- No network access at runtime (NFR-006).
