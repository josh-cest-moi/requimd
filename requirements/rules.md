# Business rules

## BR-001 Item ID format
prio: must

An item ID consists of a configured prefix, a hyphen and a decimal number
with at least the configured number of digits (default 3, zero-padded), for
example `FR-007` or `UC-012`. Numbers with more digits are valid, so ranges
never run out.

## BR-002 IDs are never reused
prio: must

Once an ID has been assigned, it is never assigned to a different item, even
if the original item is deleted. Obsolete items are set to `deprecated`
rather than deleted where possible.

## BR-003 Step labels are stable
prio: must

A step label is never changed when steps are inserted, removed or reordered.
A new step gets the next free step number in its use case. The order of the
list defines the flow; the label only identifies the step. Extension steps
append lowercase letters to the label of the step they extend, for example
`s2a`.

## BR-004 Approval is a human decision
prio: must

Only a person may set an item's status to `approved`. Tools and AI
assistants may propose status changes but never perform this one.

## BR-005 File layout carries no meaning
prio: must

Identity, type and relationships of items derive only from file content,
never from file or directory names. Files can be split, merged, moved and
renamed without changing the specification.
