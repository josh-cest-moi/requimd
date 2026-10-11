# Command line interface

## FR-300 Check command
impl: UC-003.s1, UC-003.s3, UC-003.s4, UC-003.s2a  prio: must

The system shall provide `requimd check [paths]`, which checks the project and
prints one line per diagnostic in the form
`file:line:column: severity: message [check-id]`. It ends with exit code 0
if no errors were found, 1 if errors were found, and 2 on a configuration or
usage error.

The project root is searched from the first path, or from the current
directory if no path is given (FR-013). The whole project is always loaded,
so references into other files resolve; the paths only select the files
whose diagnostics are printed. Paths that belong to different projects are
a usage error.

## FR-301 Machine-readable output
impl: UC-003.s3  prio: must

When called with `--format json`, `requimd check` shall print the diagnostics
as a JSON array instead.

## FR-302 Strict mode
impl: UC-003.s4  prio: must

When called with `--strict`, `requimd check` shall treat warnings as errors
for the exit code.

## FR-303 Init command
impl: UC-007.s1  prio: must

The system shall provide `requimd init`, which creates a `requimd.toml` with the
default configuration and comments, and an empty glossary file. It shall
not overwrite existing files.

## FR-304 Format command
impl: UC-001.s4  prio: should

The system shall provide `requimd fmt`, which replaces placeholder IDs by the
next free IDs (as in FR-212) and normalises the separators in metadata
blocks. With `--check` it changes nothing and exits with 1 if changes would
be made.

## FR-305 Show command
impl: UC-002.s5  prio: should

The system shall provide `requimd show <ID>`, which prints the item and its
incoming and outgoing references.

## FR-306 Coverage command
impl: UC-003.s3  prio: should

The system shall provide `requimd coverage`, which prints for each use case
the number of steps, the number of covered steps and the uncovered steps.

## FR-307 Review command
impl: UC-004.s4, UC-004.s5  prio: should

The system shall provide `requimd review <ID>`, which marks all suspect links
from or to the item as reviewed (FR-116).

## FR-308 LSP command
impl: UC-007.s3  prio: must

The system shall provide `requimd lsp`, which starts the language server
(FR-200).
