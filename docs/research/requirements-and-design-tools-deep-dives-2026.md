# Fix the lock, then own decisions

> Phase 2 of the study: eight deep dives. Phase 1, the broad scan with ideas R-01 to R-97, is [requirements-and-design-tools-2026.md](requirements-and-design-tools-2026.md). The notes behind each deep dive are in [`notes/`](notes/) (`dd-1` to `dd-8`).

The eight deep dives show that requimd's most urgent task is a correction, not a new feature. **The content hash that v0.1.0 is about to freeze also counts the target item's own link lines, its `prio` and the order of its metadata.** I confirmed this in `crates/requimd-core/src/lock.rs`. So adding `impl: UC-003.s2` to FR-012 makes every test that verifies FR-012 suspect, even though nothing a reader cares about has changed. Doorstop, the closest file-based precedent, leaves exactly these parts out ([Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)). Three related findings point the same way. The lock and configuration parsers reject any unknown field. The recovery hint tells users to delete an unreadable lock, which silently clears every open suspect link and can reuse deleted IDs. And the HTML-comment exclusion of ADR-0010 exists in the spec but not yet in the code. All of this belongs in the stability ADR (E4), before v0.1.0. Beyond that, the deep dives confirm phase 1's two strategic bets and make them concrete. **No tool found propagates a requirement change to the decisions that depend on it** (DD-1), and requimd's existing suspect links already do most of the work. Review evidence can live in the repository as hash-pinned records, but **no assessor, notified body, FAA or FDA statement either accepting or rejecting git-based review evidence was found** (DD-2). On writing quality, the German and English language checks remain a real differentiator: IBM's checker is English- and ASCII-only ([IBM](https://www.ibm.com/support/knowledgecenter/SS3UPN/com.ibm.help.rm.assist.doc/topics/t_running_rqa.html?view=kc)), and no Rust neighbour checks wording at all (DD-6, DD-8). The glossary claim, however, needs narrower wording. The report proposes five milestones after v0.1.0, merges overlapping proposals into one configuration design, and gives the product owner 39 decisions.

## What the deep dives changed compared with phase 1

Phase 1 (`requirements-and-design-tools-2026.md`) scanned the field and listed 97 ideas, R-01 to R-97. The deep dives tested the 16 ideas it ranked first against primary sources and against requimd's own code. Five things changed.

**The first priority moved before v0.1.0.** Phase 1 praised requimd's suspect-link design as "more refined than most". DD-3 found that the hash scope causes the very cascade of false suspects that Doorstop avoids. Because ADR-0007 and ADR-0010 freeze the hash with v0.1.0, the fix is due now, together with a lock version field and a forward-compatibility rule.

**Several ideas turned out cheaper than phase 1 thought.** Decision re-evaluation (R-56) needs no new mechanism: a decision's `addresses` and `driven-by` links are ordinary links that FR-116 already watches. "Show what changed" (B6, R-38) needs no commit stored in the lock. It can find the old text through git history by content hash.

**Some ideas were narrowed or dropped.** Item-level review state (R-36) is not recommended now. The `raises` link (R-68) runs against requimd's convention that the downstream item carries the link. Accepting OpenFastTrace-style tags (R-26) gives familiar syntax but no real interoperability. A live ReqIF round trip and imports of SDD tool specs are out.

**Proposals from different deep dives overlap**, in `requimd.toml`, in the lock file, in review evidence and in status names. The section "One design from eight proposals" merges them.

**Facts were corrected.** The main corrections: SOPHIST's current bindingness words are MUSS, SOLLTE and WIRD; "GfWR" is not a new INCOSE edition; StrictDoc has no JUnit import; and `[impl->FR-012]` is not a valid OpenFastTrace tag. The full list is in "Corrections to phase 1".

## DD-1: Decisions and open questions fill an empty niche cheaply

**Evidence.** All primary sources share a small core: ID, title, status, context, decision, alternatives, consequences. On top of that, three features recur. Decisions link to what they answer: MADR has "decision drivers", Tyree/Akerman "related requirements", and Y-statements open with "in the context of" ([MADR template](https://raw.githubusercontent.com/adr/madr/develop/template/adr-template.md); [Zimmermann](https://medium.com/olzzio/y-statements-10eb07b5a177)). Decisions can be replaced or amended. And open questions are tied to a gate. Rust RFCs split unresolved questions into those to settle before merge and those to settle before stabilization ([Rust RFC template](https://raw.githubusercontent.com/rust-lang/rfcs/master/0000-template.md)). KEPs mark disputed text with UNRESOLVED blocks and keep agreement (`status`) apart from realisation (`stage`) ([KEP process](https://raw.githubusercontent.com/kubernetes/enhancements/master/keps/sig-architecture/0000-kep-process/README.md)).

The research literature explains why this matters. Decision knowledge "vaporizes" when it is not a first-class artefact ([Jansen and Bosch, via bibsonomy](https://www.bibsonomy.org/bibtex/21aa2dfe9e83593c3d5b81b0112da625c/leonardo)). In a study of 24 architects, assumptions turned out invalid during their lifecycle ([Yang et al., Chalmers](https://research.chalmers.se/en/publication/252320)). In a survey of 112 practitioners, few recorded assumptions, mainly for lack of tools ([RUG](https://research.rug.nl/en/publications/a-survey-on-software-architectural-assumptions/)). No tool or paper was found that propagates requirement changes to decisions through stored hashes.

requimd's own practice points to the right audience. Its decision log in `docs/status.md` has about 77 rows, and most are product and scope decisions made by the owner as analyst, not architecture decisions. ADR-0010 already uses an "amends" relation next to "supersedes".

**Proposal.** One ADR on the traceability model (B9, F3) introduces the types in three steps. Step 1 is decisions (DEC) and open questions (Q). Step 2 is assumptions (ASM) and spikes (SPK). Step 3 is design elements (DES). There is a single decision type with an optional `kind: business | requirements | architecture`. Agreement lives in `status`, with values per type. Realisation is computed from `ver` links: a decision is *unconfirmed* until a TC carries `ver: DEC-004`, and *confirmed* once one does. Constraints stay BR or NFR items, and risks stay a body section.

```markdown
## DEC-004 Round VAT per line item, not per order
addresses: FR-104, UC-012.s2  driven-by: BR-007, NFR-003
answers: Q-003  assumes: ASM-002  status: accepted  kind: business

In the context of the cart total (UC-012.s2), facing totals that differ
between cart and invoice, we decided for rounding per line item and
neglected rounding per order, to match the ERP invoice (BR-007).

## Q-003 Which rounding does the ERP use?
about: FR-104  gate: approved  status: open  ask: Finance
```

Re-evaluation works through four parts:

1. A changed FR-104 makes the `DEC-004 addresses FR-104` link suspect through FR-116, with no new machinery. An item-level summary reads "DEC-004 needs re-evaluation: 2 suspect links". For a decision, FR-218's "mark as reviewed" means "the decision still holds".
2. Status changes do not change hashes, so status-driven propagation needs status rules in the style of FR-109:
   - `invalid-assumption` fires when a link `assumes` an ASM whose status is `invalidated`;
   - `superseded-target` fires when a link targets a superseded, rejected or deprecated DEC;
   - `open-question-gate` fires when a target has reached the gate status while its question is still open.
3. An edit to an accepted decision needs a hash of the item *itself*. Today the lock stores only target hashes.
4. Propagation goes one hop only. Impact analysis can show longer chains on demand.

| Phase 1 idea | Verdict after DD-1 |
|---|---|
| R-54 decision items | Confirmed; statuses `draft`, `proposed`, `accepted`, `rejected`, `superseded`, `deprecated`; tools and AI create only `draft` or `proposed` |
| R-55 decision links | Confirmed and widened to `addresses`, `driven-by`, `assumes`, `answers`, `evidence`, `supersedes`, `amends`, `about` |
| R-56 re-evaluation | Confirmed, and cheaper: reuses FR-116 links; adds an item-level summary |
| R-57 immutability | Changed: needs a new per-item anchor (see the reconciliation section) |
| R-58 supersession | Confirmed, plus `amends`; inverses are computed, never written by hand |
| R-59 confirmation | Changed: realisation is computed from `ver`, not stored as a second status field |
| R-60 open questions | Confirmed, with a `gate:` key naming the blocked status |
| R-61 assumptions | Confirmed for step 2; needs a status rule, not a hash |
| R-62 spikes | Confirmed for step 2: hypothesis, signal fixed in advance, timebox, verdict |
| R-63 options | Confirmed; labels `o1`, `o2` need FR-006 and BR-003 extended beyond use cases |
| R-66 revisit dates | Confirmed as opt-in `--as-of <date>`; a free-text `trigger:` field as well |
| R-67 design elements | Deferred to step 3 |
| R-68 `raises` | Dropped: it points downstream; the new item carries `ref: DEC-…` instead |

Kruchten's state names (Tentative, Challenged, Obsolesced) and his relation list could be read only from an unofficial excerpt. They must be checked before an ADR cites them.

## DD-2: Review records can live in git, but nobody has tested them with an assessor

**Evidence.** Jama, Modern Requirements, Codebeamer, Polarion and DOORS Next share one review model:

- a scope fixed to a version;
- three tiers of roles (moderator, approver, reviewer);
- per-item votes;
- re-review of only what changed;
- a finalized record that cannot be deleted once signed.

Codebeamer, for example, reviews each item "at the version it had when the review was created" and can restart a review for only the modified items ([Codebeamer Review Hub](https://support.ptc.com/help/codebeamer/r3.1/en/codebeamer/cbx_user_guide/review_hub/cbx_reviews_in_codebeamer.html)). Signatures are optional per review, and the moderator decides whether they apply ([Jama settings](https://help.jamasoftware.com/en/reviews-in-jama-connect/moderator-tasks/create-a-review/configure-review-settings---settings-page.html)).

Part 11 asks three things of a signed record: the printed name, the date and time, and the meaning ([21 CFR 11.50](https://www.law.cornell.edu/cfr/text/21/11.50)). The signature must also be linked to its record ([11.70](https://www.law.cornell.edu/cfr/text/21/11.70)). Uniqueness, identity proofing and two-component signing remain organisational controls ([11.100](https://www.law.cornell.edu/cfr/text/21/11.100), [11.200](https://www.law.cornell.edu/cfr/text/21/11.200)). ISPE argues that commits already record the who, when and how of code changes ([ISPE 2025](https://ispe.org/pharmaceutical-engineering/january-february-2025/gampr-approach-computerized-system-life-cycle-and)).

The safety and medical standards ask for review records that name the reviewed version, the reviewers, the date, the results and the findings, but they do not prescribe storage. The closest public precedent is **Eclipse S-CORE**, which runs ISO 26262 and ASPICE inspections in GitHub pull requests. Its status `valid_inspected` falls back to `valid` when the content changes ([S-CORE review concept](https://eclipse-score.github.io/process_description/main/general_concepts/score_review_concept.html)). S-CORE has published no assessment result.

There is also a warning. OpenRegulatory, which once promoted a git-based QMS, now advises against GitHub and GitLab as QMS software, mainly because non-technical users struggle with git and Markdown ([OpenRegulatory](https://openregulatory.com/articles/we-no-longer-recommend-github-gitlab-as-qms-software); the article returned 403, so this rests on search snippets).

One finding concerns requimd's own workflow. GitHub keeps approvals in its database. They can be read only through the API ([GitHub REST](https://docs.github.com/en/rest/pulls/reviews)). With this project's "Rebase and merge" rule, **a merged PR leaves no trace of who approved it in git at all**. Only Gerrit keeps review data in git refs ([Gerrit review notes](https://gerrit.googlesource.com/plugins/reviewnotes/+/v2.8/src/main/resources/Documentation/refs-notes-review.md)).

**Proposal.** The proposal has three parts.

*A review package* is generated for a git range. It is static HTML with print CSS, plus a pre-filled sheet and a manifest. Per item it shows ID, title, status, change kind, the content hashes at both refs, the authors from git log, a word-level diff and the suspect impact. A checklist covers the rules the tool cannot check.

*A review record* is one file per review, `reviews/RV-012.md`, written as a requimd item with prefix RV. It lists the people with their role, meaning, date and source, and a verdict per item together with the hash that verdict refers to.

*Staleness.* `requimd check` recomputes each listed item's hash and reports "RV-012 is stale for FR-011". An opt-in rule warns on an `approved` item that has no current approving record. requimd never writes `approved` (BR-004).

```markdown
## RV-012 Review of payment requirements for release 2.3
range: v2.2..4e1f9a7c  package: 3c9d0e11a2b4f675  closed: 2026-11-28

| Reviewer | Role | Meaning | Date | Source |
|---|---|---|---|---|
| A. Example | approver | approved | 2026-11-27 | signed commit |
| B. Example | reviewer | reviewed | 2026-11-26 | sheet, recorded by C. Example |

| Item | Hash | A. Example | B. Example | Finding |
|---|---|---|---|---|
| FR-011 | 1d9e44b0a2c6f871 | reject | comment | ISS-41 unit missing |
```

Reviewers without git return verdicts in three ways, from strongest to weakest evidence: their own signed commit, the sheet or `mailto:` links, or paper that the analyst transcribes. Transcribed rows say so in their source column. The documentation must state plainly what this does *not* give: Part 11 system controls, identity management, protection against history rewriting, and any assessor's acceptance. FNV-1a also detects accidental change but is not collision-resistant.

| Phase 1 idea | Verdict after DD-2 |
|---|---|
| R-44 review package | Confirmed; pins `ref2`, delta by default |
| R-45 review records | Confirmed; RV items, one file per review, outside the lock |
| R-46 signed approvals | Confirmed as opt-in offline `git verify-commit`, with a reviewer key map |
| R-47 automatic approval | Stays rejected (BR-004); requimd checks the record and the status against each other |
| R-48 hosted review app | Stays out of scope |
| R-80 spreadsheet round trip | Confirmed as the return channel into RV records (merged with DD-5) |
| R-81 print-ready site | Confirmed as part of the package |

## DD-3: The hash scope causes false suspects, and conflicts in the lock can heal themselves

**Evidence.** Doorstop keeps two hashes. The item's own fingerprint includes its link UIDs. The stamp a child stores per parent link is the parent's fingerprint *without* those links. Adding a parent link therefore never makes the children suspect ([doorstop item.py](https://github.com/doorstop-dev/doorstop/blob/develop/doorstop/core/item.py)). When Doorstop changed its algorithm to SHA-256 in 2.0, its migration advice was to run `review all` and then `clear all`, which wipes every open review ([changelog](https://doorstop.readthedocs.io/en/v2.1/about/changelog/)). A proposal to record the algorithm next to each hash was closed ([issue #358](https://github.com/doorstop-dev/doorstop/issues/358)).

ReqView configures suspicion per link type as a list of target attributes, such as `"suspect": ["heading", "text", "deleted"]` ([ReqView](https://reqview-homepage.netlify.app/doc/change-management)). Every enterprise tool limits suspicion to an allow-list of fields:

- DOORS Next: "Affects Link Validity";
- Jama: "Use to Trigger Suspect";
- R4J: "Monitor Fields".

DOORS Next also keys validity by content, so a revert restores the earlier status ([IBM link validity 7.2](https://www.ibm.com/docs/SSYMRC_7.2/com.ibm.jazz.vvc.doc/topics/c_linkval.html)). A PTC Integrity user could not find out what caused a suspect trace, and the answer was that this is not possible ([PTC community](https://community.ptc.com/t5/Integrity/Viewing-what-changed-when-clearing-suspect-trace/m-p/553247)). OpenFastTrace and tracey put revisions into the IDs instead of hashing. Neither documents an exemption for editorial changes ([tracey versioning](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md)).

The lock conflicts for a structural reason, not because of bad formatting. Two branches that each add an FR both change `FR = 504`, and both insert their new link lines at the end of the FR range. Each part has a safe meaning when two branches meet:

- `[ids]`: take the maximum.
- `[links]`: take the union, then reconcile against the merged items.
- A key with two different hashes: keep the one that equals the target's current hash; otherwise the link stays suspect.

That makes git's built-in `merge=union` driver workable ([gitattributes](https://git-scm.com/docs/gitattributes)), provided the loader tolerates duplicates.

**Proposal (P1 to P8).**

- **P1.** Take link keys and `prio` out of the item hash, sort the remaining metadata, and give the lock a version and a hash identifier (see "Before v0.1.0").
- **P2.** Suspect scope per link key, as a list of parts.
- **P3.** Commit `merge=union` in `.gitattributes`, add a tolerant loader, add `requimd lock resolve` for files with conflict markers, and fix the recovery hint.
- **P4.** `requimd review` appends reason, reviewer (from `git config user.name`) and date to an append-only log. A reason is required for `--all`.
- **P5.** An `--editorial` review on the target side, with guard rails:
  - it re-stamps only links that were clean before the edit;
  - a reason is mandatory;
  - editorial re-stamps are listed separately in `diff`;
  - a size warning appears above N changed words;
  - `--all --editorial` is not allowed.
- **P6.** Hover and `review --show` display the target's diff since the newest git revision whose content hash equals the stored hash.
- **P7.** No item-level review state for now.
- **P8.** FR-116 states that a revert restores the reviewed state and that suspicion runs in one direction only.

```text
$ git rebase origin/main
CONFLICT (content): Merge conflict in requimd.lock
$ requimd lock resolve
requimd.lock: 2 conflicts resolved
  ids FR: 505 / 506 -> 506
  TC-220 ver FR-104: kept c07b5e19d3a48f62 (matches current FR-104)
$ requimd review UC-012.s2 --editorial "typo: recieve -> receive"
Re-stamped 3 links as editorial; kept suspect (stale before): TC-231 ver UC-012.s2
```

| Phase 1 idea | Verdict after DD-3 |
|---|---|
| R-34 suspect scope per link type | Confirmed; the hash-scope fix (P1) must come first, before v0.1.0 |
| R-35 review with a reason | Confirmed; log file, required for `--all` and `--editorial` |
| R-36 item review state | Dropped for now; "changed after approval" goes to the approval anchor or the review records |
| R-41 PR impact | Confirmed; merged with the `diff --format delta` renderer |
| B6 show what changed | Confirmed and cheaper: no commit stored in the lock (preferred over DD-6's idea) |

## DD-4: One policy grammar covers steps, tests, code and results

**Evidence.** OpenFastTrace (OFT) IDs have the form `type~name~revision`. Items declare which artifact types must cover them (`Needs:`), and code tags look like `[impl->dsn~name~1]`. The target must always be a full OFT ID ([OFT system requirements](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md)). OFT and its Rust port cargo-ovft also read only OFT-style Markdown ([cargo-ovft](https://docs.rs/cargo-ovft)), so they would find nothing in requimd's items.

LOBSTER gives the clearest policy grammar. Its `requires` setting is a list of AND-groups, each a list of OR-alternatives, and its report marks items OK, Missing, Partial, Justified or Warning ([LOBSTER config](https://raw.githubusercontent.com/bmw-software-engineering/lobster/main/documentation/config_files.md)).

Tools match JUnit results to requirements in three ways:

- an explicit testcase property, such as Xray's `requirement` or `requirements`, or sphinx-test-reports' `verifies` ([Xray JUnit](https://docs.getxray.app/display/XRAYCLOUD/Taking+advantage+of+JUnit+XML+reports); [sphinx-test-reports](https://sphinx-test-reports.readthedocs.io/en/latest/configuration.html));
- the test's name;
- a separate results file keyed by requirement, as in GitLab's `requirements.json`.

Requirement status rolls up as the worst result of its tests: any failure means failed, otherwise anything not run means not run, otherwise passed; with no test at all, the requirement is uncovered. Xray and the pytest plugin jamb both use this order.

One constraint matters for requimd itself. **cargo test and cargo-nextest cannot emit testcase properties.** nextest writes one testcase per test, with the binary as `classname` ([nextest](https://nexte.st/docs/machine-readable/junit/)). Rust projects, requimd included, can therefore only match by ID in the test name (`fr_300_…`, the meter NFR-010 already uses) or by tags in the source.

JUnit 5 tags and Cucumber tags both accept `FR-012` ([JUnit tags](https://docs.junit.org/current/running-tests/tags.html); [Cucumber tags](https://cucumber.io/docs/cucumber/api/#tags)), and Cucumber tags are inherited by child elements. requimd has no regex crate and no XML parser today.

**Proposal.** Per-type rules, written as AND-groups of OR-alternatives, generalise FR-106 (steps need `impl`) and B10 (requirements need `ver`). An item can opt out of a rule with a `waive:` key and a reason in its body. A text-only scanner reads code and test files and recognises four kinds of reference:

- the native tag `requimd: ver FR-012`;
- bare IDs in comments, whose kind comes from the file path (`tests/**` verifies, `src/**` is `code`);
- Gherkin tags, with inheritance;
- optionally, OFT brackets as an alias only.

It needs no regex crate: a hand-written byte scanner and the parser's own ID function are enough. JUnit import matches by property first, then by TC ID, then by requirement ID in the test name. Results of committed files are hash-pinned, so they turn *stale* when the requirement changes. Two new matrix columns, "result" and "realised in", show the outcome.

```text
// requimd: ver FR-012, FR-013        native tag (any comment syntax)
// FR-006                             bare ID; kind from [trace] path default
@FR-012                               Gherkin tag above Feature or Scenario
fr_300_exit_code_0_without_errors     JUnit testcase name, matched to FR-300
```

| Phase 1 idea | Verdict after DD-4 |
|---|---|
| R-24 coverage policy | Confirmed; AND-of-OR grammar, merged with DD-5 rules into one `[[rules]]` mechanism |
| R-25 justified exceptions | Confirmed as a `waive:` key on the item (a new reserved key) |
| R-26 code tag scanner | Confirmed with native, bare-ID and Gherkin forms; OFT syntax is an alias, documented as syntax only |
| R-49 test-result import | Confirmed; JUnit plus a trivial `results.json`; answers B2's open matching question |
| R-50 Gherkin | Merged into the scanner |
| R-52 verification by method | Falls out of the rule filter `where = { method = … }` |
| R-53 CI gate | No new flag: rules are checks, and `--strict` (FR-302) gates them |

## DD-5: StrictDoc and Sphinx-needs show what to copy and what to avoid

**Evidence.** StrictDoc's schema is a grammar per document with four field types (String, SingleChoice, MultipleChoice, Tag). Reverse relation names live only in the grammar, through `REVERSE_ROLE` ([StrictDoc user guide](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc)). Its ReqIF export has a single profile, `p01_sdoc`. Without the optional machine IDs (MIDs), it generates random UUID identifiers on every export, which defeats re-import ([StrictDoc converter](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/strictdoc/backend/reqif/p01_sdoc/sdoc_to_reqif_converter.py)). The guide says an Excel round trip is "not yet supported", and its PDF output needs Chrome.

Sphinx-needs needed three generations of rule mechanism before it settled on declarative rules. It started with Python filter strings, then `needs_warnings`, which blocked incremental builds, then JSON-schema rules in 6.0.0 (September 2025) with three severities and a split between local and network rules. `target_types` arrived only in 9.1.0 (22 October 2026) ([changelog](https://sphinx-needs.readthedocs.io/en/latest/changelog.html)). requimd's `to = [...]` has offered the same since M1.

The market signal comes from ubCode. It is proprietary and free only for public repositories. Unlicensed private use is capped at 5 source files or 100 needs for most commands, and keys are validated online ([ubCode licensing](https://ubcode.useblocks.com/usage/ublicense.html)). **The editor experience is what that vendor charges for**, and requimd gives it away offline.

In ReqIF practice, DOORS Next requires identifiers to stay unchanged across exchanges and imports additively ([jazz.net best practices](https://jazz.net/wiki/bin/view/Deployment/ReqIFUsageBestPractices?rev=2)). Codebeamer needs the same `ReqIF.ForeignID` for round trips ([Codebeamer](https://support.ptc.com/help/codebeamer/r3.3/en/codebeamer/user_guide/ug_FieldValueManagement_RoundTrip_ReqIF_ExportImport.html)). Doorstop's spreadsheet import deletes and recreates items with no conflict check ([doorstop importer.py](https://raw.githubusercontent.com/doorstop-dev/doorstop/develop/doorstop/core/importer.py)).

**Proposal.** The proposal extends `requimd.toml` in place, without an expression language:

- typed keys (`enum`, `list`, `text`, `int`, `date`, with `pattern` limited to Rust regex syntax, and `ordered` for enumerations);
- link properties (`inverse`, `from`, `max`, `acyclic`);
- declarative rules with the existing severities.

The JSON export becomes a flat list of records, each with its source location, plus an embedded schema, computed backlinks and a hash per item (R-76). It stays reproducible and has a documented, versioned contract.

The ReqIF 1.2 export is minimal, one-way and deterministic:

- identifiers derived from requimd IDs, such as `requimd-FR-012`;
- the four ProSTEP attributes (`ReqIF.ForeignID`, `ReqIF.Name`, `ReqIF.Text`, `ReqIF.ChapterName`);
- one enumeration per value set;
- one SPECIFICATION per file;
- a conservative XHTML subset.

Import would at most be a one-off converter. The CSV-first review round trip guards each row with its hash: a row whose hash no longer matches is a conflict and is never applied. The import writes a review record and never writes text or status.

| Phase 1 idea | Verdict after DD-5 |
|---|---|
| R-02 named inverse links | Confirmed; `inverse` lives only in the schema |
| R-03 topology constraints | Confirmed as `acyclic` and `max` on `[links.<key>]` |
| R-04 field patterns | Confirmed as typed keys |
| R-08 stable machine IDs | Stays low: BR-002 already keeps IDs stable |
| R-19 declarative rules | Confirmed, merged with R-24 |
| R-73 ReqIF | Kept, but only once a real exchange partner exists |
| R-76 hash in JSON | Confirmed |
| R-80 spreadsheet round trip | Confirmed; hash per row, CSV first, XLSX via `rust_xlsxwriter` and `calamine` later |

## DD-6: Against tracey, SARA and Reqvire, the analyst focus holds

**Evidence.** All three neighbours check structure only: references, duplicates, cycles, orphans and staleness. None checks wording, and none mentions German. Reqvire has a SKOS thesaurus whose `altLabel` entries act as synonyms. Its requirements cite concepts through an explicit "Concept References" list, and nothing scans the prose for terms ([Reqvire ontologies](https://www.reqvire.org/ontologies); [concept.rs](https://github.com/reqvire-org/reqvire/blob/main/crates/reqvire-core/src/concept.rs)).

tracey is the only one with a language server, and its features match requimd's almost item for item. It adds three: a hover with a version diff, a "Did you mean" fix for unknown IDs, and document highlight ([tracey lsp.rs](https://github.com/bearcove/tracey/blob/main/crates/tracey/src/bridge/lsp.rs)). All three neighbours ship MCP servers:

- SARA: 8 tools over stdio, including create and update ([sara-mcp](https://github.com/cledouarec/sara/blob/main/sara-mcp/README.md));
- tracey: 11 tools over stdio;
- Reqvire: about 28 tools, HTTP by default, with mutations only behind a flag.

Reqvire computes the impact of a PR against its merge-base and prints a checklist of invalidated verifications. It keeps no review state ([change_impact.rs](https://github.com/reqvire-org/reqvire/blob/main/crates/reqvire-core/src/change_impact.rs)). Only requimd keeps a durable review state per link.

All three are effectively one-maintainer projects:

- tracey: 259 stars; last release v1.4.0 in May 2026, with binaries for macOS and Linux only;
- SARA: 107 stars, frequent releases;
- Reqvire: 30 stars; it closed code pull requests and is turning towards ontologies ([GitHub API](https://api.github.com/repos/bearcove/tracey); [Reqvire](https://github.com/reqvire-org/reqvire)).

**Verdict and ideas.** Four differentiators are confirmed: German and English language checks, an analyst-oriented format, use case steps as link targets, and a persistent review state per link. Three more emerged: a static site in the releases, a dedicated VS Code extension, and Windows as a first-class platform.

The glossary differentiator holds only with narrower wording: **"a glossary that checks the prose"**, since Reqvire has a glossary-like thesaurus.

Ideas worth taking:

- the "Did you mean" fix (FR-202, FR-219);
- a coverage code lens on items (R-31);
- cycle and orphan checks, and Reqvire's redundant-link lint as info;
- SARA's names `satisfies` and `supersedes` for the design level;
- a PR impact checklist that the owner can paste into a PR description.

```text
$ requimd diff --merge-base main --format delta
## MODIFIED Requirements (requirements/functional/export.md)
### FR-400 Static site
Suspect impact:
- [ ] TC-413 ver FR-400 (FR-400 changed)
- [ ] TC-414 ver FR-400 (FR-400 changed)
```

## DD-7: Agents need a deterministic layer, but `check` writes the lock

**Evidence.** Spec Kit, Kiro and OpenSpec all store Markdown specs in the repository, and all three changed noticeably during 2026. Spec Kit renamed its commands to `/speckit-*` and removed a core command in 1.3.0 ([releases](https://github.com/github/spec-kit/releases)). OpenSpec replaced its v1 commands in 2.x. Tessl reportedly turned its registry into a skills registry (secondary sources only).

**Only OpenSpec has a deterministic validator**, and it checks structure alone: SHALL or MUST present, scenarios present, delta conflicts, scenarios lost ([validator.ts](https://raw.githubusercontent.com/Fission-AI/OpenSpec/main/src/core/validation/validator.ts)). Spec Kit's `analyze` is a prompt that the agent executes, so its coverage table is re-inferred by a language model on every run ([analyze.md](https://raw.githubusercontent.com/github/spec-kit/main/templates/commands/analyze.md)). None of these tools keeps stable project-wide IDs. Spec Kit restarts `FR-001` in every feature folder ([spec template](https://raw.githubusercontent.com/github/spec-kit/main/templates/spec-template.md)), which collides with requimd IDs in the same repository.

The current MCP revision, 2026-07-28, is stateless. It drops the `initialize` handshake, makes `server/discover` mandatory, deprecates roots, and keeps stdio as a standard transport ([MCP changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)). Tool annotations such as `readOnlyHint` are only hints, and `readOnlyHint` defaults to false ([schema](https://modelcontextprotocol.io/specification/2026-07-28/schema)). Jama's official MCP server launched on 4 May 2026 with Jama Connect 9.35, but its tool list is not public ([Jama press release](https://www.jamasoftware.com/press/jama-software-launches-model-context-protocol-mcp-server/)).

The evidence that analysts, rather than developers, benefit is thin. In a May 2026 podcast, Böckeler still finds it unclear whether spec-driven development is for developers, product managers or analysts ([Thoughtworks podcast](https://www.thoughtworks.com/en-gb/insights/podcasts/technology-podcasts/what-is-spec-driven-development)).

**The hidden-write finding.** FR-116 records new links on the first check, so a "read-only" MCP check tool would write `requimd.lock`. The code confirms that `requimd check` writes the lock unless `--locked` is given (`crates/requimd-cli/src/main.rs`), and that the language server writes it on save (`crates/requimd-lsp/src/server.rs`). `--locked` is already a mode that writes nothing, but it *fails* when the lock is out of date. That is right for CI and wrong for an agent's query. A read-only check that reports unrecorded links as info is needed.

**Proposal**, in order:

1. A generated AGENTS.md section between markers.
2. `requimd context <ID>`, Markdown first, using FR-404's definition of neighbours.
3. `requimd diff --format delta`, with OpenSpec-style headings and suspect impact.
4. `requimd mcp`, over stdio only, with read-only tools.
5. Write tools behind `--allow-writes`, which only propose changes.
6. Recipes for Kiro hooks, Spec Kit and OpenSpec that cite requimd IDs.

There are no importers of those tools' specs. A new ADR records the agent interface and the data-egress caveat: requimd sends nothing, but the user's agent host may pass what requimd returns on to a cloud model.

| MCP tool | Purpose | Class |
|---|---|---|
| `requimd_get_item`, `requimd_search`, `requimd_trace` | Item, search with glossary aliases expanded, trace up or down | read |
| `requimd_suspects`, `requimd_check`, `requimd_coverage` | Suspect links; diagnostics **without writing the lock**; step coverage | read |
| `requimd_diff`, `requimd_context`, `requimd_glossary` | Delta diff (refs validated, `--` separator); context bundle; terms | read |
| `requimd_project_info` | Types, prefixes, link keys, statuses, BR-002 to BR-004, **next free ID per range** | read |
| `requimd_propose_item`, `requimd_propose_change` | Only with `--allow-writes`; status forced to `draft`; refuse ID changes and `approved` | write |

Deliberately not exposed: `review`, approving, deleting, renumbering, and committing.

```markdown
# Context: FR-012 Tolerant parsing
> requimd 0.1.0 · format 1 · commit 712e3ef (working tree clean)
> Status: approved · prio: must · requirements/functional/format.md:130
## Item
## Upstream (this item implements or refines)
## Downstream (implements, refines or verifies this item)
- TC-012 · link `ver` · suspect: **yes**, the target changed since its last review
## Glossary terms used
## Open diagnostics
## Optional          (depth-2 neighbours; cut first by --max-bytes)
```

| Phase 1 idea | Verdict after DD-7 |
|---|---|
| R-86 local MCP server | Confirmed; stdio only; needs a no-write check mode (a change to FR-116) |
| R-87 MCP propose tools | Confirmed as opt-in; the owner decides whether they are needed at all |
| R-88 context export | Confirmed; deterministic, cut from the tail by a byte budget |
| R-89 AGENTS.md section | Confirmed; Claude Code is not on the agents.md list, so the guide must show how to reference the block from `CLAUDE.md` |
| R-90 prompt export | Partly absorbed by MCP prompts (later) |
| R-91 opt-in LLM commands | Unchanged: each needs its own ADR under ADR-0011 |

## DD-8: German rules need their own logic, and SOPHIST changed its keywords

**Evidence.** The current INCOSE guide is still GtWR v4 from 2023, with 42 rules in 14 categories and 15 characteristics ([reqi.io](https://reqi.io/articles/incose-requirements-quality-42-rule-guide); [Jama](https://www.jamasoftware.com/requirements-management-guide/writing-requirements/incose-requirements-writing-guide/)). Rule numbers differ between editions, so a finding must cite the edition along with the number. The current SOPHIST MASTeR brochure (6th edition, July 2024) differs from the 2009 handout that phase 1 used. Bindingness is now **MUSS, SOLLTE and WIRD**, and KANN is not a bindingness word. The ambiguous "wenn" is split into **FALLS** (logical), **SOBALD** (event) and **SOLANGE** (period) ([Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/); [brochure excerpt](https://silo.tips/download/die-sophistenschablonen-fr-alle-flle-2)).

German needs different logic, not translated lists:

- "soll" behaves like a loophole;
- "muss nicht" means "need not", while the prohibition is "darf nicht";
- "wird" is either the intention level or the passive auxiliary;
- all nouns are capitalised;
- adjectives inflect into many forms, and compounds hide both weak words and glossary terms.

The best published precision figures come from German industrial specifications. Dictionary-based smells scored 0.70 to 0.96, but checks based on grammar tagging scored far lower: comparatives 0.48, negation 0.33, pronouns 0.26 ([Femmer et al.](https://ar5iv.labs.arxiv.org/html/1611.08847)). A German weak-word lexicon reached an F1 above 0.95, but it cannot be downloaded ([Krisch et al., LREC 2016](https://aclanthology.org/L16-1454/)).

No checked commercial tool checks German against INCOSE rule IDs. IBM's RQA is English- and ASCII-only ([IBM](https://www.ibm.com/support/knowledgecenter/SS3UPN/com.ibm.help.rm.assist.doc/topics/t_running_rqa.html?view=kc)), QVscribe lists English only, and Fraunhofer's German toolkit is a demonstrator without rule citations ([Fraunhofer FKIE](https://www.fkie.fraunhofer.de/en/departments/itf/rat.html)). Qualicen and Congree were not verified.

**Proposal.** About 20 further INCOSE rules can be checked deterministically in both languages with word lists and simple clause patterns. Five rules have German-specific value that no checked vendor offers:

| Rule | German detection | Precision risk |
|---|---|---|
| Bindingness against `prio` (SOPHIST) | muss to `must`, sollte to `should`, wird to `could`; KANN and "soll" flagged | low |
| "muss nicht" (supports INCOSE R16) | suggest "darf nicht" | low |
| Passive (R2) | form of werden plus a clause-final participle; "ist zu" plus infinitive; "lässt sich"; separates passive from WIRD | low to medium |
| Nominalisation (R2, SOPHIST) | list of function-verb phrases (erfolgt, findet statt, wird durchgeführt), not every -ung noun | low to medium |
| German EARS / MASTeR conditions | Solange / Sobald / Falls / Sofern, with the modal verb first in the main clause | medium |

No German translation of EARS exists, so the proposal aligns with the SOPHIST keywords German analysts already learn: *Solange <Zustand>, muss das System …* for state-driven requirements, *Sobald <Auslöser>, muss …* for event-driven ones, *Falls <Bedingung>, (dann) muss …* for unwanted behaviour, and *Sofern <Merkmal> vorhanden ist, muss …* for optional features. "Wenn" gets an info hint. Checks based on grammar (negation R16, pronouns R24, comparatives R33, combinators R19) default to info or off. Negation inside a condition clause is skipped, which removes the largest source of false positives that Femmer found. Every finding cites its rule (R-09):

```text
"möglichst" is an escape clause; state the condition explicitly (INCOSE GtWR v4 R8)
```

```toml
[checks.weak-word]
words = [ { w = "möglichst", why = "escape clause", rule = "INCOSE-GtWR-v4:R8" } ]
```

`Diagnostic.code` stays the check ID. `codeDescription.href` points to a rule catalogue in the documentation, which is only shown and never fetched, so NFR-006 holds. The draft German word lists in the DD-8 notes need the owner's validation, because most entries are translations of INCOSE examples rather than entries from a German source.

| Phase 1 idea | Verdict after DD-8 |
|---|---|
| R-09 rule provenance | Confirmed; the edition is part of the reference (`INCOSE-GtWR-v4:R8`) |
| R-10 INCOSE subset | Confirmed and sorted into lexical, pattern and set-level rules |
| R-11 German EARS | Changed: Solange, Sobald, Falls, Sofern; "dann" optional; "Wenn" gets a hint |
| R-12 Rupp check | Changed: MUSS, SOLLTE, WIRD (not "soll"); KANN flagged |
| R-13 passive and nominalisation | Confirmed, but with a list of function-verb phrases, not every -ung noun |
| R-14 German glossary matching | Still open: it conflicts with ADR-0005's explicit forms (Q-36) |
| R-16 quality score | Unchanged; enabled by the `rules` field in the JSON export |
| R-23 bundled spell check | Stays out; point users to LanguageTool or ltex-ls |

## One design from eight proposals

The deep dives were written in parallel, so their proposals overlap in five places. This section makes one choice for each and says where proposals were merged or chosen between.

### Link properties live in one table per link key

DD-3 proposed `[suspect.<key>]` tables, DD-5 `[links.<key>]` with `inverse`, `from` and `max`, DD-6 named reverse links, and DD-1 nine new design-level keys. **Merged:** all properties of a link kind live in its existing `[links.<key>]` table, with `suspect` as one property. A project-wide `[suspect]` table keeps only the default parts and the metadata keys to ignore. This keeps one place per link kind, which is easier for an analyst than two tables with the same key.

**Chosen:** the design-level keys are DD-1's set. They follow requimd's convention that the downstream item carries the link and its inverse is computed. SARA's `satisfies` and `supersedes` (DD-6) are the same words, and its `justifies` is not adopted, because `addresses` and `driven-by` cover it in the right direction. Code references get their own kind, `code` (DD-4), so that `impl` keeps its meaning under FR-015 ("implements a use case step").

### One rule mechanism serves coverage and conditional checks

DD-4's `[coverage]` with `[[coverage.rule]]` and DD-5's `[[rules]]` with `select` and `incoming` describe the same thing. **Merged into one `[[rules]]` list.** It uses DD-4's AND-of-OR `needs` grammar, which is more expressive than DD-5's `incoming = { ver = { min = 1 } }`, and DD-5's `require` for conditional required keys. Severity uses `sev`, as FR-117's tables already do, not DD-5's `severity`. Statuses to skip are set per rule, with `deprecated` always skipped. Rules go one hop only, and deeper coverage, as in OFT, comes later.

DD-1's status checks (`superseded-target`, `invalid-assumption`, `open-question-gate`) stay built-in checks with IDs, because they depend on the status phases of each type. Whether a suspect link still counts as coverage (DD-3 Q6, tracey's "stale is not covered") becomes a per-rule option.

```toml
[links.ver]
name = "verifies"
inverse = "verified by"                 # DD-5, DD-6 (R-02)
from = ["test"]                          # DD-5
to = ["functional", "quality", "story"]  # exists today
suspect = ["title", "body"]              # DD-3 P2

[links.addresses]                        # DD-1, after the traceability ADR
inverse = "addressed by"
from = ["decision"]
to = ["usecase", "step", "functional", "story", "rule"]

[suspect]
parts = ["title", "meta", "body"]        # meta: no status, prio or link keys
ignore-meta = ["owner"]

[[rules]]
id = "unverified"                        # B10, DD-4
for = ["functional", "quality", "story"]
needs = [[ { link = "ver", from = "test" } ]]
skip-status = ["draft"]
sev = "info"

[[rules]]
id = "must-needs-test"                   # DD-4 filter, DD-5 select
for = ["functional", "quality"]
where = { prio = "must" }
needs = [[ { link = "ver", from = "test", where = { method = ["test", "inspection"] } } ]]
sev = "warning"
```

### Four kinds of review evidence get four homes

Four proposals touch review evidence:

- DD-2's review records `reviews/RV-nnn.md`;
- DD-3's `requimd.reviews` log of cleared suspects;
- DD-5's spreadsheet import, which writes `reviews/R-2026-11-18.md`;
- DD-1's `[accepted]` lock section for immutable decisions.

They answer four different questions, so they are kept apart deliberately, but they share one hash function:

| Question | Home | Written by | Source |
|---|---|---|---|
| Is this link still valid after its target changed? | `requimd.lock` `[links]` | check (new links), review | ADR-0007 |
| Who cleared which suspect link, when, and why? | `requimd.reviews`, append-only, `merge=union` | `requimd review` | DD-3 P4 |
| Has an approved or accepted item changed since its approval? | `requimd.lock` `[approved]`, one line per item in an approved phase | check, the first time it sees the item approved; editorial action | DD-1 generalised |
| Who reviewed and approved exactly this text, with what meaning? | `reviews/RV-nnn.md`, one RV item per review | human, or `review-record import` | DD-2, DD-5 |

**Chosen:** DD-2's naming and RV numbering from `[ids]` (BR-002) over DD-5's dated file names. DD-5's `export review` becomes the sheet inside DD-2's review package, and DD-5's import becomes `review-record import`. The verdict vocabulary is left to the owner (Q-19). The log is kept separate from RV records, against DD-3's suggestion to fold them together. Clearing suspects is routine, happens per link and in large numbers, while RV records are rare, deliberate and per item. Mixing them would bury the evidence.

**Merged and generalised:** DD-1's `[accepted]` section becomes an `[approved]` anchor for every item whose status is in an "approved" phase: an accepted DEC, an approved FR, a validated ASM. This answers the question DD-3 deferred ("changed after approval") without the ceremony of a second review. It needs no review records, so teams outside regulated domains get it for free. It also keeps BR-004 intact, because a human re-approves by setting the status. When an item leaves the approved phase, its anchor is removed; when it is approved again, the anchor is recorded again.

### The lock file gets a header, an anchor section and two neighbours

The merged lock design combines several sources:

- DD-3's hash scope, version header and merge rules;
- DD-1's anchor, generalised as above;
- DD-4's code-tag links (suspect-tracked only when configured, because they grow the lock);
- DD-4's result staleness, in a separate file;
- DD-7's split between writers and read-only callers.

DD-6's idea of storing the commit of the last review is **not adopted**. DD-3's content-keyed lookup in git history gives the same hover diff without extra lines that churn the lock.

```toml
# requimd.lock: generated by requimd, commit it.
version = 1
hash = "fnv1a64-v1"                     # names the algorithm and normalisation

[ids]
FR = 504
RV = 3

[links]
"FR-101 impl UC-012.s1" = "8f3a1c0e5b7d2946"
"TC-220 ver FR-104" = "suspect"          # migrated, awaiting review

[approved]                               # later: lock version 2, with the traceability ADR
"DEC-004" = "5b1e0c9a7d23f418"
```

```text
# .gitattributes
requimd.lock     merge=union
requimd.reviews  merge=union
```

The writers are `check` (without `--locked`), the language server on save, `review`, `rename` and `fmt` (D6). Read-only callers are `check --locked`, MCP tools, `context`, `diff` and `review-package`. FR-116's sentence "recorded silently on their first check" must say which callers record. Test results that are pinned to hashes go into a sibling file, `requimd.results`, not into the lock, for the same reason review records stay out of it.

### Status names become per-type sets mapped to four phases

FR-015 fixes `draft`, `review`, `approved` and `deprecated` and says that a later traceability model replaces it. DD-1 needs per-type values. DD-1's question gates need an order. DD-2's "approved without record" rule needs to know which values mean approved, and DD-4's skipped statuses need the same. **Merged:** each type declares its values and maps them onto the four phases that FR-109, FR-116, FR-400, FR-403 and FR-404 read. The order of the phases gives the gate order. BR-004 generalises to "no tool sets a value in the approved phase".

```toml
[types.decision]
prefix = "DEC"
status = { draft = "draft", proposed = "review", accepted = "approved", rejected = "deprecated", superseded = "deprecated", deprecated = "deprecated" }
```

Three smaller merges complete the design:

- **Diff renderer.** DD-6's PR checklist, DD-7's `--format delta`, DD-2's impact section in the review package and phase 1's R-41 become one renderer in `requimd diff`, which the review package reuses.
- **MCP tools.** DD-6's SARA-based tool list and DD-7's list merge into DD-7's table. Write tools are gated by `--allow-writes`, with Reqvire's `dry_run` pattern.
- **Tag aliases.** DD-4's OFT alias and DD-6's tracey (`r[impl FR-006]`) and StrictDoc (`@relation`) aliases become one opt-in setting, `[trace] aliases = [...]`, after the native forms.

## Before v0.1.0

I verified DD-3's claims against the code on the branch `spec/requirements-validation`, read-only. `lock.rs` there is identical to `origin/main`. The FR-116 wording on HTML comments, ADR-0010 and ADR-0011 exist only on this unmerged branch. In this repository's own lock, **266 of the 358 link entries target items** and would get a new hash if the scope changed. The 92 entries that target use case steps hash only the step text and are unaffected.

| Claim | Result | Evidence |
|---|---|---|
| The item hash includes the target's own link lines | **Confirmed.** `item_content()` appends every metadata pair except `status`; `Item.meta` is documented as "including unknown keys and link keys"; the test `fr_116_item_hash_covers_title_metadata_body_but_not_status` asserts that changing `impl: UC-001.s1` changes the hash | `lock.rs`, `model.rs` |
| The hash includes `prio` and the metadata order | **Confirmed.** Pairs are concatenated as written; unknown keys, `check:` and `sev:` count too | `lock.rs` |
| The code matches the spec | **Literally yes, which is the problem.** FR-116 and ADR-0007 both say "the metadata except `status`". The cascade is a design gap in the spec, not a coding bug | FR-116, ADR-0007 |
| HTML comments are outside the hash | **Spec and code differ.** FR-116 (on the branch) and ADR-0010 (proposed) exclude them; `item_content()` hashes the raw body slice and the raw step text. This is a known, planned gap: M7 step 3 lists "Hash without HTML comments; version field in the lock file" as its first item | `lock.rs`, roadmap M7 |
| The lock parser uses `deny_unknown_fields` | **Confirmed.** A top-level `version = 1` or a new `[approved]` table makes every alpha binary fail; the test `fr_300_conflicted_lock_is_an_error` asserts that an `[other]` table is an error | `lock.rs` |
| The same applies to `requimd.toml` | **New finding, confirmed.** `config.rs` uses `deny_unknown_fields` on every table, `[links.<key>]` included (only `name` and `to` are allowed). Every proposed extension (`inverse`, `from`, `suspect`, `[[rules]]`, `[keys]`, `[trace]`) is a hard error in a v0.1.0 binary | `config.rs` |
| The error hint tells users to delete an unreadable lock | **Confirmed verbatim:** "Resolve merge conflicts in requimd.lock, or delete it to record all links anew." | `Lock::load` |
| Deleting the lock clears suspects and risks reusing IDs | **Confirmed by reading `updated()`.** Starting from an empty lock, every current link is recorded with the current hash, so open suspects disappear. `[ids]` restarts from the highest ID still present, so an ID deleted at a higher number can be assigned again, against BR-002 | `lock.rs` |
| `check` writes the lock | **Confirmed**, unless `--locked` is given; with `--locked` it writes nothing but exits 1 when the lock is out of date. The language server also writes the lock | `main.rs`, `server.rs` |
| No `.gitattributes`, no per-item hash, no review reason | **Confirmed** | repository root, `lock.rs`, FR-307 |

**The stability ADR (E4) has not been written.** The roadmap lists it under M7 step 2 as open and makes "the stability ADR is accepted" a condition of v0.1.0. The feature report describes E4 as stability promises for the format, `requimd.toml`, `requimd.lock` and the JSON export, a version field in the lock, and the hash frozen after D7. The lock version field is planned in the same implementation row as the comment fix. Nothing yet says how the version will be used.

I recommend that E4 settle the following, because each is costly to change after v0.1.0:

1. **One hash change, not two.** Remove link keys and `prio` from the item hash and sort the remaining metadata, in the same release that removes HTML comments. ADR-0010 already accepts a one-time change for items with comments. Bundling makes it the only migration, before any non-alpha user exists.
2. **A lock header that is read first.** Use `version = 1` and `hash = "fnv1a64-v1"`. A binary that finds a higher version should report "requimd.lock version 2 needs requimd 0.x or newer", not a TOML error followed by "delete it". DD-3's migration rule can wait until the hash changes again: entries matching the old scheme get the new hash, and all others become `"suspect"`.
3. **A forward-compatibility rule for `requimd.toml`.** Either keep failing on unknown keys, but with a message naming the minimum version, perhaps through a `requimd = ">=0.2"` key in `[project]`; or warn and ignore. Without a rule, every later configuration feature breaks older binaries in shared repositories.
4. **A new recovery hint, now**, in a `fix/` PR. It should never suggest deleting the lock. It should point to resolving the conflict, and later to `requimd lock resolve`.
5. **Union merge only together with a tolerant loader.** `merge=union` produces duplicate keys, which the current TOML parser rejects. Ship both in v0.1.0 if the effort fits, or neither.
6. **The scope of the promise.** FR-015's status names and the structural names are current behaviour, not a lasting promise (F4), so the traceability model can replace them. If CLI command names are covered, decide before v0.1.0 whether `requimd review` keeps meaning "clear suspect links" (DD-2 Q11), since content reviews will want that word.

Still unverified: that `merge=union` applies during `git rebase` (inferred from the gitattributes documentation, not tested); the exact exit-code path for an unreadable lock (ADR-0007 states exit code 2; I did not trace the code); and whether any alpha user outside the owner has a lock file that a migration would affect.

## Recommended roadmap

This is a proposal for the product owner, not a plan to set any item to `approved`. Effort follows phase 1: S is up to a day, M a few days, L a week or more. "Next" items already in the roadmap's "After v0.1.0" section are folded in where they fit.

| Milestone | Theme and content | Requirement and feature IDs | ADRs | Effort |
|---|---|---|---|---|
| **M7 additions (v0.1.0)** | Hash scope (link keys, `prio`, order) together with comment removal; lock header with `version` and `hash`; a clear message on a newer lock or config; new recovery hint; optionally a tolerant loader plus `merge=union`; promise scope (FR-015 names, the `review` command name) | FR-116, FR-300, FR-307, FR-015, BR-002, E4, D7 | E4 stability ADR (new); ADR-0007 amendment or a "lock file format 1" ADR superseding ADR-0007 and ADR-0010; accept ADR-0010 | S to M |
| **M8 (v0.2): a change signal analysts can trust** | Suspect scope per link key; `lock resolve`; review reason and log; `--editorial`; show what changed (hover, `review --show`); `suspect` highlighting modifier; B10 as the first rule (info); named inverses; "Did you mean"; cycle and orphan checks; `diff --format delta` with impact checklist and merge-base; rule provenance in messages; German example config with MUSS, SOLLTE, WIRD | FR-116, FR-117, FR-218, FR-307, FR-205, FR-202, FR-219, FR-216, FR-400, FR-403, FR-108, FR-110, FR-118; B6, B7, B10; R-02, R-09, R-34, R-35, R-41 | ADR-0007 amendment (merge strategy, review log, editorial); ADR-0005 amendment (provenance) | L |
| **M9 (v0.3): better writing for analysts and their agents** | German rule pack (passive, "muss nicht", function-verb phrases, bindingness against `prio`, R8, R9, R10, R26, R32, R35, R37, R38, R40) with low-precision rules at info; EARS in English and German; file language setting; AGENTS.md section; `context`; read-only `requimd mcp`; check mode without lock writes; next-free-ID query | FR-110 to FR-114, FR-118, FR-112, FR-116, FR-303, FR-305, FR-404; new FRs in the 300 and 400 ranges; A2, B5; R-10 to R-13, R-86, R-88, R-89 | Agent interface ADR (new); ADR-0009 amendment (preset or example) | L |
| **M10 (v0.4): decisions and open questions** | Traceability-model ADR; per-type statuses mapped to phases; DEC and Q types; design-level link keys; status checks (`superseded-target`, `open-question-gate`, `answer-trace`, `decision-re-evaluate`); `[approved]` anchor and `changed-after-approval` (lock version 2); decision log on the site | FR-015 (replaced), FR-014, FR-109, FR-116, FR-400, FR-401, FR-402, FR-006 (option labels), BR-003, BR-004; B9, F3, F4; R-54 to R-60, R-63, R-66, R-69 | Traceability-model ADR (new) | L |
| **M11 (v0.5): coverage you can measure** | `[[rules]]` with waivers; typed keys and link properties (`from`, `max`, `acyclic`); offline tag scanner (native, bare ID, Gherkin); JUnit and `results.json` import with hash-based staleness; coverage report and matrix columns; coverage code lens; JSON export contract with schema, backlinks and hashes | FR-106, FR-306, FR-401, FR-402, FR-209, FR-215, FR-103, FR-104, NFR-010; B2, B10; R-03, R-04, R-19, R-24 to R-31, R-49 to R-53, R-76 | Coverage and tracing ADR (new); XML dependency (amend ADR-0003) | L |
| **M12 (v0.6): reviews beyond git** | `review-package` for a git range; RV records with staleness; CSV return sheet and import; opt-in signature check; named baselines and the changelog page | FR-403, FR-400, FR-307 (renamed?); B13; R-37, R-38, R-44 to R-46, R-80, R-81 | Review-evidence ADR (new), including the hash strength for records | L |
| **Later or on demand** | ASM and SPK (DD-1 step 2); DES (step 3); MCP write tools; XLSX; ReqIF export when a partner exists; one-off ReqIF import; `needs.json` or LOBSTER adapters; tag aliases (OFT, tracey, StrictDoc); multi-repo (R-33); German inflection and compound matching | R-61, R-62, R-67, R-73, R-87, R-14, R-33; A1, B1, B3, B4 | Separate ADRs as needed (ADR-0011 for anything networked) | M to L each |

The order is a judgement, and it rests on two assumptions. First, M8 and M9 strengthen what requimd already sells (trustworthy suspects, language checks, agent access) at low risk. Second, M10 opens the niche only once the change signal it depends on is clean. If the owner values the design level above agent access, M9 and M10 can swap without breaking dependencies. The exception is the check mode without lock writes, which should then move into M8.

## Decisions for the product owner

The 39 questions below consolidate about 80 open questions from the eight notes. Each has the options, a recommendation and the deep dive it comes from. The Q numbers are local to this report: the testing guide already uses Q5 to Q10.

### Before v0.1.0: hash, lock and stability

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-01 | Should the target's own link lines and `prio` leave the item hash, with metadata hashed sorted? | Yes, in v0.1.0; yes, later with a migration; no | Yes, in v0.1.0, bundled with the HTML-comment change | DD-3 |
| Q-02 | How should the lock version be used? | Header only; header plus migration rule now; no header | Header (`version`, `hash`) now; migration logic when the hash changes again | DD-3 |
| Q-03 | What happens when an older binary meets newer `requimd.toml` or `requimd.lock` fields? | Fail with a minimum-version message; warn and ignore; fail as today | Fail clearly with the minimum version; never suggest deleting | new (code check) |
| Q-04 | Which merge strategy for the lock, and when? | `merge=union` plus tolerant loader plus `lock resolve`; a custom merge driver; none | Union plus tolerant loader (v0.1.0 if it fits, else v0.2); new hint in v0.1.0 regardless | DD-3 |
| Q-05 | Does `requimd review` keep its meaning (clear suspect links) once content reviews arrive? | Keep `review`; rename to `confirm` with `review` as an alias; give content reviews other names | Decide in E4; rename with an alias if CLI names are part of the promise | DD-2 |

### Suspect links and clearing

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-06 | Which default suspect scope, and in which direction? | All parts for every key; `ref` never suspect; downstream only, or both directions | Every key stays suspect until configured; confirm downstream only | DD-3 |
| Q-07 | How is clearing recorded? | `requimd.reviews` log; folded into RV records; nothing. Identity from `git user.name`; commit of the last review in the lock; CODEOWNERS on the lock | Separate log, `git user.name`, no commit in the lock, CODEOWNERS on `requimd.lock` and `requimd.reviews` in this repository | DD-3, DD-6 |
| Q-08 | Should there be an editorial escape at all? | No; yes with a size warning; yes with a size block | Yes, with a warning, a mandatory reason and its own section in `diff` | DD-3 |

### Design level

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-09 | What is the scope of step 1, and do design elements come at all? | DEC and Q; DEC, Q and ASM; DES or leave design to code tracing | DEC and Q first; ASM in step 2; DES only after code tracing exists | DD-1 |
| Q-10 | Which prefixes, and should requimd's own ADRs become items? | DEC or ADR; Q, OQ or QST; ASM, SPK, DES; read `docs/decisions/` now or later | DEC, OQ (avoids clashes with existing Q numbering), ASM, SPK, DES; own ADRs later (R-70) | DD-1 |
| Q-11 | Which link vocabulary? | DD-1's nine keys; merge `addresses` and `driven-by`; SARA's names including `justifies` | DD-1's keys, both `addresses` and `driven-by`; SARA spelling for `satisfies` and `supersedes`; no `justifies` | DD-1, DD-6 |
| Q-12 | How strict is immutability, and where is the anchor? | Strict; with an editorial escape; none. Anchor in an `[accepted]` lock section; a generalised `[approved]` anchor; RV records | Generalised `[approved]` anchor with an editorial escape; `changed-after-approval` as a warning | DD-1, DD-3 |
| Q-13 | How do question gates work? | `gate:` names a status of the target type; a fixed gate; no gates. Severity warning or error | Status name of the target type, ordered by phase; warning by default, error configurable | DD-1 |
| Q-14 | Which optional decision features? | `kind` value set; Y-statement form and clause check; option labels `o1`, `o2`; `revisit:` and `check-by:` with `--as-of`, or only `trigger:` | `kind` yes; Y-statement optional without a check at first; option labels yes (extends BR-003); both dates and `trigger:`, dates opt-in | DD-1 |

### Status and approval

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-15 | Should per-type status sets replace FR-015, and is realisation stored? | Per-type sets mapped to four phases; keep FR-015's four names; realisation computed from `ver`, or stored (`stage:`) | Per-type sets with phases in the traceability ADR; realisation computed | DD-1 |
| Q-16 | Who may set approved-phase values? | BR-004 for all approved-phase values; product owner for `accepted`, any analyst for `validated` and `answered`; AI-created items forced to `draft` | Product owner for `accepted` and `approved`; analysts for `validated` and `answered`; agents create only `draft` or `proposed` | DD-1, DD-7 |
| Q-17 | Is a merged PR still the approval, and does requimd warn on approval without a record? | Merge is approval; an RV record is needed in a regulated profile; warn always, in a profile only, or never | Merge stays the approval by default; record rule opt-in in a regulated profile | DD-2 |

### Review evidence

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-18 | What form do review records take? | RV items, one file per review; one file per item and review; a separate file kind | RV items, one file per review, outside the lock | DD-2, DD-5 |
| Q-19 | Which verdict values, which package mode, and who owns the checklist? | approve, reject, abstain; plus "approve with comments" or "needs work"; DD-5's ok, change, reject, question; "approve as package". Delta or full package. Checklist in a project file or a rule profile | approve, approve with comments, reject (needs a finding), abstain; delta by default with `--full`; checklist as a project file, with automated findings listed separately | DD-2, DD-5 |
| Q-20 | How strong must evidence be? | Transcribed rows count as approval or only as "reviewed"; FNV-1a or SHA-256 for records; verify signatures offline with a key map; import GitHub or GitLab approvals from a downloaded JSON or through the network | Transcribed rows count as "reviewed"; decide SHA-256 for records in the review ADR; signature check opt-in; PR import only from a downloaded file; Gerrit notes later | DD-2 |
| Q-21 | Which spreadsheet round trip? | CSV only; XLSX from the start; may the import write proposed text | CSV first (BOM, `;` option), XLSX later; never write text, only suggestions | DD-5, DD-2 |

### Coverage and tracing

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-22 | How should B10 ship? | Alone first or with the rule engine; all FR, NFR and US, or `must` only; severity warning or info; drafts skipped; suspect links count or not | B10 first as one rule, all three types, info, drafts and deprecated items skipped; suspect links count but are shown | DD-4, DD-3 |
| Q-23 | How should the schema extension look? | One `[[rules]]` mechanism or separate coverage and rules; `[keys.<name>]` beside `[values]`; enforce `from` and `max` by default; localised inverse names | One `[[rules]]`; `[keys]` with `[values]` as shorthand; `from` and `max` in the defaults for new projects only; English inverse names | DD-5, DD-4 |
| Q-24 | Are per-item waivers acceptable? | `waive: <rule>` key with the reason in the body; exclusions in the configuration; none | `waive:` key (new reserved key), reported as info | DD-4 |
| Q-25 | Should requimd read code at all, and in which syntax? | No (requirements only); native tags, bare IDs and Gherkin; plus OFT, tracey and StrictDoc aliases; relation `code` or `impl` | Yes, offline, native, bare and Gherkin, kind `code`; aliases later on demand | DD-4, DD-6 |
| Q-26 | How are test results matched, kept fresh and gated? | Name matching allowed or only through TCs; results on TCs or on requirements directly; manual results; staleness by hash or by date; a failed test is an error or a warning; environments | Name matching allowed; through a TC when one exists, else direct with info; manual results in `results.json`; hash-based staleness in `requimd.results`; warning, error with `--strict`; one environment label per file | DD-4 |
| Q-27 | Which XML dependency? | `quick-xml`; a hand-written reader | `quick-xml`, one ADR covering JUnit and the later ReqIF export | DD-4, DD-5 |

### Exchange

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-28 | Which exchange formats are worth building? | ReqIF export (granularity per file or type, steps as objects); a one-off ReqIF import; a Sphinx-needs `needs.json` adapter; LOBSTER files; multi-repo aggregation | Nothing until a named partner or user asks; then ReqIF export per file, with steps as objects | DD-5, DD-4, DD-6 |

### Agents

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-29 | What may the MCP server do? | Read only; plus propose tools behind `--allow-writes`; plus formatting. A check that writes the lock, or a no-write mode | Read only first; propose tools later and opt-in; a no-write mode for `check` (FR-116 change) | DD-7, DD-6 |
| Q-30 | Which MCP implementation? | `rmcp` (adds tokio and schemars); a hand-written stdio JSON-RPC loop; support only 2026-07-28, or the legacy `initialize` too | Measure both; prefer the hand-written loop if `rmcp` costs much binary size; support both protocol eras for a while | DD-7 |
| Q-31 | Should the MCP server honour an exclude list for confidential files? | Yes; no | Yes, a simple glob list, stated in the agent ADR | DD-7 |
| Q-32 | How far should interoperability with spec-driven tools go? | Recipes only; write AGENTS.md or only print it; generate Kiro steering and hooks; generate OpenSpec change folders; lint foreign Markdown for language only | Recipes plus a marker block in AGENTS.md written by `--write`; no folders, no foreign lint for now | DD-7 |
| Q-33 | Should real analysts be tested, and should size be checked? | A small trial with one analyst using `requimd mcp`; deterministic size checks (acceptance criteria per item, item length) | Run the trial before M9 ships; size checks as an info option later | DD-7 |

### Language rules

| ID | Question | Options | Recommendation | From |
|---|---|---|---|---|
| Q-34 | Which German bindingness set, and how does it map to `prio`? | MUSS, SOLLTE, WIRD (current SOPHIST); muss, soll, wird (2009); strict (only muss). wird to `could` or a new value; mismatch is a finding or not | Current SOPHIST set, "soll" flagged; mapping muss to `must`, sollte to `should`, wird to `could`; mismatch as a warning | DD-8 |
| Q-35 | Should requimd ship a German preset, and which German language facts are built in? | Shipped opt-in preset (`de-sophist`); documented example only (ADR-0009). "muss nicht" and the "Wenn" hint built in or configured | Opt-in preset; both facts built in for German files | DD-8 |
| Q-36 | How should inflection and compounds be matched? | Explicit forms only (ADR-0005); opt-in suffix or compound match per entry, for weak words and glossary terms | Keep explicit forms by default; opt-in per-entry suffix match later | DD-8, phase 1 R-14 |
| Q-37 | Which German EARS keywords? | Sobald or Wenn for events; Falls for unwanted behaviour; Sofern or Bei for optional features | Sobald, Falls, Sofern, Solange; accept Wenn with an info hint | DD-8 |
| Q-38 | How are sources cited, and how is the differentiator worded? | Cite INCOSE numbers with requimd's own short titles; quote rule text; no citations. "A glossary" or "a glossary that checks the prose" | Numbers and edition with own titles, no quotes; "a glossary that checks the prose" | DD-8, DD-6 |
| Q-39 | Which defaults for low-precision rules, file language and purpose phrases? | Info or off for negation, pronouns, comparatives and combinators; language per project, per path or per file; R20 only on FR and NFR | Info (pronouns off in German); project language with per-path override; R20 skips stories | DD-8 |

## Corrections to phase 1

| Phase 1 said or implied | Corrected fact | Source |
|---|---|---|
| A deep-dive brief named FR-305 as the coverage requirement | FR-305 is `requimd show`; coverage is **FR-306**; the step check is FR-106 | `requirements/functional/cli.md` |
| duvet appeared under a cryspen repository | The repository read is **awslabs/duvet** (Apache-2.0) | [awslabs/duvet](https://github.com/awslabs/duvet) |
| A January 2026 "GfWR" might be a renamed INCOSE edition | "GfWR" is the guide's older name, from a 2020 webinar; the 2026 date is an upload date. **GtWR v4 (2023) is current**: 42 rules, 14 categories | [INCOSE webinar 144](https://incose.org/events-master-hidden/2020/10/21/default-calendar/webinar-144-the-incose-guide-for-writing-requirements-raising-the-ante); [reqi.io](https://reqi.io/articles/incose-requirements-quality-42-rule-guide) |
| SOPHIST bindingness muss, soll, wird; conditions wenn or falls (R-11, R-12) | The current brochure uses **MUSS, SOLLTE, WIRD**, KANN is not a bindingness word, and conditions are **FALLS, SOBALD, SOLANGE** | [Peterjohann](https://www.peterjohann-consulting.de/satzschablonen/) |
| None of the Rust tools has a glossary | Reqvire has a SKOS thesaurus with synonyms. Only requimd uses the glossary while the analyst writes (alias detection in prose, hover) | [Reqvire ontologies](https://www.reqvire.org/ontologies) |
| requimd's suspect design already removes the false suspects that Doorstop users tune away | The item hash counts the target's own link lines, `prio` and metadata order, which Doorstop excludes from link stamps; HTML-comment exclusion exists in the spec only | `lock.rs`; [Doorstop](https://doorstop.readthedocs.io/en/latest/reference/item.html) |
| StrictDoc imports JUnit (R-49), and imports and exports ReqIF and Excel | No built-in JUnit import was found; the Excel round trip is "not yet supported"; ReqIF has only the native profile, with random IDs unless MIDs are on | [StrictDoc #2162](https://github.com/strictdoc-project/strictdoc/discussions/2162); [StrictDoc guide](https://raw.githubusercontent.com/strictdoc-project/strictdoc/main/docs/strictdoc_01_user_guide.sdoc) |
| OFT tags such as `[impl->FR-012]` (R-26) | That is not a valid OFT tag; targets must be `type~name~rev`, and OFT reads only OFT-style Markdown, so the syntax brings no interoperability | [OFT spec](https://raw.githubusercontent.com/itsallcode/openfasttrace/main/doc/spec/system_requirements.md) |
| Spec Kit `/analyze` as a read-only cross-check | It is now `/speckit-analyze`, a prompt the agent executes, so it is not deterministic | [analyze.md](https://raw.githubusercontent.com/github/spec-kit/main/templates/commands/analyze.md) |
| Tessl as a spec-as-source framework with an MCP CLI | Reported pivot to a skills registry in January 2026 (secondary sources only) | [Tessl blog](https://tessl.io/blog/skills-are-software-and-they-need-a-lifecycle-introducing-skills-on-tessl/) |
| Jama markets the first official MCP server | Launched 4 May 2026 with 9.35, sold separately; no public tool list | [Jama press release](https://www.jamasoftware.com/press/jama-software-launches-model-context-protocol-mcp-server/) |
| A stdio MCP server fits NFR-006 without caveat (R-86) | It fits ADR-0011, but `check` writes the lock (a hidden write), and answers may leave the machine through the user's agent host | `main.rs`; [MCP spec](https://modelcontextprotocol.io/specification/latest) |
| AGENTS.md as the standard place for any agent | Kiro reads it; **Claude Code is not on the agents.md list** | [agents.md](https://agents.md/) |
| German support in commercial checkers "unverified" | IBM RQA is English- and ASCII-only; QVscribe lists English; Qualicen and Congree remain unverified | [IBM](https://www.ibm.com/support/knowledgecenter/SS3UPN/com.ibm.help.rm.assist.doc/topics/t_running_rqa.html?view=kc) |
| ubCode is commercial for private projects | Unlicensed private use is capped at 5 source files or 100 needs for most commands; keys are checked online | [ubCode licensing](https://ubcode.useblocks.com/usage/ublicense.html) |
| tracey reached 1.0 in December 2025 | True, but the last release is v1.4.0 (May 2026), with macOS and Linux binaries only, and the licence is reported inconsistently | [tracey releases](https://github.com/bearcove/tracey/releases) |

## Open points and caveats

**Assessor acceptance remains the biggest unknown.** No public statement by any ASPICE or ISO 26262 assessor, notified body, FAA or FDA investigator on git-based review evidence was found. S-CORE is the nearest precedent and has published no assessment. The normative texts of ISO 26262-8, ASPICE 4.0, IEC 62304, ISO 13485 and DO-178C were paywalled. Their review-record fields come from secondary sources, and the draft Annex 11 text was not read.

**Several primary sources were out of reach.** The official INCOSE v4 rule titles and summary sheet returned HTTP 403, so rule titles are paraphrases. The SOPHIST 6th-edition brochure was not text-extractable, so its slots come from an older excerpt and a consultant summary. Kruchten's 2004 ontology could be read only from an unofficial excerpt. The ProSTEP ReqIF guide V1.10 was not read. The details of Polarion's approvals and Xray Cloud's current status labels are unverified. OpenRegulatory's reversal rests on search snippets.

**Several proposals rest on design reasoning, not evidence.** The check IDs, severities and false-positive ratings for rules that Femmer did not measure are judgement. No study shows which context-bundle shape makes agents follow requirements best. No usability study exists of business analysts using text-based RM tools, review sheets or MCP. The German word lists are drafts, and most entries are translations.

**Some things could not be tested.** Nothing was installed or run: `merge=union` behaviour during rebase, the binary-size cost of `rmcp`, `quick-xml`, `rust_xlsxwriter` and `calamine`, and whether tracey runs on Windows. The lock-entry counts in this report come from the working tree of `spec/requirements-validation` on 19 November 2026.

## Conclusion

The deep dives change where requimd's leverage lies. Phase 1 framed the next steps as adding governance features. The evidence now says that requimd's distinctive asset, a durable per-link review state, is only as valuable as its signal is clean. That signal is about to be frozen with a scope that generates noise. Fixing the hash, giving the lock a version, and replacing "delete it" with a safe resolution cost little before v0.1.0 and a great deal after. Every later idea builds on that one signal: decision re-evaluation, approval anchors, stale test results, the PR impact checklist, and agents asking "what is suspect?".

The second shift is that requimd's niche is defined by what it refuses to do as much as by what it does. Competitors sell editor tooling behind licence keys (ubCode), re-infer coverage with a language model on every run (Spec Kit), or move authoring to agents (Reqvire). requimd's position is the opposite: deterministic, offline, human-approved, and readable by an analyst in plain Markdown. The design choices in this report follow from that. Status rules are computed rather than set, review records are pinned to hashes, MCP is read-only first, and German rules are cited by number. They keep requimd the layer that others build on, rather than another tool that guesses.
