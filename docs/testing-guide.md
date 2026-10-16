# Testing guide

This guide supports the verification of the specification for the first
release, v0.1.0. It walks through every requirement of `requirements/`,
names the decisions behind it and gives tests to run by hand: end-to-end
scenarios along the use cases here, detailed tests per requirement in
`acceptance/`.

It was written in two phases. Phase 1 used only the specification, the
decision records and the decision log, without looking at the code; the
expected results of the tests come from there. Phase 2 read the code and
added, at each requirement, the decisions that exist only in the code
(tag **Code**) and the automated tests that cover it, at each test what the
code does where the spec is silent, and the commands for the benchmarks.
Section 7 summarises what phase 2 found.

## 1. Approach

Every requirement raises two questions:

- **Validation:** is this the right requirement? Only the product owner
  can answer this. The scenarios (section 4) and the points tagged
  **Validate** serve this question.
- **Verification:** does requimd do what the requirement says? The tests
  in `acceptance/` serve this question.

For each requirement, record one verdict in
`docs/verification-v0.1.0.md`:

| Verdict | Meaning | What follows |
|---------|---------|--------------|
| ok | Right requirement, requimd fulfils it | Status `approved` (by the owner, BR-004) |
| spec | The requirement must change | A `spec/` pull request |
| bug | requimd does not do what the requirement says | A `fix/` pull request with a regression test |
| question | Needs a decision first | An entry in `docs/status.md`, open questions |

Every point tagged **Gap** needs a decision as well; the findings list in
section 6 collects them.

The steps after this guide, as agreed:

1. Phase 1: this guide and the acceptance tests (done).
2. Phase 2: undocumented decisions from the code, added at the
   requirements and tests they concern (done).
3. A report on the open, planned and possible features.
4. Consolidating the requirements from the verdicts (`spec/` pull
   requests), then vision and scope of v0.1.0, then the roadmap.

### Recording the verified state with requimd itself

Set `status: approved` on each item you have verified. Approve in this
order, because an approved item that links to a draft item gets a
`status-link` warning (FR-109): first the business rules and use cases,
then the functional requirements (they link to use case steps), then the
quality requirements (they link to functional requirements). When all
items are approved, tag the commit, for example `spec-v0.1.0`. From then
on `requimd diff spec-v0.1.0` lists every change since the verification,
and suspect links on the `ver` links of `acceptance/` name the tests to
run again.

## 2. What is where

| Path | Content |
|------|---------|
| `docs/testing-guide.md` | This guide: approach, preparation, scenarios, decisions, findings |
| `acceptance/` | 185 acceptance tests as `TC` items, one file per area; conventions and tags in `acceptance/README.md` |
| `examples/acceptance/` | The base project the tests start from |
| `docs/verification-v0.1.0.md` | The record of verdicts, one row per item |
| `docs/neovim.md`, `docs/vscode.md` | Editor setup and the feature tables the tests refer to |
| `docs/decisions/` | Decision records (ADRs), summarised in section 5 |
| `docs/status.md` | Decision log (tag **Log**) and open interpretations (tags **Q5** to **Q10**) |

The acceptance tests are part of this repository's requimd project. Open
them in Neovim inside this repository: hover on the ID after `ver:` shows
the requirement, and find references on a requirement's heading lists its
tests. `requimd export matrix` in the repository root shows, per use case
step, the requirements and the tests that verify them.

Tags used in the tests, in short (details in `acceptance/README.md`):
**ADR-000n** decision record; **Log** decision log entry; **Q5** to
**Q10** interpretation to confirm; **Gap** spec silent, ambiguous or
contradictory; **Validate** clear, but possibly surprising; **Code**
decision only in the code (phase 2).

## 3. Preparation

### requimd

Use the latest pre-release from the GitHub release page, or build from
the repository root:

```bash
cargo build --release
```

Put the folder of the executable on `PATH`, so `requimd --version` works
in any folder. If Neovim is running the server, `:lsp stop` first, or
Windows refuses to overwrite `requimd.exe`.

### A fresh copy of the base project

Most tests start from a fresh copy of `examples/acceptance/`. In
PowerShell, from the repository root:

```powershell
$rq = Join-Path $env:TEMP 'rq'
Remove-Item -Recurse -Force $rq -ErrorAction Ignore
Copy-Item -Recurse examples\acceptance $rq
Set-Location $rq
```

Tests of `requimd diff` (FR-403) and of moving files (BR-005) need the
copy in git:

```bash
git init
git add .
git commit -m base
```

Tests that say "checked once" need one `requimd check` first, which
creates `requimd.lock`.

### Neovim

Configure the server as in `docs/neovim.md`. Code lenses (FR-215) are
not shown by default; add inside the `LspAttach` callback of that
configuration:

```lua
vim.lsp.codelens.enable(true, { bufnr = ev.buf })
```

Keys used in the tests: `K` hover, `<C-]>` go to definition, `grr`
references, `gri` implementation, `gra` code action, `grn` rename, `gO`
document symbols, `]d` and `[d` next and previous diagnostic, `<C-w>d`
diagnostic float, `<C-x><C-o>` completion. `:checkhealth vim.lsp` shows
whether requimd is attached; `:Inspect` shows the semantic tokens under
the cursor.

Tips: render-markdown hides raw text outside the cursor line, toggle it
if it gets in the way. marksman attaches to Markdown as well; if `<C-]>`
or `grr` offers two servers, pick requimd.

### VS Code

Install the `.vsix` of your platform as described in `docs/vscode.md`. In
VS Code the tests only make sure the features work (TC-500 to TC-506);
the details are tested in Neovim.

### The automated tests

Each requirement section in `acceptance/` lists its automated tests under
**Automated**. Their names contain the requirement ID (NFR-010), so a name
filter runs the tests of one requirement. From the repository root:

| What | Command |
|------|---------|
| All Rust tests | `cargo test --all` |
| The tests of one requirement | `cargo test fr_116` |
| Performance and robustness as in CI (NFR-001 to NFR-003, NFR-009) | `cargo test --release -p requimd-lsp -p requimd-checks nfr_00` |
| Latency, printing the 95th percentile | `cargo test --release -p requimd-lsp --test server nfr_001 -- --nocapture` |
| Neovim check (NFR-007) | `nvim --headless --clean -l scripts/nvim-check.lua target/release/requimd` |
| VS Code tests (NFR-011) | `npm ci`, then `npm test` in `editors/vscode/` |
| Test traceability (NFR-010) | `cargo test -p requimd-cli --test cli nfr_010` |

On Windows the executable for the Neovim check is
`target/release/requimd.exe`. Every requirement has automated tests except
BR-002 (covered by the tests of FR-212), BR-004 (a rule for people),
FR-112 (not built), FR-405 (deprecated) and FR-504 (not written yet).

A passing automated test shows that the code does what its author
understood. The acceptance tests are there to check that this is what the
specification says and what you want.

## 4. Scenarios along the use cases

Each scenario follows the steps of one use case in a fresh copy of the
base project, in Neovim unless noted. It shows whether the flow fits the
work of an analyst; the detailed tests follow in brackets. The
requirements per step are those that link to it with `impl`.

### UC-001 Write a new requirement

| Step | Do | Observe | Requirements | Tests |
|------|----|---------|--------------|-------|
| s1 | At the end of `functional.md`, type `## FR-? Cancel order` | `placeholder` warning | FR-001, FR-010 | TC-001, TC-022 |
| s2 | On the next line, `<C-x><C-o>`, pick `impl`, type `UC-`, then `prio: ` | keys, then steps of `UC-901` with their text, then the priority values | FR-003, FR-203, FR-204 | TC-007, TC-204 to TC-206 |
| s2a | Make the link `UC-901.s4` | `dangling-ref` error; `gra` offers to add the step | FR-101, FR-213 | TC-103, TC-220 to TC-223 |
| s3 | Write "The system should cancel the order fast." | `modal-verb` and `weak-word` while typing; `no-shall` until "shall" is written | FR-012, FR-111, FR-113, FR-114, FR-118, FR-202, FR-216 | TC-124 to TC-131, TC-147, TC-202 |
| s3a | `gra` on "fast" | reason in the message; actions "allow here", "allow in project", "add to weak words" | FR-110, FR-119, FR-219 | TC-149 to TC-153, TC-236 to TC-238 |
| s4 | `gra` on the heading, "Assign ID" | `FR-904`; warning gone | FR-212, FR-304 | TC-217, TC-218, TC-310 |

Afterwards, as the post-condition of UC-001 says, the file contains a
valid item with a unique ID, and `requimd check` reports no error.

Validate:

- Does writing a linked requirement take no more keystrokes than in a
  wiki (first business objective in `requirements/vision.md`)?
- Are the messages clear to an analyst, and are the default severities
  right (for example `no-shall` only info)?
- FR-112 (EARS patterns) also implements s3 but is not built (`could`,
  "Later").

### UC-002 Navigate traceability in the editor

| Step | Do | Observe | Requirements | Tests |
|------|----|---------|--------------|-------|
| s1 | Open `usecases.md`; `gO` | a lens per step with count and IDs; the outline with steps below the use case | FR-210, FR-215 | TC-215, TC-228 |
| s2 | Run the lens of `s2`, or `gri` on `s2` | `FR-902` | FR-208 | TC-213 |
| s3 | Open `FR-902`; `K` on `UC-901.s2` and `BR-901` | inlay hint with the step text; hover with target details | FR-205, FR-209 | TC-207 to TC-209, TC-214 |
| s4 | `<C-]>` on `UC-901.s2`; workspace symbol search for "total" | jump to the step; `FR-902` and `NFR-901` | FR-206, FR-211 | TC-210, TC-216 |
| s5 | `grr` on the heading of `FR-902`; `requimd show FR-902` | links from `NFR-901` and `TC-901` | FR-207, FR-305 | TC-211, TC-212, TC-313 |

Validate: are the default lengths of hover (500) and inlay hints (40)
right? Is a 4-line Neovim handler for the code lens acceptable (Log
2026-10-12)?

### UC-003 Validate the repository

| Step | Do | Observe | Requirements | Tests |
|------|----|---------|--------------|-------|
| s1 | `requimd check` in the copy | | FR-300 | TC-300 |
| s2 | (the parse is visible through the results) | all files read, references across files resolve | FR-001 to FR-013, FR-217 | `format.md` |
| s3 | Add a few errors from `checks.md` | one line per finding with file, line, column, severity, message and check ID; `--format json`; `requimd coverage` | FR-100 to FR-109, FR-115, FR-117, FR-300, FR-301, FR-306 | `checks.md`, TC-306, TC-315 |
| s4 | `$LASTEXITCODE` | 1 with errors, 0 without; `--strict` counts warnings | FR-300, FR-302 | TC-300, TC-307 |
| s2a | Run in a folder without `requimd.toml` | the reason, exit code 2 | FR-013, FR-300 | TC-030 |

This repository's CI runs `requimd check --locked` here and
`requimd check --strict --locked` in `examples/shop`.

Validate: is the output understandable without developer help? Would an
analyst run the check locally, or only rely on CI?

### UC-004 Review the impact of a change

| Step | Do | Observe | Requirements | Tests |
|------|----|---------|--------------|-------|
| s1 | `requimd check` once; then change the text of `FR-902` and save | | FR-116 | TC-140 |
| s2 | | the links to `FR-902` are suspect | FR-116 | TC-140 to TC-144 |
| s3 | Open `other.md` | `suspect-link` at `ref: FR-902` and `ver: FR-902` | FR-116 | TC-140 |
| s4 | `gra` on one link, "Mark as reviewed"; `requimd review FR-902` for the rest | | FR-218, FR-307 | TC-234, TC-316, TC-317 |
| s5 | `git diff requimd.lock` | new hashes; the warnings are gone | FR-218, FR-307 | TC-234 |

Validate:

- What counts as a change: status and whitespace do not; title, text and
  all other metadata, including a new outgoing link of the target, do
  (TC-141, TC-142).
- requimd shows that a target changed, not what changed. A view based on
  git history is on the "Later" list (ADR-0007).
- `requimd review FR-902` also clears suspect links from `FR-902` to its
  own targets (TC-316).

### UC-005 Rename an item ID

| Step | Do | Observe | Requirements | Tests |
|------|----|---------|--------------|-------|
| s1 | `grn` on the heading of `FR-902`, new ID `FR-920` | | FR-214 | TC-224 |
| s2 | Save all; `requimd check` | heading, links, lock entries renamed; no suspect links | FR-214 | TC-224, TC-225 |
| s1a | `grn` with `FR-901`, `FR-9`, `NFR-950` | each refused with a message | FR-214 | TC-226 |

Validate: the old number stays used for good (BR-002, TC-702). Renaming
across prefixes, for example a requirement that becomes a rule, is a
manual edit (Log 2026-10-12).

### UC-006 Publish requirements for stakeholders

Command line.

| Step | Do | Observe | Requirements | Tests |
|------|----|---------|--------------|-------|
| s1 | `requimd export site out`; `requimd export json` | | FR-400, FR-402 | TC-400, TC-404 |
| s2 | Open `out/index.html`; `requimd export matrix`; `requimd diff HEAD`; `requimd export mermaid FR-902` | pages with links and backlinks, matrix, changelog, diagram | FR-400 to FR-404 | TC-400 to TC-411 |
| s3 | Publish the site | | FR-405 (deprecated) | none |

Validate:

- Step s3 is implemented only by FR-405, which is deprecated, so no live
  requirement covers it, and `uncovered-step` does not notice because a
  deprecated item still counts (TC-118). Today CI offers the site of this
  repository as a download; GitHub Pages is "later".
- Is the site what stakeholders need? Stakeholders get PDFs from an
  external converter (Log 2026-10-12): try one on a requirements file and
  check how the metadata block looks.

### UC-007 Set up a project

| Step | Do | Observe | Requirements | Tests |
|------|----|---------|--------------|-------|
| s1 | `requimd init` in an empty folder | commented `requimd.toml`, empty glossary | FR-014, FR-303 | TC-308, TC-309 |
| s2 | Neovim configuration from `docs/neovim.md` | | NFR-007 | TC-606 |
| s2a | Install the `.vsix` in VS Code | | FR-500 to FR-503 | TC-500 to TC-506 |
| s2b | IntelliJ IDEA with LSP4IJ | | FR-504 (open) | TC-507 |
| s3 | Open a Markdown file of the project | the server attaches; files outside the project get nothing | FR-200, FR-201, FR-308 | TC-200, TC-201 |

Validate: can an analyst do this without help from a developer, as
`requirements/vision.md` assumes ("or have someone in the team who is")?

## 5. Design decisions

Eight decision records exist in `docs/decisions/`. The acceptance tests
describe the decisions at the requirements they affect; this is the
overview.

| ADR | Status | Decision | Requirements |
|-----|--------|----------|--------------|
| 0001 Implementation language | accepted 2026-10-09 | Rust: memory safety against broken input without garbage collector, a single executable, the best Markdown and LSP libraries. Go was the alternative; Python and TypeScript need a runtime. | NFR-001 to NFR-005, NFR-009 |
| 0002 Markdown dialect | accepted | Plain CommonMark with conventions: an item is a heading starting with an ID, metadata a `key: value` block, references bare IDs, steps labelled list items, code ignored. Rejected: YAML front matter, wiki links, Pandoc attributes, HTML comments, AsciiDoc. | FR-001 to FR-009, NFR-008 |
| 0003 Markdown parser and LSP library | accepted 2026-10-10 | pulldown-cmark (event stream with byte offsets, requimd builds its own item model) and lsp-server (synchronous loop, own debouncing); toml_edit to edit `requimd.toml` keeping comments; clap, serde_json. No async runtime. | FR-001 to FR-014, FR-200 to FR-219, FR-301 |
| 0004 Hosting and CI | accepted 2026-10-09 | GitHub and GitHub Actions; free runners for the three platforms. The consequence "Windows as allowed failure" is outdated: Windows is required since 2026-10-12. | NFR-004, NFR-005 |
| 0005 Language checks | accepted 2026-10-09 | Three stages (word list, pattern rules, later AI review); modal verbs a separate check; explicit word forms; titles checked; silencing by allow list and inline comment; lists in `requimd.toml`. | FR-110, FR-114, FR-118, FR-119, FR-219 |
| 0006 Project name | accepted 2026-10-12 | requimd, spoken "requi-m-d"; one name everywhere: binary, `requimd.toml`, `requimd.lock`, inline marker, LSP commands, crates. | all |
| 0007 Lock file | accepted 2026-10-12 | `requimd.lock` in TOML, sorted, one line per link with a 64-bit FNV-1a hash of the target, `[ids]` with the highest number per prefix; `check` records new links, only a review changes a hash; `--locked` for CI. | FR-116, FR-212, FR-214, FR-218, FR-300, FR-307 |
| 0008 Editor integrations | **proposed** 2026-10-13 | VS Code extension with the executable bundled per platform, packages on GitHub Releases, marketplaces later; IntelliJ through a setup guide for LSP4IJ. | FR-500 to FR-504, NFR-005, NFR-011 |

To decide: accept ADR-0008 (open business question 13 in
`docs/status.md`).

Beyond the ADRs, `docs/status.md` holds a decision log of 64 entries
(tag **Log** in the tests) and, under open business questions 5 to 10,
about 25 interpretations that are implemented but not in the
specification (tags **Q5** to **Q10**). The tests name each at the
requirement it concerns; confirming them is part of the verification.

## 6. Findings from phase 1

Points found while writing the tests. Each needs a decision: change the
spec, accept as is, or record as open question. The tests named give the
details. Phase 2 answers many of the unspecified points from the code;
section 7 says how.

### Documents out of date

- `requirements/vision.md`: "Planned later" names suspect links, static
  site, matrix and changelog, all built by now; the scope of the first
  release needs a rewrite. It calls Neovim the primary editor, while
  ADR-0008 says analysts use VS Code more often.
- `docs/status.md` counts 69 functional and 10 quality requirements; there
  are 74 and 11 since M6.
- NFR-007: the meter says manual test, but `scripts/nvim-check.lua` checks
  automatically (TC-606).
- ADR-0004: Windows is no longer an allowed failure (section 5).
- FR-014 defines the defaults as this repository's `requimd.toml`, which
  ties the repository's own configuration to the defaults of every
  project (TC-033, TC-034).

### Contradictions

- FR-213: a stub gets "a placeholder ID" and, for a dangling reference,
  "that ID" (TC-220).
- FR-205: "on a reference" includes glossary terms, which are no
  references (TC-209).
- FR-003 against FR-102: a line with an unknown key is no metadata, yet
  its key is reported as unknown (TC-108).
- FR-119 against the example of FR-110: whether an allowed phrase
  silences the weak word inside it (TC-153).
- BR-003 against FR-213: new steps should get the next free number; the
  step stub uses the referenced label (TC-222, TC-703).
- UC-006.s3 is covered only by the deprecated FR-405 (section 4, UC-006).

### Not specified

- Format: case of prefixes (TC-002); headings in block quotes (TC-003);
  several values for keys other than links (TC-012); body fields in any
  output (TC-013); the visible effect of extension steps (TC-015); IDs in
  non-link metadata as mentions (TC-019); a title heading in the glossary,
  the default glossary path (TC-026, TC-308); which diagnostic malformed
  content gets (TC-027); nested projects (TC-032); `FR-0901` against
  `FR-901` (TC-700).
- Checks: case of values (TC-111); one or several diagnostics for missing
  keys (TC-113); self references in `ref` cycles (TC-121); status
  consistency for step targets (TC-123); single quotes in language checks
  (TC-127); "shall" in titles and capitalised (TC-131); lock entries of
  deleted targets (TC-144); invalid severity values (TC-146); the form of
  configured modal verbs (TC-147); case of allow entries (TC-153).
- Language server: placeholders and duplicates as completion candidates
  (TC-204); body field keys in key completion (TC-206); nesting of item
  symbols (TC-215); the first ID of an unused prefix (TC-219); title and
  file of stubs (TC-221, TC-223); rename from a reference (TC-227);
  external changes to `requimd.toml` and `requimd.lock` (TC-233); where
  "add to weak words" is offered (TC-238).
- Command line: `--help` and `--version` (no requirement); output paths
  and order (TC-300); `--locked` failures against errors (TC-304); fields
  and units of JSON diagnostics (TC-306); `init` messages and nested
  projects (TC-309); separator form, placeholder order and lock update of
  `fmt` (TC-310, TC-311); output of `show` (TC-313); totals of `coverage`
  (TC-315); `review` of unknown IDs (TC-317).
- Export: links to filtered items and the output folder of the site
  (TC-401); CSV delimiter and encoding (TC-403); content of the JSON
  graph (TC-404); `diff` without git installed (section of FR-403).
- Quality: archives against "one file" (TC-604); commands outside the
  scale of NFR-006 (TC-605).
- Rules: tests of business rules, which cannot be `ver` targets
  (`rules.md`).

### Behaviour to validate

Clear in the spec, but possibly not what an analyst expects:

- Text after a nested item belongs to no item (TC-005).
- A blank line between heading and metadata silently turns the metadata
  into text (TC-010).
- An ID in a title is no reference (TC-020).
- An unclosed code fence silently removes the rest of the file (TC-028).
- `impl` to a whole use case covers none of its steps; deprecated items
  still cover steps (TC-117, TC-118).
- Word forms need their own entries, also for aliases ("baskets",
  TC-136); the modal verb "may" also matches the month (TC-148).
- A new outgoing link of a target makes its incoming links suspect
  (TC-142); a link edited by hand loses its review state (FR-116).
- An inline marker covers one physical line (TC-151).
- Columns count bytes (TC-301).
- `review <ID>` clears outgoing suspect links too (TC-316).
- Raising `[ids] digits` invalidates all existing IDs (TC-701).
- Only numbers up to the highest ever seen are protected from reuse
  (BR-002).
- On GitHub, a metadata block renders as one run-on paragraph (TC-607).

### Release topics not in the specification

For the scope of v0.1.0 (step 4 of section 1):

- `requimd --version` and `--help`: both exist (section 7), but no
  requirement makes them part of the contract.
- Stability promises for the format, `requimd.toml` and `requimd.lock`.
  The JSON export carries a format version (FR-402); the lock file has no
  version field in ADR-0007.
- A guide for analysts that introduces the format; today there is only
  the normative specification.
- Checksums for the release files (ADR-0008 notes that there are none).
- Licence notices of the libraries bundled in the executable and in the
  `.vsix` packages.

## 7. Findings from phase 2

What the code decides beyond the specification. Each point is also noted
at its requirement or test in `acceptance/` (tag **Code**). For each,
decide whether it goes into the spec as it is, should change, or stays an
implementation detail.

### Deviations from the specification

- FR-012: no diagnostic is recorded for malformed content; what the parser
  does not recognise stays plain text (TC-027, TC-028).
- FR-218 says the server records "its" new links on save; the code records
  the new links of the whole project, as `requimd check` does.
- NFR-006: the test checks `Cargo.lock` for network libraries instead of
  running the tests offline, as the meter says.
- NFR-007: the Neovim test passes, with a note, when Neovim is not
  installed.
- NFR-008: the test checks only that metadata blocks in `examples/shop`
  end with a blank line; the rendering stays a manual review.
- NFR-001 and NFR-003: the tests enforce the `must` values, not `plan`.

### Gaps the code answers

These were open in phase 1. The code's answer is a candidate for the
spec:

- Format: prefixes are case-sensitive (TC-002); headings in block quotes
  are items (TC-003); only link values have several values, and empty
  entries are skipped (TC-012); body fields appear in the JSON export
  (TC-404); extension steps have no special effect (TC-015); IDs in
  non-link metadata are no mentions, IDs in HTML comments are (TC-019);
  every glossary heading is a term, including `# Glossary` (TC-026);
  `requimd init` sets the glossary to `glossary.md` (TC-308); nested
  projects belong to the outer project too (TC-032); `FR-0901` and
  `FR-901` are different items (TC-700).
- Checks: values are case-sensitive (TC-111); one `missing-key` diagnostic
  lists all missing keys (TC-113); a `ref` to the item itself is a cycle
  (TC-121); a step has its use case's status (TC-123); "shall" in the
  title does not count, any capitalisation does (TC-131); lock entries of
  deleted targets are kept (FR-116); an invalid severity is a
  configuration error (TC-146); configured modal verbs take no reason
  (FR-118); allow entries ignore case (TC-153); a phrase split by a line
  break is not covered by a marker (TC-151).
- Language server: placeholders are not offered for completion,
  duplicates are (FR-203); body field keys are not offered (FR-204); no
  hover on an item's own heading (FR-205); items are listed flat in the
  outline (FR-210); an unused prefix starts at 001 (TC-219); a stub for a
  dangling reference uses that ID and has no title, a missing stub file is
  created (TC-220, TC-223); rename works from a reference (TC-227);
  changes to `requimd.toml` and `requimd.lock` outside the editor take
  effect (TC-233); "add to weak words" is offered in every item type
  (FR-219).
- Command line: `--version` and `--help` exist; output paths are relative
  to the current directory, sorted by file and position (FR-300); the
  fields of JSON diagnostics (TC-306); `init` messages, exit code 0, and a
  nested project without warning (TC-309); `fmt` uses two spaces and does
  not update the lock file (TC-310, TC-311); the output of `show` and
  `coverage` (FR-305, FR-306); `review` of an unknown ID ends with exit
  code 1 (TC-317).
- Export: the site has a page for every included file and the glossary,
  with all text; links to items left out by `--status` become plain text;
  old files in the output folder stay (TC-401); the CSV uses commas and
  UTF-8 without byte order mark (TC-403); the content of the JSON export
  (TC-404); `diff` without git ends with exit code 2 (FR-403).

### Decisions to validate

Behaviour that only the code defines and that may surprise an analyst:

- A heading such as `## FR-001: Title`, with a colon after the ID, is
  silently no item (TC-035).
- `prio:must` without a space after the colon is silently text, so a link
  written that way is lost (TC-036).
- The site renders plain CommonMark: Markdown tables, strikethrough and
  task lists appear as raw text, HTML as text (TC-412).
- The matrix CSV will likely not open cleanly in Excel with German
  regional settings (TC-403).
- Adding an inline allow marker to a target, for example with "allow
  here", makes its incoming links suspect, since the hash covers HTML
  comments (FR-116).
- With the default include pattern, every Markdown file below the root
  counts, including folders such as `node_modules`; `.gitignore` is not
  honoured (FR-013).
- A hyphen joins words: "fast-track" does not match the weak word "fast"
  (FR-110).
- An ID in an HTML comment is a mention and can be dangling (FR-008).
- Files that are not valid UTF-8 are read with replacement characters,
  without a message (FR-012).
- The site's labels are fixed in English (FR-400).
- Rename does not touch IDs in titles or in text outside items (FR-214).
- An ID that becomes duplicated is listed by `diff` as removed (FR-403).
- On save, the whole project's links are recorded (FR-218).

### Fixed names in the code

The code relies on these names from the default configuration. A project
that renames them in `requimd.toml` silently loses the related behaviour
(open business question 7 and technical question 1 in `docs/status.md`):

- types `usecase` (steps, coverage, implementations, stubs), `functional`
  (weak words, modal verbs, "shall", stubs for uncovered steps), `quality`
  (weak words), `story` (story pattern);
- link keys `impl` (coverage, implementations, matrix), `ref` (cycles),
  `ver` (matrix);
- keys `status` (hash, diff, site, status consistency, with the values
  `draft`, `review`, `approved` and `deprecated` in FR-109 and FR-404) and
  `ac` (story pattern).

### Release topics from the code

- No checksums are published, and the macOS x86_64 build is not
  smoke-tested in the release workflow (FR-502).
- The generated project of the performance requirements exists only
  inside the tests; a manual measurement of the command needs one
  (TC-601).

## 8. Not covered by tests

- FR-112 EARS patterns: `could`, not built, on the "Later" list.
- FR-405 Publishing target: deprecated.
- FR-504 IntelliJ setup guide: open in M6; TC-507 fails until the guide
  exists.
