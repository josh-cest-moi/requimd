# DD-2: Review and approval evidence in requirements tools

Deep dive DD-2, compiled 18 November 2026 from public sources. It builds on the phase 1 report (`requirements-and-design-tools-2026.md`, R-35, R-36, R-37, R-44 to R-48, R-80, R-81) and does not repeat its general scan. Paywalled standards (ISO 26262, ASPICE PAM, IEC 62304, DO-178C, ISO 13485) are summarised from secondary sources. Statements marked **[background, unverified]** come from general knowledge and were not confirmed by a source read in this deep dive.

## Q1. What is in a review in Jama, Modern Requirements, Codebeamer, Polarion and DOORS Next, and what is recorded afterwards?

### Takeaway
All five tools treat a review as a separate object. It has a scope fixed to a version (a baseline or "the version at review start"), three role tiers (moderator or creator, approver, plain or optional reviewer), per-item votes and comments, a due date, a way to restart only what changed, and a terminal "finalized" state that locks the record. What remains afterwards is a list of who decided what, when, with what meaning, on which version. In Jama, Modern Requirements and DOORS Next a baseline or report can be made from it. Signatures are optional per review, and the moderator decides whether they apply.

### Cited Findings

**Jama Connect Review Center**
- Scope: any group of items can be sent for review, such as a document, sections, a folder, a filter, a release, a baseline or a test plan. The wizard sets a deadline and invites reviewers and approvers by email. — [Jama: how reviews work](https://help.jamasoftware.com/ah/en/getting-to-know-jama-connect-features/how-reviews-work.html)
- Roles: the *moderator* owns the content, starts the review and needs a Creator licence. *Reviewers* add to the discussion. *Approvers* sign off and approve items. — [Jama: how reviews work](https://help.jamasoftware.com/ah/en/getting-to-know-jama-connect-features/how-reviews-work.html)
- Versioning: feedback is tied to the review *revision* it was given on. After the moderator works in feedback, they publish a new revision, and a review can be reopened for more revisions and to see what changed. Items are marked "Approved" or "Rejected". — [Jama: how reviews work](https://help.jamasoftware.com/ah/en/getting-to-know-jama-connect-features/how-reviews-work.html)
- Closing: a review closes at the deadline or when the moderator closes it, and the moderator can then finalize it. Finalizing updates item review statuses (e.g. "In review" to "Approved") and can drive the item workflow. Reopening a finalized review returns items to their previous review status. — [Jama: how reviews work](https://help.jamasoftware.com/ah/en/getting-to-know-jama-connect-features/how-reviews-work.html); [Jama: finalize a review](https://help.jamasoftware.com/en/reviews-in-jama-connect/moderator-tasks/finalize-a-review.html)
- Per-review settings include "Require electronic signatures from approvers" (all approvers must sign before the review is approved), "Enable signature group for approvers" (a signer role), delegation of the approver role (the signer role moves with it), notifications when participants complete or "request a new version", time tracking per reviewer, and optional voting with a configurable vote label. — [Jama: configure review settings](https://help.jamasoftware.com/en/reviews-in-jama-connect/moderator-tasks/create-a-review/configure-review-settings---settings-page.html)
- Signature contents: the user's identity (username and password), a date and time stamp, and a meaning ("I approve the content of this review"), plus an optional signer role and reason. Review activity shows signatures, time stamps, signer roles and meaning. Reopening a review revokes the approver's signature, and the revocation reason goes into the activity stream. Reviews with signatures, and baselines created from them, cannot be deleted. — [Jama: electronic signatures](https://help.jamasoftware.com/ah/en/collaborating-with-your-team/electronic-signatures.html); [Jama: sign a review electronically](https://help.jamasoftware.com/ah/en/reviews-in-jama-connect/approver-tasks/sign-a-review-electronically.html)
- Signatures are optional. The moderator can untick "require electronic signatures from approvers" per review. — [Jama community](https://community.jamasoftware.com/discussion/comment/17379/)

**Modern Requirements4DevOps (Azure DevOps)**
- A request has a subject, an optional message, work items (chosen by query, ID or title), reviewers and approvers, and a due date. Each participant gets an email link. *Reviewers* only comment; *approvers* comment and approve or reject. — [MR review module](https://www.modernrequirements.com/products/modern-requirements4devops-features/review/)
- Approvers decide per item or all at once. With "Approve as Package" the items must be decided as one unit. A rejection needs a comment ("Comment is required for Reject and Submit Review actions"). — [MR review module](https://www.modernrequirements.com/products/modern-requirements4devops-features/review/)
- Part 11 mode: "Ask User for Password on Approval" makes approvers re-enter their Azure DevOps credentials before a decision is recorded. A Pre State and Post State move approved items automatically, for example from Active to Approved. — [MR review module](https://www.modernrequirements.com/products/modern-requirements4devops-features/review/)
- Records: the Approval Audit Report (Word or PDF) lists each approver's name, role, decision, comment and timestamp. A Review Results Report holds reviewer feedback. After closure a baseline can be created from the latest revisions, from only the approved and rejected revisions, or from the existing revisions of all items. A follow-up review can be created for the same items, and "Update Review" adds participants or moves the due date while keeping earlier responses. — [MR review module](https://www.modernrequirements.com/products/modern-requirements4devops-features/review/)

**PTC Codebeamer Review Hub**
- A review collects items from a tracker, report or release. Each item is reviewed at the version it had when the review was created. — [Codebeamer 3.1: Review Hub](https://support.ptc.com/help/codebeamer/r3.1/en/codebeamer/cbx_user_guide/review_hub/cbx_reviews_in_codebeamer.html)
- The creation wizard has five steps. (1) Items: a tracker, report or release, optionally with a baseline and "review only the items that have changed since the selected Baseline". (2) Details: name, description or template. (3) Assignments: moderators, reviewers and viewers, by user, role or group, plus notifications and signatures (moderator signatures required, a minimum number of reviewer signatures; reviewers sign only after voting on every item). (4) Settings: deadline, findings items created from comments, and *item status thresholds* (how many reviewers must approve or reject). (5) Overview. A review holds at most 1000 items. — [Codebeamer 3.3: creating a review](https://support.ptc.com/help/codebeamer/r3.3/en/codebeamer/cbx_user_guide/review_hub/cbx_creating_new_review.html)
- Roles: moderators review, edit, restart and close. Reviewers approve, reject or ask for more work, and comment, but cannot edit. Viewers have read-only access. When an item changes during a review, contributors are notified, and the review can be restarted fully or only for the items modified since it started. — [Codebeamer 3.1: Review Hub](https://support.ptc.com/help/codebeamer/r3.1/en/codebeamer/cbx_user_guide/review_hub/cbx_reviews_in_codebeamer.html); [codebeamer wiki](https://codebeamer.com/cb/wiki/2016061)
- Reassigning a role removes the previous holder's votes and comments, while delegating keeps them. A checklist can sit in the description; only moderators edit it. — [codebeamer wiki: Review Hub](https://codebeamer.com/cb/wiki/2016061); [Review Hub checklist](https://codebeamer.com/cb/wiki/32714289)
- Exporting review comments and member signatures is the subject of a PTC knowledge article (Codebeamer 22.10 to 2.2), which was not read. — [PTC CS411244](https://www.ptc.com/en/support/article/CS411244)

**Siemens Polarion**
- The project's workflow configuration defines the approval steps. It can require an electronic signature when a user moves a Document from one status to another, or when approving a whole Document. Notifications go out on transitions. — [Polarion 20 help: approval phase](https://docs.plm.automation.siemens.com/content/polarion/20/help/en_US/user_and_administration_help/user_guide/polarion_for_requirements_engineers/approval_phase.html) (summary only; the page redirected to a script-rendered portal)
- Work-item approvals are a resource of their own, per user, in the REST API. A community extension resets all approvals of a work item to "Waiting" on a configured workflow transition, so reworked items get re-reviewed. — [Polarion REST client](https://polarion-rest-api-client.readthedocs.io/en/stable/code/polarion_rest_api_client.open_api_client.api.work_item_approvals.html); [Polarion extension 86](https://extensions.polarion.com/extensions/86-default-approvals-and-reset-approvals-status)
- Polarion 2512 sends notifications with direct links when a signature is needed and when others sign or decline (phase 1). — [Polarion 2512](https://blogs.sw.siemens.com/polarion/polarion-alm-2512-whats-new-and-noteworthy/)

**IBM DOORS Next**
- A review is created in a stream or a baseline. In a stream, edits to artifacts show up in the running review, so IBM says streams suit only stable content. A baseline freezes the content. While a formal review is paused, its creator can replace artifacts with their latest versions. — [IBM DOORS Next 7.1: reviews](https://www.ibm.com/docs/en/engineering-lifecycle-management-suite/doors-next/7.1.0?topic=artifacts-reviews)
- Roles: an *approver* must approve, disapprove or abstain on each artifact before the review can reach Reviewed or Finalized. A *reviewer* must review or abstain. An *optional reviewer*'s progress does not count. — [IBM DOORS Next 7.1: reviews](https://www.ibm.com/docs/en/engineering-lifecycle-management-suite/doors-next/7.1.0?topic=artifacts-reviews)
- States: Draft, In progress, Paused (participants and artifacts can change; no comments), Reviewed (the creator or an admin can reopen), Overdue (set automatically after the due date) and Finalized (saved and locked). Results are exported through a document-style report wizard. Electronic signatures belong to baselines, not reviews. — [IBM DOORS Next 7.1: reviews](https://www.ibm.com/docs/en/engineering-lifecycle-management-suite/doors-next/7.1.0?topic=artifacts-reviews)
- A user-submitted idea says reviews and review reports do not show each user's approval status per artifact. — [IBM Ideas ENGRMDN-I-457](https://ibm-ai-apps.ideas.ibm.com/ideas/ENGRMDN-I-457)

### Inferences
- The tools converge on one **common review model**. requimd can copy it without a server:

  | Aspect | Common pattern | Variants |
  |---|---|---|
  | Scope | Items selected by query, document or baseline; optionally "only changed since baseline" | Codebeamer and MR make "changed since baseline" explicit; DOORS Next warns against stream reviews |
  | Version pinning | Item versions fixed at review start, or a baseline | Jama "revisions"; Codebeamer "version at creation" |
  | Roles | Moderator or creator; approver (vote counts); reviewer (comment, sometimes vote); optional or viewer | Codebeamer thresholds; DOORS Next optional reviewers do not count |
  | Verdict | Per item: approve, reject (or disapprove), abstain or needs work; overall result derived | MR "approve as package"; reject needs a comment |
  | Re-review | New revision (Jama), restart changed items (Codebeamer), follow-up request (MR), approvals reset to Waiting (Polarion extension) | — |
  | Closing | Finalized, locked, cannot be deleted when signed | Reopening revokes signatures (Jama) |
  | Record | Name, role, decision, comment, timestamp, signature meaning, per item, plus a report (Word or PDF) | Baseline made from the reviewed set (Jama, MR) |
- "Reviewed against the version at review start, and the changed items flagged for re-review" is what content hashes give requimd for free. A record that names item IDs with hashes is the file-based equivalent of Codebeamer's "version at creation" and Jama's revision.
- Whether signatures apply is decided per review. Part 11-grade signing is a configuration, not something every review does. Most reviews in these tools are ordinary records with names and timestamps.
- In every tool, the item status (Approved) is moved by a workflow triggered by the human decision (MR Post State, Jama finalize). requimd's BR-004 forbids the automatic part, so the person sets the status and requimd only checks that it matches the record.

### Gaps
- Polarion's current help portal renders through scripts and could not be read, so the exact Polarion approval states, comment model and signature dialog are unverified.
- The exact fields of Codebeamer's review export and of the DOORS Next review report were not found.
- No vendor page described the review audit trail field by field beyond Jama's and MR's summaries.

## Q2. What do Part 11, Annex 11, ISO 26262, ASPICE, IEC 62304 and DO-178C expect as review evidence, and which parts can git satisfy? Have assessors accepted git-based evidence?

### Takeaway
Part 11 asks three things of a signature: a printed name, the date and time, and the meaning. The signature must be linked to its record so it cannot be copied to another. Around that sit identity controls (unique to one person, identity verified, two components such as ID and password, a letter of certification to FDA) and audit trails. Git with signed commits and structured trailers can carry the name, time, meaning and linkage. Uniqueness, identity proofing, non-transferability, access control and tamper protection come from the hosting setup and from procedures, not from files. The safety and medical standards ask for review records that identify the reviewed version, the reviewers, the date, the results and the findings, with independence at higher integrity levels. They prescribe content, not storage. One open automotive project (Eclipse S-CORE) runs ISO 26262 and ASPICE inspections through GitHub pull requests. No public statement by an assessor, notified body, FAA or FDA accepting or rejecting git-based review evidence was found.

### Cited Findings

**21 CFR Part 11 (US FDA)**
- §11.10 (closed systems) requires: (a) validation and the ability to detect invalid or altered records; (b) accurate and complete copies in human-readable and electronic form; (c) protection and retrieval for the retention period; (d) access limited to authorised people; (e) computer-generated, time-stamped audit trails that independently record operator actions that create, modify or delete records, where changes do not obscure earlier entries and trails are kept as long as the records; (f) operational checks on the sequence of steps; (g) authority checks on who may sign or alter; (j) written policies making people accountable for their signatures; (k) control of system documentation, with a time-sequenced change audit trail. — [21 CFR 11.10 (LII)](https://www.law.cornell.edu/cfr/text/21/11.10)
- §11.50: a signed record must show the signer's printed name, the date and time of signing, and the meaning (such as review, approval, responsibility or authorship). These are under the same controls as the record and appear in every human-readable display or printout. — [21 CFR 11.50 (LII)](https://www.law.cornell.edu/cfr/text/21/11.50)
- §11.70: signatures must be linked to their records so they cannot be excised, copied or transferred to falsify another record by ordinary means. — [21 CFR 11.70 (LII)](https://www.law.cornell.edu/cfr/text/21/11.70)
- §11.100: each signature is unique to one person and never reused or reassigned. The organisation verifies identity before issuing one, and the user certifies to FDA, by a letter with a handwritten signature, that their electronic signatures are legally binding. — [21 CFR 11.100 (LII)](https://www.law.cornell.edu/cfr/text/21/11.100)
- §11.200: non-biometric signatures use at least two distinct identification components. In one continuous session, the first signing needs all components and later ones at least one. Outside a session, every signing needs all components. Only the owner may use a signature, and misuse would need two or more people to collude. — [21 CFR 11.200 (LII)](https://www.law.cornell.edu/cfr/text/21/11.200)
- FDA's 2003 *Scope and Application* guidance narrows Part 11. FDA does not intend to enforce the validation, audit trail, retention and copying provisions as described there, but the predicate rules (the underlying record requirements) still apply. — [FDA guidance page](https://www.fda.gov/regulatory-information/search-fda-guidance-documents/part-11-electronic-records-electronic-signatures-scope-and-application)
- ISPE (2025) calls it "incorrect and unhelpful" to argue that Part 11 needs a data audit trail on software code changes. Modern tools record the who, when and how of each change through commits, and controls should be scaled to risk. — [ISPE Pharmaceutical Engineering, Jan/Feb 2025](https://ispe.org/pharmaceutical-engineering/january-february-2025/gampr-approach-computerized-system-life-cycle-and)

**EU GMP Annex 11**
- The current 2011 Annex 11 expects electronic signatures to have the same impact as handwritten ones, to be permanently linked to their record and to include date and time (clause 14). It asks for audit trails, based on risk, on GMP-relevant changes and deletions (clause 9), and for controlled changes (clause 10). — [EC Annex 11 PDF](https://health.ec.europa.eu/system/files/2016-11/annex11_01-2011_en_0.pdf) (binary, not parsed); summary from [Scilife](https://www.scilife.io/blog/eu-gmp-annex-11-explained-a-practical-guide-to-csv)
- A revised draft Annex 11 was published on 7 July 2025, with comments until 7 October 2025, together with a new Chapter 4 and Annex 22 (AI). Final versions are expected "from 2026", and the 2011 text still applies. The draft has a dedicated section 13 on signatures and an audit trail section 12 with ten subsections. — [GMP-Compliance: first analysis](https://www.gmp-compliance.org/gmp-news/annex-11-draft-first-analysis); [Scilife](https://www.scilife.io/blog/eu-gmp-annex-11-explained-a-practical-guide-to-csv)
- Commentators report that the draft rules out authentication by a smart card or static PIN alone, asks for time-zone-aware automated timestamps (13.4), and covers signatures on open systems (13.2). These come from secondary summaries; the Commission text was not read. — [QbD Group](https://qbdgroup.com/en/blog/annex-11-revision-digital-compliance); [investigationsquality.com](https://investigationsquality.com/tag/audit-trail/)

**Medical device software (IEC 62304, ISO 13485, QMSR)**
- Template mappings put IEC 62304 clause 5.2.6 "Verify software requirements" in all safety classes (A, B, C), with an SOP and a requirements review checklist as evidence. OpenRegulatory's checklist asks whether requirements are testable, have acceptance criteria and include IT security, and ends with a pass or fail result and comments. — [OpenRegulatory 62304 mapping](https://templates.openregulatory.com/techdoc/62304/62304-mapping.pdf); [OpenRegulatory checklist](https://templates.openregulatory.com/techdoc/62304/checklist-software-requirements-review.pdf)
- A sample IEC 62304 audit report lists 5.2.6 as a separate finding ("Pending"), so auditors look for this record specifically. — [Innolitics sample audit report](https://innolitics.com/files/IEC_62304_Audit_Report.pdf)
- The QMSR incorporates ISO 13485:2016 clause 7.3 by reference from 2 February 2026. FDA training material summarises 7.3.5 as reviews whose results and actions are recorded. — [FDA QMSR training](https://www.fda.gov/media/189041/download)
- Design review records traditionally include the design identification, the date and the participants (the legacy 820.30(e) wording). — [meddeviceguide](https://meddeviceguide.com/blog/design-review-evidence-medical-device-agenda-minutes-independence-guide). **[Background, unverified]**: ISO 13485:2016 7.3.5 itself requires review records to include the identification of the design under review, the participants and the date.

**Automotive (ISO 26262, ASPICE 4.0)**
- ISO 26262-8:2018 clause 9 is "Verification": 9.4.3 covers verification execution and evaluation, and 9.5 the work products (verification specification and verification report). — [DIN table of contents](https://www.din.de/de/mitwirken/normenausschuesse/naautomobil/veroeffentlichungen/wdc-beuth:din21:300423989/toc-3032793/download). ISO has a working draft of the next edition. — [ISO committee](https://committee.iso.org/standard/90027.html)
- **[Background, unverified]**: 9.4.3 asks that an evaluation of verification results names the verified work product and its version, the verification specification, the configuration used, the result (pass or fail) and any deviations. Confirmation reviews (ISO 26262-2) are a separate, independent check of safety work products.
- ASPICE 4.0 (December 2023) dropped the stand-alone SUP.2 (Verification) and SUP.4 (Joint Review) processes. Review moved into the engineering processes, with requirements review under SYS.2 and SWE.1. Secondary sources disagree on exactly which SUP processes were removed (SUP.2, SUP.4 and SUP.7 according to one, SUP.2 to SUP.7 according to another). — [UL ASPICE 4.0 at a glance](https://www.ul.com/sis/resources/automotive-spice-v40-glance); [PwC Japan](https://www.pwc.com/jp/ja/knowledge/column/automotive-research-and-development/vol06.html)
- In ASPICE 4.0, SUP.1 Quality Assurance has base practices for quality criteria (BP2), work product quality assurance (BP3), process quality assurance (BP4), summarising and reporting results (BP5) and resolving non-conformances (BP6), and requires QA without conflict of interest. Reviews count as work product QA. — [Eclipse S-CORE: SUP.1](https://eclipse-score.github.io/score/main/process/standards/aspice_40/sup/sup.1.html); [UL SUP.1](https://www.ul.com/sis/insights/automotive-spice-quality-assurance-sup1)

**Avionics (DO-178C)**
- Reviews are inspections of life-cycle data, guided by a checklist or similar aid (section 6.3). The Software Verification Results (section 11.14) include review records and configuration identification of the reviewed data. Discrepancies go into problem reports. At levels A and B the reviewer must not be the author of the data. Records may take any form, including a database, and need not be exported into a deliverable document as long as they stay available for audit. AdaCore suggests GitLab as the configuration-management repository. — [AdaCore: a fresh take on DO-178C reviews](https://www.adacore.com/blog/a-fresh-take-on-do-178c-software-reviews)
- Review records should name the author or reviewer, the date and the item versions used. Keeping review and analysis records is a common gap. — [Rapitas: SOI #3](https://www.rapitasystems.com/blog/do-178c-SOI-verification-stage-involvement-3)

**Git-based evidence in practice**
- **Eclipse S-CORE** (open automotive software stack, Sphinx-based docs with need-style attributes) runs ISO 26262 and ASPICE inspections in GitHub pull requests. A safety, security or quality manager starts the inspection and moderates. CODEOWNERS assigns reviewers. Checklist results go into PR comments, pass or fail per item, with ticket links for corrections. A committed inspection file is allowed only when PR data cannot be kept. The PR author cannot approve. Opening the PR fixes the scope and the work product version. Status goes from `valid` to `valid_inspected`, and any later change resets it to `valid`, which is checked automatically. All ASIL-rated work products must be inspected before release. The page links ISO 26262 clauses through `complies` attributes but claims nothing about assessor acceptance. — [S-CORE review and inspection concept](https://eclipse-score.github.io/process_description/main/general_concepts/score_review_concept.html); [S-CORE verification process requirements](https://eclipse-score.github.io/process_description/main/process_areas/verification/guidance/verification_process_reqs.html); [S-CORE requirements inspection checklist](https://eclipse-score.github.io/process_description/main/process_areas/requirements_engineering/guidance/requirements_inspection_checklist.html)
- **OpenQMS** (a prototype) records signatures as commits with trailers: `Signature-Meaning` (required, e.g. approved or reviewed), `Signature-Role` and `Signature-Justification`, on GPG-signed commits by HR-attested identities. `openqms signatures export --since <ref>` emits JSON with name, date and time, meaning, role, GPG status, key ID and commit. Its own limitations: trailers are "a discipline, not a security boundary"; identity mapping is procedural; web edits are signed by GitHub, not the editor, which breaks §11.70. Its claim that this meets §11.50 and §11.70 is its own and unverified. — [OpenQMS: signature meaning](https://iridiumsoftware.github.io/OpenQMS/guide/signature-meaning/); [OpenQMS auditor walkthrough](https://iridiumsoftware.github.io/OpenQMS/guide/auditor-walkthrough/)
- **OpenRegulatory** first published a git-based QMS (Markdown documents with a signing table, PR review, merge equals release). It later published "We no longer recommend GitHub/GitLab as QMS software", citing that Markdown tables are nearly impossible to edit and that git and Markdown are daunting for non-technical users. It now sells its own Part 11 tool (Formwork). The article itself returned HTTP 403, so the reasons come from search snippets. — [OpenRegulatory: QMS in GitHub/GitLab](https://openregulatory.com/articles/quality-management-system-qms-in-github-gitlab); [OpenRegulatory: no longer recommend](https://openregulatory.com/articles/we-no-longer-recommend-github-gitlab-as-qms-software)
- IntuitionLabs (vendor blog, 2025) treats git as supporting, not sufficient, evidence. A PR approval "can be seen as an electronic signature" only with unique attribution and a stated meaning. A GPG signature "does not by itself establish compliance with Part 11". Git history alone is not the complete audit trail, because approvals and access logs live on the host. Its FDA case studies are explicitly "hypothetical composite scenarios". — [IntuitionLabs: git workflows and FDA compliance](https://intuitionlabs.ai/articles/git-workflows-fda-compliance)
- The Linux kernel defines trailer meanings: `Reviewed-by` states that the reviewer examined the patch, their concerns were resolved and they judge it fit, without guaranteeing it works in every case. `Acked-by` is a lighter approval, `Tested-by` reports a test run, and `Signed-off-by` is the DCO origin certificate with a real identity. — [Kernel: submitting patches](https://www.kernel.org/doc/html/latest/process/submitting-patches.html)

### Inferences
- **Mapping Part 11 and Annex 11 to git and requimd** (inferred from the findings above):

  | Requirement | Satisfied by a git-based record? |
  |---|---|
  | Printed name, date and time, meaning (§11.50(a), Annex 11 §14c) | Yes: fields in the record file, plus a commit author and date, plus a `meaning:` value; rendered on the site and on print (§11.50(b)) |
  | Signature linked to the record (§11.70, Annex 11 §14b) | Partly: a signed commit binds the signer key to the tree that contains the record and the reviewed content. Item hashes bind the record to the exact text. Only a cryptographic signature (GPG, SSH or S/MIME) resists copying; a plain trailer does not |
  | Unique, not reassigned, identity verified (§11.100) | No: identity proofing, key issuance and revocation are the organisation's procedure (key registry, HR mapping) |
  | Two components per signing (§11.200) | Only indirectly: a passphrase-protected key, or hardware key plus PIN, is plausibly two factors, but that is a key-management argument, not a file format |
  | Certification letter to FDA (§11.100(c)) | No: organisational |
  | Audit trail of create, modify and delete actions (§11.10(e)) | Mostly, if protected: git history is time-stamped and append-only only when the host forbids force-push and deletion on protected branches. Commit dates are set by the client, not computed independently |
  | Access and authority checks (§11.10(d), (g)) | Host: branch protection, CODEOWNERS, required reviewers |
  | Validation (§11.10(a)) | Tool qualification kit (R-97) plus the customer's validation |
  | Human-readable copies (§11.10(b)) | Yes: static site, printable review record |
- **Common minimum of the safety and medical standards** for a review record: (1) unique ID of the review; (2) identification of the reviewed items *and their versions*; (3) reviewers and their roles, with independence where required (author ≠ approver); (4) date; (5) criteria or checklist used; (6) result per item and overall (pass, fail, conditional); (7) findings with their disposition (linked problem reports or tickets); (8) retrievability for the retention period. A requimd record with item IDs and content hashes, committed in git, covers (1) to (7) by design. (8) depends on the repository and its retention.
- S-CORE's `valid_inspected` to `valid` reset is the same idea as R-45's "record goes stale when the item hash changes". It is the closest public precedent and shows the pattern being used in an ISO 26262 context.
- The OpenRegulatory reversal is a warning for requimd's audience. The obstacle in regulated document control was usability for non-technical signers, not legal acceptability. A requimd design must give non-git reviewers a path (Q3) rather than assume PRs.

### Gaps
- **No public evidence of acceptance or rejection** by an ASPICE assessor, ISO 26262 assessor, notified body, FAA or DER, or FDA investigator of git-based review records was found, not even an anonymised case report. S-CORE has no published assessment result. The IntuitionLabs case studies are hypothetical.
- The normative texts of ISO 26262-8 9.4.3 and 9.5, ASPICE 4.0 SYS.2 and SWE.1 output information items, IEC 62304 5.2.6, ISO 13485 7.3.5 and DO-178C 6.3 and 11.14 were not read (paywalled; the public ISO 13485 copy was a binary PDF that could not be parsed). The field lists above are secondary.
- The text of the draft Annex 11 sections 12 and 13 was not read, and its adoption status in November 2026 is unconfirmed.
- Whether Sphinx-needs or StrictDoc users have published ASPICE or ISO 26262 assessment experience was not found beyond S-CORE.

## Q3. How do reviewers without git take part in text-based tools?

### Takeaway
Text-based tools have no reviewer surface of their own. They rely on (a) rendered output (HTML or PDF) that people read, (b) spreadsheet round trips (StrictDoc has Excel export and import), or (c) the git host's PR review with rendered prose diffs. Comments on paper or PDF come back by hand. Enterprise tools solve this with free or temporary reviewer accounts and email links (phase 1). The workable offline pattern is a static review page plus a structured return sheet keyed by item ID and content hash.

### Cited Findings
- StrictDoc exports HTML (default), PDF (HTML2PDF, experimental), RST, Excel (XLSX), ReqIF and JSON, and imports ReqIF and Excel. It has a COMMENT field per requirement and a STATUS field (Draft, Active, Deleted) that only affects statistics. A diff and changelog screen is experimental. Its web UI writes back to files but needs command-line git to commit, which makes shared deployment impractical. No review workflow is documented. — [StrictDoc user guide](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_01_user_guide.html)
- Doorstop stores a `reviewed` fingerprint per item (SHA-256 over UID, text, ref and link UIDs, extendable by configuration) and a parent fingerprint per link. `doorstop review` updates them. It publishes HTML and Markdown. No reviewer workflow for non-git users is documented. — [Doorstop item reference](https://doorstop.readthedocs.io/en/latest/reference/item.html)
- GitHub shows source and rendered views for prose files (Markdown, AsciiDoc, RST and others) in commits and PRs, with added and removed text highlighted. GitHub advises checking both views when documents contain embedded HTML. Large changes may have no rendered view. — [GitHub: working with non-code files](https://docs.github.com/en/repositories/working-with-files/using-files/working-with-non-code-files); [GitHub blog: rendered prose diffs](https://github.blog/news-insights/the-library/rendered-prose-diffs/)
- A comment-sheet round trip in another domain: AODocs exports an Excel comment sheet for contractors or external reviewers without access and imports their responses back into the document. — [AODocs: comment sheets](https://support.aodocs.com/hc/en-us/articles/51559838196251)
- In DOORS Next, comments in an imported Word document can be imported as module comments, but replies from external sources cannot. — [Jazz forum](https://jazz.net/forum/questions/282164/sharing-comments)
- Modern Requirements' "Copy URL" lets stakeholders open a review without navigating Azure DevOps. Jama creates temporary 30-day accounts for outside reviewers (phase 1). — [MR review module](https://www.modernrequirements.com/products/modern-requirements4devops-features/review/); [Jama external reviewer](https://help.jamasoftware.com/ah/en/collaborating-with-your-team/stream---staying-connected/comments/add-a-reviewer-from-outside-jama-connect.html)
- OpenRegulatory names non-technical users' difficulty with git and Markdown as a reason to abandon a git-based QMS. — [OpenRegulatory](https://openregulatory.com/articles/we-no-longer-recommend-github-gitlab-as-qms-software)

### Inferences
- The return channel is the hard part, not the reading. Every text-based tool can publish HTML. None turns a stakeholder's verdict back into a structured record. A sheet (CSV or XLSX) with fixed columns (review ID, item ID, content hash, verdict, comment, reviewer name, date) can be imported and checked mechanically. A hash column lets the import reject verdicts on outdated text.
- A static HTML page without JavaScript can still offer a return channel: per-item `mailto:` links with a prefilled subject such as `RV-012 FR-010 8f3a1c0e` and a body template. No server is involved, but the reply is free text and needs transcription.
- PDF comments (Acrobat sticky notes) can be exported to Excel by Acrobat, but this depends on the reviewer's tool. It is a fallback, not a format requimd should parse.
- A record built from a returned sheet is a *transcribed* record. The evidential weight is lower than a reviewer's own signed commit, and the record should say so (`recorded-by:` and `source:` fields, plus the original sheet or email kept as an attachment).

### Gaps
- Whether GitHub allows line comments in the rendered prose view (as opposed to the source view) was not confirmed.
- No usability study of business stakeholders reviewing in PRs, spreadsheets or rendered pages was found.
- The StrictDoc Excel import's column model and its handling of comments were not read in detail.

## Q4. Can requimd derive a review record from Gerrit, GitHub or GitLab offline, or must it record its own?

### Takeaway
Only Gerrit keeps review data inside git (`refs/notes/review`, and NoteDb change refs with `Label:` footers), and requimd could read those offline. GitHub and GitLab keep approvals in the host database, so they reach requimd only through the API (network, an explicit opt-in under ADR-0011) or a downloaded export. A merged PR leaves in git at most a merge commit message and trailers the authors typed. So requimd must define its own record. Host approvals can be imported into it or cross-referenced, but they cannot be the only evidence if the record has to outlive the host.

### Cited Findings
- Gerrit's `refs/notes/review` holds one note per submitted commit with Submitted-by, Submitted-at, Reviewed-on, Project, Branch, and every review label and score at submit time with the reviewer's name and email (e.g. `Code-Review+2`). It is not cloned by default and must be fetched explicitly. — [Gerrit: refs/notes/review](https://gerrit.googlesource.com/plugins/reviewnotes/+/v2.8/src/main/resources/Documentation/refs-notes-review.md)
- Gerrit NoteDb stores change metadata in `refs/changes/YZ/XYZ/meta`, with approvals as `Label: Label-Name=Foo` footers and reviewers in a Reviewer footer. The design document says it is not fully up to date. — [Gerrit NoteDb](https://gerrit.googlesource.com/homepage/+/md-pages/docs/Notedb.md)
- GitHub's REST API lists the reviews of a PR in order. Each has a `state` (e.g. APPROVED, CHANGES_REQUESTED, COMMENTED) and the `commit_id` it was given on, and pending reviews have no `submitted_at`. — [GitHub REST: pull request reviews](https://docs.github.com/en/rest/pulls/reviews)
- GitHub branch protection offers: required approving reviews with a minimum count; "Dismiss stale pull request approvals when new commits are pushed" (an approval is removed when the diff changes); required Code Owner review; approval of the most recent push by someone other than the pusher; required signed commits; and required linear history. — [GitHub: about protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- GitLab records changes to merge request approval settings as audit events, exportable as CSV. Its compliance violations report flags merge requests with too few approvals or self-approval, exportable per group (since 16.4 or 16.5). — [GitLab: compliance violations report](https://docs.gitlab.com/user/compliance/compliance_center/compliance_violations_report/); [GitLab 15.7 audit events (archive)](https://archives.docs.gitlab.com/15.7/ee/administration/audit_events.html)
- IntuitionLabs notes that review and approval records live in the hosting or issue system and must be exported and retained separately from the repository. — [IntuitionLabs](https://intuitionlabs.ai/articles/git-workflows-fda-compliance)
- S-CORE keeps inspection checklists in PR comments and allows a committed inspection file only as a fallback, so it accepts host-held evidence. — [S-CORE review concept](https://eclipse-score.github.io/process_description/main/general_concepts/score_review_concept.html)

### Inferences
- GitHub's "dismiss stale approvals" works per PR diff, not per item. An unrelated edit in the same PR dismisses all approvals, and once merged the information is gone from git. A per-item hash record is finer and lasts longer.
- With CLAUDE.md's "Rebase and merge" policy there is no merge commit, so a merged GitHub PR leaves *no* trace in git at all except commit messages. requimd cannot rebuild "who approved" from the repository of this very project.
- Practical options, in order of fit: (1) **own record file** (always works, offline, survives a host move); (2) **trailers** (`Reviewed-by:`, `Approved-by:`) in the commit that adds the record, readable with `git interpret-trailers` or `git log --format=%(trailers)`; (3) **Gerrit notes**, readable offline; (4) **import of a GitHub or GitLab review export** (a JSON file the user downloaded, or an explicit `--network` call under ADR-0011) that writes a record with `source: github-pr#123` and the `commit_id` of each approval.

### Gaps
- Current GitLab documentation on merge request approval data per commit (the equivalent of GitHub's `commit_id`) was not read.
- No source showed whether GitHub's merge commit or squash commit messages can be configured to include approver names automatically (Gerrit's cherry-pick strategy adds `Reviewed-on` footers; this was not verified for current versions).

## Proposal for requimd

This proposal follows from the findings above. It builds on FR-403 (`requimd diff`), ADR-0007 (lock file, FNV-1a item hashes that exclude `status`), FR-116 and FR-307 (suspect links and `requimd review`), FR-400 (static site), NFR-005 and NFR-006 (single binary, offline) and BR-004.

### (1) Review package: `requimd review-package <ref1>..<ref2>`

The name avoids a clash with the existing `requimd review` (FR-307), which clears suspect links. Options: `--select <query>` (IDs, types, tags, files), `--id RV-012` (else the next free RV number from `[ids]`), `--checklist <file>`, `--out <dir>`. It works offline from local git objects.

Output, a directory that can also be published with the site:

- `index.html`, static and without JavaScript, with print CSS (R-81):
  - **Header**: review ID, title, the range as given and as resolved commit SHAs, generation time, requimd version, and a *package hash* (hash over the manifest).
  - **Scope table**: one row per item in scope with ID, title, type, status at `ref2`, change kind (added, changed, deleted, moved, status-only, metadata-only), the content hash at `ref1` and at `ref2`, and the authors of the change from `git log` (R-39).
  - **Per-item section**: the rendered text at `ref2`, a word-level diff against `ref1` (FR-403), changed metadata and links, glossary terms used, and an empty verdict box (Approve, Reject, Abstain, plus a comment) for paper use.
  - **Impact**: links that become suspect because of these changes (FR-116), and inbound or outbound links of changed items, so the reviewer sees the blast radius (R-38, R-41).
  - **Checklist**: judgement rules that cannot be automated (INCOSE C1, C4, C6, C8, testability, acceptance criteria, security), optionally one per perspective (user, designer, tester; PBR), from a project checklist file. Automated findings (`requimd check` diagnostics on the changed items) are listed separately as "already checked by the tool".
  - **Return instructions**: per-item `mailto:` links with the subject `RV-012 FR-010 <hash>`, and a pointer to the sheet.
- `review-sheet.csv` (later XLSX, R-80): columns `review, item, hash, verdict, comment, reviewer, date`, pre-filled with review, item and hash.
- `manifest.json`: machine-readable scope (item, hashes, change kind, refs), used by the import.

Unlike Jama and Codebeamer, which pin the version at review start, the package pins `ref2`. Its hashes are what a later record must match.

### (2) Review record in the repository

One file per review, e.g. `reviews/RV-012.md`, written in the requimd format so `check`, the LSP and the site treat it as an item of type review (prefix `RV`, BR-001 and BR-002 numbering). A sketch:

```markdown
## RV-012 Review of payment requirements for release 2.3
range: v2.2..4e1f9a7c  package: 3c9d0e11a2b4f675
moderator: C. Example  due: 2026-12-01  closed: 2026-11-28
checklist: checklists/requirements.md

| Reviewer | Role | Meaning | Date | Source |
|---|---|---|---|---|
| A. Example | approver | approved | 2026-11-27 | signed commit |
| B. Example | reviewer | reviewed | 2026-11-26 | sheet, recorded by C. Example |

| Item | Hash | A. Example | B. Example | Finding |
|---|---|---|---|---|
| FR-010 | 8f3a1c0e5b7d2946 | approve | approve | |
| FR-011 | 1d9e44b0a2c6f871 | reject | comment | ISS-41 unit missing |

verdict: approved with findings
```

Fields, with their source in the evidence above:

- **Identification**: review ID; range and resolved commit; package hash (DO-178C configuration identification; ISO 13485 "design under review").
- **People**: name, role (moderator, approver, reviewer, optional), *meaning* (reviewed, approved, rejected; Part 11 §11.50 and the kernel trailer semantics), date and time with time zone (draft Annex 11 13.4), and *source* (own signed commit, transcribed sheet, imported PR, Gerrit note).
- **Per-item verdicts** with the hash each verdict refers to (Codebeamer, DOORS Next, MR per-item decisions). A reject needs a finding (MR rule). Findings link to issue keys or ISS items (DO-178C problem reports).
- **Overall verdict** and closure date. A closed record is immutable by convention. A follow-up is a new RV that `supersedes:` the old one (Jama revisions, MR follow-up request).

Staleness. `requimd check` recomputes each listed item's current hash (the ADR-0007 algorithm, which already ignores `status`, whitespace and, with ADR-0010, HTML comments) and reports:

- `RV-012 is stale for FR-011: changed since review` (a warning or info, configurable) — the S-CORE `valid_inspected` to `valid` reset and Polarion's approvals-to-Waiting reset, in file form.
- `FR-010 has status approved but no current approving review` — only if a project opts in (rule profile R-15). This is the BR-004 bridge: a person sets `status: approved`, and requimd checks that the decision has evidence. requimd never writes `approved`. `requimd review-record` may *propose* `status: review` for items in an open package.
- A re-review package can be generated from the stale items only: `requimd review-package --stale RV-012` (Codebeamer's "restart only modified items").

Relation to the lock file. The record keeps its own hashes. It does **not** go into `requimd.lock`, for three reasons. Records are human-authored and immutable, while the lock is machine-maintained. One file per review avoids the lock merge conflicts the project already has. And the lock answers "is this link reviewed?" (FR-116), while the record answers "who decided on this text?" (R-45). Both use the same hash function, so one computation serves both. Because the hash ignores `status`, setting `approved` after the review does not stale the record. That is deliberate and worth stating in the ADR.

Signatures. When the commit that adds or closes a record is signed (GPG, SSH or S/MIME), `requimd check --signatures` (local `git verify-commit`, no network) lists the signer for each record and flags records whose `source: signed commit` claim is not backed by a valid signature, or whose signer does not match the named approver according to a project key map (e.g. `reviewers.toml`: name, email, key fingerprint, roles). Trailers `Reviewed-by:` and `Approved-by:` in that commit can be cross-checked against the table.

### (3) How non-git reviewers return comments

1. **Read**: the review package page or its print, from the site or a file share.
2. **Return**, in order of evidential weight:
   - (a) a git-capable reviewer adds their verdict rows to `RV-012.md` in their own signed commit, or approves the PR, which is then imported;
   - (b) a non-git reviewer fills in `review-sheet.csv` (Excel opens it) or replies through the `mailto:` links;
   - (c) a paper or PDF mark-up, transcribed by the analyst.
3. **Import**: `requimd review-record import RV-012 sheet.csv` checks the review ID and each row's hash against the manifest. Rows on outdated text are rejected with a message. The command writes the rows with `source: sheet, recorded by <analyst>` and stores the original sheet under `reviews/RV-012/` as an attachment. It never touches item status.

### (4) What evidence this gives regulated projects, and what it does not

It gives:
- A review record per review that identifies the exact reviewed text (commit plus per-item hash), the participants with roles, the date, the meaning, per-item results, findings with their disposition, and the checklist used. This matches the common minimum of ISO 26262-8 clause 9 verification reports, IEC 62304 5.2.6 requirement verification, ISO 13485 7.3.5 design review records and DO-178C 11.14 review records (all as summarised secondarily).
- Mechanical proof that "the released text is the reviewed text": staleness at check time and at any baseline (R-37 can include "all approved items have current reviews").
- Independence checks (approver ≠ author of the change, from git log), as S-CORE and DO-178C levels A and B require.
- A human-readable rendering of name, date and time, and meaning (§11.50(b)), and a git history that is the change trail when the host protects branches.
- Optional cryptographic linkage through signed commits (§11.70-like), verified offline.

It does not give, and the documentation should say so plainly:
- Part 11 and Annex 11 *system* controls: identity proofing and uniqueness (§11.100), two-component signing (§11.200), the FDA certification letter, access and authority checks (§11.10(d), (g)), protection against history rewriting, retention and backup. These are the host's and the organisation's controls (protected branches, signed-commit enforcement, key management, SOPs).
- System validation (§11.10(a)): requimd can supply a tool qualification kit (R-97), but validating its use is the customer's job.
- Independent confirmation reviews and assessments (ISO 26262-2), QA independence (ASPICE SUP.1): organisational.
- Assurance that a transcribed sheet reflects the reviewer's intent. That is a weaker record by nature.
- Tamper resistance of the hashes themselves: FNV-1a 64-bit detects accidental change but is not collision-resistant. Integrity against deliberate manipulation rests on git object IDs and signatures.
- Any statement that an assessor will accept it. No public precedent was found. S-CORE is the nearest, and it is unassessed in public.

### (5) Open questions for the product owner

1. Does merging a PR remain the approval (as in this project today), or does an approval in a regulated profile need an RV record, with the merge only publishing it?
2. Should review records be items (`RV-` prefix, rendered on the site, linkable from items with `reviewed-in:`) or a separate file kind outside the specification?
3. Are records per review (one file, many items) or per item and per review? The first matches the tools and keeps the lock untouched.
4. Which verdict values: approve, reject and abstain only, or also "approve with comments" or "needs work" (Codebeamer)? Is "approve as package" (MR) allowed?
5. Should a reject require a finding linked to an issue key or ISS item?
6. May the analyst transcribe verdicts from sheets and emails, and should such rows count as approval for the `approved`-needs-record rule, or only for "reviewed"?
7. Should requimd warn when an `approved` item has no current approving record? Always, only in a regulated rule profile, or never?
8. Is FNV-1a 64-bit enough for records, or should records (only) use SHA-256 over the same normalised content, as Doorstop does? This touches ADR-0007.
9. Should requimd verify commit signatures and map signers to named approvers (a `reviewers.toml` key map), or leave signatures entirely to the host?
10. Should importing GitHub or GitLab PR approvals be supported, from a downloaded JSON (offline) or through an explicit network command under ADR-0011? Gerrit notes could be read offline.
11. Naming: `review-package` and `review-record` next to the existing `review` (FR-307, clearing suspect links) may confuse analysts. Should FR-307's command be renamed (e.g. `confirm-links`) before reviews of content are introduced?
12. Should the package include the full text of unchanged items in scope (a full-document review) or only changed items (a delta review)? Both appear in the tools.
13. Who owns the checklist content (project file, rule profile, or both), and does the package list automated `check` findings as already verified?
