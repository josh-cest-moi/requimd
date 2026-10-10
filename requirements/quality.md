# Quality requirements

Quality requirements use a light form of Planguage: `scale` is what is
measured, `meter` how it is measured, `must` the minimum acceptable value
and `plan` the target.

## NFR-001 Diagnostics latency
ref: FR-202  prio: must
scale: ms from a document change to published diagnostics for that document, 95th percentile
meter: benchmark on a generated project with 2,000 items in 200 files
must: 300
plan: 100

Diagnostics shall follow typing without noticeable delay. Debounce time
configured in the editor is not included.

## NFR-002 Full check duration
ref: FR-300  prio: must
scale: seconds for `reqmd check` on a whole project, wall clock
meter: benchmark on the generated project of NFR-001, standard laptop
must: 5
plan: 1

## NFR-003 Server start
ref: FR-200  prio: must
scale: seconds from server start until the item graph is complete
meter: benchmark on the generated project of NFR-001
must: 2
plan: 0.5

## NFR-004 Platforms
prio: must
scale: operating systems on which the CI test suite passes
meter: CI matrix
must: Linux, macOS
plan: Linux, macOS, Windows

## NFR-005 Installation
ref: UC-007  prio: must
scale: number of files a user must install
meter: follow the installation guide on a clean machine
must: 1

The tool ships as a single executable without a runtime to install.

## NFR-006 Offline operation
prio: must
scale: network connections opened by check, fmt and lsp
meter: run the test suite with network access blocked
must: 0

## NFR-007 Editor neutrality
impl: UC-007.s2  prio: must
scale: share of `must` language server features usable in Neovim 0.12 with its built-in client and no plugin
meter: manual test per feature, documented in `docs/neovim.md`
must: 100 %

Only standard LSP features are used; nothing requires an editor plugin.

## NFR-008 Plain Markdown readability
prio: must
scale: items in `examples/shop` whose rendering on GitHub or GitLab shows raw markup other than the metadata block
meter: manual review of the rendered files
must: 0

## NFR-009 Robustness
ref: FR-012  prio: must
scale: crashes or hangs of the language server during a fuzz run with 10,000 random edits of the example projects
meter: fuzz test in CI
must: 0

## NFR-010 Test traceability
prio: must
scale: share of `must` requirements of a completed milestone with at least one test whose name contains the requirement ID
meter: script that matches test names against requirement IDs
must: 100 %
