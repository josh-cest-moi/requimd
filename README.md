# requimd

Requirements as code: requirements live as Markdown files in a git repository.
A CLI checks them, and a language server helps you write them in any LSP
editor (primary target: Neovim 0.12).

> Spoken "requi-m-d" (ADR-0006). Status: milestones M1 to M4 done in Rust
> (check command, language server, authoring comfort, impact analysis); M5
> (publishing) is next. See `docs/status.md`.

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

## Installation

Download the archive for your system from the
[releases](https://github.com/josh-cest-moi/requimd/releases), unpack it and
put the single file `requimd` (`requimd.exe` on Windows) on your `PATH`.
Nothing else is needed. Releases with a suffix such as `-alpha.1` are
pre-releases for testing: behaviour and file formats may still change. Or
build from source:

```bash
cargo build --release
```

## Usage

The binary `requimd` provides:

| Command | Purpose |
|---------|---------|
| `requimd init` | Create `requimd.toml` and an empty glossary |
| `requimd check [paths] [--format json] [--strict] [--locked]` | Check the project and update `requimd.lock`; exit code 1 on errors |
| `requimd fmt [--check]` | Assign IDs to placeholders, normalise metadata |
| `requimd show <ID>` | Show an item or step with its references |
| `requimd coverage` | Coverage of use case steps |
| `requimd review <ID> \| --all` | Mark suspect links as reviewed |
| `requimd diff <ref1> [ref2] [--format json]` | Items added, removed or changed between two git references, or a reference and the working tree (needs `git`) |
| `requimd export site <dir> [--status …]` | Static website without JavaScript, readable from the file system |
| `requimd export matrix [-o file]` | Traceability matrix as CSV |
| `requimd export json [-o file]` | The item graph as JSON |
| `requimd export mermaid <ID> [--depth n] [-o file]` | An item and its neighbours as a Mermaid diagram in a Markdown code block |
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
| `requimd.toml`, `requimd.lock` | Make this repository itself a requimd project; the lock file records reviewed links (ADR-0007) |
| `CLAUDE.md` | Context and working rules for Claude Code sessions |

## Where to start

Read `docs/status.md`, then `requirements/README.md` and
`requirements/vision.md`.

## License

Licensed under the [MIT License](LICENSE).
