# Verification record v0.1.0

One row per item of `requirements/`, for the verification described in
`docs/testing-guide.md`. Fill in per item:

- **Valid**: is this the right requirement? `ok`, `spec` (must change) or
  `question`.
- **Verified**: does requimd fulfil it? `ok` or `bug`; `-` if not built.
- **Note**: what to change, the bug, the question, or the decision on a
  Gap or Q point of its tests.

Items with `ok` in both columns get `status: approved` (only the product
owner sets it, BR-004), in the order of section 1 of the guide. Use cases
are verified through the scenarios (section 4 of the guide), business
rules through the tests in `acceptance/rules.md`.

## Business rules

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| BR-001 | Item ID format | must | TC-700, TC-701 | | | |
| BR-002 | IDs are never reused | must | TC-702 | | | |
| BR-003 | Step labels are stable | must | TC-703 | | | |
| BR-004 | Approval is a human decision | must | TC-704 | | | |
| BR-005 | File layout carries no meaning | must | TC-705 | | | |

## Use cases

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| UC-001 | Write a new requirement | must | scenario UC-001 | | | |
| UC-002 | Navigate traceability in the editor | must | scenario UC-002 | | | |
| UC-003 | Validate the repository | must | scenario UC-003 | | | |
| UC-004 | Review the impact of a change | should | scenario UC-004 | | | |
| UC-005 | Rename an item ID | should | scenario UC-005 | | | |
| UC-006 | Publish requirements for stakeholders | should | scenario UC-006 | | | |
| UC-007 | Set up a project | must | scenario UC-007 | | | |

## File format

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| FR-001 | Item declaration | must | TC-001, TC-002, TC-003, TC-700, TC-701, TC-705 | | | |
| FR-002 | Item extent | must | TC-004, TC-005, TC-006 | | | |
| FR-003 | Metadata block | must | TC-108, TC-007, TC-008, TC-009, TC-010, TC-011 | | | |
| FR-004 | Multiple values | must | TC-012 | | | |
| FR-005 | Body fields | must | TC-109, TC-013, TC-014 | | | |
| FR-006 | Use case steps | must | TC-015, TC-016, TC-017, TC-703 | | | |
| FR-007 | Step references | must | TC-016, TC-018 | | | |
| FR-008 | Mentions in text | must | TC-004, TC-006, TC-019, TC-020 | | | |
| FR-009 | Code is ignored | must | TC-021, TC-028 | | | |
| FR-010 | Placeholder IDs | must | TC-022, TC-023 | | | |
| FR-011 | Glossary | should | TC-024, TC-025, TC-026, TC-209 | | | |
| FR-012 | Tolerant parsing | must | TC-027, TC-028, TC-608 | | | |
| FR-013 | Configuration discovery | must | TC-025, TC-029, TC-030, TC-031, TC-032, TC-701 | | | |
| FR-014 | Configuration defaults | must | TC-308, TC-033, TC-034 | | | |

## Checks

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| FR-100 | Duplicate IDs | must | TC-100, TC-101, TC-102 | | | |
| FR-101 | Dangling references | must | TC-103, TC-104, TC-105, TC-023 | | | |
| FR-102 | Unknown metadata key | must | TC-106, TC-107, TC-108, TC-109 | | | |
| FR-103 | Invalid value | must | TC-110, TC-111, TC-008 | | | |
| FR-104 | Link target type | must | TC-112 | | | |
| FR-105 | Required keys | must | TC-113, TC-114, TC-115 | | | |
| FR-106 | Uncovered steps | must | TC-102, TC-116, TC-117, TC-118 | | | |
| FR-107 | Duplicate step labels | must | TC-119 | | | |
| FR-108 | Refinement cycles | must | TC-120, TC-121 | | | |
| FR-109 | Status consistency | must | TC-122, TC-123 | | | |
| FR-110 | Weak words | should | TC-124, TC-125, TC-126, TC-127, TC-128, TC-129, TC-153 | | | |
| FR-111 | Missing "shall" | should | TC-130, TC-131 | | | |
| FR-112 | EARS patterns | could | – | | | |
| FR-113 | User story pattern | should | TC-132, TC-133, TC-134, TC-013 | | | |
| FR-114 | Glossary aliases | should | TC-135, TC-136, TC-137, TC-024 | | | |
| FR-115 | Remaining placeholders | must | TC-138, TC-022 | | | |
| FR-116 | Suspect links | should | TC-139, TC-140, TC-141, TC-142, TC-143, TC-144, TC-303, TC-316, TC-703, TC-705 | | | |
| FR-117 | Check configuration | must | TC-104, TC-129, TC-145, TC-146 | | | |
| FR-118 | Modal verbs | should | TC-147, TC-148 | | | |
| FR-119 | Allow lists and inline markers | should | TC-146, TC-149, TC-150, TC-151, TC-152, TC-153, TC-230 | | | |

## Language server

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| FR-200 | Server start | must | TC-200 | | | |
| FR-201 | Project scope | must | TC-201 | | | |
| FR-202 | Live diagnostics | must | TC-202, TC-203 | | | |
| FR-203 | Reference completion | must | TC-204, TC-205 | | | |
| FR-204 | Key and value completion | must | TC-206 | | | |
| FR-205 | Hover | must | TC-024, TC-207, TC-208, TC-209 | | | |
| FR-206 | Go to definition | must | TC-210 | | | |
| FR-207 | Find references | must | TC-211, TC-212 | | | |
| FR-208 | Go to implementation | must | TC-213 | | | |
| FR-209 | Inlay hints | should | TC-214 | | | |
| FR-210 | Document symbols | must | TC-215 | | | |
| FR-211 | Workspace symbols | must | TC-216 | | | |
| FR-212 | Assign ID | must | TC-303, TC-310, TC-217, TC-218, TC-219, TC-702, TC-704 | | | |
| FR-213 | Create stub | should | TC-220, TC-221, TC-222, TC-223, TC-704 | | | |
| FR-214 | Rename | should | TC-224, TC-225, TC-226, TC-227, TC-702, TC-704 | | | |
| FR-215 | Code lens on steps | should | TC-501, TC-228 | | | |
| FR-216 | Semantic tokens | could | TC-505, TC-229, TC-230, TC-231 | | | |
| FR-217 | External file changes | must | TC-232, TC-233 | | | |
| FR-218 | Mark as reviewed | should | TC-218, TC-234, TC-235, TC-704 | | | |
| FR-219 | Language check code actions | should | TC-236, TC-237, TC-238 | | | |

## Command line

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| FR-300 | Check command | must | TC-300, TC-301, TC-302, TC-303, TC-304, TC-305, TC-029, TC-030 | | | |
| FR-301 | Machine-readable output | must | TC-306 | | | |
| FR-302 | Strict mode | must | TC-307 | | | |
| FR-303 | Init command | must | TC-308, TC-309 | | | |
| FR-304 | Format command | should | TC-310, TC-311, TC-312, TC-702, TC-703, TC-704 | | | |
| FR-305 | Show command | should | TC-313, TC-314 | | | |
| FR-306 | Coverage command | should | TC-315 | | | |
| FR-307 | Review command | should | TC-316, TC-317, TC-704 | | | |
| FR-308 | LSP command | must | TC-200 | | | |

## Export

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| FR-400 | Static site | should | TC-400, TC-401 | | | |
| FR-401 | Traceability matrix | should | TC-402, TC-403 | | | |
| FR-402 | JSON export | should | TC-404 | | | |
| FR-403 | Requirement changelog | could | TC-405, TC-406, TC-407, TC-408, TC-705 | | | |
| FR-404 | Diagram export | could | TC-409, TC-410, TC-411 | | | |
| FR-405 | Publishing target (deprecated) | could | – | | | |

## Editor integrations

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| FR-500 | VS Code extension | must | TC-500, TC-501, TC-506 | | | |
| FR-501 | Bundled language server | must | TC-502, TC-503 | | | |
| FR-502 | Extension packages in the release | must | TC-504 | | | |
| FR-503 | Highlighting in VS Code | should | TC-505 | | | |
| FR-504 | IntelliJ setup guide | should | TC-507 | | | |

## Quality requirements

| Item | Title | Prio | Tests | Valid | Verified | Note |
|------|-------|------|-------|-------|----------|------|
| NFR-001 | Diagnostics latency | must | TC-600 | | | |
| NFR-002 | Full check duration | must | TC-601 | | | |
| NFR-003 | Server start | must | TC-602 | | | |
| NFR-004 | Platforms | must | TC-603 | | | |
| NFR-005 | Installation | must | TC-604 | | | |
| NFR-006 | Offline operation | must | TC-605 | | | |
| NFR-007 | Editor neutrality | must | TC-606 | | | |
| NFR-008 | Plain Markdown readability | must | TC-607 | | | |
| NFR-009 | Robustness | must | TC-608 | | | |
| NFR-010 | Test traceability | must | TC-609 | | | |
| NFR-011 | VS Code feature coverage | must | TC-506, TC-610 | | | |
