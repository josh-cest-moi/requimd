# VS Code setup

Target: VS Code 1.91 or later with the requimd extension (FR-500). The
extension starts the language server for the Markdown files of every
requimd project in the workspace; nothing else needs configuring.

## Installation

The extension contains the `requimd` executable for its platform
(FR-501). From the first release that includes it (FR-502):

1. On the [release page](https://github.com/josh-cest-moi/requimd/releases),
   download the package for your platform, for example
   `requimd-vscode-win32-x64-<version>.vsix`.
2. In VS Code: Extensions view → `…` menu → **Install from VSIX…**, or

   ```bash
   code --install-extension requimd-vscode-win32-x64-<version>.vsix
   ```

Updates are manual: install the package of the new release the same way.

## Settings

| Setting | Default | Meaning |
|---------|---------|---------|
| `requimd.path` | empty | Path of another `requimd` executable. Empty: the bundled one, else `requimd` from `PATH`. User settings only, not workspace settings. |
| `requimd.trace.server` | `off` | `messages` or `verbose` logs the LSP traffic in the output panel **requimd**. |

## Projects

A Markdown file belongs to the project of the nearest `requimd.toml` above
it. The extension starts one language server per project when the first
file of that project is opened, so a workspace may hold several projects
(this repository holds the specification and the example projects).
Markdown files outside any project get no requimd features.

## Features

Verified with VS Code 1.91.1 by the automated test in
`editors/vscode/src/test/` (NFR-011), which CI runs on every change. FR-213
and FR-218 are not tested separately; they reach the server the same way as
FR-212 and FR-219.

| Feature | Requirement | VS Code |
|---------|-------------|---------|
| Diagnostics | FR-202 | Problems panel, squiggles, `F8` |
| Completion | FR-203, FR-204 | automatic after `-`, `.`, and in link values after space or comma; anywhere with `Ctrl+Space` |
| Hover | FR-205 | mouse over an ID or a glossary term |
| Go to definition | FR-206 | `F12`, `Ctrl`+click |
| Find references | FR-207 | `Shift+F12` |
| Go to implementation | FR-208 | `Ctrl+F12` |
| Inlay hints | FR-209 | shown after references (`editor.inlayHints.enabled`) |
| Document symbols | FR-210 | Outline view, `Ctrl+Shift+O` |
| Workspace symbols | FR-211 | `Ctrl+T` |
| Assign ID, create stub, language check actions, mark as reviewed | FR-212, FR-213, FR-218, FR-219 | light bulb, `Ctrl+.` |
| Rename | FR-214 | `F2` on an ID |
| Code lens on steps | FR-215 | click the lens: the implementing items open in the reference view |
| Highlighting | FR-216, FR-503 | see below |
| External file changes | FR-217 | automatic |

VS Code's built-in Markdown support stays active beside requimd, so the
Outline view and symbol searches may list headings from both.

## Highlighting

The extension maps the requimd tokens (FR-216) to styles that every theme
shows: weak words and modal verbs are underlined, glossary aliases are
italic. Item IDs, references, keys and glossary terms use the theme's
colours for classes, enum members, properties and namespaces.

Own colours go into the user settings, for example:

```json
"editor.semanticTokenColorCustomizations": {
  "rules": {
    "*.weakWord:markdown": { "foreground": "#cd9731", "underline": true },
    "*.modalVerb:markdown": { "foreground": "#cd9731" },
    "namespace:markdown": { "italic": true }
  }
}
```

## Development

The extension lives in `editors/vscode/` (TypeScript, ADR-0008).

```bash
cargo build
cd editors/vscode
npm ci
npm test
```

`npm test` compiles the extension and runs the tests in VS Code 1.91.1,
downloaded once into `editors/vscode/.vscode-test/`, against
`target/debug/requimd` (or `REQUIMD_TEST_BIN`). On Linux without a display,
use `xvfb-run -a npm test`.

To test a package instead, with its bundled executable (FR-501), as CI does:

```bash
mkdir bin && cp ../../target/debug/requimd bin/
npx vsce package --target linux-x64 --out test.vsix
rm -r bin
REQUIMD_TEST_VSIX=test.vsix npm test
```

The release workflow builds one package per platform with the release
binary and attaches it to the GitHub release (FR-502). The version in
`editors/vscode/package.json` must match `Cargo.toml`; the release fails
otherwise.
