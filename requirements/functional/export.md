# Export and reporting

## FR-400 Static site
impl: UC-006.s1, UC-006.s2  prio: should

The system shall provide `requimd export site <dir>`, which generates a static
website with one page per source file, anchors per item and step, clickable
references, a backlinks section per item, and an index of all items. The
site shall work without a web server and without JavaScript. Every item
shows its status; with `--status <list>`, for example `--status
review,approved`, the site contains only items with these statuses.

## FR-401 Traceability matrix
impl: UC-006.s2  prio: should

The system shall generate a traceability matrix of use case steps against
implementing and verifying items, as part of the static site and as CSV via
`requimd export matrix`. The matrix has one row per step with the use case,
the step label and text, the items that implement the step (`impl`) and the
items that verify one of them (`ver`).

## FR-402 JSON export
impl: UC-006.s1  prio: should

The system shall provide `requimd export json`, which writes the complete item
graph with items, steps, links, mentions and source locations as JSON. The
output states its format version, so consumers detect changes.

## FR-403 Requirement changelog
impl: UC-006.s2  prio: could

The system shall provide `requimd diff <ref1> [ref2]`, which lists items
added, removed or changed between two git references. Without `ref2`, it
compares `ref1` with the working tree, including uncommitted changes. Each
side is read with its own `requimd.toml`; the files of a reference are read
with the `git` command.

Items are matched by ID, so a renamed ID appears as removed and added.
Items without a unique ID (placeholders, duplicates) are not listed. An
item is changed if one of these parts differs, where runs of whitespace
count as one space (as in FR-116):

- title, text, links, other metadata;
- status, shown with the old and new value;
- steps of a use case, shown as the labels added, removed and changed;
- file, if the item moved to another file.

The output names the changed parts of each item. With `--format json` it
is a JSON array. The command ends with exit code 0, or 2 if git fails, for
example on an unknown reference.

## FR-404 Diagram export
impl: UC-006.s2  prio: could

The system shall export the links of a given item and its neighbours as a
Mermaid diagram with `requimd export mermaid <ID>`. It prints a Mermaid
flowchart in a Markdown code block, or writes it to a file with `-o`.

Neighbours are the items that link to or mention the item and the items it
links to or mentions. With `--depth <n>` (default 1) the system follows
references up to n steps away. The diagram shows:

- each item with its ID, title and status, styled by status, and the given
  item highlighted;
- every link between the shown items as an arrow labelled with the link
  name, and every mention as a dashed arrow;
- a use case as a box containing the steps that shown links point to.

For a step reference, the diagram starts at its use case. Exit codes are
those of FR-305: 1 if the ID does not exist or is declared more than once,
2 if the argument is no item ID or step reference.

## FR-405 Publishing target
impl: UC-006.s3  prio: could  status: deprecated

The system shall publish the generated site to a Confluence space as
read-only pages.

Deprecated on 2026-10-12 by the product owner: not needed. Stakeholders
receive PDFs made by a separate Markdown-to-PDF converter, which is not part
of requimd.
