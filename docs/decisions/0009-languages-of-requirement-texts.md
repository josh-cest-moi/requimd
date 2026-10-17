# ADR-0009 Languages of requirement texts

Status: **proposed** on 2026-10-17.

## Context

Analysts who use requimd write requirements in German and in English
(decision F1 of `docs/feature-report.md`, 2026-10-16). The language checks
of ADR-0005 are English:

- the built-in weak words and modal verbs (FR-110, FR-118);
- the required verb "shall" (FR-111);
- the story pattern "As a …, I want … so that …" (FR-113).

A German team today gets a `no-shall` finding on every requirement and a
`modal-verb` finding on the German verb "will" ("wants"), and no finding
on "soll", "kann" or "schnell" unless it adds them to `requimd.toml`. The
item types a check reads and the name of the field `ac` are fixed in the
code (open business question 7, technical question 1).

## Decision

1. **Language-neutral configuration.** Everything English in the language
   checks becomes configurable in `requimd.toml`; requimd ships English
   defaults only:
   - weak words and modal verbs: `builtin = false` switches the built-in
     list off; configured `words` extend it, or replace it with
     `builtin = false`;
   - the required verbs of FR-111: `words` under `[checks.no-shall]`,
     default `shall`;
   - the story patterns of FR-113: `patterns` under
     `[checks.story-pattern]`, each a list of phrases in order;
   - acceptance criteria: a check of their own, `acceptance-criteria`
     (FR-121), with the option `field`, default `ac`;
   - every language check has the option `types`, the item types it
     reads, with today's types as default.
2. **One configuration per project.** A project with texts in both
   languages configures both: for example `words = ["shall", "muss"]`, and
   the English and the German story patterns.
3. **English output.** Messages, hover texts and the site stay English,
   like the output of compilers and linters.
4. **A German example configuration** in the documentation: weak words
   with their common forms, modal verbs, "muss", and the patterns
   "Als …, möchte ich …, damit …" and "Als …, möchte ich …, um …".

## Considered

- **English only by design:** the smallest effort, but it leaves out the
  German-writing teams among the primary users.
- **Built-in language packs** (`language = "de"`): no setup for German
  teams, but requimd would curate and maintain the lists of every
  language, while word lists are a team decision anyway (ADR-0005,
  decision 6).
- **Translated messages and site:** a large effort for little gain; the
  owner decided that the output stays English.

## Consequences

- FR-110, FR-111, FR-113 and FR-118 change; FR-121 is new. FR-015 names
  the fixed meanings that remain (decision F4).
- Check IDs keep their names; `no-shall` also checks "muss".
- Explicit word forms (ADR-0005, decision 3) cost more in German, where an
  adjective has up to six forms. They stay, for predictable results; the
  German example lists the common forms.
