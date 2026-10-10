# Expected diagnostics for broken.md

Line numbers refer to `broken.md`. Checks marked (M3) are expected once that
milestone is implemented. This file has no items and must produce no
diagnostics. The example uses the built-in default lists (FR-014, FR-110,
FR-118).

| Line | Check | Severity | Reason |
|------|-------|----------|--------|
| 7 | duplicate-step | error | label `s2` used twice |
| 7 | uncovered-step | warning | step `s2` has no implementing item |
| 8 | duplicate-step | error | label `s2` used twice |
| 8 | uncovered-step | warning | step `s2` has no implementing item |
| 10 | duplicate-id | error | `FR-001` declared twice |
| 10 | weak-word | warning | "Fast" in the title (M3, FR-110) |
| 10 | no-shall | info | text has no "shall" (M3) |
| 11 | invalid-value | error | `prio: urgent` is not in the value set |
| 11 | unknown-key | warning | `colour` is not a configured key |
| 13 | modal-verb | warning | "should", use "shall" (M3, FR-118) |
| 13 | weak-word | warning | "fast" (M3) |
| 15 | duplicate-id | error | `FR-001` declared twice |
| 16 | dangling-ref | error | `UC-001.s9` does not exist |
| 24 | dangling-ref | warning | mention of `FR-999`, which does not exist |
| 26 | placeholder | warning | placeholder ID `FR-?` |
| 26 | glossary-alias | warning | "Basket" in the title is an alias of "Cart" (M3, FR-114) |
| 28 | glossary-alias | warning | "basket" is an alias of "Cart" (M3) |
| 30 | missing-key | error | `NFR` requires `scale` and one of `must`/`plan` |
| 33 | weak-word | warning | "quickly" is in the default list (M3) |
| 43 | link-target | error | `impl` must point to a use case or step, not a functional requirement |
| 51 | weak-word | warning | "easy"; the marker on this line allows only "quick" (M3) |
| 51 | unused-allow | warning | marker allows "quick", which is not on the line (M3, FR-119) |

Must not produce diagnostics:

- Line 21: `impl: FR-001` points to a duplicated ID. It resolves to no item
  and produces no further diagnostic (FR-100). Line 43 tests `link-target`
  instead.
- Coverage of `UC-001.s1`: covered, because links from duplicated items are
  evaluated normally (FR-100); FR-004 also implements it.
- The fenced block at lines 35–38 and the inline code on line 40 (FR-009).
- Line 50: "Fast" is allowed by the inline marker (FR-119).
