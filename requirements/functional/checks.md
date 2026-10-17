# Checks

Every check has a check ID used in output and configuration. The default
severity is given by `sev`; FR-117 allows overriding it.

## FR-100 Duplicate IDs
impl: UC-003.s3  prio: must  check: duplicate-id  sev: error

If two items declare the same ID, the system shall report a diagnostic at
each declaration. References to a duplicated ID shall resolve to no item and
produce no further diagnostics, so the duplicate is the only finding. Links
from the duplicated items themselves are still evaluated.

## FR-101 Dangling references
impl: UC-001.s2a, UC-003.s3  prio: must  check: dangling-ref  sev: error

If a link or mention refers to an item or step that does not exist, the
system shall report a diagnostic at the reference. For mentions, the default
severity is warning. A severity configured for `dangling-ref` (FR-117)
applies to links and mentions alike.

## FR-102 Unknown metadata key
impl: UC-003.s3  prio: must  check: unknown-key  sev: warning

If a line directly after an item heading starts with an identifier and a
colon that is not a configured key for the item type, the system shall
report a diagnostic that names the allowed keys. Body field keys of the
item type (FR-005) are not reported; such a line is a body field.

## FR-103 Invalid value
impl: UC-003.s3  prio: must  check: invalid-value  sev: error

If a metadata value is not in the configured value set of its key, the
system shall report a diagnostic listing the allowed values.

## FR-104 Link target type
impl: UC-003.s3  prio: must  check: link-target  sev: error

If a link points to an item or step of a type that is not allowed for the
link type, the system shall report a diagnostic.

## FR-105 Required keys
impl: UC-003.s3  prio: must  check: missing-key  sev: error

If an item lacks a key that is required for its type, the system shall
report a diagnostic at the heading. A requirement written as `a|b` is met
if at least one of the keys is present.

## FR-106 Uncovered steps
impl: UC-003.s3  prio: must  check: uncovered-step  sev: warning

If a use case step has no incoming `impl` link, the system shall report a
diagnostic at the step.

## FR-107 Duplicate step labels
impl: UC-003.s3  prio: must  check: duplicate-step  sev: error

If two steps within one use case have the same label, the system shall
report a diagnostic at each of them.

## FR-108 Refinement cycles
impl: UC-003.s3  prio: must  check: ref-cycle  sev: error

If `ref` links form a cycle, the system shall report a diagnostic at every
link that is part of the cycle.

## FR-109 Status consistency
impl: UC-003.s3  prio: must  check: status-link  sev: warning

If an item with status `approved` links to an item with status `draft` or
`deprecated`, the system shall report a diagnostic at the link.

## FR-110 Weak words
impl: UC-001.s3a  prio: should  check: weak-word  sev: warning

If the title or text of a functional or quality requirement contains a
configured weak word, the system shall report a diagnostic at the word that
states the reason configured for that word.

Matching rules:

- Whole words only, case-insensitive: `fast` matches "Fast" but not
  "breakfast".
- An entry is either one word or a phrase, such as `as needed`; a phrase
  matches as a unit, with any whitespace between its words.
- No wildcards or stemming. Word forms are separate entries, such as `quick`
  and `quickly`.
- Ignored: code spans, code blocks, item IDs and text in double quotes,
  straight or typographic (for example UI labels such as "Quick order").
- The text is the item body without the metadata block.

The list is configured in `requimd.toml` under `[checks.weak-word]` as
`words`, each entry with the word `w` and the reason `why`. Configured words
extend the built-in list; a built-in word is switched off by the `allow`
list (FR-119). The built-in list contains at least `fast`, `quick`,
`quickly`, `easy`, `user-friendly`, `intuitive`, `flexible`, `robust`,
`appropriate`, `adequate`, `several`, `etc.`, `and/or`, `as needed` and `if
possible`, each with a reason.

```toml
[checks.weak-word]
words = [
  { w = "fast", why = "not measurable; state a time limit" },
  { w = "etc.", why = "incomplete list; name all items" },
]
allow = ["Fast Track"]
```

## FR-111 Missing "shall"
impl: UC-001.s3  prio: should  check: no-shall  sev: info

If the text of a functional requirement does not contain the word "shall",
the system shall report a diagnostic at the heading.

## FR-112 EARS patterns
impl: UC-001.s3  prio: could  check: ears  sev: info

The system shall detect the EARS pattern of a functional requirement
(ubiquitous, event-driven "When", state-driven "While", unwanted behaviour
"If … then", optional feature "Where") and report a diagnostic if the first
sentence matches none of them. The detected pattern shall be shown on hover and its keywords highlighted
as semantic tokens (FR-216).

## FR-113 User story pattern
impl: UC-001.s3  prio: should  check: story-pattern  sev: warning

If the text of a user story does not follow "As a …, I want … so that …",
the system shall report a diagnostic at the heading. If a story has no `ac`
body field, the system shall report a diagnostic as well.

## FR-114 Glossary aliases
impl: UC-001.s3  prio: should  check: glossary-alias  sev: warning

If the title or text of an item uses an alias of a glossary term, the system
shall report a diagnostic that names the preferred term. Matching follows the
rules of FR-110. An alias inside an occurrence of a glossary term is not
reported.

## FR-115 Remaining placeholders
impl: UC-003.s3  prio: must  check: placeholder  sev: warning

If an item has a placeholder ID, the system shall report a diagnostic at
the heading.

## FR-116 Suspect links
impl: UC-004.s1, UC-004.s2, UC-004.s3  prio: should  check: suspect-link  sev: warning

The system shall store, per link, a content hash of the target as of the
last review in the lock file (ADR-0007). If the current hash of the target
differs, the system shall report a diagnostic at the link. For a step
target, the hash covers the step text; for an item, the title, the metadata
except `status`, and the body. HTML comments, the inline markers of FR-119
included, are not part of the hash, since readers do not see them
(ADR-0010). Runs of whitespace, including line breaks, count as one space,
so re-wrapping text does not make a link suspect.

Links without an entry in the lock file are recorded silently with the
current hash on their first check; entries of links that no longer exist
are removed. A check never changes the hash of an existing entry; only a
review does (FR-218, FR-307). Links from items without a unique ID
(placeholders, duplicates) are recorded once the item has one. Mentions are
not links and are never suspect.

## FR-117 Check configuration
impl: UC-003.s3  prio: must

The system shall allow setting the severity of every check by its check ID
in the configuration, including `off` to disable it. A check is configured
either by a severity string (`uncovered-step = "info"` in `[checks]`) or by a
table `[checks.<check-id>]` with the key `sev` and check-specific options,
such as `words` and `allow` for FR-110.

## FR-118 Modal verbs
impl: UC-001.s3  prio: should  check: modal-verb  sev: warning

If the title or text of a functional requirement contains a configured modal
verb other than "shall", the system shall report a diagnostic at the word
that suggests "shall". The default list is `should`, `may`, `can`, `could`,
`might` and `will`; configured `words` under `[checks.modal-verb]` extend
it. Matching follows the rules of FR-110. This check is separate from FR-110 and
is configured independently.

## FR-119 Allow lists and inline markers
impl: UC-001.s3a  prio: should

The system shall not report a weak-word, modal-verb or glossary-alias
diagnostic for an occurrence that is allowed:

- project-wide, if the word or phrase is in the `allow` list of that check
  in `requimd.toml`;
- for one line, if that line ends with a marker of the form
  `<!-- requimd: allow <word or phrase> -->`. A comma separates multiple
  entries.

Each entry of a marker that allows nothing on its line shall itself produce
a diagnostic (check `unused-allow`, severity warning), so stale markers are
found.

```markdown
The Fast Track page shall load the order form. <!-- requimd: allow fast -->
```
