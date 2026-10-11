# ADR-0005 Design of the language checks

Status: **accepted** on 2026-10-09 by the product owner.

## Context

Weak words and similar checks must catch vague requirements without
annoying analysts with false positives. Specified in FR-110, FR-114,
FR-118, FR-119 and FR-219.

## Decisions

1. **Three stages.** Stage 1 is a dictionary with a reason per entry
   (FR-110). Stage 2 adds simple pattern rules, starting with modal verbs
   (FR-118). Stage 3, an on-demand AI review, stays on the "later" list;
   it is not deterministic enough for CI.
2. **Modal verbs are a separate check** (`modal-verb`), not weak words, so
   they get their own message ("use shall") and severity.
3. **Explicit word forms.** No wildcards or stemming; `quick` and
   `quickly` are separate entries. Predictable beats short.
4. **Titles are checked**, because they appear in completion, hover and the
   outline (FR-110, FR-114).
5. **Silencing**: a project-wide `allow` list per check, and an inline
   HTML comment `<!-- requimd: allow <word> -->` per line. The comment is
   invisible on GitHub and GitLab (NFR-008) and is written by a code action
   (FR-219). Stale markers are reported (`unused-allow`).
6. **Lists live in `requimd.toml`**, versioned and shared, so editor and CI
   behave identically.

## Rejected alternatives

- Per-item `allow:` metadata key: visible to readers, too coarse.
- Wildcards like `quick*`: over-match words such as "quickstart".
