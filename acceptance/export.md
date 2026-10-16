# Acceptance tests: export and reporting

Tests for FR-400 to FR-405, run in a terminal in a fresh copy of the base
project. The site is checked in a browser opened on the files, without a
web server.

Decisions for the whole area:

- Log 2026-10-12: stakeholders read the static site, and PDFs made by an
  external Markdown-to-PDF converter; no Confluence (FR-405 deprecated).
- Log 2026-10-12: CI builds the site of this repository and offers it as
  download `requimd-site`; publishing on GitHub Pages comes later.

## Static site (FR-400)

- Log 2026-10-12: the site shows all items with a status badge;
  `--status` filters; no JavaScript.
- Gap: links to items that `--status` leaves out; the handling of an
  existing, non-empty output folder; whether the glossary and the text
  outside items (introductions) appear on the site.

### TC-400 One page per file, anchors, references, backlinks and index
ver: FR-400

Given the base project.

When `requimd export site out` runs and `out/index.html` is opened in a
browser by double click, with JavaScript switched off in the browser.

Then there is one page per Markdown file and an index of all items. Each
item and each step has an anchor; every reference is a link that jumps to
its target, also to the step `UC-901.s2`; each item lists its backlinks
(for `FR-902`: `NFR-901` and `TC-901`). Everything works without
JavaScript and without a web server.

### TC-401 Status on every item; `--status` filters
ver: FR-400

Given `FR-903` with `status: review` and `FR-902` with `status: approved`.

When `requimd export site out`; then
`requimd export site out2 --status review,approved`.

Then every item on the first site shows its status. The second site
contains only `FR-902` and `FR-903`. Note how links to left-out items, such
as `UC-901.s2`, appear (Gap).

## Traceability matrix (FR-401)

- Log 2026-10-12: one row per step with implementing and verifying items.
- Gap: delimiter, character encoding and destination of the CSV output
  are not specified. Excel with German regional settings expects a
  semicolon and opens UTF-8 without byte order mark with broken umlauts.

### TC-402 The matrix as a page and as CSV
ver: FR-401

Given the base project.

When `requimd export site out` and `out` is opened; then
`requimd export matrix`.

Then the site has a matrix page, and the CSV has one row per step: `s1`
with `FR-901` and no verifying item; `s2` with `FR-902` and `TC-901`;
`s2a` with `FR-903` and none. Each row has use case, step label, step
text, implementing and verifying items.

### TC-403 The CSV in a spreadsheet
ver: FR-401

Given step `s1` of `UC-901` with the text "Kunde öffnet den Warenkorb,
prüft die Größe." (umlauts and a comma).

When the CSV of `requimd export matrix` is saved to a file and opened in
Excel by double click.

Then each value is in its own column and the umlauts are correct.

Validate: this is how stakeholders will open it (Gap: format).

## JSON export (FR-402)

- Gap: whether body fields, metadata such as status and priority, and the
  glossary are part of the "complete item graph"; the unit of positions
  (bytes or characters, 0- or 1-based).

### TC-404 The item graph as JSON with a format version
ver: FR-402

Given the base project.

When `requimd export json` runs (and is saved to a file).

Then valid JSON with a format version, all eight items, the three steps,
all links with type, the mention of `BR-901` in `FR-902`, and a source
location for each.

## Requirement changelog (FR-403)

- Log 2026-10-12: `requimd diff` calls the `git` command, no git library,
  to keep a single executable (NFR-005). Without `ref2` it compares with
  the working tree, including uncommitted changes. Changed means title,
  text, links, other metadata, steps or file differ, whitespace-insensitive
  as in FR-116; status is listed with old and new value.
- Q9 (to confirm): a reference without `requimd.toml` in the project folder
  has no items, so everything is "added"; output is sorted by prefix and
  number and ends with a summary line ("no changes" if none); exit code 0
  also when there are changes; a use case's text is compared without its
  steps, steps one by one by label, and moving a step in the list is no
  change.
- Gap: the result when `git` is not installed.

Preparation for these tests, in the copy of the base project:

```text
git init
git add .
git commit -m base
```

### TC-405 `diff` against the working tree
ver: FR-403

Given the base project committed. Then: the title of `FR-901` changes;
`FR-902` gets `status: review`; `UC-901` gets a step
`- s3: Customer pays.`; `TC-901` is deleted; all uncommitted.

When `requimd diff HEAD` runs.

Then the output lists `FR-901` (title), `FR-902` (status draft → review),
`UC-901` (step `s3` added) and `TC-901` (removed), sorted, with a summary
line; exit code 0 (Q9).

### TC-406 What counts as a change
ver: FR-403

Given the base project committed. Then: the text of `FR-903` is wrapped
differently; `FR-901` moves to a new file `moved.md`; `BR-901` is renamed
to `BR-902` by hand; the steps `s1` and `s2` of `UC-901` swap places; a
new `## FR-? New` item is added.

When `requimd diff HEAD` runs.

Then `FR-903` is not listed; `FR-901` is listed as moved (file);
`BR-901` as removed and `BR-902` as added; `UC-901` is not listed (Q9:
order is no change); the placeholder is not listed.

### TC-407 Two references, JSON output and errors
ver: FR-403

Given TC-405's changes committed as a second commit.

When `requimd diff HEAD~1 HEAD`; then the same with `--format json`; then
`requimd diff no-such-ref`.

Then the first lists the same as TC-405; the second the same as a JSON
array; the third prints the git error and ends with exit code 2.

### TC-408 Each side uses its own configuration
ver: FR-403

Given a commit in which `requimd.toml` has `[values]` with
`prio = ["high", "low"]` and the items use `prio: high`, followed by
a commit back to the defaults with `prio: must`; and a repository whose
first commit has no `requimd.toml`.

When `requimd diff HEAD~1 HEAD` in each case.

Then the items are compared with their own value sets, and the priority
change is listed as metadata change. Against the commit without
`requimd.toml`, every item is "added" (Q9, to confirm).

## Diagram export (FR-404)

- Log 2026-10-12: Mermaid flowchart in a Markdown code block, direct
  neighbours by default, `--depth` for more; steps shown inside their use
  case, mentions dashed, status per node.
- Q10 (to confirm): neighbours follow links and mentions in both
  directions and arrows carry the link name of `requimd.toml`
  ("implements"); a use case with shown steps is an untitled box holding
  the use case node and only the steps that shown items point to; draft
  has a dashed border, review yellow, approved green, deprecated grey,
  other values no style; step text is cut at 50 characters.

### TC-409 An item and its direct neighbours
ver: FR-404

Given the base project.

When `requimd export mermaid FR-902` runs and the output is pasted into
<https://mermaid.live> or a Markdown file on GitHub.

Then a Mermaid code block: `FR-902` highlighted with ID, title and status;
an arrow "implements" to step `s2` inside a box of `UC-901`; arrows
"refines" from `NFR-901` and "verifies" from `TC-901`; a dashed arrow to
`BR-901` (mention). Nothing else.

### TC-410 Depth, step references and output file
ver: FR-404

Given the base project; `FR-903` with `status: review`.

When `requimd export mermaid UC-901 --depth 2 -o uc.md`; then
`requimd export mermaid UC-901.s2`.

Then `uc.md` contains the diagram with `UC-901`, its steps with
`FR-901` to `FR-903`, and two steps away `NFR-901`, `TC-901` and
`BR-901`; `FR-903` is styled as review. The step reference starts at
`UC-901`.

### TC-411 Exit codes of the diagram export
ver: FR-404

Given a new file `a.md` with `## FR-901 Copy`.

When `requimd export mermaid FR-999`; `requimd export mermaid FR-901`;
`requimd export mermaid hello`.

Then exit codes 1, 1 and 2, each with a message.

## Publishing target (FR-405)

Deprecated on 2026-10-12 by the product owner: no Confluence; PDFs come
from an external Markdown-to-PDF converter. No tests.
