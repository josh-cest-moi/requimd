## UC-002 Navigate traceability in the editor
actor: Analyst  prio: must

pre: The project contains use cases and items linked to their steps.

- s1: Analyst opens a use case; system shows for each step how many items implement it.
- s2: Analyst requests the implementations of a step; system lists all items linking to it.
- s3: Analyst opens one of these items; system shows the text of linked targets inline and on hover.
- s4: Analyst jumps from a link to its target.
- s5: Analyst requests all references to an item; system lists all links and mentions.

post: The analyst has reviewed the requirements related to a step without searching manually.
