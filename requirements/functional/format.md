# File format and parsing

This file is the normative definition of the reqmd format. Examples in code
blocks are not items (FR-009).

## FR-001 Item declaration
impl: UC-001.s1, UC-003.s2  prio: must

The parser shall treat a Markdown heading of any level as an item
declaration if its first token is an item ID whose prefix is configured
(BR-001, FR-013). The remaining heading text is the item title.

```markdown
## FR-104 VAT per line item
```

## FR-002 Item extent
impl: UC-003.s2  prio: must

The body of an item shall extend from the line after its heading to the
next heading of the same or a higher level, or to the end of the file. A
lower-level heading belongs to the body, unless it is itself an item
declaration; then it starts a new item and ends the current body.

## FR-003 Metadata block
impl: UC-001.s2, UC-003.s2  prio: must

The parser shall read the lines directly following an item heading, up to
the first blank line, as the metadata block, provided each line consists of
`key: value` pairs with configured metadata or link keys. Pairs on one line
are separated by two or more spaces or a tab. If the first line after the
heading does not start with a configured key, the item has no metadata
block and the line belongs to the body.

```markdown
## FR-104 VAT per line item
impl: UC-012.s2  prio: must
status: review

When the cart is opened, ...
```

## FR-004 Multiple values
impl: UC-003.s2  prio: must

Where a metadata key allows multiple values, the parser shall split the
value at commas and trim surrounding whitespace. Link keys always allow
multiple values.

## FR-005 Body fields
impl: UC-003.s2  prio: must

The parser shall recognise a body line that starts with a body field key
configured for the item type, followed by a colon, as a body field. Its
value is the rest of the line plus a Markdown list that follows directly,
if any.

```markdown
pre: Customer is logged in.

ac:
- Given a past order, when I click "Reorder", then the items are in the cart.
- Unavailable items are listed with a hint.
```

## FR-006 Use case steps
impl: UC-003.s2  prio: must

Within items of type use case, the parser shall recognise list items whose
text starts with a step label followed by a colon as steps. A step label is
`s` followed by digits, optionally followed by lowercase letters and digits,
for example `s2`, `s2a` or `s2a1`. The label is independent of the list
position (BR-003). A label with letters is an extension of the step whose
label is its longest numeric-and-letter prefix, e.g. `s2a` extends `s2`.

```markdown
- s1: Customer opens the cart.
- s2: System shows the total incl. VAT.
- s2a: Cart is empty → system shows a hint, use case ends.
```

## FR-007 Step references
impl: UC-003.s2  prio: must

The parser shall resolve a reference of the form `<use case ID>.<step label>`
to the step with that label in that use case, for example `UC-012.s2`.

## FR-008 Mentions in text
impl: UC-003.s2  prio: must

The parser shall recognise every whole-word token in an item body, including
body fields and steps, that matches the ID pattern of a configured prefix,
optionally with a step suffix, as a mention. Mentions are references without
a link type.

## FR-009 Code is ignored
impl: UC-003.s2  prio: must

The parser shall ignore the content of fenced code blocks, indented code
blocks and inline code spans when detecting item declarations, metadata,
body fields, steps and references. This allows documentation to show
examples of the format.

## FR-010 Placeholder IDs
impl: UC-001.s1  prio: must

The parser shall treat a heading whose first token is a configured prefix
followed by `-?` as an item with a placeholder ID. Placeholder items take
part in all checks except the duplicate ID check, and they cannot be link
targets.

```markdown
## FR-? New requirement
```

## FR-011 Glossary
impl: UC-003.s2  prio: should

The parser shall treat every heading in the configured glossary file as a
glossary term; the heading text is the preferred term. A metadata line with
the key `alias` lists alternative spellings, separated by commas.

```markdown
## Cart
alias: basket, shopping cart

Items a customer has selected but not yet ordered.
```

## FR-012 Tolerant parsing
impl: UC-001.s3, UC-003.s2  prio: must

If an item contains malformed content, the parser shall record a diagnostic
for that item and continue with the remaining items and files. Incomplete
input while typing shall never stop parsing of the whole project.

## FR-013 Configuration discovery
impl: UC-003.s2, UC-003.s2a  prio: must

The system shall locate the project root by searching for `reqmd.toml` in
the start directory and its parents. From this file it shall read include
patterns, glossary path, item types with prefix, allowed and required keys,
link types with allowed target types, value sets, defaults, check settings
and language server settings. The file `reqmd.toml` at the repository root is the reference
example.

## FR-014 Configuration defaults
impl: UC-007.s1  prio: must

Where a configuration section is missing, the system shall use built-in
defaults: all Markdown files below the project root as include pattern, and
the item types, link types, fields, value sets and defaults of the reference
`reqmd.toml` at the repository root, without the project-specific keys
`check` and `sev` and the value set `sev`. In short:

| Type | Prefix | Metadata keys | Body fields | Required |
|------|--------|---------------|-------------|----------|
| usecase | UC | `actor` | `pre`, `post`, `trigger` | |
| functional | FR | | | |
| quality | NFR | `scale`, `meter`, `must`, `plan` | | `scale`, `meter`, `must\|plan` |
| rule | BR | `src` | | |
| story | US | | `ac` | |
| test | TC | | | |

Every type also allows `status` (`draft`, `review`, `approved`,
`deprecated`, default `draft`) and `prio` (`must`, `should`, `could`,
`wont`). Link types: `impl` to use cases and steps, `ref` to any item or
step, `ver` to functional and quality requirements and stories. IDs have at
least 3 digits.

Project configuration is merged with the defaults per entry: an entry such
as `[types.quality]`, `[links.impl]` or a key in `[values]` replaces the
default entry of the same name completely; all other default entries stay.
