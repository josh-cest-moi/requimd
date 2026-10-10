## UC-001 Write a new requirement
actor: Analyst  prio: must

pre: The file is inside a reqmd project and the editor's LSP client is attached.

- s1: Analyst writes a heading with a placeholder ID and a title.
- s2: Analyst writes the metadata block; system offers completion for keys, values and link targets.
- s3: Analyst writes the requirement text; system reports diagnostics while the analyst types.
- s4: Analyst requests ID assignment; system replaces the placeholder with the next free ID.
- s2a: A link target does not exist → system reports a diagnostic and offers to create a stub item.
- s3a: The text contains a weak word → system reports a diagnostic with the reason.

post: The file contains a valid item with a unique ID.
