# requimd

Requirements as code: requirements live as Markdown files in a git repository.
A CLI checks them, and a language server helps you write them in any LSP
editor (primary target: Neovim 0.12).

> Spoken "requi-m-d" (ADR-0006). Status: milestones M1 to M3 implemented in Rust (check
> command, language server, authoring comfort); M4 (impact analysis) is
> next. See `docs/status.md`.

## Why

Requirements in wikis have no branches, no reviews by pull request, no
baselines and no reliable links. requimd keeps the format close to plain
Markdown, so files stay readable anywhere, and adds just enough convention
for a machine to understand items, links and use case steps.

A taste of the format:

```markdown
## FR-104 VAT per line item
impl: UC-012.s2  prio: must

When the cart is opened, the system shall calculate VAT per line item
based on the delivery country (see BR-003).
```

## Usage

```bash
cargo build --release
```

The binary `target/release/requimd` provides:

| Command | Purpose |
|---------|---------|
| `requimd init` | Create `requimd.toml` and an empty glossary |
| `requimd check [paths] [--format json] [--strict]` | Check the project; exit code 1 on errors |
| `requimd fmt [--check]` | Assign IDs to placeholders, normalise metadata |
| `requimd show <ID>` | Show an item or step with its references |
| `requimd coverage` | Coverage of use case steps |
| `requimd lsp` | Language server on stdio |

Editor setup: `docs/neovim.md`.

## Repository layout

| Path | Content |
|------|---------|
| `crates/` | Rust workspace: `requimd-core` (config, parser, model), `requimd-checks`, `requimd-lsp`, `requimd-cli` (binary `requimd`) |
| `scripts/nvim-check.lua` | Checks the language server in Neovim without plugins |
| `requirements/` | Specification of requimd, written in the requimd format (dogfooding) |
| `docs/status.md` | Current state, decision log, open questions |
| `docs/roadmap.md` | Milestones and their scope |
| `docs/decisions/` | Architecture decision records |
| `docs/neovim.md` | Editor setup |
| `examples/shop/` | Small valid sample project, used as test fixture |
| `examples/broken/` | Sample with intentional errors and expected diagnostics |
| `requimd.toml` | Makes this repository itself a requimd project |
| `CLAUDE.md` | Context and working rules for Claude Code sessions |

## Where to start

Read `docs/status.md`, then `requirements/README.md` and
`requirements/vision.md`.
