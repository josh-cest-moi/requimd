# ADR-0008 Editor integrations

Status: **proposed** on 2026-10-13.

## Context

Analysts, the primary users, more often use VS Code than Neovim. VS Code
has no generic LSP client, so it needs an extension to use the language
server. IntelliJ IDEA has LSP4IJ, a free plugin by Red Hat that can start
any language server. The language server itself needs no change for either
editor (NFR-007).

## Decision

- **VS Code:** an extension in TypeScript under `editors/vscode/`, built on
  the `vscode-languageclient` library. It starts `requimd lsp` over stdio
  for Markdown files and adds what the client side needs: the reference
  view for the code lens (FR-500) and the semantic token modifiers with
  their theme mapping (FR-503).
- **Binary bundled:** the release job builds one platform-specific package
  per release binary (`linux-x64` from the static musl build,
  `darwin-arm64`, `darwin-x64`, `win32-x64`) and puts the matching
  executable into it (FR-501). Extension and executable always have the
  same version. `requimd.path` overrides the bundled executable.
- **Distribution:** the `.vsix` packages are attached to the GitHub release
  (FR-502). Publishing to the VS Code Marketplace and Open VSX comes later
  and needs publisher accounts of the owner.
- **IntelliJ:** a setup guide for LSP4IJ (FR-504). A dedicated plugin only
  on demand.

## Considered

- **Binary on PATH only:** a smaller extension, but two installations
  (NFR-005), and analysts would have to edit PATH.
- **Download on first start** from GitHub Releases: a small extension, but
  it fails behind corporate proxies and firewalls, works against the
  offline principle (NFR-006), needs checksums the releases do not publish
  yet, and lets extension and executable versions drift apart.
- **Dedicated IntelliJ plugin** (on LSP4IJ or on JetBrains' own LSP API):
  no setup for the user, but a Kotlin and Gradle project and a second
  marketplace. Not worth it before there is demand.

## Consequences

- The repository gets a Node and TypeScript toolchain beside the cargo
  workspace. It does not affect the single executable (NFR-005); the
  extension's npm dependencies are limited to the language client and the
  build and test tools.
- CI runs the extension tests in a real VS Code (NFR-011), which downloads
  VS Code at test time; the tool itself stays offline (NFR-006).
- VS Code users install one file, from the release, with "Install from
  VSIX". Updates are manual until the extension is in a marketplace.
