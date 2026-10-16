# Acceptance tests: business rules

Business rules cannot be the target of `ver` (FR-014). The tests below
verify the requirements that put each rule into practice and mention the
rule.

Gap for the whole area: should rules be verifiable, for example by
allowing `ver` to rules? Related to open business questions 11
(verification methods) and 12 (design between requirements and tests) in
`docs/status.md`.

## Item ID format (BR-001)

- Q5 (to confirm): a heading such as `## FR-12 Title` (too few digits) is
  silently no item. The "Later" list of `docs/roadmap.md` has a warning
  for such near-miss IDs.
- Gap: whether `FR-0901` and `FR-901` are the same ID. BR-001 allows more
  digits than configured, so both are valid.
- Code: IDs are compared as text, so `FR-0902` and `FR-902` are two
  different items side by side; for the next free number both count as 902
  (TC-700).
- Automated: `br_001_id_needs_configured_digits` (core parse.rs).

### TC-700 Valid and invalid ID forms
ver: FR-001

Given a new file `a.md`:

```markdown
## FR-9001 Four digits
## FR-91 Two digits
## FR-0902 Leading zero
```

When `requimd check`, `requimd show FR-9001`, `requimd show FR-91` run.

Then `FR-9001` is an item; `FR-91` is none, without a diagnostic (Q5).
For `FR-0902` next to the existing `FR-902` the spec is silent (Gap):
note whether it is a second item or a `duplicate-id`.

Code: `FR-0902` is a second item next to `FR-902`, without `duplicate-id`.

### TC-701 Changing the number of digits later
ver: FR-001, FR-013

Given in `requimd.toml`:

```toml
[ids]
digits = 4
```

When `requimd check` runs.

Then all items of the base project with three digits are no items any
more, and their links and mentions are gone with them.

Validate: raising `digits` in a running project invalidates every
existing ID. A migration (for example by `requimd fmt`) or a warning may
be needed.

## IDs are never reused (BR-002)

- ADR-0007: `[ids]` in the lock file never decreases.
- Validate: only numbers up to the highest ever seen are protected. An ID
  below that, deleted and then typed again by hand, is not detected.
- Code: no automated test carries the ID; the tests of FR-212 cover it.
- Automated: none.

### TC-702 Deleted and renamed IDs are not assigned again
ver: FR-212, FR-304, FR-214

Given the base project checked once.

When `FR-903` (the highest) is deleted and `requimd check` runs; a new
`## FR-? Refund` is added and `requimd fmt` runs; then `FR-902` is renamed
to `FR-950` in Neovim, saved, and another `## FR-? Fee` gets an ID with
"Assign ID".

Then the first placeholder becomes `FR-904`, not `FR-903`; the second
`FR-951`. `FR-902` and `FR-903` are not assigned again.

## Step labels are stable (BR-003)

- Gap: BR-003 says a new step gets the next free step number, but no
  feature assigns step labels; the analyst picks them. FR-213 appends a
  missing step with the label of the reference, not the next free number.
- Automated: `br_003_step_label_independent_of_position` (core parse.rs).

### TC-703 No feature renumbers steps
ver: FR-006, FR-304, FR-116

Given the base project checked once; `FR-901` with an additional link
`ref: UC-901`.

When the steps `s1` and `s2` of `UC-901` swap places in the list; then
`requimd fmt` and `requimd check` run.

Then the labels are unchanged and all step references still resolve; the
links to `UC-901.s1` and `UC-901.s2` are not suspect, while `ref: UC-901`
to the whole use case is (its body changed).

## Approval is a human decision (BR-004)

- Code: no automated test; stubs are written without a `status` line.
- Automated: none.

### TC-704 No feature sets status `approved`
ver: FR-212, FR-213, FR-214, FR-218, FR-304, FR-307

Given the base project checked once.

When every writing feature runs once: `requimd fmt`,
`requimd review --all` after a change, and in Neovim "Assign ID", "Create
stub", rename and "Mark as reviewed".

Then no item gets `status: approved` (search the files for `approved`);
new stubs have no `status` line or `status: draft`.

## File layout carries no meaning (BR-005)

- Automated: `br_005_file_layout_carries_no_meaning` (core project.rs).

### TC-705 Moving items between files changes nothing
ver: FR-001, FR-116, FR-403

Given the base project checked once and committed in git.

When `FR-903` moves from `functional.md` to a new file `extra/moved.md`,
and `other.md` is renamed to `misc.md`; then `requimd check` and
`requimd diff HEAD` run.

Then the check reports nothing new and no `suspect-link`; `diff` lists
`FR-903` as moved to another file and `NFR-901`, `BR-901`, `US-901`,
`TC-901` as moved as well, and nothing else.
