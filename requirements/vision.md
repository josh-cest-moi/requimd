# Vision and scope

## Background

Business analysts write requirements mostly in wikis such as Confluence.
Wikis offer no branching, no review workflow comparable to pull requests, no
baselines, and links between requirements are plain hyperlinks that nobody
checks. Traceability, for example from a functional requirement to the use
case step it implements, is maintained by hand or not at all.

Software teams solved these problems for code long ago with git, linters and
language servers. Dedicated requirements tools exist, but they are either
heavyweight and expensive or store requirements in formats that are
unpleasant to write by hand.

## Vision

For business analysts who write requirements as text, requimd is a
lightweight requirements toolset that keeps requirements as plain Markdown
in git and adds validation, traceability and editor support. Unlike wiki
pages or heavyweight requirements tools, it makes writing requirements as
comfortable as writing code in a good editor, without extra typing.

## Business objectives

- Writing a linked requirement takes no more keystrokes than in a wiki.
- Broken or missing links are found automatically, not in review meetings.
- The impact of a change becomes visible at the moment it is made.
- Requirements get the same workflow as code: branches, pull requests, tags.
- Readers without git or an editor can still read the requirements.

## Users and stakeholders

| Who | Interest |
|-----|----------|
| Analyst (primary user) | Writes and maintains requirements in the editor, in German or English |
| Developer | Reads requirements, links tests and code to them |
| Tester | Links test cases, checks coverage |
| Stakeholder / reviewer | Reads published requirements, reviews changes |
| CI pipeline | Validates the repository on every change |

## Scope

In scope for the first release, v0.1.0: the file format; a CLI that
checks, formats and exports a repository (static site, traceability
matrix, JSON, requirement changelog, diagrams); a language server with
authoring, navigation and impact analysis features; and an extension for
VS Code. Details per milestone are in `docs/roadmap.md`.

Long-term goal: a model that covers requirements and design. Design
decisions and design items link to the requirements they answer or
satisfy, take part in tracing and impact analysis, and are verified on
their own level. The first release covers requirements only.

Out of scope: a graphical editor, a web application with user management,
storing requirements in a database, real-time collaborative editing.

## Assumptions and constraints

- Users are comfortable with a text editor and basic git usage, or have
  someone in the team who is.
- The format must stay plain CommonMark with conventions; no custom markup
  that breaks rendering on GitHub or GitLab (NFR-008).
- Analysts work in VS Code with the requimd extension or in Neovim 0.12
  with its built-in LSP client; any editor with an LSP client works
  (NFR-007).
- Requirement texts are in German or English. The language checks are
  configured per project; messages, hover texts and the site are in
  English (ADR-0009).
