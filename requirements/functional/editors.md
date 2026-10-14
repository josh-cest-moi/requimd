# Editor integrations

The language server (FR-200 to FR-299) works in any editor with an LSP
client (NFR-007). The items here make it available in editors that need an
extension or a setup guide for that.

## FR-500 VS Code extension
impl: UC-007.s2a  prio: must

The system shall provide a VS Code extension that starts the language
server of FR-200 for the Markdown files of a requimd project. Executing the
code lens of FR-215 shall list the implementing items in VS Code's
reference view.

## FR-501 Bundled language server
ref: FR-500  prio: must

The VS Code extension shall contain the `requimd` executable of its own
version for its platform, so that no other installation is needed
(NFR-005). The setting `requimd.path` shall make it start another
executable instead.

## FR-502 Extension packages in the release
ref: FR-501  prio: must

The release shall attach one VS Code extension package (`.vsix`) per
platform of the release binaries to the GitHub release. The installation
guide shall describe how to install such a package in VS Code.

## FR-503 Highlighting in VS Code
ref: FR-216  prio: should

The VS Code extension shall declare the semantic token modifiers of FR-216
and map them to theme styles, so that weak words, modal verbs and glossary
aliases are highlighted without user configuration in every theme.

## FR-504 IntelliJ setup guide
impl: UC-007.s2b  prio: should

The system shall provide a guide that sets up the language server in
IntelliJ IDEA with the LSP4IJ plugin, verified feature by feature like the
Neovim guide.
