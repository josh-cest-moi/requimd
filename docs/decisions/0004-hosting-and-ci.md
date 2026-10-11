# ADR-0004 Hosting and CI

Status: **accepted** on 2026-10-09 by the product owner.

## Decision

The repository lives on GitHub; CI runs on GitHub Actions.

## Reasons

- Works directly with Claude Code on the web.
- Free runners for Linux, macOS and Windows cover NFR-004.

## Consequences

- M1 adds `.github/workflows/ci.yml`: `cargo fmt --check`, `cargo clippy`,
  `cargo test` on Linux and macOS (Windows as allowed failure until NFR-004
  `plan` is met), plus `requimd check` on this repository and `examples/shop`.
- Release binaries are built by Actions in M5.
