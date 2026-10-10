# ADR-0002 Markdown dialect

Status: **accepted**

## Context

The format must be easy to type, readable in any Markdown renderer
(NFR-008) and unambiguous for a machine.

## Decision

Plain CommonMark with conventions, specified in
`requirements/functional/format.md`:

- An item is a heading whose first token is an ID (FR-001).
- Metadata is a `key: value` block directly below the heading, several pairs
  per line allowed (FR-003).
- References are bare IDs; no brackets or sigils (FR-008). Prefixes are
  configured, which keeps false positives low.
- Use case steps are list items with stable labels such as `s2` (FR-006,
  BR-003).
- Code blocks and inline code are ignored (FR-009), so the spec can contain
  examples of itself.

## Rejected alternatives

- YAML front matter per item: verbose, forces one item per file.
- Wiki links like `[[FR-001]]`: more typing, no gain once prefixes are known.
- Pandoc attributes `{#id key=value}`: rendered as raw text on GitHub/GitLab.
- HTML comments for metadata: invisible to readers, awkward to type.
- AsciiDoc or reStructuredText: better anchors, but less familiar and less
  well supported by renderers and editors.

## Consequences

Bare-ID references need a configured prefix list; an ID-like word that is
not meant as a reference must be written as inline code.
