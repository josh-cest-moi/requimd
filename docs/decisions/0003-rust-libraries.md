# ADR-0003 Markdown parser and LSP library

Status: **accepted** on 2026-10-10 by the product owner: pulldown-cmark as
Markdown parser, lsp-server with lsp-types as LSP library.

Full comparison with sources: Claude Doc "reqmd Parser and LSP Library
Comparison" (2026-10-10). Versions as found on 2026-10-09; check again when
adding the dependencies.

## Markdown parser

| Criterion | pulldown-cmark | comrak | markdown-rs |
|-----------|----------------|--------|-------------|
| Output | Event stream, no tree | Full syntax tree | mdast syntax tree |
| Positions | Byte range per event (`into_offset_iter`) | Line and column per node (optional) | Line, column and offset per node |
| Spec | CommonMark | CommonMark and GFM | CommonMark plus extensions |
| Safety | Pure Rust, no unsafe except opt-in SIMD | Port of GitHub's C parser, inherits its bugs | Older README warned hostile input can crash it |
| Maturity | 0.13.x, used by mdBook | 0.5x, frequent releases | 1.0.0 after long alpha |
| Code style | Idiomatic | Heavy `RefCell`, unidiomatic | Idiomatic |

Proposal: **pulldown-cmark**. reqmd builds its own item graph anyway, so the
missing tree costs little; byte offsets map directly to LSP ranges.

## LSP library

| Criterion | lsp-server | tower-lsp-server |
|-----------|------------|------------------|
| Origin | rust-analyzer team | Community fork of the unmaintained tower-lsp |
| Model | Synchronous, crossbeam channels, own dispatch loop | Async on tokio, `LanguageServer` trait |
| Boilerplate | More | Less |
| Control | Full control of threading, debouncing, cancellation | Framework-driven; async is harder to debug |
| Version | 0.10.x | 0.23.x, pre-1.0, recently moved to `ls-types` |
| Used by | rust-analyzer | Oxc, Biome |

Proposal: **lsp-server** with `lsp-types`. The work is CPU-bound parsing
and checking, so async adds complexity without benefit; a synchronous loop
makes debouncing for NFR-001 easier to control.

## Further crates (not contentious)

- `toml_edit` for writing `reqmd.toml` without losing comments (FR-219);
  `toml` / `serde` for reading.
- `clap` for the CLI, `serde_json` for JSON output (FR-301).
- `globset` or `ignore` for include patterns (FR-013).

## Consequences

- The parser consumes pulldown-cmark's event stream with byte ranges and
  builds reqmd's own item model; there is no intermediate Markdown tree.
  Positions stay byte offsets until the LSP boundary.
- pulldown-cmark knows nothing about items, metadata blocks or mentions.
  reqmd recognises them itself on the text of headings and paragraphs,
  skipping code (FR-009).
- The language server (M2) runs its own synchronous loop on lsp-server's
  channels; debouncing and cancellation for NFR-001 are reqmd code.
- No tokio or other async runtime in the dependency tree (NFR-005).
- Versions are checked again when each dependency is added to `Cargo.toml`.
