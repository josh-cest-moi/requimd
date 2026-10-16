# Acceptance tests: quality requirements

Tests for NFR-001 to NFR-011. Each quality requirement names its own
measurement (`meter`); the tests follow it. Where the meter is an
automated benchmark or test, phase 2 of the testing guide adds the command
that runs it.

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

### TC-600 Diagnostics latency on a large project
ver: NFR-001

Given the generated project of the meter (2,000 items in 200 files).

When the latency benchmark runs (command: phase 2); and in Neovim one of
its files is edited by typing fast.

Then the 95th percentile is at most 300 ms (plan: 100 ms). In Neovim the
diagnostics follow the typing without noticeable delay.

## Full check duration (NFR-002)

### TC-601 `requimd check` on a large project
ver: NFR-002

Given the generated project of NFR-001 on a standard laptop.

When `Measure-Command { requimd check }` runs in PowerShell in its root.

Then at most 5 seconds (plan: 1 second).

## Server start (NFR-003)

### TC-602 Time until the item graph is complete
ver: NFR-003

Given the generated project of NFR-001.

When the start benchmark runs (command: phase 2); and in Neovim a file of
the project is opened and `gri` is used on a use case right away.

Then at most 2 seconds (plan: 0.5 seconds); the implementations are listed
completely at once.

## Platforms (NFR-004)

- ADR-0004 planned Windows as allowed failure; since 2026-10-12 all three
  platforms are required jobs (`docs/status.md`).

### TC-603 The CI test suite passes on Linux, macOS and Windows
ver: NFR-004

Given the latest run of the CI workflow on `main` on GitHub.

When its jobs are inspected.

Then the test jobs for Linux, macOS and Windows have passed.

## Installation (NFR-005)

- Gap: the releases ship each binary in an archive (status of M5). The
  scale counts files a user installs; an archive with one executable
  inside presumably counts as one.

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

### TC-606 All `must` language server features in plain Neovim
ver: NFR-007

Given Neovim 0.12 started with `--clean` and only the configuration of
`docs/neovim.md`.

When every `must` feature of the table in `docs/neovim.md` is tried in the
base project (TC-200 to TC-213, TC-215 to TC-217).

Then each is usable without any plugin.

## Plain Markdown readability (NFR-008)

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

## Robustness (NFR-009)

### TC-608 The server survives broken input
ver: NFR-009, FR-012

Given the fuzz test of the meter (10,000 random edits, command: phase 2);
and `functional.md` open in Neovim.

When the fuzz test runs; and in Neovim text is typed at random, a binary
file's content is pasted, the whole file is deleted and restored with
`u`, and a heading `## FR-` is left half-typed.

Then the fuzz test reports no crash or hang. In Neovim the server keeps
answering (hover still works) and diagnostics recover after the undo.

## Test traceability (NFR-010)

### TC-609 Every `must` requirement of a done milestone has a named test
ver: NFR-010

Given the script of the meter (command: phase 2).

When it runs; and for a few `must` requirements (for example FR-100,
FR-212, FR-300) the test names in the code are searched for the ID.

Then the script reports 100 %; each sampled ID appears in a test name.

## VS Code feature coverage (NFR-011)

### TC-610 The automated VS Code test passes in CI
ver: NFR-011

Given the latest CI run on `main`.

When the job `vscode` is inspected.

Then it has passed, against the extension built from source and against
the packaged `.vsix` with its bundled executable. The manual counterpart
is TC-506.
