# Requirements specification

This directory is the specification of reqmd, written in the reqmd format.
It is both the spec and the first real test case for the tool.

## Reading order

1. `vision.md` – problem, goals, scope
2. `glossary.md` – terms used throughout
3. `usecases/` – what users do with the tool
4. `rules.md` – business rules (BR)
5. `functional/format.md` – the file format, normative
6. `functional/checks.md`, `lsp.md`, `cli.md`, `export.md` – behaviour
7. `quality.md` – quality requirements (NFR)

## ID ranges

| Prefix | Range | Area |
|--------|-------|------|
| UC | 001– | Use cases |
| BR | 001– | Business rules |
| FR | 001–099 | File format and parsing |
| FR | 100–199 | Checks |
| FR | 200–299 | Language server |
| FR | 300–399 | CLI |
| FR | 400–499 | Export and reporting |
| NFR | 001– | Quality requirements |

## Status and priority

Every item is `draft` unless it says otherwise (see `reqmd.toml`). Only the
product owner sets `approved` (BR-004).

Priority follows MoSCoW. The milestone an item belongs to is defined in
`docs/roadmap.md`, not in the item itself.
