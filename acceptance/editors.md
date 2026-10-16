# Acceptance tests: editor integrations

Tests for FR-500 to FR-504 and the VS Code check of the language server
features (NFR-011). The language server is tested in detail in Neovim
(`lsp.md`); in VS Code these tests make sure the features work.

Decisions for the whole area:

- ADR-0008 Editor integrations (**proposed**, to be accepted by the
  product owner): analysts use VS Code more often than Neovim, and VS Code
  has no generic LSP client. A TypeScript extension in `editors/vscode/`
  on `vscode-languageclient` starts `requimd lsp` for Markdown files and
  adds the reference view for the code lens and the theme mapping of the
  token modifiers. The release builds one package per platform with the
  matching executable inside; `requimd.path` overrides it. The packages go
  to GitHub Releases; VS Code Marketplace and Open VSX come later and need
  publisher accounts. IntelliJ IDEA gets a setup guide for the LSP4IJ
  plugin, a dedicated plugin only on demand. Considered: the executable on
  `PATH` only (two installations, PATH editing for analysts) and a
  download on first start (fails behind proxies, against NFR-006, no
  checksums in the releases yet, versions may drift).
- Log 2026-10-13: VS Code `must`, IntelliJ guide `should`; new ID range
  FR 500–599.
- Log 2026-10-14: no store publishing planned; the extension carries the
  Cargo version including the alpha suffix, and the release fails if they
  differ; one language client per project root, server commands
  registered once by the extension; finding modifiers mapped to underline
  and italic, not colours, since default themes have no warning colour;
  publisher name `josh-cest-moi` until a Marketplace account exists.
- Log 2026-10-14: the VS Code tests run against a pinned VS Code 1.91.1,
  the oldest supported version, in one Linux job; a small PASS/FAIL runner
  instead of a test framework.

## VS Code extension (FR-500)

- `docs/vscode.md` states that `requimd.path` is read from user settings
  only, not from workspace settings. The spec does not say so.

### TC-500 The extension starts the server for the project's Markdown files
ver: FR-500

Given the `.vsix` for your platform installed as in `docs/vscode.md`; a
folder with two copies of the base project (`p1`, `p2`) and a Markdown
file `loose.md` outside both, opened as workspace in VS Code.

When `p1/functional.md`, `p2/functional.md` and `loose.md` are opened, and
in `p1` the link `impl: UC-901.s1` is changed to `UC-901.s9`.

Then the Problems panel shows the `dangling-ref` of `p1` only; `p2` is
clean; `loose.md` gets no requimd features. The output panel shows one
server per project.

### TC-501 The code lens opens the reference view
ver: FR-500, FR-215

Given `usecases.md` of the base project open in VS Code.

When the lens above step `s2` is clicked.

Then the reference view lists `FR-902`.

## Bundled language server (FR-501)

### TC-502 No other installation is needed
ver: FR-501

Given a computer or user account without `requimd` on `PATH`, and the
`.vsix` installed.

When the base project is opened in VS Code.

Then the server runs (diagnostics appear). The executable inside the
installed extension (below `.vscode/extensions/` in the user folder)
prints with `--version` the version of the extension.

### TC-503 `requimd.path` starts another executable
ver: FR-501

Given a second `requimd` executable, for example a debug build
`target/debug/requimd.exe`.

When `requimd.path` is set to it in the user settings and VS Code is
reloaded.

Then the output panel shows that this executable runs.

## Extension packages in the release (FR-502)

### TC-504 One package per platform on the release page
ver: FR-502

Given the latest pre-release on the GitHub release page.

When its assets are listed.

Then there is one `.vsix` per platform of the binaries (linux-x64,
darwin-arm64, darwin-x64, win32-x64), named with the release version, and
`docs/vscode.md` describes the installation from such a package.

## Highlighting in VS Code (FR-503)

### TC-505 Findings are visible in every theme without configuration
ver: FR-503, FR-216

Given `FR-901` with the text "The cart should open fast, see the basket."
and no `editor.semanticTokenColorCustomizations` in the settings.

When the themes Light+, Dark+ and a High Contrast theme are chosen in
turn.

Then in each "fast" and "should" are underlined and "basket" is italic;
item IDs, keys and glossary terms have theme colours.

## VS Code feature check (NFR-011)

- NFR-011 is met by an automated test in CI (TC-610 in `quality.md`).
  This test is the manual counterpart.

### TC-506 The language server features work in VS Code
ver: NFR-011, FR-500

Given the base project open in VS Code with the extension and no other
configuration.

When each feature of the table in `docs/vscode.md` is tried once:
diagnostics, completion of references, keys and values, hover (also on a
glossary term), go to definition, find references, go to implementation,
inlay hints, outline and symbol search, assign ID, create stub, rename,
the language check actions, mark as reviewed, code lens, highlighting,
and a file change outside VS Code.

Then each works as its Neovim test in `lsp.md` describes, with the VS Code
keys of `docs/vscode.md`.

## IntelliJ setup guide (FR-504)

Open: not written yet (M6).

### TC-507 The IntelliJ guide sets up the language server
ver: FR-504

Given IntelliJ IDEA with the LSP4IJ plugin and the guide of FR-504.

When the guide is followed and each feature of the Neovim table in
`docs/neovim.md` is tried.

Then the server starts for the base project and every feature the guide
lists as verified works.
