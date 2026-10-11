# ADR-0001 Implementation language

Status: **accepted** on 2026-10-09 by the product owner.

## Context

The tool consists of a parser, checks, a CLI and a language server that
share one item graph. Relevant requirements: single-file installation
(NFR-005), low latency (NFR-001, NFR-002), robustness against broken input
(NFR-009), three platforms (NFR-004). The Markdown parser must provide exact
source positions for diagnostics and LSP ranges.

## Options considered

| Option | Strengths | Weaknesses |
|--------|-----------|------------|
| C / C++ | Maximum performance, `cmark` reference parser | No memory safety against broken input, no standard build or package system, thin LSP libraries, slowest development |
| Rust | Memory safety without GC, single binary, cargo, best Markdown and LSP crates, enums model the domain well | Steep learning curve, slower compiles |
| Go | Simple and readable, fast compiles, trivial cross-compilation, goldmark | Less polished LSP libraries, no sum types, verbose error handling |
| Python / TypeScript | Fastest prototyping | Need a runtime, conflicts with NFR-005 |

## Decision

Rust. Robustness against half-typed input and the library situation weigh
most, and most code will be written with Claude Code. Go was the
alternative.

## Consequences

- One cargo workspace producing one binary `requimd` (NFR-005).
- Standard tooling: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`.
- Library choices are recorded in ADR-0003.
