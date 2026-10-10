## UC-005 Rename an item ID
actor: Analyst  prio: should

- s1: Analyst requests a rename on an item ID and enters the new ID.
- s2: System changes the declaration and all references in all files.
- s1a: The new ID already exists or is invalid → system refuses the rename with a message.

post: No reference points to the old ID.
