## UC-003 Validate the repository
actor: CI pipeline or Analyst  prio: must

trigger: A change is pushed, or the analyst wants to check the whole project.

- s1: Actor runs the check command.
- s2: System parses all included files and builds the item graph.
- s3: System runs all enabled checks and prints the diagnostics with file, line and check ID.
- s4: System ends with an exit code that tells whether errors were found.
- s2a: No configuration found or configuration invalid → system prints the reason and ends with a configuration error code.

post: The actor knows whether the repository is consistent.
