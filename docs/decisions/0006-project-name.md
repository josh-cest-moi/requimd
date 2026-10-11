# ADR-0006 Project name

Status: **accepted** on 2026-10-12 by the product owner.

## Context

The working title "reqmd" is already used by several other projects. The
name had to be settled before the repository goes public (open business
question 4 in `docs/status.md`).

## Decision

The project is called **requimd**, spoken "requi-m-d": requirements in
Markdown.

## Considered

- **reqmd**: taken by other projects.
- **requiescat**: a play on "requirements as code", but it is the first
  word of "requiescat in pace" (rest in peace), which most readers would
  think of first.

## Consequences

- One name everywhere, nothing kept from the old one, since there are no
  users yet: binary `requimd`, configuration `requimd.toml`, lock file
  `requimd.lock` (M4), inline marker `<!-- requimd: allow … -->`, LSP
  commands `requimd.*`, crates `requimd-core`, `requimd-checks`,
  `requimd-lsp`, `requimd-cli`.
- The Neovim setup in `docs/neovim.md` uses the new client name and command.
- Search engines may read the name as a typo of "requiem" or
  "requisition"; the README and repository description spell it out.
- Documents outside the repository (Claude Docs) keep their old titles.
