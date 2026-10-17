# ADR-0010 HTML comments outside the content hash

Status: **proposed** on 2026-10-17. Amends ADR-0007.

## Context

ADR-0007 defines the content hash of a link target (FR-116): for an item,
the title, the metadata except `status`, and the body; for a step, the
step text. HTML comments are part of the body, so they count.

"Allow here" (FR-219) writes an inline marker such as
`<!-- requimd: allow fast -->` into an item. Every link to that item then
becomes suspect, although readers see no change: GitHub does not show
comments, the site leaves them out (FR-400), and the language checks
ignore them. False suspect links teach analysts to mark links as reviewed
without looking.

The hash is to be frozen with v0.1.0 (E4 in `docs/feature-report.md`). A
later change would make many links of every project suspect at once, so
the decision is due now (decision D7, 2026-10-17).

## Decision

HTML comments, the inline markers of FR-119 included, are not part of the
content hash: neither in the title, the metadata or the body of an item,
nor in the text of a step. The hash covers what readers see. Everything
else of ADR-0007 stays.

## Considered

- **Only the inline markers leave the hash:** narrower, but a rule that
  counts some comments and not others is harder to explain and to test.
- **No change:** no work, but the false suspect links remain.

## Consequences

- The hash of every item or step that contains a comment changes once. On
  the first check after the update, the links to them are suspect; alpha
  users look at them and run `requimd review --all`.
- A change that an author writes only into a comment no longer makes links
  suspect. Readers do not see such a change either.
- FR-116 says so; the stability promise of E4 covers the hash from v0.1.0
  on.
