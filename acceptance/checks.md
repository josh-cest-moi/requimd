# Acceptance tests: checks

Tests for the checks (FR-100 to FR-119). They run with `requimd check` in a
fresh copy of the base project; every diagnostic shows the check ID in
brackets (FR-300). The same diagnostics appear in the editor (FR-202).

Decisions for the whole area:

- Every check has a check ID and a default severity (`sev`); FR-117 allows
  overriding it per project.
- ADR-0005 Design of the language checks (accepted), for FR-110, FR-114,
  FR-118, FR-119 and FR-219:
  1. Three stages: a word list with a reason per entry (FR-110), simple
     pattern rules starting with modal verbs (FR-118), and an on-demand AI
     review that stays on the "later" list because it is not deterministic
     enough for CI.
  2. Modal verbs are a check of their own, with their own message and
     severity.
  3. Word forms are explicit entries, no wildcards or stemming: predictable
     beats short (`quick*` would match "quickstart").
  4. Titles are checked, because they appear in completion, hover and the
     outline.
  5. Silencing by a project-wide `allow` list per check and an inline HTML
     comment per line, which GitHub and GitLab do not show; stale markers
     are reported. Rejected: an `allow:` metadata key per item (visible to
     readers, too coarse).
  6. The lists live in `requimd.toml`, so editor and CI behave alike.
- Q7 (to confirm): the language checks are bound to fixed type names:
  weak words to `functional` and `quality`, modal verbs and "no shall" to
  `functional`, the story pattern to `story`. A project that names its
  types differently loses these checks. (Technical open question 1 in
  `docs/status.md` names the same for `usecase`, `impl` and `ref`.)

## Duplicate IDs (FR-100)

- Log 2026-10-09: references to a duplicated ID resolve to no item; only
  the duplicate is reported.
- Q5 (to confirm): steps of a duplicated use case are not reported as
  uncovered.
- Code: the message names the other declarations, for example
  `ID FR-901 is declared more than once, also at functional.md:3`; the
  diagnostic sits on the ID. Confirms Q5: steps of a duplicated use case are
  not reported as uncovered.
- Automated: `fr_100_duplicate_ids_reported_at_each_declaration`,
  `fr_100_links_from_duplicated_items_are_evaluated`,
  `fr_100_to_fr_119_broken_example_matches_expected` (checks tests);
  `fr_100_duplicate_ids_resolve_to_no_item` (core project.rs).

### TC-100 A duplicate ID is reported at every declaration
ver: FR-100

Given a new file `a.md` with `## FR-901 Copy`, and a new file `b.md` with
`## FR-901 Second copy`.

When `requimd check` runs.

Then all three declarations of `FR-901` get a `duplicate-id` error.

### TC-101 References to a duplicated ID give no further findings
ver: FR-100

Given a new file `a.md`:

```markdown
## FR-902 Copy
impl: UC-901.s9

The system shall do it twice.
```

When `requimd check` runs.

Then both declarations of `FR-902` get `duplicate-id`. The links from
`NFR-901` and `TC-901` to `FR-902` get no diagnostic of any kind (no
`dangling-ref`, `link-target`, `status-link` or `suspect-link`). The link
`impl: UC-901.s9` of the copy is still evaluated: `dangling-ref`.

### TC-102 A duplicated use case reports no uncovered steps
ver: FR-100, FR-106

Given a new file `a.md`:

```markdown
## UC-901 Copy
- s5: Nobody implements this.
```

When `requimd check` runs.

Then `UC-901` gets `duplicate-id` twice and step `s5` no `uncovered-step`
(Q5, to confirm).

## Dangling references (FR-101)

- Log 2026-10-11: a severity configured for `dangling-ref` applies to
  links and mentions alike.
- Q5 (to confirm): a link value that is no valid ID (`impl: foo`, `FR-12`,
  `FR-?`) is a `dangling-ref` error.
- Code: confirms Q5. A link value that is no valid ID gets the message
  `"foo" is not a valid item ID or step reference`; findings on mentions
  start with `mention:`.
- Automated: `fr_101_dangling_links_and_mentions`,
  `fr_101_override_applies_to_links_and_mentions` (checks tests).

### TC-103 Missing link targets are errors, missing mentions warnings
ver: FR-101

Given `FR-901` gets a metadata line `ref: FR-999, UC-901.s9` and the
sentence `See FR-998.` (without backticks) in its text.

When `requimd check` runs.

Then both link values get a `dangling-ref` error at their position; the
mention gets a `dangling-ref` warning.

### TC-104 One severity override for links and mentions
ver: FR-101, FR-117

Given TC-103, plus `dangling-ref = "info"` under `[checks]`.

When `requimd check` runs.

Then all three findings have severity info; exit code 0.

### TC-105 Link values that are no IDs
ver: FR-101

Given `FR-901` gets a metadata line `ref: foo, FR-12, FR-?`.

When `requimd check` runs.

Then each value is a `dangling-ref` error (Q5, to confirm).

## Unknown metadata key (FR-102)

- Log 2026-10-11: body field keys directly after a heading are body
  fields, not unknown keys.
- Gap: "a line directly after an item heading" can mean only the first
  line or every line of the metadata block (TC-108).
- Code: every unknown key of the metadata block is reported, on any line and
  in any pair; on the first line it also means that there is no block
  (FR-003, TC-108).
- Automated: `fr_102_unknown_metadata_key_names_allowed_keys`,
  `fr_102_unknown_key_directly_after_heading`,
  `fr_102_body_field_keys_are_not_unknown` (checks tests);
  `fr_102_unknown_first_key_is_recorded_without_metadata_block`,
  `fr_102_unknown_key_in_later_pair` (core parse.rs).

### TC-106 An unknown key is reported with the allowed keys
ver: FR-102

Given `FR-901` gets `owner: Anna` as first line after its heading,
followed by its metadata line.

When `requimd check` runs.

Then `owner` gets an `unknown-key` warning that names the allowed keys of
functional requirements (`status`, `prio`, `impl`, `ref`, `ver`).

### TC-107 A key of another item type is unknown
ver: FR-102

Given `FR-901` gets `actor: Customer` on its metadata line.

When `requimd check` runs.

Then `actor` gets `unknown-key`: it is a key of use cases only.

### TC-108 An unknown key next to known keys
ver: FR-102, FR-003

Given the metadata of `FR-901` is first:

```markdown
owner: Anna  prio: high
```

and then:

```markdown
prio: high
owner: Anna
```

When `requimd check` runs each time.

Then `owner` gets `unknown-key` in the first variant. Whether `prio: high`
is read there (and reported as `invalid-value`), and whether `owner` on the
second line counts as "directly after the heading", is not specified
(Gap): note what happens.

Code: in the first variant `owner` is reported and `prio: high` is not read,
so there is no `invalid-value`. In the second variant `owner` on the second
line is reported too.

### TC-109 A body field key directly after the heading is no unknown key
ver: FR-102, FR-005

Given `UC-901` without its `actor` line, so `pre:` follows a blank line;
then with `pre: The customer is logged in.` moved directly below the
heading.

When `requimd check` runs each time.

Then nothing is reported either time.

## Invalid value (FR-103)

- Gap: whether values are case-sensitive (`status: Draft`).
- Code: values are compared case-sensitively, so `Draft` is an
  `invalid-value` error (TC-111).
- Automated: `fr_103_invalid_value_lists_allowed_values` (checks tests).

### TC-110 A value outside the value set is reported with the allowed values
ver: FR-103

Given `FR-901` has `prio: high`.

When `requimd check` runs.

Then `high` gets an `invalid-value` error listing `must`, `should`,
`could`, `wont`.

### TC-111 Case of values and keys without a value set
ver: FR-103

Given `FR-902` gets `status: Draft`, and `UC-901` has the line
`actor: Anyone at all`.

When `requimd check` runs.

Then `actor` accepts any value. For `Draft` the spec is silent (Gap): note
what happens.

Code: `Draft` is an `invalid-value` error: values are case-sensitive.

## Link target type (FR-104)

- Automated: `fr_104_link_target_type` (checks tests).

### TC-112 Links to targets of a type that is not allowed
ver: FR-104

Given `FR-901` gets the metadata lines below, one at a time:

```markdown
impl: BR-901
ver: UC-901
ver: UC-901.s1
ref: BR-901
impl: UC-901
```

When `requimd check` runs each time.

Then the first three are `link-target` errors (`impl` only to use cases
and steps, `ver` only to functional and quality requirements and stories);
`ref` to a rule and `impl` to a whole use case are accepted.

## Required keys (FR-105)

- Q5 (to confirm): a required key counts as present if `[defaults]` sets
  it.
- Gap: whether several missing keys give one diagnostic or one each.
- Code: one diagnostic per item lists all missing keys
  (`lacks required keys: meter, must or plan`). A key counts as present with
  an empty value (`meter:`), as a body field, or through `[defaults]`
  (confirms Q5).
- Automated: `fr_105_required_keys_with_alternatives` (checks tests).

### TC-113 Missing required keys are reported at the heading
ver: FR-105

Given `NFR-901` keeps only its `ref` and `scale` lines.

When `requimd check` runs.

Then `missing-key` errors at the heading of `NFR-901` name `meter` and
`must|plan`.

Code: one diagnostic, `NFR-901 lacks required keys: meter, must or plan`.

### TC-114 Either key of `a|b` is enough
ver: FR-105

Given `NFR-901` has `plan: 300` instead of `must: 500`.

When `requimd check` runs.

Then nothing is reported.

### TC-115 A default value satisfies a required key
ver: FR-105

Given `NFR-901` without its `meter` line, and in `requimd.toml`:

```toml
[defaults]
meter = "manual measurement"
```

When `requimd check` runs.

Then nothing is reported (Q5, to confirm).

## Uncovered steps (FR-106)

- Code: `impl` links from placeholders, duplicates and deprecated items
  cover a step; only an `impl` link to the step itself counts.
- Automated: `fr_106_uncovered_steps` (checks tests).

### TC-116 A step without an `impl` link is reported
ver: FR-106

Given `UC-901` gets a step `- s3: Customer pays.`; then a new item in
`functional.md`:

```markdown
## FR-904 Payment
impl: UC-901.s3

The system shall accept payment.
```

When `requimd check` runs after each change.

Then first `s3` gets an `uncovered-step` warning at the step; after the
second change it is gone.

### TC-117 Other references do not cover a step
ver: FR-106

Given step `s3` as in TC-116 and, one at a time, in `FR-901`: a link
`ref: UC-901.s3`; the sentence `See UC-901.s3.` (without backticks); a
second link `impl: UC-901` to the whole use case.

When `requimd check` runs each time.

Then `s3` stays uncovered every time.

Validate: `impl` to a use case as a whole covers none of its steps.

### TC-118 Deprecated implementations and extension steps
ver: FR-106

Given `FR-903` has `status: deprecated`.

When `requimd check` runs.

Then `s2a` is still covered: a deprecated item still has an `impl` link.
Extension steps such as `s2a` need coverage like any step.

Validate: both points; the spec makes no exception for either.

## Duplicate step labels (FR-107)

- Automated: `fr_107_duplicate_step_labels` (checks tests).

### TC-119 A label used twice in one use case
ver: FR-107

Given `UC-901` gets a second step `- s2: System shows the total again.`;
and a new file `a.md` with a use case `## UC-910 Other` and a step
`- s1: Other first step.`

When `requimd check` runs.

Then both `s2` of `UC-901` get a `duplicate-step` error; `s1` in two use
cases is no finding.

## Refinement cycles (FR-108)

- Gap: whether a link of an item to itself (`ref` from `FR-901` to
  `FR-901`) is a cycle.
- Code: cycles are found as strongly connected components of the `ref`
  links. A link of an item to itself is a cycle (TC-121); a `ref` to a step
  counts as a `ref` to its use case.
- Automated: `fr_108_ref_cycles_reported_at_every_link`,
  `fr_108_no_cycle_without_back_link` (checks tests).

### TC-120 A `ref` cycle is reported at every link of it
ver: FR-108

Given `FR-901` gets `ref: FR-902`, `FR-902` gets `ref: FR-903`, and
`FR-903` gets `ref: FR-901`.

When `requimd check` runs.

Then each of the three links gets a `ref-cycle` error.

### TC-121 Self references and other link types
ver: FR-108

Given first `FR-901` with `ref: FR-901`; then instead a loop of another
link type: `FR-902` gets `ver: FR-901` and `FR-901` gets `ver: FR-902`.

When `requimd check` runs each time.

Then the `ver` loop is no `ref-cycle`. For the self reference the spec is
silent (Gap): note what happens.

Code: the self reference is a `ref-cycle` error.

## Status consistency (FR-109)

- Gap: steps have no status. Whether an approved item with `impl` to a
  step of a draft use case gets a warning (using the use case's status) is
  not specified.
- Code: a step target has the status of its use case, so an approved item
  with `impl` to a step of a draft use case gets the warning (TC-123). An
  item without `status` counts with the default `draft`.
- Automated: `fr_109_approved_item_links_to_draft_or_deprecated` (checks
  tests).

### TC-122 An approved item that links to a draft or deprecated item
ver: FR-109

Given `NFR-901` has `status: approved` (it links to `FR-902` with `ref`);
then `FR-902` gets `status: review`; then `status: deprecated`.

When `requimd check` runs after each change.

Then first a `status-link` warning at `ref: FR-902`; with `review` none;
with `deprecated` the warning is back.

### TC-123 Other combinations give no warning
ver: FR-109

Given `NFR-901` has `status: review` (target `FR-902` draft); then `FR-901`
has `status: approved` and mentions `FR-903` (draft) in its text; then
`FR-901` (approved) links to `UC-901.s1` while `UC-901` is draft.

When `requimd check` runs after each change.

Then no warning for review to draft, none for the mention. For the step
link the spec is silent (Gap): note what happens.

Code: the step link gets a `status-link` warning, since a step has the
status of its use case (draft).

## Weak words (FR-110)

- Log 2026-10-09: weak words are checked in titles too; word forms are
  explicit entries.
- Log 2026-10-12: configured `words` extend the built-in list, `allow`
  switches words off. The built-in list has the 15 words of FR-110 plus
  simple, efficient, sufficient, normally and usually, each with a reason.
- Log 2026-10-12 (also Q7): title and body text are checked, not metadata;
  typographic quotes are ignored like straight ones.
- Gap: single quotes are not ignored, only double quotes. Apostrophes and
  single quotes are common in German and English text.
- Code: a hyphen or an underscore belongs to the word, so "fast-track" or
  "non-robust" do not match "fast" or "robust". German quotes „…“ are
  ignored like “…”; guillemets and single quotes are not. Every HTML comment
  is ignored. The text includes steps, body fields and lower-level headings
  of the item. A configured word without reason reports
  `weak word (no reason given)`.
- Automated: `fr_110_weak_words_in_title_and_text`,
  `fr_110_message_states_reason`,
  `fr_110_ignores_code_quotes_ids_and_metadata`,
  `fr_110_configured_words_extend_defaults` (checks tests);
  `fr_110_configured_words_extend_built_in_list` (core config.rs);
  `fr_110_whole_words_case_insensitive`,
  `fr_110_phrases_match_across_whitespace`, `fr_110_punctuation_in_entries`,
  `fr_110_quotes_and_comments_are_ignored` (core words.rs).

### TC-124 Every built-in weak word is reported with its reason
ver: FR-110

Given a new file `a.md`:

```markdown
## FR-960 Quick search
The system shall be fast, quick, easy, user-friendly, intuitive, flexible,
robust, appropriate, adequate and several etc. It shall work and/or stop as
needed if possible. It shall quickly be simple, efficient, sufficient,
normally and usually.
```

When `requimd check` runs.

Then every listed word or phrase gets a `weak-word` warning with a reason,
"Quick" in the title as well.

### TC-125 Whole words only, case-insensitive
ver: FR-110

Given `FR-901` gets the sentence "It shall be FAST at breakfast and be
steadfast, and Fast."

When `requimd check` runs.

Then "FAST" and "Fast" are reported, "breakfast" and "steadfast" are not.

### TC-126 Phrases match across spaces and line breaks
ver: FR-110

Given `FR-901` gets the text below, with the line break inside the phrase:

```markdown
The system shall open the cart as
needed and if    possible.
```

When `requimd check` runs.

Then "as needed" and "if possible" are each reported once.

### TC-127 Code, IDs and quoted text are ignored
ver: FR-110

Given `FR-901` gets the text below:

```markdown
The system shall show the button "Fast order" and “Quick order”,
the code `fast` and the item FR-902. It shall not be 'easy'.
```

When `requimd check` runs.

Then only "easy" in single quotes is reported (Gap: single quotes).

### TC-128 Metadata and other item types are not checked
ver: FR-110

Given `NFR-901` gets `meter: fast benchmark with 100 carts` and the text
"The total shall appear fast."; `UC-901`, `BR-901`, `US-901` and `TC-901`
each get the word "fast" in their text.

When `requimd check` runs.

Then only "fast" in the text of `NFR-901` is reported.

### TC-129 Own words extend the list, `allow` switches words off
ver: FR-110, FR-117

Given `FR-901` gets the text "The cart opens fast and seamless." and in
`requimd.toml`:

```toml
[checks.weak-word]
words = [{ w = "seamless", why = "marketing word; describe the behaviour" }]
```

then additionally `allow = ["fast"]` in that table.

When `requimd check` runs after each change.

Then first "fast" (built-in reason) and "seamless" (the configured reason)
are reported; with the `allow` entry only "seamless".

## Missing "shall" (FR-111)

- Q7 (to confirm): "shall" counts anywhere in the text except code, also
  in quotes.
- Gap: whether the title counts as text, and whether "Shall" counts
  (case).
- Code: "shall" is matched as a whole word, ignoring case, in the text after
  the metadata block; the title does not count; quotes and HTML comments
  count, code does not (TC-131).
- Automated: `fr_111_functional_requirement_without_shall` (checks tests).

### TC-130 A functional requirement without "shall"
ver: FR-111

Given `FR-901` has the text "The cart opens on request."; `NFR-901` and
`BR-901` have no "shall" anyway.

When `requimd check` runs.

Then `FR-901` gets a `no-shall` info at its heading; `NFR-901` and
`BR-901` get none.

### TC-131 Where "shall" counts
ver: FR-111

Given `FR-901` with each of these texts in turn: `The cart "shall" open.`;
the word shall only as inline code; the title "The cart shall open" with
the text "It opens."; the text "Shall the cart open? Yes."

When `requimd check` runs each time.

Then the quoted "shall" counts (Q7); inline code does not (`no-shall`).
Title and capital "Shall" are not specified (Gap): note what happens.

Code: the title "The cart shall open" does not count, so `no-shall` is
reported; "Shall" with a capital counts.

## EARS patterns (FR-112)

- Automated: none.

`could`, not implemented; on the "Later" list of `docs/roadmap.md`
together with EARS keyword highlighting (FR-216). No tests yet.

## User story pattern (FR-113)

- Q7 (to confirm): the first paragraph must start with "As a" or "As an"
  and contain "I want" followed by "so that", ignoring case and line
  breaks; the comma is not required.
- Code: the field name `ac` is fixed in the code, like the type names of Q7.
  The first paragraph ends at the first blank line; "I want" must be
  followed by a space and "so that" surrounded by spaces.
- Automated: `fr_113_user_story_pattern_and_acceptance_criteria` (checks
  tests).

### TC-132 A story that does not follow the pattern
ver: FR-113

Given `US-901` has the text "I want to reorder a past order."

When `requimd check` runs.

Then `US-901` gets a `story-pattern` warning at its heading.

### TC-133 Accepted variants of the pattern
ver: FR-113

Given `US-901` with each of these first paragraphs in turn:

```markdown
As an editor I want a preview so that I see the layout.

as a customer, i want
to reorder SO THAT I save time.
```

and then a text whose first paragraph is "Reordering saves time." and
whose second paragraph follows the pattern.

When `requimd check` runs each time.

Then the first two give no warning; the third gets `story-pattern` (Q7:
first paragraph).

### TC-134 A story without `ac` field
ver: FR-113

Given `US-901` without its `ac` field; then also with the text "Reorder
quickly."

When `requimd check` runs after each change.

Then first one `story-pattern` warning for the missing `ac`; then two.

## Glossary aliases (FR-114)

- Log 2026-10-09: aliases are checked in titles too.
- Log 2026-10-12: an alias inside a term is not reported.
- Unlike FR-110, the check applies to every item type.
- Code: an alias inside an occurrence of a longer alias is not reported
  either.
- Automated: `fr_114_glossary_aliases_in_any_item`,
  `fr_114_alias_inside_a_term_is_not_reported` (checks tests).

### TC-135 An alias is reported with the preferred term
ver: FR-114

Given the title of `FR-901` is "Open the basket"; `UC-901`, `US-901` and
`TC-901` each get the word "basket" in their text.

When `requimd check` runs.

Then every "basket" gets a `glossary-alias` warning naming "Cart", in all
item types.

### TC-136 Aliases match like weak words
ver: FR-114

Given `FR-901` gets the text below:

```markdown
The system shall show the Basket, the shopping
cart, the "basket" and `basket`, and all baskets.
```

When `requimd check` runs.

Then "Basket" and "shopping cart" (across the line break) are reported;
the quoted and the code occurrence are not.

Validate: "baskets" is not reported, since word forms are explicit
entries (ADR-0005); the plural needs its own alias.

### TC-137 An alias inside a term is not reported
ver: FR-114

Given `FR-902` gets the sentence "The total includes the basket fee."

When `requimd check` runs.

Then nothing is reported: "basket" is part of the term "Basket fee".

## Remaining placeholders (FR-115)

- Automated: `fr_115_placeholder_reported_at_heading` (checks tests).

### TC-138 A placeholder is reported until it gets an ID
ver: FR-115

Given a new item `## FR-? Payment` with the text "The system shall accept
payment." in `functional.md`.

When `requimd check` runs; then `requimd fmt`; then `requimd check` again.

Then first a `placeholder` warning at the heading; after `fmt` the item has
`FR-904` and the warning is gone.

## Suspect links (FR-116)

- ADR-0007 Lock file (accepted): the review state lives in `requimd.lock`
  in the project root, in TOML, committed with the specification. `[ids]`
  holds the highest number per prefix ever seen and never decreases.
  `[links]` has one line per link: the key is source, link key and target
  as written (`"FR-101 impl UC-012.s1"`), the value the target's hash at
  the last review, 16 hex digits. Entries are sorted, so merge conflicts
  stay small. The hash is a 64-bit FNV-1a hash implemented in requimd, so
  its values do not depend on platform or Rust version. Considered and
  rejected: JSON (larger merge conflicts), storing the reviewed text
  (file grows to the size of all linked text; a diff from git history is
  on the "Later" list), only explicit commands writing the lock (a new
  link would never become suspect before its first review).
- Log 2026-10-12: the hash ignores `status` and whitespace; `check` adds
  and removes entries but never changes a hash; `--locked` for CI; the
  language server records on save; no entries for items without a unique
  ID; an unreadable lock stops `check` with exit code 2.
- Gap: what happens to the lock entry of a link whose target was deleted
  (the link still exists, now dangling).
- Validate: a link changed by hand (source ID edited without rename) loses
  its review state silently; the new link is recorded as reviewed.
- Decision D7 (2026-10-17), ADR-0010 (proposed): HTML comments leave the
  hash (TC-161).
- Code (before ADR-0010 is built): the hash of an item covers everything
  after the metadata block up to the end of the item: text, steps,
  lower-level headings, code blocks and HTML comments. So adding an inline
  allow marker to a target ("allow here",
  FR-219) makes its incoming links suspect.
- Code: the lock entry of a link whose target was deleted is kept while the
  link exists (gap above); if the target comes back with other content, the
  link is suspect. Links with an invalid target are not recorded. The
  message reads
  `FR-902 changed since this link was last reviewed; check TC-901 and mark the link as reviewed`.
- Automated: `fr_116_suspect_link_after_target_change` (checks tests);
  `fr_116_check_records_links_and_reports_changed_targets` (cli tests);
  `fr_116_hash_ignores_whitespace_and_line_endings`,
  `fr_116_new_links_recorded_existing_hashes_kept_vanished_removed`,
  `fr_116_no_entries_for_items_without_unique_id`,
  `fr_116_item_hash_covers_title_metadata_body_but_not_status`,
  `fr_116_lock_file_roundtrip_is_sorted` (core lock.rs);
  `fr_116_fr_218_suspect_links_over_protocol` (lsp tests); also checked in
  Neovim (`scripts/nvim-check.lua`).

### TC-139 New links are recorded silently
ver: FR-116

Given a fresh copy of the base project without `requimd.lock`.

When `requimd check` runs.

Then nothing is reported. `requimd.lock` exists with one sorted line per
link (`"FR-901 impl UC-901.s1"`, …, `"TC-901 ver FR-902"`), each with a
16-digit hex hash, and an `[ids]` table with the highest number per
prefix.

### TC-140 A changed target makes its incoming links suspect
ver: FR-116

Given TC-139 done; the text of `FR-902` changes to "The system shall show
the total with VAT."

When `requimd check` runs twice.

Then both runs report a `suspect-link` warning at `ref: FR-902` in
`NFR-901` and at `ver: FR-902` in `TC-901`; the second run does not clear
them, and the hashes in the lock do not change.

### TC-141 Status and whitespace do not count
ver: FR-116

Given TC-139 done; `FR-902` gets `status: review`; its text is wrapped
differently and gets double spaces between words; `functional.md` is saved
with Windows line endings.

When `requimd check` runs.

Then no `suspect-link` is reported.

### TC-142 Title and other metadata count
ver: FR-116

Given TC-139 done; one at a time, each followed by `requimd review --all`:
the title of `FR-902` changes; `FR-902` gets `prio: should`; `FR-902` gets
a new link `ref: FR-901`.

When `requimd check` runs after each change.

Then each change makes the links to `FR-902` suspect.

Validate: a new outgoing link of the target makes its incoming links
suspect, because the hash covers all metadata except `status`.

### TC-143 A changed step makes the links to that step suspect
ver: FR-116

Given TC-139 done; `FR-901` gets a second link `ref: UC-901`; the text of
step `s2` changes to "System shows the total and the VAT."

When `requimd check` runs.

Then `impl: UC-901.s2` in `FR-902` and `ref: UC-901` in `FR-901` are
suspect (the use case's body changed); the links to `UC-901.s1` and
`UC-901.s2a` are not.

### TC-144 Removed links, mentions, placeholders and duplicates
ver: FR-116

Given TC-139 done; `TC-901` loses its `ver` line; the text of `BR-901`
changes (`FR-902` only mentions it); a new item `## FR-? New` with the link
`ref: FR-901`; a new file `a.md` with `## FR-903 Copy` and `ref: FR-901`.

When `requimd check` runs.

Then the lock no longer has the entry of `TC-901`; no `suspect-link` for
the mention; no entries for the links of the placeholder and of the
duplicate. After `requimd fmt` (ID for the placeholder) and the next
`requimd check`, the link of the new item is recorded.

### TC-161 HTML comments do not make links suspect
ver: FR-116, FR-119

Given the base project checked once; `FR-902` gets a new last line
`<!-- Author note: check the rounding with finance. -->`, and the text of
step `s2` of `UC-901` gets ` <!-- todo -->` at its end; then `FR-902` gets
the word "fast" with "allow here" (FR-219) on that line.

When `requimd check` runs after each change.

Then no `suspect-link` is reported: neither at `ref: FR-902` and
`ver: FR-902`, nor at `impl: UC-901.s2`.

Code (before ADR-0010 is built): the links to `FR-902` and to `UC-901.s2`
become suspect.

## Check configuration (FR-117)

- Gap: an invalid severity value (`uncovered-step = "loud"`) is not
  specified; Q5 makes unknown check IDs a configuration error.
- Code: an invalid severity is a configuration error with exit code 2
  (TC-146).
- Automated: `fr_117_severity_override_and_off` (checks tests);
  `fr_117_invalid_check_settings_are_config_errors` (core config.rs).

### TC-145 Severity as string or as table, and `off`
ver: FR-117

Given `UC-901` gets an uncovered step `- s3: Customer pays.` and `FR-901`
the word "fast"; then in `requimd.toml`, one at a time:

```toml
[checks]
uncovered-step = "error"
```

```toml
[checks.weak-word]
sev = "off"
```

When `requimd check` runs after each change.

Then first `uncovered-step` is an error and the exit code is 1; with the
table, "fast" is no longer reported.

### TC-146 Every check can be configured, including `unused-allow`
ver: FR-117, FR-119

Given `FR-901` gets a line ending with `<!-- requimd: allow slow -->` and
`unused-allow = "off"` under `[checks]`; then instead the line
`uncovered-step = "loud"`.

When `requimd check` runs each time.

Then the unused marker is not reported. For the invalid severity the spec
is silent (Gap): note what happens.

Code: `uncovered-step = "loud"` stops the check with a configuration error,
exit code 2.

## Modal verbs (FR-118)

- Log 2026-10-09: modal verbs are a separate check (ADR-0005).
- Gap: FR-118 says configured `words` extend the list, but not their form;
  weak words use `{ w = …, why = … }`.
- Code: `words` entries are strings or `{ w, why }` tables; a reason is
  ignored for modal verbs. The message reads
  `"should": use "shall" for a requirement`.
- Automated: `fr_118_modal_verbs_in_functional_requirements` (checks tests).

### TC-147 Modal verbs other than "shall" are reported
ver: FR-118

Given a new file `a.md`:

```markdown
## FR-961 The cart should open
The system may show, can show, could show, might show and will show the
total.
```

When `requimd check` runs.

Then "should" in the title and the five verbs in the text get a
`modal-verb` warning suggesting "shall".

### TC-148 Only functional requirements; separate configuration
ver: FR-118

Given `NFR-901` and `US-901` get the word "should"; `FR-901` gets "The
cart should open fast."; then `[checks.weak-word]` with `sev = "off"`.

When `requimd check` runs after each change.

Then only `FR-901` is reported, for "should" and "fast"; with weak words
off, "should" is still reported.

Validate: "May" as a month ("due in May") is reported as well; the allow
list or marker silences it.

## Allow lists and inline markers (FR-119)

- Log 2026-10-09: false positives are silenced by an inline HTML comment
  or a project allow list.
- Log 2026-10-12: `unused-allow` is reported per marker entry.
- Gap: the `allow` example of FR-110 (`"Fast Track"`) implies that an
  allowed phrase silences the weak word inside it, while FR-119 speaks of
  "the word or phrase" that is in the list. Whether allow entries are
  case-insensitive is not specified.
- Code: allow-list and marker entries match like weak words, ignoring case
  and as phrases. An occurrence is allowed if it lies inside an occurrence
  of an allowed entry, so "Fast Track" silences "Fast" within it (TC-153). A
  marker covers findings that start on its line, and its entry must be found
  on that line before the marker; a phrase split over two lines is reported,
  and its entry is `unused-allow` (TC-151). Markers inside code are ignored.
- Automated: `fr_119_project_allow_list`,
  `fr_119_inline_markers_and_unused_entries`,
  `fr_119_markers_in_code_are_ignored`,
  `fr_100_to_fr_119_broken_example_matches_expected` (checks tests);
  `fr_119_invalid_language_options_are_config_errors` (core config.rs);
  `fr_119_inline_markers` (core words.rs).

### TC-149 The project allow list of each check
ver: FR-119

Given `FR-901` gets the text "The cart should open fast, see the basket.";
then in `requimd.toml`:

```toml
[checks.weak-word]
allow = ["fast"]

[checks.modal-verb]
allow = ["should"]

[checks.glossary-alias]
allow = ["basket"]
```

When `requimd check` runs before and after.

Then before, three warnings; after, none.

### TC-150 An inline marker allows words on its line only
ver: FR-119

Given `FR-901` gets the text:

```markdown
The cart should open fast. <!-- requimd: allow fast, should -->
It should be fast.
```

When `requimd check` runs.

Then the first line reports nothing; the second reports "should" and
"fast".

### TC-151 A marker must end the line
ver: FR-119

Given `FR-901` gets the text:

```markdown
The cart opens fast <!-- requimd: allow fast --> and in time.
The cart opens as
needed. <!-- requimd: allow as needed -->
```

When `requimd check` runs.

Then "fast" is reported (the marker does not end the line). For the
phrase split over two lines, the spec is silent: note what happens.

Validate: a marker covers one physical line, not a sentence.

Code: the phrase split over two lines is reported, and the marker entry "as
needed" gets `unused-allow`.

### TC-152 Unused marker entries are reported
ver: FR-119

Given `FR-901` gets the line:

```markdown
The cart opens fast. <!-- requimd: allow fast, quick -->
```

When `requimd check` runs.

Then "fast" is silenced, and the entry "quick" gets an `unused-allow`
warning.

### TC-153 An allowed phrase that contains a weak word
ver: FR-119, FR-110

Given `allow = ["Fast Track"]` under `[checks.weak-word]` and `FR-901` with
the text "The Fast Track page shall open. The cart shall open fast."; then
the entry written `"fast track"`.

When `requimd check` runs each time.

Then "Fast" in "Fast Track" is not reported, the second "fast" is (as in
the example of FR-110). With the lower-case entry, the spec is silent
(Gap): note what happens.

Code: the lower-case entry works the same: allow entries ignore case.
