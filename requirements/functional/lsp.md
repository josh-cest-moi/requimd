# Language server

## FR-200 Server start
impl: UC-007.s3  prio: must

The system shall provide a language server speaking the Language Server
Protocol over stdio, started with `requimd lsp`.

## FR-201 Project scope
impl: UC-007.s3  prio: must

The language server shall process Markdown files that match the include
patterns of the project. For other Markdown files it shall provide no
diagnostics and no completion.

## FR-202 Live diagnostics
impl: UC-001.s3  prio: must

When a document is opened or changed, the language server shall publish
the diagnostics of all checks for that document, and update diagnostics of
other open documents affected by the change.

## FR-203 Reference completion
impl: UC-001.s2  prio: must

The language server shall offer item IDs and step references as completion
candidates in link values and in body text. In link values, candidates are
limited to allowed target types (FR-104). Each candidate shows the title or
step text as detail. Typing part of the ID or the title shall find it.

## FR-204 Key and value completion
impl: UC-001.s2  prio: must

The language server shall offer configured metadata keys at the start of a
metadata line, and configured values after keys that have a value set.

## FR-205 Hover
impl: UC-002.s3  prio: must

When the cursor is on a reference, the language server shall show on hover
the ID, type, status and title of the target, and its text up to a
configurable length. For a step reference it shows the step text; for a
glossary term, its definition. The length is set in characters by
`hover-length` in the `[lsp]` section of `requimd.toml`, default 500.

## FR-206 Go to definition
impl: UC-002.s4  prio: must

The language server shall resolve go-to-definition on a reference to the
declaration of the item or to the step.

## FR-207 Find references
impl: UC-002.s5  prio: must

The language server shall list all links and mentions that refer to the
item or step under the cursor.

## FR-208 Go to implementation
impl: UC-002.s2  prio: must

The language server shall answer go-to-implementation on a use case or step
with all items that link to it with `impl`. For a use case, this includes
items that link to any of its steps.

## FR-209 Inlay hints
impl: UC-002.s3  prio: should

The language server shall offer an inlay hint after each reference in a
metadata block, showing the title of the target item or the text of the
target step, shortened to `hint-length` characters (`[lsp]` section,
default 40).

## FR-210 Document symbols
impl: UC-002.s1  prio: must

The language server shall provide the items of a document and the steps of
its use cases as document symbols, nested accordingly.

## FR-211 Workspace symbols
impl: UC-002.s4  prio: must

The language server shall provide all items of the project as workspace
symbols, searchable by ID and title.

## FR-212 Assign ID
impl: UC-001.s4  prio: must

The language server shall offer a code action on a placeholder ID that
replaces it with the next free ID of its prefix. The next free ID is one
higher than the highest ID of that prefix in the project and in the lock
file, so deleted IDs are not reused (BR-002). The lock file keeps the
highest number per prefix ever seen, also after the item is deleted
(ADR-0007).

## FR-213 Create stub
impl: UC-001.s2a  prio: should

The language server shall offer code actions that create a stub item with a
placeholder ID and a link: on a dangling reference, a stub with that ID; on
an uncovered step, a functional requirement stub with `impl` to the step.
On a dangling step reference, the missing step is appended to the steps of
its use case instead. The stub is inserted in the file set by `stub-file`
(`[lsp]` section) or, if unset, at the end of the current file.

## FR-214 Rename
impl: UC-005.s1, UC-005.s2, UC-005.s1a  prio: should

The language server shall support renaming an item ID, updating the
declaration, all references across all files and the entries of that ID in
the lock file, so a rename makes no link suspect. It shall refuse the rename
if the new ID is invalid (BR-001), already exists or has a different
prefix.

## FR-215 Code lens on steps
impl: UC-002.s1  prio: should

The language server shall show a code lens on each use case step with the
number and IDs of implementing items; executing it lists them as in FR-208.

## FR-216 Semantic tokens
impl: UC-001.s3  prio: could

The language server shall provide semantic tokens for highlighting in the
editor:

- item IDs in headings, and references (links and mentions);
- metadata keys, body field keys and step labels;
- glossary terms in item titles and text, matched by the rules of FR-110;
- every occurrence reported by the weak-word, modal-verb or glossary-alias
  check (FR-110, FR-114, FR-118). Allowed occurrences (FR-119) and
  occurrences of a disabled check are not highlighted, so highlighting and
  diagnostics agree.

EARS keywords are highlighted once FR-112 is implemented.

## FR-217 External file changes
impl: UC-003.s2  prio: must

When included files are created, changed or deleted outside the editor, for
example by a git checkout, the language server shall update its item graph
and diagnostics.

## FR-218 Mark as reviewed
impl: UC-004.s4, UC-004.s5  prio: should

The language server shall offer a code action on a suspect link that
records the current target hash in the lock file (FR-116). When a document
is saved, the language server shall record its new links in the lock file,
as a check does.

## FR-219 Language check code actions
impl: UC-001.s3a  prio: should

The language server shall offer these code actions:

- **Add to weak words**, on any word or selected phrase without a
  diagnostic: adds it to `words` of `[checks.weak-word]` with an empty
  reason and opens `requimd.toml` at that entry for the reason.
- **Allow here**, on a weak-word, modal-verb or glossary-alias diagnostic:
  adds or extends the inline marker of FR-119 on that line.
- **Allow in project**, on the same diagnostics: adds the word to the
  `allow` list of that check.

Changes to `requimd.toml` shall keep its comments and formatting. After the
change, the server shall reload the configuration and update the
diagnostics of all open documents.
