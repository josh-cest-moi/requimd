## UC-007 Set up a project
actor: Analyst  prio: must

- s1: Analyst runs the init command in a repository; system creates a configuration with defaults.
- s2: Analyst adds the language server to the editor configuration.
- s2a: Analyst uses VS Code → analyst installs the requimd extension instead.
- s2b: Analyst uses IntelliJ IDEA → analyst installs the LSP4IJ plugin and configures the language server as the guide describes.
- s3: Analyst opens a Markdown file of the project; system attaches the language server.

post: The project is ready for writing requirements.
