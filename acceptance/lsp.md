# Acceptance tests: language server

Tests for the language server (FR-200 to FR-219), run in Neovim 0.12 with
the configuration of `docs/neovim.md`, in a fresh copy of the base project.
The VS Code check of the same features is TC-506 in `editors.md`.

Neovim keys used below (Neovim defaults, `:help lsp-defaults`): `K` hover,
`<C-]>` go to definition, `grr` references, `gri` implementation, `gra`
code action, `grn` rename, `gO` document symbols, `]d` and `[d` next and
previous diagnostic, `<C-w>d` diagnostic float, `<C-x><C-o>` completion.

Decisions for the whole area:

- ADR-0003 (accepted): lsp-server with lsp-types, a synchronous loop
  instead of an async framework; the work is CPU-bound, and debouncing and
  cancellation for NFR-001 stay under requimd's control. No async runtime
  in the dependency tree.
- NFR-007: only standard LSP features; nothing requires an editor plugin.
- Log 2026-10-11: completion triggers are `-` and `.`, in link values also
  space and comma; manual completion searches titles.
- Log 2026-10-11 and 2026-10-12: client file watching if the client
  offers it, else the server polls every 2 seconds; the server resolves
  paths to their real spelling and rescans the disk on file events that do
  not fit it (Neovim on Windows reports 8.3 short paths).

## Server start (FR-200, FR-308)

### TC-200 The server starts over stdio with `requimd lsp`
ver: FR-200, FR-308

Given the base project and Neovim configured as in `docs/neovim.md`.

When `functional.md` is opened and `:checkhealth vim.lsp` runs; and, in a
terminal, `requimd lsp` is started and stopped with Ctrl+C.

Then the health check lists the client `requimd` attached to the buffer.
In the terminal the server waits silently for input on stdin.

## Project scope (FR-201)

- Q6 (to confirm): language features work only in included files, also
  hover and go to definition, not only diagnostics and completion.

### TC-201 Only included files get language features
ver: FR-201

Given under `[project]` in `requimd.toml` the line below, a new file
`notes.md` with the text `See FR-999 and the basket.`, and the file
`README.md` of this repository (outside any project):

```toml
include = ["usecases.md", "functional.md", "other.md"]
```

When `notes.md` and then `README.md` are opened, and in `notes.md` hover,
go to definition and completion are tried.

Then `notes.md` shows no diagnostics and offers no completion; hover and
go to definition do nothing (Q6). In `README.md` no requimd client
attaches.

## Live diagnostics (FR-202)

- NFR-001 sets the latency limit (`quality.md`).

### TC-202 Diagnostics follow typing without saving
ver: FR-202

Given `functional.md` open.

When `impl: UC-901.s1` of `FR-901` is changed to `UC-901.s9` without
saving, and then back.

Then the `dangling-ref` error appears while typing and disappears again.

### TC-203 A change updates the diagnostics of other open documents
ver: FR-202

Given `usecases.md` and `functional.md` side by side (`:vsplit`).

When in `usecases.md` the label `s2` is changed to `s5` without saving,
and then undone with `u`.

Then `functional.md` immediately shows `dangling-ref` at `impl: UC-901.s2`
and `usecases.md` an `uncovered-step` at `s5`; after the undo both are
gone.

## Reference completion (FR-203)

- Gap: whether placeholder and duplicate items appear as candidates.

### TC-204 Completion in link values offers allowed targets with details
ver: FR-203

Given at the end of `functional.md` a new heading `## FR-? Payment`.

When on the next line `impl: UC-` is typed; then `UC-901.`; then after
`impl: UC-901.s1, ` a space; and on another line `ver: ` with
`<C-x><C-o>`.

Then after `UC-` the menu offers `UC-901` and its steps, each with title
or step text as detail, and no functional requirements; after
`UC-901.` only the steps of `UC-901`; after comma and space it opens
again. After `ver: ` only functional and quality requirements and stories
are offered.

### TC-205 Completion in body text, found by ID or title
ver: FR-203

Given the text of `FR-903`.

When typing `see BR-`; then the word `total` followed by `<C-x><C-o>`.

Then `BR-901` is offered after `BR-`; for `total` the menu finds `FR-902`
"Show the total" and `NFR-901` "Time to show the total" by their titles.

## Key and value completion (FR-204)

- Gap: whether body field keys (`pre`, `post`, `trigger` for use cases)
  are offered as keys, and which keys are offered on the first line below
  a heading that has no metadata block yet.

### TC-206 Keys at the start of a metadata line, values after a key
ver: FR-204

Given a new metadata line below the heading of `FR-901`; then the same
below `UC-901`.

When `<C-x><C-o>` at the start of the line; then after `prio: `; then
after `actor: ` in the use case.

Then the keys of the item type are offered (`status`, `prio`, `impl`,
`ref`, `ver`, and `actor` for the use case); after `prio: ` the values
`must`, `should`, `could`, `wont`; after `actor: ` nothing, since it has
no value set.

## Hover (FR-205)

- Log 2026-10-11: `[lsp] hover-length`, default 500 characters.
- Gap: FR-205 speaks of "a reference" but includes glossary terms, which
  are none. Hover on the ID in an item's own heading is not specified.

### TC-207 Hover on a reference shows the target
ver: FR-205

Given `functional.md` and `other.md` open.

When `K` on `UC-901.s2` in `FR-902`; on `BR-901` in the text of `FR-902`;
on `FR-902` in `TC-901`.

Then the step text "System shows the total."; for `BR-901` ID, type, status,
title and text; the same for `FR-902`.

### TC-208 Hover text is cut at `hover-length`
ver: FR-205

Given in `requimd.toml`:

```toml
[lsp]
hover-length = 20
```

When `K` on `FR-902` in `TC-901` after saving `requimd.toml`.

Then the text of `FR-902` is cut after 20 characters.

### TC-209 Hover on a glossary term or alias shows the definition
ver: FR-205, FR-011

Given `FR-901` gets the sentence "The basket opens."

When `K` on "cart" in the title of `FR-901`; then on "basket".

Then both show the definition of Cart.

## Go to definition (FR-206)

### TC-210 Go to definition on links, step references and mentions
ver: FR-206

Given `functional.md` open; then a new file `a.md` with `## FR-902 Copy`.

When `<C-]>` on `UC-901.s2` in `FR-902`; on the mention `BR-901`; then,
with the duplicate in place, on `FR-902` in `TC-901`.

Then the cursor jumps to the step line in `usecases.md` and to the heading
of `BR-901`. A duplicated ID resolves to no item (FR-100): no jump.

## Find references (FR-207)

- Q6 (to confirm): find references on an item also lists references to
  its steps.

### TC-211 Find references on an item heading
ver: FR-207

Given `other.md` open.

When `grr` on the ID in the heading of `BR-901`; then on the heading of
`FR-902` in `functional.md`; then on `FR-902` in `TC-901`.

Then `BR-901`: the mention in `FR-902`. `FR-902`: the links from `NFR-901`
and `TC-901`, from the heading and from the reference alike.

### TC-212 Find references on a step and on a use case
ver: FR-207

Given `usecases.md` open.

When `grr` on the label `s2`; then on the ID in the heading of `UC-901`.

Then `s2`: the link from `FR-902`. `UC-901`: the links to all its steps
from `FR-901`, `FR-902` and `FR-903` (Q6, to confirm).

## Go to implementation (FR-208)

- Log 2026-10-11: go to implementation on a use case includes links to
  its steps.

### TC-213 Implementations of a use case and of a step
ver: FR-208

Given `usecases.md` open, and `FR-901` with an additional `ref: UC-901.s2`.

When `gri` on the heading of `UC-901`; then on the label `s2`.

Then `UC-901`: `FR-901`, `FR-902`, `FR-903`. `s2`: only `FR-902`; the
`ref` link of `FR-901` is no implementation.

## Inlay hints (FR-209)

- Log 2026-10-12: `[lsp] hint-length`, default 40.

### TC-214 Hints after references in metadata blocks
ver: FR-209

Given `functional.md` and `other.md` open; then `hint-length = 10` under
`[lsp]`, saved.

When looking at the metadata lines and at the text of `FR-902`.

Then after `impl: UC-901.s2` the hint "System shows the total."; after
`ref: FR-902` in `NFR-901` "Show the total"; no hint after the mention of
`BR-901` in body text. With `hint-length = 10` the hints are cut to 10
characters.

## Document symbols (FR-210)

- Gap: "nested accordingly" is clear for steps below their use case, not
  for an item heading below another item heading (TC-005).

### TC-215 Items of a document with steps nested below their use case
ver: FR-210

Given `functional.md`, then `usecases.md`.

When `gO` in each.

Then `FR-901` to `FR-903` in the first; `UC-901` with `s1`, `s2`, `s2a`
nested below it in the second.

## Workspace symbols (FR-211)

### TC-216 All items of the project, searchable by ID and title
ver: FR-211

Given any file of the base project open.

When `:lua vim.lsp.buf.workspace_symbol('total')`; then with `UC-9`.

Then `FR-902` and `NFR-901` for "total", from both files; `UC-901` for
`UC-9`.

## Assign ID (FR-212)

- ADR-0007: `[ids]` in the lock file keeps the highest number per prefix
  ever seen, also after the item is deleted.
- Log 2026-10-11: until M4, assign ID used only IDs in the project; the
  lock file joined in M4.
- Gap: the first ID of a prefix without any item or lock entry is not
  specified (presumably `-001`).

### TC-217 A placeholder gets the next free ID
ver: FR-212

Given at the end of `functional.md` the heading `## FR-? Payment`.

When `gra` on the heading and "Assign ID" is chosen.

Then the heading becomes `## FR-904 Payment` (highest so far `FR-903`),
zero-padded to three digits, and the `placeholder` warning disappears.

### TC-218 A deleted ID is not assigned again
ver: FR-212, FR-218

Given TC-217 done and saved, so the lock knows `FR-904`.

When the item `FR-904` is deleted and saved, a new `## FR-? Refund` is
written and "Assign ID" is chosen.

Then it becomes `FR-905`, not `FR-904` (BR-002).

### TC-219 A prefix without items
ver: FR-212

Given a new file `a.md` with the heading `## US-? Second story`; and in
`requimd.toml` a new type `[types.risk]` with `prefix = "RSK"` and the
heading `## RSK-? First risk`.

When "Assign ID" on both.

Then `US-902`; for the prefix without any item the spec is silent (Gap):
note what happens.

## Create stub (FR-213)

- Log 2026-10-12: `[lsp] stub-file`; a dangling step reference adds the
  step to its use case.
- Gap: the first sentence says a stub gets "a placeholder ID and a link",
  the second "a stub with that ID" for a dangling reference. Which ID the
  stub gets, and what its link is, is contradictory.
- Gap: the stub's title is not specified (for an uncovered step, the step
  text would suggest itself), nor what happens if `stub-file` names a
  missing file.
- BR-003 says a new step gets the next free number; the step stub uses
  the label of the reference instead.

### TC-220 Stub for a dangling reference
ver: FR-213

Given `FR-901` with an additional `ref: FR-990`.

When `gra` on `FR-990` and "Create stub" is chosen.

Then a stub item for `FR-990` is added at the end of `functional.md` and
the `dangling-ref` disappears. Note which ID the stub gets (Gap).

### TC-221 Stub for an uncovered step
ver: FR-213

Given `UC-901` with a new step `- s3: Customer pays.`

When `gra` on `s3` and the "create requirement" action is chosen.

Then a functional requirement with a placeholder ID and `impl: UC-901.s3`
is added at the end of the current file; the `uncovered-step` disappears.

### TC-222 A dangling step reference adds the step
ver: FR-213

Given `FR-901` with `impl: UC-901.s7` instead of `UC-901.s1`.

When `gra` on `UC-901.s7` and the "add step" action is chosen.

Then `- s7:` is appended to the steps of `UC-901` in `usecases.md`; the
`dangling-ref` is gone and `s1` is uncovered.

### TC-223 Stubs go to `stub-file` if it is set
ver: FR-213

Given in `requimd.toml` under `[lsp]` the line `stub-file = "inbox.md"`,
saved; then TC-220 and TC-221 again.

When the stubs are created.

Then both land in `inbox.md` (Gap if it does not exist yet: note what
happens).

## Rename (FR-214)

- Log 2026-10-12: rename keeps the prefix; changing the type is a manual
  edit. The server rewrites the lock entries only once the documents show
  the rename, so a cancelled rename changes nothing.
- Gap: whether rename works from a reference as well as from the heading.
  Step labels cannot be renamed (BR-003 keeps them stable).

### TC-224 Rename updates declaration, references and lock entries
ver: FR-214

Given the base project checked once (`requimd check`, so the lock exists),
`functional.md` open.

When `grn` on the ID in the heading of `FR-902`, new ID `FR-920`; all files
saved; then `requimd check`.

Then the heading, the links in `NFR-901` and `TC-901` and all lock entries
use `FR-920`; `requimd check` reports no `suspect-link`.

### TC-225 Renaming a use case updates its step references
ver: FR-214

Given `usecases.md` open.

When `grn` on `UC-901`, new ID `UC-920`, all files saved.

Then every `UC-901.s…` in `functional.md` reads `UC-920.s…`; nothing is
reported.

### TC-226 Invalid renames are refused
ver: FR-214

Given `functional.md` open.

When `grn` on `FR-901` with `FR-902` (exists), `FR-9` (too few digits) and
`NFR-950` (other prefix).

Then each rename is refused with a message; no file changes.

### TC-227 Rename from a reference and a cancelled rename
ver: FR-214

Given `other.md` open.

When `grn` on `FR-902` in `TC-901`; then `grn` on the heading of `FR-901`
cancelled with Escape.

Then for the reference the spec is silent (Gap): note what happens. The
cancelled rename changes neither files nor `requimd.lock`.

## Code lens on steps (FR-215)

- Log 2026-10-12: the lens title lists the IDs; running it needs a
  4-line handler in the Neovim configuration (`docs/neovim.md`), else the
  server opens the first implementing item.

### TC-228 A lens per step with number and IDs of implementing items
ver: FR-215

Given `usecases.md` open, `UC-901` with a new step `- s3: Customer pays.`,
lenses shown (`vim.lsp.codelens.refresh()` if needed).

When the lens of `s2` is run with `:lua vim.lsp.codelens.run()`.

Then each step shows a lens with count and IDs, such as one implementing
item `FR-902` for `s2`, and `s3` shows none (zero). Running it lists
`FR-902` in the quickfix list (with the handler) or opens `FR-902`
(without).

## Semantic tokens (FR-216)

- Log 2026-10-12: FR-216 was moved ahead of FR-403 and FR-404; it adds step
  labels, body fields, glossary terms, modal verbs and aliases; EARS
  keywords move to FR-112. Weak words, modal verbs and aliases are
  highlighted only where a diagnostic is reported.
- Q8 (to confirm): glossary terms match like weak words (not in code,
  quotes or IDs) and the glossary file itself gets no tokens; where tokens
  overlap, keys, IDs and step labels win over findings, and findings over
  terms; unknown metadata keys are not highlighted as keys.
- `docs/neovim.md` lists the token types and the default highlight groups.

### TC-229 IDs, references, keys, step labels and terms are highlighted
ver: FR-216

Given `functional.md` and `usecases.md` open.

When `:Inspect` on the heading ID of `FR-902`, on `UC-901.s2`, on `impl`,
on the label `s2`, on `pre`, and on "cart".

Then each has its token type as in `docs/neovim.md`: the class type with
modifier declaration for the heading ID, the class type for the reference,
property for the keys, enumMember for the label, namespace for "cart".

### TC-230 Findings are highlighted only where they are reported
ver: FR-216, FR-119

Given `FR-901` with the text "The cart should open fast, see the basket.";
then with the marker `<!-- requimd: allow fast -->` at the end of that
line; then `modal-verb = "off"` under `[checks]`.

When `:Inspect` on "should", "fast" and "basket" after each change.

Then first all three are highlighted (keyword with the modifiers
weakWord, modalVerb, glossaryAlias); with the marker "fast" is plain;
with the check off "should" is plain.

### TC-231 Overlaps, unknown keys and the glossary file
ver: FR-216

Given `FR-901` with the text "The shopping cart opens." and the metadata
line `owner: Anna`; `glossary.md` open.

When `:Inspect` on "cart" inside "shopping cart", on `owner`, and on
"Cart" in `glossary.md`.

Then "shopping cart" is one alias token, "cart" inside it is no term
token; `owner` has no key token; the glossary file has no tokens (Q8, to
confirm).

## External file changes (FR-217)

- Gap: FR-217 covers included files. Changes to `requimd.toml` or
  `requimd.lock` outside the editor (for example by `git pull`) are not
  specified.

### TC-232 Changes outside the editor update graph and diagnostics
ver: FR-217

Given `functional.md` open in Neovim.

When in another editor or a terminal: `s2` in `usecases.md` becomes `s5`;
then a new file `b.md` with `## FR-950 Outside` and `impl: UC-901.s1` is
created; then `b.md` is deleted.

Then within about two seconds Neovim shows `dangling-ref` at the link to
`UC-901.s2` without reloading; `gri` on `UC-901` lists `FR-950` while
`b.md` exists and no longer after it is deleted.

### TC-233 Changes to configuration and lock file outside the editor
ver: FR-217

Given `functional.md` open; `FR-901` with the word "fast".

When `allow = ["fast"]` under `[checks.weak-word]` is added in another
editor; then a lock entry hash is edited by hand in another editor.

Then the spec is silent on both (Gap): note whether the warning
disappears and whether a suspect link appears.

## Mark as reviewed (FR-218)

### TC-234 Marking a suspect link as reviewed
ver: FR-218

Given the base project checked once; the text of `FR-902` changed and
saved, so `ver: FR-902` in `TC-901` is suspect.

When `gra` on that link and "Mark as reviewed" is chosen.

Then the warning disappears and `git diff requimd.lock` (or a look at the
file) shows a new hash for `"TC-901 ver FR-902"`; the link from `NFR-901`
stays suspect.

### TC-235 New links are recorded on save, not while typing
ver: FR-218

Given the base project checked once; `FR-903` gets a second line
`ref: FR-901`.

When the lock file is inspected before and after saving `functional.md`.

Then before saving it has no entry `"FR-903 ref FR-901"`, after saving it
has one.

## Language check code actions (FR-219)

- Log 2026-10-12: "Add to weak words" writes an empty reason and opens
  `requimd.toml` at the entry.
- Changes keep the comments and formatting of `requimd.toml` (toml_edit,
  ADR-0003).
- Gap: whether "Add to weak words" is offered in item types where the
  weak-word check does not apply (use cases, stories).

### TC-236 Allow here adds or extends the inline marker
ver: FR-219

Given `FR-901` with the text "The cart should open fast."

When `gra` on "fast" and "allow on this line"; then the same on "should".

Then the line ends with `<!-- requimd: allow fast -->`, then with
`<!-- requimd: allow fast, should -->`; both diagnostics are gone.

### TC-237 Allow in project writes the allow list and keeps comments
ver: FR-219

Given `FR-901` with the text "The basket opens."; `functional.md` and
`other.md` open; a comment line `# my comment` in `requimd.toml`.

When `gra` on "basket" and "allow in project".

Then `requimd.toml` has `allow = ["basket"]` under
`[checks.glossary-alias]`, the comment is still there; the diagnostic
disappears in all open documents without saving them.

### TC-238 Add to weak words, for a word or a selected phrase
ver: FR-219

Given `FR-901` with the text "The checkout opens at once."

When `gra` on "checkout" and "add to weak words"; a reason is typed into
`why` and `requimd.toml` saved; then the phrase "at once" is selected in
visual mode and added the same way.

Then the entry `{ w = "checkout", why = "" }` is added and
`requimd.toml` opens at it; after saving, "checkout" is reported with the
typed reason. The phrase is added and reported as one finding.
