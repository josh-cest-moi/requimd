## UC-004 Review the impact of a change
actor: Analyst  prio: should

pre: Items link to the item that is about to change.

- s1: Analyst changes the text of an item or a use case step.
- s2: System marks all links pointing to the changed target as suspect.
- s3: Analyst opens a linking item; system shows the suspect link as a diagnostic.
- s4: Analyst adapts or confirms the linking item and marks the link as reviewed.
- s5: System records the review in the lock file and removes the diagnostic.

post: Every item affected by the change has been reviewed.
