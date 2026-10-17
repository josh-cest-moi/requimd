# Acceptance tests: file format

Tests for the file format and the parser (FR-001 to FR-014). They run on the
command line in a fresh copy of the base project; `requimd show` (FR-305)
and `requimd export json` (FR-402) make the parse result visible.

Decisions for the whole area:

- ADR-0002 Markdown dialect (accepted): plain CommonMark with conventions.
  An item is a heading that starts with an ID; metadata is a block of
  `key: value` pairs below it; references are bare IDs without brackets;
  steps are list items with stable labels; code is ignored. Rejected: YAML
  front matter (verbose, one item per file), wiki links (more typing),
  Pandoc attributes (raw text on GitHub), HTML comments for metadata
  (invisible to readers), AsciiDoc and reStructuredText (less familiar).
  Consequence: a word that looks like an ID but is no reference must be
  written as inline code.
- ADR-0003 Markdown parser and LSP library (accepted): pulldown-cmark.
  requimd builds its own item model from the parser's event stream and
  keeps positions as byte offsets until the LSP boundary.
- BR-005: identity, type and relationships come from file content only
  (tests in `rules.md`).

## Item declaration (FR-001)

- Gap: the spec does not say whether prefixes are case-sensitive
  (`fr-001`), nor whether a heading inside a block quote or list counts.
- Code: prefixes are case-sensitive, so `fr-914` is no item. A heading
  inside a block quote or a list item is an item, since the Markdown parser
  reports it as a heading. The first token must be the ID alone: a heading
  such as `## FR-001: Title`, or one that starts with inline code, is
  silently no item (TC-035). A heading with an ID and no title is an item
  with an empty title, without a diagnostic.
- Automated: `fr_001_heading_with_id_declares_item`,
  `fr_001_unconfigured_prefix_and_other_first_tokens_are_no_items`,
  `fr_001_any_heading_level_and_setext` (core parse.rs).

### TC-001 A heading that starts with an ID declares an item
ver: FR-001

Given a new file `a.md`:

```markdown
# FR-910 Level one
#### FR-911 Level four
```

When `requimd show FR-910` and `requimd show FR-911` run.

Then both items exist, with the titles "Level one" and "Level four".

### TC-002 A heading is no item if its first token is no ID of a configured prefix
ver: FR-001

Given a new file `a.md`:

```markdown
## About FR-912
## XY-913 Unknown prefix
## fr-914 Lower case
```

When `requimd show FR-912` and `requimd show FR-914` run and the output
of `requimd export json` is inspected.

Then none of the three headings is an item. For `fr-914` the spec is silent
(Gap): note what happens.

Code: `fr-914` is no item: prefixes are case-sensitive.

### TC-003 Setext headings and headings in block quotes
ver: FR-001

Given a new file `a.md`:

```markdown
FR-915 Setext heading
=====================

> ## FR-916 Quoted heading
```

When `requimd show FR-915` and `requimd show FR-916` run.

Then `FR-915` is an item, since a setext heading is a Markdown heading.
For `FR-916` the spec is silent (Gap): note what happens.

Code: `FR-916` is an item: the parser treats a heading in a block quote like
any other heading.

### TC-035 Characters after the ID in a heading
ver: FR-001, FR-120

Given a new file `a.md`:

```markdown
## FR-960: Colon after the ID
## `FR-961` In code
## FR-962
```

When `requimd check` and `requimd show` run for the three IDs.

Then by FR-001 `FR-960` and `FR-961` are no items (the first token is no
ID); `FR-962` is an item without title. `FR-960` gets a `format-slip`
warning at the ID (FR-120); `FR-961` none, since code is an example.

Code (before FR-120 is built): none of the three headings gets a
diagnostic.

## Item extent (FR-002)

- Code: as specified; a body ends at the next heading of the same or a
  higher level, or at the next item heading of any level.
- Automated: `fr_002_body_ends_at_same_or_higher_level_or_item_heading`,
  `fr_002_body_extends_to_end_of_file` (core parse.rs).

### TC-004 The body ends at the next heading of the same or a higher level
ver: FR-002, FR-008

Given a new file `a.md`:

```markdown
## FR-920 First
The system shall do one thing, see BR-901.

### Details
More text about FR-901.

## FR-921 Second
The system shall do another thing.
```

When `requimd show FR-920` and `requimd show FR-921` run.

Then `FR-920` mentions `BR-901` and `FR-901`: the lower-level heading
"Details" belongs to its body. `FR-921` has no references.

### TC-005 A nested item ends the body of the outer item
ver: FR-002

Given a new file `a.md`:

```markdown
## FR-922 Outer
The system shall do one thing, see BR-901.

### FR-923 Inner
The system shall do more, see FR-901.

### Notes
Text that mentions FR-902.

## FR-924 Next
The system shall do the rest.
```

When `requimd show` runs for `FR-922`, `FR-923` and `FR-902`.

Then `FR-922` mentions only `BR-901`, and `FR-923` only `FR-901`. The
section "Notes" ends `FR-923` (same level) and does not belong to `FR-922`
either, whose body ended at `FR-923`: the mention of `FR-902` belongs to no
item, and `requimd show FR-902` lists no mention from this file.

Validate: text after a nested item belongs to no item.

### TC-006 Text before the first item belongs to no item
ver: FR-002, FR-008

Given a new file `a.md`:

```markdown
# Chapter
This introduction refers to FR-999.

## FR-925 First item
The system shall work.
```

When `requimd check` runs.

Then no `dangling-ref` is reported for `FR-999`: mentions exist only in
item bodies.

## Metadata block (FR-003)

- Gap: a line with an unknown key directly after the heading is no
  metadata line by FR-003, but FR-102 reports its key as unknown. What
  happens to the known keys on that line and to the lines after it is
  open; see TC-108.
- Code: the block ends at the first blank line or at the first line that
  does not start with `key:`, so a text line directly below the metadata is
  body text. A key needs a space or a tab after its colon: `prio:must` is no
  key, and the line becomes text without a diagnostic (TC-036). Up to three
  spaces of indentation are allowed.
- Code: if the first line starts with an unknown key, the item has no
  metadata block; the unknown key is still reported (FR-102), and the other
  pairs on that line are ignored. Unknown keys on later lines of a block are
  reported, and the block continues.
- Automated: `fr_003_metadata_block_with_pairs`,
  `fr_003_single_space_does_not_split_pairs`,
  `fr_003_no_metadata_if_first_line_is_not_a_key`,
  `fr_003_metadata_ends_at_blank_line_or_text` (core parse.rs).

### TC-007 Lines directly below the heading are the metadata block
ver: FR-003

Given in `functional.md` below the heading of `FR-901`, replacing its
metadata line:

```markdown
impl: UC-901.s1  prio: must
status: review
```

and the same once more with a tab instead of the two spaces.

When `requimd show FR-901` runs.

Then `FR-901` has the link to `UC-901.s1`, priority `must` and status
`review`, in both variants.

### TC-008 A single space does not separate two pairs
ver: FR-003, FR-103

Given the metadata line of `FR-901` is (one space before `status`):

```markdown
impl: UC-901.s1  prio: must status: review
```

When `requimd check` runs.

Then `prio` has the value `must status: review`, reported as
`invalid-value`; the status of `FR-901` stays `draft`.

### TC-009 Without a key in the first line there is no metadata block
ver: FR-003

Given a new file `a.md`:

```markdown
## FR-930 Late metadata
The system shall work.
prio: high
```

When `requimd check` runs.

Then `prio: high` is body text: no `invalid-value` is reported, and the
item has no priority.

### TC-010 A blank line between heading and metadata
ver: FR-003, FR-120

Given a new file `a.md`:

```markdown
## FR-931 Blank line first

prio: high
```

When `requimd check` runs.

Then the item has no metadata block, and the line `prio: high` gets a
`format-slip` warning (FR-120).

Code (before FR-120 is built): nothing is reported.

### TC-011 The metadata block ends at the first blank line
ver: FR-003

Given a new file `a.md`:

```markdown
## FR-932 Two blocks
prio: must

status: wrong
```

When `requimd check` and `requimd show FR-932` run.

Then `status: wrong` is body text: no `invalid-value`, status `draft`.

### TC-036 A key needs a space after its colon
ver: FR-003, FR-120

Given the metadata line of `FR-901` is:

```markdown
impl:UC-901.s1  prio:must
```

When `requimd check` runs.

Then the line is no metadata, so `s1` of `UC-901` becomes uncovered, and
`impl` gets a `format-slip` warning at the key (FR-120).

Code: a key needs a space or a tab after the colon (TC-036 was a gap
before FR-120). Before FR-120 is built, nothing points at the line.

## Multiple values (FR-004)

- Gap: only link keys can have several values. There is no setting that
  allows several values for another key, so the first sentence of FR-004
  has no effect today (`prio: must, should`). Empty entries such as
  `impl: UC-901.s1,,` are not specified.
- Code: only link values are split at commas; other values never are, so
  `prio: must, should` is one invalid value. Empty entries such as
  `impl: UC-901.s1,,` are skipped without a diagnostic.
- Automated: `fr_004_link_values_split_at_commas`,
  `fr_004_invalid_link_value_has_no_target` (core parse.rs).

### TC-012 Link values are split at commas and trimmed
ver: FR-004

Given the metadata line of `FR-901` is:

```markdown
impl: UC-901.s1 ,UC-901.s2,  UC-901.s2a
```

When `requimd show FR-901` and `requimd check` run.

Then `FR-901` has three `impl` links; nothing is reported.

## Body fields (FR-005)

- Gap: body field values appear in no output. The JSON export (FR-402)
  lists items, steps, links and mentions, not body fields. Apart from the
  `ac` field of stories (FR-113) and highlighting (FR-216), extracting them
  has no visible effect.
- Code: the JSON export lists body fields with their values (`fields`,
  FR-402), which closes the gap above for that output. A body field is any
  body line after the metadata that starts with its key; a list belongs to
  the value only if it starts on the very next line. The `required` keys of
  a type may name body fields (FR-105).
- Automated: `fr_005_body_fields_with_following_list`,
  `fr_005_body_field_rest_of_line`,
  `fr_005_body_field_directly_after_heading_is_not_metadata`,
  `fr_005_body_fields_only_for_their_type` (core parse.rs).

### TC-013 Body fields of the item type are recognised
ver: FR-005, FR-113

Given `US-901` in `other.md` as it is, then with `ac:` renamed to
`criteria:`.

When `requimd check` runs each time.

Then the first run reports nothing; the second reports a `story-pattern`
diagnostic for the missing `ac` field. The list below `ac:` belongs to its
value.

### TC-014 A body field key of another type is plain text
ver: FR-005

Given the text of `FR-901` gets a line `ac: The cart opens.`

When `requimd check` runs.

Then nothing new is reported: `ac` is no body field of functional
requirements, and the line is not directly after the heading (FR-102).

## Use case steps (FR-006)

- Gap: an extension such as `s2a` "extends" `s2`, but the spec gives that
  relation no visible effect (symbols, coverage, matrix).
- Code: the extension relation has no effect in the code either; symbols,
  coverage and matrix treat every step alike. The colon must follow the
  label directly (`- s1 :` is no step); the text may follow without a space.
- Automated: `fr_006_steps_in_use_cases`, `fr_006_steps_only_in_use_cases`
  (core parse.rs).

### TC-015 List items that start with a label are steps
ver: FR-006

Given a new file `a.md`:

```markdown
## UC-910 Steps
- s1: First.
- s2: Second.
- s2a: Extension of s2.
- s2a1: Extension of s2a.
- s10: Tenth.
- Note: not a step.
- S3: upper case, not a step.
- s4 without colon, not a step.
```

When `requimd show UC-910` and `requimd check` run.

Then `s1`, `s2`, `s2a`, `s2a1` and `s10` are steps (each gets an
`uncovered-step` warning); the last three list items are not.

### TC-016 The label, not the position, identifies a step
ver: FR-006, FR-007

Given a new file `a.md`:

```markdown
## UC-911 Order
1. s2: Shown first.
2. s1: Shown second.
```

When `requimd show UC-911.s1` runs.

Then it prints "Shown second.": ordered lists count, and the label is
independent of the position.

### TC-017 Steps exist only in use cases
ver: FR-006

Given a new file `a.md`, and in `FR-901` a link `ref: FR-940.s1`:

```markdown
## FR-940 Not a use case
- s1: Looks like a step.
```

When `requimd check` runs.

Then the reference to `FR-940.s1` is a `dangling-ref` error.

## Step references (FR-007)

- Automated: `fr_007_step_reference` (core parse.rs);
  `fr_007_resolves_items_and_steps` (core project.rs).

### TC-018 A step reference resolves to the step
ver: FR-007

Given the base project.

When `requimd show UC-901.s2` runs, and in Neovim go to definition on
`UC-901.s2` in `FR-902`.

Then the step text "System shows the total." and `FR-902` as implementing
item are printed; the editor jumps to the step line in `usecases.md`.

## Mentions in text (FR-008)

- Gap: whether IDs in metadata values that are not links (`src: FR-901` in
  a rule) are mentions. FR-002 makes the metadata block part of the body,
  FR-110 excludes it from the "text", FR-008 does not say.
- Code: IDs in metadata values that are not links are no mentions; the
  metadata block is skipped. IDs inside HTML comments are mentions, since
  only code is skipped, so `<!-- see FR-999 -->` gives a `dangling-ref`
  warning. A letter, digit, hyphen or underscore directly before or after an
  ID makes it part of a longer word (`a-FR-001` is none). IDs in lower-level
  headings within an item's body are mentions; the item's own title is not
  searched.
- Automated: `fr_008_mentions_in_body_fields_and_steps`,
  `fr_008_only_whole_words`, `fr_008_metadata_is_not_mentioned` (core
  parse.rs).

### TC-019 IDs in text, body fields and steps are mentions
ver: FR-008

Given a new file `a.md`:

```markdown
## UC-912 Mentions
pre: Requires FR-901.

- s1: Customer reads BR-901.

## FR-941 Text
The system shall follow BR-901 (see FR-902, FR-903.) and UC-901.s2.
```

When `requimd show UC-912` and `requimd show FR-941` run.

Then `UC-912` mentions `FR-901` and `BR-901`; `FR-941` mentions `BR-901`,
`FR-902`, `FR-903` and the step `UC-901.s2`. Brackets, commas and a full
stop after an ID do not prevent a mention.

### TC-020 Parts of words and titles are no mentions
ver: FR-008

Given a new file `a.md`:

```markdown
## FR-942 Refines FR-901
The system shall use XFR-901 and FR-901x.
```

When `requimd show FR-942` runs.

Then `FR-942` has no references: an ID inside a longer word is no
whole-word token, and the title is not part of the body.

Validate: an ID in a title is no reference.

## Code is ignored (FR-009)

- Code: HTML comments are no code: they are searched for mentions, while the
  language checks ignore them (FR-110).
- Automated: `fr_009_code_is_ignored`,
  `fr_009_indented_code_after_heading_is_no_metadata` (core parse.rs).

### TC-021 Code blocks and inline code hide items and references
ver: FR-009

Given a new file `a.md`:

````markdown
## FR-943 Example
The system shall accept FR-999 as an example.

```markdown
## FR-944 Not an item
impl: UC-999.s1
```

    ## FR-945 Indented code, not an item
````

When `requimd check` and `requimd show FR-944` run.

Then only `FR-943` is an item; neither `FR-944` nor `FR-945` exists, and
nothing is reported for `FR-999` or `UC-999.s1`.

## Placeholder IDs (FR-010)

- Code: confirms Q5 for placeholders: `ref: FR-?` is a `dangling-ref` error
  with the message `placeholder FR-? cannot be a link target`.
- Automated: `fr_010_placeholders_take_part_in_checks_but_not_duplicates`
  (checks tests); `fr_010_placeholder_id` (core parse.rs).

### TC-022 Placeholder items take part in the checks except duplicate IDs
ver: FR-010, FR-115

Given a new file `a.md`:

```markdown
## FR-? First new
prio: high

## FR-? Second new
```

When `requimd check` runs.

Then each heading gets a `placeholder` warning, `prio: high` an
`invalid-value` error, and there is no `duplicate-id`.

### TC-023 A placeholder cannot be a link target
ver: FR-010, FR-101

Given `FR-901` gets a second metadata line `ref: FR-?`, and its text the
words `see FR-?`.

When `requimd check` runs.

Then the link is a `dangling-ref` error (Q5: a link value that is no valid
ID is a `dangling-ref` error, to confirm). The text `FR-?` is no mention
and reports nothing.

## Glossary (FR-011)

- Q7 (to confirm): the glossary file belongs to the project even if the
  include patterns do not match it. If `[project] glossary` names a
  missing file, `requimd check` stops with exit code 2, while the language
  server works without a glossary.
- Gap: "every heading" makes a title heading such as `# Glossary` a term,
  as in this repository's own glossary.
- Gap: FR-014 names no default glossary path, so a project without
  `[project] glossary` has no glossary. FR-303 creates "an empty glossary
  file" without saying where.
- Code: every heading of any level is a term, also a title such as
  `# Glossary`; a unit test expects this. The `alias` line must directly
  follow the heading. A definition runs to the next heading of any level.
  `requimd init` writes `glossary = "glossary.md"` and creates that file;
  without the setting there is no glossary.
- Code: confirms Q7: a missing glossary file stops `requimd check` with exit
  code 2; the language server ignores it.
- Automated: `fr_011_glossary_terms_with_aliases` (core parse.rs).

### TC-024 Headings of the glossary file are terms, `alias` lists aliases
ver: FR-011, FR-114, FR-205

Given the base project; `FR-901` gets the sentence "The basket shows all
goods."

When `requimd check` runs, and in Neovim hover on "cart" in `FR-901`.

Then "basket" is reported as `glossary-alias` naming "Cart"; the hover
shows the definition of Cart.

### TC-025 Glossary outside the include patterns and missing glossary
ver: FR-011, FR-013

Given in `requimd.toml` under `[project]` first the line below, then
instead `glossary = "missing.md"`:

```toml
include = ["functional.md", "usecases.md", "other.md"]
```

When `requimd check` runs each time; with the missing glossary also open
`functional.md` in Neovim.

Then the first run still knows the glossary (TC-024 reports the alias)
(Q7). The second ends with a configuration error and exit code 2; the
language server still works, without glossary (Q7).

### TC-026 A title heading in the glossary file
ver: FR-011

Given `# Glossary` as first line of `glossary.md`, and the word "glossary"
in the text of `FR-901`.

When in Neovim hover on "glossary" in `FR-901`.

Then by the spec "Glossary" is a term (every heading), so the hover shows
its (empty) definition. Validate: probably not wanted (Gap).

Code: "Glossary" is a term; the hover shows only its name, since the next
heading follows directly.

## Tolerant parsing (FR-012)

- Gap: FR-012 asks for "a diagnostic for that item" but names no check ID
  or severity, and does not say which content counts as malformed.
- NFR-009 tests robustness in the editor (`quality.md`).
- Code (deviation): no diagnostic is recorded for malformed content. The
  parser never fails; whatever it does not recognise stays plain text, and
  only the regular checks report something (TC-027). Files that are not
  valid UTF-8 are read with replacement characters, without a message.
- Decision D2 (2026-10-17): FR-012 keeps only robustness; the slips that
  turn content into text silently get a diagnostic of FR-120 (TC-010,
  TC-028, TC-035, TC-036, TC-700).
- Automated: `fr_012_crlf_line_endings`, `fr_012_survives_arbitrary_input`
  (core parse.rs).

### TC-027 Malformed content does not stop the check
ver: FR-012

Given a new file `a.md`:

```markdown
## FR-
impl: ,,  prio:
## FR-950
:::: ]] [[ <!--
```

When `requimd check` runs.

Then the check completes; the items of the base project are checked as
before. Note which diagnostics appear for `a.md`.

Code: expect only a `no-shall` info for `FR-950`, an item with an empty
title. `## FR-` and the other lines are plain text and get no diagnostic.

### TC-028 An unclosed code fence
ver: FR-012, FR-009, FR-120

Given a new file `a.md`:

````markdown
## FR-951 Before
The system shall work.

```text
unclosed fence
## FR-952 After
````

When `requimd show FR-952` runs.

Then `FR-952` is no item: in CommonMark an unclosed fence runs to the end
of the file. The opening fence gets a `format-slip` warning (FR-120).

Code (before FR-120 is built): no diagnostic.

## Configuration discovery (FR-013)

- Q5 (to confirm): unknown check IDs and unknown keys in `requimd.toml`
  are configuration errors (exit code 2).
- Gap: with the default include pattern (all Markdown files below the
  root), a nested project with its own `requimd.toml` is part of the outer
  project too. `docs/vscode.md` says a file belongs to the nearest
  `requimd.toml`, the spec does not.
- Code: configuration errors (exit code 2) are: invalid TOML; every unknown
  section, key or option (confirms Q5); a prefix that is not letters and
  digits, or that two types use; a `to` entry that names no configured type;
  a `required` key that is no key of its type; an invalid include pattern;
  `digits = 0`; an `[lsp]` length of 0; an unknown check ID or severity. The
  check ID `ears` is accepted, though FR-112 is not built.
- Code: the file walk skips only `.git`. With the default include pattern,
  every Markdown file below the root belongs to the project: nested projects
  (TC-032), but also folders such as `node_modules`; `.gitignore` is not
  honoured.
- Automated: `fr_013_reads_all_sections`,
  `fr_013_invalid_configs_are_errors`,
  `fr_013_finds_root_in_parent_directory` (core config.rs);
  `fr_013_load_uses_include_patterns` (core project.rs).

### TC-029 The project root is found from a subdirectory
ver: FR-013, FR-300

Given an empty subdirectory `sub` in the base project.

When `requimd check` runs inside `sub`.

Then the result equals a run in the project root.

### TC-030 No configuration or an invalid one is a configuration error
ver: FR-013, FR-300

Given first an empty folder outside any project; then the base project with
`[project` (no closing bracket) in `requimd.toml`; then instead a line
`no-such-check = "off"` under `[checks]`; then instead `foo = 1` under
`[project]`.

When `requimd check` runs each time.

Then every run prints the reason and ends with exit code 2; the last two
per Q5 (to confirm).

### TC-031 Every part of the configuration is read
ver: FR-013

Given in `requimd.toml`:

```toml
[types.risk]
prefix = "RSK"
meta = ["impact"]

[links.mit]
name = "mitigates"
to = ["risk"]

[values]
impact = ["low", "high"]
```

and a new file `a.md`:

```markdown
## RSK-901 Supplier fails
impact: medium

## FR-960 Second supplier
mit: RSK-901, FR-901

The system shall keep a second supplier.
```

When `requimd check` runs.

Then `RSK-901` is an item; `impact: medium` is an `invalid-value` error
listing `low` and `high`; `mit` to `FR-901` is a `link-target` error, `mit`
to `RSK-901` is accepted.

### TC-032 Nested projects
ver: FR-013

Given a subdirectory `sub` with its own `requimd.toml` (copy of the base
project's) and a file `sub/a.md` with `## FR-901 Same ID`.

When `requimd check` runs in `sub` and in the project root.

Then in `sub` only the inner project is checked. In the root, the spec
does not say whether `sub/a.md` belongs to the outer project, where it
would be a `duplicate-id` (Gap): note what happens.

Code: in the root, `sub/a.md` belongs to the outer project too, so both
declarations of `FR-901` get `duplicate-id`.

## Configuration defaults (FR-014)

- Log 2026-10-11: built-in defaults are the types, links, fields and
  values of the root `requimd.toml` without `check` and `sev`; project
  configuration is merged with the defaults per entry.
- Q5 (to confirm): because of the per-entry merge, default item types can
  be replaced but not removed.
- Gap: FR-014 defines the defaults as this repository's `requimd.toml`.
  The repository cannot change its own types (for example body fields for
  its tests) without changing the defaults of every project. The table in
  FR-014 could be the normative definition alone.
- Code: the built-in defaults are the template that `requimd init` writes
  (`INIT_TEMPLATE` in `crates/requimd-core/src/config.rs`), without its
  glossary line. A unit test (`fr_014_defaults_match_reference_config`)
  keeps this repository's `requimd.toml` equal to them. So the reference in
  FR-014 holds, but the repository cannot change its own types without
  failing that test.
- Automated: `fr_014_defaults_contain_types_links_and_values`,
  `fr_014_defaults_match_reference_config`,
  `fr_014_project_entry_replaces_default_entry` (core config.rs);
  `fr_014_default_include_is_all_markdown_files` (core project.rs).

### TC-033 A configuration without sections uses the defaults
ver: FR-014

Given the base project, whose `requimd.toml` sets only the glossary path.

When `requimd check` runs and `NFR-901` loses its `meter` line, `FR-901`
gets `prio: high`, and a new file `a.md` contains `## FR-90 Two digits`.

Then the base project has the types UC, FR, NFR, BR, US and TC; the
missing `meter` is a `missing-key` error; `high` is an `invalid-value`
listing `must`, `should`, `could`, `wont`; `FR-90` is no item (three
digits); every item without `status` has status `draft` (`requimd show`).

### TC-034 A project entry replaces the default entry completely
ver: FR-014

Given in `requimd.toml`:

```toml
[types.quality]
prefix = "NFR"
meta = ["scale"]

[values]
prio = ["high", "low"]
```

When `requimd check` runs.

Then `NFR-901` reports `meter` and `must` as `unknown-key` and no
`missing-key` (the replaced entry has no required keys); `prio: must` on
the functional requirements is an `invalid-value`; `status` values are
unchanged; all other types still exist.
