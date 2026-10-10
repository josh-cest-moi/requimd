# Roadmap

Each milestone lists the requirements in scope. A milestone is done when
all its `must` items are implemented, each has a test containing its ID
(NFR-010), and `reqmd check` passes on this repository once available.

## M0 Specification (done)

Spec in `requirements/`, decisions in `docs/decisions/`. Rust chosen
(ADR-0001). Current state and open questions: `docs/status.md`.

## M1 Parser and check command (next)

Goal: `reqmd check` validates this repository and the examples in CI.

- Prerequisite: ADR-0003 (Markdown parser) confirmed
- Setup: cargo workspace, CI workflow per ADR-0004
- Format: FR-001 to FR-010, FR-012 to FR-014
- Checks: FR-100 to FR-109, FR-115, FR-117
- CLI: FR-300 to FR-303
- Quality: NFR-002, NFR-004, NFR-006, NFR-008; NFR-010 applies to every milestone
- Rules: BR-001, BR-003, BR-005

## M2 Language server core

Goal: writing and navigating in Neovim 0.12 without plugins.

- LSP: FR-200 to FR-208, FR-210 to FR-212, FR-217, FR-308
- Quality: NFR-001, NFR-003, NFR-007, NFR-009
- Editor guide in `docs/neovim.md` verified feature by feature

## M3 Authoring comfort

- Format: FR-011 (glossary)
- Checks: FR-110, FR-111, FR-113, FR-114, FR-118, FR-119
- LSP: FR-209, FR-213 to FR-215, FR-219
- CLI: FR-304 to FR-306

## M4 Impact analysis

- FR-116, FR-218, FR-307, lock file format decided in an ADR

## M5 Publishing

- FR-400 to FR-402, then the `could` items FR-403 to FR-405
- NFR-005 and release packaging

## Later / ideas

- FR-112 (EARS detection), FR-216 (semantic tokens)
- Issue tracker links (e.g. `jira: SHOP-412`)
- Import of test results so test items show pass/fail
- On-demand AI review for ambiguity and contradictions (stage 3, ADR-0005)
- Further pattern rules: comparatives without reference, passive voice
  without actor
