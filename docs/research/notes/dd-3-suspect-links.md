# DD-3 Suspect-link and review-state mechanics

Scope: a deeper look at how Doorstop, ReqView, OpenFastTrace (OFT), tracey and the enterprise tools (DOORS Next, Jama, Polarion, Codebeamer, plus PTC Integrity and R4J where they add something) detect, show and clear "suspect" links. It also covers how tools with generated state files avoid merge conflicts. The goal is to decide the refinements to FR-116, FR-307, FR-218 and ADR-0007. Research date: 2026-11-18. Phase 1 findings are not repeated unless a detail was corrected or deepened.

Baseline of requimd as built (read from the repository, not from the phase 1 report):

- `requimd.lock` has only two tables. `[ids]` holds the highest number per prefix. `[links]` holds one line per link, `"<source> <key> <target>" = "<16 hex FNV-1a of the target>"`. **There is no per-item hash and no item review state.** The task background suggests an item hash, but the lock does not have one. — `C:/Users/grafj/projects/reqmd/requimd.lock`, `C:/Users/grafj/projects/reqmd/crates/requimd-core/src/lock.rs`
- The item hash covers the title, then every metadata entry except `status` in written order (`key: value`), then the body. So the target's own `impl:`/`ref:`/`ver:` lines, `prio` and the metadata order all count. A step hash covers only the step text. — `crates/requimd-core/src/lock.rs` (`item_content`)
- The lock parser uses `serde(deny_unknown_fields)`, so today's binaries reject any new table or key in the lock. — `lock.rs`
- A lock that cannot be parsed stops `check` with this hint: "Resolve merge conflicts in requimd.lock, or delete it to record all links anew." — `lock.rs` (`Lock::load`)
- `requimd review <ID>` marks all suspect links **from or to** the item as reviewed. `--all` marks every suspect link. Neither records a reason or a reviewer. — `C:/Users/grafj/projects/reqmd/requirements/functional/cli.md` (FR-307)
- ADR-0010 (proposed) takes HTML comments out of the hash. ADR-0007 says the hash is frozen with v0.1.0 (E4). — `C:/Users/grafj/projects/reqmd/docs/decisions/0007-lock-file.md`, `0010-html-comments-outside-the-hash.md`
- The repository has no `.gitattributes`. — the repository root

## Doorstop: fingerprints, link stamps, review vs. clear, known problems

### Takeaway
Doorstop keeps two different hashes. The item's `reviewed` fingerprint includes the UIDs of its own links. The stamp a child stores per parent link is the parent's fingerprint **without** the parent's links. So adding a parent link makes the parent "unreviewed" but does not make its children's links suspect. `review` and `clear` are separate commands with no reason field. The project itself recommended `review all` + `clear all` as the migration when its hash algorithm changed, which is the audit anti-pattern requimd should avoid.

### Cited Findings
- Fingerprint inputs: by default "the item's UID, the `text` and `ref` values, and the UIDs in `links`" (plus `references`). `active`, `derived`, `normative`, `level` and `header` "do **not** contribute to the fingerprint", and neither do extended attributes by default. — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- Configurable scope: extended attributes count only when listed under `attributes: reviewed:` in `.doorstop.yml`. The list must be non-empty. A listed attribute missing from an item is skipped silently. Value types count ("empty strings, lists, and dictionaries affect the fingerprint"). Changing the order of the list changes fingerprints. Removing an attribute changes the fingerprint of every item that has it. — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- Algorithm and storage: SHA256, URL-safe Base64, stored in the item file. "You should not calculate this value manually, use `doorstop review`." — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- Source code, `stamp()`: `values = [self.uid, self.text, self.ref]`, then `references` if present, then `self.links` **only if `links=True`**, then each key in `extended_reviewed` serialised with a type-tagged string. `review()` sets `self._data["reviewed"] = self.stamp(links=True)`. `clear()` sets `uid.stamp = item.stamp()` for each parent, optionally only for the given parent UIDs. The `cleared` property is false if any parent link's stored stamp differs from `item.stamp()`. Links are saved as `[{uid: stamp}]`. — [doorstop/core/item.py](https://github.com/doorstop-dev/doorstop/blob/develop/doorstop/core/item.py)
- The link stamp does "**not** contribute to the fingerprint of the item". Only the parent UID does. So clearing a link does not make the child unreviewed. — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- Display: validation prints `WARNING: LLT: LLT005: suspect link: REQ001`. Unreviewed changes give a WARNING. Never-reviewed items give an INFO shown only in verbose mode. `doorstop clear LLT005` clears all suspect links of an item; `doorstop clear LLT005 REQ002 REQ003` clears only those to the named parents. — [Doorstop validation](https://doorstop.readthedocs.io/en/latest/cli/validation.html)
- History: `review` and `clear` arrived in 0.8 (2014). In 1.2, whitespace handling changed and "Running `doorstop review all` will be required". In 2.0 the hash moved to SHA256 (BREAKING), with the instruction "Use `doorstop review all` followed by `doorstop clear all`" and the warning "This marks all items as reviewed and clears all suspect links." — [Doorstop changelog v2.1](https://doorstop.readthedocs.io/en/v2.1/about/changelog/)
- Issue #358 "MD5 as a stamp algo" (2019): a contributor proposed recording the algorithm with each hash so future changes would not break compatibility. A maintainer said "The fingerprint just needs to cover random content changes. For this MD5 is fine". It was reopened for a change-control-board use case needing secure signatures, then closed. — [Issue #358](https://github.com/doorstop-dev/doorstop/issues/358)
- Issue #178 "Running doorstop causes links to be cleared" (2014): plain validation auto-calculated link stamps, so new links never warned. The maintainer saw little value in warnings "immediately after items get created" and kept the behaviour. Related issues: #173 "Consider links trusted upon creation", #123 "Add an option to disable suspect/review status" (merged), #374 "Clear suspect links by UIDs" (merged), #564 "hash based review of external reference content". — [Issue #178](https://github.com/doorstop-dev/doorstop/issues/178); [issue search "suspect"](https://github.com/doorstop-dev/doorstop/issues?q=suspect)
- Ecosystem: the VS Code extension offers a quick fix "Clear All Suspect Links"; the doorstop-edit GUI offers review and clear. — [doorstop-vscode](https://github.com/simonwalbrun-lab/doorstop-vscode), [doorstop-edit](https://github.com/ownbee/doorstop-edit) (both via search summary). Jamb copies the concept ("Downstream links become suspect"; `jamb review mark`, `jamb review clear`), but its FAQ does not say what is hashed or where stamps live. — [Jamb FAQ](https://jamb.readthedocs.io/en/latest/faq.html)

### Inferences
- requimd behaves like Doorstop's issue #178 outcome ("trust on creation"). FR-116 records new links silently, and ADR-0007 argues the same way.
- requimd's item hash includes the target's own metadata lines, so adding `impl: UC-003.s2` to FR-012 makes every `ver FR-012` link suspect. Doorstop deliberately avoids this cascade by leaving `links` out of the link stamp. This is the single biggest source of false suspects to fix before the v0.1.0 hash freeze. `prio` changes and metadata reordering cause false suspects the same way.
- Doorstop's migration advice shows the cost of an unversioned hash: every algorithm or normalisation change ends in a blanket clear. Recording a hash version in the lock (the #358 suggestion) lets a tool migrate without clearing real suspects.
- Doorstop has no reason, reviewer or date on `review`/`clear`. The audit trail is whatever git shows. A one-click "Clear All Suspect Links" in editors makes blanket clearing even easier.

### Gaps
- Whether `doorstop review` also re-stamps links in current versions could not be confirmed. The code shown sets only `reviewed`. CLI command docs (`/cli/commands.html`) returned 404.
- No public reports of Doorstop merge conflicts in stamps were found. The issue search for "suspect" showed none.

## ReqView: per-link-type suspect configuration, triggers, clearing

### Takeaway
ReqView configures suspicion per link type as a list of target attributes (`"suspect": ["heading", "text", "deleted"]`, also `attachments` and custom attribute IDs). A link type without the property is never suspect. Clearing works per link, per selection (outgoing or incoming) and project-wide. No reason is recorded, and the feature depends on ReqView's stored change history.

### Cited Findings
- Configuration example: `"satisfaction": { "name": "Satisfaction", "source": "Satisfies", "target": "Is satisfied by", "suspect": ["heading", "text", "deleted"] }`. "Permitted suspect attributes are: heading, text, deleted, attachments and custom attribute IDs." "A link becomes suspect if any of the suspect attributes of its target object changes." — [ReqView change management](https://reqview-homepage.netlify.app/doc/change-management)
- Direction: a change of a user story in NEEDS sets "the suspect flag for all its incoming satisfaction links", shown on the linking SRS requirements. — [ReqView change management](https://reqview-homepage.netlify.app/doc/change-management); [ReqView suspect links beta (2017)](https://reqview-homepage.netlify.app/blog/2017-02-07-beta-suspect-links)
- Clearing has four scopes:
  - one link: context menu "Clear Suspect Link Flag"; the flag returns when the target changes again;
  - selected objects, outgoing links: Ctrl+J;
  - selected objects, incoming links: Ctrl+Shift+J;
  - whole project: Project > Maintenance > Clear All Suspect Flags.
  The documentation does not mention a reason or comment. — [ReqView change management](https://reqview-homepage.netlify.app/doc/change-management)
- Disabling "Save History" "impacts several ReqView features (for instance Suspect Links)". — [ReqView change management](https://reqview-homepage.netlify.app/doc/change-management)

### Inferences
- ReqView's per-link-type list maps directly onto a `requimd.toml` table keyed by link key (`impl`, `ref`, `ver`). "deleted" corresponds to requimd's existing `broken-link` check, so it is not needed as a part.
- Clearing "incoming" links of a selection (Ctrl+Shift+J) is the same operation as requimd's `review <target>`. ReqView offers it as a plain clear, the editorial escape in all but name (see the section on editorial changes).

### Gaps
- ReqView's JSON schema for link types and whether the history records who cleared a flag were not verified.

## OpenFastTrace and tracey: revisions in IDs, stale and outdated states, pre-commit enforcement

### Takeaway
Both tools put a revision number in the item ID and make the author decide when a change is semantic. OFT has no change detection at all: forgetting a bump goes unnoticed. tracey closes that hole with `tracey pre-commit`, which fails when rule text changed without a bump, and `tracey bump`, which bumps automatically. Neither tool documents an "editorial change" exemption. With tracey the only way around the hook is to skip it.

### Cited Findings
- OFT ID: "A specification item ID consists of an artifact type, an item name, and a numeric revision" (`req~specification-item-id-syntax~1`). `#` is allowed as a separator so tildes are not read as strikethrough in Markdown. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- OFT outgoing link statuses (`req~tracing.outgoing-coverage-link-status~1`):
  - outdated: the link "points to a specification item which has a higher revision number";
  - predated: it "points to a specification item which has a lower revision number";
  - ambiguous: the target has duplicates;
  - orphaned: "link is broken";
  - unwanted: the requester does not want this artifact type.
  Incoming statuses are "covered predated" and "covered outdated". OFT's requirements do not say when an author must bump a revision. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- tracey: `r[auth.login+3]` in the spec, `// r[impl auth.login+3]` in code. After a bump to `+4`, "tracey reports the `+3` reference as **stale**". `tracey query stale` and the MCP tool `tracey_stale` list them. — [tracey README](https://github.com/bearcove/tracey)
- tracey versioning guide:
  - "When you edit a requirement's text in the spec, you need to bump its version number."
  - `tracey bump` "Automatically increments the version number of staged requirements whose text changed, then re-stages the modified files."
  - `tracey pre-commit` "Checks staged spec files for requirements whose text changed without a version bump", installed by hand in `.git/hooks/pre-commit`.
  - Unversioned IDs are implicitly version 1.
  - "Stale requirements are not counted as covered."
  - The bump of a code annotation "is a signal that you've reviewed and updated the implementation". The implementation must be changed first, "and ONLY ONCE THAT'S DONE must the code annotation be bumped."
  - [tracey versioning guide](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md)
- Neither the README nor the guide documents an exemption for typo fixes, or says whether whitespace is normalised. — [tracey README](https://github.com/bearcove/tracey); [tracey versioning guide](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md)

### Inferences
- Revisions and hashes are two answers to one question: who decides that a change matters? With OFT, only the author decides, so changes are missed silently. With tracey, the tool detects and the author confirms. With requimd, the tool detects and the downstream reviewer confirms. requimd's model is the stricter one. An editorial escape would give requimd tracey's "author confirms" path, but explicitly and with a record.
- tracey's "stale is not covered" is a policy choice worth noting for DD-4. Today a suspect link still counts as coverage in requimd (FR-116 is a warning).
- tracey's hook works per commit. requimd's equivalent is `requimd check --locked` in CI per PR, which is coarser but does not need hooks installed in every clone.
- For a business analyst, revisions in IDs (`UC-004~2`) are noisy and change every reference on a bump. Since requimd's IDs must not change (BR-002, BR-003), revisions do not fit.

### Gaps
- tracey's change detection (diff against HEAD or index, normalisation) is not documented in the pages read. `docs/spec/tracey.md` returned 404.

## Enterprise tools: direction, triggers, clearing, permissions, reasons

### Takeaway
Every enterprise tool limits suspicion to configured fields:

- DOORS Next: the "Affects Link Validity" attribute flag (7.0.3+).
- Jama: "Use to Trigger Suspect" per field.
- Codebeamer: per reference field, with "Omit Suspected when changing".
- R4J: "Monitor Fields" per scheme.
- PTC Integrity: "significant" fields.

Most flag downstream only. Codebeamer and R4J offer reverse or both directions. Comments on clearing are optional (DOORS Next) or not documented (Jama, Codebeamer, ReqView). None documents a per-edit "minor change" switch. A recurring user complaint is that one cannot see *what* changed when clearing.

### Cited Findings
- **DOORS Next / IBM ELM link validity**:
  - Status values are Suspect (also the default for new or unknown links), Valid, Invalid and Unknown.
  - Changing the content of either artifact makes its link suspect.
  - From 7.0.3, "only attributes explicitly marked as Affects Link Validity influence the link validity status". Metadata such as creator, dates and priority does not count.
  - Users "can also include comments about why you chose a status".
  - An administrator can make test-case links Valid automatically when a test case review is approved.
  - Direction affects only the artifact validity summary.
  - [IBM link validity 7.0.3](https://www.ibm.com/docs/SSYMRC_7.0.3/com.ibm.jazz.vvc.doc/topics/c_linkval.html); [IBM link validity 7.2](https://www.ibm.com/docs/SSYMRC_7.2/com.ibm.jazz.vvc.doc/topics/c_linkval.html)
- DOORS Next validity is **content-keyed**. A status is reused wherever "the same two artifacts with that same content and link type" appear. "If content reverts to a known state, the earlier status is reapplied." A merge of change sets that modified the artifact makes the link Suspect. — [IBM link validity 7.2](https://www.ibm.com/docs/SSYMRC_7.2/com.ibm.jazz.vvc.doc/topics/c_linkval.html)
- A user on 7.0.2 complained that changing a "responsible" attribute turned valid links suspect. The answer was the 7.0.3 admin setting. No per-edit override was mentioned. — [Jazz forum 286898](https://jazz.net/forum/questions/286898/attributes-affecting-link-validity). Suspicion profiles are no longer supported for DOORS Next without configuration management. — [IBM link validity 7.0.3](https://www.ibm.com/docs/SSYMRC_7.0.3/com.ibm.jazz.vvc.doc/topics/c_linkval.html)
- **Jama**:
  - A suspect link appears when an upstream item changes, "only if the change updates a field that the organization admin has configured to trigger suspect links".
  - Clearing works per link ("Clear") or for all visible links ("Clear All"). The page says nothing about permissions, comments or a record.
  - [Jama: clear suspect links](https://help.jamasoftware.com/en/manage-content/coverage-and-traceability/relationships/clear-suspect-links.html)
  - The field option is named "Use to Trigger Suspect". — [Jama: add, edit, or delete fields](https://help.jamasoftware.com/en/administration/organization-administrator/managing-process/fields/add,-edit,-or-delete-fields.html) (via search summary)
  - Direction is downstream only. A staff reply in the community said other directions are not in scope. — [Jama community thread](https://community.jamasoftware.com/discussion/comment/7274/) (via search summary; date not shown)
  - A support article says the UI does not necessarily show all suspect links. — [Jama support](https://support.jamasoftware.com/hc/en-us/articles/34383432239373-Not-All-Suspect-Links-are-Displayed) (via search summary)
- **Codebeamer**:
  - "Propagate suspects" is set per reference field, with reverse and bidirectional variants. — [Codebeamer associations](https://support.ptc.com/help/codebeamer/r3.3/en/codebeamer/user_guide/ug_associations.html) (via search summary)
  - A per-field "Omit Suspected when changing" option suppresses the badge for edits in that field. — [Codebeamer default reference settings](https://support.ptc.com/help/codebeamer/r3.2/en/codebeamer/user_guide/ug_set_default_ref_settings.html) (via search summary, not opened)
  - The SUSPECTED badge has an up or down arrow for direction. Clearing works via "Clear Suspected" in the Merge dialog or the icon next to the badge, with a confirmation. The page says nothing about logging. — [Codebeamer propagate suspects](https://support.ptc.com/help/codebeamer/r3.0/en/codebeamer/user_guide/ug_propogate_suspects_reference.html)
  - Bulk clearing is a community workaround ("Inspect All Changes", "Clear all suspected links in all pages"). — [PTC community](https://community.ptc.com/t5/Codebeamer/Bulk-edit-suspected-flags-in-codebeamer/td-p/1005196) (via search summary)
- **Polarion**: link roles are configured globally and overridden per project. A checkbox controls whether new work items get the Suspect attribute automatically. — [Polarion: configure work item linking](https://docs.plm.automation.siemens.com/content/polarion/20/help/en_US/user_and_administration_help/administrators_guide/configure_work_items/configure_work_item_linking.html) (via search summary). The Polarion clearing workflow was not found. In PTC Modeler's Polarion integration, links are suspect if the remote item changed "since the link was created or since the suspect status was last cleared". — [PTC Modeler OSLC](https://support.ptc.com/help/modeler/r10.1.0.1/en/Modeler/rtsme/managing_changes_to_remote_oslc_data.html) (via search summary)
- **PTC Integrity**:
  - In the Clear Suspect dialog, "By default, all identified suspect traces are marked for clearing." — [PTC Integrity help](https://support.ptc.com/help/windchillrvs/r12.3.0.0/en/IntegrityHelp/client_docs_managing_suspect_trace_relationships-1.html)
  - By default a trace becomes suspect on a "significant edit": the Input Revision Date changes together with a significant field such as Text. A user could not see what caused a suspect trace. The reply: "It's not currently possible to see what caused a suspect trace." — [PTC community 553247](https://community.ptc.com/t5/Integrity/Viewing-what-changed-when-clearing-suspect-trace/m-p/553247)
- **R4J (Requirements for Jira)**:
  - Suspect schemes combine a Query (which issues trigger), "Monitor Fields" (advice: monitor only fields that affect related issues, e.g. Description), Link Types, and a Scope of Downstream, Upstream or Both.
  - Events are logged as revisions: `R4J Suspect: Suspect set in ABC-123 due to change in ABC-456`.
  - Configuring schemes needs a "Suspect Configuration" permission.
  - [R4J suspect schemes](https://easesolutions.atlassian.net/wiki/spaces/REQ4J/pages/5406964/Configuring+Suspect+Schemes)

### Inferences
- The industry default is downstream-only suspicion with a field allow-list. Only Codebeamer and R4J offer upstream. requimd's direction (the source of a link becomes suspect when its target changes) matches Jama, ReqView and Doorstop. There is no strong case for upstream suspicion.
- The enterprise tools have permissions on configuring suspicion (R4J, DOORS Next admins), not on clearing it. requimd's git-native equivalent of a clearing permission is the PR review of `requimd.lock` changes. A CODEOWNERS rule on `requimd.lock` could require the owner's review. This is my inference; CODEOWNERS was not researched.
- DOORS Next's content-keyed validity is the same property requimd's hash already has: a revert restores the reviewed hash, and the link is no longer suspect. This is worth stating in FR-116 as intended behaviour.
- "What changed since the last review?" is a real unmet need (PTC Integrity). requimd can answer it offline from git history alone, without storing text in the lock.

### Gaps
- Jama, Codebeamer and Polarion permissions for clearing, and whether clearing is recorded in history, were not found in public docs.
- No enterprise tool was found with a per-edit "minor/editorial change" flag. This is absence of evidence from the pages read, not proof.

## An "editorial change" escape: how tools handle non-semantic edits, and the risks

### Takeaway
No tool studied has an explicit per-edit "editorial" flag. Tools handle the problem in one of three ways:

1. **Scope**: fields that never trigger suspicion (all enterprise tools, Doorstop, ReqView).
2. **Author-declared revisions**: the author simply does not bump (OFT, tracey).
3. **Target-side bulk clear**: clear the incoming links of the changed item (ReqView Ctrl+Shift+J, requimd's `review <target>`).

requimd already has the third route. What it lacks is a record of the reason and some guard rails.

### Cited Findings
- Scope route: DOORS Next "Affects Link Validity" — [IBM 7.2](https://www.ibm.com/docs/SSYMRC_7.2/com.ibm.jazz.vvc.doc/topics/c_linkval.html); Jama "Use to Trigger Suspect" — [Jama fields](https://help.jamasoftware.com/en/administration/organization-administrator/managing-process/fields/add,-edit,-or-delete-fields.html); Codebeamer "Omit Suspected when changing" — [Codebeamer](https://support.ptc.com/help/codebeamer/r3.2/en/codebeamer/user_guide/ug_set_default_ref_settings.html); Doorstop `attributes: reviewed` — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- Revision route: tracey requires a bump on every text change, checked by `tracey pre-commit`. No exemption is documented. — [tracey versioning guide](https://github.com/bearcove/tracey/blob/main/docs/content/guide/versioning.md). OFT leaves bumping entirely to the author. — [OFT system requirements](https://github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md)
- Target-side clear: ReqView "Clear Suspect Flags > Incoming Links". — [ReqView change management](https://reqview-homepage.netlify.app/doc/change-management). requimd: `review <ID>` marks suspect links "from or to the item". — `requirements/functional/cli.md` (FR-307)
- Risk evidence: Doorstop's blanket `review all` + `clear all` warning — [changelog](https://doorstop.readthedocs.io/en/v2.1/about/changelog/); PTC Integrity selects all suspect traces for clearing by default — [PTC Integrity help](https://support.ptc.com/help/windchillrvs/r12.3.0.0/en/IntegrityHelp/client_docs_managing_suspect_trace_relationships-1.html); ADR-0010 itself argues that false suspect links "teach analysts to mark links as reviewed without looking" — `docs/decisions/0010-html-comments-outside-the-hash.md`

### Inferences
- Main risk: the author of a change judges its impact on other people's items. An "editorial" label on a change that alters meaning (e.g. "within 5 days" to "within 5 working days") silently skips downstream review. The safeguards should make the label visible to the PR reviewer, not try to detect semantics.
- Second risk: overuse. If every change is labelled editorial, suspect links lose their value. Recording the reason per link and listing editorial re-stamps separately in `requimd diff` and the PR makes overuse visible.
- Third risk: the label applied to the wrong scope. Target-side re-stamping should touch only links whose stored hash equals the target's hash *before* this change, i.e. links that were clean before the edit. Links that were already suspect for an earlier change stay suspect. This needs the previous hash, which the lock already has: entries with the target's old hash.
- A heuristic guard ("editorial only if at most N words changed") is possible, since requimd can diff the normalised text against git HEAD. It is a weak signal, because one word can change meaning, so it should only warn.
- Not recommended: a commit-message trailer (`Requimd-Editorial: FR-012`). Squash or rebase can lose it, and `check` would have to read commit history to decide suspicion, which couples the lock state to history.

### Gaps
- No empirical data on how often editorial escapes are misused in practice was found.

## Item-level review state vs. link-level suspect

### Takeaway
Only Doorstop (and Jamb) keeps a separate "item changed since last review" state. The enterprise tools express it through versions, baselines and review workflows (DD-2), not as a link-like flag. requimd currently tracks only link state. For an analyst the item state would duplicate git, status and review records. Its one unique benefit covers items without incoming links, whose changes no suspect link reports.

### Cited Findings
- Doorstop: `reviewed` is "the fingerprint of the item when it was last reviewed", used to "detect unreviewed changes to an item". Never-reviewed items give only verbose INFO messages. — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html); [validation](https://doorstop.readthedocs.io/en/latest/cli/validation.html)
- Doorstop's item fingerprint includes link UIDs, so adding a link makes the item unreviewed. Clearing a link does not. — [doorstop/core/item.py](https://github.com/doorstop-dev/doorstop/blob/develop/doorstop/core/item.py)
- DOORS Next reuses validity by content and automates Valid on test-case review approval. This ties link state to a review workflow, not to a separate item flag. — [IBM 7.2](https://www.ibm.com/docs/SSYMRC_7.2/com.ibm.jazz.vvc.doc/topics/c_linkval.html)
- requimd has `status` (excluded from the hash) and BR-004: tools never set `approved`. — `C:/Users/grafj/projects/reqmd/CLAUDE.md`, `requirements/functional/checks.md` (FR-116)

### Inferences
- An item review stamp in the lock would add one line per item. It would also add a second review ceremony ("review the item" vs. "review the link"), which Doorstop users skip with `review all`.
- The question an analyst really asks is "was an *approved* item changed after approval?". That is better answered by comparing against the review record or baseline in which it was approved (R-45/R-37 in DD-2), or by a status-transition rule (R-40), than by a free-floating stamp.
- Recommendation: do not add item review state to ADR-0007 now. Revisit if DD-2's review record does not cover "changed after approval".

### Gaps
- No user evidence was found that Doorstop users value the item stamp separately from the link stamp.

## Lock file merge conflicts: how other tools avoid them, and what fits requimd

### Takeaway
requimd's sorted line-per-entry format already follows the Cargo approach. Conflicts remain for three reasons:

1. `[ids]` counters change on every branch that adds an item of the same prefix.
2. New items get the next free number, so their link lines are inserted at the same place on parallel branches.
3. A review rewrites every line for one target.

All three have a deterministic semantic resolution: `ids` take the maximum; links take the union, reconciled against the merged content; for a conflicting hash, keep the side that matches the current target, else stay suspect. That makes the lock suitable for automatic resolution. The cheapest robust option is `merge=union` in a committed `.gitattributes`, a tolerant loader and reconciliation in `requimd check`. An explicit `requimd lock resolve` covers files that still contain conflict markers. An optional custom merge driver can come later.

### Cited Findings
- Git's merge attribute:
  - built-in drivers are `text`, `binary` and `union`;
  - `union` takes "lines from both versions, instead of leaving conflict markers", tends to leave added lines "in random order", and comes with the warning "Do not use this if you do not understand the implications";
  - custom drivers are defined in `.git/config` or `~/.gitconfig`, "not in the gitattributes file", as `driver = cmd %O %A %B %L %P`; the driver leaves its result in `%A` and exits 0 for a clean merge, non-zero for conflicts;
  - the attribute applies to `git merge` and to other commands "such as git revert and git cherry-pick".
  - [gitattributes](https://git-scm.com/docs/gitattributes)
- Cargo: the lock encoder was designed "to reduce merge conflicts". — [cargo encode.rs](https://rust.googlesource.com/cargo/+/fb7fbf0bb9bdbe10912f5ea031efdaea9d5d10a2/src/resolver/encode.rs) (via search summary). A third-party `cargo-merge-driver` exists. — [cargo-merge-driver](https://awesome.ecosyste.ms/projects/github.com%2Frelrelb%2Fcargo-merge-driver). Regenerating by deleting the file can pick newer versions. — [cargo-generate-lockfile](https://man.archlinux.org/man/cargo-generate-lockfile.1.en); [Pijul discourse](https://discourse.pijul.org/t/putting-generated-files-into-a-repository/713) (via search summary). Mozilla reported repeated `Cargo.lock` conflicts during rebases. — [Mozilla bug 1424096](https://bugzilla.mozilla.org/show_bug.cgi?id=1424096) (via search summary)
- npm (since 5.7.0): resolve `package.json` by hand, then rerun `npm install [--package-lock-only]`. "npm will automatically resolve any conflicts for you and write a merged package lock". — [npm package-locks v6](https://docs.npmjs.com/cli/v6/configuring-npm/package-locks) (via search summary). A user reported that this did not clear thousands of conflicts. — [Stack Overflow mirror](https://kaweah.freedombox.rocks/kiwix/content/stackoverflow.com_en_all_2023-11/a/54127283) (via search summary). Merge drivers exist: [npm-merge-driver](https://awesome.ecosyste.ms/projects/github.com%2Fnpm%2Fnpm-merge-driver), [package-lock-merge-driver](https://awesome.ecosyste.ms/projects/github.com%2Fboneskull%2Fpackage-lock-merge-driver)
- ReqView avoids merging its JSON files altogether by using file locks in git. — [ReqView git collaboration](https://reqview-homepage.netlify.app/doc/git-collaboration) (phase 1). Doorstop keeps stamps inside each item's YAML file, so the conflicts land next to the content change. — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- requimd specifics:
  - `[ids]` is a single line per prefix (`FR = 504`);
  - link keys are sorted by source ID;
  - many links share one target hash (about 20 lines carry `c723dd50f7b65014` for `UC-003.s2`);
  - an unparsable lock stops `check` and is never overwritten, and the hint suggests deleting it.
  - `requimd.lock`, `crates/requimd-core/src/lock.rs`, `docs/decisions/0007-lock-file.md`

### Inferences
- Why requimd conflicts so often: two branches that each add one FR item both change `FR = 504` (to 505 and 506). Their new link lines (`FR-505 …`, `FR-506 …`) are inserted after the same last FR line. Git's text merge reports changes at the same or adjacent position as a conflict. The sorted format cannot avoid this, because new IDs always sort to the end of their range.
- Deleting the lock, as the current hint suggests, is the Doorstop anti-pattern. All links are re-recorded as reviewed, so every open suspect is silently cleared. `[ids]` also falls back to the highest ID present, which can reuse IDs deleted on a branch (BR-002). The hint should change regardless of the rest.
- "Take one side, then regenerate" (the Cargo/npm habit) is also lossy for requimd. Reviews made on the other side return as suspect, which is safe but annoying. Deleted-ID high-water marks of the other side are lost, which is unsafe for BR-002.
- Resolution rules that are safe without a merge base, given the merged item files:
  - `[ids]`: the maximum per prefix.
  - `[links]`, key on one side only: keep it; `check` drops it if the link no longer exists.
  - `[links]`, same key with two hashes: keep the one that equals the target's current hash; otherwise keep either, and the link stays suspect.
  Principle: **ambiguity resolves to suspect, never to reviewed.**
- `merge=union` fits because the rules above are line-local. Union output for a conflicting hunk is just both sides' lines: duplicate `FR = …` lines, duplicate link keys and unsorted entries, all of which the rules resolve. The `[ids]`/`[links]` headers are unchanged context, so they are not duplicated. It needs one committed line (`requimd.lock merge=union`) and no per-clone configuration or binary on the PATH of a git GUI. Plain TOML parsers reject duplicate keys, so requimd needs a tolerant loader for this one file. `check --locked` must fail on a non-normalised lock so CI forces a local `check`.
- A custom merge driver (`requimd lock merge %O %A %B`) gives a clean result immediately and can use the base. But it must be configured per clone in `.git/config`, and requimd must be on the PATH of whatever runs git, which is fragile on Windows with GUI clients. GitHub's server-side merge does not run custom drivers. That does not matter here, since the branch is rebased locally before "Rebase and merge" (CLAUDE.md). Rebase replays commits with the same merge machinery as cherry-pick, so union and custom drivers both apply during `git rebase`. This is my inference from the gitattributes wording; it was not tested.
- Splitting the lock per document (one lock file per requirements file) would reduce conflicts but would scatter the state and complicate renames across files. It is not recommended.

### Gaps
- Whether git falls back to the text driver when `.gitattributes` names a merge driver that is not configured was not verified in the docs read.
- Yarn's and pnpm's built-in lock conflict auto-resolution were not fetched.

## Proposal for requimd

### Takeaway
Six refinements in priority order:

1. Narrow the hash scope and version the lock before the v0.1.0 freeze.
2. Add per-link-type suspect scope in `requimd.toml`.
3. Make lock conflicts self-healing (`merge=union` plus tolerant load, `requimd lock resolve`, a fixed recovery hint).
4. Record reason, reviewer and date for reviews in an append-only log.
5. Add an explicit `--editorial` target-side review with guard rails.
6. Show the target diff since the last review.

Item-level review state is deferred to DD-2.

### Proposals

**P1. Hash scope and lock version (amend ADR-0007, FR-116; decide before v0.1.0, E4).**
- Exclude the target's own link lines (`impl`, `ref`, `ver`, and any other link key) and `prio` from the item hash, as Doorstop does for link stamps. Hash the remaining metadata sorted by key, so reordering does not count.
- Add a header to the lock: `version = 1` and `hash = "fnv1a64-ws1"`. A later change of algorithm or normalisation then migrates without blanket clearing. On load, an entry whose stored hash equals the *old-scheme* hash of the current content is clean and gets the new hash. Any other entry is suspect and gets a sentinel (`"suspect"`) that stays until reviewed. The same rule handles a scope change in P2.
- Because of `deny_unknown_fields`, the new keys must land together with the parser change.

```toml
# requimd.lock: generated by requimd, commit it.
version = 1
hash = "fnv1a64-ws1"

[scope]                       # signature of the parts hashed per link key (P2)
impl = "title,meta,body"
ver  = "title,meta,body"

[ids]
FR = 504

[links]
"FR-101 impl UC-012.s1" = "8f3a1c0e5b7d2946"
"TC-220 ver FR-104" = "suspect"     # migrated, still awaiting review
```

**P2. Per-link-type suspect scope (FR-116, new config keys under FR-117 style).**

```toml
# requimd.toml
[suspect]                 # default for all link keys
parts = ["title", "meta", "body"]   # meta = metadata except status, prio and link keys
ignore-meta = []          # extra metadata keys to ignore, e.g. ["owner"]

[suspect.ref]
parts = []                # "never suspect": loose references

[suspect.ver]
parts = ["title", "body"] # a test cares about wording, not about metadata
```

- Step targets always use the step text. `parts` applies to item targets only.
- Mentions stay never suspect.
- A change of `parts` re-stamps clean links and keeps suspect ones (P1 rule), so editing the config never clears an open suspect.
- `requimd check` reports the migration once, as an info diagnostic.

**P3. Merge strategy for `requimd.lock` (amend ADR-0007, FR-300).**
1. `requimd init` (and the docs) add `requimd.lock merge=union` to `.gitattributes`. This repository should commit it too.
2. The loader tolerates duplicate keys and unsorted entries in the lock only:
   - `[ids]` take the maximum;
   - for a duplicate link key, keep the hash equal to the current target hash, else `"suspect"`.
   - `requimd check` rewrites the file normalised. `check --locked` fails with "lock not normalised; run requimd check".
3. `requimd lock resolve` handles a lock that still contains conflict markers (driver missing, or another tool's merge). It applies the same rules to both sides, then reconciles with the project (adds new links, drops dead ones) and prints a summary. `check` keeps refusing a lock with markers (ADR-0007), but its hint changes from "delete it" to "run `requimd lock resolve`".
4. Optional, later: `requimd lock merge %O %A %B` as a custom driver for teams that want a clean merge without a follow-up `check`.

```text
$ git rebase origin/main
CONFLICT (content): Merge conflict in requimd.lock
$ requimd lock resolve
requimd.lock: 2 conflicts resolved
  ids FR: 505 / 506 -> 506
  TC-220 ver FR-104: kept c07b5e19d3a48f62 (matches current FR-104)
  3 links added, 1 removed
$ git add requimd.lock && git rebase --continue
```

**P4. Review with reason, reviewer and date (FR-307, FR-218, new item).**
- `requimd review` appends one line per re-stamped link to an append-only log `requimd.reviews`, also with `merge=union` in `.gitattributes`. Order does not matter and duplicates are harmless, so union is fully safe here.
- The reviewer comes from `git config user.name`, which works offline (NFR-006).
- `--reason` is optional for a single source item, and **required** for `--all` and for target-side reviews (P5).
- The lock itself stays hash-only, so it does not grow or conflict more.

```text
# requimd.reviews: appended by requimd review, commit it. One line per link.
2026-11-18  A. Analyst  TC-220 ver FR-104  c07b5e19d3a48f62 -> 4b1e09d2a7c35f10  confirmed: test still valid
2026-11-18  A. Analyst  FR-101 impl UC-012.s1  8f3a1c0e5b7d2946 -> 0d77e2c1b9a84f53  editorial: typo in step text
```

```text
$ requimd review FR-104 --reason "test steps adapted"
$ requimd review --all
error: --all needs --reason (FR-307)
```

- LSP (FR-218): "Mark as reviewed" stays one click and logs reason "confirmed". A second code action "Mark as reviewed with reason…" asks for the text via `window/showMessageRequest` choices or an input-capable client command.
- Overlap with DD-2: the log could become the `reviews/` record format of R-45. Decide together.

**P5. Editorial-change escape with safeguards (FR-307, FR-116).**

```text
$ requimd review UC-012.s2 --editorial "typo: 'recieve' -> 'receive'"
UC-012.s2 changed in 1 word since the last review (git HEAD).
Re-stamped 3 links to UC-012.s2 as editorial:
  FR-104 impl UC-012.s2
  FR-110 impl UC-012.s2
  TC-230 ver UC-012.s2
Kept suspect (stale before this change): TC-231 ver UC-012.s2
```

Safeguards:
1. **Target-side only.** It re-stamps links *to* the named target. It only re-stamps links whose stored hash equals the target's hash before the current edit. Links already stale for an earlier change stay suspect.
2. **The reason is mandatory** and is logged with the prefix `editorial` (P4).
3. **Visible in review.** `requimd diff` and the PR description list editorial re-stamps in their own section ("changed without downstream review").
4. **Size warning**, not a block. If more than `editorial-max-words` (default 5) words changed against git HEAD, `--editorial` asks for confirmation in the terminal, or needs `--force` in scripts.
5. **No `--all --editorial`.**

**P6. Show what changed (FR-307, LSP hover).** `requimd review <ID> --show` and the LSP hover on a suspect link show the word-level diff of the target between now and the most recent git revision whose content hash equals the stored hash. This works offline, survives rebases because it is content-based like DOORS Next validity, and needs no text in the lock. ADR-0007's "Later" item becomes concrete.

**P7. Not now: item-level review state.** Leave it out of ADR-0007. "Changed after approval" goes to DD-2 (review records, R-45) or R-40 (status transitions). Revisit if those do not cover it.

**P8. Small spec clarifications.**
- FR-116: say that a revert restores the reviewed state (content-keyed).
- FR-116: say that suspicion is one-directional (source becomes suspect when the target changes).
- FR-116: state whether a suspect link still counts as coverage (tracey says no).

### Open questions for the product owner
1. Hash scope before v0.1.0: should the target's own link lines and `prio` leave the hash (P1)? Should metadata be hashed sorted? This changes every existing lock entry once. The P1 migration keeps open suspects.
2. Defaults per link key: should `ref` be "never suspect" by default, or should every key stay suspect until configured (P2)?
3. Merge strategy: is `merge=union` plus a tolerant loader acceptable, or do you prefer an explicit custom merge driver, accepting a per-clone setup step (P3)?
4. Review log: one file `requimd.reviews` at the root, a Markdown table (readable on GitHub and the site), or fold into DD-2's review records (P4)? Is the git user name enough as reviewer identity?
5. Editorial escape: do you want it at all? If yes, should the word threshold warn (proposed) or block (P5)?
6. Coverage: should a suspect link still count as coverage for `uncovered` checks, as today, or not, as in tracey?
7. Direction: confirm that requimd stays downstream-only, so a change in a test case never makes the requirement it verifies suspect.
8. Permissions: should this repository add CODEOWNERS for `requimd.lock` and `requimd.reviews`, so every clearing of suspects needs your PR approval?
