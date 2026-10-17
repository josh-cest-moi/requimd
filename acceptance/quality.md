# Acceptance tests: quality requirements

Tests for NFR-001 to NFR-011. Each quality requirement names its own
measurement (`meter`); the tests follow it. Where the meter is an
automated benchmark or test, the test names the command that runs it.

Decisions for the whole area:

- ADR-0001 Implementation language (accepted): Rust, for memory safety
  against broken input without a garbage collector, a single executable,
  and the best Markdown and LSP libraries. Go was the alternative; C and
  C++ lack memory safety and a standard build system; Python and
  TypeScript need a runtime (against NFR-005).
- ADR-0004 Hosting and CI (accepted): GitHub and GitHub Actions; free
  runners for Linux, macOS and Windows cover NFR-004.
- Log 2026-10-12: performance tests enforce their limits in release builds;
  CI runs them with `--release`.

## Diagnostics latency (NFR-001)

- Code: `nfr_001_diagnostics_latency_on_2000_items` makes 20 edits that
  alternate a valid and an invalid value and takes the 95th percentile. It
  enforces only `must` (300 ms) in release builds and 1 second in debug
  builds; `plan` (100 ms) is not checked. `docs/status.md` reports 77 ms.
- Automated: `nfr_001_diagnostics_latency_on_2000_items` (lsp tests).

### TC-600 Diagnostics latency on a large project
ver: NFR-001

Given the generated project of the meter (2,000 items in 200 files).

When the latency benchmark runs
(`cargo test --release -p requimd-lsp --test server nfr_001 -- --nocapture`
prints the 95th percentile); and in Neovim one of its files is edited by
typing fast.

Then the 95th percentile is at most 300 ms (plan: 100 ms). In Neovim the
diagnostics follow the typing without noticeable delay.

## Full check duration (NFR-002)

- Code: `nfr_002_full_check_of_2000_items_in_200_files` measures loading and
  checking inside the test process, not the command, with the `must` bound
  of 5 seconds in every build.
- Automated: `nfr_002_full_check_of_2000_items_in_200_files` (checks tests).

### TC-601 `requimd check` on a large project
ver: NFR-002

Given the generated project of NFR-001 on a standard laptop.

When `Measure-Command { requimd check }` runs in PowerShell in its root.

Then at most 5 seconds (plan: 1 second).

Code: the generated project exists only inside the tests;
`cargo test --release -p requimd-checks --test checks nfr_002` measures
loading and checking in the test process. A manual measurement of the
command needs such a project, which requimd cannot create (Gap).

## Server start (NFR-003)

- Code: `nfr_003_server_start_until_item_graph_complete` measures from
  starting the server, including the initialize handshake, until a workspace
  symbol query answers; bound 2 seconds (`must`).
- Automated: `nfr_003_server_start_until_item_graph_complete` (lsp tests).

### TC-602 Time until the item graph is complete
ver: NFR-003

Given the generated project of NFR-001.

When the start benchmark runs
(`cargo test --release -p requimd-lsp --test server nfr_003`); and in
Neovim a file of the project is opened and `gri` is used on a use case
right away.

Then at most 2 seconds (plan: 0.5 seconds); the implementations are listed
completely at once.

## Platforms (NFR-004)

- ADR-0004 planned Windows as allowed failure; since 2026-10-12 all three
  platforms are required jobs (`docs/status.md`).
- Code: CI runs fmt, clippy, `requimd check --locked` here and in
  `examples/shop`, and all tests on the three platforms, then the
  performance tests in release mode
  (`cargo test --release -p requimd-lsp -p requimd-checks nfr_00`).
- Automated: `nfr_004_output_paths_use_forward_slashes` (cli main.rs).

### TC-603 The CI test suite passes on Linux, macOS and Windows
ver: NFR-004

Given the latest run of the CI workflow on `main` on GitHub.

When its jobs are inspected.

Then the test jobs for Linux, macOS and Windows have passed.

Code: `nfr_004_output_paths_use_forward_slashes` checks the paths in the
output on every platform.

## Installation (NFR-005)

- Gap: the releases ship each binary in an archive (status of M5). The
  scale counts files a user installs; an archive with one executable
  inside presumably counts as one.
- Code: `nfr_005_single_binary_runs_alone` copies the executable into an
  empty folder and runs `init`, `check` and `export site` there. The release
  smoke test runs `--version` and a check on every platform except macOS
  x86_64.
- Automated: `nfr_005_single_binary_runs_alone` (cli tests).

### TC-604 Installation from one file on a clean machine
ver: NFR-005

Given a Windows user account without Rust, requimd or developer tools.

When the Windows package of the latest pre-release is downloaded, the
executable is placed in a folder on `PATH`, and `requimd check` runs in a
copy of the base project; then the `.vsix` is installed in VS Code.

Then the check runs without installing any runtime or library; the
extension works without further installation (TC-502).

## Offline operation (NFR-006)

- Gap: the scale names only `check`, `fmt` and `lsp`; the other commands
  (`export`, `diff`, `show`, `coverage`, `review`, `init`) are not covered,
  though the project rules ask for no network access at runtime at all.
- Decision F5 (2026-10-16), ADR-0011 (proposed): NFR-006 covers all
  commands; the network only for commands invoked and configured for it,
  with an ADR of their own. TC-605 already runs all commands offline.
- Code (meter differs): `nfr_006_no_network_crates_in_dependency_tree`
  checks `Cargo.lock` for network libraries instead of running the tests
  offline. `requimd diff` calls the local `git` command.
- Automated: `nfr_006_no_network_crates_in_dependency_tree` (cli tests).

### TC-605 Working without network
ver: NFR-006

Given the computer offline (airplane mode or network cable unplugged).

When `requimd check`, `requimd fmt` and a Neovim session with completion,
hover and a code action run in a copy of the base project; also
`requimd export site out` and `requimd diff HEAD` (in a committed copy).

Then everything works as online. In a firewall log, if available, requimd
opens no connection.

## Editor neutrality (NFR-007)

- Gap: the meter says "manual test per feature, documented in
  `docs/neovim.md`"; by now `scripts/nvim-check.lua` checks the features
  automatically (Log, M2).
- Code: `nfr_007_neovim_builtin_client_without_plugins` runs
  `scripts/nvim-check.lua` in `nvim --clean`; without Neovim installed, the
  test passes and only prints a note.
- Automated: `nfr_007_neovim_builtin_client_without_plugins` (cli tests);
  also checked in Neovim (`scripts/nvim-check.lua`).

### TC-606 All `must` language server features in plain Neovim
ver: NFR-007

Given Neovim 0.12 started with `--clean` and only the configuration of
`docs/neovim.md`.

When every `must` feature of the table in `docs/neovim.md` is tried in the
base project (TC-200 to TC-213, TC-215 to TC-217).

Then each is usable without any plugin.

## Plain Markdown readability (NFR-008)

- Code: `nfr_008_shop_metadata_blocks_end_with_blank_line` is a proxy: it
  checks that every metadata block in `examples/shop` ends with a blank
  line, so GitHub does not join it with the text. The rendering itself stays
  a manual review.
- Automated: `nfr_008_shop_metadata_blocks_end_with_blank_line` (cli tests).

### TC-607 Requirements render cleanly on GitHub
ver: NFR-008

Given the files of `examples/shop` and of `requirements/` in the GitHub
web view (and on GitLab, if available).

When each file is read as a stakeholder would.

Then no raw markup shows other than the metadata block; inline allow
markers are invisible; code examples appear as code.

Validate: on GitHub the lines of a metadata block run together into one
paragraph (`impl: … prio: … status: …`). Readable enough for a
stakeholder?

Code: `nfr_008_shop_metadata_blocks_end_with_blank_line` checks only that
metadata blocks end with a blank line.

## Robustness (NFR-009)

- Code: `nfr_009_no_crash_on_10000_random_edits` edits the files of
  `examples/shop` and `examples/broken` at random and sends a hover,
  definition, implementation or completion request every fifth edit;
  `fr_012_survives_arbitrary_input` does the same for the parser alone.
- Automated: `nfr_009_no_crash_on_10000_random_edits` (lsp tests).

### TC-608 The server survives broken input
ver: NFR-009, FR-012

Given the fuzz test of the meter, 10,000 random edits
(`cargo test --release -p requimd-lsp --test server nfr_009`); and
`functional.md` open in Neovim.

When the fuzz test runs; and in Neovim text is typed at random, a binary
file's content is pasted, the whole file is deleted and restored with
`u`, and a heading `## FR-` is left half-typed.

Then the fuzz test reports no crash or hang. In Neovim the server keeps
answering (hover still works) and diagnostics recover after the undo.

## Test traceability (NFR-010)

- Code: the `must` requirements per milestone are written into
  `nfr_010_every_must_requirement_of_a_milestone_has_a_test` by hand,
  following `docs/roadmap.md`; a new `must` item must be added there. BR-002
  and BR-004 are not in the list and have no test of their own.
- Automated: `nfr_010_every_must_requirement_of_a_milestone_has_a_test` (cli
  tests).

### TC-609 Every `must` requirement of a done milestone has a named test
ver: NFR-010

Given the test of the meter, `cargo test -p requimd-cli --test cli nfr_010`.

When it runs; and for a few `must` requirements (for example FR-100,
FR-212, FR-300) the test names in the code are searched for the ID.

Then the script reports 100 %; each sampled ID appears in a test name.

## VS Code feature coverage (NFR-011)

- Code: `npm test` in `editors/vscode/` runs 19 tests in VS Code 1.91.1; CI
  runs them twice, against the source and against a packaged `.vsix`.
- Automated: Checked in VS Code
  (`editors/vscode/src/test/suite/features.ts`).

### TC-610 The automated VS Code test passes in CI
ver: NFR-011

Given the latest CI run on `main`.

When the job `vscode` is inspected.

Then it has passed, against the extension built from source and against
the packaged `.vsix` with its bundled executable. The manual counterpart
is TC-506.
