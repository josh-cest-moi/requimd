# Glossary

## Item
alias: requirement item, artifact

A uniquely identified unit of the specification, such as a use case, a
functional requirement or a business rule. Declared by a heading that starts
with an item ID.

## Item ID
alias: requirement ID

The unique identifier of an item: a prefix, a hyphen and a number, for
example `FR-007`. See BR-001.

## Prefix

The leading letters of an item ID. The prefix determines the item type and
is configured in `reqmd.toml`.

## Item type

The kind of item, for example use case, functional requirement or quality
requirement. Determines which metadata keys and checks apply.

## Metadata block

The lines directly below an item heading consisting of `key: value` pairs,
for example status, priority and links.

## Body field

A labelled line inside an item body, such as `pre:` or `ac:`, that the
parser extracts as a named value.

## Link

A typed relationship from one item to another item or to a use case step,
written as a metadata entry, for example `impl: UC-012.s2`.

## Link type

The meaning of a link, such as implements, refines or verifies. Configured
in `reqmd.toml` together with its metadata key.

## Mention

An untyped reference to an item ID in the running text of an item.

## Reference

Either a link or a mention.

## Use case step
alias: step

A numbered action in the flow of a use case, identified by a step label.

## Step label

The stable identifier of a use case step within its use case, such as `s2`
or `s2a`. Independent of the position in the list (BR-003).

## Placeholder ID

An item ID without a number, such as `FR-?`, used while writing. Replaced
by a real ID on request.

## Suspect link

A link whose target has changed since the link was last reviewed.

## Lock file

The file `reqmd.lock` in the project root that records review state of
links and the highest assigned number per prefix.

## Coverage

The share of use case steps that at least one item implements.

## Project root

The directory containing `reqmd.toml`.

## Diagnostic

A finding reported by a check, with location, severity, check ID and
message.

## Weak word

A word that makes a requirement vague or untestable, for example `fast`,
`user-friendly` or `etc.`.

## EARS

Easy Approach to Requirements Syntax: sentence templates for requirements,
such as `When <trigger>, the system shall <response>`.
