# Acceptance tests

Manual acceptance tests of requimd, written in the requimd format. Each test
is a `TC` item that verifies (`ver`) one or more requirements, so the
traceability matrix shows which requirements have tests, and a change to a
requirement makes the links of its tests suspect (FR-116): the tests to run
again.

How to use them, the preparation and the end-to-end scenarios along the use
cases are in `docs/testing-guide.md`. Read that first.

## Files and ID ranges

| File | Tests | Requirements |
|------|-------|--------------|
| `format.md` | TC 001–099 | FR 001–014, file format and parsing |
| `checks.md` | TC 100–199 | FR 100–119, checks |
| `lsp.md` | TC 200–299 | FR 200–219, language server |
| `cli.md` | TC 300–399 | FR 300–308, command line |
| `export.md` | TC 400–499 | FR 400–405, export and reporting |
| `editors.md` | TC 500–599 | FR 500–504 and NFR 011, editor integrations |
| `quality.md` | TC 600–699 | NFR 001–011, quality requirements |
| `rules.md` | TC 700–799 | BR 001–005, through the requirements that enforce them |

A business rule cannot be the target of `ver` (only functional and quality
requirements and stories can, FR-014). Tests of a rule therefore verify the
requirements that put the rule into practice and mention the rule.

## Structure of a file

Each requirement has a section heading, such as "Duplicate IDs (FR-100)",
that is not an item. Below it, a few lines name the decisions behind the
requirement, the open points, what the code decides beyond the spec
(**Code**) and the automated tests that cover it (**Automated**); then
follow its tests. A test reads:

- **Given**: the starting situation, usually a change to the base project.
- **When**: the action, a command or an editor action.
- **Then**: the expected result, derived from the specification only.
- **Code**, where the spec is silent or the code differs: what the
  implementation does.

## Tags

| Tag | Meaning |
|-----|---------|
| ADR-000n | Decision record in `docs/decisions/` |
| Log yyyy-mm-dd | Entry in the decision log of `docs/status.md` |
| Q5 … Q10 | Interpretation listed under open business questions 5 to 10 in `docs/status.md`: implemented, not in the spec, to be confirmed |
| Gap | Found while writing the tests: the spec is silent, ambiguous or contradicts itself. Needs a decision. |
| Validate | The spec is clear, but the behaviour may surprise an analyst. Confirm that it is intended. |
| Code | Decision found only in the code (phase 2); "Code (deviation)" where it differs from the spec |

## Base project

`examples/acceptance/` is a small, clean project (`requimd check` reports
nothing) with built-in defaults except the glossary path:

| File | Content |
|------|---------|
| `usecases.md` | `UC-901` Order goods, steps `s1`, `s2`, `s2a` |
| `functional.md` | `FR-901` to `FR-903`, one per step; `FR-902` mentions `BR-901` |
| `other.md` | `NFR-901` (`ref: FR-902`), `BR-901`, `US-901`, `TC-901` (`ver: FR-902`) |
| `glossary.md` | Cart (aliases basket, shopping cart), Basket fee |

Its IDs use numbers from 901, and tests write every ID of the base project
in backticks. An ID of the base project written without backticks would be
a mention of an item that does not exist in this repository, which
`requimd check` reports.

Each test starts from a fresh copy of the base project unless it says
otherwise. IDs such as `FR-910` in a test are new items the test creates.
